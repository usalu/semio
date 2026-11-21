//! 🧪 Reads the native file sink tests from the independently authored PDF bytes.

#[path = "sink.rs"]
mod sink;
use sink::NativeDownloadSink;
use std::{fs, io, path::{Path, PathBuf}};

fn path(name: &str) -> PathBuf {
    let root = PathBuf::from(std::env::var_os("SEMIO_WGPU_DOWNLOAD_TEST").unwrap());
    let dir = root.join(name);
    fs::create_dir_all(&dir).unwrap();
    dir.join("delivery.pdf")
}

fn staged(path: &Path) -> usize {
    fs::read_dir(path.parent().unwrap()).unwrap().flatten()
        .filter(|entry| entry.file_name().to_string_lossy().starts_with(".semio-save-")).count()
}

#[test]
fn native_save_keeps_old_target_until_exact_commit() {
    let target = path("accepted");
    let original = fs::read(target.parent().unwrap().parent().unwrap().join("in.pdf")).unwrap();
    fs::write(&target, b"previous").unwrap();
    let mut sink = NativeDownloadSink::create(&target, original.len() as u64).unwrap();
    for bytes in original.chunks(911) { sink.write_page(bytes).unwrap(); }
    let prepared = sink.prepare().unwrap();
    assert_eq!(fs::read(&target).unwrap(), b"previous");
    assert_eq!(staged(&target), 1);
    prepared.commit().unwrap();
    assert_eq!(staged(&target), 0);
    assert_eq!(fs::read(target).unwrap(), original);
}

#[test]
fn native_save_refusal_and_revocation_remove_partial_files() {
    let target = path("refused");
    fs::write(&target, b"previous").unwrap();
    let mut oversize = NativeDownloadSink::create(&target, 2).unwrap();
    assert_eq!(oversize.write_page(&[0,1,2]).unwrap_err().kind(), io::ErrorKind::InvalidData);
    drop(oversize);
    let empty = NativeDownloadSink::create(&target, 2).unwrap();
    assert_eq!(empty.prepare().err().unwrap().kind(), io::ErrorKind::UnexpectedEof);
    let mut revoked = NativeDownloadSink::create(&target, 2).unwrap();
    revoked.write_page(&[0,1]).unwrap();
    drop(revoked.prepare().unwrap());
    assert_eq!(fs::read(&target).unwrap(), b"previous");
    assert_eq!(staged(&target), 0);
}
