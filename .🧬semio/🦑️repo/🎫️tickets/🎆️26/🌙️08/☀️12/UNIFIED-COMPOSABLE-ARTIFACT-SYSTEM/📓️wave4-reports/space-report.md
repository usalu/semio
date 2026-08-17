# W4 batch D — `space` composes stdio `model`? — investigated, found not applicable; 3 real pre-existing bugs fixed instead

**ucas-status: complete (no-op composition, evidence-based) — 94 tests: 58 passed, 36 failed, 0 skipped (stable across 3 runs); all 36 failures independently traced to ONE pre-ticket, out-of-boundary root cause; sharedFileRequest filed for it**

Crate: `semio-s-plugin-space` (`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml`). ~8.1k lines of Rust across the plugin.

## Summary — no composition edit was made, and here is why

Per `📌️important.md`/`📓️migration-recipe.md`'s instruction to "read space's current artifact root file first to see exactly what it duplicates today before planning the change," I did that, then went further: I read every persisted-state struct this plugin owns, and every UI/command file that could plausibly hold spatial/model content. Conclusion, backed by direct evidence below: **`✏️s/🔌️plugins/🪐️space/**` owns exactly one artifact (`s.home`), and its snapshot has never carried spatial-tree/object/model content — there is nothing here to replace with a composed `model` child.** Fabricating a composed-child field with no real duplicated content to migrate would violate this ticket's own "never invent vocabulary/content to fill a gap" discipline (`📌️important.md`'s mutation-authoring section states this principle explicitly for the adjacent case of un-authorable mutation facets; I'm applying the same standard here).

### Evidence

1. **The plugin's only artifact, `SHomeSnapshot`** (`🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`), has exactly two fields: `schema: String` and `catalog_generation: u64` — a cache-invalidation counter, not content. Confirmed unchanged in shape across the last 3 commits touching the file (`git log`). The plugin's OWN doc comments already say this explicitly, written before I arrived:
   - `🎛️apps/🏠️home/🎚️config/🦀️component.rs:5`: *"`SHomeSnapshot` is a two-field counter document (`schema` + `catalog_generation`) with no tree..."*
   - `🧬️schema/💡️inferences/🦀️component.rs:7`: *"The home snapshot is just two scalars (`schema`, `catalogGeneration`) — no positions, no..."*
   - `🧬️schema/🧬️mutations/🦀️component.rs:14`: *"(`catalog_generation`, the counter that forces a studio-list re-materialize)."*
2. **The plugin declares no second artifact.** `find ... -name 🦀️component.rs | grep 🗿️artifacts` returns only `🏠️home/🦀️component.rs` — one artifact root, period.
3. **The apps' UI state that superficially looks spatial (`SpaceApp`'s `WorkflowSnapshot`, the `🕸️compiled-dag`/`🔄️workflow` windows, `🧩️nodes`/`🔍️instance-nav` commands) wraps a FRAMEWORK-owned kernel type**, not plugin content: `use semio_framework_os::{... WorkflowSnapshot ...}`, declared in `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` — out of this plugin's ownership and out of my write scope (`📌️important.md`: framework kernel files are read-only for plugin fan-out agents). The plugin's own root file says this outright (`✏️s/🔌️plugins/🪐️space/🦀️component.rs:63-68`): *"`🪐️space`'s app wraps the kernel-owned `WorkflowSnapshot` and owns no `🗿️artifacts` node of its own in this plugin... the codec is keyed by a foreign kind (`OS_SPACE_SCHEMA`/`"os.space"`, owned by framework/os's `SpaceSnapshot`, not by any type this plugin declares)."*
4. **Independent corroboration from a prior ticket.** `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w1d-semio-s-plugin-space-report.md` (W1d, a different agent, different ticket) reached the identical conclusion independently while trying to close this plugin's `.setup()` residue: `SpaceApp`'s document codec is keyed to a "foreign, kernel-owned kind string... this plugin has no legitimate right to author a schema for a type it doesn't own," calling it a "genuine category-4 gap," not an oversight.
5. **The catalogue/vfs/nodes command files I checked for hidden duplicated content** (`📌️panels/🛍️catalogue`, `🎮️commands/🗂️vfs`, `🎮️commands/🧩️nodes`) hold UI tree-building logic over the framework registry or VFS-node navigation state — no "instance model" or "object" struct duplicating stdio's `model` subset shape (BIM/IFC elements+geometry-refs+transforms) anywhere.
6. stdio's `✳️model` subset itself (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/**`) is architectural/BIM content (IFC/BCF import-export) — the same subset `cad`/`architect`/`energy`/`layout` compose. Nothing in `space` resembles this domain.

