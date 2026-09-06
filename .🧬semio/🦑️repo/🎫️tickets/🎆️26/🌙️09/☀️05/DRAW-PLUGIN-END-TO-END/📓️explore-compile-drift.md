# 🔬 Draw plugin — static compile-drift audit

Scope: crate `semio-s-plugin-draw` (`✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml`, entry
`📦️packages/🦀️rust/🦀️.rs`, 713 lines) mounting the plugin root `✏️s/🔌️plugins/🖍️draw/🦀️.rs` and the
artifact tree `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/**`. No cargo/nx/bun was run — every finding
below is a source-level comparison against the dependency crates' current definitions on disk.

**Live-repo caveat**: this crate is being edited concurrently by other sessions working the same
ticket. Two bugs found early in this audit (plugin-root `draw`/`drawing` module-name mismatch;
`io.rs`'s `DrawingCanvas`/`DrawingLayer`/`DrawingNode`/`DrawingStyle` stdio names; the svg
`subsets::any` path) were **fixed on disk between the start and end of this audit** — they are
recorded below as "found, then fixed during this audit" for the paper trail, not as open work.
Every other finding was re-verified against the file's live content immediately before this report
was written.

## 1 — `#[path]` / `include_str!` mount integrity

Walked the crate entry (`📦️packages/🦀️rust/🦀️.rs`, 713 lines) and every file it transitively
mounts via `#[path]`, resolving each literal against its containing file's directory (236
`#[path]` attributes + all `include_str!`/`include_bytes!` calls checked programmatically, then
spot-verified with `ls`).

### 1a. Real breaks — 12 `#[cfg(test)]` mounts in the crate's OWN entry file

The crate entry `📦️packages/🦀️rust/🦀️.rs` mounts one `#[cfg(test)] mod tests_*` per mutation kind,
pointing at a specific example-named test directory under each mutation's `🧪️tests/`. All 12 of
these directories were renamed with a hash suffix by a repo-wide rename pass, and the entry file's
`#[path]` literals were never updated — every one dangles:

| entry line | mounted (missing) path segment | current directory on disk |
|---|---|---|
| 307 | `➕️create-layer/🧪️tests/➕️appends-shape-b-at-the-root/🦀️.rs` | `➕️appends-shape-b-0b0435` |
| 319 | `🗑️delete-layer/🧪️tests/🚫️removes-group-a-with-its-child/🦀️.rs` | `🚫️removes-group-a-41e1e0` |
| 331 | `📋️duplicate-layer/🧪️tests/🚫️rejects-a-missing-source-layer/🦀️.rs` | `🚫️rejects-a-c88127` |
| 343 | `🔃reorder-layer/🧪️tests/⬆️moves-shape-a-above-shape-b/🦀️.rs` | `⬆️moves-shape-a-d7c515` |
| 364 | `🖊️replace-layer-stroke/🧪️tests/🖊️adds-a-dashed-stroke/🦀️.rs` | `🖊️adds-a-dashed-92bad7` |
| 376 | `🎨️replace-layer-fill/🧪️tests/🌈️solid-to-linear-gradient/🦀️.rs` | `🌈️solid-to-linear-9bdbe8` |
| 388 | `🌓️set-layer-blend-mode/🧪️tests/✖️normal-to-multiply/🦀️.rs` | `✖️normal-to-b12530` |
| 400 | `🌫️set-layer-opacity/🧪️tests/🌫️dims-shape-a-to-half/🦀️.rs` | `🌫️dims-shape-a-to-c25ad9` |
| 421 | `🔄️update-layer-transform/🧪️tests/📐️translates-and-scales-shape-a/🦀️.rs` | `📐️translates-and-f4316d` |
| 433 | `🔀set-layer-boolean-operation/🧪️tests/➖️union-to-subtract/🦀️.rs` | `➖️union-to-subtract-845e1f` |
| 445 | `🔍️update-layer-trace-params/🧪️tests/🔍️sharpens-the-trace/🦀️.rs` | `🔍️sharpens-the-117131` |
| 466 | `✏️rename-layer/🧪️tests/✏️renames-shape-a-without-touching-its-id/🦀️.rs` | `✏️renames-shape-a-ba5eed` |

