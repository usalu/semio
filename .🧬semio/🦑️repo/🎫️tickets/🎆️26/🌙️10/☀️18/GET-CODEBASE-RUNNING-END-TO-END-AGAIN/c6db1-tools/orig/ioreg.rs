//! 📇️ Db I/O kernel backend registry, close and retirement turns.
#![allow(async_fn_in_trait)]

use db_ids::{ArtifactId, DbError, GenerationId};
use db_io_core::{BLOCKING_QUEUE, DB_IO_BACKEND_CONTROLS, DB_IO_LOST_OWNERS, DB_IO_LOST_OWNER_OVERFLOW, DB_IO_LOST_OWNER_QUARANTINE, DB_IO_LOST_OWNER_SLOTS, DB_IO_LOST_OWNER_WAKE_REQUESTED, DB_IO_MAX_READ_BYTES, DB_IO_OPERATION_ITEMS, DB_IO_OPERATION_PAGES, DB_IO_PAGE_BYTES, DB_IO_PAGE_RETIREMENT_SLOTS, DB_IO_RETIREMENT_PRESSURE_FAULT, DB_IO_TASK_SLOTS, DB_IO_TOTAL_PAGES, DbIoBackendControl, DbIoCredit, DbIoDriverReservation, DbIoExecutionStep, DbIoExecutorMode, DbIoExternalBytes, DbIoFault, DbIoFaultCause, DbIoFaultKind, DbIoLostOwner, DbIoPagePhase, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoTaskHandle, DbIoTaskKind, DbIoTaskPhase, DbIoTaskSlot, DbIoTerminal, DbIoText, DbIoU64List, DbIoWriterReleaseStep, EMPTY_DB_IO_PAGE_SLOT, PayloadStorage as _, WAL_WRITER_CONTROLLERS, WAL_WRITER_SIGNALS, WalWriterController, cell, db_io_admission_released, db_io_backend_owner_reserve, db_io_backend_parts, db_io_error_text, db_io_operation_detach_task, db_io_operation_return, db_io_page_arena, db_io_page_credit, db_io_result_handback, db_io_slot_matches, db_io_task_arena, db_io_text_literal, defer_fault_waiters, fault_all_requested, request, request_controller};
use pack::ContentHash;
use semio_framework_async::{Lane, WorkerMaintenanceRequest, WorkerMaintenanceStep, WorkerPool, WorkerPoolUse, WorkerSubmitErrorKind};
use std::{future::Future, pin::Pin, sync::Arc, sync::Mutex, sync::atomic::AtomicU64, sync::atomic::Ordering, task::Context, task::Poll, task::Waker};
#[cfg(test)]
use db_io_core::{DB_IO_LEDGER_OWNER, DbIoLedgerOwner, db_io_ledger_census};

//#region 🔖️Limits
/// 📥 One-fragment-per-poll copy into already checked-out fixed pages.
pub struct DbIoPageCopy<'a> {
    pub source: &'a [u8],
    pub cursor: usize,
    pub writer: Option<DbIoPageWriter>,
}

pub struct DbIoPageOwnerCopy<'a> {
    pub source: &'a DbIoPages,
    pub cursor: u8,
    pub writer: Option<DbIoPageWriter>,
}

pub struct DbIoPageHash<'a> {
    pub source: &'a DbIoPages,
    pub cursor: u8,
    pub hasher: semio_framework_hash::Hasher,
}

pub fn db_io_copy_pages(source: &[u8]) -> Result<DbIoPageCopy<'_>, DbError> {
    let pages = source.len().div_ceil(DB_IO_PAGE_BYTES);
    let writer = DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)?;
    Ok(DbIoPageCopy { source, cursor: 0, writer: Some(writer) })
}

/// 🪜 Retained external-driver byte conversion with one observed owner/page/close opportunity per poll.
pub struct DbIoObservedBytesWrite<'a> {
    pub reservation: Option<DbIoDriverReservation>,
    pub source: Option<DbIoExternalBytes>,
    pub output: &'a mut DbIoPageWriter,
    pub cursor: usize,
    limit: usize,
    pub phase: u8,
}

pub fn db_io_write_observed_bytes(reservation: DbIoDriverReservation, source: Vec<u8>, output: &mut DbIoPageWriter) -> DbIoObservedBytesWrite<'_> {
    let limit = source.len();
    DbIoObservedBytesWrite { reservation: Some(reservation), source: Some(DbIoExternalBytes::new(source)), output, cursor: 0, limit, phase: 0 }
}

/// 🧷 Retained ranged external-driver transfer with the same exact close authority.
pub fn db_io_write_observed_bytes_range(reservation: DbIoDriverReservation, source: Vec<u8>, offset: usize, length: usize, output: &mut DbIoPageWriter) -> Result<DbIoObservedBytesWrite<'_>, DbError> {
    let limit = offset.checked_add(length).ok_or(DbError::LimitExceeded("DB I/O observed-byte range"))?;
    if limit > source.len() {
        return Err(DbError::InvalidArgument("DB I/O observed-byte range exceeds result".to_string()));
    }
    Ok(DbIoObservedBytesWrite { reservation: Some(reservation), source: Some(DbIoExternalBytes::new(source)), output, cursor: offset, limit, phase: 0 })
}

impl Future for DbIoObservedBytesWrite<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        match owner.phase {
            0 => {
                let capacity = owner.source.as_ref().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its exact source".to_string()))?.capacity()?;
                owner.reservation.as_mut().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its reservation".to_string()))?.observe_capacity(capacity)?;
                owner.phase = 1;
            }
            1 => {
                let source = owner.source.as_ref().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its source during copy".to_string()))?.as_slice()?;
                if owner.cursor < owner.limit {
                    let end = owner.cursor.checked_add(DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("DB I/O observed-byte cursor"))?.min(owner.limit);
                    let written = owner.output.write_fragment(&source[owner.cursor..end])?;
                    owner.cursor += written;
                } else {
                    owner.phase = 2;
                }
            }
            2 => {
                let source = owner.source.as_mut().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its source during close".to_string()))?;
                if !source.close_step() {
                    owner.source.take();
                    owner.phase = 3;
                }
            }
            3 => {
                owner.reservation.as_mut().ok_or_else(|| DbError::Internal("DB I/O observed-byte cursor lost its reservation during close".to_string()))?.close_step()?;
                owner.reservation.take();
                owner.phase = 4;
            }
            _ => match owner.output.seal_retained_step()? {
                Some(published) => return std::task::Poll::Ready(Ok(published)),
                None => {}
            },
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

pub fn db_io_copy_page_owner(source: &DbIoPages) -> Result<DbIoPageOwnerCopy<'_>, DbError> {
    let writer = DbIoPageWriter::try_reserve(source.page_count() as usize).map_err(DbIoPageWriterRejected::into_error)?;
    Ok(DbIoPageOwnerCopy { source, cursor: 0, writer: Some(writer) })
}

pub fn db_io_hash_pages(source: &DbIoPages) -> DbIoPageHash<'_> {
    DbIoPageHash { source, cursor: 0, hasher: semio_framework_hash::Hasher::new() }
}

/// 🔢 Persisted one-scalar transfer and one-owner close authority for driver lists.
pub struct DbIoListTransfer<'a> {
    pub source: Option<DbIoU64List>,
    pub output: &'a mut DbIoU64List,
    pub cursor: usize,
    pub phase: u8,
}

pub fn db_io_transfer_list(source: DbIoU64List, output: &mut DbIoU64List) -> DbIoListTransfer<'_> {
    DbIoListTransfer { source: Some(source), output, cursor: 0, phase: 0 }
}

