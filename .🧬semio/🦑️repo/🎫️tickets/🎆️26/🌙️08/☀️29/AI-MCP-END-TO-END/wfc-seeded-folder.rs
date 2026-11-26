use semio_os_store_sync_native::FolderEventLogStorage;
use std::{env, fs, path::PathBuf};

fn main() {
    let mut arguments = env::args().skip(1);
    let [operation, folder, id, pack_path, spr_path] = arguments.by_ref().take(5).collect::<Vec<_>>()
        .try_into().expect("operation, folder, artifact ID, Pack and SPR required");
    let kind = arguments.next().unwrap_or_else(|| "s.wfc.bitmap".into());
    let (pack, spr) = (fs::read(pack_path).unwrap(), fs::read(spr_path).unwrap());
    let folder = PathBuf::from(folder);
    assert!(folder.is_dir());
    let storage = FolderEventLogStorage::new(folder);
    semio_framework_async::block_on(async {
        if operation == "seed" {
            assert!(storage.write_if_current(&id, &kind, None, &pack, &spr).await.unwrap());
        } else {
            assert_eq!(operation, "verify");
        }
        let (observed_pack, observed_spr, archive) = storage.read_snapshot(&id).await.unwrap().unwrap();
        assert_eq!(observed_pack, pack);
        assert_eq!(observed_spr, spr);
        assert!(archive.is_none());
        println!("[DEBUG] {operation}: exact persisted Pack={} and SPR={} bytes", pack.len(), spr.len());
    });
}
