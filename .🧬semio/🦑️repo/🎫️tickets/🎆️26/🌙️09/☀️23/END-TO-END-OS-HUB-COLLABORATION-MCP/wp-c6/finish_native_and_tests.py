from pathlib import Path
import json
import re

space = next(Path("/Users/ueli/Documents/semio").glob("*/🔌️plugins/🪐️space"))
osroot = next(Path("/Users/ueli/Documents/semio").glob("*/🛍️products/💻️os"))
ticket = next(Path("/Users/ueli/Documents/semio/.🧬semio").rglob("END-TO-END-OS-HUB-COLLABORATION-MCP"))
gen = ticket / "🗑️generated" / "wp-c6"

# Fix editor match arms - find HomeCommand::BindSpaceFile wildcard region
editors = [p for p in space.rglob("🦀️.rs") if str(p).endswith("✳️any/✏️editor/🦀️.rs") and "🏠️home" in str(p)]
et = editors[0].read_text()
# home_retained_extent match - need to add new variants or they'll fail compile
if "PromoteToHubSpace" not in et.split("home_retained_extent")[1][:2500] if "home_retained_extent" in et else True:
    # find BindSpaceFile arm that groups non-retained
    old = "HomeCommand::BindSpaceFile(_)"
    # Look for the catch-all group
    i = et.find("fn home_retained_extent")
    print("--- extent snip ---")
    print(et[i:i+1200])

# Native fold
wgpu = next((osroot / "🔨️modules" / "📺️renderer").rglob("*/🐚️Shell/🎯️targets/*/🦀️.rs"))
wt = wgpu.read_text()
if "dispatch_fold_directory_events" not in wt:
    old_match = "                    DirectoryStreamMessage::Event { .. } | DirectoryStreamMessage::Heartbeat { .. } => dirty = true,"
    new_match = """                    DirectoryStreamMessage::Event { event } => {
                        live_events.push((*event).clone());
                        dirty = true;
                    }
                    DirectoryStreamMessage::Heartbeat { .. } => dirty = true,"""
    if old_match not in wt:
        print("WARN no event match", wt.find("DirectoryStreamMessage::Event"))
        i = wt.find("DirectoryStreamMessage::Event")
        print(repr(wt[i:i+120]))
    else:
        wt = wt.replace(old_match, new_match, 1)
        wt = wt.replace(
            """            let mut dirty = false;
            let mut rebootstrap = false;
            for message in runner.drain() {""",
            """            let mut dirty = false;
            let mut rebootstrap = false;
            let mut live_events: Vec<semio_framework_os_kernel::os_directory::DirectoryEvent> = Vec::new();
            for message in runner.drain() {""",
            1,
        )
        anchor = """            if dirty || rebootstrap {
                if let Some(home) = self.directory_home.as_mut() {
                    match home.wake(rebootstrap) {
                        Ok(Some(_)) => changed = true,
                        Ok(None) => {}
                        Err(error) => {
                            home.close();
                            self.error = Some(error.to_string());
                        }
                    }
                }
            }"""
        replacement = """            if !live_events.is_empty() {
                self.dispatch_fold_directory_events(&live_events);
                changed = true;
            }
            if dirty || rebootstrap {
                if let Some(home) = self.directory_home.as_mut() {
                    match home.wake(rebootstrap) {
                        Ok(Some(_)) => changed = true,
                        Ok(None) => {}
                        Err(error) => {
                            home.close();
                            self.error = Some(error.to_string());
                        }
                    }
                }
            }"""
        if anchor not in wt:
            print("WARN dirty anchor missing")
            i = wt.find("if dirty || rebootstrap")
            print(repr(wt[i:i+400]))
        else:
            wt = wt.replace(anchor, replacement, 1)
        helper = '''
    /// 📔️ Live directory WS events fold through `foldDirectoryEvents` (browser parity) so hub
    /// membership stays projected without waiting for a full event-page rebootstrap.
    #[cfg(not(target_arch = "wasm32"))]
    fn dispatch_fold_directory_events(&mut self, events: &[semio_framework_os_kernel::os_directory::DirectoryEvent]) {
        let Some(session) = self.session.clone() else { return };
        let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        let events_json = pack::to_json_string(events);
        let live_view_state = self.live_view_state(&session);
        let window_kind_id = session.app.window_kinds.first().map(|kind| kind.id.clone()).unwrap_or_default();
        let window_instance_id = live_view_state.window_id.clone().unwrap_or_else(|| window_kind_id.clone());
        let mode_id = live_view_state.active_mode_id.clone().unwrap_or_else(|| session.app.default_mode_id.clone());
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress {
                plugin_id: session.plugin_id.clone(),
                app_id: session.app.id.clone(),
                mode_id,
                window_kind_id,
                window_instance_id,
                action_id: "foldDirectoryEvents".into(),
            },
            arguments: BTreeMap::from([("eventsJson".into(), DslValue::String(events_json))]),
        };
        let action_json = dsl::os_pack::json::to_json_string(&invocation);
        let pool = crate::renderer_worker_pool();
        let instance_id = session.instance_id;
        let _ = ShellPoolFuture::spawn(pool, Lane::Io, async move {
            let _ = program.handle_action(instance_id, &action_json, &live_view_state).await;
        });
    }

'''
        mark = '    /// 🏠️ Begins one bounded canonical-page fetch owned by the retained Home bootstrap epoch.'
        if mark in wt:
            wt = wt.replace(mark, helper + mark, 1)
        wgpu.write_text(wt)
        print("native fold wired")