impl Future for DbIoListTransfer<'_> {
    type Output = Result<DbIoU64List, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if owner.phase == 2 {
            return std::task::Poll::Ready(Ok(std::mem::take(owner.output)));
        }
        let source = owner.source.as_mut().ok_or_else(|| DbError::Internal("DB I/O list transfer lost its exact source".to_string()))?;
        match owner.phase {
            0 if owner.cursor < source.len() => {
                owner.output.push(source.as_slice()[owner.cursor])?;
                owner.cursor += 1;
            }
            0 => owner.phase = 1,
            1 if source.close_step() => {}
            1 => {
                owner.source.take();
                owner.phase = 2;
            }
            _ => return std::task::Poll::Ready(Err(DbError::Internal("DB I/O list transfer lost its exact source".to_string()))),
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageCopy<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if owner.cursor == owner.source.len() {
            let Some(writer) = owner.writer.as_mut() else {
                return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-copy cursor lost its retained writer".to_string())));
            };
            return match writer.seal_retained_step()? {
                Some(pages) => {
                    owner.writer.take();
                    std::task::Poll::Ready(Ok(pages))
                }
                None => {
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
            };
        }
        let start = owner.cursor;
        let end = start.checked_add(DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("DB I/O page-copy cursor"))?.min(owner.source.len());
        let source = &owner.source[start..end];
        let Some(writer) = owner.writer.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-copy cursor lost its retained writer".to_string())));
        };
        let written = writer.write_fragment(source)?;
        owner.cursor += written;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageOwnerCopy<'_> {
    type Output = Result<DbIoPages, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(source) = owner.source.page(owner.cursor) else {
            let Some(writer) = owner.writer.as_mut() else {
                return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-owner cursor lost its retained writer".to_string())));
            };
            return match writer.seal_retained_step()? {
                Some(pages) => {
                    owner.writer.take();
                    std::task::Poll::Ready(Ok(pages))
                }
                None => {
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
            };
        };
        let Some(writer) = owner.writer.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O page-owner cursor lost its retained writer".to_string())));
        };
        let written = writer.write_fragment(source)?;
        if written != source.len() {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O source fragment exceeded one fixed page".to_string())));
        }
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for DbIoPageHash<'_> {
    type Output = ContentHash;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(fragment) = owner.source.page(owner.cursor) else {
            let hasher = std::mem::replace(&mut owner.hasher, semio_framework_hash::Hasher::new());
            return std::task::Poll::Ready(ContentHash(*hasher.finalize().as_bytes()));
        };
        owner.hasher.update(fragment);
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

pub const DB_IO_PLATFORM_BUFFERS: usize = 16;

const DB_IO_PLATFORM_RETIREMENT_SLOTS: usize = DB_IO_PLATFORM_BUFFERS * 2;

const DB_IO_PLATFORM_BUFFER_BYTES: usize = DB_IO_MAX_READ_BYTES as usize;

struct DbIoPlatformBacking(std::cell::UnsafeCell<[u8; DB_IO_PLATFORM_BUFFER_BYTES]>);

impl DbIoPlatformBacking {
    pub const fn new() -> Self {
        Self(std::cell::UnsafeCell::new([0; DB_IO_PLATFORM_BUFFER_BYTES]))
    }
}

unsafe impl Sync for DbIoPlatformBacking {}

static DB_IO_PLATFORM_BACKINGS: [DbIoPlatformBacking; DB_IO_PLATFORM_BUFFERS] = [const { DbIoPlatformBacking::new() }; DB_IO_PLATFORM_BUFFERS];

#[derive(Clone, Copy)]
struct DbIoPlatformSlot {
    pub generation: u64,
    pub occupied: bool,
}

struct DbIoPlatformArena {
    pub slots: [DbIoPlatformSlot; DB_IO_PLATFORM_BUFFERS],
    pub retired: [Option<(u8, u64)>; DB_IO_PLATFORM_RETIREMENT_SLOTS],
    pub retired_read: usize,
    pub retired_len: usize,
    pub next_generation: u64,
}

static DB_IO_PLATFORM_ARENA: std::sync::Mutex<DbIoPlatformArena> =
    std::sync::Mutex::new(DbIoPlatformArena { slots: [DbIoPlatformSlot { generation: 0, occupied: false }; DB_IO_PLATFORM_BUFFERS], retired: [None; DB_IO_PLATFORM_RETIREMENT_SLOTS], retired_read: 0, retired_len: 0, next_generation: 1 });

/// 🧩 Explicit contiguous platform-call buffer backed by a fixed process slot.
pub struct DbIoPlatformBuffer {
    pub slot: u8,
    pub generation: u64,
    pub len: usize,
    pub copied: usize,
    pub credit: DbIoPageWriter,
    pub returned: bool,
}

pub struct DbIoPlatformCopy<'a> {
    pub source: &'a DbIoPages,
    pub cursor: u8,
    pub owner: Option<DbIoPlatformBuffer>,
}

/// 🚪 One retained prepared-platform close opportunity per poll.
pub struct DbIoPlatformClose {
    pub owner: Option<DbIoPlatformBuffer>,
}

/// 🪡 One retained prepared-platform slice fragment per poll.
pub(crate) struct DbIoPlatformSlicesCopy<'a> {
    pub first: &'a [u8],
    pub second: &'a [u8],
    pub source: u8,
    pub cursor: usize,
    pub owner: Option<DbIoPlatformBuffer>,
    pub fault: Option<DbError>,
}

pub fn db_io_prepare_platform(source: &DbIoPages) -> Result<DbIoPlatformCopy<'_>, DbError> {
    let _ = db_io_platform_maintenance_step()?;
    if source.len() > DB_IO_PLATFORM_BUFFER_BYTES {
        return Err(DbError::LimitExceeded("db_io prepared platform buffer"));
    }
    let (slot, generation) = {
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = arena.slots.iter().position(|slot| !slot.occupied).ok_or_else(|| DbError::Unavailable("DB I/O prepared platform capacity exhausted".to_string()))?;
        let generation = arena.next_generation;
        arena.next_generation = arena.next_generation.checked_add(1).filter(|next| *next != 0).ok_or(DbError::LimitExceeded("db_io platform generation"))?;
        arena.slots[slot] = DbIoPlatformSlot { generation, occupied: true };
        (slot as u8, generation)
    };
    let credit = match DbIoPageWriter::try_reserve_for_operation(source.operation(), source.page_count() as usize) {
        Ok(credit) => credit,
        Err(error) => {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
            return Err(error.into_error());
        }
    };
    let owner = DbIoPlatformBuffer { slot, generation, len: source.len(), copied: 0, credit, returned: false };
    Ok(DbIoPlatformCopy { source, cursor: 0, owner: Some(owner) })
}

impl DbIoPlatformBuffer {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &(&*DB_IO_PLATFORM_BACKINGS[self.slot as usize].0.get())[..self.len] }
    }

    pub fn as_static_driver_slice(&self) -> &'static [u8] {
        unsafe { &(&*DB_IO_PLATFORM_BACKINGS[self.slot as usize].0.get())[..self.len] }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.credit.close_step()?.is_some() {
            return Ok(true);
        }
        if self.returned {
            return Ok(false);
        }
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut arena.slots[self.slot as usize];
        if !slot.occupied || slot.generation != self.generation {
            return Err(DbError::Internal("DB I/O prepared platform slot lost ABA authority".to_string()));
        }
        *slot = DbIoPlatformSlot { generation: 0, occupied: false };
        self.returned = true;
        Ok(true)
    }
}

pub fn db_io_close_platform(owner: DbIoPlatformBuffer) -> DbIoPlatformClose {
    DbIoPlatformClose { owner: Some(owner) }
}

impl Future for DbIoPlatformClose {
    type Output = Result<(), DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(platform) = owner.owner.as_mut() else { return std::task::Poll::Ready(Ok(())) };
        if platform.close_step()? {
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        owner.owner.take();
        std::task::Poll::Ready(Ok(()))
    }
}

fn db_io_reserve_platform_buffer(operation: u64, total: usize) -> Result<DbIoPlatformBuffer, DbError> {
    if total > DB_IO_PLATFORM_BUFFER_BYTES {
        return Err(DbError::LimitExceeded("db_io prepared platform slices"));
    }
    let (slot, generation) = {
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = arena.slots.iter().position(|slot| !slot.occupied).ok_or_else(|| DbError::Unavailable("DB I/O prepared platform capacity exhausted".to_string()))?;
        let generation = arena.next_generation;
        arena.next_generation = arena.next_generation.checked_add(1).filter(|next| *next != 0).ok_or(DbError::LimitExceeded("db_io platform generation"))?;
        arena.slots[slot] = DbIoPlatformSlot { generation, occupied: true };
        (slot as u8, generation)
    };
    let credit = match DbIoPageWriter::try_reserve_for_operation(operation, total.div_ceil(DB_IO_PAGE_BYTES)) {
        Ok(credit) => credit,
        Err(error) => {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
            return Err(error.into_error());
        }
    };
    Ok(DbIoPlatformBuffer { slot, generation, len: total, copied: 0, credit, returned: false })
}

pub fn db_io_prepare_platform_slices<'a>(operation: u64, first: &'a [u8], second: &'a [u8]) -> DbIoPlatformSlicesCopy<'a> {
    let reserved = first.len().checked_add(second.len()).ok_or(DbError::LimitExceeded("db_io prepared platform slices")).and_then(|total| db_io_reserve_platform_buffer(operation, total));
    match reserved {
        Ok(owner) => DbIoPlatformSlicesCopy { first, second, source: 0, cursor: 0, owner: Some(owner), fault: None },
        Err(error) => DbIoPlatformSlicesCopy { first, second, source: 0, cursor: 0, owner: None, fault: Some(error) },
    }
}

