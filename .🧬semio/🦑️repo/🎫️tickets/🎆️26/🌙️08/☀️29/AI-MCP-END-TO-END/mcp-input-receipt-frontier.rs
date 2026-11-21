use semio_framework_actor::{OriginalActorTurnWork, OriginalJobBudget, RetainedTurnInput};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_guest_contract::original_actor_context::{
    OriginalActorBootstrapCall, OriginalActorBootstrapStatus, OriginalActorBootstrapTransitionCall,
    OriginalActorFixedCall, OriginalActorFixedKind, OriginalActorFixedReceipt,
    OriginalActorGuestEntry, OriginalEscrowExtent, OriginalEscrowSnapshot,
};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

#[cfg(not(target_arch = "wasm32"))]
mod low_heap {
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
const REGION: usize = 64 << 20;
static BASE: AtomicUsize = AtomicUsize::new(0);
static CURSOR: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" {
    fn mmap(address: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut u8;
}

struct LowHeap;

#[global_allocator]
static HEAP: LowHeap = LowHeap;

unsafe impl GlobalAlloc for LowHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let base = BASE.load(Relaxed);
        if base == 0 {
            return unsafe { System.alloc(layout) };
        }
        let start_at = |cursor: usize| (base + cursor).next_multiple_of(layout.align());
        let start = CURSOR.try_update(Relaxed, Relaxed, |cursor| {
            let end = start_at(cursor).checked_sub(base)?.checked_add(layout.size())?;
            (end <= REGION).then_some(end)
        });
        start.map_or(std::ptr::null_mut(), |cursor| start_at(cursor) as *mut u8)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let base = BASE.load(Relaxed);
        if base == 0 || !(base..base + REGION).contains(&(ptr as usize)) {
            unsafe { System.dealloc(ptr, layout) };
        }
    }
}

pub fn init() {
    let base = unsafe { mmap(0x4000_0000 as *mut u8, REGION, 3, 0x1002, -1, 0) } as usize;
    assert!(base > 0 && base < u32::MAX as usize - REGION, "low heap absent at {base:#x}");
    BASE.store(base, Relaxed);
}
pub fn verify(pointer: usize, length: usize) {
    let base = BASE.load(Relaxed);
    assert!(base <= pointer && pointer.checked_add(length).unwrap() <= base + REGION);
}
}

static CLOCK_BINDING: AtomicU64 = AtomicU64::new(0);

fn frame(pointer: usize, length: usize) -> &'static [u8] {
    #[cfg(not(target_arch = "wasm32"))]
    low_heap::verify(pointer, length);
    unsafe { std::slice::from_raw_parts(pointer as *const u8, length) }
}

fn extent(word: u64) -> (usize, usize) {
    (word as u32 as usize, (word >> 32) as usize)
}

fn observe(escrow: OriginalEscrowExtent) -> OriginalEscrowSnapshot {
    OriginalEscrowSnapshot::read(frame(escrow.pointer as usize, escrow.length as usize)).unwrap()
}

fn original_policy() -> serde_json::Value {
    serde_json::from_str(include_str!(env!("MCP_HOST_DRIVER_POLICY"))).unwrap()
}

fn reissue(input: &mut RetainedTurnInput, spent: RetainedCloneProgress, work: u64) {
    let returned = input.return_original(spent, work).unwrap();
    input.epoch += 1;
    input.grant = returned.remaining;
    input.work.fuel = returned.remaining_fuel;
}

