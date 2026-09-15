//! 🕹️ What one framework-reserved interaction verb repaints in this artifact — for BOTH roles.
//!
//! 🧾️ Before this lane the procedural 3d surfaces declared no `ArtifactApp::interaction_scope`, so
//! `dispatch_interaction_action` kept the framework's widest `UiDirtyScope::Full` and a single pointer
//! move re-rendered every window body, every panel body, the utilities/tools/engagements rails, the
//! labels, the measures and the app-static catalogue — on BOTH renderers
//! (`📓️wgpu-selection-roundtrip-2026-09-15.md` §7). The narrow answer is not hand-written per window
//! here: the framework derives it from the app's own `window_kind_interactions` declarations
//! (`semio_framework::interaction_declared_refresh_scope`), and
//! `🧫️fixtures/🕹️interaction-scope.json` is the table that derivation must answer.
//!
//! 🛡️ The law that keeps the narrowing honest is `publishes`: every body whose render consumes the
//! interaction marks must be in the scope of the verb that moves its lane. A window added to this
//! artifact that paints a hover and forgets `.window_kind_interactions(...)` fails here before it can
//! go stale in a browser.

use super::*;
use crate::editor::generation3d::unit_tests::serial_execution;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::InteractionVerb;

//#region 🕹️Fixture
const INTERACTION_SCOPE_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🕹️interaction-scope.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InteractionScopeFixture {
    format: String,
    version: u8,
    domain: String,
    undeclared_domain: String,
    roles: Vec<InteractionScopeRole>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InteractionScopeRole {
    id: String,
    declared_window_bodies: Vec<String>,
    declared_panel_bodies: Vec<String>,
    publishes_hover: Vec<String>,
    publishes_selection: Vec<String>,
    quiet: Vec<String>,
    verbs: Vec<InteractionScopeRow>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InteractionScopeRow {
    verb: String,
    window_bodies: Vec<String>,
    panel_bodies: Vec<String>,
    measures: bool,
}

fn interaction_scope_fixture() -> InteractionScopeFixture {
    let fixture: InteractionScopeFixture = serde_json::from_str(INTERACTION_SCOPE_FIXTURE_JSON).expect("interaction scope fixture");
    assert_eq!(fixture.format, "semio.generation3d.interaction-scope");
    assert_eq!(fixture.version, 1);
    fixture
}

fn role_definition(role: &str) -> semio_framework_plugin::AppDefinition {
    match role {
        "editor" => create_generation3d_app(),
        "viewer" => crate::viewer::generation3d::create_generation3d_viewer(),
        other => panic!("fixture names an unknown role {other}"),
    }
}

fn declared_scope(definition: &semio_framework_plugin::AppDefinition, verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
    let windows = semio_framework_plugin::interaction_window_bodies_by_domain(definition);
    let panels = semio_framework_plugin::panel_leaf_body_keys(definition);
    semio_framework_plugin::interaction_declared_refresh_scope(verb, domains, &windows, &panels)
}

fn scope_parts(scope: &UiDirtyScope) -> (Vec<String>, Vec<String>, bool) {
    match scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } => {
            assert!(!utilities && !tools && !engagements && !labels, "an interaction verb moves no utility/tool/engagement/label lane");
            (window_bodies.clone(), panel_bodies.clone(), *measures)
        }
        other => panic!("a declared interaction scope must be partial, got {other:?}"),
    }
}
//#endregion 🕹️Fixture

//#region 🕹️Laws
/// 🐢️ Every reserved verb, both roles: the derived scope IS the fixture's row, body for body.
#[test]
fn every_reserved_verb_repaints_exactly_the_declared_surfaces() {
    let _serial = serial_execution::lock();
    let fixture = interaction_scope_fixture();
    let domain = fixture.domain.as_str();
    for role in &fixture.roles {
        let definition = role_definition(&role.id);
        assert_eq!(role.verbs.len(), InteractionVerb::ALL.len(), "role {} must score every reserved verb", role.id);
        for row in &role.verbs {
            let verb = InteractionVerb::of_action(&row.verb).unwrap_or_else(|| panic!("{} is not a reserved interaction verb", row.verb));
            let scope = declared_scope(&definition, verb, &[domain]).unwrap_or_else(|| panic!("role {} verb {} derived no scope", role.id, row.verb));
            let (windows, panels, measures) = scope_parts(&scope);
            assert_eq!(windows, row.window_bodies, "role {} verb {} window bodies", role.id, row.verb);
            assert_eq!(panels, row.panel_bodies, "role {} verb {} panel bodies", role.id, row.verb);
            assert_eq!(measures, row.measures, "role {} verb {} measures", role.id, row.verb);
            eprintln!("[SCOPE] {} {} windows={windows:?} panels={panels:?} measures={measures}", role.id, row.verb);
        }
    }
}