impl Future for DbIoPlatformSlicesCopy<'_> {
    type Output = Result<DbIoPlatformBuffer, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if let Some(error) = owner.fault.take() {
            return std::task::Poll::Ready(Err(error));
        }
        if owner.source > 1 {
            return std::task::Poll::Ready(owner.owner.take().ok_or_else(|| DbError::Internal("DB I/O prepared-platform slice cursor lost its exact owner".to_string())));
        }
        let source = if owner.source == 0 { owner.first } else { owner.second };
        if owner.cursor == source.len() {
            owner.source += 1;
            owner.cursor = 0;
        } else {
            let written = (source.len() - owner.cursor).min(DB_IO_PAGE_BYTES);
            let platform = owner.owner.as_mut().ok_or_else(|| DbError::Internal("DB I/O prepared-platform slice cursor lost its owner during copy".to_string()))?;
            let end = platform.copied.checked_add(written).ok_or(DbError::LimitExceeded("db_io prepared platform slices cursor"))?;
            unsafe { (&mut *DB_IO_PLATFORM_BACKINGS[platform.slot as usize].0.get())[platform.copied..end].copy_from_slice(&source[owner.cursor..owner.cursor + written]) };
            platform.copied = end;
            owner.cursor += written;
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Drop for DbIoPlatformBuffer {
    fn drop(&mut self) {
        if !self.returned {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let slot = arena.slots[self.slot as usize];
            if slot.occupied && slot.generation == self.generation {
                if arena.retired_len >= DB_IO_PLATFORM_BUFFERS {
                    DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                let write = (arena.retired_read + arena.retired_len) % DB_IO_PLATFORM_RETIREMENT_SLOTS;
                arena.retired[write] = Some((self.slot, self.generation));
                arena.retired_len += 1;
                self.returned = true;
            }
        }
    }
}

pub fn db_io_platform_maintenance_step() -> Result<bool, DbError> {
    let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if arena.retired_len == 0 {
        return Ok(false);
    }
    let read = arena.retired_read;
    let Some((slot, generation)) = arena.retired[read].take() else {
        DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        return Err(DbError::Internal("DB I/O platform retired length lost exact owner".to_string()));
    };
    let owner = arena.slots[slot as usize];
    if !owner.occupied || owner.generation != generation {
        return Err(DbError::Internal("DB I/O platform retirement lost ABA authority".to_string()));
    }
    arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
    arena.retired_read = (read + 1) % DB_IO_PLATFORM_RETIREMENT_SLOTS;
    arena.retired_len -= 1;
    Ok(true)
}

impl Future for DbIoPlatformCopy<'_> {
    type Output = Result<DbIoPlatformBuffer, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(fragment) = owner.source.page(owner.cursor) else {
            return std::task::Poll::Ready(owner.owner.take().ok_or_else(|| DbError::Internal("DB I/O platform cursor lost its retained buffer".to_string())));
        };
        let Some(platform) = owner.owner.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O platform cursor lost its retained buffer".to_string())));
        };
        let end = platform.copied.checked_add(fragment.len()).ok_or(DbError::LimitExceeded("db_io prepared platform cursor"))?;
        unsafe { (&mut *DB_IO_PLATFORM_BACKINGS[platform.slot as usize].0.get())[platform.copied..end].copy_from_slice(fragment) };
        platform.copied = end;
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

pub fn db_io_page_maintenance_step() -> Result<Option<usize>, DbError> {
    let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let retired_read = state.retired_read;
    let Some((slot, generation)) = state.retired[retired_read].take() else { return Ok(None) };
    state.retired_read = (state.retired_read + 1) % DB_IO_PAGE_RETIREMENT_SLOTS;
    state.retired_len -= 1;
    let owner = state.slots[slot as usize];
    if owner.generation != generation || owner.phase != DbIoPagePhase::Closing {
        return Err(DbError::Internal("db I/O page retirement lost ABA authority".to_string()));
    }
    state.slots[slot as usize] = EMPTY_DB_IO_PAGE_SLOT;
    let write = (state.free_read + state.free_len) % DB_IO_TOTAL_PAGES;
    state.free[write] = slot;
    state.free_len += 1;
    drop(state);
    db_io_operation_return(owner.operation, db_io_page_credit(1))?;
    Ok(Some(DB_IO_PAGE_BYTES))
}

/// 🔤 Converts one pre-admitted external-driver string into fixed repository text.
pub async fn db_io_copy_observed_text(mut reservation: DbIoDriverReservation, source: String) -> Result<DbIoText, DbError> {
    let mut source = DbIoExternalBytes::new(source.into_bytes());
    reservation.observe_capacity(source.capacity()?)?;
    let value = std::str::from_utf8(source.as_slice()?).map_err(|_| DbError::Corrupt("DB I/O observed text is not UTF-8".to_string()))?;
    let text = DbIoText::try_from_str(value)?;
    while !source.terminal_is_empty() {
        let _ = source.close_step();
        semio_framework_async::yield_once().await;
    }
    reservation.close_step()?;
    Ok(text)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoBackendKind {
    Memory,
    Filesystem,
    Sqlite,
    Postgres,
    Neo4j,
}
//#region 🔖️DbIoCensus
pub static DB_IO_CENSUS_TASKS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

pub static DB_IO_CENSUS_STEPS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

pub static DB_IO_CENSUS_TURNS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

static DB_IO_CENSUS_ADMISSION_WAITS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

static DB_IO_CENSUS_ADMISSION_WAIT_MICROS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

static DB_IO_CENSUS_ADMISSION_REFUSALS: [std::sync::atomic::AtomicU64; DbIoTaskKind::ALL.len()] = [const { std::sync::atomic::AtomicU64::new(0) }; DbIoTaskKind::ALL.len()];

/// ⏳️ Counts one admission wait of a `kind` task that lasted `waited` and whether it ended in the refusal.
pub fn db_io_census_admission_wait(kind: DbIoTaskKind, waited: std::time::Duration, refused: bool) {
    DB_IO_CENSUS_ADMISSION_WAITS[kind as usize].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    DB_IO_CENSUS_ADMISSION_WAIT_MICROS[kind as usize].fetch_add(u64::try_from(waited.as_micros()).unwrap_or(u64::MAX), std::sync::atomic::Ordering::Relaxed);
    if refused {
        DB_IO_CENSUS_ADMISSION_REFUSALS[kind as usize].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

/// 🧮️ A reading of the process-wide DB I/O census: typed tasks submitted, executor steps
/// driven and worker-pool turns spent per [`DbIoTaskKind`] since the process started, plus the
/// admission waits of [`submit_db_io_task_admitted`] (backend opens count as `BackendOpen`): waits
/// begun, microseconds waited, and waits that ended in the refusal. Two readings subtract to the exact
/// I/O an interval cost, which is what the throughput laws bound (independent of machine load).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DbIoCensusV1 {
    pub tasks: [u64; DbIoTaskKind::ALL.len()],
    pub steps: [u64; DbIoTaskKind::ALL.len()],
    pub turns: [u64; DbIoTaskKind::ALL.len()],
    admission_waits: [u64; DbIoTaskKind::ALL.len()],
    admission_wait_micros: [u64; DbIoTaskKind::ALL.len()],
    admission_refusals: [u64; DbIoTaskKind::ALL.len()],
}

impl DbIoCensusV1 {
    pub fn tasks(&self, kind: DbIoTaskKind) -> u64 {
        self.tasks[kind as usize]
    }

    pub fn steps(&self, kind: DbIoTaskKind) -> u64 {
        self.steps[kind as usize]
    }

    pub fn turns(&self, kind: DbIoTaskKind) -> u64 {
        self.turns[kind as usize]
    }

    pub fn total_tasks(&self) -> u64 {
        self.tasks.iter().sum()
    }

    pub fn total_steps(&self) -> u64 {
        self.steps.iter().sum()
    }

    pub fn total_turns(&self) -> u64 {
        self.turns.iter().sum()
    }

    pub fn admission_waits(&self, kind: DbIoTaskKind) -> u64 {
        self.admission_waits[kind as usize]
    }

    pub fn admission_wait_micros(&self, kind: DbIoTaskKind) -> u64 {
        self.admission_wait_micros[kind as usize]
    }

    pub fn admission_refusals(&self, kind: DbIoTaskKind) -> u64 {
        self.admission_refusals[kind as usize]
    }

    pub fn total_admission_waits(&self) -> u64 {
        self.admission_waits.iter().sum()
    }

    pub fn total_admission_wait_micros(&self) -> u64 {
        self.admission_wait_micros.iter().fold(0, |total, micros| total.saturating_add(*micros))
    }

    pub fn total_admission_refusals(&self) -> u64 {
        self.admission_refusals.iter().sum()
    }

    /// ➖️ What happened between `earlier` and this reading.
    pub fn since(&self, earlier: &DbIoCensusV1) -> DbIoCensusV1 {
        DbIoCensusV1 {
            tasks: std::array::from_fn(|kind| self.tasks[kind].saturating_sub(earlier.tasks[kind])),
            steps: std::array::from_fn(|kind| self.steps[kind].saturating_sub(earlier.steps[kind])),
            turns: std::array::from_fn(|kind| self.turns[kind].saturating_sub(earlier.turns[kind])),
            admission_waits: std::array::from_fn(|kind| self.admission_waits[kind].saturating_sub(earlier.admission_waits[kind])),
            admission_wait_micros: std::array::from_fn(|kind| self.admission_wait_micros[kind].saturating_sub(earlier.admission_wait_micros[kind])),
            admission_refusals: std::array::from_fn(|kind| self.admission_refusals[kind].saturating_sub(earlier.admission_refusals[kind])),
        }
    }

    /// 🧾️ `kind=tasks/steps/turns` for every kind that saw I/O, followed by
    /// `waits/µs/refusals` for every kind that waited for admission, for law output and captures.
    pub fn describe(&self) -> String {
        DbIoTaskKind::ALL
            .iter()
            .filter(|kind| self.tasks(**kind) != 0 || self.steps(**kind) != 0 || self.admission_waits(**kind) != 0)
            .map(|kind| match self.admission_waits(*kind) {
                0 => format!("{kind:?}={}/{}/{}", self.tasks(*kind), self.steps(*kind), self.turns(*kind)),
                waits => format!("{kind:?}={}/{}/{} admission={waits}/{}us/{}", self.tasks(*kind), self.steps(*kind), self.turns(*kind), self.admission_wait_micros(*kind), self.admission_refusals(*kind)),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// 🧮️ Reads the process-wide DB I/O census.
pub fn db_io_census() -> DbIoCensusV1 {
    DbIoCensusV1 {
        tasks: std::array::from_fn(|kind| DB_IO_CENSUS_TASKS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        steps: std::array::from_fn(|kind| DB_IO_CENSUS_STEPS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        turns: std::array::from_fn(|kind| DB_IO_CENSUS_TURNS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        admission_waits: std::array::from_fn(|kind| DB_IO_CENSUS_ADMISSION_WAITS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        admission_wait_micros: std::array::from_fn(|kind| DB_IO_CENSUS_ADMISSION_WAIT_MICROS[kind].load(std::sync::atomic::Ordering::Relaxed)),
        admission_refusals: std::array::from_fn(|kind| DB_IO_CENSUS_ADMISSION_REFUSALS[kind].load(std::sync::atomic::Ordering::Relaxed)),
    }
}
//#endregion 🔖️DbIoCensus

struct DbIoBackendRegistrySlot {
    pub generation: u64,
    pub kind: DbIoBackendKind,
    pub executor: Option<Box<dyn DbIoTaskExecutor>>,
    pub executor_retired: bool,
    pub close_requested: bool,
    pub leased_operation: u64,
    pub admitted_operation: u64,
    executing_steps: u32,
    pub pending_operations: usize,
    pub owner_operation: u64,
    pub owner_credit: DbIoCredit,
    pub mode: DbIoExecutorMode,
    pub pool: Option<Arc<WorkerPool>>,
    pub pool_use: Option<Arc<WorkerPoolUse>>,
    pub close_fault: Option<DbIoText>,
    pub cleanup_fault: Option<DbIoFault>,
}

struct DbIoBackendRegistry {
    pub slots: [DbIoBackendRegistrySlot; DB_IO_BACKEND_CONTROLS],
    pub free: [u16; DB_IO_BACKEND_CONTROLS],
    pub free_read: usize,
    pub free_len: usize,
    pub next_generation: u64,
}

impl DbIoBackendRegistry {
    pub fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| DbIoBackendRegistrySlot {
                generation: 0,
                kind: DbIoBackendKind::Memory,
                executor: None,
                executor_retired: false,
                close_requested: false,
                leased_operation: 0,
                admitted_operation: 0,
                executing_steps: 0,
                pending_operations: 0,
                owner_operation: 0,
                owner_credit: DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 0 },
                mode: DbIoExecutorMode::BlockingLane,
                pool: None,
                pool_use: None,
                close_fault: None,
                cleanup_fault: None,
            }),
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: DB_IO_BACKEND_CONTROLS,
            next_generation: 1,
        }
    }
}

static DB_IO_BACKEND_REGISTRY: std::sync::OnceLock<std::sync::Mutex<DbIoBackendRegistry>> = std::sync::OnceLock::new();

pub fn db_io_backend_registry() -> &'static std::sync::Mutex<DbIoBackendRegistry> {
    DB_IO_BACKEND_REGISTRY.get_or_init(|| std::sync::Mutex::new(DbIoBackendRegistry::new()))
}

pub struct DbIoRejectedBackendSlot {
    pub generation: u64,
    pub reserved: bool,
    pub executor: Option<Box<dyn DbIoTaskExecutor>>,
    pub operation: u64,
    pub credit: DbIoCredit,
    pub pool: Option<Arc<WorkerPool>>,
    pub pool_use: Option<Arc<WorkerPoolUse>>,
    pub executor_retired: bool,
    scheduled: bool,
    lane_turn: bool,
    wake_requested: bool,
    pub fault: Option<DbIoText>,
}

impl DbIoRejectedBackendSlot {
    pub fn empty() -> Self {
        Self { generation: 0, reserved: false, executor: None, operation: 0, credit: DbIoCredit::default(), pool: None, pool_use: None, executor_retired: false, scheduled: false, lane_turn: false, wake_requested: false, fault: None }
    }
}

struct DbIoRejectedBackendRegistry {
    pub slots: [DbIoRejectedBackendSlot; DB_IO_BACKEND_CONTROLS],
    pub next_generation: u64,
}

static DB_IO_REJECTED_BACKENDS: std::sync::OnceLock<std::sync::Mutex<DbIoRejectedBackendRegistry>> = std::sync::OnceLock::new();

pub fn db_io_rejected_backends() -> &'static std::sync::Mutex<DbIoRejectedBackendRegistry> {
    DB_IO_REJECTED_BACKENDS.get_or_init(|| std::sync::Mutex::new(DbIoRejectedBackendRegistry { slots: std::array::from_fn(|_| DbIoRejectedBackendSlot::empty()), next_generation: 1 }))
}

pub struct DbIoBackendRollbackReservation {
    pub index: usize,
    pub generation: u64,
    pub active: bool,
}

impl DbIoBackendRollbackReservation {
    pub fn try_reserve() -> Result<Self, DbError> {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let index = registry.slots.iter().position(|slot| slot.generation == 0).ok_or_else(|| DbError::Unavailable(DB_IO_BACKEND_ROLLBACK_EXHAUSTED.to_string()))?;
        let generation = registry.next_generation;
        registry.next_generation = generation.checked_add(1).filter(|generation| *generation != 0).ok_or(DbError::LimitExceeded("DB I/O backend rollback generation"))?;
        registry.slots[index] = DbIoRejectedBackendSlot { generation, reserved: true, ..DbIoRejectedBackendSlot::empty() };
        Ok(Self { index, generation, active: true })
    }

    pub fn commit(mut self, executor: Box<dyn DbIoTaskExecutor>, operation: u64, credit: DbIoCredit, pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>) -> DbIoBackendRegistrationRetirement {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[self.index];
        assert!(owner.generation == self.generation && owner.reserved && owner.executor.is_none());
        *owner = DbIoRejectedBackendSlot {
            generation: self.generation,
            reserved: false,
            executor: Some(executor),
            operation,
            credit,
            pool: Some(pool),
            pool_use: Some(pool_use),
            executor_retired: false,
            scheduled: false,
            lane_turn: false,
            wake_requested: false,
            fault: None,
        };
        self.active = false;
        drop(registry);
        let _ = db_io_request_rejected_backend_close(self.index, self.generation);
        DbIoBackendRegistrationRetirement { index: self.index, generation: self.generation }
    }
}

impl Drop for DbIoBackendRollbackReservation {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.slots[self.index].generation == self.generation && registry.slots[self.index].reserved {
            registry.slots[self.index] = DbIoRejectedBackendSlot::empty();
        }
        drop(registry);
        db_io_admission_released();
    }
}

struct DbIoBackendRegistrationRetirement {
    pub index: usize,
    pub generation: u64,
}

struct DbIoBackendAdmittedFailure {
    pub cause: DbError,
    pub retirement: DbIoBackendRegistrationRetirement,
}

impl DbIoBackendAdmittedFailure {
    fn into_cause(self) -> DbError {
        self.cause
    }
}

pub enum DbIoBackendRegistrationRejectedOwner {
    Incoming { kind: DbIoBackendKind, executor: Box<dyn DbIoTaskExecutor>, pool: Arc<WorkerPool> },
    Retiring(DbIoBackendRegistrationRetirement),
}

#[must_use = "backend registration rejection retains an executor or exact rollback retirement"]
pub struct DbIoBackendRegistrationRejected {
    pub cause: DbError,
    pub cleanup_error: Option<DbError>,
    pub owner: Option<DbIoBackendRegistrationRejectedOwner>,
}

impl std::fmt::Debug for DbIoBackendRegistrationRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DbIoBackendRegistrationRejected")
            .field("cause", &self.cause)
            .field("cleanup_error", &self.cleanup_error)
            .field("retained_executor", &matches!(self.owner, Some(DbIoBackendRegistrationRejectedOwner::Incoming { .. })))
            .field("retained_retirement", &matches!(self.owner, Some(DbIoBackendRegistrationRejectedOwner::Retiring(_))))
            .finish()
    }
}

impl DbIoBackendRegistrationRejected {
    pub fn error(&self) -> &DbError {
        &self.cause
    }

    pub fn has_retained_executor(&self) -> bool {
        matches!(self.owner, Some(DbIoBackendRegistrationRejectedOwner::Incoming { .. }))
    }

    pub fn cleanup_error(&self) -> Option<&DbError> {
        self.cleanup_error.as_ref()
    }

    pub fn retry(mut self) -> Result<DbIoBackendControl, Self> {
        let Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool }) = self.owner.take() else { return Err(self) };
        match register_db_io_backend(kind, executor, pool) {
            Ok(control) => Ok(control),
            Err(rejected) => Err(rejected),
        }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        let Some(owner) = self.owner.take() else { return Ok(true) };
        let retirement = match owner {
            DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool } => {
                let reservation = match DbIoBackendRollbackReservation::try_reserve() {
                    Ok(reservation) => reservation,
                    Err(_) => {
                        self.owner = Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool });
                        return Ok(false);
                    }
                };
                let pool_use = match pool.acquire_use() {
                    Ok(pool_use) => pool_use,
                    Err(_) => {
                        self.owner = Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool });
                        return Ok(false);
                    }
                };
                reservation.commit(executor, 0, DbIoCredit::default(), pool, pool_use)
            }
            DbIoBackendRegistrationRejectedOwner::Retiring(retirement) => retirement,
        };
        {
            let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &mut registry.slots[retirement.index];
            if owner.generation != retirement.generation {
                return Ok(true);
            }
            owner.fault = None;
        }
        if let Err(error) = db_io_request_rejected_backend_close(retirement.index, retirement.generation) {
            self.cleanup_error = Some(error);
            self.owner = Some(DbIoBackendRegistrationRejectedOwner::Retiring(retirement));
            return Ok(false);
        }
        let terminal = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[retirement.index].generation != retirement.generation;
        if !terminal {
            self.owner = Some(DbIoBackendRegistrationRejectedOwner::Retiring(retirement));
        }
        Ok(terminal)
    }
}

