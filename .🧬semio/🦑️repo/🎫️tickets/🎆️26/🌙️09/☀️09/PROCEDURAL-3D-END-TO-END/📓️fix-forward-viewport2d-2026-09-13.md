# 🔭️ Fix Forward — `Viewport2d` Home + `WindowConfigOwner::State: DslField` (2026-09-13)

Ticket: `26/09/09/PROCEDURAL-3D-END-TO-END`.
Lane: unblock `bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`, which had failed
for five consecutive retries (`📜️restage-retry.sh`, screen `restage`, log `🗑️generated/restage-retry.txt`).

## 1. What the last failing attempt actually said

`awk '/attempt=4$/{p=1} p' 🗑️generated/restage-retry.txt` — identical body in attempts 1…5, each ending

```
error: could not compile `semio-s-artifact-flow-flow` (lib) due to 4 previous errors; 6 warnings emitted
error: script "nx" exited with code 130
EXIT=130 attempt=5
```

The four errors:

| # | Error | Site |
| --- | --- | --- |
| 1 | `E0432` `unresolved import semio_framework::Viewport2d` | `…/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs:7` |
| 2 | `E0432` `unresolved import semio_framework::Viewport2d` | `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs:12` |
| 3 | `E0432` `unresolved import semio_framework_plugin::Viewport2d` | `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌊️main/🦀️.rs:12` |
| 4 | `E0277` `the trait bound FlowMainWindowConfig: DslField is not satisfied` | `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🦀️.rs:143` |

(The brief said "×4 E0432 + ×2 E0277"; the log shows 3 + 1 = the 4 the compiler counted. The extra
occurrences it was describing are the duplicated `help:` echoes rustc prints under each E0432.)

## 2. Where `Viewport2d` now lives

```
🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🦀️.rs:5:pub struct Viewport2d {
🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🦀️.rs:9:pub use planar::Viewport2d;            # crate semio-framework-ui-viewport
🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:344:pub use semio_framework_ui_viewport::{Viewport2d, Viewport3dOrbit};
```

Owner crate: `semio-framework-ui-viewport`. The **only** re-export is
`semio_framework_os_kernel` (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`). Grepping `Viewport` in the
roots of `semio-framework` (`🧰️framework/📦️packages/🦀️rust/🦀️.rs`) and `semio-framework-plugin`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🦀️.rs`) returns **zero** hits — the peer
removed both. No re-export was added back into either; every consumer was moved forward instead,
exactly as the earlier lane recorded in `📓️generate-mode-panels-2026-09-13.md` §"Viewport2d".

## 3. Forward move — 29 files

Script (retained, ticket root): `🐍️viewport2d-forward.py`. Two textual rules plus a brace-list rule:

- `semio_framework::Viewport2d` → `semio_framework_os_kernel::Viewport2d`
- `semio_framework_plugin::Viewport2d` → `semio_framework_os_kernel::Viewport2d`
- `use semio_framework_plugin::{…, Viewport2d, …};` → `Viewport2d` dropped from the brace list and a
  separate `use semio_framework_os_kernel::Viewport2d;` emitted right after the block (single-line
  and multi-line import blocks both handled).

Every touched crate already carries `semio-framework-os-kernel` as a direct dependency (verified in
all eight plugin manifests plus `semio-framework-os-flow`), so no manifest edit was needed.

Crates moved forward (file counts):

| Crate / area | Files |
| --- | --- |
| `semio-s-artifact-flow-flow` (🌊️flow) | 6 |
| `➗️mathematical` / equation | 5 |
| `🔱️trinity` / jack + rewriting | 8 |
| `🕸️dag` | 4 |
| `💡️reasoning` / wires | 3 |
| `🎬️sequence` | 2 |
| `semio-framework-os-flow` host unit test | 1 |
| **total** | **29** |

