# Audit — every generation3d example vs. the React playground (2026-09-12)

Read-only audit. Goal under audit: all examples of the `generation3d` artifact must load in
`http://127.0.0.1:6018/?plugin=generation3d` with a live 3d preview (`meshes >= 1`).

Base path (elided below as `<artifact>`):
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`

---

## 1. Example inventory

`<artifact>✏️editor/🦀️.rs:1989` `pub fn examples() -> Vec<ExampleSource>` returns exactly **8**
sources, in this order, from `<artifact>📚️examples/<slug>/🦀️.rs` leaves. `📚️examples/🎬️demo-session/`
exists on disk but is **not** in the list — its primary text is a `generation.3d.cmd` command
envelope, not a `flow.fixture` DSL, and would fail to parse if selected
(`📓️example-geometry-tests-2026-09-09.md` §4). `examples()` is wired to the manifest at
`.editor_with_examples::<Generation3dPlayApp>(...)` per the docstring at `🦀️.rs:1985-1987`; the
default boot document is `hexagonal-mushroom-column`
(`<artifact>🧬️schema/🦀️.rs:229` `PROCEDURAL_EXAMPLE_HEX_COLUMN`, matched again in the `"demo"` alias
at `:281`).

Every example directory has 4 asset kinds; only ONE carries real content:

| id | label (en / de) | op chain (`neuron-kind=`) | `.dsl.semio` bytes | `.op.semio` | `.pack.semio` | `.spr.semio` |
|---|---|---|---|---|---|---|
| `hexagonal-mushroom-column` | Hexagonal Mushroom Column / Sechseckige Pilzsäule | `brep.curve.polygon`, `math.vector`, `brep.solid.extrude` | 1494 | 73 | 171 | 170 |
| `rectangle-extrude-volume` | Rectangle Extrude Volume / Rechteck-Extrusionsvolumen | `brep.curve.rectangle`, `math.vector`, `brep.solid.extrude`, `brep.measure.volume` | 1244 | 73 | 171 | 170 |
| `sphere-cut-with-torus` | Sphere Cut With Torus / Kugel mit Torus geschnitten | `brep.prim3d.sphere`, `brep.prim3d.torus`, `brep.bool.cut`, `brep.measure.volume` | 1610 | 73 | 171 | 170 |
| `box-fillet-preview` | Box Fillet Preview / Kantenrundung Vorschau | `brep.prim3d.box`, `brep.solid.fillet` | 932 | 73 | 171 | 170 |
| `sphere-box-fuse` | Sphere Box Fuse / Kugel und Quader vereinen | `brep.prim3d.box`, `brep.prim3d.sphere`, `brep.bool.fuse` | 1044 | 73 | 171 | 170 |
| `face-sweep-extrude` | Face Sweep Extrude / Fläche extrudieren | `brep.curve.rectangle`, `brep.surf.planarFaceWire`, `math.vector`, `brep.sweep.extrude` | 1194 | 73 | 171 | 170 |
| `rectangle-wire-preview` | Rectangle Wire Preview / Rechteck-Draht Vorschau | `brep.curve.rectangle` | 586 | 73 | 171 | 170 |
| `box-shell-preview` | Box Shell Preview / Hohlkörper Vorschau | `brep.prim3d.box`, `brep.solid.shell` | 853 | 73 | 171 | 170 |

The `.op.semio` (73 B, `semio procedural.generation3d.op v1 / edit reuse started="0" actor=example`),
`.pack.semio` (171 B) and `.spr.semio` (170 B) files are byte-identical genesis envelope headers with
**no applied history** across all 8 examples — verified by `cat`. They are not read by the example
mechanism: `<artifact>📚️examples/box-shell-preview/🦀️.rs` (representative of all 8) is
`PRIMARY_TEXT = include_str!("🖼️assets/<slug>/🗣️.dsl.semio")` then
`ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)` — the DSL text is the *only* asset that ever
reaches the manifest. `ExampleSource::into_example_definition`
(`🧰️framework/…/🔌️plugin/🦀️.rs:7492`) maps it 1:1 into `ExampleDefinition.artifact_json`, which is
what `manifest.examples[].artifactJson` (TS) carries.

Per-example native test (`<artifact>📚️examples/<slug>/🧪️tests/🧩️example/🦀️.rs`) is, for all 8:
```rust
#[test]
fn primary_asset_is_nonempty() { assert!(text.len() > 8); }
```
— length-only, proves nothing about geometry. The TypeScript twin at the same path was upgraded
2026-09-09 to validate the fixture schema and recompute closed-form numbers (still not a kernel
evaluation — see §3).

---

## 2. How examples reach the guest at runtime

1. **Manifest.** `examples()` (§1) → `.editor_with_examples(...)` → `PluginManifest.examples[]`,
   each entry `{id, label, iconId, artifactJson}` (`ExampleDefinition`,
   `🧰️framework/…/🔌️plugin/🦀️.rs:7454-7502`).
2. **Boot default.** `VITE_SEMIO_DEFAULT_EXAMPLE` → `defaults.exampleId`
   (`🧰️framework/…/🧑‍💻dev/🟦️.ts:39-41`) → passed into `bootFrameworkOs({..., defaults, ...})`
   (`:44`). `VITE_SEMIO_LOCKED_EXAMPLE` is the separate hard-lock path (`:30`).
3. **Picker.** `ShellHost` builds `exampleOptions` from
   `activePluginManifest?.examples ?? []` filtered by `example.appId === session.app.id`
   (`🏛️ShellHost/🟦️.tsx:8365-8386`) and renders `<NavbarExampleSelect>` (`:8404-8417`). Selecting an
   option does two things: `dispatch({type:"SET_ACTIVE_EXAMPLE_ID", value: exampleId})` (label only)
   and `dispatchActiveExample(exampleId)` (`:8411-8413`), which calls
   `onAction(buildActiveExampleAction(session.app.controllerId, exampleId))` (`:8391-8397`).
4. **Action id.** `buildActiveExampleAction` + `SET_ACTIVE_EXAMPLE_ACTION_ID = "setActiveExample"`
   (`🛠️ShellHelpers/🟦️.tsx:339-347`) — a value, not an inline literal (this replaced an inline
   dispatch, per `📓️example-switch-2026-09-10.md` §3.2).
5. **Guest gesture.** `Generation3dCommand::SetActiveExample` arm
   (`<artifact>✏️editor/🦀️.rs:408-423`, was `:400` before concurrent edits) reads the attached
   preview-window roster off the retained job context and extends its emit's effects with
   `rearm_attached_previews(window_ids)` (`<artifact>✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs:37`),
   which arms one `flowEvalTick` `Effect::DispatchAction` per attached window — the fix that makes the
   switch's own consequence independent of a host `refresh-ui` round trip
   (`📓️example-switch-2026-09-10.md` §2.1/§3.1). `Generation3dPlayApp::pending_effects` is now at
   `<artifact>✏️editor/🦀️.rs:1680` (was `:1655`).
6. **ReadDocument fallback for operator scoping.** The live document (`ReadDocument`,
   `🧰️framework/…/🔌️plugin/🦀️.rs:33826`) can still be genesis (no `neuron-kind` reachable, since a
   fresh app instance has an empty op/pack/spr, matching §1's asset finding). When
   `resolveDocumentOperatorKinds` on the live document is `unresolved`, `ShellHost`
   (`🏛️ShellHost/🟦️.tsx:4538-4545`) falls back to
   `exampleArtifactSources(receiverPlugin?.manifest.examples ?? [], nextSession.app.id)`
   (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:332-349`) — i.e. it scopes the `flow.extension` push from
   **all 8** examples' `artifactJson`, not from what is actually open. `scopeContributionsJson`
   (`🎠️kernel/🟦️.ts:299-315`) then cuts the host→guest contributions to the union of kinds
   `reachableKindsFromUnknown` (`:282-286`) finds in those 8 documents, producing the
   248 635-char/one-page pack recorded live in `📓️contributions-example-scope-2026-09-11.md`.
