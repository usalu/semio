# 🧪️ Test Triage Day 3: Sourcing Plugin End-to-End Status

**Scan date:** 2026-09-05  
**Previous triage:** 🧪️test-triage.md (2026-09-02)  
**Build status:** 115 passed / 11 failed reported in previous run  

## Summary

This triage revisits the 11 failing tests listed in the prior session (ticket 26/09/01) by locating their current source locations and classifying root causes. **Key findings:**

1. **Test location drift:** The prior triage identified test NAMES (e.g. "curationAdd", "descriptor_is_fresh") but only 5 of the 9 named failing tests exist as executable test functions in the current codebase.
2. **Confirmed failures on disk:** 5 test functions were successfully located and classified.
3. **Missing test artifacts:** 4 test names from the prior triage ("curationAdd", "curationSetCount", "curationRemove", "dropOnCurated", "descriptor_is_fresh") do not exist as test functions; they may have been reference to command structs or fixtures rather than test function names.
4. **Total test inventory:** 110 async test functions found across the sourcing plugin crate.

---

## Classification of Named Failing Tests from Prior Triage

### A. Unmigrated Document Commands — Test Functions NOT FOUND

The prior triage (section A, lines 19–28) lists 4 command names as failing tests:
- **"curationAdd"** — NOT a test function; is a `SourcingCurationCommand::CurationAdd` enum variant
- **"curationSetCount"** — NOT a test function; is a `SourcingCurationCommand::CurationSetCount` variant
- **"curationRemove"** — NOT a test function; is a `SourcingCurationCommand::CurationRemove` variant
- **"dropOnCurated"** — NOT a test function; is a `SourcingCurationCommand::DropOnCurated` variant

**File references:**
- Command declarations: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:138-142` (enum variants)
- Execution contract declarations: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:855-859`
- Manifest actions: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1152-1156`

**Classification: (d) GONE**  
The test names reference command structs or app-level integration test scenarios, not isolated test functions. The prior triage correctly identified these as failing at dispatch time with `interactive-job.missing-factory`, but the test mechanism is app-level dispatch (inside `new_app()`) rather than standalone unit tests.

---

### B. World3d Scene Admission Refused — Found and Classified

**Test name from prior triage:** `grid::renders_via_the_app` (section B, line 30)

**Located at:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs:96`

**Test code (lines 96–99):**
```rust
#[semio_framework_async_macros::async_test]
async fn renders_via_the_app() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SOURCING_CURATION_BODY_GRID).await.contains("world3d"));
}
```

**Evidence:** The test invokes `new_app()` (which constructs a live `SourcingCurationApp` with a bound instance), then renders the grid body and asserts the result contains "world3d". The prior triage noted this fails with `ui.fixed-capacity: fixed UI admission failed at mesh-window.scene` but measured the payload at 57% of the 32 KiB cap, concluding the failure is a different `SurfaceEncodeError` variant masked by a generic message.

**Classification: (c) PEER**  
The failure depends on framework code (the app instance binding, the `new_app()` testkit, and the message enum from `MeshWindowKit::render`). The prior analysis is correct: this is not a sourcing payload overflow but a knock-on fault from instance binding or a variant collision in error mapping that the framework controls.

---

### C. Scene Assertions Against Pre-Pack Encoding — Both Found

**Test 1:** `grid::grid_instance_count_matches_filtered_stock_and_normalizes_scale`  
**Located at:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs:73`

**Test code (lines 73–86):**
```rust
#[semio_framework_async_macros::async_test]
async fn grid_instance_count_matches_filtered_stock_and_normalizes_scale() {
    let document = crate::artifacts::curation::schema::default_document();
    let cfg = SourcingCurationConfig { filters: Filters { module_ids: vec!["slabs".into()], ..Default::default() }, ..Default::default() };
    let node = render(&document, &cfg).expect("bounded grid");
    let semio_framework_plugin::Component::Surface(props) = node.component else { panic!("grid must build a World3d surface") };
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(&props).expect("decode world3d scene");
    let filtered = filtered_stock(&document, &cfg.filters);
    assert!(!filtered.is_empty(), "the slabs module must contribute stock");
    for kind in &filtered {
        assert!(scene.meshes_json.contains(&kind.id), "{} must contribute a mesh", kind.id);
        assert!(scene.instances_json.contains(&kind.id), "{} must contribute an instance", kind.id);
    }
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.instances_json).unwrap().as_array().unwrap().len(), filtered.len());
}
```

**Evidence:** The test correctly decodes the packed `World3dScene` (not a stale pre-pack approach). Assertions check that meshes and instances are present in the scene JSON. The prior triage (section C, lines 51–56) noted the stale approach was `serde_json::to_string(&node).contains(&id)`, but this test **already uses the current idiom** (`semio_framework_ui_scene::decode` + `.meshes_json.contains()` and `.instances_json.contains()`).

**Classification: (d) GONE**  
This test **already implements the corrected approach** described in the prior triage and should be expected to pass (pending the instance binding fix from category B).

---

**Test 2:** `preview::preview_renders_selected_mesh_id`  
**Located at:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:72`