fn db_io_park_rejected_backend(executor: Box<dyn DbIoTaskExecutor>, operation: u64, credit: DbIoCredit, pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>) -> Result<(), (Box<dyn DbIoTaskExecutor>, Arc<WorkerPool>, Arc<WorkerPoolUse>)> {
    let reservation = match DbIoBackendRollbackReservation::try_reserve() {
        Ok(reservation) => reservation,
        Err(_) => return Err((executor, pool, pool_use)),
    };
    drop(reservation.commit(executor, operation, credit, pool, pool_use));
    Ok(())
}

struct DbIoRejectedBackendWake {
    pub index: usize,
    pub generation: u64,
}

impl std::task::Wake for DbIoRejectedBackendWake {
    fn wake(self: Arc<Self>) {
        let _ = db_io_request_rejected_backend_close(self.index, self.generation);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let _ = db_io_request_rejected_backend_close(self.index, self.generation);
    }
}

fn db_io_poll_rejected_backend_on_lane_io(index: usize, generation: u64) {
    {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[index];
        if owner.generation != generation {
            return;
        }
        owner.scheduled = false;
        owner.lane_turn = true;
        owner.wake_requested = false;
    }
    let waker = std::task::Waker::from(Arc::new(DbIoRejectedBackendWake { index, generation }));
    let context = &mut std::task::Context::from_waker(&waker);
    let mut terminal = false;
    let mut fault = None;
    let mut executor = {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[index];
        if owner.generation != generation {
            return;
        }
        if owner.executor_retired {
            None
        } else {
            let Some(executor) = owner.executor.take() else {
                owner.fault = Some(db_io_text_literal("DB I/O rejected backend lost its exact executor"));
                owner.lane_turn = false;
                return;
            };
            Some(executor)
        }
    };
    let close = executor.as_mut().map(|executor| executor.close_backend_step(context));
    {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[index];
        if owner.generation != generation {
            return;
        }
        if let Some(close) = close {
            match close {
                Ok(false) => owner.executor = executor.take(),
                Ok(true) if executor.as_ref().is_some_and(|executor| executor.backend_terminal_is_empty()) => {
                    owner.executor_retired = true;
                }
                Ok(true) => {
                    owner.executor = executor.take();
                    fault = Some(db_io_text_literal("DB I/O rejected backend returned a false terminal witness"));
                }
                Err(error) => {
                    owner.executor = executor.take();
                    fault = Some(db_io_error_text(&error));
                }
            }
        } else if owner.operation != 0 {
            match db_io_operation_return(owner.operation, owner.credit) {
                Ok(()) => {
                    owner.operation = 0;
                    owner.credit = DbIoCredit::default();
                }
                Err(error) => fault = Some(db_io_error_text(&error)),
            }
        } else {
            terminal = true;
        }
        owner.fault = fault;
        owner.lane_turn = false;
    }
    if terminal {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let pool_use = if registry.slots[index].generation == generation {
            registry.slots[index].scheduled = true;
            registry.slots[index].pool_use.take()
        } else {
            None
        };
        drop(registry);
        drop(pool_use);
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.slots[index].generation == generation && registry.slots[index].scheduled {
            registry.slots[index] = DbIoRejectedBackendSlot::empty();
        }
        drop(registry);
        db_io_admission_released();
    } else {
        let _ = db_io_request_rejected_backend_close(index, generation);
    }
}

