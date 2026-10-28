//! 🧭️ Encoded DXF analysis preserves original source bytes until genuine retained identity and parser preparation.
use crate::interchange::{LayoutImportSourcePlan,InterchangeControl};
/// 📥️ This cold source plan is never a published parent snapshot or inline child payload.
pub fn analyze_source(bytes:&[u8],name:&str,control:&mut InterchangeControl<'_>)->Result<LayoutImportSourcePlan,String>{crate::layout_import_sources_from_format("dxf",bytes,name,control)}
