//! 🖼️ `view-linework`: the drawing of every authored view, derived from the model and never stored. A view is a plan or ceiling plan of one storey, a section or an elevation through a vertical plane, or a camera;
//! this field generalises the `plan-linework` of a storey to any of them. The drawing is a [`ViewLinework`]: the same typed primitives as a plan (filled regions with holes, stroked paths, text anchors, each
//! with a style, a kind and the id of its element), in the coordinates of the view: building metres for a plan, `(u, z)` for a section or elevation, with `u` along the plane and `z` above the building datum.
//!
//! * A **plan** is the plan of its storey cut at the height the view resolves (its own, else the storey's, else the plan convention), filtered by hidden categories, phase and detail, cropped.
//! * A **ceiling plan** is the same cut looked at from below: what lies above the cut is drawn as projection, what lies below it dashed.
//! * A **section** cuts every solid the plane passes through (the poché is the section of its triangle mesh by the plane) and projects what lies behind the plane; an **elevation** cuts nothing. Projection removes
//!   hidden lines: an edge is drawn where no nearer face covers it, which is exact for polyhedra and so for the prisms of walls, columns, beams and slabs (see [`hidden_lines`]).
//! * A **camera** view draws nothing here; it is rendered by the 3D window.
//!
//! The graph is `Storey` (+ the plan inputs of its storey, or the `Solid`s of its building) → `View(id)`: editing one view recomputes that view only, editing the model recomputes the views that see the edit.
//!
//! Related: <https://en.wikipedia.org/wiki/Orthographic_projection>, <https://en.wikipedia.org/wiki/Hidden-line_removal>.

use super::super::element_solids::{dep_object, dep_value, ElementSolid};
use super::super::plan_linework::{self, Inputs as PlanInputs, PlanKind, PlanLinework, PlanStyle};
use crate::{DetailLevel, ModelSnapshot, View, ViewKind};
use semio_framework_2d::booleans::BooleanOperation;
use semio_framework_2d::regions::{region_boolean, Region};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

#[path = "✂️clip/🦀️.rs"]
pub mod clip;
#[path = "🪓️cut/🦀️.rs"]
pub mod cut;
#[path = "🔎️filters/🦀️.rs"]
pub mod filters;
#[path = "📐️frame/🦀️.rs"]
pub mod frame;
#[path = "👁️hidden-lines/🦀️.rs"]
pub mod hidden_lines;
#[path = "🗺️plans/🦀️.rs"]
pub mod plans;
#[path = "🏛️vertical/🦀️.rs"]
pub mod vertical;

//#region 🔖️Values
/// 🖼️ The drawing of one view: the kind, scale and detail it was drawn at, and the linework in the coordinates of the view. A plan view's `lines.storey` is its storey, a vertical view's is empty.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ViewLinework {
    pub view: String,
    pub kind: ViewKind,
    pub scale: u32,
    pub detail: DetailLevel,
    pub lines: PlanLinework,
}

impl ViewLinework {
    /// 🕳️ The empty drawing of `view`: a camera, or a view whose storey or plane is not there.
    pub fn empty(id: &str, view: &View) -> Self {
        let lines = plan_linework::Sheet::default().finish(view.storey.as_deref().unwrap_or_default(), 0.0);
        Self { view: id.to_string(), kind: view.kind, scale: view.scale, detail: view.detail, lines: PlanLinework { cut_height: 0.0, ..lines } }
    }
}
//#endregion 🔖️Values

//#region 🔖️Inputs
/// 🧾️ What a view is drawn from: the parents of its `View` node. A plan view reads the plan inputs of its storey, a vertical view the levels of its building and the solids in `solids`, by element id.
#[derive(Default)]
pub struct ViewInputs<'a> {
    pub plan: PlanInputs<'a>,
    pub solids: BTreeMap<&'a str, &'a ElementSolid>,
}