fn db_io_request_rejected_backend_close(index: usize, generation: u64) -> Result<bool, DbError> {
    let pool = {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[index];
        if owner.generation != generation || owner.reserved || owner.fault.is_some() {
            return Ok(false);
        }
        if owner.scheduled || owner.lane_turn {
            owner.wake_requested = true;
            return Ok(false);
        }
        owner.scheduled = true;
        owner.pool.clone().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its shared WorkerPool".to_string()))?
    };
    match pool.try_submit(Lane::Io, Box::new(move || db_io_poll_rejected_backend_on_lane_io(index, generation))) {
        Ok(()) => Ok(true),
        Err(error) if matches!(error.kind(), WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated) => {
            pool.submit_at(pool.now_ms(), Lane::Io, error.into_job());
            Ok(true)
        }
        Err(error) => {
            let kind = error.kind();
            drop(error.into_job());
            DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
            let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &mut registry.slots[index];
            if owner.generation == generation {
                owner.scheduled = false;
                owner.fault = Some(db_io_error_text(&DbError::Unavailable(format!("DB I/O rejected backend retirement submission failed closed: {kind:?}"))));
            }
            Ok(false)
        }
    }
}

pub fn db_io_rejected_backend_maintenance_step() -> Result<bool, DbError> {
    let candidate = {
        let registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        registry.slots.iter().enumerate().find(|(_, owner)| owner.generation != 0 && !owner.reserved && !owner.scheduled && !owner.lane_turn && owner.fault.is_none()).map(|(index, owner)| (index, owner.generation))
    };
    let Some((index, generation)) = candidate else { return Ok(false) };
    db_io_request_rejected_backend_close(index, generation)
}

pub fn register_db_io_backend(kind: DbIoBackendKind, executor: Box<dyn DbIoTaskExecutor>, pool: Arc<WorkerPool>) -> Result<DbIoBackendControl, DbIoBackendRegistrationRejected> {
    let rollback = match DbIoBackendRollbackReservation::try_reserve() {
        Ok(rollback) => rollback,
        Err(cause) => return Err(DbIoBackendRegistrationRejected { cause, cleanup_error: None, owner: Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool }) }),
    };
    let pool_use = match pool.acquire_use() {
        Ok(pool_use) => pool_use,
        Err(error) => {
            return Err(DbIoBackendRegistrationRejected {
                cause: DbError::Unavailable(format!("DB I/O backend WorkerPool use rejected: {error:?}")),
                cleanup_error: None,
                owner: Some(DbIoBackendRegistrationRejectedOwner::Incoming { kind, executor, pool }),
            })
        }
    };
    let owner_credit = db_io_backend_credit(executor.as_ref());
    let owner_operation = match db_io_backend_owner_reserve(owner_credit) {
        Ok(operation) => operation,
        Err(error) => {
            let retirement = rollback.commit(executor, 0, DbIoCredit::default(), pool, pool_use);
            return Err(DbIoBackendRegistrationRejected { cause: error, cleanup_error: None, owner: Some(DbIoBackendRegistrationRejectedOwner::Retiring(retirement)) });
        }
    };
    register_db_io_backend_reserved_with_use(kind, executor, pool, pool_use, owner_operation, owner_credit, rollback).map_err(|failure| DbIoBackendRegistrationRejected {
        cause: failure.cause,
        cleanup_error: None,
        owner: Some(DbIoBackendRegistrationRejectedOwner::Retiring(failure.retirement)),
    })
}

pub fn db_io_backend_credit(executor: &dyn DbIoTaskExecutor) -> DbIoCredit {
    let base = DbIoCredit { pages: 0, bytes: executor.owner_backing_bytes(), items: 1, controls: 1 };
    base.checked_add(crate::controller_credit()).expect("fixed DB I/O maintenance controller credit")
}

pub fn register_db_io_backend_reserved_with_use(
    kind: DbIoBackendKind,
    mut executor: Box<dyn DbIoTaskExecutor>,
    pool: Arc<WorkerPool>,
    pool_use: Arc<WorkerPoolUse>,
    owner_operation: u64,
    owner_credit: DbIoCredit,
    rollback: DbIoBackendRollbackReservation,
) -> Result<DbIoBackendControl, DbIoBackendAdmittedFailure> {
    if db_io_backend_credit(executor.as_ref()) != owner_credit {
        let cause = DbError::Internal("DB I/O backend backing differs from reserved bytes".to_string());
        let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
        return Err(DbIoBackendAdmittedFailure { cause, retirement });
    }
    if let Err(error) = executor.bind_owner_operation(owner_operation) {
        let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
        return Err(DbIoBackendAdmittedFailure { cause: error, retirement });
    }
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if registry.free_len == 0 || registry.next_generation == u64::MAX {
        drop(registry);
        let cause = DbError::Unavailable(DB_IO_BACKEND_CONTROLS_EXHAUSTED.to_string());
        let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
        return Err(DbIoBackendAdmittedFailure { cause, retirement });
    }
    let slot = registry.free[registry.free_read];
    let generation = registry.next_generation;
    registry.next_generation += 1;
    let control = db_io_backend_control(kind, slot, generation);
    if executor.supports_writer_authority() {
        if let Err(error) = executor.bind_writer_control(control) {
            drop(registry);
            let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
            return Err(DbIoBackendAdmittedFailure { cause: error, retirement });
        }
    }
    if let Err(error) = crate::install_controller(control, pool.clone()) {
        drop(registry);
        let retirement = rollback.commit(executor, owner_operation, owner_credit, pool, pool_use);
        return Err(DbIoBackendAdmittedFailure { cause: error, retirement });
    }
    drop(rollback);
    registry.free_read = (registry.free_read + 1) % DB_IO_BACKEND_CONTROLS;
    registry.free_len -= 1;
    let mode = executor.mode();
    registry.slots[slot as usize] = DbIoBackendRegistrySlot {
        generation,
        kind,
        executor: Some(executor),
        executor_retired: false,
        close_requested: false,
        leased_operation: 0,
        admitted_operation: 0,
        executing_steps: 0,
        pending_operations: 0,
        owner_operation,
        owner_credit,
        mode,
        pool: Some(pool),
        pool_use: Some(pool_use),
        close_fault: None,
        cleanup_fault: None,
    };
    Ok(control)
}

pub fn db_io_writer_release_lane_step(control: DbIoBackendControl, context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[usize::from(slot)];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control || owner.executor_retired || owner.admitted_operation != 0 || owner.leased_operation != 0 {
        return Ok(DbIoWriterReleaseStep::Idle);
    }
    let Some(executor) = owner.executor.as_ref() else { return Ok(DbIoWriterReleaseStep::Idle) };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| executor.writer_release_step(context))).unwrap_or_else(|_| Err(DbError::Internal("WAL writer guard release panicked; exact executor remains retained".to_string())))
}

pub fn db_io_backend_control(kind: DbIoBackendKind, slot: u16, generation: u64) -> DbIoBackendControl {
    match kind {
        DbIoBackendKind::Memory => DbIoBackendControl::Memory { slot, generation },
        DbIoBackendKind::Filesystem => DbIoBackendControl::Filesystem { slot, generation },
        DbIoBackendKind::Sqlite => DbIoBackendControl::Sqlite { slot, generation },
        DbIoBackendKind::Postgres => DbIoBackendControl::Postgres { slot, generation },
        DbIoBackendKind::Neo4j => DbIoBackendControl::Neo4j { slot, generation },
    }
}

pub fn db_io_executor_execute(control: DbIoBackendControl, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let executor: *const dyn DbIoTaskExecutor = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation {
            return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
        }
        if owner.mode != DbIoExecutorMode::BlockingLane || owner.pending_operations == 0 {
            return Err(DbError::Unavailable("DB I/O blocking backend step has no admitted operation".to_string()));
        }
        let executor = owner.executor.as_deref().ok_or(DbError::Closed)?;
        owner.executing_steps = owner.executing_steps.checked_add(1).ok_or(DbError::LimitExceeded("DB I/O executing backend steps"))?;
        executor as *const dyn DbIoTaskExecutor
    };
    // SAFETY: the executor stays boxed in its registry slot while `executing_steps` counts this step and
    // this operation is admitted (`pending_operations` > 0): every path that moves, drops or mutably
    // borrows it (backend close, retirement, the async-native lease) first requires both to be zero, so
    // concurrent steps of different operations only share `&self` (`DbIoTaskExecutor: Sync`).
    let execution = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let executor = unsafe { &*executor };
        executor.pin_writer_operation(operation, task)?;
        executor.execute_step(operation, task)
    }));
    {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        owner.executing_steps = owner.executing_steps.saturating_sub(1);
    }
    match execution {
        Ok(result) => result,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

pub fn db_io_executor_mode(control: DbIoBackendControl) -> Result<DbIoExecutorMode, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    Ok(owner.mode)
}