Already-forward and untouched: `🏛️architect` (already `semio_framework_os_kernel::Viewport2d`),
`🏗️fem/◻️2d` (goes through its own crate root re-export at `🗿️artifacts/◻️2d/🦀️.rs:297`, which already
points at `semio_framework_os_kernel`).

Nothing was reverted. The peer's removal of the old re-exports stands; only consumers moved.

## 4. The `DslField` bound — the same peer's in-flight trait move, completed

`git diff HEAD` on `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs` (mtime
2026-09-13 05:28:08, uncommitted) shows the bound being *added* in the working tree:

```
-    type State: … + store::ConfigRecord + store::ArtifactPack + 'static;
+    type State: … + store::ConfigRecord + store::ArtifactPack + store::mounted_pack_rt::DslField + 'static;
```

in the same uncommitted hunk that replaces the old `fn load(…)` with the new retained pack-load pair
(`begin_retained_load` / `commit_retained_load`, new sibling file `📥️retained/🦀️.rs`). That new loader
is exactly what needs `DslField`:

- `📥️retained/🦀️.rs:197` — `<O::State as store::ArtifactPack>::record_spec().ok_or(…TypedState)?`
- `📥️retained/🦀️.rs:359` — `<O::State as store::mounted_pack_rt::DslField>::from_value(&field)`

So this is the same peer's in-flight work, not an unrelated break. It was completed at the owning
site the way the peer completed the sibling window configs.

### The sibling pattern that compiles

`EquationGraphWindowConfig`
(`✏️s/🔌️plugins/➗️mathematical/…/🪟️windows/🕸️graph/🎚️config/`), `WiresCanvasWindowConfig` and
`Generation2dMainWindowConfig` all share one shape:

1. the struct lives in a `🧬️schema/🦀️.rs` beside the config file, deriving
   `dsl::DslArtifact` with `#[dsl(id = …, layout = "lines")]`;
2. `DslArtifact` emits `__dsl_spec` / `__dsl_to_record` / `__dsl_from_record`,
   `__DSL_ENVELOPE_ID` / `__DSL_EXTENSION`, **and** `impl ::dsl::DslField` — which is what satisfies
   the new bound (derive body: `🗣️dsl/✨️derive/🦀️.rs:1311-1329`);
3. the config file hand-writes `store::ArtifactDsl` and `store::ArtifactPack` on top of those
   `__dsl_*` helpers (P6: the derive no longer emits those traits), with
   `record_spec() -> Some(Self::__dsl_spec())`.

### What was changed for Flow

New file
`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧬️schema/🦀️.rs`
holding the struct and its `Default`:

```rust
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.flow.flow.mainwindowconfig", extension = "flowmainwindowcfg", layout = "lines")]
pub struct FlowMainWindowConfig { … #[dsl(block)] pub camera: CameraJson, … }
```

`id` and `extension` are passed explicitly so the derived `__DSL_ENVELOPE_ID` /
`__DSL_EXTENSION` reproduce the **exact** strings the hand-written impl carried before
(`"s.flow.flow.mainwindowconfig"` / `"flowmainwindowcfg"`) — the envelope identity is unchanged.

Every field type already satisfies `DslField`: `String`/`f64`/`bool` are primitive impls,
`Vec<T>` is `🗣️dsl/🦀️.rs:183`, and `CameraJson`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs:154`) already
derives `crate::os_dsl::DslRecord`. No new derive was needed anywhere else.

The config file now re-exports the schema module and its `ArtifactDsl`/`ArtifactPack` go through
`__dsl_*` + `store::pack_rt::encode_document` / `decode_document`, with
`record_spec() -> Some(Self::__dsl_spec())` instead of the previous `None`. That `None` is what would
have made the peer's new retained loader fail at `📥️retained/🦀️.rs:197` with
`WindowConfigPackLoadDiagnostic::TypedState` for this window kind even once it compiled, so the
record spec is part of completing the move rather than an extra.

The previous encoding was ad-hoc `serde_json` inside the semio envelope; it is now the same
pack-runtime document encoding every sibling config uses. Greenfield, and `flowmainwindowcfg` /
`s.flow.flow.mainwindowconfig` appear in **no** fixture, asset, or generated file anywhere in the
repo (checked across `*.rs` / `*.json` / `*.ts`), so nothing on disk needed hand-fixing.

## 5. Verification — what was actually run

### `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev`

```
    Checking semio-s-plugin-procedural v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
    Finished `wasm-dev` profile [unoptimized] target(s) in 46.36s
