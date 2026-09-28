/// ⚖️ LAW: the document-IO route's wire ceiling is DERIVED from the bounds that actually bind, and
/// each one is reachable.
///
/// The framework reassembles a picked file before `importDocument` runs
/// (`semio_framework::kernel::ImportStaging`), so the route carries ONE whole import. An imported file
/// is planted in the graph as an `InputNote`'s text, so one import is bounded by one Artifact-lane
/// edit; the route's body must hold that import plus its envelope, and the framework staging must be
/// able to reassemble every import the route admits.
#[test]
fn document_io_route_declares_a_reachable_wire_ceiling() {
    use crate::editor::generation3d::commands::import_document::GENERATION3D_IMPORT_TOTAL_BYTES;
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    assert_eq!(GENERATION3D_IMPORT_TOTAL_BYTES, GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES, "the import budget is one Artifact-lane edit, never a literal");
    assert_eq!(GENERATION3D_DOCUMENT_IO_RAW_BYTES, semio_framework::PUBLIC_INVOCATION_BODY_BYTES);
    assert_eq!(generation3d_document_io_contract().max_raw_wire_bytes, GENERATION3D_DOCUMENT_IO_RAW_BYTES, "the registered contract and the factory-side cap are one bound");
    assert!(GENERATION3D_IMPORT_TOTAL_BYTES < GENERATION3D_DOCUMENT_IO_RAW_BYTES, "one whole import plus its envelope has to fit the route's body");
    assert!(GENERATION3D_IMPORT_TOTAL_BYTES <= semio_framework::kernel::IMPORT_STAGING_MAXIMUM_BYTES, "the framework staging reassembles every import this route admits");
    assert_eq!(generation3d_document_io_contract().shape, semio_framework::ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation3d_document_io_contract().cancellation, semio_framework::ToolCancellationPolicy::PerOperation, "an import run is cancellable per operation");
}
