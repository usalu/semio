# Lane 5-D report — `dev s` plugin catalogue: sweep, fixes, resilient orchestrator

## Bar

Fixed every non-stdio-rooted failure in the 33-crate catalogue (10 of 13), made `buildPlugins` (the
strict `plugin s` orchestrator path) log-and-continue with an end-of-run summary instead of aborting on
the first broken crate, and ran the real orchestrator end to end. The orchestrator run did not reach a
final `EXIT:` line before this session's background-task budget was exhausted mid-way through a `stdio`
recompile — that is reported honestly below, with the partial evidence that exists, rather than claimed
as a clean pass.

## 1. Fresh 33-crate sweep — before

`cargo check -p <crate> --target wasm32-wasip2` run individually (never `--workspace`) on all 33
catalogue crates. Full log: `🧪️5-d-sweep1.txt`. Result: **20 pass / 13 fail** — the exact same 13 crate
names lane 4-E reported (`📓️.../PRESERVE-SEEDED-DIALOG-CONTEXT-ARGUMENTS/w4-e-report.md`), confirming
nothing had moved since that sweep except the tree churn everyone already knew about.

| Crate | Before | After my fixes | Root cause | Fixed here? |
|---|---|---|---|---|
| `semio-s-plugin-writer` | PASS | PASS | — | — |
| `semio-s-plugin-mathematical` | **FAIL** | **PASS** | `mathematical_geometry` used but not imported in a diff module | yes |
| `semio-s-plugin-procedural` | **FAIL** | **PASS** | `Procedural{2,3}dStore::new` now returns `Result`; two wasm-bridge files never added the `.map_err(...)?` sibling crates already use | yes |
| `semio-s-plugin-flow` | PASS | PASS | — | — |
| `semio-s-plugin-gis` | **FAIL** | **PASS** | `declaration()` now returns `Result`; plugin root never added `.map_err(PluginAssemblyError::definition)?` (the pattern every other plugin already uses) | yes |
| `semio-s-plugin-vcs` | PASS | PASS | — | — |
| `semio-s-plugin-animate` | **FAIL** | **FAIL (unchanged, by design)** | Own bugs (`InteractionView` import, `SvgSnapshot.lexical`, `Mp4Track` field-shape, type inference) — explicitly listed "known-foreign, do not fix" in the worker brief | **no — out of scope** |
| `semio-s-plugin-shooting` | PASS | PASS | — | — |
| `semio-s-plugin-demonstrator` | **FAIL** | **PASS** | Zero crate-local errors — it only failed transitively through its `gis`/`procedural` path dependencies; fixing those two fixed this one for free | yes (transitively) |
| `semio-s-plugin-sequence` | **FAIL** | **PASS** | `dag_lod_scale_json` called unqualified at crate root of `infinite_board_port_directed_dag`, but it only lives at `board::ports::directed_dag::` inside that crate — fixed the call site's path, no framework touch needed | yes |
| `semio-s-plugin-fem` | **FAIL** | **PASS** | Both `fem2d`/`fem3d` wasm bridges imported `Fem2dEnvelope`/`Fem2dStore` from a stale `::op::` path; they live at `::mutations::` now | yes |
| `semio-s-plugin-architect` | PASS | PASS | — | — |
| `semio-s-plugin-process` | PASS | PASS | — | — |
| `semio-s-plugin-lowpoly` | **FAIL** | **PASS** | `UiNode` used in a viewer window's `render` signature without being imported | yes |
| `semio-s-plugin-reasoning-mindmap` | PASS | PASS | — | — |
| `semio-s-plugin-forms` | PASS | PASS | — | — |
| `semio-s-plugin-layout` | **FAIL** | **FAIL (unchanged, by design)** | Consumes stdio's `DwgSnapshot`/`DwgDecodeStatus`, which no longer carry the fields this file expects (`bytes`, `decode_status`, `section_names`, `sections`) — `🗄️stdio/**` is forbidden territory (`FULL-STDIO` ticket, still open) | **no — stdio-rooted, out of scope** |
| `semio-s-plugin-cad` | PASS | PASS | — | — |
| `semio-s-plugin-norm` | PASS | PASS | — | — |
| `semio-s-plugin-playbook` | PASS | PASS | — | — |
| `semio-s-plugin-imperative` | PASS | PASS | — | — |
| `semio-s-plugin-remodel` | **FAIL** | **PASS** | Two `.collect()` calls needed an explicit `Vec<_>` annotation (ambiguous inference once combined with a struct literal a few lines later) | yes |
| `semio-s-plugin-energy` | PASS | PASS | — | — |
| `semio-s-plugin-trinity` | PASS | PASS | — | — |
| `semio-s-plugin-dag` | PASS | PASS | — | — |
| `semio-s-plugin-draw` | **FAIL** | **PASS** | Viewer canvas window imported `ui_wgpu::wgpu::SurfaceKind`, but `draw`'s own `Cargo.toml` never depends on `ui_wgpu` (unlike `cad`/`flow`/`dag`) — every sibling file in the same crate already imports `SurfaceKind` from `semio_framework_plugin`; matched that | yes |
| `semio-s-plugin-raster` | **FAIL** | **PASS** | 3 independent bugs in one crate (see below) | yes |
| `semio-s-plugin-stdio` | PASS | PASS | — | — |
| `semio-s-plugin-note` | **FAIL** | **FAIL (unchanged, by design)** | Consumes stdio's `SvgSnapshot`, which no longer has a `lexical` field — same class of break as `layout`, same forbidden territory | **no — stdio-rooted, out of scope** |
| `semio-s-plugin-puzzle` | PASS | PASS | — | — |
| `semio-s-plugin-block` | PASS | PASS | — | — |
| `semio-s-plugin-space` | PASS | PASS | — | — |
| `semio-s-plugin-sourcing` | PASS | PASS | — | — |

