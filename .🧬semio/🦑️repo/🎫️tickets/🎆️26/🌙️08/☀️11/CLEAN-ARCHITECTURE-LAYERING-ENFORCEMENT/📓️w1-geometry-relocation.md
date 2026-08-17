# W1 Geometry Relocation — semio-s-3d / semio-s-2d → semio-framework-3d / semio-framework-2d

Agent: geometry-relocation subagent. Scope: move ✏️s/🔨️modules/🧊️3d and ✏️s/🔨️modules/◻2d into 🧰️framework/🔨️modules/, rename crates, fix Cargo.toml graph, fix the ui-wgpu glue.rs #[path] mount.

## Plan
1. Confirm current crate names/roles (done: semio-s-3d, semio-s-2d, role=s-module, both `[lib] path = "📦️glue.rs"`, no explicit [lib] name → extern crate name derives from package name).
2. `mv` the two module dirs into 🧰️framework/🔨️modules/.
3. Rename [package] name + role in both moved Cargo.toml.
4. Update root Cargo.toml: workspace members (2 lines) + workspace.dependencies (2 lines), new paths, new names.
5. Enumerated ALL Cargo.toml files repo-wide referencing semio-s-3d/semio-s-2d (grep). In-scope (own): root, 🌊️flow(os), ♾️infinite(os), wgpu renderer target, 🖥️host, s-plugins: 🌊️flow/🧩️extensions/📐️brep, 🌊️flow/🧩️extensions/🖍️draw, 🏭️process, 📐️cad, 🎪️demonstrator, 🌀️procedural, 🖍️draw, 💠️lowpoly. Out-of-scope: scratch Cargo.toml files inside OTHER tickets' folders (not workspace members) — left untouched.
6. Recomputed every relative `path = "..."` via python os.path.relpath (emoji-safe) rather than manual counting.
7. Updated glue.rs `#[path]` mount only (line ~222) to new shorter relative path.
8. IMPORTANT DECISION on step 7 of task instructions ("fix `use semio_s_3d` / `use semio_s_2d` occurrences repo-wide"): this conflicts with my explicit file-ownership restriction (Cargo.toml only for consumer crates; NOT their .rs source). The consumer .rs files using `semio_s_3d::...` / `semio_s_2d::...` live in os host, os flow, ui shell, and multiple s-plugins (cad, draw, process, demonstrator, procedural, lowpoly) — none of these .rs files are in my ownership list. Per the CRITICAL OPERATIONAL RULES ("only edit files explicitly listed... do not touch any other file, even if related — other parallel agents own them") and step 8's explicit allowance to stop when remaining errors are outside ownership, I performed the LITERAL clean rename (no compat aliasing) in every Cargo.toml I own, and did NOT touch any .rs file outside my ownership. This is expected to break `cargo check --workspace` in those foreign-owned crates with "unresolved crate semio_s_3d/semio_s_2d" errors — reported at the end, not fixed.

## Execution log

