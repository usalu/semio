# 📏 Result WIRING-BUDGET-ROOTS (slice `wbr`, 2026-10-16)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command, no cargo (compiler admission is still closed for this fleet, so nothing was compiled). `F` = this folder. Tools (kept): `F/wbr-tools/`. Logs: `F/🗑️generated/wbr-*` (nothing deleted). No `[DEBUG]` logs were added. `F/tx3-tools/` named in the task does not exist; the taxonomy check is `F/wbr-tools/taxcheck.ts` (a copy of the frs one, which calls the repo's own `matchDirectoryKind`).

## 0. Verdict

**`lib-wiring-budget`: before 2 pass / 1 fail (three named roots), after 3 pass / 0 fail** (`wbr-wiring-before.log`, `wbr-wiring-after.log`). The budget (150) and the ratchet fixture were not touched. All three roots were slimmed by mounting cohesive children; the public names are identical.

| Root | Before (test count) | After (test count) | Pin | Children |
| --- | ---: | ---: | --- | --- |
| energy `👁️viewer/🦀️.rs` | 512 | 394 | 508 (breach: grew) | `🔖️view-command` (78 lines), `🎥️camera-gesture` (59) |
| energy `🧵️simulation-session/🦀️.rs` | 1620 | 319 | 1619 (breach: grew) | `🧮️retained-input` (719), `🧵️run-job` (611) |
| remodeling `🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs` | 208 | 16 | none (breach: over 150) | `🧫️committed-vector` (92), `⚖️vector-laws` (117) |

("Test count" is `split("\n").length` of the test, one more than `wc -l`.)

**Why the two pinned roots stop above 150.** The test has a third case, "the ratchet lists no root that complies": a pinned root that drops to 150 lines or fewer makes its ratchet entry stale and fails that case. Because the ratchet may not be touched, the two pinned roots are cut at their natural boundary and stay between 151 and their pin. When the ratchet owner is ready, the next cut is: delete the two entries from `🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` in the same change that moves the remainder (see section 5, hand-offs).

`--check-applied` for the two touched packages: see section 4.6 (the remodeling package is done, exit 0 before and after; the energy package is recorded there as well).

## 1. How the roots were found

`bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` (17 s). Baseline output (`wbr-wiring-before.log`):

- `…/🔋️energy/…/🔋️model/…/✳️any/👁️viewer/🦀️.rs: grew from 508 to 512 lines`
- `…/🔋️energy/…/🔋️model/…/✳️any/🧵️simulation-session/🦀️.rs: grew from 1619 to 1620 lines`
- `…/📸️remodel/…/📸️remodeling/…/🧬️schema/🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs: 208 lines exceed the 150-line wiring budget`

The vectors root is the `semio-s-artifact-remodel-remodeling-mut-vectors` library that `RESULT-DUPLICATE-MOUNTS-2.md` §3.11 created (it moved the one shared 207-line file into its own crate and left it as a single root).

## 2. Found state (concurrent edits)

At 17:16 a peer rewrote the facets block of the viewer root (`#[path = "…"]` and `pub mod …` joined on one line) and dropped one blank line of the session root. My snapshot was from 17:08, so `install.py` refused ("live target changed since snapshot"). I re-snapshotted, re-based the plan (line ranges moved by one in the session root), added content anchors to both plans (the `//#region` and `//#endregion` lines at every cut), re-ran every differential and installed after that. The peer's edits are untouched. The installer refuses whenever the live root differs from the snapshot, so a later peer edit would not be overwritten silently. The sha prefixes of the nine installed files are in `wbr-installed-sha.txt`.

## 3. What changed

### 3.1 Energy viewer (`E` = `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any`)

Moved out of `E/👁️viewer/🦀️.rs`:

- `🔖️view-command/🦀️.rs`: the `//#region 🔖️Command` block, i.e. `EnergyModelViewCommand` (enum, `Default`, `action_id`) and its `OpBinary` codec. It reads the root's private `ENERGY_MODEL_VIEW_TOOL_IDS` through `use super::…`.
- `🎥️camera-gesture/🦀️.rs`: the `//#region 🎥️Camera` block, i.e. `camera_emit` and `command_from_action`. Both became `pub(super)` (the root's reducer, the root's `command_from_action` trait method and the unit tests call them).