**10 of 13 failures fixed. 3 left exactly as found, correctly attributed as out-of-lease.**

### `raster`'s 3 bugs (all in this lane's lease, all fixed)

1. `👁️viewer/…/🪟️windows/🧭️navigator/🦀️component.rs` called `super::composite::composited_image_view(...)`,
   but the actual mount point (per `📦️glue.rs`) makes `composite` a *nested* module two levels deep
   (`windows::composite::component`, bridged by `pub use component::*;`), not a direct sibling of
   `navigator` — `super::composite` never resolved. Fixed the import to the crate-rooted path the
   compiler itself suggested (`crate::viewer::raster::modes::view::windows::composite`), which then
   surfaced a second, real bug: `composited_image_view` was `pub(super)`, which (given where the file is
   actually mounted) only exposes it inside `composite` itself, not to sibling `navigator` — widened to
   plain `pub`, the same visibility every other cross-window-shared render helper in this codebase uses.
2. `👁️viewer/…/🪟️windows/🖼️composite/🦀️component.rs` called `base64::engine::general_purpose::STANDARD
   .decode(...)`/`.encode(...)` without `base64::Engine` in scope (the trait that defines those methods)
   — added the import.
3. The DWG import deserializer (`🚪️io/📥️import/…/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs`) had a
   `deserialize(from: &DwgSnapshot) -> ... { deserialize_bytes(&from.bytes) }` wrapper that could never
   work again once stdio's `DwgSnapshot` was decomposed into structured fields (no raw `.bytes` left to
   read) — but it also had **no caller anywhere in the repo** (verified: the real import call site,
   `🚪️io/🦀️component.rs:377`, already calls `deserialize_bytes` directly with real bytes, exactly like
   every sibling format deserializer — `png`, `jpg`, etc. — none of which have a
   `deserialize(from: &Snapshot)` wrapper at all). Removed the dead wrapper; `deserialize_bytes` (the one
   real callers use) is untouched. This is a within-crate cleanup, not a stdio edit — `DwgSnapshot`'s
   shape itself was only read, never modified.

## 2. Re-verification of the 10 fixed crates

