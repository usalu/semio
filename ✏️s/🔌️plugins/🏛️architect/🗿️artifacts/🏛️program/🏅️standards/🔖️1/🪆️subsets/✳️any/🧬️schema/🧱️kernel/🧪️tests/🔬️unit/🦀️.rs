
use super::*;

#[semio_framework_async_macros::async_test]
async fn entity_id_orders_lexicographically() {
    let a = EntityId("element-2".into());
    let b = EntityId("element-10".into());
    assert!(a > b);
}

#[semio_framework_async_macros::async_test]
async fn entity_id_serial_increments() {
    let first = EntityId::new_serial("test", "test");
    let second = EntityId::new_serial("test", "test");
    assert_ne!(first, second);
    assert!(first.to_string().starts_with("test-"));
}