The root keeps `#[path]` mounts, `pub use view_command::EnergyModelViewCommand;` and `use camera_gesture::{camera_emit, command_from_action};`. The `FromValueDerive` / `ToValueDerive` import moved to the command child, because the splitter masks derive lists and would have kept an unused root import (caught by the warning differential and fixed with `root_drop`).

**Deliberately kept in the root:** the action id consts, the retained-command factory and reducer, the `impl ArtifactViewer` with `bounded_first_step_tool_proofs!` (its `owner_file` literal names this file), and the manifest builder. The tool-job coverage law (`⚖️laws/🎯️tool-job-coverage/🏭️factory`) resolves the owner impl, the factory impl and the tool id list from one file and compares the macro's `owner_file` with that file; `E/👁️viewer` is not in `POLICY_SPLIT_SOURCE_ROOTS`, so a split of those parts would change the law's verdict. Result of that law before and after (script `wbr-tools/tooljob.ts`, the same functions the census calls): identical failure list (988 failures, 640 proofs, all pre-existing, none caused by this change; the only difference is the source count, 16,727 to 16,731 for the four new children), see section 4.5.

### 3.2 Energy simulation session (`E/🧵️simulation-session/🦀️.rs`)

- `🧮️retained-input/🦀️.rs`: the whole `//#region 🧮️RetainedInput` (the bounded census `CaptureCensus` and the grant-wise capture `ModelCapture`).
- `🧵️run-job/🦀️.rs`: the whole `//#region 🧵️RunJob` (`EnergySimulationRunJob`, its cursor, markers, tick publication and the `InteractiveJob` impl).

The root keeps the public vocabulary: the contract consts, the stage, counter and reason enums with `energy_simulation_run_definition`, the surface payload codec, `simulation_config_for`, the extern crate aliases and the test mount. Public surface is unchanged (`pub use run_job::EnergySimulationRunJob;`). Private items other files name were widened to `pub(super)` by the same token scan the frs splits used (33 lines: `CaptureCensus`, `ModelCapture`, `EnergyRunCursor` and their fields and `new`/`step_one`/`finish`/… members) and re-exported privately in the root, so `use super::*` in `🧪️tests/🔬️unit` keeps resolving. Imports only the unit tests need are gated with `#[cfg(test)]` in the root (`Generation`, `OperationId`, `RetainedCloneGrant`, `ToolRunIdentity`, `Arc`, `EnergyModelSnapshot`, `EnergyModelCloseCursor`, `RetirementDemand`, …), the rest of the root imports moved with the code.

### 3.3 Remodeling vectors (`V` = `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️vectors`)

Hand-cut with coverage assertions (`wbr-tools/plan_vectors.py`); the root is the docstring, two `#[path]` mounts and `pub use committed_vector::Vector; pub use vector_laws::{assert_case, assert_inverse_sum};`.

- `V/🧫️committed-vector/🦀️.rs`: the `//#region 🧫️Vector` block (`Vector`, `fixture_root`, `read`, `json_of`, `refusal_level`). The six fields the laws read (`label`, `before`, `after`, `mutation`, `diff`, `absent_marker`), the five private methods (`expected_after`, `declared`, `status`, `produced`, `committed_diff`) and the two helpers are `pub(super)`; `outcome` stays private. `fixture_root` reads `CARGO_MANIFEST_DIR`, so the move changes no path.
- `V/⚖️vector-laws/🦀️.rs`: the `//#region ⚖️Laws` block (`assert_case`, `assert_inverse_sum` and the private law functions), importing the reader helpers with `use super::committed_vector::{json_of, refusal_level, Vector};`.

### 3.4 Naming and taxonomy

New folders are `<emoji>️<kebab>` with U+FE0F, every emoji already used as a folder prefix in the repo and unique among siblings (`🔖️`, `🎥️`, `🧮️`, `🧵️` under the energy roots, `🧫️`, `⚖️` under the vectors root). Each file starts with a `//!` docstring whose first character is its own emoji; no comment inside any definition was added. All `//#region` markers moved with their block (region counts balanced per file). A first draft named the camera folder `🎥️camera-emit` and the module `camera_emit`, the same as its function; it was renamed to `🎥️camera-gesture` / `camera_gesture` before installing.

