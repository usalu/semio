# 📓️ terra — D1-descriptor-emission — report

CARGO_TARGET_DIR used throughout: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target-d1`

## Peer-coexistence

- **Liveness check before starting** (per `important.md`): all 11 peer-stable plugins' `git log --date=iso --oneline -3` showed only the stale `🌙️06☀️04` history and zero files with mtime in the last 30 minutes. Proceeded on all 11.
- **`CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`**: never touched a declaration channel (`.artifact(declaration())` / `.declare_artifact(artifact())`), never deleted a `definition()` row. Descriptor emission only reads whatever surface each plugin already registers.
- **A sibling packet inside THIS ticket (self-labelled "Z1" in its own doc comments) is concurrently editing the exact shared file I have `path_scope` permission for** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`. Its staged diff (visible via `git diff --cached`) is a `dead_code` cleanup plus a real hardening: `register_composer_entries`/`register_subset_validator` now return `Result` and the `subset!` macro `.expect()`s on it ("a failure here means a dialect collision, a real programmer error"). **This is very likely why several of my capability-claim and dialect-collision failures surfaced as hard errors** — Z1's change turns a previously-quiet registration collision into a loud one. My own edit (the `DESCRIPTOR_MIGRATED_PLUGINS` line) is a single isolated hunk inside Z1's larger diff — confirmed via `git diff --cached` that no other line of mine was touched and none of Z1's lines were touched by me.
- **A third, unidentified live session** independently modified 13 files under `✏️s/🔌️plugins/🗄️stdio/**` and one under `✏️s/🔌️plugins/🧩️puzzle/.../🌉️wasm/🦀️component.rs` (+37/−14 total) — discovered already staged via `git status`, mtimes ~17:45–17:53. **I did not touch any of these files.** Plausibly a sibling D-wave packet independently repairing the same stdio/puzzle capability gaps I measured as failing.
- Net effect: my `stdio`/`puzzle` (and possibly others') capability-claim failures below are a snapshot at the moment I ran them and may already be fixed by the time this report is read — re-measure before treating them as still-open.

## Disk stewardship

Mid-session, the coordinator flagged `🎯️target-d1` at 84 GB after 15 plugins, extrapolating to ~180 GB against then-178 GB free. Measured the real cause: `wasm32-wasip2/debug/incremental/` (52 GB) and `debug/incremental/` (24 GB) — Rust's incremental-compilation cache, fully regenerable, not the `.wasm` outputs themselves (which totalled only 730 MB). Deleted both `incremental/` directories plus the top-level `.wasm` files (also regenerable — the committed descriptor is the artifact of record). Repeated this prune 2–3 more times through the run. Final state: `🎯️target-d1` ≈ 12 GB, 244 GB free (`df -g /System/Volumes/Data`). Never dropped anywhere near the 60 GB stop-line.

## Per-plugin table — peer-stable batch (11, ratchet-eligible)

| plugin | crate | describe exit | descriptor committed | added to ratchet | `cargo test --lib` |
|---|---|---|---|---|---|
| 🎬️sequence | semio-s-plugin-sequence | 0 | yes | yes | 146/146 ok, `descriptor_is_fresh` ok |
| 🌿️vcs | semio-s-plugin-vcs | 0 | yes | yes | 59/59 ok, `descriptor_is_fresh` ok |
| 📋️forms | semio-s-plugin-forms | 0 | yes | yes | 123/125 — 2 **pre-existing, unrelated** failures (`inference_determinism_law`, `try_wizard::render_falls_back_to_a_placeholder_for_an_empty_document`); `descriptor_is_fresh` itself **ok** |
| 🪵️sourcing | semio-s-plugin-sourcing | 0 | yes | yes | 92/92 ok (1 ignored), `descriptor_is_fresh` ok |
| 🕸️dag | semio-s-plugin-dag | 0 | yes | yes | 113/114 — 1 **pre-existing, unrelated** failure (`two_instances_converge_disjoint_edits_via_backbone`, panics inside the shared testkit, not descriptor code); `descriptor_is_fresh` **ok** |
| ➗️mathematical | semio-s-plugin-mathematical | 0 | yes | yes | 265/283 — 18 **pre-existing, unrelated** polynomial/graph-mutation failures (e.g. `insert_point_inverse_is_remove_point_at_same_index`, `is_irreducible_hand_cases`); `descriptor_is_fresh` **ok** |
| ✒️writer | semio-s-plugin-writer | 0 | yes | yes | 126/126 ok, `descriptor_is_fresh` ok |
| 🖍️draw | semio-s-plugin-draw | **101** | **no** (nothing written — build failed before wasm existed) | no | not run |
| 💡️reasoning | semio-s-plugin-reasoning-mindmap | 0 | yes | yes (as `reasoning-mindmap`, its real `pluginId`) | 92/94 — 2 **pre-existing, unrelated** failures; `descriptor_is_fresh` **ok** |
| 🎞️animate | semio-s-plugin-animate | 101 → **0 after fix** | yes | yes | 244/244 ok, `descriptor_is_fresh` ok |
| 🗒️note | semio-s-plugin-note | (already done, prior packet) | yes | yes (pre-existing entry) | not re-run |

**draw — real error, classified, not fixed (outside path_scope):**
```
error: symbol `semio_plugin_bundle_installer_link_shim` is already defined
  --> ✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📦️glue.rs:601:1
```
`plugin_exports!` defines this symbol `#[unsafe(no_mangle)]`; the framework crate's own default is `#[linkage = "weak"]` and should be silently overridden. Confirmed via `cargo tree -f "{p} [{f}]"` that `semio-s-plugin-stdio`'s `plugin-root` feature (which would explain a *second* strong definition from stdio) is **not** enabled in draw's build (`[]`), and grepped the whole `🖍️draw/` tree — only one `plugin_exports!` call exists. The collision is therefore a genuine weak-linkage failure in the wasm32-wasip2 link step itself, not a data/config problem in draw — fixing it would mean editing `🔌️plugin/🦀️component.rs`'s linkage mechanism, outside this packet's `path_scope` (limited there to the ratchet lists only). Left classified, not touched.

**animate — real pre-existing type error, fixed (one line, in path_scope):**
```
error[E0308]: mismatched types
  --> …/🌉️wasm/🦀️component.rs:39:39
        Ok(Self { store: RefCell::new(store) })
                         ------------ ^^^^^ expected `ArtifactStore<...>`, found `Result<ArtifactStore<...>, VcsError>`
```
This file is `#![cfg(target_arch = "wasm32")]`-gated — native cargo never compiles it (matches the standing project note "Native Cargo Misses Wasm-Gated Code"), so this is the first time it has ever actually been built. `PresentStore::new()` (= `ArtifactStore::new`) returns `Result<Self, VcsError>`; the `match` was assigned straight into `RefCell::new` without unwrapping. Fixed by appending `.map_err(|e| JsValue::from_str(&e.to_string()))?` after the `match` block (mirrors every other error-conversion in the same file). Verified: `cargo test -p semio-s-plugin-animate --lib` → 244/244, and `describe` now emits a real (`pluginId: "animate"`) descriptor.

## Per-plugin table — remaining 22 (not peer-stable; none added to ratchet regardless of outcome)

| plugin | crate | describe exit | descriptor committed | error class |
|---|---|---|---|---|
| 🌀️procedural | semio-s-plugin-procedural | 0 (but `pluginId: "assembly-failed"`) | **no — deleted** | **dialect collision**: `dialect:s.stdio.dwg@ac1018/* is already registered by s.procedural2d.composer.dwg` |
| 🌊️flow | semio-s-plugin-flow | 0 | **yes** | — |
| 🌍️gis | semio-s-plugin-gis | 0 (assembly-failed) | **no — deleted** | **dialect collision**, mirror of procedural: `…already registered by s.gismap.composer.dwg` — procedural and gis both claim the same `ac1018` DWG dialect |
| 🎥️shooting | semio-s-plugin-shooting | 0 | **yes** | — |
| 🏗️fem | semio-s-plugin-fem | 0 (assembly-failed) | **no — deleted** | capability-claim: "no declared **composer** capability owns the runtime claims" |
| 🏛️architect | semio-s-plugin-architect | 0 | **yes** | — |
| 🏭️process | semio-s-plugin-process | 0 | **yes** | — |
| 💠️lowpoly | semio-s-plugin-lowpoly | 0 | **yes** | — |
| 📏️layout | semio-s-plugin-layout | 0 (assembly-failed) | **no — deleted** | capability-claim: composer |
| 📐️cad | semio-s-plugin-cad | 0 | **yes** | — |
| 📕️norm | semio-s-plugin-norm | 0 | **yes** | — |
| 📖️playbook | semio-s-plugin-playbook | 0 (assembly-failed) | **no — deleted** | capability-claim: composer |
| 📜️imperative | semio-s-plugin-imperative | **101** | **no** (build failed) | **weak-linkage duplicate symbol** — same class as draw, this time `semio_extension_bundle_installer_link_shim` via `extension_exports!` at `…/🧩️extensions/🧮️math/🦀️component.rs:157` |
| 📸️remodel | semio-s-plugin-remodel | 0 | **yes** | — |
| 🔱️trinity | semio-s-plugin-trinity | 0 (assembly-failed) | **no — deleted** | capability-claim: "no declared **codec** capability owns the runtime claims" |
| 🖨️raster | semio-s-plugin-raster | 0 | **yes** | — |
| 🗄️stdio | semio-s-plugin-stdio | 0 (assembly-failed) | **no — deleted** | capability-claim: "no declared **inference** capability owns the runtime claims" — despite `important.md` calling stdio "already brought into consistency", this specific inference-capability row is still a gap (or was fixed by the third live session after I measured this — see peer-coexistence) |
| 🧩️puzzle | semio-s-plugin-puzzle | 0 (assembly-failed) | **no — deleted** | capability-claim: codec |
| 🧱️block | semio-s-plugin-block | 0 (assembly-failed) | **no — deleted** | capability-claim: codec |
| 🪐️space | semio-s-plugin-space | 0 | **yes** (`pluginId: "s"`) | — |
| 🎪️demonstrator | semio-s-plugin-demonstrator | 0 (assembly-failed) | **no — deleted** | **descriptor conflict**, downstream of the above: `artifact kind "kit.catalog" has conflicting descriptors` — confirms `important.md`'s sequencing note that demonstrator (bundling cad/process/puzzle/procedural/gis/sourcing panes) must migrate last, since it aggregates every unresolved conflict in its dependencies |
| 🔋️energy | semio-s-plugin-energy | **not attempted** | no | pre-existing, already known (D0): `[lib]` in `Cargo.toml` declares no `crate-type` at all — produces no wasm artifact to describe. Re-confirmed by reading the file; not worth a build attempt. |

None of these 20 attempted-outside-batch-1 plugins were added to `DESCRIPTOR_MIGRATED_PLUGINS`, successes included — they are not part of the peer-confirmed-stable set this packet was chartered to ratchet, so their descriptors are committed but the freshness gate does not yet enforce them.

## Failure-class tally

| class | count | plugins |
|---|---|---|
| weak-linkage duplicate `#[no_mangle]` symbol (build failure, no descriptor possible) | 2 | draw, imperative |
| capability-claim ("no declared X capability owns the runtime claims") | 7 | fem (composer), layout (composer), playbook (composer), trinity (codec), stdio (inference), puzzle (codec), block (codec) |
| dialect collision (two plugins claim the same dialect id) | 2 | procedural, gis (mirror pair, `s.stdio.dwg@ac1018/*`) |
| descriptor conflict (aggregation of the above) | 1 | demonstrator (`kit.catalog`) |
| pre-existing no-wasm gap (not attempted) | 1 | energy |
| pre-existing unrelated test failures (descriptor_is_fresh still passes) | 23 individual test failures across 4 crates | forms (2), dag (1), mathematical (18), reasoning-mindmap (2) |

**Not fixed and not fabricated in every failure case**: every `"pluginId": "assembly-failed"` descriptor produced by a failed `try_build()` was deleted immediately, never left on disk or reported as success.

## Registry gate

`bun nx run @semio-tech/plugin-registry:check` was run twice:
- **Early in the run** (after the 9 peer-stable emissions): printed `descriptor gate: 10/59 crates have a 🔣️descriptor.json` and exited 0 ("catalog is fresh").
- **At the end** (after all 19 new descriptors landed): **fails** — `plugin registry catalog is stale: generated/🔣️plugins.json, generated/🟦️plugins.ts` (exit 1), because the aggregated catalog file has not been regenerated to reflect the new descriptor count. Per the coordinator's explicit instruction, `plugin-registry:generate` was **not** run — that is registrar territory and the coordinator will run it after this report lands.

**Measured directly on disk** (not through the now-stale `check` command): `find ✏️s/🔌️plugins -maxdepth 2 -iname 🔣️descriptor.json | wc -l` → **20** (note + 9 ratcheted + 10 unratcheted). This is the true count; `20/59` is the honest headline once the registrar refreshes the catalog and reruns `check`.

## Ratchet — `DESCRIPTOR_MIGRATED_PLUGINS`

Edited **only** the array literal at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:16282` (line number shifted by Z1's concurrent edits elsewhere in the same file; content confirmed via `git diff --cached` to be an isolated single-line change):

```rust
const DESCRIPTOR_MIGRATED_PLUGINS: &[&str] = &["note", "sequence", "vcs", "forms", "sourcing", "dag", "mathematical", "writer", "reasoning-mindmap", "animate"];
```

Every one of these 10 was verified with its own `cargo test -p <crate> --lib` run above; `descriptor_is_fresh` passes for all 10 (9 new + note, not re-run since already accepted by a prior packet).

**Explicitly NOT added, including successes**: flow, shooting, architect, process, lowpoly, cad, norm, remodel, raster, space. These 10 have committed, genuine (non-placeholder) descriptors, but were emitted outside the peer-confirmed-stable batch-1 set this packet was chartered to ratchet against. Adding them to the ratchet was out of this packet's charter and was not done.

## Residual risk (in my own words, per coordinator's request)

An emitted-but-unratcheted descriptor (the 10 above) has no test guarding its freshness. If any of those 10 plugins' declarations change under a future peer edit, the committed `🔣️descriptor.json` silently goes stale — nothing turns red, `plugin-registry:check` will keep reporting it as present, and the generated catalog (once regenerated) will carry incorrect capability/manifest data for that plugin into every consumer of `🤖️generated/🔣️plugins.json`. This is a **data-correctness** risk, not a build-breakage risk: nothing fails, something is quietly wrong. The only plugins protected against this are the 10 in the ratchet, where `descriptor_is_fresh` will fail loudly the moment the checked-in descriptor and a freshly-assembled one diverge. Whoever eventually confirms peer-stability for the other 10 (or the remaining 22 as they clear their own blockers) should ratchet them the same way this packet did for batch-1: describe → commit → `cargo test --lib` proving `descriptor_is_fresh` → then and only then add the name.

## Commands run (representative; full list is every `describe`/`cargo test` invocation referenced above)

```
CARGO_TARGET_DIR=…/🎯️target-d1 bun ✏️s/🔌️plugins/<p>/📦️packages/🦀️rust/📜️script.ts describe    # ×32 (all plugins except energy)
CARGO_TARGET_DIR=…/🎯️target-d1 cargo test -p <crate> --lib                                          # ×10 (every ratchet candidate)
bun nx run @semio-tech/plugin-registry:check                                                        # ×2 (mid-run: 10/59 fresh; final: stale, exit 1)
```
Exact exit codes are recorded per-plugin in the tables above; every one was captured via `; echo "REAL EXIT: $?"` immediately after the command (not through a pipe, which would have masked the real status — caught and corrected once, on draw, early in the run).

## Files touched

- **Created** (committed via the repo's auto-commit bot, confirmed via `git log`): `🛂️descriptor.semio` + `🔣️descriptor.json` at the owner root of: 🎬️sequence, 🌿️vcs, 📋️forms, 🪵️sourcing, 🕸️dag, ➗️mathematical, ✒️writer, 💡️reasoning, 🎞️animate, 🌊️flow, 🎥️shooting, 🏛️architect, 🏭️process, 💠️lowpoly, 📐️cad, 📕️norm, 📸️remodel, 🖨️raster, 🪐️space (19 plugins × 2 files).
- **Edited**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (`DESCRIPTOR_MIGRATED_PLUGINS` array only, within `path_scope`); `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs` (one-line `Result` unwrap fix).
- **Created then deleted** (garbage, never committed): `assembly-failed` placeholder descriptors for 🌀️procedural, 🌍️gis, 🏗️fem, 📏️layout, 📖️playbook, 🔱️trinity, 🗄️stdio, 🧩️puzzle, 🧱️block, 🎪️demonstrator.
- **Not touched**: no declaration-channel files, no capability rows repaired (all capability-claim failures were classified and left for their owning packets), root `📜️script.ts`/`project.json`/registry files, `🤖️generated/**`.

## Lease-requests

None. All work stayed within `✏️s/🔌️plugins/**` and the single permitted line in `🔌️plugin/🦀️component.rs`.
