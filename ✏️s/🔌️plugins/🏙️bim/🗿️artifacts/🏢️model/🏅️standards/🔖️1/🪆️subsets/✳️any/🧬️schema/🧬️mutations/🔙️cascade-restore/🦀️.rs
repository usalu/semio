//! 🔙️ The concrete restoring half of the shared cascade: the aggregate states the creates and setters that give a removal back, so the delete leaves, which only name [`RemovalInverse`], stay below the leaves they restore.

use super::cascade::{Removal, RemovalInverse};
use super::set_component_override::SetComponentOverride;
use super::set_element_classification::SetElementClassification;
use super::set_element_property::SetElementProperty;
use super::set_space_conditions::SetSpaceConditions;
use super::ModelMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

macro_rules! restore {
    ($($field:ident => $variant:ident($module:ident, $record:ident);)*) => {
        impl RemovalInverse for ModelMutation {
            /// ↩️ One concrete row per removed record in storage order: the store replays the vector reversed, so sites come last and are
            /// recreated first, then buildings, storeys, grid lines, storey contents, openings, and finally the property and classification setters.
            fn removal_inverse(removal: &Removal, base: &ModelSnapshot) -> Vec<Self> {
                let mut rows = Vec::new();
                $(
                    for id in &removal.$field {
                        if let Some(record) = base.$field.get(id) {
                            rows.push(ModelMutation::$variant(super::$module::$variant { id: id.clone(), $record: record.clone() }));
                        }
                    }
                )*
                for key in &removal.component_overrides {
                    if let Some(record) = base.component_overrides.get(key) {
                        rows.push(ModelMutation::SetComponentOverride(SetComponentOverride { component: record.component.clone(), name: record.name.clone(), value: record.value.clone() }));
                    }
                }
                for id in &removal.properties {
                    for (set, properties) in base.properties.get(id).into_iter().flatten() {
                        for (property, value) in properties {
                            rows.push(ModelMutation::SetElementProperty(SetElementProperty { id: id.clone(), pset: set.clone(), property: property.clone(), value: value.clone() }));
                        }
                    }
                }
                for id in &removal.classifications {
                    for (system, code) in base.classifications.get(id).into_iter().flatten() {
                        rows.push(ModelMutation::SetElementClassification(SetElementClassification { id: id.clone(), system: system.clone(), code: code.clone() }));
                    }
                }
                for id in &removal.space_conditions {
                    if let Some(record) = base.space_conditions.get(id) {
                        rows.push(ModelMutation::SetSpaceConditions(SetSpaceConditions::stating(id, record)));
                    }
                }
                for id in &removal.element_options {
                    if let Some(member) = base.element_options.get(id) { rows.push(ModelMutation::SetElementOption(super::set_element_option::SetElementOption { id: id.clone(), option: Some(member.target.clone()) })); }
                }
                for id in &removal.element_worksets {
                    if let Some(member) = base.element_worksets.get(id) { rows.push(ModelMutation::SetElementWorkset(super::set_element_workset::SetElementWorkset { id: id.clone(), workset: Some(member.target.clone()) })); }
                }
                rows.reverse();
                rows
            }

            fn data_inverse(base: &ModelSnapshot, id: &str) -> Vec<Self> {
                let properties = base.properties.get(id).into_iter().flatten().flat_map(|(set, properties)| {
                    properties.iter().map(move |(property, value)| ModelMutation::SetElementProperty(SetElementProperty { id: id.to_string(), pset: set.clone(), property: property.clone(), value: value.clone() }))
                });
                let classifications = base.classifications.get(id).into_iter().flatten().map(|(system, code)| ModelMutation::SetElementClassification(SetElementClassification { id: id.to_string(), system: system.clone(), code: code.clone() }));
                properties.chain(classifications).collect()
            }
        }
    };
}

restore! {
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