## 4. Verification (all results are real runs; log files in `F/🗑️generated/`)

### 4.1 `lib-wiring-budget`

Before: 2 pass, 1 fail, breaches as in section 1 (`wbr-wiring-before.log`). After: 3 pass, 0 fail, 5 expect() calls (`wbr-wiring-after.log`).

### 4.2 Lossless line multiset (`wbr-tools/lossless.py`, `explain.py`; `wbr-lossless-final.log`, `wbr-explain-final.log`)

Stripped non-empty line multiset of the snapshot root against the union of root and children:

| Root | removed lines | added lines | explanation |
| --- | ---: | ---: | --- |
| viewer | 2 | 21 | the two widened fns (`pub(super)` twins); added: 2 docstrings, 4 mount lines, 2 `use` re-exports and 13 import lines for the two children |
| session | 40 | 66 | 33 widened twins (`pub(super)` field, fn and struct heads); removed 7 import lines of the old root (the multi-line `semio_framework_tool_run` use, three single-name uses, the job-publication use); added: 2 docstrings, 4 mount lines, 3 re-export lines, 7 `#[cfg(test)]` and the re-formed import lines of root and children |
| vectors | 14 | 28 | 13 widened twins and the old combined `mutations::{apply…, inverse…, RemodelingMutation}` import; added: 2 docstrings, 4 mount lines, 2 `pub use`, and each child's own imports |

After pairing every removed line with its `pub(super)` twin, **no body line is lost**: the only unpaired removed lines are import lines (list in `wbr-explain-final.log`).

### 4.3 Stub-layer full-resolution differential (`frs-tools` machinery, copied to `wbr-tools/`: `oracle.py`, `stubgen.py`)

Baseline sandbox = the pre-edit tree (`wbr-work/<name>/snap`), current = a copy of the live tree after install (`oracle.py <name> live`); `rustc +nightly-2026-07-20 --edition 2021 --emit=metadata`, output file in the session scratchpad and deleted at once, never in the repo. Sandbox-only patches identical on both sides: the oracle's glob and `extern crate` patches, plus the frs stub layer extended for this slice (alias crates `crate::kernel::` to the real crate name, `CARGO_MANIFEST_DIR` set, a dangling `#[cfg(test)]` removed together with its removed `extern crate`, trait stubs with associated fns; a path from a crate alias that is also a module name was left out of the stub list for the viewer). Result, four configurations each (lib, `--test`, `wasm32-unknown-unknown`, `--test` wasm):

| Crate | Baseline = current error counts (stub noise, rustc reaches type check) | Result |
| --- | --- | --- |
| vectors | 84 / 84 / 84 / 84 | IDENTICAL x4 |
| viewer | 116 / 146 / 116 / 146 | IDENTICAL x4 |
| session | 1,007 / 1,240 / 1,007 / 1,240 | IDENTICAL x4 with `--nolabel` (message and code compared, label text not). With labels two multi-line-span errors differ only in the rendered label (`base);` versus `------`), the same message and code. |

Warning differential (`--warn`, new unused-import, private-interface and unreachable-pub warnings): viewer new 0 (3 / 6 pre-existing), session new 0 (0 / 0), vectors: one `unused import … as protocol` that fires in both trees (the stub rewrite turns `use semio_framework_os_kernel as protocol;` into an alias nothing names; both children really use `protocol::`). Two earlier drafts were corrected by this differential: the viewer root kept an unused `FromValueDerive` import, and the session root kept about 40 import names now used only by the children or only by the unit tests (dropped or `#[cfg(test)]`-gated).

Negative controls (mutating the current sandbox only): vectors un-widened `before` gave E0616, un-widened `declared` gave E0624 and un-widened `json_of` gave E0603; viewer un-widened `camera_emit` gave E0603 and a missing `use super::ENERGY_MODEL_VIEW_TOOL_IDS` gave E0425; session un-widened `finish` gave E0624 (two sites in the lib configuration, four in the test configuration). An un-widened `template` field gave no difference, which shows the token scan over-approximates slightly (the safe direction).

Independent visibility audit on the installed tree (`wbr-tools/vis_audit.py`): no private definition of any child is referenced from another file of its crate (viewer clean, session clean).