Individually re-ran `cargo check -p <crate> --target wasm32-wasip2` on each of the 10 after the edits
(never as a batch/workspace check). Log: `🧪️5-d-refixed-sweep.txt` (+ `🧪️5-d-raster-final-check.txt` for
raster's final pass after the visibility fix, since it needed a second round). All 10: `EXIT:0`.

```
mathematical  EXIT:0
procedural    EXIT:0
gis           EXIT:0
demonstrator  EXIT:0
sequence      EXIT:0
fem           EXIT:0
lowpoly       EXIT:0
remodel       EXIT:0
draw          EXIT:0
raster        EXIT:0   (after the pub(super)->pub follow-up fix; see 🧪️5-d-raster-final-check.txt)
```

## 3. Orchestrator resilience (`buildPlugins`)

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`, lines
862–888 only (re-read immediately before editing both times; confirmed no lane 5-A collision either
time). `buildPlugins` previously called `await buildPlugin(target)` for each catalogue target with no
try/catch — one compiler error anywhere in the catalogue threw and killed the whole `plugin s` run before
later crates were even attempted. It now mirrors the existing best-effort `buildPluginsStreaming` (the
React dev-runner's streaming variant, untouched): every target is attempted regardless of earlier
failures, and a summary is printed at the end:

```diff
 export async function buildPlugins(filterPlugin?: string): Promise<void> {
   ensureAppleDeveloperDir();
   const targets = await preparePluginBuildTargets(filterPlugin);
+  const failed: string[] = [];
   for (const target of targets) {
-    await buildPlugin(target);
+    try {
+      await buildPlugin(target);
+    } catch (error) {
+      failed.push(target.pluginId);
+      console.error(`[DEBUG] plugin build failed, continuing with remaining targets: ${target.pluginId}`, error);
+    }
   }
+  const builtCount = targets.length - failed.length;
+  console.log(`[DEBUG] plugin catalog build summary: ${builtCount}/${targets.length} crate(s) produced .wasm`);
+  if (failed.length > 0) {
+    console.log(`[DEBUG] plugin catalog build failures (${failed.length}): ${failed.join(", ")}`);
+  }
 }
```

**Proven working in the real run** (not just read), `🧪️5-d-final-plugin-s-run.txt`:

```
[DEBUG] program build scope: all (59 plugin crates)
[DEBUG] plugin build failed, continuing with remaining targets: animate
[DEBUG] built program architect (wasm32-wasip2, dev) -> …/🔌️plugin-modules/architect
[DEBUG] built program block (wasm32-wasip2, dev) -> …/🔌️plugin-modules/block
[DEBUG] built program cad (wasm32-wasip2, dev) -> …/🔌️plugin-modules/cad
[DEBUG] built program cad-extension-aec-building (wasm32-wasip2, dev) -> …
[DEBUG] built program cad-extension-aec-building-energy (wasm32-wasip2, dev) -> …
[DEBUG] built program cad-extension-aec-building-structure (wasm32-wasip2, dev) -> …
[DEBUG] built program cad-extension-spatial-shape (wasm32-wasip2, dev) -> …
```

`animate` (registry-ordered before `architect`) failed and the run visibly kept going through
`architect`, `block`, `cad`, and all four `cad` extensions afterward — exactly the behavior the fix was
meant to produce. Note the real registry is **59 entries**, not 33: the 33-crate figure in the brief
covers top-level catalogue crates only, but the live registry also carries per-crate *extension*
sub-crates (`cad-extension-*`, `flow-extension-*`, `imperative-extension-*`, `process-extension-*`,
`sourcing-module-*`, `playbook-module-procedural`) that `preparePluginBuildTargets("s")` expands to.

## 4. Final `plugin s` orchestrator run — honest status

`bun ./📜️script.ts plugin s` was launched for real (`🧪️5-d-final-plugin-s-run.txt`, `cargo build`, not
`check`, for every one of the 59 registry entries in sequence — a full, non-incremental-from-cold run of
this size is genuinely long). It **did not reach a terminal `EXIT:` line**: this session's own background
task budget was exhausted (repeated 590s blocking waits) while the run was mid-way through recompiling
`semio-s-plugin-stdio` for a downstream target, and the underlying `bun` process was not alive by the time
I could re-check it. This is an environment/session-lifetime limitation, not a build failure — nothing in
the log up to that point shows any error besides the expected, already-attributed `animate` failure.

**What is verifiably true from the partial run + disk state, stated precisely (no rounding up):**
- The resilience fix is proven correct in a real run (§3): `animate` failed, the catalogue kept going.
- 7 targets got a logged `[DEBUG] built program …` line in this specific run before it was cut off:
  `architect`, `block`, `cad`, `cad-extension-aec-building`, `cad-extension-aec-building-energy`,
  `cad-extension-aec-building-structure`, `cad-extension-spatial-shape`. Plus 1 logged failure: `animate`.
- 51 plugin directories exist under `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/`
  with `*_component.core.wasm` artifacts on disk — but most of these predate this run (built by my
  earlier individual `cargo check`/`cargo build -p <crate> --target wasm32-wasip2` verification passes in
  §1–2, or by other lanes/sessions) and are **not** independent confirmation of *this* orchestrator run
  reaching them; only the 7 above are confirmed by this run's own log.
- I did not re-attempt the full run a second time before writing this report, per the coordinator's
  explicit instruction to land the report first rather than start new work.

**If a fresh, uninterrupted run is needed**: re-running `bun ./📜️script.ts plugin s` now should be
materially faster than the run in this log, since every crate's own `cargo` incremental cache (and
`sccache`) is now warm from all the individual verification builds already done in §1–2 — the earlier run
spent most of its time on cold/near-cold compiles.

## 5. Regression checks (absolute rules)

| Command | Required bar | Actual | Log |
|---|---|---|---|
| `cargo test -p semio-s-plugin-space --lib` | 210/0 | **210 passed / 0 failed** | `🧪️5-d-space-test-regression.txt` |
| `cargo check -p semio-s-plugin-writer --target wasm32-wasip2` (spot-check, already built) | 0 errors | `EXIT:0` | `🧪️5-d-spotcheck-3-passing.txt` |
| `cargo check -p semio-s-plugin-flow --target wasm32-wasip2` (spot-check, already built) | 0 errors | `EXIT:0` | `🧪️5-d-spotcheck-3-passing.txt` |
| `cargo check -p semio-s-plugin-vcs --target wasm32-wasip2` (spot-check, already built) | 0 errors | `EXIT:0` | `🧪️5-d-spotcheck-3-passing.txt` |

No `#[cfg]`-deletion or stubbing anywhere — every fix above is a real source-level correction (missing
import, stale module path, missing `.map_err(...)?`, missing type annotation, dead-code removal with a
verified-zero-callers check, visibility widening).

