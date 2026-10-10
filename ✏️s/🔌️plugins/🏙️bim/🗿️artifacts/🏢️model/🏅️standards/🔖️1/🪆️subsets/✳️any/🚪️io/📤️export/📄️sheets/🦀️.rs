//! 📄️ `s.bim.model@1/*` → sheets: the authored sheets of a [`ModelSnapshot`] as paper-size documents of the existing stdio dialects, written from the inferred `sheet-layout` and `view-linework`: one SVG 1.1 file per sheet
//! (`s.stdio.svg@1.1/*`, see [`svg`]) and the sheet set as one PDF 1.7 (`s.stdio.pdf@1.7/*`, one page per sheet, see [`pdf`]). Both draw the same marks ([`ink`]) with the headings of the language they are given.
//! 🔖 `IoFidelity::Lossy`: a sheet shows cuts and projections of the model, never the model.
//! 📎 https://www.w3.org/TR/SVG11/, https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf

use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::SheetLayout;
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry;
use crate::{ModelInference, ModelSnapshot};
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use std::collections::BTreeMap;

#[path = "✒️ink/🦀️.rs"]
pub mod ink;
#[path = "📖️pdf/🦀️.rs"]
pub mod pdf;
#[path = "🎨️svg/🦀️.rs"]
pub mod svg;
#[path = "📊️tables/🦀️.rs"]
pub mod tables;

pub use ink::TitleLabels;

/// 🪪️ The PDF 1.7 dialect the sheet set is written in.
pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId::ANY };

/// 🔢️ The ids of the sheets of `model` in print order: by sheet number, then id.
pub fn ordered(model: &ModelSnapshot) -> Vec<String> {
    let mut sheets: Vec<(&String, &String)> = model.sheets.iter().map(|(id, sheet)| (&sheet.number, id)).collect();
    sheets.sort();
    sheets.into_iter().map(|(_, id)| id.clone()).collect()
}

/// 🖼️ The drawings of the views by view id.
pub fn drawings(inferred: &ModelInference) -> BTreeMap<&str, &ViewLinework> {
    inferred.view_linework.iter().map(|(id, drawing)| (id.as_str(), drawing)).collect()
}

/// 📄️ The layouts of the sheets `only` names, or of all sheets in print order; a sheet without a layout (a fault of the inference) is left out.
pub fn layouts<'a>(model: &ModelSnapshot, inferred: &'a ModelInference, only: Option<&str>) -> Vec<&'a SheetLayout> {
    ordered(model).iter().filter(|id| only.is_none_or(|wanted| wanted == id.as_str())).filter_map(|id| inferred.sheet_layouts.get(id)).collect()
}

/// 🏷️ The stem of the file name of a sheet: its number with every run of characters other than letters and digits turned into one dash.
pub fn file_stem(number: &str) -> String {
    let mut stem = String::new();
    for letter in number.chars() {
        if letter.is_alphanumeric() {
            stem.push(letter);
        } else if !stem.ends_with('-') && !stem.is_empty() {
            stem.push('-');
        }
    }
    let stem = stem.trim_end_matches('-');
    if stem.is_empty() { "sheet".to_string() } else { stem.to_string() }
}

/// 🎨️ The SVG of the sheet `sheet`, none when the model has no layout for it.
pub fn sheet_svg(model: &ModelSnapshot, inferred: &ModelInference, sheet: &str, labels: &TitleLabels) -> Option<Result<String, String>> {
    inferred.sheet_layouts.get(sheet).map(|layout| svg::sheet_svg(model, layout, &drawings(inferred), labels))
}

/// 📖️ The PDF of the sheets `only` names, or of the whole set, one page per sheet.
pub fn sheets_pdf(model: &ModelSnapshot, inferred: &ModelInference, only: Option<&str>, labels: &TitleLabels) -> Result<Vec<u8>, String> {
    let title = if model.project.name.is_empty() { "BIM model" } else { model.project.name.as_str() };
    pdf::sheets_pdf(title, &layouts(model, inferred, only), &drawings(inferred), labels)
}

//#region 🔖️Serializer
/// 📖️ The PDF 1.7 serializer of the sheet set of the BIM model: one page per sheet in print order, English headings.
pub struct ModelIntoSheetsPdf;

impl Serializer<ModelSnapshot> for ModelIntoSheetsPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let bytes = registry::try_with_inference(None, from, |inferred| sheets_pdf(from, inferred, None, &TitleLabels::english()))
            .map_err(|error| error.to_string())
            .and_then(|written| written)
            .map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ModelIntoSheetsPdf: {message}"))))?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