Limits (as in `RESULT-FRAMEWORK-RUST-FILE-SPLITS.md` §3.2): expressions whose type derives from a stub type are error-typed and rustc skips privacy on them; trait methods on stub traits are invisible. Only a real `cargo check` closes this (section 6).

### 4.4 rustfmt, paths, taxonomy

- `wbr-tools/parsecheck.py` (`rustc -Zunpretty=normal`, which loads every out-of-line module, and `rustfmt --check`): 0 parse errors in all nine files and trees (session, viewer, vectors; `wbr-parse-path-final.log`). Not rustfmt-clean before and after for session (14 to 18 hunks) and viewer (35 to 38 hunks; the baseline here is the 17:16 tree): the extra hunks are `use`-ordering of the added mount and import lines, nothing was reformatted. vectors is rustfmt-clean before and after (0 hunks).
- `wbr-tools/pathcheck.py`: every `#[path]` target of session and vectors exists, each new file is reachable exactly once, each starts with a unique emoji docstring, regions balance, longest path 217 bytes (limit 240), largest child 719 lines (limit about 2,500). For the viewer the script lists `🧬️schema/🦀️.rs`-style targets as missing: that is its naive resolver for `#[path] pub mod x { #[path] … }` blocks, not a real miss (the same targets exist in the baseline and `rustc -Zunpretty=normal` loads them without error).
- `wbr-tools/taxcheck.ts` (`wbr-taxcheck-final.log`): 9 of 9 folders resolve (`crate-module-children` for the four energy children, `members-of-tests` for the two vectors children, which matches the existing `🧪️tests/🏛️mutate-energy-model-1/🎯️subject-role` precedent); 0 unresolved, 0 ambiguous.
- The viewer purity law (`policyViewerPurityBreaches`, forbids `.mutation(`, `Emit::mutations`, `artifact_mutations`, `::editor::` under a `👁️viewer` directory): the scan of the root and both new viewer children finds none.

### 4.5 Source-text readers

Searched tracked `ts`/`tsx`/`py`/`json`/`rs`/`toml` (outside tickets and `.vscode`): no reader names the three root files except their own Cargo `[lib] path`, the viewer's `owner_file:` literal (unchanged, and the macro stays in that file) and the per-crate rows of `📋️build-owners.json`. Walk-based readers cover children (interactivity audit tool-run scan for `🧵️simulation-session`, viewer purity). Tool-job coverage: `wbr-tools/tooljob.ts` before and after (`wbr-tooljob-before.json`, `wbr-tooljob-after.json`): the proof list (640) and the failure list (988, of which 48 name energy rows) are identical, only `sources` grew by the four new children. The owner rows of `📋️build-owners.json` are per crate and name only the package directories, so they do not change.

Ten other library tests that touch taxonomy and fixtures placement were run once after the change (`wbr-lib-tests-after.log`: 66 pass, 26 fail, 3 errors); none of the failures or logs names a path of this slice (zero mentions of the six new folder names, of `viewer`, `simulation-session` or `vectors`); they are foreign (for example `artifact-support-leaf-authority` fails on a fixture missing in the library). I could not run them on the pre-edit tree without reverting, so these are attributed by absence of any mention, not by a baseline.

### 4.6 `new crates <pkg> --check-applied --base=f856dff0813`

Command from the repository root (as in `RESULT-PLUGIN-CRATE-HYGIENE.md` §6): `nice bun ./📜️script.ts new crates <artifact>/📦️packages/🦀️rust --check-applied --base=f856dff0813 --sandbox=<dir outside the repository>`. One run takes 15 to 35 minutes at load 40 to 170; the sandbox dirs are in the session scratchpad and are removed by the tool.