1. Moved directories: `✏️s/🔨️modules/🧊️3d` → `🧰️framework/🔨️modules/🧊️3d`, `✏️s/🔨️modules/◻2d` → `🧰️framework/🔨️modules/◻2d` (plain `mv`, filesystem-only, no git op).
2. Renamed both crates in their own Cargo.toml: `semio-s-3d` → `semio-framework-3d`, `semio-s-2d` → `semio-framework-2d`; `role = "s-module"` → `role = "framework"` (matches convention confirmed against 🧰️framework/📦️packages, 🖱️ui, 🗺️surface). Reworded 3d's description to drop "s-module" phrasing.
3. Fixed self-referencing files inside the moved trees (in-ownership, since I own the whole moved subtree): `📋️project.json` (nx project name + namedInputs + all `cwd` fields), `📜️script.ts` (router docstring + `runCargoTestBudgeted`/`runCmd -p`/`runCargoLint` args) for both modules, and `benches/kernel.rs` for 3d (doc comments + `use semio_s_3d::` → `use semio_framework_3d::`). Import path to the shared TS script-router library in `📜️script.ts` needed no change — both old and new locations are 5 path segments deep from repo root, same relative depth.
4. Root `Cargo.toml`: updated the 2 workspace `members` paths and the 2 `workspace.dependencies` entries (kept their existing position under the "plugin" section rather than reorganizing headers/counts, to minimize collateral diff in a file every other agent also edits).
5. Enumerated every Cargo.toml repo-wide referencing `semio-s-3d`/`semio-s-2d` via `grep -rl`. In-scope crates updated (path recomputed with `os.path.relpath`, verified every result resolves to a real directory via a second Python pass):
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` (both 3d+2d, no alias → literal rename)
   - `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml` (had `semio_s_3d = { package = "semio-s-3d" }` alias → renamed key to `semio_framework_3d`, package to `semio-framework-3d`; verified this dep is UNUSED anywhere in infinite's own .rs source, so this is a no-op-safe rename)
   - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml` (3d, no alias → literal rename)
   - `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml` (3d, no alias → literal rename)
   - `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml` (3d, no alias → literal rename)
   - `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/Cargo.toml` (2d, no alias → literal rename)
   - `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml` (3d, no alias → literal rename)
   - `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` (3d, no alias → literal rename)
   - `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` (3d, no alias → literal rename)
   - `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` (3d, dev-dependencies, no alias → literal rename)
   - `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml` (had `semio_s_2d = { package = "semio-s-2d" }` alias, and this one's source DOES use `semio_s_2d::` → **preserved the alias key**, only updated `package` value + `path`, so this consumer keeps compiling untouched)
   - `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml` (3d, no alias → literal rename)
   Out of scope, deliberately left untouched: two Cargo.toml files inside OTHER tickets' scratch folders (`.🦑️repo/🎫️tickets/…/OS-IMPLEMENTATIONS-FULL-ERADICATION/🧪isolated-workspace/Cargo.toml`, `.../DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT/🔧️flow-check/Cargo.toml`) — neither is a workspace member, both belong to other agents' tickets.
6. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`: updated the one `#[path = "...✏️s/🔨️modules/🧊️3d/🎬️scene/🦀️component.rs"]` mount → `#[path = "../../../../../🧊️3d/🎬️scene/🦀️component.rs"]`. Verified resolves to the real file post-move.
7. **Deliberate scope decision on task step 7** ("fix `use semio_s_3d`/`use semio_s_2d` occurrences repo-wide"): repo-wide grep found these imports used in os host, os flow, ui-wgpu Shell component, and 6 s-plugins (cad, draw's own artifact engine, process, demonstrator, procedural, lowpoly) — none of these consuming `.rs` files are in my file-ownership list (only their Cargo.toml is). Per the CRITICAL OPERATIONAL RULES ("only edit files explicitly listed... other parallel agents own them") and step 8's explicit allowance to stop and report when remaining errors are outside ownership, I did the literal crate rename in every Cargo.toml I own (no compatibility aliasing introduced beyond the two pre-existing alias sites, which I preserved as-is) and did **not** touch any foreign `.rs` file. This is confirmed to produce `error[E0433]: cannot find module or crate semio_s_3d/semio_s_2d` in those foreign files — see cargo check results below.

## cargo check --workspace results

Ran `cargo check --workspace` twice (first to completion, second redirected to a log file with correct `2>&1` ordering, saved at `/private/tmp/claude-501/.../scratchpad/cargo-check-full.txt` during the session, not persisted in-repo). Full workspace run does **not** go green, but for reasons entirely outside this ticket's scope:

- `semio-framework-3d` and `semio-framework-2d` (the two crates I moved/renamed) **check clean** — `Checking semio-framework-3d ... 15 warnings` (pre-existing warnings, no errors), `Checking semio-framework-2d ... ` no errors.
- `semio-framework-os-kernel-db` fails: `error: couldn't read 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/../../📄️document/🦀️component.rs: No such file or directory`. Unrelated to geometry/3d/2d. This file/module did not exist at all under `🔨️modules/`, and this exact error is **absent from the ticket's baseline-cargo-check.txt** — this is fresh concurrent churn from another agent moving/deleting the 📄️document module right now (classic "Concurrent Cargo Workspace Churn", see memory note), not caused by me and not fixable within my file ownership.
- `semio-compose-rs` fails: `dsl`/`vcs` unresolved-crate errors in `compose/client/lib/rs/lib.rs`. This is **present verbatim in the ticket's `📸️baseline-cargo-check.txt`** (pre-existing, predates this ticket entirely) — confirmed unrelated to this task.
- `semio-s-plugin-stdio` fails on unrelated `dsl::DslField`/`dsl::DslDiff`/`OpText` derive-macro errors (DXF/XML artifact schema code) — also absent from baseline, i.e. fresh churn from a different concurrent DSL-related agent, not related to 3d/2d/geometry. This blocks `semio-s-plugin-cad` and `semio-s-plugin-lowpoly` (both depend on stdio) from being reachable in the full workspace check.
- Isolated `cargo check -p semio-framework-os-flow` (bypassing the unrelated blockers above) reproduces exactly the predicted, in-scope-acknowledged fallout: 9× `error[E0433]: cannot find module or crate semio_s_3d`/`semio_s_2d` in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🖍️drawing/🦀️component.rs` and `.../📐️brep-geometry/🦀️component.rs` — both files outside my ownership. This is the exact, expected consequence of the literal rename described in step 7 above, not a mistake.

### Summary of what still fails and why (per step 8 instructions)
| Crate | Cause | In my ownership? | Pre-existing? |
|---|---|---|---|
| `semio-framework-os-kernel-db` | missing `📄️document/🦀️component.rs` (unrelated module move by another agent) | No | No (new churn, absent from baseline) |
| `semio-compose-rs` | unresolved `dsl`/`vcs` crates in `compose/client/lib/rs/lib.rs` | No | Yes (verbatim in baseline-cargo-check.txt) |
| `semio-s-plugin-stdio` | `dsl::DslField`/`DslDiff`/`OpText` derive-macro breakage in DXF/XML artifact schema | No | No (new churn, absent from baseline) |
| `semio-framework-os-flow`, `semio-framework-os` (host), ui-wgpu Shell component, `semio-s-plugin-cad`, `semio-s-plugin-draw` (artifact engine), `semio-s-plugin-process`, `semio-s-plugin-demonstrator`, `semio-s-plugin-procedural`, `semio-s-plugin-lowpoly` | `use semio_s_3d::`/`use semio_s_2d::` now unresolved after the crate rename — their `.rs` source is outside my file ownership (only their Cargo.toml was mine) | Only Cargo.toml, not the `.rs` source | Direct, expected consequence of this ticket's rename; needs a follow-up pass (or a different owning agent) to bulk-rename `semio_s_3d::` → `semio_framework_3d::` and `semio_s_2d::` → `semio_framework_2d::` in those files |

## Files touched (created/moved/edited)
- `mv`: `✏️s/🔨️modules/🧊️3d` → `🧰️framework/🔨️modules/🧊️3d` (whole subtree, unchanged file contents except those listed below)
- `mv`: `✏️s/🔨️modules/◻2d` → `🧰️framework/🔨️modules/◻2d` (whole subtree, unchanged file contents except those listed below)
- Edited: `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml`
- Edited: `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📋️project.json`
- Edited: `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📜️script.ts`
- Edited: `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/benches/kernel.rs`
- Edited: `🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/Cargo.toml`
- Edited: `🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/📋️project.json`
- Edited: `🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/📜️script.ts`
- Edited: `Cargo.toml` (root — members + workspace.dependencies)
- Edited: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml`
- Edited: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`
- Edited: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
- Edited: `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`
- Edited: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml`
- Edited: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/Cargo.toml`
- Edited: `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml`
- Edited: `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml`
- Edited: `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml`
- Edited: `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`
- Edited: `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml`
- Edited: `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml`
- Edited: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` (the one `#[path]` mount only)

Not touched (explicitly out of ownership, listed here so the next agent knows exactly what's left): the `.rs` source files enumerated in the table above that still say `use semio_s_3d::` / `use semio_s_2d::`.
