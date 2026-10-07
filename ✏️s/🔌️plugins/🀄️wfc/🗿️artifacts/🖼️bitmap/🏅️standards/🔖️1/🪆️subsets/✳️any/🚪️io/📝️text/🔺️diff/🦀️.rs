//! 🔺️ Bitmap physical diff codec.

/// 🔺️ Physical sparse delta text encoding.
impl protocol::os_spr::DiffText for crate::schema::diff::BitmapDiff {
    fn print_diff(&self) -> String { crate::standards::v1::subsets::any::io::text::bitmap_json_encode(self) }
    fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { crate::standards::v1::subsets::any::io::text::bitmap_json_decode(text).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1))) }
}