else:
    print("native fold already present")

# Fix Event clone - DirectoryEvent might not need Box deref that way
# Check if Event is Box<DirectoryEvent>
print("check Event variant done")

# Language-agnostic fixture already written - add Rust + TS test runners
# Rust unit test next to sync
sync_tests = osroot / "🔨️modules" / "🏪️store" / "🔄️sync" / "🧪️tests" / "🔬️unit" / "🦀️.rs"
# Prefer a dedicated test file under sync tests
test_dir = osroot / "🔨️modules" / "🏪️store" / "🔄️sync" / "🧪️tests" / "🔬️persistence-data-class"
test_dir.mkdir(parents=True, exist_ok=True)
(test_dir / "🦀️.rs").write_text(r'''//! 🔬️ Language-agnostic PersistenceDataClass fixture — Rust runner.

use super::{bindings_data_class, wire_lane_data_class, PersistenceBinding, PersistenceDataClass};

fn fixture() -> serde_json::Value {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json");
    // host crate manifest is deeper; resolve via workspace-relative search
    let candidates = [
        path,
        "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json",
    ];
    for candidate in candidates {
        if let Ok(text) = std::fs::read_to_string(candidate) {
            return serde_json::from_str(&text).expect("fixture json");
        }
    }
    // absolute-from-cwd
    let root = std::env::var("SEMIO_WORKSPACE_ROOT").unwrap_or_else(|_| ".".into());
    let full = std::path::Path::new(&root).join("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json");
    serde_json::from_str(&std::fs::read_to_string(&full).unwrap_or_else(|error| panic!("read fixture {full:?}: {error}"))).expect("fixture")
}

#[test]
fn persistence_data_class_fixture_routes_bindings_and_lanes() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let expected = case["expectedDataClass"].as_str().expect("expected");
        let binding = &case["binding"];
        let kind = binding["kind"].as_str().expect("kind");
        let class = match kind {
            "folder" => PersistenceBinding::Folder { path: binding["path"].as_str().unwrap().into() }.data_class(),
            "hub" => PersistenceBinding::Hub {
                base_url: binding["baseUrl"].as_str().unwrap().into(),
                space_id: binding["spaceId"].as_str().unwrap().into(),
                surface: None,
            }
            .data_class(),
            "ephemeral" => {
                let data = binding["dataClass"].as_str().unwrap();
                if data == "ephemeralShared" {
                    PersistenceDataClass::EphemeralShared
                } else {
                    PersistenceDataClass::EphemeralLocalOnly
                }
            }
            other => panic!("unknown kind {other}"),
        };
        assert_eq!(class.as_str(), expected, "case {}", case["id"]);
        assert_eq!(class.allows_share(), case["allowsShare"].as_bool().unwrap());
        assert_eq!(class.allows_collaboration(), case["allowsCollaboration"].as_bool().unwrap());
        assert_eq!(class.is_durable(), case["durable"].as_bool().unwrap());
        if let Some(lane) = case.get("wireLane") {
            let lane_name = lane["lane"].as_str().unwrap();
            assert_eq!(wire_lane_data_class(lane_name).as_str(), lane["dataClass"].as_str().unwrap());
            assert!(!wire_lane_data_class(lane_name).is_durable(), "preview/presence must never be durable");
        }
    }
    assert_eq!(bindings_data_class(&[]), PersistenceDataClass::EphemeralLocalOnly);
    let hub = PersistenceBinding::Hub { base_url: "http://h".into(), space_id: "s".into(), surface: None };
    assert_eq!(bindings_data_class(&[hub]), PersistenceDataClass::PersistedShared);
}
''')