pub fn db_io_backend_return_operation(control: DbIoBackendControl, operation: u64) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation
        || db_io_backend_control(owner.kind, slot, generation) != control
        || owner.pending_operations == 0
        || owner.mode != DbIoExecutorMode::BlockingLane && (owner.admitted_operation != operation || owner.leased_operation != 0)
    {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    if owner.mode != DbIoExecutorMode::BlockingLane {
        owner.admitted_operation = 0;
    }
    owner.pending_operations -= 1;
    drop(registry);
    db_io_core::request_controller(control);
    db_io_admission_released();
    Ok(())
}

pub fn db_io_return_async_executor(control: DbIoBackendControl, operation: u64, executor: Box<dyn DbIoTaskExecutor>) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control || owner.leased_operation != operation || owner.executor.is_some() {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    owner.executor = Some(executor);
    owner.leased_operation = 0;
    drop(registry);
    db_io_core::request_controller(control);
    crate::defer_fault_notifications(control);
    Ok(())
}

fn db_io_executor_close_operation(control: DbIoBackendControl, operation: u64, task: &DbIoTask) -> Result<bool, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    let executor = owner.executor.as_ref().ok_or(DbError::Closed)?;
    let terminal = executor.close_operation_step(operation, task)?;
    if terminal {
        executor.finish_writer_operation(operation, task)?;
    }
    Ok(terminal)
}

pub fn db_io_backend_close_step(control: DbIoBackendControl, context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let executor = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
        }
        if owner.pending_operations != 0 || owner.admitted_operation != 0 || owner.leased_operation != 0 || owner.executing_steps != 0 {
            return Ok(false);
        }
        if owner.executor_retired {
            None
        } else {
            Some(owner.executor.take().ok_or(DbError::Closed)?)
        }
    };
    if let Some(mut executor) = executor {
        let close = executor.close_backend_step(context);
        let terminal_empty = executor.backend_terminal_is_empty();
        crate::notify_terminal(control);
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
        }
        match close {
            Ok(false) => owner.executor = Some(executor),
            Ok(true) if terminal_empty => owner.executor_retired = true,
            Ok(true) => {
                owner.executor = Some(executor);
                return Err(DbError::Internal("DB I/O backend returned a false terminal witness".to_string()));
            }
            Err(error) => {
                owner.executor = Some(executor);
                return Err(error);
            }
        }
        return Ok(false);
    }
    let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let owner = &mut registry.slots[slot as usize];
    if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
        return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(owner.generation) });
    }
    if !crate::retire_controller(control)? {
        return Ok(false);
    }
    if owner.owner_operation != 0 {
        db_io_operation_return(owner.owner_operation, owner.owner_credit)?;
        owner.owner_operation = 0;
        owner.owner_credit = DbIoCredit::default();
    }
    let pool_use = owner.pool_use.take().ok_or_else(|| DbError::Internal("DB I/O backend close lost its WorkerPool use".to_string()))?;
    registry.slots[slot as usize] = DbIoBackendRegistrySlot {
        generation: 0,
        kind: DbIoBackendKind::Memory,
        executor: None,
        executor_retired: false,
        close_requested: false,
        leased_operation: 0,
        admitted_operation: 0,
        executing_steps: 0,
        pending_operations: 0,
        owner_operation: 0,
        owner_credit: DbIoCredit::default(),
        mode: DbIoExecutorMode::BlockingLane,
        pool: None,
        pool_use: None,
        close_fault: None,
        cleanup_fault: None,
    };
    let write = (registry.free_read + registry.free_len) % DB_IO_BACKEND_CONTROLS;
    registry.free[write] = slot;
    registry.free_len += 1;
    drop(registry);
    drop(pool_use);
    db_io_admission_released();
    Ok(true)
}

/// 🪦️ One retirement turn of a close-requested backend, run only inside its own pre-admitted
/// maintenance hook, so a dropped or closed backend can never lose its retirement request.
pub(crate) fn db_io_backend_retirement_turn(control: DbIoBackendControl, context: &mut std::task::Context<'_>) -> DbIoBackendRetirementTurn {
    let (slot, generation) = db_io_backend_parts(control);
    {
        let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control || !owner.close_requested || owner.close_fault.is_some() {
            return DbIoBackendRetirementTurn::Idle;
        }
        if owner.pending_operations != 0 || owner.admitted_operation != 0 || owner.leased_operation != 0 || owner.executing_steps != 0 {
            return DbIoBackendRetirementTurn::Idle;
        }
    }
    match db_io_backend_close_step(control, context) {
        Ok(true) => DbIoBackendRetirementTurn::Retired,
        Ok(false) => DbIoBackendRetirementTurn::More,
        Err(error) => {
            DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
            let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = &mut registry.slots[slot as usize];
            if owner.generation == generation && db_io_backend_control(owner.kind, slot, generation) == control {
                owner.close_fault = Some(db_io_error_text(&error));
            }
            DbIoBackendRetirementTurn::Idle
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DbIoBackendRetirementTurn {
    Idle,
    More,
    Retired,
}

pub fn db_io_request_backend_close(control: DbIoBackendControl) -> Result<bool, DbError> {
    {
        let (slot, generation) = db_io_backend_parts(control);
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return Ok(false);
        }
        if let Some(fault) = owner.close_fault.as_ref() {
            return Err(DbError::Internal(fault.as_str().to_string()));
        }
        owner.close_requested = true;
    }
    crate::request_retirement(control)
}

pub fn db_io_task_fault(kind: DbIoFaultKind, error: &DbError) -> DbIoFault {
    let cause = match kind {
        DbIoFaultKind::Cancelled => DbIoFaultCause::Closed,
        DbIoFaultKind::Panic => DbIoFaultCause::Internal,
        DbIoFaultKind::Saturated => DbIoFaultCause::Unavailable,
        DbIoFaultKind::Backend | DbIoFaultKind::Stale => DbIoFaultCause::from_db_error(error),
    };
    let detail = match error {
        DbError::Io(detail)
        | DbError::NotFound(detail)
        | DbError::AlreadyExists(detail)
        | DbError::InvalidArgument(detail)
        | DbError::Conflict(detail)
        | DbError::Unavailable(detail)
        | DbError::Timeout(detail)
        | DbError::Corrupt(detail)
        | DbError::Unauthorized(detail)
        | DbError::Internal(detail) => DbIoText::try_from_str(detail).unwrap_or_else(|_| db_io_text_literal("DB I/O fault detail exceeded fixed authority")),
        DbError::LimitExceeded(detail) | DbError::Unimplemented(detail) => db_io_text_literal(detail),
        DbError::Rejected { .. } => db_io_text_literal("DB I/O backend returned an artifact-layer rejection"),
        DbError::Fenced { .. } | DbError::StaleGeneration { .. } | DbError::Closed => db_io_error_text(error),
    };
    DbIoFault { kind, cause, detail, result_handback: None }
}

pub fn db_io_literal_fault(kind: DbIoFaultKind, cause: DbIoFaultCause, detail: &'static str) -> DbIoFault {
    DbIoFault { kind, cause, detail: db_io_text_literal(detail), result_handback: None }
}

/// 🧯 Records one operation-cleanup fault on the backend it belongs to — never on the caller that happened to drive the close
/// ring — so only that backend's own task waiters and its close report it; the task stays in the ring and is retried. A
/// successful cleanup of the backend (`None`) clears it.
fn db_io_note_backend_cleanup(control: DbIoBackendControl, outcome: Option<&DbError>) {
    let (slot, generation) = db_io_backend_parts(control);
    let stale = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return;
        }
        std::mem::replace(&mut owner.cleanup_fault, outcome.map(|error| db_io_task_fault(DbIoFaultKind::Backend, error)))
    };
    if let Some(mut stale) = stale {
        while stale.close_step() {}
    }
}

fn db_io_task_closable(owner: &DbIoTaskSlot) -> bool {
    let running = owner.async_driver.is_some() || owner.phase == DbIoTaskPhase::Executing || owner.phase == DbIoTaskPhase::Queued && owner.retry_attempt.is_none();
    let awaiting_owner = !owner.abandoned && (matches!(owner.terminal, Some(DbIoTerminal::Result(_) | DbIoTerminal::Fault(_))) || owner.terminal.is_none() && owner.phase != DbIoTaskPhase::Closing);
    !running && !awaiting_owner
}

/// 🧺️ One bounded batch of the task close ring, run by a backend's maintenance hook. Reports
/// whether a closable task remains; running or owner-held tasks resume the hook through their wakers.
pub(crate) fn db_io_task_retirement_batch() -> bool {
    for _ in 0..DB_IO_OPERATION_ITEMS {
        if !matches!(db_io_task_close_step(), Ok(Some(_))) {
            break;
        }
    }
    let arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    (0..arena.closing_len).filter_map(|offset| arena.closing[(arena.closing_read + offset) % DB_IO_OPERATION_ITEMS]).any(|handle| {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        db_io_slot_matches(&owner, handle) && db_io_task_closable(&owner)
    })
}

pub fn db_io_wake(owner: &mut DbIoTaskSlot) -> Option<std::task::Waker> {
    owner.waker.take()
}