7. **Picker → label vs. graph.** Confirmed fixed for the *guest* side by 4 native laws in
   `<artifact>✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` (all 8 examples switched in one session,
   asserted no leaked node from the previous example) — see §5 for what remains broken downstream.

---

## 3. Native tests: geometry vs. non-empty-asset only, and linked vs. contributed (unlinked) shape

Three tiers exist; only the **third** is close to what the browser actually runs.

**Tier A — non-empty only.** `<artifact>📚️examples/<slug>/🧪️tests/🧩️example/🦀️.rs` × 8, `text.len() > 8`
(§1). No evaluation.

**Tier B — real Rust kernel evaluation, but LINKED (not the served shape).**
`[[test]] example-geometry` at
`<artifact>📚️examples/🧪️tests/🧩️geometry/🦀️.rs` (declared in `📦️packages/🦀️rust/Cargo.toml`,
`[[test]] name = "example-geometry"`). It calls `install_flow_extension` with
`semio_s_plugin_flow_extension_brep::register(registry)` and
`semio_s_plugin_flow_extension_math::register(registry)` directly (`🦀️.rs:104-105`) — the two kernel
crates are Cargo `[dev-dependencies]`, **compiled into the test binary**. This is a real
`FlowHost::evaluate → tessellate_geometry → parry3d` pipeline that DOES produce meshes ≥ 1 and check
triangle/vertex counts, closedness, volume (own divergence-theorem + `parry3d`'s independent
recompute) and bbox for all 8 examples (`📓️example-geometry-tests-2026-09-09.md` §1.1, §2, §7.1). Its
own docstring calls this "the same linked installation the headless geometry lane uses, which needs
no host half" (`<artifact>🧪️tests/🔬️flow-operators/🦀️.rs`, closing `@see` comment) — i.e. it is
explicitly NOT exercising the contributed/unlinked wire the wasm guest uses in the browser.