/// 🪟️ The fixture's rosters ARE the app's own declarations — the table can never describe a shell the
/// artifact does not build, which is what lets the TypeScript twin simulate a refresh pass from it.
#[test]
fn the_fixture_rosters_are_the_declared_surfaces() {
    let _serial = serial_execution::lock();
    let fixture = interaction_scope_fixture();
    for role in &fixture.roles {
        let definition = role_definition(&role.id);
        let windows: Vec<String> = definition.window_kinds.iter().map(|window| window.body_key.clone()).collect();
        assert_eq!(windows, role.declared_window_bodies, "role {} declared window bodies", role.id);
        assert_eq!(semio_framework_plugin::panel_leaf_body_keys(&definition), role.declared_panel_bodies, "role {} declared panel bodies", role.id);
    }
}

/// 🛡️ A verb never narrows below the surfaces that publish its lane: hover reaches every body that
/// paints a hover mark, and the three selection verbs reach every body that paints a selection.
#[test]
fn no_verb_narrows_below_the_surfaces_that_publish_its_lane() {
    let _serial = serial_execution::lock();
    let fixture = interaction_scope_fixture();
    let domain = fixture.domain.as_str();
    for role in &fixture.roles {
        let definition = role_definition(&role.id);
        for verb in InteractionVerb::ALL {
            let scope = declared_scope(&definition, verb, &[domain]).unwrap_or_else(|| panic!("role {} verb {verb:?} derived no scope", role.id));
            let owed: &[String] = match verb {
                InteractionVerb::Hover => &role.publishes_hover,
                InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll => &role.publishes_selection,
                InteractionVerb::SetSelectionMode | InteractionVerb::SetGranularity => &role.publishes_hover,
            };
            for body in owed {
                assert!(scope.wants_window_body(body) || scope.wants_panel_body(body), "role {} verb {verb:?} dropped {body}, which publishes its lane", role.id);
            }
            for body in &role.quiet {
                if verb == InteractionVerb::Hover {
                    assert!(!scope.wants_window_body(body) && !scope.wants_panel_body(body), "role {} hover must not reach {body}", role.id);
                }
            }
        }
    }
}

/// 🚫️ A domain this artifact never declared is not a narrowing opportunity — the framework keeps its
/// widest answer, exactly as it did for every verb before the declarations existed.
#[test]
fn an_undeclared_domain_keeps_the_frameworks_widest_scope() {
    let _serial = serial_execution::lock();
    let fixture = interaction_scope_fixture();
    for role in &fixture.roles {
        let definition = role_definition(&role.id);
        for verb in InteractionVerb::ALL {
            assert!(declared_scope(&definition, verb, &[fixture.undeclared_domain.as_str()]).is_none(), "role {} verb {verb:?} narrowed an undeclared domain", role.id);
            assert!(declared_scope(&definition, verb, &[]).is_none(), "role {} verb {verb:?} narrowed a verb that touched no domain", role.id);
        }
    }
}

/// 🌀️ The counter-model kept permanently: the pre-lane answer. A `Full` scope wants every body of
/// both roles, which is what made one pointer move re-render the whole shell.
#[test]
fn the_full_scope_this_replaces_wanted_every_body() {
    let _serial = serial_execution::lock();
    let fixture = interaction_scope_fixture();
    let full = UiDirtyScope::Full;
    for role in &fixture.roles {
        for body in role.publishes_selection.iter().chain(role.quiet.iter()) {
            assert!(full.wants_window_body(body) && full.wants_panel_body(body), "the framework's widest scope wants {body}");
        }
    }
}
//#endregion 🕹️Laws