pub static DB_IO_RETRY_MAINTENANCE_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// ⏱️ How long one worker-pool turn keeps driving the same task's resumable steps (one page,
/// one list item, one seal opportunity each) before it yields the worker: every step used to be its
/// own pool job, so a single index-run read cost ~35 pool turns and a whole commit ~14 000.
pub const DB_IO_EXECUTOR_TURN_MICROS: u64 = 2_000;

/// 🧮️ The most resumable steps one worker-pool turn drives, whatever the clock says.
pub const DB_IO_EXECUTOR_TURN_STEPS: usize = 4 * DB_IO_OPERATION_PAGES;

pub fn db_io_cancel_requested(handle: DbIoTaskHandle) -> bool {
    let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    !db_io_slot_matches(&owner, handle) || owner.cancelled
}

pub(crate) fn db_io_lost_owner_maintenance_batch() -> Result<bool, DbError> {
    DB_IO_LOST_OWNER_WAKE_REQUESTED.store(false, std::sync::atomic::Ordering::Release);
    for _ in 0..DB_IO_LOST_OWNER_SLOTS {
        if !db_io_lost_owner_close_step()? {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn db_io_lost_owner_close_step() -> Result<bool, DbError> {
    let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(index) = owners.iter().position(Option::is_some) else {
        drop(owners);
        let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = owners.iter_mut().find(|slot| slot.is_none()) else {
                drop(owners);
                let terminal = db_io_lost_owner_close_opportunity(quarantine[index].as_mut().ok_or_else(|| DbError::Internal("DB I/O quarantine retirement changed exact owner".to_string()))?)?;
                if terminal {
                    quarantine[index] = None;
                }
                return Ok(true);
            };
            *slot = quarantine[index].take();
            return Ok(true);
        };
        let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = owners.iter_mut().find(|slot| slot.is_none()) else {
            drop(owners);
            let terminal = db_io_lost_owner_close_opportunity(overflow[index].as_mut().ok_or_else(|| DbError::Internal("DB I/O overflow retirement changed exact owner".to_string()))?)?;
            if terminal {
                overflow[index] = None;
            }
            return Ok(true);
        };
        *slot = overflow[index].take();
        return Ok(true);
    };
    let terminal = db_io_lost_owner_close_opportunity(owners[index].as_mut().ok_or_else(|| DbError::Internal("DB I/O lost-owner cursor changed owner".to_string()))?)?;
    if terminal {
        owners[index] = None;
    }
    Ok(true)
}

pub fn db_io_lost_owner_close_opportunity(owner: &mut DbIoLostOwner) -> Result<bool, DbError> {
    let terminal = match owner {
        DbIoLostOwner::PageWriter(owner) => owner.close_step()?.is_none(),
        DbIoLostOwner::Pages(owner) => owner.close_step()?.is_none(),
        DbIoLostOwner::List(owner) => {
            let _ = owner.close_step();
            owner.terminal_is_empty()
        }
        DbIoLostOwner::Lease(owner) => {
            let _ = owner.resource.close_step() || owner.holder.close_step() || owner.handback_step();
            owner.terminal_is_empty()
        }
        DbIoLostOwner::Fault(owner) => {
            let _ = owner.close_step();
            owner.detail.terminal_is_empty() && owner.result_handback.is_none()
        }
        DbIoLostOwner::DriverReservation(owner) => {
            owner.close_step()?;
            true
        }
        DbIoLostOwner::ExternalBytes(owner) => !owner.close_step(),
        DbIoLostOwner::ArtifactId(owner) => {
            let _ = owner.close_step()?;
            owner.terminal_is_empty()
        }
        DbIoLostOwner::Backend { owner, operation, credit, pool, pool_use } => {
            let backend = owner.take().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its exact executor".to_string()))?;
            let worker_pool = pool.take().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its shared WorkerPool".to_string()))?;
            let worker_pool_use = pool_use.take().ok_or_else(|| DbError::Internal("DB I/O rejected backend lost its WorkerPool use".to_string()))?;
            match db_io_park_rejected_backend(backend, *operation, *credit, worker_pool, worker_pool_use) {
                Ok(()) => {
                    *operation = 0;
                    *credit = DbIoCredit::default();
                    true
                }
                Err((backend, worker_pool, worker_pool_use)) => {
                    *owner = Some(backend);
                    *pool = Some(worker_pool);
                    *pool_use = Some(worker_pool_use);
                    DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                    return Ok(false);
                }
            }
        }
        DbIoLostOwner::ResultLease { handback, result } => {
            if let Some(owner) = result.as_mut() {
                if owner.close_step()?.is_some() {
                    return Ok(false);
                }
                *result = None;
                return Ok(false);
            }
            db_io_result_handback(*handback)?;
            true
        }
    };
    Ok(terminal)
}

pub const DB_IO_BACKEND_CONTROLS_EXHAUSTED: &str = "db I/O backend control capacity exhausted";

pub const DB_IO_BACKEND_ROLLBACK_EXHAUSTED: &str = "DB I/O backend rollback capacity exhausted";

const DB_IO_MAINTENANCE_CLASSES: usize = 6;

pub static DB_IO_MAINTENANCE_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn db_io_maintenance_turn(cursor: &std::sync::atomic::AtomicUsize, mut opportunity: impl FnMut(usize) -> Result<bool, DbError>) -> Result<bool, DbError> {
    let start = cursor.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % DB_IO_MAINTENANCE_CLASSES;
    for offset in 0..DB_IO_MAINTENANCE_CLASSES {
        if opportunity((start + offset) % DB_IO_MAINTENANCE_CLASSES)? {
            return Ok(true);
        }
    }
    Ok(false)
}

static DB_IO_TASK_CLOSE_TURN: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn db_io_task_close_step() -> Result<Option<usize>, DbError> {
    let _turn = match DB_IO_TASK_CLOSE_TURN.try_lock() {
        Ok(turn) => turn,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(Some(0)),
        Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
    };
    let handle = {
        let arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if arena.closing_len == 0 {
            return Ok(None);
        }
        arena.closing[arena.closing_read].ok_or_else(|| DbError::Internal("DB I/O close ring length lost its exact handle".to_string()))?
    };
    let mut owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if !db_io_slot_matches(&owner, handle) {
        return Err(DbError::Internal("DB I/O close lost task ABA authority".to_string()));
    }
    if owner.async_driver.is_some() || owner.phase == DbIoTaskPhase::Executing || owner.phase == DbIoTaskPhase::Queued && owner.retry_attempt.is_none() {
        drop(owner);
        db_io_rotate_close_head(handle)?;
        return Ok(Some(0));
    }
    if owner.cancelled && owner.phase == DbIoTaskPhase::Queued && owner.retry_attempt.is_some() {
        if let Some(task) = owner.task.as_ref() {
            task.transition_pages(DbIoPagePhase::Queued, DbIoPagePhase::TerminalResult)?;
        }
        owner.retry_attempt = None;
        owner.phase = DbIoTaskPhase::Cancelled;
        if owner.terminal.is_none() {
            owner.terminal = Some(DbIoTerminal::Cancelled(None));
        }
        if let Some(waker) = db_io_wake(&mut owner) {
            drop(owner);
            waker.wake();
        }
        return Ok(Some(0));
    }
    if !db_io_task_closable(&owner) {
        drop(owner);
        db_io_rotate_close_head(handle)?;
        return Ok(Some(0));
    }
    if !owner.backend_cleanup_done {
        let task = owner.task.as_ref().ok_or_else(|| DbError::Internal("DB I/O cleanup lost typed task owner".to_string()))?;
        let backend = task.backend();
        let result = db_io_executor_close_operation(backend, handle.operation, task);
        if matches!(result, Ok(true)) {
            owner.backend_cleanup_done = true;
        }
        drop(owner);
        let rotated = if matches!(result, Ok(true)) { Ok(()) } else { db_io_rotate_close_head(handle) };
        drop(_turn);
        crate::defer_fault_notifications(backend);
        match &result {
            Ok(true) => db_io_note_backend_cleanup(backend, None),
            Ok(false) => {}
            Err(error) => db_io_note_backend_cleanup(backend, Some(error)),
        }
        rotated?;
        return Ok(Some(0));
    }
    if let Some(terminal) = owner.terminal.as_mut() {
        let step = match terminal {
            DbIoTerminal::Result(result) => result.close_step()?,
            DbIoTerminal::Fault(fault) => fault.detail.close_step().then_some(0),
            DbIoTerminal::Cancelled(result) => {
                if let Some(cancelled) = result.as_mut() {
                    if let Some(bytes) = cancelled.close_step()? {
                        return Ok(Some(bytes));
                    }
                    *result = None;
                    if let Some(waker) = db_io_wake(&mut owner) {
                        drop(owner);
                        waker.wake();
                    }
                    return Ok(Some(0));
                }
                if !owner.abandoned {
                    owner.close_enqueued = false;
                    drop(owner);
                    db_io_remove_close_head(handle)?;
                    return Ok(Some(0));
                }
                None
            }
        };
        if let Some(bytes) = step {
            return Ok(Some(bytes));
        }
        owner.terminal = None;
        return Ok(Some(0));
    }
    if let Some(task) = owner.task.as_mut() {
        if let Some(bytes) = task.close_step()? {
            return Ok(Some(bytes));
        }
        if !task.terminal_is_empty() {
            return Err(DbError::Internal("DB I/O task reported false terminal".to_string()));
        }
        owner.task = None;
        return Ok(Some(0));
    }
    if owner.retry_attempt.take().is_some() || owner.terminal_submit_kind.take().is_some() || owner.waker.take().is_some() || owner.pool.take().is_some() {
        return Ok(Some(0));
    }
    if owner.counted {
        owner.counted = false;
        BLOCKING_QUEUE.dequeued(owner.aggregate_credit.bytes);
        return Ok(Some(0));
    }
    if owner.backend_admitted {
        let backend = owner.backend.ok_or_else(|| DbError::Internal("DB I/O backend admission lost control".to_string()))?;
        let result = db_io_backend_return_operation(backend, handle.operation);
        if result.is_ok() {
            owner.backend_admitted = false;
        }
        drop(owner);
        let rotated = if result.is_ok() { Ok(()) } else { db_io_rotate_close_head(handle) };
        drop(_turn);
        crate::defer_fault_notifications(backend);
        db_io_note_backend_cleanup(backend, result.as_ref().err());
        rotated?;
        return Ok(Some(0));
    }
    if let Some(backend) = owner.backend_to_close {
        let (slot, generation) = db_io_backend_parts(backend);
        let backend_live = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[slot as usize].generation == generation;
        if backend_live {
            let _ = db_io_request_backend_close(backend)?;
            drop(owner);
            db_io_rotate_close_head(handle)?;
            return Ok(Some(0));
        }
        owner.backend_to_close = None;
        return Ok(Some(0));
    }
    if !owner.aggregate_returned {
        let task_credit = owner.aggregate_credit.checked_sub(owner.result_retained_credit).ok_or_else(|| DbError::Internal("DB I/O retained result credit exceeded its task aggregate".to_string()))?;
        db_io_operation_detach_task(handle.operation, task_credit)?;
        owner.aggregate_returned = true;
        return Ok(Some(0));
    }
    *owner = DbIoTaskSlot::empty();
    drop(owner);
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let closing_read = arena.closing_read;
    let retired = arena.closing[closing_read].take();
    if retired != Some(handle) {
        return Err(DbError::Internal("DB I/O close queue changed exact handle".to_string()));
    }
    arena.closing_read = (arena.closing_read + 1) % DB_IO_OPERATION_ITEMS;
    arena.closing_len -= 1;
    let write = (arena.free_read + arena.free_len) % DB_IO_OPERATION_ITEMS;
    arena.free[write] = handle.slot;
    arena.free_len += 1;
    drop(arena);
    db_io_admission_released();
    Ok(Some(0))
}

fn db_io_rotate_close_head(handle: DbIoTaskHandle) -> Result<(), DbError> {
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let closing_read = arena.closing_read;
    if arena.closing_len == 0 || arena.closing[closing_read] != Some(handle) {
        return Err(DbError::Internal("DB I/O close rotation lost exact ABA handle".to_string()));
    }
    arena.closing[closing_read] = None;
    arena.closing_read = (arena.closing_read + 1) % DB_IO_OPERATION_ITEMS;
    let write = (arena.closing_read + arena.closing_len - 1) % DB_IO_OPERATION_ITEMS;
    arena.closing[write] = Some(handle);
    Ok(())
}

fn db_io_remove_close_head(handle: DbIoTaskHandle) -> Result<(), DbError> {
    let mut arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let closing_read = arena.closing_read;
    if arena.closing_len == 0 || arena.closing[closing_read] != Some(handle) {
        return Err(DbError::Internal("DB I/O close suspension lost exact ABA handle".to_string()));
    }
    arena.closing[closing_read] = None;
    arena.closing_read = (arena.closing_read + 1) % DB_IO_OPERATION_ITEMS;
    arena.closing_len -= 1;
    Ok(())
}
//#endregion 🔖️Limits
//#region 🔖️RetainedDbIo
pub const DB_IO_RETRY_LIMIT: u8 = 8;

pub const DB_IO_RETRY_DELAY_MS: u64 = 1;
//#endregion 🔖️RetainedDbIo

pub(crate) fn notify_terminal(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    for cell in &WAL_WRITER_SIGNALS[usize::from(slot)] {
        let waiter = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner).notification.take();
        if let Some(waiter) = waiter {
            waiter.wake();
        }
    }
}

pub(crate) fn notify_faults(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    for cell in &WAL_WRITER_SIGNALS[usize::from(slot)] {
        let waiter = {
            let mut cell = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if cell.active.is_some_and(|key| key.backend == backend) && cell.fault.is_some() {
                cell.waiter.take()
            } else {
                None
            }
        };
        if let Some(waiter) = waiter {
            waiter.wake();
        }
    }
}

fn has_runnable_request(backend: DbIoBackendControl) -> bool {
    let (slot, _) = db_io_backend_parts(backend);
    WAL_WRITER_SIGNALS[usize::from(slot)].iter().any(|cell| {
        let cell = cell.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        cell.active.is_some_and(|key| key.backend == backend) && cell.requested && cell.fault.is_none()
    })
}

struct WalWriterControllerWake {
    pub backend: DbIoBackendControl,
}

impl std::task::Wake for WalWriterControllerWake {
    fn wake(self: Arc<Self>) {
        request_controller(self.backend);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        request_controller(self.backend);
    }
}

pub const fn controller_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: (size_of::<WalWriterControllerWake>() + 2 * size_of::<usize>()) as u64, items: 1, controls: 1 }
}

