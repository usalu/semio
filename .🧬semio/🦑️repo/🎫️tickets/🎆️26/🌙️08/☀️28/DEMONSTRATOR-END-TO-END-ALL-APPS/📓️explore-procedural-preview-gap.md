# 🌀️ Procedural preview gap investigation

## 1. Known gap from §3 of 📓️app-generator.md

**Quote from §3 (lines 48-54):**

> Preview window (edit mode) — **STRUCTURAL GAP.** Both `Procedural3dPlayApp::handle` (`✏️editor/🦀️component.rs:381`) and `::render` (:441) construct a **fresh, empty** `FlowEvalSession` on every call (`FlowEvalSessionState { eval_json: String::new(), .. }`, `🖥️host/🦀️component.rs:2283-2300`). `render()` never calls `session.tick()`/`host.evaluate_step()`; it only reads `session.eval_json()`, which is therefore always `""` (`👁️preview/🦀️component.rs:62-63`). `preview_payload_from_eval_with_session` short-circuits on empty input to `("[]", "[]")` (`✏️editor/🦀️component.rs:943-945`) → zero meshes, zero instances.

**Proposed fix from §7 (lines 110-114):**

> Fixes, ordered:
> 1. **Persist the eval session** — hold a `Mutex<FlowEvalSession>` (the documented framework pattern) instead of constructing a fresh one in `handle`/`render`, or persist `eval_json` on `Procedural3dConfig`.
> 2. Strengthen the preview window test to assert non-empty mesh/instance JSON, so this cannot regress.
> 3. Thread `InteractionView` into `render`/`context_menu` (cross-cutting, shared with aggregator/aussuchen/generator).

---

## 2. Current implementation (actual code sites)

### Where the empty session is created:

**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

**Line 146 in `generation3d_render_body()`:**
```rust
fn generation3d_render_body(body_key: &str, document: &Generation3dSnapshot, config: &Generation3dConfig, marks: &PreviewInteractionMarks) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = generation3d_labels(config);
    let active_utility = config.active_utility_id.as_str();
    let session = FlowEvalSession::new();  // 👈 FRESH EMPTY SESSION
    let node = match body_key {
        flow_window::GENERATION_3D_PLAY_BODY_MAIN => flow_window::render(document, config, &session, marks),
        edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, &session, active_utility, marks),
        // ... other branches ...
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}
```

### Where the preview window reads from config (NOT from the session):

**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`

**Lines 62-64 in `render()`:**
```rust
pub fn render(document: &Generation3dSnapshot, config: &Generation3dConfig, session: &FlowEvalSession, active_utility: &str, marks: &PreviewInteractionMarks) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let eval_json = config.preview_eval_text.clone().unwrap_or_default();  // 👈 READS FROM CONFIG, NOT SESSION
    let payload = preview_payload(&eval_json, &document.fixture, config, Some(session), marks);
```

The session parameter is passed but NEVER TICKED. The session is only used as a read-only view; the actual eval data comes from `config.preview_eval_text`.

### How eval_json gets populated (the intended path):

**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️.rs`

**Lines 14-32:**
```rust
pub fn handle(_payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = flow_host_with_session(fixture, session);
    let more = session.tick(&mut host);  // 👈 SESSION TICKED HERE
    let mut effects = if more { vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(103), action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
    let eval_json = session.eval_json().to_string();  // 👈 READ FROM TICKED SESSION
    // ... extension handling ...
    let config_mutations = vec![Generation3dConfigMutation::SetPreviewEval { eval_text: (!eval_json.is_empty()).then_some(eval_json) }];  // 👈 PERSISTED TO CONFIG
    Ok(Emit { effects, config_mutations, ..Default::default() })
}
```

---

## 3. Mechanism explanation

**Why the session is "always-fresh, never-ticked":**

1. **Fresh:** Each render call creates a new `FlowEvalSession::new()` at line 146 of the main editor file. This yields an empty struct with `eval_json: String::new()`.

2. **Never-ticked:** The fresh session is never passed to `session.tick(&mut host)`. Only in the `flowEvalTick` command handler (separate from render) is the session ticked.

**Who is supposed to tick it:**

- **In commands:** The `flowEvalTick` handler calls `session.tick(&mut host)` and then persists the result to `config.preview_eval_text` via `SetPreviewEval`.
- **In render:** Nothing ticks it; the render path is read-only and uses the persisted `config.preview_eval_text`.

**Existing tick/evaluate API on FlowEvalSession:**

