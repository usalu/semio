use std::sync::Arc;

use super::common::drain;
use crate::kit::KitStore;
use crate::KitStoreRef;

#[test]
fn subscribe_receives_ordered_stream() {
    let kit: KitStoreRef = Arc::new(std::sync::RwLock::new(KitStore::new("a")));
    let mut a = kit.read().unwrap().subscribe();
    let mut b = kit.read().unwrap().subscribe();
    kit.write().unwrap().set_name("b".into());
    let ea = drain(&mut a);
    let eb = drain(&mut b);
    assert_eq!(ea, eb);
    assert!(!ea.is_empty());
}

#[test]
fn no_lock_held_across_concurrent_read_and_async_setter() {
    let kit: KitStoreRef = Arc::new(std::sync::RwLock::new(KitStore::new("c")));
    let k2 = kit.clone();
    futures_lite::future::block_on(async {
        let _ = futures_lite::future::zip(
            crate::KitStore::set_name_async(&k2, "d".into()),
            async { k2.read().map(|_| ()).unwrap_or(()) },
        )
        .await;
    });
    assert_eq!(kit.read().unwrap().name, "d");
}

#[test]
fn drop_kit_closes_bus() {
    let kit: KitStoreRef = Arc::new(std::sync::RwLock::new(KitStore::new("e")));
    let mut rx = kit.read().unwrap().subscribe();
    drop(kit);
    let r = futures_lite::future::block_on(rx.recv());
    assert_eq!(r, Err(async_broadcast::RecvError::Closed));
}