/// 🖼️ The drawing of the view `id` from the values its parents inferred.
pub fn view_of(snapshot: &ModelSnapshot, id: &str, view: &View, inputs: &ViewInputs<'_>) -> ViewLinework {
    match view.kind {
        ViewKind::Plan | ViewKind::CeilingPlan => plans::plan_view(snapshot, id, view, &inputs.plan),
        ViewKind::Section | ViewKind::Elevation => vertical::vertical_view(snapshot, id, view, &vertical::Inputs { levels: &inputs.plan.levels, solids: &inputs.solids }),
        ViewKind::Orthographic | ViewKind::Perspective => ViewLinework::empty(id, view),
    }
}

/// 🖼️ The drawing of every view (the `View` nodes of the model graph).
#[cfg(test)]
pub fn compute_view_linework(snapshot: &ModelSnapshot) -> BTreeMap<String, ViewLinework> {
    std::mem::take(&mut super::super::model_graph::infer_selected::<{ super::super::model_graph::kinds::VIEWS }>(snapshot).view_linework)
}
//#endregion 🔖️Inputs

//#region 🔖️Dependency
/// 🔑️ Everything the drawing of the view `id` reads of the snapshot besides the values of its parents: the view without its name, the storeys of its building (the datum labels carry their names), the phases of the walls
/// when the view filters by phase, and for a plan view what the plan of its storey reads.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let Some(view) = snapshot.views.get(id) else { return DslValue::Null };
    let storeys = snapshot.storeys.iter().filter(|(_, storey)| storey.building == view.building).map(|(storey, row)| (storey.clone(), dep_value(row)));
    let phases = if view.phase.is_some() { DslValue::object(filters::phases_of(snapshot, &view.building).into_iter().map(|(wall, phase)| (wall, dep_value(&phase)))) } else { DslValue::Null };
    dep_object([
        ("view", dep_value(&View { name: String::new(), ..view.clone() })),
        ("storeys", DslValue::object(storeys)),
        ("phases", phases),
        ("plan", view.storey.as_deref().map_or(DslValue::Null, |storey| plan_linework::dependency(snapshot, storey))),
    ])
}

/// 📖️ The snapshot collections the drawing of a view reads.
pub const READS: &[&str] = &["views", "storeys", "buildings", "sites", "walls", "curtain_walls", "curtain_wall_types", "curtain_panel_overrides", "columns", "beams", "slabs", "ceilings", "roofs", "stairs", "railings", "ramps", "spaces", "openings"];
//#endregion 🔖️Dependency

//#region 🔖️Metrics
fn rings_area(outer: &[plan_linework::PlanVertex], holes: &[Vec<plan_linework::PlanVertex>]) -> Region {
    let ring = |vertices: &[plan_linework::PlanVertex]| vertices.iter().map(|vertex| [vertex.x, vertex.y]).collect::<Vec<_>>();
    Region::new(&ring(outer), &holes.iter().map(|hole| ring(hole)).collect::<Vec<_>>())
}

impl ViewLinework {
    /// 📐️ The area, in square metres of the drawing, of the union of the regions of `kind`: what a third party computes with `unary_union`.
    pub fn union_area_of(&self, kind: PlanKind) -> f64 {
        let regions: Vec<Region> = self.lines.regions.iter().filter(|region| region.kind == kind).map(|region| rings_area(&region.outer, &region.holes)).collect();
        match regions.len() {
            0 => 0.0,
            1 => regions[0].area(),
            _ => region_boolean(BooleanOperation::Union, &regions, &[], &mut |_| true).map_or(f64::NAN, |united| united.iter().map(Region::area).sum()),
        }
    }

    /// 📏️ The length of the stroked paths of `kind` drawn in `style`.
    pub fn length_in(&self, kind: PlanKind, style: PlanStyle) -> f64 {
        let only = PlanLinework { polylines: self.lines.polylines.iter().filter(|line| line.style == style).cloned().collect(), ..self.lines.clone() };
        only.length_of(kind)
    }
}

/// 📏️ The measures of a vertical view that third-party geometry libraries adjudicate: the union area of the silhouettes (the projection outline), the area of the cut and the length of the datum lines. The edges are decided by hidden-line
/// removal, which no third-party library offers; they are pinned by the unit tests of [`hidden_lines`].
pub const VERTICAL_METRICS: &[&str] = &["Silhouette.union_area", "SectionCut.area", "Datum.length"];
//#endregion 🔖️Metrics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
