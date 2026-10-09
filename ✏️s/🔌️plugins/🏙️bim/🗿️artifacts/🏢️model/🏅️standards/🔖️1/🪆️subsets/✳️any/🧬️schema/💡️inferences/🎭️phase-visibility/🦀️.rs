//! 🎭️ `phase-visibility`: which elements a view of a storey shows for each view phase. The phase of an element is authored (`phase` on walls, curtain walls, columns, beams, slabs, roofs, stairs, railings and spaces); an opening
//! takes the phase of its host, and an element kind that carries no phase of its own (ceilings, ramps) counts as new construction. A view phase is the filter of a view: `All` shows every element, any other value shows exactly the
//! elements of that phase. One [`PhaseVisibility`] per storey is a node of the model graph; it reads authored phases, storeys and hosts only, so an edit of any geometry never recomputes it.
//!
//! Related: IFC2x3 `IfcProcess`, the construction process an element belongs to, <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcprocessextension/lexical/ifcprocess.htm>.

use crate::{ModelSnapshot, Phase};
use std::collections::BTreeMap;

/// 🗺️ The snapshot collections the view phases read: the storeys, the phase-bearing element kinds and the openings that follow their host.
pub const READS: &[&str] = &["storeys", "walls", "curtain_walls", "columns", "beams", "slabs", "ceilings", "roofs", "stairs", "railings", "ramps", "spaces", "openings"];

//#region 🔖️Values
/// 🎭️ The phase filter of a view: every phase, or exactly one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum ViewPhase {
    #[default]
    All,
    Existing,
    New,
    Demolished,
    Temporary,
}

impl ViewPhase {
    /// 🔢️ Every view phase: all first, then the phases in the order of a project's life.
    pub const ALL: [ViewPhase; 5] = [ViewPhase::All, ViewPhase::Existing, ViewPhase::New, ViewPhase::Demolished, ViewPhase::Temporary];

    /// 🏷️ The stable key of the view phase (`all`, `existing`, `new`, `demolished`, `temporary`).
    pub const fn key(self) -> &'static str {
        match self {
            ViewPhase::All => "all",
            ViewPhase::Existing => "existing",
            ViewPhase::New => "new",
            ViewPhase::Demolished => "demolished",
            ViewPhase::Temporary => "temporary",
        }
    }

    /// 🔎️ The view phase a typed name denotes, ignoring case and blanks; an empty name means every phase, anything unknown `None`.
    pub fn parse(text: &str) -> Option<ViewPhase> {
        let text = text.trim();
        if text.is_empty() {
            return Some(ViewPhase::All);
        }
        ViewPhase::ALL.into_iter().find(|view| view.key().eq_ignore_ascii_case(text))
    }

    /// 🎭️ The view phase that shows exactly `filter`'s phase, every phase for `None`.
    pub const fn of(filter: Option<Phase>) -> ViewPhase {
        match filter {
            None => ViewPhase::All,
            Some(Phase::Existing) => ViewPhase::Existing,
            Some(Phase::New) => ViewPhase::New,
            Some(Phase::Demolished) => ViewPhase::Demolished,
            Some(Phase::Temporary) => ViewPhase::Temporary,
        }
    }

    /// 🎯️ The one phase this view phase shows, `None` for `All`.
    pub const fn filter(self) -> Option<Phase> {
        match self {
            ViewPhase::All => None,
            ViewPhase::Existing => Some(Phase::Existing),
            ViewPhase::New => Some(Phase::New),
            ViewPhase::Demolished => Some(Phase::Demolished),
            ViewPhase::Temporary => Some(Phase::Temporary),
        }
    }

    /// 👁️ Whether an element in `phase` is shown.
    pub fn shows(self, phase: Phase) -> bool {
        self.filter().is_none_or(|only| only == phase)
    }
}

/// 🎭️ The elements of one storey a view shows, per view phase: sorted element ids under the key of each [`ViewPhase`].
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub struct PhaseVisibility {
    pub visible: BTreeMap<String, Vec<String>>,
}

impl PhaseVisibility {
    /// 📃 The ids a view with this phase filter shows, in id order; empty for a storey without elements.
    pub fn ids(&self, view: ViewPhase) -> &[String] {
        self.visible.get(view.key()).map_or(&[], Vec::as_slice)
    }

    /// 👁️ Whether the element is shown under this phase filter.
    pub fn shows(&self, view: ViewPhase, id: &str) -> bool {
        self.ids(view).binary_search_by(|row| row.as_str().cmp(id)).is_ok()
    }

    /// 🔎️ Whether the element stands on this storey and carries a phase here (an id the visibility does not know is never filtered).
    pub fn knows(&self, id: &str) -> bool {
        self.shows(ViewPhase::All, id)
    }

    /// 🙈️ Whether the filter hides the element: it is known here and its phase is not shown.
    pub fn hides(&self, view: ViewPhase, id: &str) -> bool {
        self.knows(id) && !self.shows(view, id)
    }
}
//#endregion 🔖️Values