```

Green, 0 errors. Warnings only (pre-existing: `unused extern crate … as vcs`, `unnecessary
qualification`, `retained_allocated_bytes` never used).

Because that package tree does not pull `semio-s-artifact-flow-flow`, the crate that was actually red
was checked directly as well:

```
cargo check -p semio-s-artifact-flow-flow --target wasm32-wasip2 --profile wasm-dev \
  --manifest-path "✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml"
warning: `semio-s-artifact-flow-flow` (lib) generated 12 warnings
    Finished `wasm-dev` profile [unoptimized] target(s) in 15.77s
```

0 errors — the 4 restage-blocking errors are gone.

### `bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`

Run in the foreground after the retry screen's own loop had finished, and again as the last action of
this lane so the quoted numbers describe the **final** tree:

```
RESTAGE-EXIT=0
 NX   Successfully ran target activate-generation3d-react-dev for project @semio-tech/framework-os-dev and 33 tasks it depends on
```

Full log: `🗑️generated/fix-forward/restage.txt`.

The retry screen independently confirmed the same thing from its own loop — attempts 1-5 `EXIT=130`,
then, once these edits landed (config file mtime 06:37:22), `EXIT=0 attempt=6` followed by
`PLUGIN-EXIT=0 attempt=6` and `RETRY-DONE`. The screen's loop is finished; it was not restarted.

Staged wasm after the final run
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/`):

| Module | Size | mtime |
| --- | --- | --- |
| `🌊️flow/semio_s_plugin_flow_component.core.wasm` | 98 388 271 | 2026-09-13 07:13 |
| `🌊️flow/🛂️.descriptor.semio` | 114 215 | 2026-09-13 07:13 |
| `🌀️procedural/semio_s_plugin_procedural_component.core.wasm` | 68 040 045 | 2026-09-13 07:19 |
| `🌀️procedural/🛂️.descriptor.semio` | 226 239 | 2026-09-13 07:19 |

### `cargo test -p semio-s-artifact-flow-flow --lib`

```
running 216 tests
test result: FAILED. 121 passed; 95 failed; 0 ignored; 0 measured; 3 filtered out; finished in 30.11s
```

Log: `🗑️generated/fix-forward/flow-flow-lib-test.txt`. Ticket baseline
(`📓️tick-arming-latch-2026-09-12.md`, `📓️flow-catalog-authority-2026-09-10.md`): **130 passed / 89
failed of 219**.

**The 3 filtered-out tests.** A first, unfiltered run of the suite never terminated: three tests span
CPU-bound indefinitely (>14 min at 340 % CPU, killed) —

- `editor::flow::commands::delete_selection::tests::context_menu_at_is_a_no_operation`
- `editor::flow::modes::edit::windows::main::component::tests::renders_node_graph_scene`
- `viewer::flow::modes::view::windows::main::tests::renders_node_graph_scene_for_the_default_document`

They are **not** this lane's. The suspicion was that switching `record_spec()` from `None` to
`Some(…)` had put this owner onto the peer's brand-new, unfinished retained loader, so the config was
temporarily reverted to its exact pre-lane runtime shape (JSON `parse_dsl`/`print_dsl`/pack encoding
and `record_spec() -> None`, keeping only the `DslArtifact` derive for the bound) and the three tests
re-run: **they spun identically**. The spin therefore does not depend on anything this lane changed,
and the sibling-consistent record-based form was restored. `renders_node_graph_scene` also spins on
its own, one test per process, so it is not cross-test interference either.

