use super::*;

#[semio_framework_async_macros::async_test]
async fn normalize_pair_orders_endpoints() {
    let a = EntityId("element-2".into());
    let b = EntityId("element-10".into());
    assert_eq!(normalize_pair(&b, &a), (b, a));
}