//#region 🔖️Rules
/// 🕰️ The phase an element effectively has: its own, its host's for an opening or a wall sweep, new construction for an element kind without a phase; `None` for an id that is no element of a storey.
pub fn phase_of(snapshot: &ModelSnapshot, id: &str) -> Option<Phase> {
    snapshot
        .walls
        .get(id)
        .map(|row| row.phase)
        .or_else(|| snapshot.curtain_walls.get(id).map(|row| row.phase))
        .or_else(|| snapshot.columns.get(id).map(|row| row.phase))
        .or_else(|| snapshot.beams.get(id).map(|row| row.phase))
        .or_else(|| snapshot.slabs.get(id).map(|row| row.phase))
        .or_else(|| snapshot.roofs.get(id).map(|row| row.phase))
        .or_else(|| snapshot.stairs.get(id).map(|row| row.phase))
        .or_else(|| snapshot.railings.get(id).map(|row| row.phase))
        .or_else(|| snapshot.spaces.get(id).map(|row| row.phase))
        .or_else(|| (snapshot.ceilings.contains_key(id) || snapshot.ramps.contains_key(id)).then_some(Phase::New))
        .or_else(|| snapshot.openings.get(id).and_then(|opening| snapshot.walls.get(&opening.host).map(|row| row.phase).or_else(|| snapshot.curtain_walls.get(&opening.host).map(|row| row.phase))))
        .or_else(|| snapshot.wall_sweeps.get(id).and_then(|sweep| snapshot.walls.get(&sweep.host)).map(|row| row.phase))
}

/// 🪜️ Every element standing on `storey` with its effective phase, in id order: the phase-bearing kinds, the kinds that count as new, and the openings of the walls and curtain walls on it.
fn elements_on(snapshot: &ModelSnapshot, storey: &str) -> BTreeMap<String, Phase> {
    let mut rows: BTreeMap<String, Phase> = BTreeMap::new();
    macro_rules! on {
        ($($collection:ident => $phase:expr),+ $(,)?) => {
            $( rows.extend(snapshot.$collection.iter().filter(|(_, row)| row.storey == storey).map(|(id, row)| (id.clone(), $phase(row)))); )+
        };
    }
    on!(
        walls => |row: &crate::Wall| row.phase,
        curtain_walls => |row: &crate::CurtainWall| row.phase,
        columns => |row: &crate::Column| row.phase,
        beams => |row: &crate::Beam| row.phase,
        slabs => |row: &crate::Slab| row.phase,
        roofs => |row: &crate::Roof| row.phase,
        stairs => |row: &crate::Stair| row.phase,
        railings => |row: &crate::Railing| row.phase,
        spaces => |row: &crate::Space| row.phase,
        ceilings => |_: &crate::Ceiling| Phase::New,
        ramps => |_: &crate::Ramp| Phase::New,
    );
    for (id, opening) in &snapshot.openings {
        let host = rows.get(&opening.host).filter(|_| snapshot.walls.contains_key(&opening.host) || snapshot.curtain_walls.contains_key(&opening.host));
        if let Some(phase) = host {
            let phase = *phase;
            rows.insert(id.clone(), phase);
        }
    }
    for (id, sweep) in &snapshot.wall_sweeps {
        if let Some(phase) = rows.get(&sweep.host).filter(|_| snapshot.walls.contains_key(&sweep.host)).copied() {
            rows.insert(id.clone(), phase);
        }
    }
    rows
}

/// 🕰️ The effective phase of every element standing on a storey of `building`, by id: what a phase-filtered view of the building reads.
pub fn phases_in(snapshot: &ModelSnapshot, building: &str) -> BTreeMap<String, Phase> {
    snapshot.storeys.iter().filter(|(_, storey)| storey.building == building).flat_map(|(id, _)| elements_on(snapshot, id)).collect()
}

/// 🎭️ The elements `storey` shows under each view phase.
pub fn visibility_of(snapshot: &ModelSnapshot, storey: &str) -> PhaseVisibility {
    let rows = elements_on(snapshot, storey);
    PhaseVisibility { visible: ViewPhase::ALL.into_iter().map(|view| (view.key().to_string(), rows.iter().filter(|(_, phase)| view.shows(**phase)).map(|(id, _)| id.clone()).collect())).collect() }
}

/// 📋️ The canonical JSON the phase oracle compares: `{storey id: {view phase key: [element ids]}}`.
pub fn table_json(visibility: &BTreeMap<String, PhaseVisibility>) -> String {
    let rows: BTreeMap<&String, &BTreeMap<String, Vec<String>>> = visibility.iter().map(|(storey, row)| (storey, &row.visible)).collect();
    semio_framework_pack_json::to_json_string(&rows)
}

/// 🔑️ What the node of `storey` reads of the snapshot: the visibility itself, which changes exactly when an element joins or leaves the storey or changes phase.
pub fn dependency(snapshot: &ModelSnapshot, storey: &str) -> semio_framework_value::DslValue {
    semio_framework_value::ToValue::to_value(&visibility_of(snapshot, storey))
}
//#endregion 🔖️Rules

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