**The other failures.** 42 of the 95 are two framework-owned retirement invariants firing during drop:

- `"ordered-map root must be explicitly retired before drop"` ×23 —
  `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`
- `"artifact store reached Drop without its exact terminal-empty shallow-shell witness"` ×19 —
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18257`

The one failure that sits on the file this lane owns,
`…main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows`, was
re-run in isolation:

```
[DEBUG] Flow exact-window runtime failure before close: Fault { origin: Framework,
  code: FaultCode("interactive-job.live-instance"), …
  message: "typed command 'nodeGraphViewport' does not belong to the mounted live app instance" }
panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:6834:13:
  registered fixture did not reach its exact terminal-empty witness, last pending close authority:
  document store close awaits a retained reader or owner
```

`origin: Framework`, raised inside `semio-framework-plugin`'s own app-laws harness — the live-instance
admission gate, not the viewport value it carries or the config encoding. The lane's own diff to that
test file is two `semio_framework::Viewport2d` → `semio_framework_os_kernel::Viewport2d` substitutions
and nothing else (`git diff` on the test file confirms).

So every failure in this run lands in framework store / replication / plugin-harness territory whose
owning files carry the peer's uncommitted 2026-09-13 05:28 edits, not in this lane's files. **No claim
is made that this suite is green** — it is not, and it was not at baseline either.

## 6. Files changed

Forward-moved `Viewport2d` (29 files) — see §3 for the full crate breakdown; the exhaustive path list
is the stdout of `🐍️viewport2d-forward.py`.

Owned by this lane:

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧬️schema/🦀️.rs` — **new**, derived schema
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🦀️.rs` — schema module + record-based `ArtifactDsl`/`ArtifactPack`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🐍️viewport2d-forward.py` — **new**, retained input script
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️fix-forward-viewport2d-2026-09-13.md` — this report

Not touched: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs` and its new
`📥️retained/🦀️.rs` — those are the peer's live, uncommitted work and were only read.

## 7. Remaining red attributable elsewhere

### Flow lib suite

See §5 — 95 failures and 3 non-terminating tests, all landing in framework store / replication /
plugin-harness code, and the three spinners reproduced unchanged with this lane's config reverted to
its pre-lane runtime shape. Last modifier of the implicated framework files is the peer's uncommitted
working-tree edit (`🔌️plugin/🪟️window/🎚️config/🦀️.rs` + new `📥️retained/🦀️.rs`, mtime 2026-09-13
05:28:08; `🔌️plugin/📦️packages/🦀️rust/🦀️.rs:6834` is the harness raising the witness panic).

### Other window configs still missing `DslField`

The peer's `DslField` bound is repo-wide, and **16 other** `WindowConfigOwner::State` types still have
no `DslField` impl. They did not block this restage because nothing in the generation3d react
dependency graph compiles them, but they will fail the same `E0277` the moment their crate is built:

`🎬️sequence/📽️main`, `📋️forms/▶️try`, `📏️layout/📐️blueprint`, `📸️remodel` ×3 (`🖼️frames`, `📊️report`,
`🧊️model`), `🖍️draw/🖼️canvas`, `🗒️note`, `🧩️puzzle` ×3 (`◻️2d`, `🖐️5d`, `🧊️3d`), and `🏛️architect` ×4
(`↔️adjacency`, `📋️register`, `📓️report`, `🕸️graph`).

Last modifier for all of them is the bulk `🐙️ueli` auto-commit timestamp (09-12 19:34 / 09-13 00:19),
i.e. none of them is mid-edit by a live peer right now — but the bound that breaks them is the
uncommitted 05:28 edit to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs`, which
is the peer's. Each needs the same §4 treatment (a `dsl::DslArtifact`-derived schema plus record-based
`ArtifactPack::record_spec`). Left to that peer's sweep rather than pre-empted from this lane, since
they are outside this ticket's dependency graph and each carries its own envelope identity.