Measured result (`cargo test -p semio-s-artifact-procedural-generation3d --test example-geometry`,
2026-09-09): **2 passed / 6 failed** — `rectangle-wire-preview` and `box-shell-preview` pass;
`rectangle-extrude-volume`, `face-sweep-extrude`, `hexagonal-mushroom-column` fail
`blocked-on-extrude-orientation` (kernel `measure.volume` is correct, the tessellated mesh's side
walls are inverted — root-caused to `➡️sweep/🧮️core/🦀️.rs:271`'s `flipped = true` hack, see
example-geometry-tests §7.2); `box-fillet-preview` fails `blocked-on-fillet-kernel` (open shell, 40
free edges, `🧊️brep` subset, §7.3); `sphere-box-fuse`/`sphere-cut-with-torus` fail
`blocked-on-boolean-kernel` (`imprint point does not lie on the face's boundary loop`, or — after a
concurrent kernel edit — a >61 min non-terminating tessellation, §7.8). These are kernel defects
independent of the browser wiring; a third-party oracle (§4) confirms every expected number is
correct, so the fixtures are not at fault.

**Tier C — the actual served (contributed/unlinked) shape.** Root module
`<artifact>🦀️.rs:163` `pub(crate) mod flow_operators;` →
`<artifact>🧪️tests/🔬️flow-operators/🦀️.rs::installed()` registers BOTH a **linked** installer
(`register_linked_flow_extension_installer`) AND a **contributed manifest**
(`install_flow_extension_manifest`) for `brep`/`math`. Its own docstring: "A linked installer SHADOWS
the contributed stub for the same operator id... without it every geometry operator is a
`ContributedExtensionStub`". `test_support::lock()`
(`<artifact>✏️editor/🧪️tests/🔬️test-support/🦀️.rs:9`) calls `flow_operators::installed()`
unconditionally, so **every** test using `test_support::lock()` — including the set-contributions
catalogue tests `a_paged_run_installs_the_contributed_registry` and
`a_one_page_host_shaped_run_indexes_contributed_operators`
(`<artifact>✏️editor/🎮️commands/🧩️set-contributions/🧪️tests/🔬️unit/🦀️.rs:31,80`) — runs with the
linked shadow present, and therefore proves nothing about the served guest's actual
`ContributedExtensionStub → PendingExtension` path
(`📓️guest-operator-catalogue-2026-09-11.md`, `📓️unknown-kind-after-restage-2026-09-11.md` "Native
tests are not the served shape").

The **one** test that does exercise the real served shape is
`a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted`
(`<artifact>✏️editor/🧪️tests/🔬️unit/🦀️.rs:1531`), which wraps the run in
`UnlinkedFlowExtensions::take()` (`:1642-1662`) to retire the linked installers for the law's
duration, asserts the geometry kernel is unaddressable beforehand, drives
`SetActiveExample(PROCEDURAL_EXAMPLE_HEX_COLUMN)`, confirms the preview *faults* with 0 meshes, then
pages a `SetContributions` closure through the retained ladder and asserts `meshes >= 1` afterward.
This is the only native law that proves the exact chain the browser needs — and it only covers
**one** example (`hexagonal-mushroom-column`), not all 8.

---

## 4. Expected geometry per example (for a browser probe to assert against)

Source: `<artifact>📚️examples/<slug>/🧪️tests/🧩️example/🔣️.json`
(schema `s.procedural.generation3d.example-geometry/v1`), cross-checked against `.dsl.semio` sliders.

| example | expect.minTriangles | closed | expect.volume | bbox (min–max) | `t` (tessellation tol.) | `kernelStatus` |
|---|---|---|---|---|---|---|
| `rectangle-wire-preview` | — (wire, 0 tri) | false | — (perimeter 7.0) | `[0,0,0]`–`[2,1.5,0]` | 0.05 | `green` |
| `box-shell-preview` | 24 | true | 3.904 | `[0,0,0]`–`[2,2,2]` | 0.05 | `green` |
| `rectangle-extrude-volume` | 12 | true | 12.0 | `[0,0,0]`–`[2,2,3]` | 0.05 | `blocked-on-extrude-orientation` |
| `face-sweep-extrude` | 12 (from harness run) | true | 12.0 | `[0,0,0]`–`[2,1.5,4]` | 0.05 | `blocked-on-extrude-orientation` |
| `hexagonal-mushroom-column` | 20 (from harness run) | true | 3.897114317030 | `[-0.5,-0.4330127,0]`–`[0.5,0.4330127,6]` | 0.05 | `blocked-on-extrude-orientation` |
| `box-fillet-preview` | 996 (measured; open shell) | **false** | 7.888634924 | `[0,0,0]`–`[2,2,2]` | 0.01 | `blocked-on-fillet-kernel` |
| `sphere-box-fuse` | n/a — refuses/never terminates | — | 7.245424930456 | `[-1.2,-1.2,-1.2]`–`[1.5,1.5,1.5]` (2.7³) | 0.01 | `blocked-on-boolean-kernel` |
| `sphere-cut-with-torus` | n/a — refuses/never terminates | — | 37.841142613316 | 4.4³ | 0.01 | `blocked-on-boolean-kernel` |

For the 6 `blocked-*` examples the volume above is the **correct analytic/quadrature value**, not
what the current kernel returns — a browser probe should NOT expect `meshes >= 1` with the right
shape from these until the linked kernel-defect fixes land (§3); the honest near-term probe target is
`meshes >= 1` at all (any non-degenerate soup), which is what the one Tier-C law (§3) actually checks
for `hexagonal-mushroom-column`.

**Third-party oracles** (both `testOnly`, `productionReachable: false`, registered in
`<artifact>🔮️oracle/🔣️.json`, capability `procedural-3d-1-example-geometry`):

- **`example-geometry-parry3d`** (rust, `parry3d 0.17`) — recomputes volume/COM/AABB from the same
  triangle soup the harness measures, inside `[[test]] example-geometry` itself (Tier B, §3).
- **`example-geometry-scipy`** (python, `scipy`+`numpy`) —
  `<artifact>🧪️tests/📐️example-geometry-3d-1/🐍️.py`, re-derives every one of the 23 committed
  expectations across all 8 examples from the DSL + the packaged brep extension descriptor only (no
  Rust import, no graph evaluation). Run directly: `./.venv/bin/python3 "<artifact>🧪️tests/📐️example-geometry-3d-1/🐍️.py"`
  — 2026-09-09 result: **PASS, all 23 expectations, including the two the kernel cannot yet produce**.
  Also reachable through the repo coordinator:
  `bun nx run @semio-tech/repo-test-domain:test-discover` then
  `bun nx run @semio-tech/repo-test-domain:test-oracle -- --id test-…-📐️example-geometry-3d-1`.
- No `brepjs`/OCCT oracle exists for this artifact — `📓️example-geometry-tests-2026-09-09.md` §1.3
  confirms brepjs/spatial-kernel stays scoped to the `cad` plugin and this plugin has no reachable
  TypeScript evaluation path.
- TypeScript twin (not an independent oracle of the kernel, but of the *fixture*):
  `<artifact>📚️examples/🧪️tests/🧩️geometry/🟦️.ts` + per-example `🟦️.ts`, run via
  `bun nx run @semio-tech/procedural-js:test` (2026-09-09: 35 pass / 238 `expect()`).

---

## 5. Known gaps in the example switch path, with file:line

1. **Picker label vs. graph — fixed on the guest side, 2026-09-10.** Root cause was
   `own effects=0` at `<artifact>✏️editor/🦀️.rs:400` (now `:408-423`): the switch replaced the whole
   fixture but armed nothing, so both windows' recovery depended entirely on a host `refresh-ui`
   round trip through `Generation3dPlayApp::pending_effects`
   (`🏛️ShellHost/🟦️.tsx:4235`/`:4132`/`🛠️ShellHelpers/🟦️.tsx:4139`) — silent when any link in that
   chain didn't fire. Fixed by `rearm_attached_previews`
   (`<artifact>✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs:37`). 4 native laws pin it; TS law in
   `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` ("the example picker dispatches the chosen
   example id..."). **Restage required and was pending** as of `📓️example-switch-2026-09-10.md` §6 —
   confirm current staged wasm date before trusting this in the browser.

2. **Contributions push was O(all loaded plugins), then O(all 8 examples) — not O(open document).**
   `exampleArtifactSources` (§2 step 6) scopes from every example's `artifactJson`, not from what is
   actually open, because the live `ReadDocument` is genesis before any history is applied. This is
   documented as deliberate/interim in `📓️contributions-example-scope-2026-09-11.md`: "the document
   graph is the only honest source" for reachable kinds, and a push-based design is "O(what exists)
   however well scoped" — a demand-driven/pull design is named as the correct end state but **not
   built** (`📓️status.md`, "The OOM is gone" section, final paragraph).

3. **`unknown kind: brep.curve.polygon` / `math.vector` after contributions land (2026-09-11,
   latest recorded state).** Per `📓️unknown-kind-after-restage-2026-09-11.md`: the served guest
   (restaged 14:15, wasm 94 138 842 B) still faults `unknown kind` at eval time even though the
   scoped pack (248 635 chars) installs display-catalogue labels and addresses correctly. Two
   concrete follow-ups named there, not yet confirmed landed:
   - `register_contributed_manifest` still `return`s silently on parse failure (no diagnostic).
   - `invalidate_for_flow_extension_registry` clears `eval_json` but not `previous_snapshot`/
     `previous_channels`, so `FlowHost::evaluate_step` can skip re-dispatch when the tree looks
     unchanged.

4. **`invokeExtension` failed with `window-transient publication is retiring a rejected authority`**
   (`📓️invoke-extension-rejected-authority-2026-09-11.md`) — traced to
   `🔌️plugin/🦀️.rs`'s window-transient publication loop returning `Err` on every incomplete
   `close_step` of a `Closing` publication that also carried a fault, instead of `Ok(())`. Reported
   **fixed in source** (`Ok(())` on incomplete close_step / on `Complete` + terminal-empty) as of
   15:35 that day, but the **served wasm was still the 14:15 build** at time of writing — restage was
   the next required step, not yet confirmed done in this ticket's files.

5. **Sequencing/order defect, native-only, unrelated to the above:**
   `component::fold_contract::set_active_example_artifact_gesture_fits_its_declared_fold_envelope_for_every_example`
   fails for `sphere-box-fuse` — replaying the authored gesture reaches the example's widgets in
   order `size,radius` instead of the declared `radius,size` (`📓️unit-suite-3d-2026-09-09.md` §0,
   listed as excluded from that lane's scope, owner "fold-contract publication lane").

6. **Two remaining widget-move/transform tests are red for a real product defect, not a test bug:**
   `translate_selection_persists_transform_into_flow_graph` and
   `rotate_and_scale_selection_persist_into_flow_graph` fail
   `interactive-job.instance-owner-poisoned` because `FlowHost::history_store_from_baseline`
   (`🌊️flow/🖥️host/🦀️.rs:1969`), `evaluate_step` (`:1042`) and `set_neuron_params` (`:872`) all
   displace owned neural `Dictionary`/`FlowFixture` values they can neither retire (retiring hangs,
   §5 of `📓️unit-suite-3d-2026-09-09.md`) nor safely drop (aborts the pool worker). Explicitly
   reverted rather than landed; needs "one design pass" per that report §5/§9.

7. **Envelope-load class-D failure specific to generation3d, downstream of the framework fix.**
   `📓️envelope-load-2026-09-10.md` §5.1 fixed the framework-level fault-vs-pending confusion, which
   moved generation3d's failure to
   `Generation3dMutationSession::grant()` refusing at
   `<artifact>🧬️mutations/💾️binary/🦀️.rs:2158` — `generation3d-envelope.mutation-ingress-malformed at
   offset 358 path $.edits` — and a second, analysed-not-fixed defect: `Generation3dMountedPackSession::grant`
   (`<artifact>🧬️schema/📸️snapshot/💾️binary/🦀️.rs:1128-1185`) admits a fresh catalog byte into an
   already-finished/undrained `RetainedValueCursor` for any non-empty canonical document
   (`🎒️pack/🌱️value/🦀️.rs:813-819`), and the `RetainedRecordBodyCursor` twin
   (`🧬️mutations/💾️binary/🦀️.rs:1793`) for mutations. Both are load-path defects that would affect
   restoring a real (non-genesis) example document, separate from the DSL-fixture path examples
   currently use.

**Bottom line as of the last entries in `📓️status.md` (2026-09-11 restage, 14:15):** the picker/graph
switch and the OOM are fixed on the guest side; contributions now push a scoped ~249 KB pack in one
crossing; the remaining blocker chain, in order, is (a) confirm the `invokeExtension`
rejected-authority fix and the `unknown kind` fix are both in the *currently served* wasm (restage
required, not confirmed landed in this ticket's files), then (b) `meshes > 0` for at least
`hexagonal-mushroom-column`, then (c) the 6 kernel-defect examples (§3/§4) need the `🧊️brep` subset
owner's extrude-orientation, fillet and boolean fixes before their volumes will be correct — the
DSL/example assets themselves are not at fault for any of the 8.
