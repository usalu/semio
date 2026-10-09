# Runtime Schema IO Dependencies

Unused physical imports were removed; imports used only by test code were guarded with cfg(test). The following lexical candidates still need semantic ownership review; nested test modules can appear as candidates.

## ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs

`const SEMIO_DRAW_EXAMPLE_TEXT: &str = crate::standards::v1::subsets::any::io::text::snapshot::SEMIO_DRAW_EXAMPLE_TEXT;`

## ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-stroke/🦀️.rs

`use crate::standards::v1::subsets::any::io::{raster_image_pack_asset, semio_image_from_rgba8};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🔺️diff/🦀️.rs

`use crate::standards::v6_0::subsets::document::io::binary::diff::write_bytes_lp;`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🎨️paint-region/🦀️.rs

`impl protocol::MutationKind<TiffSnapshot, TiffMutation> for PaintRegionMutation {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

`impl MdSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs

`use crate::standards::v1_0::subsets::base::io::binary::diff::dec_children_diff_bin;`

`use crate::standards::v1_0::subsets::base::io::binary::diff::enc_children_diff_bin;`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🦀️.rs

`pub fn blank_epw_snapshot() -> EpwSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🦀️.rs

`pub fn blank_gif_snapshot() -> GifSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs

`pub fn register() {`

`pub fn register_pilot_languages() {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🦀️.rs

`pub fn register() {`

`pub fn register_pilot_languages() {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🦀️.rs

`pub fn blank_strict_pptx_snapshot()->crate::PptxSnapshot{`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs

`pub fn demo_pdf_snapshot() -> PdfSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🦀️.rs

`pub fn blank_gif_snapshot() -> GifSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs

`use crate::standards::v_ecma_376::subsets::base::io::{attribute_value, element_matches, expanded_element_name, namespace_scope, resolve_office_document_relationship, DRAWINGML_NAMESPACES, OFFICE_RELATIONSHIP_NAMESPACES, PRESENTATIONML_NAMESPACES};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🦀️.rs

`pub fn blank_pptx_snapshot() -> PptxSnapshot {`

`pub async fn demo_pptx_snapshot() -> PptxSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs

`use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::{retire_xml_document, XmlDocumentView};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs

`impl PptxSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🏗️construction/🦀️.rs

`use crate::standards::v_ecma_376::subsets::base::io::{     attr, attribute_value, element_matches, expanded_element_name, namespace_scope, resolve_office_document_relationship, DRAWINGML_NAMESPACES, OFFICE_RELATIONSHIP_NAMESPACES, PRESENTATIONML_NAMESPACES, SLIDE_CONTENT_TYPE, };`

`pub(crate) fn append_slide(snapshot: &mut PptxSnapshot) -> Result<(), String> {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs

`use crate::standards::v_ecma_376::subsets::base::io::namespaces::{apply_bindings, expanded_name, is_word_name, qualified_word_prefix, set_word_attr, word_attr, XML_NAMESPACE};`

`fn main_body_path(snapshot: &DocxSnapshot) -> Result<(String, Vec<usize>), ValueError> {`

`fn styles_part_path(snapshot: &DocxSnapshot) -> Result<String, ValueError> {`

`pub(super) fn replace_addressed_node(snapshot: &mut DocxSnapshot, address: &DocxXmlAddress, replacement: XmlNode) -> Result<(), ValueError> {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs

`fn edit_main_body(snapshot: &mut DocxSnapshot, edit: impl FnOnce(&mut Vec<XmlNode>) -> Option<()>) -> Option<()> {`

`fn edit_styles_root(snapshot: &mut DocxSnapshot, edit: impl FnOnce(&mut Vec<XmlNode>) -> Option<()>) -> Option<()> {`

`fn apply_to_snapshot(base: &DocxSnapshot, mutation: &DocxMutation) -> Option<DocxSnapshot> {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🦀️.rs

`pub async fn demo_docx_snapshot() -> DocxSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs

`use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::{XmlDocumentView,retire_xml_document};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs

`use crate::{     standards::v_ecma_376::subsets::base::io::{DocxError, REL_TYPE_STYLES, STRICT_REL_TYPE_OFFICE_DOCUMENT, STRICT_REL_TYPE_STYLES},     STDIO_DOCX_DOCUMENT_SCHEMA, };`

`impl Default for DocxSnapshot {`

`impl DocxSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs

`use crate::standards::v_ecma_376::subsets::base::io::{attribute_value, column_letter, element_matches, expanded_element_name, namespace_scope, REL_TYPE_OFFICE_DOCUMENT_STRICT, R_NS, R_NS_STRICT, SML_NS, SML_NS_STRICT};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️canonical-edit/🦀️.rs

`use crate::standards::v_ecma_376::subsets::base::io::export::serializers::worksheet_to_xml_with_namespace;`

`use crate::standards::v_ecma_376::subsets::base::io::{attribute_value, element_matches, expanded_element_name, namespace_scope, REL_TYPE_WORKSHEET, R_NS, R_NS_STRICT, SML_NS, SML_NS_STRICT, WORKSHEET_CONTENT_TYPE};`

`fn cell_column(node: &XmlNode, scope: &[(String, String)]) -> Result<Option<u32>, String> {`

`fn insert_addressed_cell(snapshot: &mut XlsxSnapshot, address: &cell_address::XlsxCellVacancyAddress, value: &XlsxCellValue) -> Result<(), String> {`

`fn shared_strings_path(snapshot: &XlsxSnapshot) -> Result<String, String> {`

`fn rename_sheet(snapshot: &mut XlsxSnapshot, old_name: &str, new_name: &str) -> Result<(), String> {`

`fn workbook_path(snapshot: &XlsxSnapshot) -> Result<String, String> {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🦀️.rs

`pub fn demo_xlsx_snapshot() -> XlsxSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs

`use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::{retire_xml_document, XmlDocumentView};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs

`impl Default for XlsxSnapshot {`

`impl XlsxSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs

`pub(crate) fn agg_diff(this: &DwgMutation, base: &DwgSnapshot) -> protocol::MutationOutcome<DwgDiff> {`

`pub(crate) fn version_sentinel_check(snapshot: &DwgSnapshot) -> Result<(), String> {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐dimensions/🦀️.rs

`pub fn compute_png_dimensions(snapshot: &PngSnapshot) -> PngDimensions {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs

`impl MutationDiff<PngSnapshot> for PngDiff {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-native-samples/🦀️.rs

`use crate::standards::v1_2::subsets::any::io::{PngNativePaint, PngRegion};`

`impl protocol::MutationKind<PngSnapshot, PngMutation> for PaintNativeSamplesMutation {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️change-gamma/🦀️.rs

`impl protocol::MutationKind<PngSnapshot, PngMutation> for ChangeGammaMutation {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-pixels/🦀️.rs

`impl protocol::MutationKind<PngSnapshot, PngMutation> for PatchPixelsMutation {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🦀️.rs

`pub fn demo_png_snapshot() -> PngSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

`impl Default for PngSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs

`use crate::standards::v1_7::subsets::base::io::carry_graph_edit;`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs

`pub fn blank_pdf_snapshot() -> PdfSnapshot {`

`pub fn demo_pdf17_snapshot() -> PdfSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🏅️conformance-support/🦀️.rs

`use crate::standards::v1_7::subsets::base::io::carry_graph_edit;`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs

`impl MutationDiff<ZipSnapshot> for ZipDiff {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/💡️inferences/🏷️kind/🦀️.rs

`use crate::standards::v1::subsets::base::io::binary::snapshot::{subset_ordinal};`

`use crate::standards::v1::subsets::base::io::text::snapshot::{subset_tag};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🦀️.rs

`pub fn semio_subset_tag(snapshot: &SemioSnapshot) -> &'static str {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/📦️codec/🫳️borrowed/🦀️.rs

`use protocol::io::binary::operation_bytes::{OperationByteOutput,OperationByteLimitedOutput};`

`impl SemioFlowMutation{`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/✅validation-report/🦀️.rs

`use crate::standards::v1::subsets::brep::io::check_brep_referential_integrity;`

## ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs

`use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::{circle_normal_form, compose_affine, flatten_segments, semio_transform_affine};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🦀️.rs

`pub fn blank_jpg_snapshot() -> JpgSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/🦀️.rs

`use crate::standards::v1_1::subsets::base::io::text::snapshot::{parse_transform_list};`

`use crate::standards::v1_1::subsets::base::io::text::snapshot::{parse_view_box};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/🧬️schema/🧬️mutations/🦀️.rs

`use crate::standards::v1_1::subsets::base::io::text::snapshot::{parse_transform_list};`

`use crate::standards::v1_1::subsets::base::io::text::snapshot::{parse_view_box};`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🦀️.rs

`pub fn demo_svg_snapshot() -> SvgSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐dimensions/🦀️.rs

`pub fn compute_bmp_dimensions(snapshot: &BmpSnapshot) -> BmpDimensions {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs

`impl MutationDiff<BmpSnapshot> for BmpDiff {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-indexed-region/🦀️.rs

`impl protocol::MutationKind<BmpSnapshot, BmpMutation> for PaintIndexedRegion {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-direct-region/🦀️.rs

`impl protocol::MutationKind<BmpSnapshot, BmpMutation> for PaintDirectRegion {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🦀️.rs

`pub fn demo_bmp_snapshot() -> BmpSnapshot {`

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

`impl Default for BmpSnapshot {`