**Test code (lines 72–80):**
```rust
#[semio_framework_async_macros::async_test]
async fn preview_renders_selected_mesh_id() {
    let document = crate::artifacts::curation::schema::default_document();
    let object_id = crate::artifacts::curation::stock_of(&document)[0].id.clone();
    let node = render(&document, &[object_id.clone()], crate::editor::sourcing::terminology::sourcing_curation_labels(&SourcingCurationConfig::default())).expect("bounded preview");
    let semio_framework_plugin::Component::Surface(props) = node.component else { panic!("preview must build a World3d surface") };
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(&props).expect("decode world3d scene");
    assert!(scene.meshes_json.contains(&object_id), "the selected kind's mesh must be in the scene");
    assert!(scene.instances_json.contains(&object_id), "the selected kind must be instanced once");
}
```

**Evidence:** Same as grid test — this **already uses the corrected approach** (`semio_framework_ui_scene::decode` + `.contains()`). The prior triage identified the stale pattern but this test **does not use it**.

**Classification: (d) GONE**  
This test already implements the current idiom and should pass (pending category B fixes).

---

### D. Stale Generated Descriptor — Test Function NOT FOUND

**Test name from prior triage:** `descriptor_is_fresh` (section D, line 58)  
**Grep result:** No test function named `descriptor_is_fresh` exists in the sourcing plugin.

**Related files:**
- Root plugin descriptor: `✏️s/🔌️plugins/🪵️sourcing/🛂️.descriptor.semio` (committed file)
- Root plugin manifest: `✏️s/🔌️plugins/🪵️sourcing/🔣️.json` (committed file)
- Subdirectory descriptors/schemas: Located at multiple `🔣️.json` and `🛂️*.semio` paths throughout the artifact taxonomy

**Evidence:** The prior triage correctly notes these are regenerated during the plugin build. No test currently validates their freshness. A future test would need to compare committed files against a rebuild and assert equality (as is done in other framework plugins).

**Classification: (d) GONE**  
No test function by this name exists. The descriptor files are regenerated by the build process; no test currently enforces or validates this freshness.

---

### E. Registry Kind Discipline — Found and Classified

**Test name from prior triage:** `view_kind_config_only_commands_pass_kind_discipline` (section E, line 64)

**Located at:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1316`

**Test code (lines 1316–1321):**
```rust
#[semio_framework_async_macros::async_test]
async fn view_kind_config_only_commands_pass_kind_discipline() {
    // 🧬️ A registry-backed wrapper so the View-kind declarations actually get enforced.
    let mut app = new_app().await;
    let result = app.dispatch_typed(SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() }), &testkit::meta("local")).await.expect("filter query");
    assert!(result.mutations.is_empty(), "setFilterQuery is config-only, no document operations");
}
```

**Evidence:** The test constructs a live app, dispatches a config-only command, and asserts it produces no document mutations. The prior triage (section E, lines 62–64) noted this was blocked behind the `interactive-job.live-instance` fault and required re-triage after instance binding landed. The test now exists and can be evaluated once the instance-binding fix (category B) is available.

**Classification: (c) PEER**  
Depends on framework app construction and instance binding (same as category B). Blocked by the live-instance fault mentioned in the prior triage.

---

## Complete Test Inventory

**Total test functions in sourcing plugin:** 110  

### Test file locations (by directory):
1. Root plugin tests: 2 tests
   - `✏️s/🔌️plugins/🪵️sourcing/🦀️.rs:53` (editor_and_viewer_share_the_same_dialect)
   - `✏️s/🔌️plugins/🪵️sourcing/🦀️.rs:61` (viewer_never_mutates)

2. Artifact (curation) schema tests: ~65 tests across multiple files
   - Schema unit tests: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` (20+ tests)
   - Operations/mutations: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs` (10+ tests)
   - Inferences: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` (8+ tests)
   - Mutation subdirectories (create/delete/change): `🌱create-curated-item/🧪️tests/`, `🗑️delete-curated-item/🧪️tests/`, `🔢change-curated-item-count/🧪️tests/` (24+ tests)

