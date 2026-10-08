//! 🌊️ Shared cascade of every delete leaf: the closure of records that leave with a set of root elements (site → buildings →
//! storeys and grid lines → everything on a storey → openings of removed walls and curtain walls → properties and classifications
//! of every removed element), the sparse diff that deletes it, and its inverse of one concrete create (or data setter) per record.

use super::super::set_element_classification::SetElementClassification;
use super::super::set_element_property::SetElementProperty;
use crate::{Entry, KeyedDelta, ModelDiff, ModelMutation, ModelSnapshot, Patch, TopConstraint};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeSet;

/// 🚫️ Why a removal cannot be performed: the code, the id the refusal names and the human message.
pub struct Refusal {
    pub code: OutcomeCode,
    pub target: String,
    pub message: String,
}

fn deleted<T: Clone + PartialEq, P: Patch<T>>(ids: &BTreeSet<String>) -> Option<KeyedDelta<T, P>> {
    (!ids.is_empty()).then(|| KeyedDelta(ids.iter().map(|id| (id.clone(), Entry::Deleted)).collect()))
}

//#region 🔖️Removal
macro_rules! removal {
    ($($field:ident => $variant:ident($module:ident, $record:ident);)*) => {
        /// 🧺️ The ids to delete per collection, in creation order; properties and classifications hold the element ids whose entry goes.
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct Removal {
            $( pub $field: BTreeSet<String>, )*
            pub properties: BTreeSet<String>,
            pub classifications: BTreeSet<String>,
        }

        impl Removal {
            /// 🔢️ How many element records leave.
            pub fn len(&self) -> usize {
                0 $( + self.$field.len() )*
            }

            /// 🔎️ Whether the element `id` leaves.
            pub fn contains(&self, id: &str) -> bool {
                false $( || self.$field.contains(id) )*
            }

            fn root(&mut self, base: &ModelSnapshot, id: &str) -> bool {
                let mut found = false;
                $( if base.$field.contains_key(id) { self.$field.insert(id.to_string()); found = true; } )*
                found
            }

            /// 🔺️ The sparse diff that deletes every record of the removal.
            pub fn diff(&self) -> ModelDiff {
                ModelDiff {
                    $( $field: deleted(&self.$field), )*
                    properties: deleted(&self.properties),
                    classifications: deleted(&self.classifications),
                    ..ModelDiff::default()
                }
            }

            /// ↩️ One concrete row per removed record in storage order: the store replays the vector reversed, so sites come last and are
            /// recreated first, then buildings, storeys, grid lines, storey contents, openings, and finally the property and classification setters.
            pub fn inverse(&self, base: &ModelSnapshot) -> Vec<ModelMutation> {
                let mut rows = Vec::new();
                $(
                    for id in &self.$field {
                        if let Some(record) = base.$field.get(id) {
                            rows.push(ModelMutation::$variant(super::super::$module::$variant { id: id.clone(), $record: record.clone() }));
                        }
                    }
                )*
                for id in &self.properties {
                    for (set, properties) in base.properties.get(id).into_iter().flatten() {
                        for (property, value) in properties {
                            rows.push(ModelMutation::SetElementProperty(SetElementProperty { id: id.clone(), pset: set.clone(), property: property.clone(), value: value.clone() }));
                        }
                    }
                }
                for id in &self.classifications {
                    if let Some(classification) = base.classifications.get(id) {
                        rows.push(ModelMutation::SetElementClassification(SetElementClassification { id: id.clone(), classification: classification.clone() }));
                    }
                }
                rows.reverse();
                rows
            }
        }
    };
}

removal! {
    sites => CreateSite(create_site, site);
    buildings => CreateBuilding(create_building, building);
    storeys => CreateStorey(create_storey, storey);
    grids => CreateGridLine(create_grid_line, grid_line);
    walls => CreateWall(create_wall, wall);
    curtain_walls => CreateCurtainWall(create_curtain_wall, curtain_wall);
    columns => CreateColumn(create_column, column);
    beams => CreateBeam(create_beam, beam);
    slabs => CreateSlab(create_slab, slab);
    roofs => CreateRoof(create_roof, roof);
    stairs => CreateStair(create_stair, stair);
    railings => CreateRailing(create_railing, railing);
    spaces => CreateSpace(create_space, space);
    openings => CreateOpening(create_opening, opening);
}
//#endregion 🔖️Removal

fn constrains(top: &TopConstraint, storeys: &BTreeSet<String>) -> Option<String> {
    match top {
        TopConstraint::Storey { storey, .. } if storeys.contains(storey) => Some(storey.clone()),
        _ => None,
    }
}