These are real `#[cfg(test)]` mod declarations belonging to `semio-s-plugin-draw` itself (verified
by direct file read of `📦️packages/🦀️rust/🦀️.rs:296-467`; not a template/generated-test-host
artifact). `cargo check -p semio-s-plugin-draw --lib` (no `--tests`) will NOT trip these
(`#[cfg(test)]`-gated), but `cargo test -p semio-s-plugin-draw --lib` / `cargo check --tests` fails
with 12× "file not found for module" (E0583). Re-verified live: still missing.

### 1b. Generated-test-host quirk (not a real break for this crate)

72 more `include_str!` misses were found under `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/*/🧪️tests/<mutate-case>/🦀️.rs`
(e.g. `✳️any/🧪️tests/🎨️mutate-drawing-1-any-style/🦀️.rs:38-55` and the equivalent per-subset files),
all `include_str!`-ing the SAME pre-rename fixture-JSON paths broken by the 1a rename. These files
`use semio_repo_test_host::{...}` / `use semio_s_plugin_stdio_test_oracle::law;` and are **not
mounted anywhere in `semio-s-plugin-draw`'s own `#[path]` tree** (confirmed: no mount of any
`…/🧪️tests/mutate-drawing-1-*/🦀️.rs` file exists in either `📦️packages/🦀️rust/🦀️.rs` or
`🦀️.rs`). They belong to the repo-wide generated test-host harness the task brief calls out — real
fixture-path drift, but out of scope for `semio-s-plugin-draw`'s own `cargo check`/`cargo test`.
Flagged for the harness, not counted as a draw-crate compile blocker.

### 1c. Everything else resolves

All `#[path]`-mounted `🚪️io/📥️import/🧩️deserializers` and `📤️export/🧵️serializers` leaves (svg,
pdf, png, json, dwg, dxf — `✳️any/🚪️io/🦀️.rs:129-289`) exist on disk. No dangling mounts found
anywhere outside 1a/1b.

## 2 — `use semio_s_plugin_stdio::…` imports

Stdio entry: `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs` (14118 lines, `[lib] path` per its
`Cargo.toml:19`). The `drawing` subset lives at
`🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/`, mounted as
`semio::standards::v1::subsets::drawing` (stdio entry line 6820).

| draw file:line | imported as | stdio's actual name/path | status |
|---|---|---|---|
| `✳️any/🚪️io/🦀️.rs:17` | `DrawingCanvas`/`DrawingLayer`/`DrawingNode`/`DrawingStyle` (aliased) | stdio defines `DrawCanvas`/`DrawLayer`/`DrawNode`/`DrawStyle` (`…/drawing/🧬️schema/📸️snapshot/🦀️.rs:52,89,109,121`) | **found broken, fixed on disk during this audit** (now reads `DrawCanvas as SemioDrawCanvas`, etc. — correct) |
| `✳️any/🚪️io/🦀️.rs:19`, `:250` | `svg::standards::v1_1::subsets::any::schema::snapshot::…` | stdio's svg 1.1 subsets are `base`/`tiny`/`basic`, no `any` (stdio entry lines 3358-3499) | **found broken, fixed on disk during this audit** (now `subsets::base`) |
| `📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:10` | `json::standards::v_rfc8259::subsets::any::schema::snapshot::parse_json_text` | stdio's json rfc8259 has only `subsets::base` (stdio entry line 533-537) | **STILL BROKEN** — E0433 unresolved module `any` |
| `📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:10` | `json::standards::v_rfc8259::subsets::any::schema::snapshot::write_json_pretty` | same | **STILL BROKEN** — E0433 unresolved module `any` |
| `✳️any/🚪️io/🦀️.rs:15` | `semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform}` | all 5 exist verbatim (`…/base/🧬️schema/🧮️geometry/🦀️.rs:25,33,49,64,81`) | OK |
| `✳️any/🚪️io/🦀️.rs:20` | `svg::SvgSnapshot` | re-exported at svg module root (`🗿️artifacts/🎨️svg/🦀️.rs:7`) | OK |
| `✳️any/🚪️io/🦀️.rs:145`/`159` (register calls) | `semio::standards::v1::subsets::drawing::io::register`/`register_artifact_inferences` | both exist (`…/drawing/🚪️io/🦀️.rs:145,159`) | OK |

Fix direction for the 2 still-broken json imports: change `subsets::any` → `subsets::base` in both
files (mirrors the svg fix already applied elsewhere in the same `🚪️io/🦀️.rs`).

## 3 — framework / semio_s_2d / job / schema / dispatch-macros imports