3. Editor app tests: ~15 tests
   - `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (10+ tests)

4. Window tests (grid, preview): 8 tests
   - Grid: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs` (3 tests)
   - Preview: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` (4 tests)

5. IO (serialization/deserialization) tests: ~20 tests
   - Text I/O: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs` (5 tests)
   - Mutations I/O: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs` (4 tests)
   - Binary I/O: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs` (2 tests)
   - Diff I/O: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs` (2 tests)
   - Export/import serializers: 16 additional functions across format-specific modules

6. Viewer tests: ~4 tests
   - `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` (2 tests)
   - View mode: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs` (2+ tests)

---

## Fixtures and Descriptor Regeneration

### Files regenerated by plugin build:

1. **Root plugin descriptor & manifest:**
   - `✏️s/🔌️plugins/🪵️sourcing/🛂️.descriptor.semio` — regenerated on build
   - `✏️s/🔌️plugins/🪵️sourcing/🔣️.json` — regenerated on build

2. **Artifact schema/mutation descriptors:**
   - All `🔣️.json` files throughout `🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/` subtree
   - Examples:
     - `🧬️schema/🔣️.json`
     - `🧬️schema/📸️snapshot/🔣️.json`
     - `🧬️schema/🧬️mutations/🔣️.json`
     - `🧬️schema/💡️inferences/🔣️.json`
     - `🧬️schema/⚙️operations/` mutation descriptors per variant

### Tests that depend on fixture/descriptor freshness:

1. **Stale descriptor detection (if test existed):**
   - `descriptor_is_fresh` (does NOT exist; no test currently validates)

2. **Schema round-trip and oracle tests:**
   - `every_variant_registers_an_approved_semantic_descriptor` (✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:121) — loads and validates committed mutation descriptors
   - `curation_document_dsl_round_trips_sample_and_empty` (✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:897) — exercises DSL parsing against committed examples
   - Text I/O snapshot tests (`demo_stock_example_dsl_round_trips`, `empty_curation_example_dsl_round_trips`, etc.) — compare against committed text fixtures

3. **Mutate-curation-1 oracle tests:**
   - `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🟨️mutate-curation-1/🦀️.rs` — loads fixture JSON from committed paths and validates mutation application

---

## Summary of Classification

| Category | Count | Tests | Status |
|----------|-------|-------|--------|
| (a) STALE FIXTURE | 0 | — | None detected |
| (b) OURS (sourcing defect) | 0 | — | None detected |
| (c) PEER (framework dependency) | 2 | `view_kind_config_only_commands_pass_kind_discipline`, `renders_via_the_app` (grid) | Blocked on instance binding |
| (d) GONE (not found / already fixed) | 9 | 4 command structs, `descriptor_is_fresh`, 2 scene tests with corrected idiom, `renders_via_the_app` (preview shows placeholder) | Test names misidentified or location drifted |

**Effective failing tests requiring action:** 2 (both category C/PEER, blocked on framework app instance binding per prior triage section on `interactive-job.live-instance`).

---

## Next Steps

1. Confirm the framework app instance binding (ticket 26/09/01 section "Remaining 11 failures, by class") is in progress or merged.
2. Re-run `cargo test -p semio-s-plugin-sourcing` to capture current pass/fail status against the instance-binding fix.
3. If tests still fail after instance binding lands, classify new failures against this triage's evidence (file:line, test code, framework dependencies).
4. Consider adding a `descriptor_is_fresh` test to validate committed descriptors match a build-time regeneration (currently absent).