fn pinned(base: &ModelSnapshot, removal: &Removal) -> Option<(String, String)> {
    let hit = |id: &String, top: &TopConstraint| (!removal.contains(id)).then(|| constrains(top, &removal.storeys).map(|storey| (storey, id.clone()))).flatten();
    base.walls
        .iter()
        .find_map(|(id, row)| hit(id, &row.top))
        .or_else(|| base.curtain_walls.iter().find_map(|(id, row)| hit(id, &row.top)))
        .or_else(|| base.columns.iter().find_map(|(id, row)| hit(id, &row.top)))
        .or_else(|| base.stairs.iter().find_map(|(id, row)| hit(id, &row.top)))
}

/// 🌊️ The closure of everything that leaves with `roots`, or the refusal: an unknown root is `TargetMissing`, a storey that a surviving
/// wall, curtain wall, column or stair still constrains its top to is `TargetReferenced` (that reference cannot cascade).
pub fn closure(base: &ModelSnapshot, roots: &[String]) -> Result<Removal, Refusal> {
    let mut removal = Removal::default();
    for id in roots {
        if !removal.root(base, id) {
            return Err(Refusal { code: OutcomeCode::TargetMissing, target: id.clone(), message: format!("Element \"{id}\" does not exist.") });
        }
    }
    removal.buildings.extend(base.buildings.iter().filter(|(_, row)| removal.sites.contains(&row.site)).map(|(id, _)| id.clone()));
    removal.storeys.extend(base.storeys.iter().filter(|(_, row)| removal.buildings.contains(&row.building)).map(|(id, _)| id.clone()));
    removal.grids.extend(base.grids.iter().filter(|(_, row)| removal.buildings.contains(&row.building)).map(|(id, _)| id.clone()));
    let on = |storey: &String| removal.storeys.contains(storey);
    let (walls, curtain_walls, columns, beams, slabs, roofs, stairs, railings, spaces) = (
        base.walls.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>(),
        base.curtain_walls.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>(),
        base.columns.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>(),
        base.beams.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>(),
        base.slabs.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>(),
        base.roofs.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>(),
        base.stairs.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>(),
        base.railings.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>(),
        base.spaces.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>(),
    );
    removal.walls.extend(walls);
    removal.curtain_walls.extend(curtain_walls);
    removal.columns.extend(columns);
    removal.beams.extend(beams);
    removal.slabs.extend(slabs);
    removal.roofs.extend(roofs);
    removal.stairs.extend(stairs);
    removal.railings.extend(railings);
    removal.spaces.extend(spaces);
    let hosted: Vec<String> = base.openings.iter().filter(|(_, row)| removal.walls.contains(&row.host) || removal.curtain_walls.contains(&row.host)).map(|(id, _)| id.clone()).collect();
    removal.openings.extend(hosted);
    if let Some((storey, by)) = pinned(base, &removal) {
        return Err(Refusal { code: OutcomeCode::TargetReferenced, target: storey.clone(), message: format!("Storey \"{storey}\" is still the top constraint of \"{by}\".") });
    }
    removal.properties = base.properties.keys().filter(|id| removal.contains(id)).cloned().collect();
    removal.classifications = base.classifications.keys().filter(|id| removal.contains(id)).cloned().collect();
    Ok(removal)
}

/// 🚮️ The shared outcome of a delete leaf: the sparse removal diff with a cascade note when records beyond the roots leave, or the refusal.
/// `noun` names the root kind in messages and `anchor` the id a refusal about a pinned storey reports (the payload's own id).
pub fn outcome(base: &ModelSnapshot, roots: &[String], noun: &str, anchor: Option<&str>) -> MutationOutcome<ModelDiff> {
    match closure(base, roots) {
        Err(refusal) => {
            let target = match (refusal.code, anchor) {
                (OutcomeCode::TargetReferenced, Some(anchor)) => anchor.to_string(),
                _ => refusal.target,
            };
            let message = match refusal.code {
                OutcomeCode::TargetMissing => format!("{noun} \"{target}\" does not exist."),
                _ => refusal.message,
            };
            MutationOutcome::refuse(refusal.code, message, [target])
        }
        Ok(removal) => {
            let cascaded = removal.len() - roots.iter().collect::<BTreeSet<_>>().len();
            let outcome = MutationOutcome::new(removal.diff());
            if cascaded == 0 {
                outcome
            } else {
                outcome.info(OutcomeCode::Cascade, format!("{noun} \"{}\" took {cascaded} dependent element(s) with it.", roots.first().map(String::as_str).unwrap_or_default()))
            }
        }
    }
}

/// ↩️ The inverse rows of a delete leaf: empty when a root is unknown or the removal is refused, else the removal's concrete rows.
pub fn inverse(base: &ModelSnapshot, roots: &[String]) -> Vec<ModelMutation> {
    closure(base, roots).map(|removal| removal.inverse(base)).unwrap_or_default()
}