fn main() {
    let policy = original_policy();
    let operation: RetainedCloneGrant = serde_json::from_value(policy["operation"].clone()).unwrap();
    let admission: RetainedCloneGrant = serde_json::from_value(policy["admission"].clone()).unwrap();
    let drive: RetainedCloneGrant = serde_json::from_value(policy["drive"].clone()).unwrap();
    let fuel = policy["root"]["turnFuel"].as_u64().unwrap();
    let root = &policy["root"];
    #[cfg(not(target_arch = "wasm32"))]
    low_heap::init();
    let mut input = RetainedTurnInput {
        operation: root["operation"].as_u64().unwrap(),
        generation: root["generation"].as_u64().unwrap(),
        epoch: 1,
        work: OriginalActorTurnWork {
            clock_binding: 71,
            absolute_deadline_us: root["maximumLifetimeMicroseconds"].as_u64().unwrap(),
            fuel: root["operationFuel"].as_u64().unwrap(),
        },
        grant: operation,
    };
    let clock = || Some(1);
    let budget = |input: RetainedTurnInput, policy| OriginalJobBudget {
        input, retained_policy: policy, fuel, deadline_us: 2_000_000, cancelled: false,
    };
    let mut entry = std::mem::ManuallyDrop::new(Box::new(OriginalActorGuestEntry::new()));
    let mut boot_ready = false;
    for _ in 0..64 {
        let call = budget(input, admission);
        let word = entry.admit_owned_header_step(
            call, clock, || CLOCK_BINDING.load(Relaxed), |binding| CLOCK_BINDING.store(binding, Relaxed),
            |_| Ok(()),
        ).expect("bootstrap authority").expect("framed bootstrap");
        let (pointer, length) = extent(word);
        if length == 304 {
            let reply = OriginalActorBootstrapCall::read(frame(pointer, length)).unwrap();
            reply.validate_expected(call).unwrap();
            assert!(!matches!(reply.status, OriginalActorBootstrapStatus::Failed(_)));
            assert!(entry.release_bootstrap_header(pointer, length).unwrap());
            reissue(&mut input, reply.actual.progress, reply.actual.work);
            continue;
        }
        assert_eq!(word, entry.expose_source_header().unwrap());
        let (reply, escrow) = if length == 200 {
            let transition = OriginalActorBootstrapTransitionCall::read(frame(pointer, length)).unwrap();
            (transition.call, transition.escrow)
        } else {
            OriginalActorFixedReceipt::read(frame(pointer, length)).unwrap()
        };
        let snapshot = observe(escrow);
        println!("[DEBUG] bootstrap epoch {} rows {} revision {} acknowledgement {} {:?}", input.epoch,
            snapshot.len(), snapshot.revision, snapshot.acknowledged_revision, reply.kind);
        assert!(entry.release_source_header(pointer, length).unwrap());
        reissue(&mut input, reply.spent, reply.work_units_spent);
        if matches!(reply.kind, OriginalActorFixedKind::Admission { ready: true, refusal: None }) {
            boot_ready = true;
            break;
        }
    }
    assert!(boot_ready);
    let wire = b"{}";
    let pending = OriginalActorFixedKind::InputAdmission { ready: false, input_returned: false, refusal: None };
    for attempt in 1..=3 {
        let result = entry.dispatch_actor_input_header_step(wire, false, input, budget(input, drive).step_budget(), clock,
            |_| Ok(None), |cx| {
                eprintln!("[DEBUG] Input {attempt} producer entered at fuel {} with {:?}", cx.fuel_remaining(), cx.retained_grant());
                Ok(pending)
            });
        let word = result.expect("input authority").unwrap_or_else(|| panic!("input receipt {attempt} unframed, fault {:?}", entry.original_job_refusal()));
        assert_eq!(word, entry.expose_source_header().unwrap());
        let (pointer, length) = extent(word);
        let (reply, escrow): (OriginalActorFixedCall, _) = OriginalActorFixedReceipt::read(frame(pointer, length)).unwrap();
        let snapshot = observe(escrow);
        println!("[DEBUG] input {attempt} epoch {} rows {} revision {} acknowledgement {} {:?} work {}",
            input.epoch, snapshot.len(), snapshot.revision, snapshot.acknowledged_revision, reply.kind, reply.work_units_spent);
        assert!(matches!(reply.kind, OriginalActorFixedKind::InputAdmission { ready: false, input_returned: false, refusal: None }));
        assert!(entry.release_source_header(pointer, length).unwrap());
        reissue(&mut input, reply.spent, reply.work_units_spent);
    }
}
