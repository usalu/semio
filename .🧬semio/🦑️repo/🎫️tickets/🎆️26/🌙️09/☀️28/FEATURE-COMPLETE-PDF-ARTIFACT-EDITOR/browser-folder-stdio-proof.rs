//! 🧪 Harness around the exact native companion, folder event append and paid capture.
#![allow(dead_code)]
extern crate self as semio_framework_async;
extern crate self as semio_os_spr;
extern crate self as semio_os_store_sync_native;
extern crate self as semio_os_vcs;

mod disk { include!("disk.rs"); }
mod archive_readback { include!("archive_readback.rs"); }
mod receipt { include!("receipt.rs"); }
mod first_archive_exchange { include!("first_archive_exchange.rs"); }
mod entry { include!("entry.rs"); }
pub use first_archive_exchange::FirstArchiveKey;
pub mod wire { pub mod command_ingress { include!("wire_records.rs"); } }
pub mod channel {
    pub use super::{AppDocumentIdentity, DOCUMENT_ARCHIVE_MAXIMUM_BYTES};
    pub use super::receipt::*;
    pub const DOCUMENT_ARCHIVE_VERSION: u8 = 1;
}
include!("folder.rs");

pub const DOCUMENT_ARCHIVE_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppDocumentIdentity { pub app_instance_id: u32, pub base_revision: u64, pub parent_document_id: Option<String> }

#[derive(Debug)]
pub enum VcsError { Backbone(String) }
impl std::fmt::Display for VcsError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::Backbone(detail) => out.write_str(detail) }
    }
}
impl std::error::Error for VcsError {}
async fn now_ms() -> u64 { 1 }
fn is_repository_root(_: &std::path::Path) -> Result<bool, String> { Ok(false) }

pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    let mut pinned = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop { if let std::task::Poll::Ready(output) = pinned.as_mut().poll(&mut context) { return output; } }
}

pub struct Archive { pub parent_pack: Vec<u8>, pub parent_spr: Vec<u8> }
pub async fn decode_document_archive_bytes(bytes: &[u8]) -> Result<Archive, VcsError> {
    if bytes.first() != Some(&1) || bytes.len() > DOCUMENT_ARCHIVE_MAXIMUM_BYTES {
        return Err(VcsError::Backbone("fixture ordinary archive header is invalid".into()));
    }
    Ok(Archive { parent_pack: Vec::new(), parent_spr: Vec::new() })
}
pub fn encode_document_archive_bytes(_: &Archive) -> Result<Vec<u8>, VcsError> {
    Err(VcsError::Backbone("fixture does not replace document pairs".into()))
}
fn main() -> std::process::ExitCode {
    if std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("hold")) {
        use std::io::{Read, Write};
        let lock = std::fs::OpenOptions::new().read(true).write(true).open(std::env::args_os().nth(2).unwrap()).unwrap();
        lock.try_lock().unwrap();
        std::io::stdout().write_all(b"ready").unwrap();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_exact(&mut [0]).unwrap();
        return std::process::ExitCode::SUCCESS;
    }
    entry::child_main()
}