**Conclusion**: the design doc's one-line summary table (`space→C:model`) does not match this plugin's actual content. I recommend the orchestrator/W7 canary check treat this as a confirmed, evidence-based non-applicability rather than a gap — see the note at the bottom of this report for what W7 should expect when it re-checks this plugin.

## What I actually did instead: fixed 3 real, in-scope, pre-existing compile breaks (baseline was RED, not clean)

Per the recipe's step 1 ("run `cargo check -p <crate> --all-targets` BEFORE touching anything, note the baseline"), the true baseline — before any edit of mine — was **6 compile errors**, not 0:

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-space --all-targets
→ error: could not compile `semio-s-plugin-space` (lib) due to 6 previous errors
```

All 6 traced to real, in-scope, pre-existing field/type renames elsewhere in the repo that this plugin's own consumer code was never updated to match. Traced by real commit date (`git log -1 --date=iso`, per `📌️important.md`'s fake-date warning — never parsed message digits):

| # | File (mine, in `✏️s/🔌️plugins/🪐️space/**`) | Cause | Root-cause commit (real date) |
|---|---|---|---|
| 1-2 | `🚪️io/📤️export/🧵️serializers/.../📊️csv/.../🦀️component.rs` | stdio's `CsvSnapshot` renamed `headers`/`rows` → `has_header`/`records` (RFC4180 rework) | `0da3884894`, **2026-08-11 02:11:48** |
| 3-4 | `🚪️io/📥️import/🧩️deserializers/.../📊️csv/.../🦀️component.rs` | same rename, consumed on the import side | same commit |
| 5 | `🚪️io/📤️export/🧵️serializers/.../🔣️json/.../🦀️component.rs` | stdio's `JsonSnapshot.value` changed from `serde_json::Value` to stdio's own lexeme-preserving `JsonValue` (RFC8259 rework) | pre-dates this ticket (same rework wave as above) |
| 6 | `🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs` | framework's `OsAppRegistration.document: Vec<String>` renamed to `.breadcrumb: Vec<String>` | `20252aa16d`, **2026-08-12 23:24:26** (during this ticket's window, but framework churn, not composition-related) |

All 4 commits above pre-date or are contemporaneous with unrelated framework/stdio rework — none are mine, none are DKM's math dissolution, none relate to model composition. Every other plugin's identically-shaped "any subset → csv/json" export boilerplate has the same latent break wherever it hasn't yet been touched by this ticket's fan-out — confirmed by checking `demonstrator`'s still-broken sibling file (`🎪️playground/.../📊️csv/.../🦀️component.rs`, still uses `headers`/`rows`) as a live control.

### Fixes applied (minimal, behavior-preserving)

- **CSV export** (`serialize`): `SHomeSnapshot` was never able to produce real `headers`/`rows` content anyway (its own JSON shape is `{schema, catalogGeneration}`, so the old lookup-by-key always missed and silently produced an empty table). Preserved that exact degenerate behavior under the new shape: `CsvSnapshot { schema, has_header: true, records: Vec::new() }`. Documented why in a doc comment rather than inventing table content the snapshot never carried.
- **CSV import** (`deserialize`): symmetric — the old bridge always failed to deserialize (missing required `schema` key), preserved that exact behavior under the new field names (`hasHeader`/`records` in the constructed JSON, still missing `schema`).
- **JSON export** (`serialize`/`serialize_bytes`): switched to `JsonSnapshot::from_value(value)` + `write_json_pretty(&value)`, the exact pattern already used by `demonstrator`'s and `trinity`'s already-migrated equivalents (copied, not invented) — real, correct behavior (this one COULD produce real content, unlike the two above, so I matched the established correct pattern rather than a degenerate stand-in).
- **`catalogue.rs`**: `row.document.clone()` → `row.breadcrumb.clone()` (and one stale doc-comment line updated to match) — mechanical rename to the new framework field name, same `Vec<String>` type, same semantics (breadcrumb segments).

### Verification

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-space --all-targets
→ Finished `dev` profile [unoptimized] target(s) — 0 errors (was 6)
```

One transient re-appearance of an UNRELATED error (`semio-framework-os` failed with `E0432: unresolved imports media_export_raster::register_dwg_import_handler` etc. — DKM's live `DISSOLVE-KERNELS-...` fan-out mid-removal of those 6 functions) was observed on a later recheck, confirmed via `grep -- "🪐️space"` on the full log (**zero** matches — every error was under `🧰️framework/🛍️products/💻️os/🖥️host/**`) and via `stat` on the offending file (mtime newer than any commit — live uncommitted edit). Retried per the transient-failure protocol; cleared on the 3rd retry. Final state reconfirmed clean.

## Test suite

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo nextest run -p semio-s-plugin-space --no-fail-fast
→ 94 tests run: 58 passed, 36 failed, 0 skipped
```

Reproduced **3 times** (not flaky) — identical 58/36 split every run, identical failing-test set every run.

### All 36 failures trace to ONE root cause — out of my scope, sharedFileRequest filed

Every one of the 36 failures panics at the exact same location: `✏️s/🔌️plugins/🪐️space/🦀️component.rs:41`, the `.expect("bundled example/✏️demo.s is valid WorkflowSnapshot DSL text")` in `parse_demo_space_document()`. Same message every time:

```
TextError { message: "list element made no progress at Ident 'document-ref' — likely an unrecognized field key", ... }
```

Root cause, traced fully:

- The DSL is `include_str!`-compiled from `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.space.studio.dsl.semio` — **outside `✏️s/🔌️plugins/🪐️space/**`, framework territory, out of my write scope.**
- `WorkflowNode`'s field (`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs:431`, also framework-owned) was renamed `document_ref` → `artifact_ref` in commit `c31024cc6c`, real date **2026-08-10 23:04:11** — `git log -1 --date=iso` confirmed, genuinely pre-dates this ticket's open (2026-08-12 15:02:49) by ~1.5 days.
- The bundled DSL fixture was never updated to match (`git log` on the fixture shows its last real edit at `503afb28b4`, **2026-08-06**, before the rename existed) — it still emits the DSL grammar's now-dead key `document-ref=`.
- **Verified the fix is complete and sufficient** without touching the framework file: added a temporary `#[cfg(test)] mod debug_fixture_patch_probe` in my own plugin's root `🦀️component.rs` that patches `DEMO_STUDIO_DSL` **in memory** (`.replace("document-ref=documents/", "artifact-ref=artifacts/")`) and re-parses it — passed clean (`ok`, both glue-mirrored copies of the module). Removed the probe immediately after (`grep -rn debug_fixture_patch_probe` → 0 hits, confirmed).

**5 occurrences** in the fixture, all on its single `graph { ... }` line:
```
document-ref=documents/node-app-draw-1   → artifact-ref=artifacts/node-app-draw-1
document-ref=documents/node-app-draw-2   → artifact-ref=artifacts/node-app-draw-2
document-ref=documents/node-app-writer-1 → artifact-ref=artifacts/node-app-writer-1
document-ref=documents/node-app-raster-1 → artifact-ref=artifacts/node-app-raster-1
document-ref=documents/node-app-note-2   → artifact-ref=artifacts/node-app-note-2
```
(Only the KEY needs renaming for the grammar to accept it; the VALUE prefix rename to `artifacts/` is cosmetic consistency with the Rust-side default `format!("artifacts/{node_id}")`, not required for parsing.)

This is a **pre-existing bug, confirmed by real commit date, entirely outside this plugin's boundary and outside this ticket's stated scope for a plugin fan-out agent** (framework kernel + framework-owned example asset, both read-only per `📌️important.md`). I did not attempt a workaround inside my own plugin (e.g. forking a local copy of the shared fixture) — that file's own path segments (`♻️reuse/.../♻️reuse/`) mark it as intentionally shared/reused content, not mine to fork, and `📌️important.md` explicitly says to write this up rather than route around it.

## sharedFileRequests

**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.space.studio.dsl.semio`
**Region**: single DSL line 3 (`graph { schema=workflow.graph nodes=[...] }`), 5 occurrences of `document-ref=documents/`
**Fix**: find/replace `document-ref=documents/` → `artifact-ref=artifacts/` (5×), matching `WorkflowNode.artifact_ref`'s rename (commit `c31024cc6c`, 2026-08-10, pre-tickets `document_ref`→`artifact_ref`).
**Reason**: blocks `parse_demo_space_document()`/`demo_space_projection()` — used by both `home` and `space` apps' fixtures — from parsing at all; every test exercising either app's demo path fails identically. Verified sufficient (no second latent break) via an in-memory patch probe (added and removed in this pass, see above) — a straight find/replace should turn 36 failures into 0 with no further changes needed on the space-plugin side.
**Owner**: framework/os (not in the hot-file-ownership table by exact path, but squarely under `🧰️framework/`, out of a plugin fan-out agent's scope per `📌️important.md`'s general rule).
**Patch file**: not written separately — the fix is a single 5-occurrence find/replace, given verbatim above; no patch file needed.

## Concurrent-churn observations

1. **9 files were already staged (not mine) before I started**, confirmed via `git diff --cached` at the very start of this pass: small, mechanical `Transient`/`TransientMutation` associated-type additions to `HomeApp`/`SpaceApp` plus "persistent fields only" → "artifact-lane fields only" doc-comment rewording across the `s.home` schema facets (snapshot/mutations `.rs`/`.ts`/`.graphql`/`.proto`). Unrelated to composition, didn't touch content fields, left untouched.
2. **`🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs` was live-edited by another session mid-pass** (not by me) — its own new comment names the source: "Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1." `register_dwg_import_handler` was deleted from the framework and this test call site was adapted to the replacement primitive (`register_os_media_import_handler_kind`). Left as-is per the standing instruction not to revert an in-flight concurrent edit (matches `cad-report.md`'s identical precedent) — confirmed it doesn't conflict with anything in my diff, and my final green `cargo check`/stable 58/36 test split already reflect this file's current (adapted) state.
3. **One transient `semio-framework-os` compile break** during verification (`E0432`, DKM's live removal of `media_export_raster::{register_dwg_import_handler, register_solid_exporter, ...}` mid-flight) — zero errors in my own boundary (`grep -- "🪐️space"` on the full log → 0 matches), cleared on retry per the transient-failure protocol. Not blocked-mechanism; resolved naturally.

## Note for W7's space-plugin canary check

This plugin's own compile+test health is real and stable (0 compile errors, 58/94 passing, 36/94 failing for one fully-diagnosed pre-existing reason with a filed fix). **Do not expect a composed `model` child anywhere in this plugin** — none was added, for the evidence-based reasons above. If W7's canary check specifically probes for `s.stdio.semio.model` composition inside `space`, it will correctly find none; that is the intended, investigated outcome of this pass, not a missed step.

## Files touched (all inside `✏️s/🔌️plugins/🪐️space/**`)

- `🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs` — csv export field-rename fix.
- `🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs` — csv import field-rename fix.
- `🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — json export bridged onto `JsonValue`/`write_json_pretty`.
- `🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs` — `OsAppRegistration.document` → `.breadcrumb` rename fix (2 lines: field access + a stale comment).
- `🦀️component.rs` (plugin root) — temporary debug probe added and fully removed in the same pass (net zero diff, verified via `git diff`).

No files touched outside this plugin's boundary. No `🧬️mutations/**` facets authored or modified (the existing single `change-catalog-generation` triad was already conformant and untouched). No `📦️glue.rs`/`📦️index.ts` touched.

ucas-status: complete
