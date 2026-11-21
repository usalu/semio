//! 🪪️ Lends the sealed DCT operation to the existing Original operation Pack projection.
use semio_framework_dsl_record::{
    BorrowedFieldSpec, BorrowedRecordSpec, BorrowedShape, RecordLayout,
    native_encoding::{FieldProjectionSource, FieldProjectionView, projection_path_error},
};
use semio_framework_value::{ValueError, paged::ByteReadSource};
use semio_os_store_one_item_publication::lifted_prepared_operation_wire::ArtifactPreparedOperationSource;
use super::placement_wire::DCT_INSTALL_PLACEMENT_TAG;

const FIELDS: &[BorrowedFieldSpec] = &[BorrowedFieldSpec::new(1, "sealedPlacement", BorrowedShape::Bytes64)];

pub struct PdfDctPlacementOperation<'a> {
    sealed: &'a dyn ByteReadSource,
}

impl<'a> PdfDctPlacementOperation<'a> {
    /// 🧷️ Keeps the same admitted original operation payload alive through its native wire projection.
    pub fn from_sealed(sealed: &'a dyn ByteReadSource) -> Self { Self { sealed } }

    /// 🗃️ Produces the domain override for the store's original operation wire source.
    pub fn wire(&self) -> ArtifactPreparedOperationSource<'_> {
        ArtifactPreparedOperationSource::Pack {
            tag: DCT_INSTALL_PLACEMENT_TAG,
            body: self,
            spec: BorrowedRecordSpec { keyword: None, layout: RecordLayout::Lines, fields: FIELDS },
        }
    }
}

impl FieldProjectionSource for PdfDctPlacementOperation<'_> {
    fn projection_identity(&self) -> usize { self.sealed as *const dyn ByteReadSource as *const () as usize }

    fn projection_view(&self, path: &[usize]) -> Result<FieldProjectionView<'_>, ValueError> {
        match path {
            [] => Ok(FieldProjectionView::Record(&[1])),
            [0] => Ok(FieldProjectionView::BytesSource(self.sealed)),
            _ => Err(projection_path_error()),
        }
    }
}