| Package | Before my edit | After my edit |
| --- | --- | --- |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling` | exit 0: 47 crates, native base 3179 / combined 3180 / onlyCombined 0, test 4102 / 4098 / 0, admission and shared empty (`wbr-check-remodel-before.log`) | exit 0, identical numbers (`wbr-check-remodel-after.log`) |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model` | exit 0: 68 crates, native 3704 / 3709 / 0, test 3927 / 3926 / 0, admission and shared empty, 3 healed base files (`wbr-check-energy-before.log`) | exit 0: 68 crates, native 3707 / 3712 / 0, test 3929 / 3928 / 0, admission and shared empty (`wbr-check-energy-after.log`). Exit status and `onlyCombined` are the same as before; the base and combined diagnostic counts rose together (native +3 and +5, test +2 and +2), with `onlyCombined` 0 on both sides. `healedBase` lists the same three files before and after (the session root, the editor `✏️edit` mode root and the viewer root): the gate copies the current text of a file whose archived text differs into its base tree. I did not trace the individual added diagnostics; they appear on both sides, which is what the zero `onlyCombined` shows. |

## 5. Hand-offs and foreign findings

1. **Ratchet entries (owner of `🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json`).** The viewer and session roots stay pinned at 508 and 1619 and are now 394 and 319. Both pins can be lowered to the new counts (a pin may only shrink), or removed once the remainder is cut to 150 lines or fewer. The viewer remainder is the tool-job owner file (impl, macro, factory, reducer): cutting it needs `E/👁️viewer/🦀️.rs` added to `POLICY_SPLIT_SOURCE_ROOTS` in `🔍️discovery/📖️source-access/🟦️.ts`, as was done for the puzzle editors. The session remainder could move the surface payload and the contract helpers into a third child (about 70 lines) and the contract vocabulary into a fourth; that would take the root below 150, hence the ratchet edit.
2. The tool-job coverage law already fails for the energy editor and viewer rows before this change ("forged bounded reducer owner/file authority", "…document schema", "…factory or compiler witness" for `👁️viewer`; the same family for the editor's `EnergyModelCommandJobFactory` rows). The failure list is byte-identical before and after.
3. Both pinned roots and the viewer facets were edited by a peer at 17:16 (formatting only); concurrent edits in the energy tree are likely to continue, so run the real check below after the last peer edit.

## 6. Commands run and real-check commands for the compile queue

Run (results above): `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` (before, after), `python3 wbr-tools/{snapshot,plan_viewer,plan_session,plan_vectors,install,oracle,parsecheck,pathcheck,lossless,explain,vis_audit}.py`, `bun wbr-tools/taxcheck.ts <list>`, `bun wbr-tools/tooljob.ts` (before, after), the two `--check-applied` runs per package, one ten-file `bun test` run of the library fixtures and taxonomy tests.

Not run: any cargo build (admission closed; the sfx and frs slices recorded the same). Queue these under the fleet mutex, one launcher call each; both packages are members of their own artifact workspace:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-energy-model-viewer -p semio-s-artifact-energy-model-session --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-remodel-remodeling-mut-vectors -p semio-s-artifact-remodel-remodeling-mut-params -p semio-s-artifact-remodel-remodeling-mut-media -p semio-s-artifact-remodel-remodeling-mut-calib -p semio-s-artifact-remodel-remodeling-mut-results --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml
```

On an E0603, E0616 or E0624 in these crates widen the named item to `pub(super)`; on E0599 for a trait method add the trait import to the child that names the error; an `unused import` in the session root means a name that only the old root used (drop it).

## 7. Files

Edited:
- `E/👁️viewer/🦀️.rs` (512 to 394 lines)
- `E/🧵️simulation-session/🦀️.rs` (1620 to 319 lines)
- `V/🦀️.rs` (208 to 16 lines)

Created:
- `E/👁️viewer/🔖️view-command/🦀️.rs`
- `E/👁️viewer/🎥️camera-gesture/🦀️.rs`
- `E/🧵️simulation-session/🧮️retained-input/🦀️.rs`
- `E/🧵️simulation-session/🧵️run-job/🦀️.rs`
- `V/🧫️committed-vector/🦀️.rs`
- `V/⚖️vector-laws/🦀️.rs`
- this file and `F/wbr-tools/` (`frslib.py`, `snapshot.py`, `splitter.py` with multi-line `use` support, `plan_session.py`, `plan_viewer.py`, `plan_vectors.py`, `install.py`, `oracle.py`, `stubgen.py`, `lossless.py`, `explain.py`, `vis_audit.py`, `parsecheck.py`, `pathcheck.py`, `taxcheck.ts`, `tooljob.ts`, `rslex.py`)
