//! 🌊️ Shared cascade of every delete leaf: the closure of records that leave with a set of root elements (site → buildings →
//! storeys and grid lines → the views and schedules scoped to them → everything on a storey → panel overrides and openings of removed curtain walls and openings of removed walls → properties and classifications
//! of every removed element), the sparse diff that deletes it, and its inverse of one concrete create (or data setter) per record.

use super::super::set_component_override::SetComponentOverride;
use super::super::set_element_classification::SetElementClassification;
use super::super::set_element_property::SetElementProperty;
use super::super::set_space_conditions::SetSpaceConditions;
use crate::{Entry, KeyedDelta, ModelDiff, ModelMutation, ModelSnapshot, Patch, TopConstraint};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeSet;

/// 🚫️ Why a closure(base, roots).ok().filter(|removal| removal.rows(base) <= INVERSE_ROWS).map(|removal| removal.inverse(base)).unwrap_or_default() cannot be performed: the code, the id the refusal names and the human message.
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
            pub component_overrides: BTreeSet<String>,
            pub properties: BTreeSet<String>,
            pub classifications: BTreeSet<String>,
            pub space_conditions: BTreeSet<String>,
            pub element_options: BTreeSet<String>,
            pub element_worksets: BTreeSet<String>,
        }

        impl Removal {
            /// 🔢️ How many element records leave.
            pub fn len(&self) -> usize {
                0 $( + self.$field.len() )*
            }

            /// 🪪️ The id of every record that leaves, collection by collection in creation order.
            pub fn ids(&self) -> Vec<String> {
                let mut ids = Vec::new();
                $( ids.extend(self.$field.iter().cloned()); )*
                ids
            }

            /// 🔎️ Whether the element `id` leaves.
            pub fn contains(&self, id: &str) -> bool {
                false $( || self.$field.contains(id) )*
            }

            fn with_root(mut self, base: &ModelSnapshot, id: &str) -> Option<Self> {
                let mut found = false;
                $( if base.$field.contains_key(id) { self.$field.insert(id.to_string()); found = true; } )*
                found.then_some(self)
            }

            /// 🔺️ The sparse diff that deletes every record of the removal.
            pub fn diff(&self) -> ModelDiff {
                ModelDiff {
                    $( $field: deleted(&self.$field), )*
                    component_overrides: deleted(&self.component_overrides),
                    properties: deleted(&self.properties),
                    classifications: deleted(&self.classifications),
                    space_conditions: deleted(&self.space_conditions),
                    element_options: deleted(&self.element_options),
                    element_worksets: deleted(&self.element_worksets),
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
                for key in &self.component_overrides {
                    if let Some(record) = base.component_overrides.get(key) {
                        rows.push(ModelMutation::SetComponentOverride(SetComponentOverride { component: record.component.clone(), name: record.name.clone(), value: record.value.clone() }));
                    }
                }
                for id in &self.properties {
                    for (set, properties) in base.properties.get(id).into_iter().flatten() {
                        for (property, value) in properties {
                            rows.push(ModelMutation::SetElementProperty(SetElementProperty { id: id.clone(), pset: set.clone(), property: property.clone(), value: value.clone() }));
                        }
                    }
                }
                for id in &self.classifications {
                    for (system, code) in base.classifications.get(id).into_iter().flatten() {
                        rows.push(ModelMutation::SetElementClassification(SetElementClassification { id: id.clone(), system: system.clone(), code: code.clone() }));
                    }
                }
                for id in &self.space_conditions {
                    if let Some(record) = base.space_conditions.get(id) {
                        rows.push(ModelMutation::SetSpaceConditions(SetSpaceConditions::stating(id, record)));
                    }
                }
                for id in &self.element_options {
                    if let Some(member) = base.element_options.get(id) { rows.push(ModelMutation::SetElementOption(super::super::set_element_option::SetElementOption { id: id.clone(), option: Some(member.target.clone()) })); }
                }
                for id in &self.element_worksets {
                    if let Some(member) = base.element_worksets.get(id) { rows.push(ModelMutation::SetElementWorkset(super::super::set_element_workset::SetElementWorkset { id: id.clone(), workset: Some(member.target.clone()) })); }
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
    views => CreateView(create_view, view);
    schedules => CreateSchedule(create_schedule, schedule);
    sheets => CreateSheet(create_sheet, sheet);
    viewports => CreateViewport(create_viewport, viewport);
    sheet_revisions => CreateSheetRevision(create_sheet_revision, sheet_revision);
    clash_sets => CreateClashSet(create_clash_set, clash_set);
    rules => CreateRule(create_rule, rule);
    issues => CreateIssue(create_issue, issue);
    issue_comments => CreateIssueComment(create_issue_comment, issue_comment);
    walls => CreateWall(create_wall, wall);
    curtain_walls => CreateCurtainWall(create_curtain_wall, curtain_wall);
    curtain_panel_overrides => CreateCurtainPanelOverride(create_curtain_panel_override, curtain_panel_override);
    columns => CreateColumn(create_column, column);
    beams => CreateBeam(create_beam, beam);
    slabs => CreateSlab(create_slab, slab);
    ceilings => CreateCeiling(create_ceiling, ceiling);
    roofs => CreateRoof(create_roof, roof);
    stairs => CreateStair(create_stair, stair);
    ramps => CreateRamp(create_ramp, ramp);
    railings => CreateRailing(create_railing, railing);
    spaces => CreateSpace(create_space, space);
    zones => CreateZone(create_zone, zone);
    area_schemes => CreateAreaScheme(create_area_scheme, area_scheme);
    openings => CreateOpening(create_opening, opening);
    wall_sweeps => CreateWallSweep(create_wall_sweep, wall_sweep);
    components => CreateComponent(create_component, component);
    mep_elements => CreateMepElement(create_mep_element, mep);
    dimensions => CreateDimension(create_dimension, dimension);
    tags => CreateTag(create_tag, tag);
    text_notes => CreateTextNote(create_text_note, text_note);
    leaders => CreateLeader(create_leader, leader);
    load_cases => CreateLoadCase(create_load_case, load_case);
    supports => CreateSupport(create_support, support);
    loads => CreateLoad(create_load, load);
}
//#endregion 🔖️Removal

/// 🧾️ The most inverse rows a cascading delete leaf of the whole-selection route (`delete-elements`, `delete-site`, `delete-building`, `delete-storey`) yields: the
/// declared bound of their payload schemas, small enough that the rows of several of them still fit one gesture of the store (65 536 staged rows).
pub const INVERSE_ROWS: usize = 8191;

impl Removal {
    /// 🔢️ How many inverse rows the removal yields: one create per removed record and one setter per removed property and classification.
    pub fn rows(&self, base: &ModelSnapshot) -> usize {
        let properties: usize = self.properties.iter().map(|id| base.properties.get(id).map_or(0, |sets| sets.values().map(|set| set.len()).sum::<usize>())).sum::<usize>();
        let classifications: usize = self.classifications.iter().map(|id| base.classifications.get(id).map_or(0, |set| set.len())).sum();
        let records = self.len();
        records + properties + classifications + self.space_conditions.len() + self.component_overrides.len() + self.element_options.len() + self.element_worksets.len()
    }
}

fn constrains(top: &TopConstraint, storeys: &BTreeSet<String>) -> Option<String> {
    match top {
        TopConstraint::Storey { storey, .. } if storeys.contains(storey) => Some(storey.clone()),
        _ => None,
    }
}

/// 🔗️ The first removed roof, slab or ceiling that a surviving wall still attaches its top or base to, with the wall: an attach reference cannot cascade, so it refuses.
fn attached(base: &ModelSnapshot, removal: &Removal) -> Option<(String, String)> {
    base.walls.iter().filter(|(id, _)| !removal.contains(id)).find_map(|(id, wall)| {
        let top = match &wall.top {
            TopConstraint::Roof { roof, .. } if removal.roofs.contains(roof) => Some(roof),
            TopConstraint::Slab { slab, .. } if removal.slabs.contains(slab) => Some(slab),
            TopConstraint::Ceiling { ceiling, .. } if removal.ceilings.contains(ceiling) => Some(ceiling),
            _ => None,
        };
        top.or(wall.base_slab.as_ref().filter(|slab| removal.slabs.contains(*slab))).map(|target| (target.clone(), id.clone()))
    })
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

/// 📏️ Adds the annotations that leave with the removal: every annotation of a removed storey, and every dimension, tag and leader that names a removed element.
fn annotated(base: &ModelSnapshot, removal: &mut Removal) {
    let follows = |id: Option<&str>| id.is_some_and(|id| removal.contains(id));
    let on = |storey: &String| removal.storeys.contains(storey);
    let dimensions: Vec<String> = base.dimensions.iter().filter(|(_, row)| on(&row.storey) || row.elements().any(|id| follows(Some(id)))).map(|(id, _)| id.clone()).collect();
    let tags: Vec<String> = base.tags.iter().filter(|(_, row)| on(&row.storey) || follows(Some(&row.element))).map(|(id, _)| id.clone()).collect();
    let notes: Vec<String> = base.text_notes.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect();
    let leaders: Vec<String> = base.leaders.iter().filter(|(_, row)| on(&row.storey) || follows(row.element())).map(|(id, _)| id.clone()).collect();
    removal.dimensions.extend(dimensions);
    removal.tags.extend(tags);
    removal.text_notes.extend(notes);
    removal.leaders.extend(leaders);
}

/// 🌊️ The closure of everything that leaves with `roots`, or the refusal: an unknown root is `TargetMissing`, a storey that a surviving
/// wall, curtain wall, column or stair still constrains its top to is `TargetReferenced` (that reference cannot cascade).
pub fn closure(base: &ModelSnapshot, roots: &[String]) -> Result<Removal, Refusal> {
    let mut removal = Removal::default();
    for id in roots {
        let Some(rooted) = removal.with_root(base, id) else {
            return Err(Refusal { code: OutcomeCode::TargetMissing, target: id.clone(), message: format!("Element \"{id}\" does not exist.") });
        };
        removal = rooted;
    }
    removal.buildings.extend(base.buildings.iter().filter(|(_, row)| removal.sites.contains(&row.site)).map(|(id, _)| id.clone()));
    removal.storeys.extend(base.storeys.iter().filter(|(_, row)| removal.buildings.contains(&row.building)).map(|(id, _)| id.clone()));
    removal.grids.extend(base.grids.iter().filter(|(_, row)| removal.buildings.contains(&row.building)).map(|(id, _)| id.clone()));
    removal.views.extend(base.views.iter().filter(|(_, row)| removal.buildings.contains(&row.building) || row.storey.as_ref().is_some_and(|storey| removal.storeys.contains(storey))).map(|(id, _)| id.clone()));
    removal.schedules.extend(base.schedules.iter().filter(|(_, row)| row.storeys.iter().any(|storey| removal.storeys.contains(storey))).map(|(id, _)| id.clone()));
    removal.viewports.extend(base.viewports.iter().filter(|(_, row)| removal.sheets.contains(&row.sheet) || removal.views.contains(&row.view)).map(|(id, _)| id.clone()));
    removal.sheet_revisions.extend(base.sheet_revisions.iter().filter(|(_, row)| removal.sheets.contains(&row.sheet)).map(|(id, _)| id.clone()));
    let scoped = |storeys: &[String]| storeys.iter().any(|storey| removal.storeys.contains(storey));
    removal.clash_sets.extend(base.clash_sets.iter().filter(|(_, row)| scoped(&row.a.storeys) || scoped(&row.b.storeys)).map(|(id, _)| id.clone()));
    removal.rules.extend(base.rules.iter().filter(|(_, row)| scoped(&row.scope.storeys)).map(|(id, _)| id.clone()));
    removal.issue_comments.extend(base.issue_comments.iter().filter(|(_, row)| removal.issues.contains(&row.issue)).map(|(id, _)| id.clone()));
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
    removal.ceilings.extend(base.ceilings.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()));
    removal.roofs.extend(roofs);
    removal.stairs.extend(stairs);
    removal.ramps.extend(base.ramps.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()));
    removal.railings.extend(railings);
    removal.spaces.extend(spaces);
    let fenced: Vec<String> = base.railings.iter().filter(|(_, row)| row.host.as_ref().is_some_and(|host| removal.contains(&host.element))).map(|(id, _)| id.clone()).collect();
    removal.railings.extend(fenced);
    let overridden: Vec<String> = base.curtain_panel_overrides.iter().filter(|(_, row)| removal.curtain_walls.contains(&row.curtain)).map(|(id, _)| id.clone()).collect();
    removal.curtain_panel_overrides.extend(overridden);
    let hosted: Vec<String> = base.openings.iter().filter(|(_, row)| removal.walls.contains(&row.host) || removal.curtain_walls.contains(&row.host)).map(|(id, _)| id.clone()).collect();
    removal.openings.extend(hosted);
    let swept: Vec<String> = base.wall_sweeps.iter().filter(|(_, row)| removal.walls.contains(&row.host)).map(|(id, _)| id.clone()).collect();
    removal.wall_sweeps.extend(swept);
    removal.components.extend(base.components.iter().filter(|(_, row)| on(&row.storey) || row.host.as_ref().is_some_and(|host| removal.walls.contains(host))).map(|(id, _)| id.clone()).collect::<Vec<_>>());
    removal.mep_elements.extend(base.mep_elements.iter().filter(|(_, row)| on(&row.storey)).map(|(id, _)| id.clone()).collect::<Vec<_>>());
    removal.component_overrides.extend(base.component_overrides.iter().filter(|(_, row)| removal.components.contains(&row.component)).map(|(key, _)| key.clone()).collect::<Vec<_>>());
    removal.element_options.extend(base.element_options.keys().filter(|id| removal.contains(id)).cloned().collect::<Vec<_>>());
    removal.element_worksets.extend(base.element_worksets.keys().filter(|id| removal.contains(id)).cloned().collect::<Vec<_>>());
    let supports: Vec<String> = base.supports.iter().filter(|(_,row)| removal.contains(&row.member)).map(|(id,_)|id.clone()).collect();
    removal.supports.extend(supports);
    let loads: Vec<String> = base.loads.iter().filter(|(_,row)| removal.contains(&row.member) || removal.load_cases.contains(&row.load_case)).map(|(id,_)|id.clone()).collect();
    removal.loads.extend(loads);
    annotated(base, &mut removal);
    if let Some((storey, by)) = pinned(base, &removal) {
        return Err(Refusal { code: OutcomeCode::TargetReferenced, target: storey.clone(), message: format!("Storey \"{storey}\" is still the top constraint of \"{by}\".") });
    }
    if let Some((target, by)) = attached(base, &removal) {
        return Err(Refusal { code: OutcomeCode::TargetReferenced, target: target.clone(), message: format!("\"{target}\" is still the attach target of wall \"{by}\".") });
    }
    removal.properties = base.properties.keys().filter(|id| removal.contains(id)).cloned().collect();
    removal.classifications = base.classifications.keys().filter(|id| removal.contains(id)).cloned().collect();
    removal.space_conditions = base.space_conditions.keys().filter(|id| removal.spaces.contains(*id)).cloned().collect();
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
        Ok(removal) if removal.rows(base) > INVERSE_ROWS => MutationOutcome::refuse(OutcomeCode::InverseRefused, format!("{noun} \"{}\" takes {} records with it where one removal restores at most {INVERSE_ROWS}; delete in parts.", roots.first().map(String::as_str).unwrap_or_default(), removal.rows(base)), [roots.first().cloned().unwrap_or_default()]),
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

/// ↩️ The inverse rows of a delete leaf: empty when a root is unknown, the removal is refused or it restores more than [`INVERSE_ROWS`] rows, else the removal's concrete rows.
pub fn inverse(base: &ModelSnapshot, roots: &[String]) -> Vec<ModelMutation> {
    closure(base, roots).ok().filter(|removal| removal.rows(base) <= INVERSE_ROWS).map(|removal| removal.inverse(base)).unwrap_or_default()
}

/// 🏷️ The sparse diff that deletes the properties and classifications keyed by `id`, a record of a library that is not an element (a type): data belongs to its holder and leaves with it.
pub fn data_diff(base: &ModelSnapshot, id: &str) -> ModelDiff {
    ModelDiff {
        properties: base.properties.contains_key(id).then(|| KeyedDelta::one(id, Entry::Deleted)),
        classifications: base.classifications.contains_key(id).then(|| KeyedDelta::one(id, Entry::Deleted)),
        ..ModelDiff::default()
    }
}

/// ↩️ One concrete setter per property and per classification keyed by `id`: the rows that give a deleted holder its data back once it is recreated.
pub fn data_rows(base: &ModelSnapshot, id: &str) -> Vec<ModelMutation> {
    let properties = base.properties.get(id).into_iter().flatten().flat_map(|(set, properties)| {
        properties.iter().map(move |(property, value)| ModelMutation::SetElementProperty(SetElementProperty { id: id.to_string(), pset: set.clone(), property: property.clone(), value: value.clone() }))
    });
    let classifications = base.classifications.get(id).into_iter().flatten().map(|(system, code)| ModelMutation::SetElementClassification(SetElementClassification { id: id.to_string(), system: system.clone(), code: code.clone() }));
    properties.chain(classifications).collect()
}

/// 🗂️ The holders (element or type id, code) classified in the classification system `system`, in id order.
pub fn classified_by(base: &ModelSnapshot, system: &str) -> Vec<(String, String)> {
    base.classifications.iter().filter_map(|(holder, set)| set.get(system).map(|code| (holder.clone(), code.clone()))).collect()
}