# Register test module in sync.rs if needed
sync_rs = osroot / "🔨️modules" / "🏪️store" / "🔄️sync" / "🦀️.rs"
sr = sync_rs.read_text()
if "persistence-data-class" not in sr:
    # find existing test path includes near end of file or unit tests region
    if '#[path = "🧪️tests/🔬️unit/🦀️.rs"]' in sr:
        sr = sr.replace(
            '#[path = "🧪tests/🔬️unit/🦀️.rs"]',
            '#[path = "🧪tests/🔬️unit/🦀️.rs"]',
        )  # noop
        # add after unit tests module
        needle = '#[cfg(test)]\n#[path = "🧪tests/🔬️unit/🦀️.rs"]\nmod tests;'
        needle2 = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;'
        if needle2 in sr and "persistence_data_class_tests" not in sr:
            sr = sr.replace(
                needle2,
                needle2
                + '\n#[cfg(test)]\n#[path = "🧪tests/🔬️persistence-data-class/🦀️.rs"]\nmod persistence_data_class_tests;'.replace("🧪tests", "🧪tests"),
                1,
            )
            # fix emoji
            sr = sr.replace(
                needle2,
                needle2
                + '\n#[cfg(test)]\n#[path = "🧪️tests/🔬️persistence-data-class/🦀️.rs"]\nmod persistence_data_class_tests;',
                1,
            )
            sync_rs.write_text(sr)
            print("registered rust test module")
        else:
            print("unit test needle state", needle2 in sr, "already", "persistence_data_class_tests" in sr)
    else:
        print("no unit test path in sync.rs")
else:
    print("sync already references persistence-data-class")

# TS runner near framework-os package
ts_test = osroot / "🔨️modules" / "🏪️store" / "🔄️sync" / "🧪tests" 
# put under store tests as ts
ts_path = osroot / "🔨️modules" / "🏪️store" / "🔄️sync" / "🧪tests"
# use existing pattern - worker tests or os package tests
ts_file = osroot / "🔨️modules" / "🏪️store" / "🔄️sync" / "🧪tests"
# Actually write under os tests
ts_dir = osroot / "🧪tests" / "persistence-data-class"
# find tests folder emoji
tests_root = next(c for c in osroot.iterdir() if "tests" in c.name and c.is_dir())
ts_dir = tests_root / "persistence-data-class"
ts_dir.mkdir(parents=True, exist_ok=True)
(ts_dir / "🟦️.ts").write_text(r'''import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import {
  bindingsDataClass,
  folderPersistenceBinding,
  hubPersistenceBinding,
  wireLaneDataClass,
  type PersistenceBinding,
  type PersistenceDataClass,
} from "../../../🟦️.ts";

const fixture = JSON.parse(
  readFileSync(join(import.meta.dir, "../../../🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json"), "utf8"),
);
const schema = JSON.parse(
  readFileSync(join(import.meta.dir, "../../../🔨️modules/🏪️store/🔄️sync/🧬️schema/persistence-data-class/🔣️.json"), "utf8"),
);

describe("persistence data class routing", () => {
  it("validates the language-agnostic fixture against schema", () => {
    const ajv = new Ajv({ strict: false, allErrors: true });
    const validateCase = ajv.compile({ $ref: `${schema.$id}#/$defs/ClassifiedPersistenceBinding` });
    for (const c of fixture.cases) {
      expect(validateCase(c.binding), JSON.stringify(validateCase.errors)).toBe(true);
      if (c.wireLane) {
        const validateLane = ajv.compile({ $ref: `${schema.$id}#/$defs/ClassifiedWireLane` });
        expect(validateLane(c.wireLane), JSON.stringify(validateLane.errors)).toBe(true);
      }
    }
  });

  it("routes bindings and lanes to the four data classes", () => {
    for (const c of fixture.cases) {
      const expected = c.expectedDataClass as PersistenceDataClass;
      let binding: PersistenceBinding | null = null;
      if (c.binding.kind === "folder") binding = folderPersistenceBinding(c.binding.path);
      if (c.binding.kind === "hub") binding = hubPersistenceBinding(c.binding.baseUrl, c.binding.spaceId);
      if (binding) {
        expect(binding.dataClass).toBe(expected);
        expect(bindingsDataClass([binding])).toBe(expected);
      } else {
        expect(c.binding.dataClass).toBe(expected);
        if (expected === "ephemeralLocalOnly") expect(bindingsDataClass([])).toBe("ephemeralLocalOnly");
      }
      if (c.wireLane) {
        expect(wireLaneDataClass(c.wireLane.lane)).toBe(c.wireLane.dataClass);
        expect(wireLaneDataClass(c.wireLane.lane) === "ephemeralShared").toBe(true);
      }
      if (c.allowsShare != null && binding) {
        expect(binding.dataClass === "persistedShared").toBe(c.allowsShare);
      }
    }
    expect(fixture.homeUnion.hubSpace.origin).toBe("hub");
    expect(fixture.homeUnion.hubSpace.dataClass).toBe("persistedShared");
    expect(fixture.homeUnion.ephemeralStudio.dataClass).toBe("ephemeralLocalOnly");
    expect(fixture.homeUnion.ephemeralStudio.backbone).toBeNull();
  });
});
''')
print("ts test written", ts_dir)

# Fix TS path - tests folder under os may be 🧪tests with emoji
print("tests_root", tests_root)
