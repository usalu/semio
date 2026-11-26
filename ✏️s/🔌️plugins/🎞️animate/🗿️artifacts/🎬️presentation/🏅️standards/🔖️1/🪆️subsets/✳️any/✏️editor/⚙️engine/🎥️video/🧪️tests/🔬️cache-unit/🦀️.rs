mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[semio_framework_async_macros::async_test]
    async fn segment_hash_is_stable() {
        let a = PartialMovieLut::segment_hash("abc", 0, 10);
        let b = PartialMovieLut::segment_hash("abc", 0, 10);
        assert_eq!(a, b);
        assert_ne!(a, PartialMovieLut::segment_hash("abc", 0, 11));
    }

    #[semio_framework_async_macros::async_test]
    async fn lru_evicts_oldest_entry() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("animate_cache_lru_{stamp}"));
        let _ = fs::remove_dir_all(&root);
        let mut cache = PartialMovieLut::open_with_limit(&root, 2).expect("open");
        let first = root.join("first.mp4");
        let second = root.join("second.mp4");
        let third = root.join("third.mp4");
        let keys = ["first", "second", "third"].map(|name| PartialMovieLut::segment_hash(name, 0, 1));
        fs::write(&first, b"a").expect("first");
        fs::write(&second, b"b").expect("second");
        fs::write(&third, b"c").expect("third");
        cache.insert(keys[0].clone(), &first).expect("insert first");
        cache.insert(keys[1].clone(), &second).expect("insert second");
        cache.get(&keys[0]).expect("touch first");
        cache.insert(keys[2].clone(), &third).expect("insert third");
        drop(cache);
        let mut reopened = PartialMovieLut::open_with_limit(&root, 2).expect("reopen");
        assert!(reopened.get(&keys[1]).is_none());
        assert_eq!(fs::read(reopened.get(&keys[0]).expect("first by full key")).unwrap(), b"a");
        assert_eq!(fs::read(reopened.get(&keys[2]).expect("third by full key")).unwrap(), b"c");
        assert_eq!(fs::read(second).unwrap(), b"b", "only the cached link was evicted");
        let _ = fs::remove_dir_all(&root);
    }

    #[semio_framework_async_macros::async_test]
    async fn reopen_evicts_the_movie_with_the_older_last_access() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("animate_cache_reopen_{stamp}"));
        let scratch = MovieScratch::new(&root).expect("render dir");
        let mut cache = PartialMovieLut::open(&root).expect("open");
        let keys = ["recent", "old"].map(|name| PartialMovieLut::segment_hash(name, 0, 1));
        for key in &keys {
            let movie = scratch.file(key);
            fs::write(&movie, b"movie").expect("fixture");
            cache.insert(key.clone(), &movie).expect("publish");
        }
        let older = cache.path_for(&keys[1]).expect("path");
        fs::OpenOptions::new().write(true).open(older).unwrap().set_modified(UNIX_EPOCH + std::time::Duration::from_secs(1_000_000_000)).unwrap();
        cache.get(&keys[0]).expect("recent hit");
        drop(cache);
        let mut reopened = PartialMovieLut::open_with_limit(&root, 1).expect("reopen");
        assert!(reopened.get(&keys[0]).is_some());
        assert!(reopened.get(&keys[1]).is_none());
        drop(scratch);
        let _ = fs::remove_dir_all(root);
    }

    #[semio_framework_async_macros::async_test]
    async fn simultaneous_cache_publications_leave_one_complete_movie() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("animate_cache_duplicate_{stamp}"));
        let scratch = MovieScratch::new(&root).expect("render dir");
        let key = PartialMovieLut::segment_hash("same", 0, 2);
        let candidates = [vec![0x11; 4096], vec![0x22; 4096]];
        let barrier = std::sync::Barrier::new(candidates.len());
        std::thread::scope(|scope| {
            for (index, bytes) in candidates.iter().enumerate() {
                let source = scratch.file(format!("{index}.mp4"));
                let mut cache = PartialMovieLut::open(&root).expect("open");
                let (barrier, key) = (&barrier, key.clone());
                scope.spawn(move || {
                    fs::write(&source, bytes).expect("complete source");
                    barrier.wait();
                    cache.insert(key, &source).expect("atomic publish");
                });
            }
        });
        let mut cache = PartialMovieLut::open(&root).expect("reopen");
        let published = fs::read(cache.get(&key).expect("full key")).expect("movie");
        assert!(candidates.contains(&published));
        assert_eq!(fs::read_dir(&root).unwrap().filter_map(Result::ok).filter(|entry| cache_key(&entry.path()).is_some()).count(), 1);
        assert_eq!(PartialMovieLut::flush(&root).expect("flush cache"), 1);
        assert!(scratch.file("0.mp4").is_file(), "cache flush leaves the active render's scratch alone");
        drop(scratch);
        let _ = fs::remove_dir_all(root);
    }
}