- `session.tick(&mut host) → bool` — advances evaluation, returns true if more steps are pending
- `session.eval_json() → &str` — reads current eval JSON (empty if never ticked)
- `flow_host_with_session(fixture, session) → FlowHost` — constructs a host bound to the session

**Call sites using the tick pattern (grep results):**

1. **`flowEvalTick` handler** (lines 14-32, file above): `session.tick(&mut host)` after creating host
2. **`pending_effects`** (lines 1004-1012 of main editor file):
   ```rust
   let mut session = FlowEvalSession::new();
   let host = flow_host_with_session(&doc.snapshot.fixture, &session);
   if session.sync(&host) { /* dispatch flowEvalTick */ }
   ```
   Uses `session.sync()` not `session.tick()`, but same pattern.
3. **Flow plugin's evaluation** (search results show flow editor has its own eval infrastructure with stored sessions)

---

## 4. Ticket status check

**File:** `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/PROCEDURAL-3D-END-TO-END/🎫️ticket.json`

**Status:** `open`

**Description:** "Get procedural 3d (app `generation3d`...) working end to end...every window must render non-empty content (the flow/node-graph window and the 3d scene preview in particular)"

**From 📓️status.md (lines 50-73):**

The real root cause was identified: gen3d's tool proofs lacked a `factory_type`, making all 23 `Migrated` actions dispatch-dead at runtime. This prevented `flowEvalTick` from running (→ `preview_eval_text` never written → preview empty). The fix implemented in that ticket adds `Generation3dBoundedCommandJobFactory` with proper publication lanes and factory registration.

**Verdict:** The ticket IS aware of the gap and CLAIMS to have fixed it by wiring up the factory. However, the status shows the fix was only code-reviewed and awaits runtime verification (stdio gates, boot attempts, etc.). The preview gap itself (fresh session in render, never ticked) is the *symptom* of the underlying factory wiring, not a separate bug.

---

## 5. Fix scope and ownership

### Where the fix belongs:

**Dual location (both are necessary):**

1. **Framework contract** (already documented at `🧰️framework/…/🔌️plugin/🦀️component.rs:10489`):
   - The documented pattern is a `Mutex<FlowEvalSession>` held once per dispatch.
   - Gen3d should follow this pattern instead of creating fresh sessions in both `handle()` and `render()`.

2. **Procedural plugin** (generation3d artifact):
   - Store the ticked session in the app state or config.
   - Reuse it in render instead of creating a fresh one.
   - OR: ensure `flowEvalTick` runs before every render (currently ticked by pending_effects dispatch chain).

### Rough size estimate:

- **Small to medium:** ~50-200 lines
  - Add a `Mutex<FlowEvalSession>` field to `Generation3dConfig` or app state
  - Modify `generation3d_render_body()` to reuse rather than create fresh
  - Ensure `pending_effects` properly triggers ticks before render is called
  - Modify preview window's test to assert non-empty meshes/instances (lines 126-145 already has regression guard comment at line 135-139)

- **Note:** The prerequisite is that `flowEvalTick` dispatch chain (via `pending_effects`) must be wired—that's what the sibling procedural-3d-end-to-end ticket (26/09/03) is fixing.

---

## 6. Test regression guard

**Current test (lines 126-145 of preview/🦀️.rs):**
```rust
#[test]
fn renders_world_preview_scene() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app();
    crate::editor::generation3d::testkit::drain_flow_eval_ticks(&mut app);  // TICKS ARE DRAINED
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW);
    assert!(json.contains("world-3d"));
    // 🐛️ Regression guard for the empty-scene defect: ... (lines 135-139)
    let value: serde_json::Value = serde_json::from_str(&json).expect("preview render must be valid json");
    let meshes_json = find_json_string_field(&value, "meshesJson").expect("world-3d scene must carry a meshesJson field");
    let instances_json = find_json_string_field(&value, "instancesJson").expect("world-3d scene must carry an instancesJson field");
    assert_ne!(meshes_json, "[]", "hexagonal-mushroom-column must tessellate into non-empty preview meshes");  // ✅ STRONG ASSERTION
    assert_ne!(instances_json, "[]", "hexagonal-mushroom-column must produce non-empty preview instances");  // ✅ STRONG ASSERTION
}
```

The test already calls `drain_flow_eval_ticks(&mut app)` to flush pending ticks before rendering, so it catches the gap. The strong assertions (comparing against `"[]"`) exist and are documented as regression guards. This test will FAIL until the factory wiring (26/09/03 ticket) is complete.
