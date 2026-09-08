
use super::*;

#[semio_framework_async_macros::async_test]
async fn dimension_signature_compatible_checks_equality() {
    assert!(DimensionSignature::LENGTH.compatible(DimensionSignature::LENGTH));
    assert!(!DimensionSignature::LENGTH.compatible(DimensionSignature::LENGTH_3));
    assert!(!DimensionSignature::DIMENSIONLESS.compatible(DimensionSignature::LENGTH));
}

#[semio_framework_async_macros::async_test]
async fn cardinality_variants_and_satisfies() {
    let optional = Cardinality::optional();
    assert!(optional.satisfies(0));
    assert!(optional.satisfies(1));
    assert!(!optional.satisfies(2));
    let required = Cardinality::required();
    assert!(!required.satisfies(0));
    assert!(required.satisfies(1));
    let unbounded = Cardinality::unbounded();
    assert!(unbounded.satisfies(0));
    assert!(unbounded.satisfies(1_000));
}

#[semio_framework_async_macros::async_test]
async fn geometry_catalogue_default_primitives() {
    let primitives = part_2::GeometryCatalogue::default_primitives();
    let ids: Vec<&str> = primitives.iter().map(|p| p.id.as_str()).collect();
    assert_eq!(ids, vec!["box", "cylinder", "sphere"]);
}

#[semio_framework_async_macros::async_test]
async fn bounding_box_overlaps() {
    let a = part_2::BoundingBox::from_size(1.0, 1.0, 1.0);
    let touching = part_2::BoundingBox { min: [0.9, 0.9, 0.9], max: [1.9, 1.9, 1.9] };
    assert!(a.overlaps(touching, 0.0));
    let far = part_2::BoundingBox { min: [5.0, 5.0, 5.0], max: [6.0, 6.0, 6.0] };
    assert!(!a.overlaps(far, 0.0));
    assert!(a.overlaps(far, 10.0));
}

#[semio_framework_async_macros::async_test]
async fn script_limits_default_values() {
    let limits = part_5::ScriptLimits::default();
    assert_eq!(limits.max_steps, 10_000);
    assert_eq!(limits.max_recursion, 64);
    assert_eq!(limits.timeout_ms, 50);
}
