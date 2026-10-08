//! 🎨️ `s.bim.model@1/*` → `s.stdio.svg@1.1/*`: the floor plans of a [`ModelSnapshot`] as one SVG 1.1 sheet at 1:100 in paper millimetres.
//! One `<g class="storey">` per storey (id `storey-<id>`, title and level), stacked top-down per building; inside it the poché regions, the stroked lines in one class per line style
//! (cut heavy, projection medium, hidden dashed, annotation fine) and the texts (space tags, grid labels). The linework is the one of the `plan-linework` inference; the viewBox fits the sheet.
//! 🔖 `IoFidelity::Lossy`: a plan is a cut at a fixed height; everything above it is dashed, everything the model knows beyond lines and labels is dropped.
//! The tree is the typed SVG model of the `s.stdio.svg` artifact and the text is written by the XML writer of `s.stdio.xml`, both behind [`codec`].
//! 📎 https://www.w3.org/TR/SVG11/

use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{compute_plan_linework, PlanLinework};
use crate::ModelSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
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

/// 🎨️ The SVG document of `model` from already inferred plans.
pub fn plans_to_svg(model: &ModelSnapshot, plans: &BTreeMap<String, PlanLinework>) -> Result<String, String> {
    let layout = sheet::layout(model, plans);
    let title = if model.project.name.is_empty() { "BIM model" } else { model.project.name.as_str() };
    let (width, height) = (path::snap(layout.width), path::snap(layout.height));
    let mut common = codec::CommonAttrs::new().with_class("plan");
    common.extra_attrs = vec![codec::attr("version", "1.1"), codec::attr("data-scale", path::SCALE), codec::attr("data-unit", "mm")];
    let mut children = vec![
        codec::element("title", vec![], vec![codec::text(title)]),
        codec::element("desc", vec![], vec![codec::text(&format!("Floor plans of {title}, one group per storey, scale 1:{}, paper millimetres, north up.", path::SCALE))]),
        codec::element("style", vec![], vec![codec::text(&style::sheet())]),
    ];
    children.extend(layout.slots.iter().map(|slot| drawing::storey_group(slot, &plans[&slot.storey])));
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

/// 🎨️ The SVG text of the plans of `model`.
pub fn export_svg(model: &ModelSnapshot) -> Result<String, String> {
    plans_to_svg(model, &compute_plan_linework(model))
}

//#region 🔖️Serializer
/// 🎨️ The SVG serializer of the BIM model.
pub struct ModelIntoSvg;

impl Serializer<ModelSnapshot> for ModelIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
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
