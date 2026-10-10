//! 🎨️ `s.bim.model@1/*` → `s.stdio.svg@1.1/*`: the authored views of a [`ModelSnapshot`] as one SVG 1.1 sheet in paper millimetres, each view at its own scale (1:100 by default).
//! One `<g class="view <kind>">` per view that has a drawing (id `view-<id>`, title, kind, scale and, for a plan, its storey): the plans, ceiling plans, sections and elevations, stacked top-down per building; a camera view draws nothing and is not exported.
//! Inside a group the poché regions, the stroked lines in one class per line style (cut heavy, projection medium, hidden dashed, annotation fine) and the texts (space tags, grid labels, datum labels). The linework is the one of the `view-linework`
//! inference, in view coordinates (building metres for a plan, `(u, z)` for a section or elevation); the viewBox fits the sheet.
//! 🔖 `IoFidelity::Lossy`: a drawing is a cut or a projection; everything the model knows beyond lines and labels is dropped.
//! The tree is the typed SVG model of the `s.stdio.svg` artifact and the text is written by the XML writer of `s.stdio.xml`, both behind [`codec`].
//! 📎 https://www.w3.org/TR/SVG11/

use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry;
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use crate::ModelSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use std::collections::BTreeMap;

#[path = "🧱️codec/🦀️.rs"]
pub mod codec;
#[path = "✒️path/🦀️.rs"]
pub mod path;
#[path = "🎚️style/🦀️.rs"]
pub mod style;
#[path = "📐️sheet/🦀️.rs"]
pub mod sheet;
#[path = "🖍️drawing/🦀️.rs"]
pub mod drawing;
#[path = "📏️projection/🦀️.rs"]
pub mod projection;

/// 🪪️ The SVG 1.1 dialect this leaf writes.
pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

const NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// 🎨️ The SVG document of `model` from already inferred view drawings.
pub fn views_to_svg(model: &ModelSnapshot, drawings: &BTreeMap<String, ViewLinework>) -> Result<String, String> {
    let layout = sheet::layout(model, drawings);
    let title = if model.project.name.is_empty() { "BIM model" } else { model.project.name.as_str() };
    let (width, height) = (path::snap(layout.width), path::snap(layout.height));
    let mut common = codec::CommonAttrs::new().with_class("sheet");
    common.extra_attrs = vec![codec::attr("version", "1.1"), codec::attr("data-unit", "mm")];
    let mut children = vec![
        codec::element("title", vec![], vec![codec::text(title)]),
        codec::element("desc", vec![], vec![codec::text(&format!("Views of {title}, one group per view (plan, ceiling plan, section, elevation) at the scale of the view, paper millimetres."))]),
        codec::element("style", vec![], vec![codec::text(&style::sheet())]),
    ];
    children.extend(layout.slots.iter().map(|slot| drawing::view_group(model, slot, &drawings[&slot.view].lines)));
    let root = codec::SvgElement::Svg {
        common,
        view_box: Some(codec::ViewBox { min_x: 0.0, min_y: 0.0, width, height }),
        width: Some(format!("{width}mm")),
        height: Some(format!("{height}mm")),
        xmlns: Some(NAMESPACE.into()),
        children,
    };
    codec::document_text(&root)
}

/// 🎨️ The SVG text of the views of `model`.
pub fn export_svg(model: &ModelSnapshot) -> Result<String, String> {
    registry::try_with_inference(None, model, |inferred| views_to_svg(model, &inferred.view_linework)).map_err(|error| error.to_string())?
}

//#region 🔖️Serializer
/// 🎨️ The SVG serializer of the BIM model.
pub struct ModelIntoSvg;

impl Serializer<ModelSnapshot> for ModelIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let text = export_svg(from).map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ModelIntoSvg: {message}"))))?;
        Ok(IoOutcome::clean(IoPayload::Text(text)))
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