Spot-checked every distinct top-level import target used across the tree
(`semio_framework_plugin`, `semio_framework`, `dsl`/`store`/`protocol` aliases of
`semio_framework_os_kernel`, `schema` alias of `semio_framework_schema`,
`semio_framework_dispatch_macros::{dyn_enum_close!, plugin_exports!}`,
`semio_framework_job::FixedOperationOwner`). No unresolved item names found in this pass beyond
the trait-signature drift catalogued in §4 (which is drift in draw's *implementations* of
framework traits, not in the `use` paths themselves). `semio_s_2d` and
`semio_framework_dispatch_macros::dyn_enum_close!` (plugin root, `🦀️.rs:10`) resolve as declared
in `Cargo.toml`.

## 4 — Trait-signature drift (oracle: block plugin post-W7 fix, `BLOCK-PLUGIN-END-TO-END/📓️w7a-block2d-compile.md`)

### (a) `async fn` on sync framework trait impls — CONFIRMED, still live

Framework trait declarations (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`):
- `ArtifactEditor::handle` — line 26818, **sync**.
- `ArtifactEditor::render` — line 26878, **sync** (`fn render(...) -> UiAssemblyResult<ComponentTree>;`).
- `ArtifactViewer::handle` — line 27083, **sync**.
- `ArtifactViewer::render` — line 27100, **sync**.

Draw's impls (all 4 declared `async fn`, verified live):
- `✳️any/✏️editor/🦀️.rs:1086` — `async fn handle(...)`.
- `✳️any/✏️editor/🦀️.rs:1102` — `async fn render(...)`.
- `✳️any/👁️viewer/🦀️.rs:63` — `async fn handle(...)`.
- `✳️any/👁️viewer/🦀️.rs:67` — `async fn render(...)`.

This is exactly block's "ROOT COMPILE BLOCKER" pattern (`BLOCK-PLUGIN-END-TO-END/📓️status.md:30`)
— will fail with E0407/E0053 (method has an incompatible signature: `async fn` vs `fn`). draw's
`fn diff`/`fn inverse` on `protocol::MutationKind` (14 files under `🧬️schema/🧬️mutations/*/🦠️mutation/🦀️.rs`)
and `OpText::parse_op`/`print_op` (`✏️editor/👥️presence/🦀️.rs:109,124`) ARE correctly sync — this
drift is confined to the two `render`/`handle` pairs above. Total `async fn` count in the draw tree:
288, but only these 4 are trait-impl violations; the rest are `#[semio_framework_async_macros::async_test]`
test functions or legitimately-async framework-required hooks (`command_from_intent`, `merge_ui_values`, etc. — draw does not override these).

### (b) `Config`/`Presence` missing `DESCRIPTORS`/`descriptor` — CONFIRMED, still live

`protocol::Mutation<P>` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:145-151`) requires, beyond `diff`/`inverse`:
```
const DESCRIPTORS: &'static [MutationLeafDescriptor];
fn descriptor(&self) -> &'static MutationLeafDescriptor;
```
Draw's two hand-written impls provide neither:
- `impl Mutation<DrawingConfig> for DrawingConfigMutation` — `✳️any/✏️editor/🎚️config/🦀️.rs:186-209` (only `type Diff`, `diff`, `inverse`).
- `impl Mutation<DrawingPresence> for DrawingPresenceMutation` — `✳️any/✏️editor/👥️presence/🦀️.rs:95-106` (same gap).

E0046 "not all trait items implemented" ×2. Fix precedent: block's W7a #3
(`🧩️puzzle/…/✏️editor/🎚️config/🦀️.rs` — one `MutationLeafDescriptor` per aggregate variant plus a
`descriptor` match) or puzzle's presence equivalent.

### (c) `render` returning bare `ComponentTree` — N/A (already `Result`-shaped)

All of draw's top-level trait `render` impls already return
`UiAssemblyResult<ComponentTree>`/`UiAssemblyResult<BuiltNode>` (never a bare `ComponentTree`) —
the drift here is exclusively the `async` mismatch in (a), not the return type.

### (d) `with_ports` on `AppIo` — not used

`with_ports` does not appear anywhere in the draw tree (`grep` empty) — draw does not build an
`AppIo` via that builder path at all, so this specific block-precedent bug class does not apply.

### (e) `ActionDescriptor` helper signatures — clean

`✳️any/✏️editor/🦀️.rs:48-56` builds `ActionDescriptor` via `semio_framework_plugin::ActionFactory::new(...).action(...)`
and a literal `ActionDescriptor { controller_id, action, args: None }` — same shape
`ActionFactory::action` expects (matches block's already-fixed pattern). No override of
`command_from_action` exists in draw (falls through to the framework default), so item (g)
(DslValue vs JsonValue) does not apply — draw never hand-writes that signature.

### (f) prelude `Label: From<…>` — latent, not yet triggered

Framework crate root re-exports TWO different `Label` types at two different paths:
`semio_framework_plugin::plugin_app_close_prelude::Label` = `semio_framework_ui_contract::Label`
(glob re-export, `🔌️plugin/🦀️.rs:38681`) vs bare `semio_framework_plugin::Label` =
`ui_wgpu::wgpu::Label` (glob re-export, `🔌️plugin/🦀️.rs:38898`) — confirmed no `From` between them
exists in this file. Draw's `✳️any/✏️editor/🦀️.rs:25` imports the bare (wgpu) `Label` directly from
`semio_framework_plugin::{…, Label, …}`, and its one use site
(`✏️editor/🦀️.rs:942: built_text_node(Label::data(...))`) matches `built_text_node`'s parameter
type (`ui_wgpu::wgpu::Label`) correctly — so this import is NOT currently a bug, but it is the
same trap block hit (block needed the *contract* Label for `tree_item*` builders and had to add a
local `ui_label` shim). If a future draw change starts calling `tree_item`/`tree_item_desc`/etc.
directly with this imported `Label`, it will hit the identical `Label: From<Label>` E0277. Noted
as latent risk, not a live break.

### (g) `command_from_action` DslValue vs JsonValue — N/A

No override in draw (see (e)) — the framework default (`Option<&DslValue>`,
`🔌️plugin/🦀️.rs:26827`) is used as-is.

### (h) `↩️inverse` referencing a `mutation` module — clean

All 14 `↩️inverse/🦀️.rs` files correctly call `super::mutation::<Type>` (e.g.
`🧱️structure/…/➕️create-layer/↩️inverse/🦀️.rs:8: pub fn inverse(payload: &super::mutation::CreateLayer, …)`),
and `mutation` is genuinely declared as a sibling module in the crate entry
(`📦️packages/🦀️rust/🦀️.rs:301,305` etc. mount `diff`/`inverse`/`mutation` as siblings under each
`pub mod <kind>` block). This resolves correctly — draw does NOT have block's E0433
"`mutation` missing in `inverse`" bug.

### (i) hand-written `ToValue`/`FromValue` / lingering `serde` — clean

No hand-written `impl ToValue for`/`impl FromValue for` anywhere in the draw tree (all types use
`#[derive(dsl::ToValue, dsl::FromValue, dsl::DslRecord/DslArtifact/DslOps)]`) — no
`store::to_dsl_value` self-recursion trap possible. No non-test `serde::Serialize`/`Deserialize`
usage found; every `derive(serde::…)` is behind `#[cfg_attr(test, …)]`.

## 5 — Plugin identity triple

- `Plugin::<DrawApps>::builder("draw")` — `🦀️.rs:37`.
- `.package_id("semio:draw")` — `🦀️.rs:40`; matches `Cargo.toml`'s
  `[package.metadata.component] package = "semio:draw"` (`📦️packages/🦀️rust/Cargo.toml:12`). OK.
- `.declare_artifact(crate::artifacts::drawing::artifact())` → `artifact()`
  (`🗿️artifacts/🖍️drawing/🦀️.rs:531`) parses `ArtifactKindId::parse("s.draw.drawing")` — correct
  `s.<plugin>.<kind>` grammar, owned by plugin id `draw`. OK.
- `crate::artifacts::drawing::artifact_kind().id` (`🗿️artifacts/🖍️drawing/🦀️.rs:453`) = the
  **legacy flat id `"2d.drawing"`** — used only for `.activation(ActivationEvent::OnArtifactKind{kind})`
  (`🦀️.rs:44`) and for `export_stdio_kinds`/`import_stdio_kinds` bookkeeping inside the same
  struct. This does NOT follow the `s.draw.<kind>` grammar and differs from the kind actually
  declared via `.declare_artifact()`. A sibling explorer on this same ticket
  (`DRAW-PLUGIN-END-TO-END/📓️status.md:22`) already checked this against the repo convention
  (lowpoly uses the same flat `"3d.lowpoly"` for its own `artifact_kind()`/activation pairing) and
  concluded **this is the established, intentional pattern, not a defect** — `artifact_kind()` is
  explicitly kept as legacy-but-load-bearing debt per the docstring at `🦀️.rs:26-29`
  ("`artifact_kind()` is kept because `🦀️.rs`'s own `.activation(...)` still reads
  `artifact_kind().id`"). Traced `plugin-assembly.surface-dependency-gate`
  (`🧰️framework/…/🔌️plugin/🏗️builder/🦀️.rs:732`, backed by `crate::app::surface_dependency_breaches`)
  — it validates cross-plugin mutation-roster/contribution ownership against `plugin.manifest`, not
  activation-event kind strings, so this flat/`s.`-namespaced split does not trip that gate. Not
  counted as a compile-drift finding; recorded for completeness only.
- Registry rows (`🧰️framework/…/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json:246-266`,
  `🎠️playgrounds.json:307-308`) list `pluginId: "draw"`, `packageId: "semio:draw"`,
  `activationEvents: ["on-artifact-kind:2d.drawing"]` — consistent with the source (both reflect
  the same flat activation id), so the generated registry is not itself stale relative to source on
  this axis; whether its cached `wasmSha256`/`descriptorSha256` predate the current source is a
  build-freshness question outside this static audit's scope.

## 6 — `cfg(target_arch = "wasm32")` blocks

None found anywhere in the draw tree (`grep -rn 'cfg(target_arch = "wasm32")'` — zero matches).
Nothing here would diverge between native and wasm32 compilation.

## Summary table

| predicted error class | count | example file:line | fix direction |
|---|---|---|---|
| E0583 file-not-found module (`#[cfg(test)]` mounts, §1a) | 12 | `📦️packages/🦀️rust/🦀️.rs:307` → `➕️appends-shape-b-at-the-root/🦀️.rs` | repoint each `#[path]` literal at the current hash-suffixed directory name |
| E0433 unresolved `subsets::any` for stdio json (§2) | 2 | `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:10` | `subsets::any` → `subsets::base` |
| E0407/E0053 `async fn` vs sync trait method (§4a) | 4 | `✳️any/✏️editor/🦀️.rs:1102` (`async fn render`) | drop `async` from `handle`/`render` in editor + viewer `🦀️.rs` |
| E0046 missing `DESCRIPTORS`/`descriptor` (§4b) | 2 | `✳️any/✏️editor/🎚️config/🦀️.rs:186` | add `const DESCRIPTORS` + `fn descriptor` per puzzle's `🎚️config`/`👥️presence` precedent |
| (already fixed during this audit) stdio snapshot struct renames | 4 | was `✳️any/🚪️io/🦀️.rs:17` | none — landed on disk before this report |
| (already fixed during this audit) svg `subsets::any`→`base` | 2 | was `✳️any/🚪️io/🦀️.rs:19,250` | none — landed on disk before this report |
| generated-test-host `include_str!` fixture drift (§1b, out of crate scope) | 72 | `✳️any/🧪️tests/🎨️mutate-drawing-1-any-style/🦀️.rs:38` | out of scope for `semio-s-plugin-draw`; belongs to the repo test-host harness |
| latent `Label` ambiguity (§4f) | 0 (not yet triggered) | `✳️any/✏️editor/🦀️.rs:25` | none needed now; if future code calls `tree_item*` with this `Label`, add a local `ui_label` shim like block's |

## Recommended fix order

1. Fix the 2 remaining stdio json `subsets::any` → `subsets::base` imports (§2) — trivial, one-line each, unblocks the whole crate's `cargo check`.
2. Drop `async` from the 4 `ArtifactEditor`/`ArtifactViewer` `handle`/`render` impls (§4a) — the single highest-count real compile blocker class, same as block's root cause.
3. Add `DESCRIPTORS`/`descriptor` to `DrawingConfigMutation`/`DrawingPresenceMutation` (§4b), copying puzzle's `🎚️config`/`👥️presence` shape.
4. Re-point the 12 `#[cfg(test)]` `#[path]` mounts in `📦️packages/🦀️rust/🦀️.rs` at their hash-suffixed directories (§1a) — needed for `cargo test`, not `cargo check`.
5. Only after 1-4 are green: re-run a real `cargo check -p semio-s-plugin-draw --lib` to catch anything this static pass could not see (macro-expansion-only errors, generic bound mismatches, etc.) — this audit is source-comparison only, not a compiler run.
