# Result: Plugin File Splits A (slice PLUGIN-FILE-SPLITS-A, prefix `pfa`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command in the shared repo, no ticket tool. Run 2026-10-16 about 12:55 to 15:20 CEST.
Folder F = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/pfa-tools/` (`pfa.py` splitter, `specs/*.py` one spec per file, `oracle.py`, `defscan.py`, `verify_installed.py`, `patch_tax.py`, `ab.py` + `abgate.ts`, `taxdirs.py`, `counts.py`, `mutate.py`). Logs and staged copies: `F/🗑️generated/pfa-*` (kept). Originals of the six files, as they were before the first write: `F/pfa-tools/orig/*.rs.txt`.

## 0. Verdict

All six files are split into cohesive children, wired with `#[path]` plus explicit re-exports, with the regions and the public paths kept. Statically verified; **nothing was compiled by cargo** (the launcher refuses compile admission, see §8).

| Check | Result |
|---|---|
| Line multiset (old vs new tree, installed files) | 0 lines missing in all six; the extra lines are only docstrings, `use super::*;`, `#[path]`/`mod`/`use child::*;` wiring, repeated region markers and, for en1993 only, 4 lines of one function extraction (§3.2) |
| Scoped patched differential, lib, `--test`, wasm32-wasip2 lib, wasm32-wasip2 `--test` | identical error and warning multisets for process, raster, vdi, en1999, en1993 (all four configs, run on the installed tree); bim: see §5.2 |
| rustfmt parse | all 39 Rust files (6 parents, 33 children) exit 0 (the original `raster` file exited 1 because of a misplaced inner attribute, §3.4; the split exits 0) |
| Paths | every `#[path]` resolves; 33 new files, longest absolute path 230 bytes (limit 240) |
| Emoji | 29 folder names, all RGI + U+FE0F, all already used as folder prefixes in the taxonomy, none repeated among siblings |
| Census (lib and test closure) | every touched crate stays within the hard budget; file counts 1 to 10 (§6) |
| `new crates <pkg> --check-applied` | **bim model: exit 0** (`onlyCombined` 0 native and test, `admission` and `shared` empty). **en1999, en1993, vdi3805, process3d: exit 1**, and so does an untouched sibling (en1990) and, for en1999, the pure HEAD tree; the A/B runs with my split reverted give identical numbers (§7) |
| Taxonomy | 28 names registered; the simulator shows 0 previously resolved folders changed and 0 ambiguities (§4) |

## 1. What was split

Largest child: 1,270 raw lines (vdi `📚️document-checks`); 23 of 33 children are under 800. Raw lines of the old file and of the new tree (parent first):

| File (plugin) | Before | Parent | Children |
|---|---:|---:|---|
| `bim` model `🧬️schema/🔺️diff/🩹️patches/🦀️.rs` | 3,751 | 27 | `📝️documentation` 711, `📚️library` 1,086, `🤝️coordination` 261, `🏢️site-and-spaces` 540, `🧱️building-elements` 1,023, `🖋️hand-written` 150 |
| `process` process3d `🔨️modules/🏠️host/🧰️owned/🦀️.rs` | 3,597 | 53 | `📬️publication-authority` 339, `🚀️store-initialization` 829, `🧮️census-copy-timeline` 776, `🧵️retained-cursors` 660, `🔓️snapshot-decode` 284, `🧬️mutation-decode` 606, `🪪️decode-owner-catalog` 94 |
| `raster` raster `🔨️modules/🏠️host/🧰️owned/🦀️.rs` | 3,306 | 54 | `🔓️owned-field-decode` 301, `♻️retirement` 144, `🚀️store-initialization` 747, `🧬️snapshot-clone` 777, `📐️bounds-census` 588, `🧩️mutation-candidate` 736 |
| `norm` vdi3805 `🧬️schema/💡️inferences/🦀️.rs` | 3,031 | 106 | `🧰️helpers` 338, `🧱️sheet-checks` 1,081, `📚️document-checks` 1,270, `📏️parts` 372 |
| `norm` en1999 `…/💡️inferences/⚖️checks/🦀️.rs` | 2,976 | 99 | `📏️parts` 725, `🧱️member-checks` 811, `🔗️connection-checks` 356, `🔥️fire-fatigue-checks` 202, `🥫️thin-wall-checks` 451, `🧾️structure-evaluation` 386 |
| `norm` en1993 `…/💡️inferences/⚖️checks/🦀️.rs` | 2,925 | 127 | `📏️parts` 801, `🧰️structure-helpers` 433, `🧱️member-checks` 618, `🔗️special-element-checks` 987 |

Children live at `<dir of the old file>/<emoji>️<kebab>/🦀️.rs`. The parent keeps the docstring, inner attributes, `extern crate` and `use` lines, the shared constants and the helpers every child needs, plus the region markers and the test mounts. Per child it has `#[path = "<folder>/🦀️.rs"] mod <snake>;` and `pub use <snake>::*;` (plain `use` when the child has no `pub` item, `pub(crate) use` never needed). Every child starts with an emoji docstring and `use super::*;`, except the two `📏️parts` children of en1999 and en1993: their items are inline `pub mod part_*` bodies that start with `use crate::*;`, so a top-level glob would be an unused import.

### 1.1 The norm seams are shared

The three norm files had one structure: shared helpers, clause parts, per-subject checks, structure evaluation. They are cut at the same seams with the same folder names where the content is the same:

| Seam | vdi3805 | en1999 | en1993 |
|---|---|---|---|
| clause parts (`pub mod part_*`, annex parameters, macro-generated parts) | `📏️parts` | `📏️parts` | `📏️parts` |
| helpers | `🧰️helpers` | stay in the parent (`q_*`, `loc`, `scale_*`) | `🧰️structure-helpers` (+ `AnnexParams` and the subject helpers stay in the parent) |
| member / sheet checks | `🧱️sheet-checks` | `🧱️member-checks` | `🧱️member-checks` |
| other element checks | `📚️document-checks` | `🔗️connection-checks`, `🔥️fire-fatigue-checks`, `🥫️thin-wall-checks` | `🔗️special-element-checks` |
| structure evaluation | in `📚️document-checks` (`evaluate`) | `🧾️structure-evaluation` | stays with `check_full_steel_structure` |

The only repeated code is the small quantity/label helper set (`q_force`, `loc`, `copy`, ...): 12 norm artifacts define their own `loc` or `copy` (`git grep` over `✏️s/🔌️plugins/📕️norm`). It sits at the top of each parent, next to the mounts, so repeated code stays close. Moving it into `semio-s-plugin-norm-compliance` would change those 12 artifacts, so it is a hand-off (§9), not part of a split.

## 2. Method

1. `pfa.py` parses the original with the lexer of `dirinf-tools/rsitems.py` (copied), turns the file into items with exact line spans (leading region markers, inner attributes and `//!` lines are not part of an item), and moves whole items by line range. A range that cuts an item is an error, a moved item with unbalanced region markers is an error, top-level `use`, `extern crate` and `mod x;` items stay in the parent.
2. Region markers: the parent keeps every original marker line in place. A child that holds items of a region repeats that region's `//#region` / `//#endregion` pair, balanced, with the original marker text (the restore convention of `RESULT-REGION-RESTORE.md`). Regions concerned: vdi `🔖️ComplianceReport`, en1999 and en1993 `🔖️ComplianceHelpers`; bim, process and raster have none. The stray closing marker `//#endregion ️ComplianceTests` of en1999 is kept as it was.
3. Wiring is one block of mounts at the position of the first moved item (so it sits inside the same region).
4. Visibility: only `pub(super)`, never wider, name-based superset (`widen.py` of the hub split): a private top-level item, named struct field, tuple-struct field or inherent method is widened when its name occurs as an identifier (format-string captures included) in any other file of the tree, in the parent, in the siblings and in the test files that are descendants. Widened: process 166 (98 fields, 51 members, 17 items), raster 164 (78, 61, 25), vdi 24 items, en1993 21 (4 fields, 17 items), en1999 6 items, bim 0 (everything was already `pub`). The lists are in `F/🗑️generated/pfa-widen-*.tsv`. Because the parent of every child is the crate root (bim: the `patches` module), `pub(super)` equals the old module-private reach, so no `private_interfaces` difference appears.
5. Independent definition scan (`defscan.py`, own regexes, no `widen.py` code): after widening, every remaining private declaration in a child (process 90, raster 30, vdi 10, en1993 5, en1999 3, bim 0) has a name that occurs in no other file. 0 leftovers.
6. Nothing else is edited in the moved text. Macros stay with the code that uses them: the `merge_slot!` macro of bim is defined in the diff parent before `mod patches`; the vdi `define_vdi_part!` macro and its 99 invocations, and the two raster field-authority macros with the catalog that instantiates them, each sit in one child. The vdi parent has an external glob import (`use semio_s_artifact_norm_vdi3805_model::*;`); its crate declares 71 top-level names and re-exports none, and none of them equals a name defined in a child, so no glob ambiguity can appear (checked by identifier intersection: empty).
7. Two language facts checked with a toy file (`rustc --emit=metadata`, output to the scratch dir): a `use Trait as _;` in the parent reaches a child through `use super::*;` (process has three such imports), and an `extern crate x as y;` at the crate root is visible as `y::…` in nested modules.

## 3. Per file notes

### 3.1 bim `🩹️patches`: a generated file

The file is generated by `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r3-f1-gen-model.ts`. Running the generator in a sandbox reproduces lines 1 to 3,608 of the file byte for byte, but not the last 143 lines (the patches of OptionGroup, DesignOption, Workset, ElementMembership, StructuralSupport, LoadCase and StructuralLoad), which were appended by hand and are not in the model. A split that the generator would overwrite would be undone, so:
- The generator's `RustPatches` region now builds one block per patch struct, assigns every struct to a group (a missing or duplicated assignment throws), and writes the parent with the mount block plus one child per group. Its output, run in a sandbox copy of the tree (nothing was written to the repo), is **byte-identical** to the five generated children and the parent of the split (`cmp` on all six files).
- The seven hand-written patches (lines 3,612 to 3,751 of the old file) form their own child `🖋️hand-written`, which the generator never writes. Regenerating no longer drops them.
- Open item: those seven patches should become part of the model; then the child disappears.

### 3.2 en1993: one function extraction

`check_full_steel_structure` was one 1,589-line function (member loop of 573 lines, then 14 independent loops over joints, fatigue, fire, ..., crane runways). One child would have been 1,610 lines. The loop section after the member loop (old lines 1,940 to 2,917) is now `check_special_elements(document, annex, params, &mut report)` in `🔗️special-element-checks`; the member loop and the report stay in `🧱️member-checks`. Nothing inside the moved text changed. The only added lines: the call, the function header, one `///` docstring and the closing brace (4 lines; the lossless check lists them as `pre-edit`). Facts that make this safe: the function has three locals (`annex`, `params`, `report`); `annex` and `params` are `Copy` (they are passed by value repeatedly in the original), and after the member loop `report` is only used through `report.push(…)` and returned at the end (`grep` over the moved range: no other use, no `return`, no `?`). `check_special_elements` is `pub(super)`. Unverified by a compiler, resolved by the differential (no new unresolved name, §5).

### 3.3 process: source-text reader

`🧪️tests/🔬️retained-laws/🦀️.rs` read the old file with `include_str!("../../🦀️.rs")` and cut the region between `enum Process3dRetainedMutationPhase` and `enum Process3dMutationDecodeState` to assert it has no `read_str_lp`. Both enums are in `🧬️mutation-decode`, in the same order, so the test now reads `include_str!("../../🧬️mutation-decode/🦀️.rs")` (the one line changed). The string `read_str_lp` occurs 0 times in the whole original, so the assertion is not weakened. Not run (cargo is gated). The test is an integration test of the host crate and does not compile against the old layout either: it uses the private `Process3dPublicationLease` through `use …host::*` and `process3d_all_retained_mutation_fixtures_for_test`, which no host source defines (pre-existing, not changed by the split).

### 3.4 raster: an unmounted file with a syntax error

`🔨️modules/🏠️host/🧰️owned/🦀️.rs` of raster is the intended lib root of `semio-s-artifact-raster-raster-host`, which the raster editor already calls (`semio_s_artifact_raster_raster_host::RasterOneItemApply`, `raster_envelope_decode_owner_bundle`, ...), but no `Cargo.toml` exists for that crate (the `owned` folder has no `📦️packages`), so nothing mounts or compiles the file. It also had two inner attributes after a `use` line (`rustfmt` refuses it, and rustc rejects inner attributes that follow an item). I moved the `use semio_framework_job_publication::…` line below the two `#![allow(…)]` lines (a reorder, 0 lines added). Everything else is as for the other files. No census, no `--check-applied` and no source-text reader apply. Hand-off: whoever creates the manifest.

### 3.5 vdi: the embedded Rust facet shrinks

`vdi3805_artifact_inference_descriptor` (kept in the parent) embeds the file itself as the Rust facet of the inference schema: `rust: include_str!("🦀️.rs")`. After the split that leaf holds the parent text (106 lines: the `Vdi3805Inference` schema struct, its `Inference`/`InferenceSpec` impls, the descriptor and the mounts) instead of the whole file; the compliance-report code moved into the children and is no longer part of that facet. The schema struct, which is what the facet documents, is unchanged. I found no test or fixture that compares the leaf text (`git grep` for the descriptor and for `check_catalog_integrity` outside Rust: none). Hand-off if a golden of the facet exists: regenerate it.

## 4. Taxonomy and the other registered facts

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: 28 names added to `semanticDirectoryMemberKinds.crate-module-children.memberNames` (sorted insertion: 28 name lines plus one changed line for a trailing comma; applied by `patch_tax.py` after asserting that the parsed JSON equals the old JSON minus the new names, hash-guarded against the peer edits the file already carries). `♻️retirement` needed no row: it already resolves as the kind `retirement`.
- Evidence (`F/🗑️generated/taxsweep/sim.ts`, the sweep's simulator, over 106,179 directories: every tracked directory of `✏️s`, `🧰️framework` and `🌎️hub` plus the new ones): before, the 32 new directories were unresolved; after, 0 unresolved, 0 ambiguous, 0 previously resolved folders changed their kind id; 4 existing folders with the same names elsewhere (`🧬️snapshot-clone` in the store, three `🤝️coordination` folders in bim) now resolve too.
- `bun ./📜️script.ts verify taxonomy report --scope <en1999 checks dir>` and `<process3d owned dir>`: `clean=true errors=0 warnings=0` (before and after the registration; a scope run does not report unresolved kinds, which is why the simulator was used).
- `validateRustTaxonomyMounts` and `validateTaxonomyTree` over the four plugin directories (`F/🗑️generated/pw0-taxcheck.ts`, log `pfa-taxcheck.log`): each plugin reports its plugin-root finding `missing Cargo manifest 📦️packages/🦀️rust/Cargo.toml` (1 mount, 1 tree finding) and bim two viewer-surface findings (`✳️any/👁️viewer is missing 🎚️config/…`); none names a folder of this slice.
- `🧰️framework/…/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json`: the lib roots of process3d host, vdi3805 inferences, en1993 checks and en1999 checks are now 54 to 128 lines (budget 150), so their 4 pins are removed (the ratchet test fails on a pin whose root complies). `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` from `L`: 2 pass, 1 fail; the single breach is a foreign root, `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧰️testing/🦀️.rs: 152 lines`; no root of this slice is listed.
- No `launch.json` entry: no new executable command. No `Cargo.toml`, `Cargo.lock` or `project.json` was touched (the `Cargo.toml` and `Cargo.lock` files of the norm, process and raster workspaces carry `pch` edits that are not mine).

## 5. Differential (static, patched, scoped)

Tool: `F/pfa-tools/oracle.py <tag>` (`PFA_LIVE=1` runs it on the installed tree). It copies the artifact directory twice into a sandbox under the scratch dir (the original text restored in A, the split in B), removes every `extern crate` line, replaces every glob import from a non-local crate with `const _: () = ();` (the two patches of `📓️RESOLUTION-ORACLE.md`), and runs
`rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name <crate> --emit=metadata -o /dev/null --error-format=short -W unused-imports [--test] [--target wasm32-wasip2] <lib root>`
from the lib root (the changed file for the five single-file crates, `🏢️model/🦀️.rs` for bim). Path, line and column are stripped and the sorted multisets of `error` and `warning` lines are compared. A missing `#[path]` file outside the artifact is pulled from the repo into the sandbox and the run repeated.

### 5.1 Results (installed tree, `F/🗑️generated/pfa-oracle-live-5.log`)

| Crate | lib | `--test` | wasip2 lib | wasip2 `--test` |
|---|---|---|---|---|
| process3d host | 478 = 478, only-new 0 | 485 = 485, 0 | 478 = 478, 0 | 485 = 485, 0 |
| raster (file as crate root) | 356 = 356, 0 | 421 = 421, 0 | 356 = 356, 0 | 421 = 421, 0 |
| vdi3805 inferences | 426 = 426, 0 | 437 = 437, 0 | 426 = 426, 0 | 437 = 437, 0 |
| en1999 checks | 39 = 39, 0 | 39 = 39, 0 | 39 = 39, 0 | 39 = 39, 0 |
| en1993 checks | 17 = 17, 0 | 46 = 46, 0 | 17 = 17, 0 | 46 = 46, 0 |

(`errors original = errors split`; every difference set, errors and warnings, is empty. The baseline errors are the external crates and attributes that cannot resolve, `E0433`/`E0432`/`cannot find attribute`, and they cancel.) The first run on en1993 found a real defect that is fixed: the `--test` mount `compliance_tests` calls `crate::psi_factors`, which was left private in `🧰️structure-helpers`; the unit-test files of the module tree are now part of the widening scan (`external=` in the spec) and the run is clean.

### 5.2 bim

Whole crate `semio-s-artifact-bim-model` (root `🏢️model/🦀️.rs`, the `3,170` `.rs` files of the artifact copied twice; the `--test` run needed one `#[path]` target outside the artifact, pulled in automatically). The sandbox of the split is the staged tree, which `cmp` shows byte-identical to the installed files. Logs `F/🗑️generated/pfa-oracle-bim.log` (lib, wasip2 lib) and `pfa-oracle-bim-test.log` (`--test`, wasip2 `--test`):

| config | errors original = split | warnings original = split | only-original / only-split |
|---|---|---|---|
| lib | 9,559 = 9,559 | 38 = 38 | 0 / 0 |
| `--test` | 21,077 = 21,077 | 50 = 50 | 0 / 0 |
| wasm32-wasip2 lib | 9,559 = 9,559 | 38 = 38 | 0 / 0 |
| wasm32-wasip2 `--test` | 21,077 = 21,077 | 50 = 50 | 0 / 0 |

(The first `--test` attempt compared two copies of the same `couldn't find file` error; that run was repeated after the harness learned to pull missing files, and the numbers above are from the repeat.) The patches module only contains `pub` items, so no widening was needed.

### 5.3 What the oracle cannot see (limits, as predicted by the oracle note)

Mutation test: removing every widened field and method from the process tree (149 `pub(super)` deletions) produced **no** new diagnostic, so `E0616`/`E0624` are not observable here (the receivers have external, unresolved types). Removing widened items does produce `E0425` (checked on en1999 `actions_digest` and on all process items). So field and method visibility rests on the name-based superset plus the independent definition scan of §2, not on rustc. The real check is the compile in §8.

## 6. Census (the gate's canonical counter, `🗑️generated/pw0-closure.ts`, files / bytes / lines)

Hard budget 32 / 262,144 / 4,000, target 28 / 230,000 / 3,500. `F/🗑️generated/pfa-census-before.log` and `-after.log`.

| Crate | lib before | lib after | test before | test after |
|---|---|---|---|---|
| semio-s-artifact-process-process3d-host | 1 / 197,851 / 3,522 | 8 / 201,662 / 3,573 | 1 / 197,851 / 3,596 | 8 / 201,662 / 3,647 |
| semio-s-artifact-norm-vdi3805-inferences | 6 / 173,041 / 3,063 | 10 / 174,711 / 3,203 | 8 / 174,126 / 3,093 | 12 / 175,796 / 3,233 |
| semio-s-artifact-norm-en1999-checks | 1 / 151,076 / 2,977 | 7 / 153,017 / 3,037 | 1 / 151,076 / 2,977 | 7 / 153,017 / 3,037 |
| semio-s-artifact-norm-en1993-checks | 1 / 145,160 / 2,924 | 5 / 147,116 / 2,969 | 2 / 176,400 / 3,616 | 6 / 178,356 / 3,661 |
| semio-s-artifact-bim-model | 1,288 (derived) | 1,294 / 6,281,904 / 88,474 | 11,106 (the figure of `RESULT-PLUGIN-WAVE0-GENERATORS.md` §5) | 11,112 / 27,128,446 / 189,337 |

All rows of the first four crates keep `ok: true` (hard budget); the `target` flags are unchanged (process lib and test and en1993 test were already above the 3,500-line target). The bim model crate was never within budget (it is not carved: 1,288 files); this split adds 6 files to it. raster has no manifest, so there is no census.

## 7. `new crates <pkg> --check-applied --sandbox=<dir>`

Command (repo root, per artifact; `F/pfa-tools/checkapplied.sh`): `bun ./📜️script.ts new crates <artifact>/📦️packages/🦀️rust --check-applied --sandbox=<dir under the scratch dir>`; exit 0 needs `onlyCombined` 0 for lib and test, `admission` empty and `shared` empty (`🎮️command/🟦️.ts`).

**Result: exit 1 for en1999, en1993, vdi3805 and process3d (and for the untouched en1990), exit 0 for the bim model.** In every case `admission` is empty and `shared` has 0 entries; only the layout differential differs (`F/🗑️generated/pfa-checkapplied-*.log`):

| Artifact | crates | native: base / combined / onlyCombined | `--test`: base / combined / onlyCombined |
|---|---:|---|---|
| en1999 | 12 | 49 / 469 / 20 | 348 / 662 / 21 |
| en1993 | 16 | 49 / 848 / 27 | 265 / 1,140 / 28 |
| vdi3805 | 14 | 372 / 616 / 12 | 788 / 1,012 / 17 |
| process3d | 20 | 16 / 2,397 / 45 | 56 / 3,078 / 46 |
| **en1990 (norm, not touched by me)** | 15 | 49 / 527 / 22 | 132 / 787 / 24 |
| **bim model (exit 0)** | 0 carved (facade only) | 5,604 / 5,624 / 0 | 15,487 / 15,344 / 0 |

The first twelve samples of each list are `E0432 unresolved import crate::En1999MembersRows` style names in `🧬️mutations/*/🔺️diff` and `🚪️io` files, `cannot find attribute` for derive helpers and (en1993) `an inner attribute is not permitted` in `🧬️mutations/🏗️general`. None of the printed samples names a file or an item of this slice (the report prints 12 of them).

Why it is red, and why it is not this slice:
1. The untouched norm artifact en1990 is red in the same way (22 / 24), and **the gate is red on the pure HEAD tree**: in the scratch repository with the working tree reset to HEAD (no peer edit, no edit of mine) en1999 gives native 49 / 469 / 20 and `--test` 348 / 662 / 21, and with only my split laid over HEAD it gives the same numbers (`F/pfa-tools/ab2.py`, `F/🗑️generated/pfa-ab2-en1999.log`). So the red result is a property of the gate on these hand-built multi-crate artifacts, not of any working-tree edit. (The concurrent `pch` edit of the workspace `exclude` lists, `git diff` of the en1999 `Cargo.toml`, is therefore not the cause either.)
2. **A/B proof, my split reverted:** `F/pfa-tools/ab.py` builds a scratch repository outside the shared one (`git init` plus one commit of `git archive HEAD` of the plugin, in the scratch dir; the shared repository's HEAD stayed `36fbe9da786`), overlays the working tree of the plugin, runs the gate with `verifyAppliedLayout` (the function behind the flag, driver `abgate.ts`), then restores my six original files and removes the child folders and runs it again:

| Artifact | with the split (native / test: base, combined, onlyCombined) | split reverted |
|---|---|---|
| en1999 | 49, 469, 20 / 348, 662, 21 | 49, 469, 20 / 348, 662, 21 |
| en1993 | 49, 848, 27 / 265, 1,140, 28 | 49, 848, 27 / 265, 1,140, 28 |
| vdi3805 | 372, 616, 12 / 788, 1,012, 17 | 372, 616, 12 / 788, 1,012, 17 |
| process3d | 16, 2,397, 45 / 56, 3,078, 46 | 16, 2,397, 45 / 56, 3,078, 46 |
| bim model | 5791, 5649, 0 / 15134, 14906, 0 | 5854, 5681, 2 / 15486, 15130, 0 (see below) |

For the four norm and process3d rows the totals match exactly (not only the `onlyCombined` column), so the split adds and removes no diagnostic in the gate's combined layout.

**bim is different and I do not claim equality there.** bim has no carved crates (only the facade), so the combined layout is the facade mounted as one module and the gate is meaningful: the gate itself, run on the real tree, exits 0 (`F/🗑️generated/pfa-checkapplied-bim.log`). The three bim runs disagree in the `base` column (5,604 in the real-tree run, 5,791 and 5,854 in the two scratch runs, native; 15,487, 15,134 and 15,486, test) although every base is the same `git archive HEAD`, so the harness output for this 3,170-file crate is not reproducible between runs (not investigated; the machine load was 35 to 47 and a rustc cut short by memory pressure would give exactly this symptom). What the runs do show: with the split `onlyCombined` is 0 native and 0 test; with the split reverted it is 2 native (`E0432 crate::editor::bim::BimCommand` in `✏️editor/🎮️commands/📊️export`, `E0432 semio_framework_ui_contract` in `✏️editor/📌️panels/🔍️properties`) and 0 test; neither involves `🩹️patches`. The bim result that carries the claim is the whole-crate differential of §5.2, which is identical in all four configurations. The scratch repository and the gate sandboxes live under the scratch dir only; the driver needed `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts` and one more file copied into the scratch tree (an `include_str!` target outside the plugin), identical in both runs.
3. A re-run for en1999 against the pre-carve base `--base=f856dff0813` (the base `pch` §6 uses) gives exactly the same numbers (native 49 / 469 / 20, `--test` 348 / 662 / 21, `admission` and `shared` empty, exit 1; `F/🗑️generated/pfa-checkapplied-base-en1999.log`), so the choice of base ref does not matter for these artifacts. The flagged names are `crate::…Rows` / `crate::…Delta` imports of the `🧬️mutations/*/🔺️diff` files and `crate::…Analyzer` / `…Composer` imports of the `🚪️io` files, i.e. crate-root paths that the gate's combined module layout does not resolve for these hand-built multi-crate artifacts.
4. A transient failure seen once: while a peer was rewriting `🔣️taxonomy.json`, one gate run died with `Invalid taxonomy schema: generatorContracts["jco-package-adapter"].inputPatterns must be unique and lexically ordered` (not my edit: my change touches `crate-module-children` only; the file is valid again, 10 patterns, sorted and unique, and the runs above were made after it).
5. Scope of the HEAD-tree proof: it was run for en1999 only (the A/B runs cover the other artifacts); the other artifacts carry the same kind of diagnostics.

## 8. Commands for the compile queue (not run: launcher refuses compile admission)

Run from `/Users/ueli/Documents/semio`, one launcher call at a time, with the fleet compile mutex. Each artifact is its own workspace, so `--manifest-path` after `--` is required; repeat `-p` for every crate of the row.

```
SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <crates> [--lib | --tests] --keep-going --message-format=short --manifest-path <artifact>/Cargo.toml
```

| Artifact (`--manifest-path`) | `-p` crates (changed crate, then its direct dependents) | Variants |
|---|---|---|
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml` | `semio-s-artifact-process-process3d-host semio-s-artifact-process-process3d-editor semio-s-artifact-process-process3d-modes semio-s-artifact-process-process3d-panels semio-s-artifact-process-process3d-wasm` | `--lib`, `--tests`, and `--lib --target wasm32-wasip2` for the host and wasm crates |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml` | `semio-s-artifact-norm-vdi3805-inferences semio-s-artifact-norm-vdi3805 semio-s-artifact-norm-vdi3805-editor semio-s-artifact-norm-vdi3805-viewer` | `--lib`, `--tests` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml` | `semio-s-artifact-norm-en1993-checks semio-s-artifact-norm-en1993-inferences semio-s-artifact-norm-en1993 semio-s-artifact-norm-en1993-editor semio-s-artifact-norm-en1993-viewer` | `--lib`, `--tests` (the `compliance_tests` mount) |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml` | `semio-s-artifact-norm-en1999-checks semio-s-artifact-norm-en1999-inferences semio-s-artifact-norm-en1999 semio-s-artifact-norm-en1999-editor semio-s-artifact-norm-en1999-viewer` | `--lib`, `--tests` |
| `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml` | `semio-s-artifact-bim-model` | `--lib`, `--tests` |
| raster | none: `semio-s-artifact-raster-raster-host` has no manifest | check when the manifest exists |

Then, once the launcher admits: `cargo test -p semio-s-artifact-process-process3d-host --test process3d_host_retained_laws` (the reader test of §3.3; expected to fail to compile for the pre-existing reason there), and `bun ./📜️script.ts new crates <artifact>/📦️packages/🦀️rust --check-applied --sandbox=<dir>` for the five artifacts that have a crate once the gate has an owner (§7).

## 9. Hand-offs and foreign findings

1. **`--check-applied` is red for the norm artifacts (including untouched en1990) and for process3d on the pure HEAD tree already** (§7): the combined-layout differential of the gate does not resolve the `crate::…` imports of these hand-built multi-crate artifacts; it needs an owner before it can serve as a pass criterion for these plugins.
2. **Generator drift (bim):** the model generator does not know the seven hand-written patches (`🖋️hand-written`); add them to the model and delete the child.
3. **Raster host manifest missing** (§3.4): create `📦️packages/🦀️rust/Cargo.toml` for `semio-s-artifact-raster-raster-host` and add it to the workspace members; then the file gets a census and a compile.
4. **Norm helpers:** `q_force`/`loc`/`copy`-style helpers are repeated per artifact (12 artifacts define `loc` or `copy`); a shared module in `semio-s-plugin-norm-compliance` would remove the repetition.
5. **Foreign `lib-wiring-budget` breach:** `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧰️testing/🦀️.rs` has 152 lines (budget 150), not pinned.
6. **Largest single functions left** after the split: `check_catalog_integrity` (vdi, 1,120 lines) and `check_member` (en1999, 736 lines); both children stay below 1,300 lines.

## 10. Files

Changed: `bim` patches parent, `process` host parent, `raster` host parent, `vdi` inferences parent, `en1999` and `en1993` checks parents (each reduced to the parent shape of §1), `process` test `🧪️tests/🔬️retained-laws/🦀️.rs` (one `include_str!` path), `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r3-f1-gen-model.ts` (patch emission), `🔣️taxonomy.json` (28 names), `🔣️.json` of the lib-wiring ratchet (4 pins removed).
New: 33 `🦀️.rs` children (list in §1), `F/pfa-tools/**`, `F/🗑️generated/pfa-*`.