## 6. Still failing — owning ticket + evidence (as required)

| Crate | Error class | Owning ticket | Evidence |
|---|---|---|---|
| `semio-s-plugin-animate` | `E0432` unresolved `InteractionView` import, `E0560` `SvgSnapshot.lexical`, `E0063` `Mp4Track` missing fields, `E0282` type annotation | Worker brief explicitly lists this as known-foreign/other-ticket, do not fix | `🧪️5-d-sweep1.txt` (`=== semio-s-plugin-animate ===` block); root commit `c8a29e41c5` 2026-08-16 20:26:15 +0200 per lane 4-E, file itself last touched by the repo's continuous auto-commit at `0b9f1d3a04` 2026-08-17 12:10:50 +0200 (auto-commit timestamps churn constantly on this live tree; the root-cause commit is the meaningful one) |
| `semio-s-plugin-layout` | `E0432`/`E0560`/`E0609` — consumes stdio's `DwgSnapshot`/`DwgDecodeStatus`, fields renamed/removed | `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` (still open — `🗄️stdio/**` forbidden) | `🧪️5-d-sweep1.txt` (`=== semio-s-plugin-layout ===` block, all 12 errors reference `Dwg*` types) |
| `semio-s-plugin-note` | `E0560` `SvgSnapshot` has no field `lexical` | `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` (still open) | `🧪️5-d-sweep1.txt` (`=== semio-s-plugin-note ===` block) |

## 7. Pipeline finding for lane 5-C (recorded, not fixed — outside this lease)

Lane 5-C reported `plugin-modules/*/*_component.core.wasm` files reject `Component::from_binary`.
Verified directly on this machine's own build output (`architect`'s freshly-built artifacts, both files
timestamped from this session's own `buildPlugin` run):

```
🔌️plugin-modules/architect/semio_s_plugin_architect_component.core.wasm  → 00 61 73 6d 01 00 00 00  (core-module encoding: version=1, layer=0)
target/wasm32-wasip2/debug/semio_s_plugin_architect.wasm                → 00 61 73 6d 0d 00 01 00  (Component Model encoding: layer=1)
```

This is **by design, not a bug**: `buildPlugin`'s own doc comment (script.ts, `🪶️` region) states
"plugin-modules never receives a copy of the full component `.wasm`… the browser only ever fetches jco's
extracted `${componentBase}.core.wasm`… native `os run` now reads straight from `target/`." Any native
Rust loader calling `Component::from_binary` on a `plugin-modules/**/*.core.wasm` path is pointed at the
wrong artifact for that purpose — the true component binary is
`target/wasm32-wasip2/<profile>/<crate_name_with_underscores>.wasm`, produced directly by `cargo build`
(not by jco). Not fixed here since it's a lane-5-C-owned consumer, not a build-pipeline defect; flagging
with exact evidence so it isn't re-investigated as a mystery.

## Changed files

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️replace-points/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/🦀️component.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧭️navigator/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (lines 862–888 only)

Logs: `🧪️5-d-sweep1.txt`, `🧪️5-d-refixed-sweep.txt`, `🧪️5-d-raster-final-check.txt`,
`🧪️5-d-space-test-regression.txt`, `🧪️5-d-spotcheck-3-passing.txt`, `🧪️5-d-final-plugin-s-run.txt`.

## sharedFileRequests

None. `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` is explicitly
named as a coordination point in the brief (lane 5-A also edits it) — handled by re-reading the exact
region immediately before each edit rather than requesting exclusivity; no conflict occurred.

## What is NOT done

- `animate`, `layout`, `note` remain broken — correctly out of scope (see §6).
- The full `plugin s` end-to-end orchestrator run did not reach a terminal `EXIT:` line in this session
  (see §4) — the resilience fix is proven correct from the partial run, but I cannot state the final
  crate-produced-.wasm count for the complete 59-entry registry with the same rigor as the individually
  re-verified 10-crate fix set. A follow-up run (now warm-cached) would finish materially faster.
- Did not investigate or touch lane 5-C's `Component::from_binary` consumer code — out of this lane's
  lease; §7 records the exact evidence for whoever owns that loader.
- Ticket left open — coordinator owns `ticket_close`.