pub(crate) fn install_controller(backend: DbIoBackendControl, pool: Arc<WorkerPool>) -> Result<(), DbError> {
    let (slot, generation) = db_io_backend_parts(backend);
    let mut row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if row.is_some() {
        return Err(DbError::Conflict("WAL writer controller occupied".to_string()));
    }
    let deferred_wake_ticket = pool.install_deferred_wake_partition().map_err(|error| DbError::Unavailable(format!("WAL writer deferred-wake admission: {error:?}")))?;
    let ticket = match pool.install_maintenance_hook(Lane::Io, controller_step, [u64::from(slot), generation]) {
        Ok(ticket) => ticket,
        Err(error) => {
            assert!(pool.remove_deferred_wake_partition(deferred_wake_ticket).is_ok_and(|removed| removed));
            return Err(DbError::Unavailable(format!("WAL writer maintenance admission: {error:?}")));
        }
    };
    let waker = Waker::from(Arc::new(WalWriterControllerWake { backend }));
    *row = Some(WalWriterController { backend, pool, ticket: Some(ticket), deferred_wake_ticket: Some(deferred_wake_ticket), waker });
    Ok(())
}

pub fn defer_fault_notifications(backend: DbIoBackendControl) {
    let (slot, _) = db_io_backend_parts(backend);
    let controller = {
        let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(row) = row.as_ref().filter(|row| row.backend == backend) else { return };
        let Some(ticket) = row.deferred_wake_ticket else { return };
        (row.pool.clone(), ticket)
    };
    defer_fault_waiters(backend, &controller.0, controller.1);
}

pub fn controller_step([slot, generation]: [u64; 2]) -> WorkerMaintenanceStep {
    let Some(controller) = WAL_WRITER_CONTROLLERS.get(slot as usize) else { return WorkerMaintenanceStep::Fault };
    let (backend, waker) = {
        let row = controller.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(row) = row.as_ref().filter(|row| db_io_backend_parts(row.backend) == (slot as u16, generation)) else { return WorkerMaintenanceStep::Idle };
        (row.backend, row.waker.clone())
    };
    let writer = if has_runnable_request(backend) {
        let result = crate::db_io_writer_release_lane_step(backend, &mut Context::from_waker(&waker));
        if let Err(error) = &result {
            fault_all_requested(backend, error);
        }
        notify_terminal(backend);
        notify_faults(backend);
        Some(result)
    } else {
        None
    };
    match crate::db_io_lost_owner_maintenance_batch() {
        Ok(true) => return WorkerMaintenanceStep::More,
        Err(_) => return WorkerMaintenanceStep::Fault,
        Ok(false) => {}
    }
    let tasks = crate::db_io_task_retirement_batch();
    let writer_more = matches!(writer, Some(Ok(DbIoWriterReleaseStep::More | DbIoWriterReleaseStep::Faulted)));
    match crate::db_io_backend_retirement_turn(backend, &mut Context::from_waker(&waker)) {
        DbIoBackendRetirementTurn::Retired => WorkerMaintenanceStep::Retire,
        DbIoBackendRetirementTurn::More => WorkerMaintenanceStep::More,
        DbIoBackendRetirementTurn::Idle if writer_more || tasks => WorkerMaintenanceStep::More,
        DbIoBackendRetirementTurn::Idle => WorkerMaintenanceStep::Idle,
    }
}

/// 📣️ Coalesces one retirement request into the backend's pre-admitted maintenance hook; it
/// never allocates, never queues a closure, and cannot be lost to lane contention.
pub(crate) fn request_retirement(backend: DbIoBackendControl) -> Result<bool, DbError> {
    let (slot, _) = db_io_backend_parts(backend);
    let row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(row) = row.as_ref().filter(|row| row.backend == backend) else { return Ok(false) };
    let Some(ticket) = row.ticket else { return Ok(false) };
    row.pool.request_maintenance(ticket).map(|request| request == WorkerMaintenanceRequest::Requested).map_err(|error| DbError::Unavailable(format!("DB I/O backend retirement request refused: {error:?}")))
}

/// 🪦️ Runs inside the backend's own hook turn: the hook returns `Retire`, so its slot is released
/// by the pool without an external removal racing the running invocation.
pub(crate) fn retire_controller(backend: DbIoBackendControl) -> Result<bool, DbError> {
    let (slot, _) = db_io_backend_parts(backend);
    let mut row = WAL_WRITER_CONTROLLERS[usize::from(slot)].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(owner) = row.as_mut() else { return Ok(true) };
    if owner.backend != backend {
        return Err(DbError::Fenced { expected: db_io_backend_parts(owner.backend).1, actual: db_io_backend_parts(backend).1 });
    }
    if let Some(ticket) = owner.deferred_wake_ticket {
        if !owner.pool.remove_deferred_wake_partition(ticket).map_err(|error| DbError::Unavailable(format!("WAL writer deferred-wake retirement: {error:?}")))? {
            return Ok(false);
        }
        owner.deferred_wake_ticket = None;
    }
    *row = None;
    Ok(true)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs"]
mod db_io_retained_fixtures;
//#endregion 🧪️Tests
