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
        fs::write(&first, b"a").expect("first");
        fs::write(&second, b"b").expect("second");
        fs::write(&third, b"c").expect("third");
        cache.insert("first".into(), first).expect("insert first");
        cache.insert("second".into(), second).expect("insert second");
        cache.get("first");
        cache.insert("third".into(), third).expect("insert third");
        assert!(!cache.entries.contains_key("second"));
        assert!(cache.entries.contains_key("first"));
        assert!(cache.entries.contains_key("third"));
        let _ = fs::remove_dir_all(&root);
    }
}
