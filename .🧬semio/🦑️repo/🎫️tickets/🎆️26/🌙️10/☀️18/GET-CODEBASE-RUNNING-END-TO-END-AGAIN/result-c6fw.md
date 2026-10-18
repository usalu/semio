# Result C6FW (executor prefix `c6fw`): census 6 packages P13, P14, P15, P16 (framework 3d, actor, job)

Executor: Sonnet 5.5, foreground, no sub-agents, no cargo/nx/`bun repo`, no modifying git command. `C` = this ticket, `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c6fw-tools/` (descendant of `c6fin2-tools`; specs in `specs/`, snapshots in `orig/`, `files.tsv`). Sandboxes and stages: `/Users/ueli/.cache/c6fw/`. Logs: `F/🗑️generated/c6fw-*`. Nothing was compiled.

## 0. Verdict

All 15 files are split: every resulting file is under 1,000 lines (largest 486, largest root 471). No glob re-export exists in the result. Per-crate source gate: worst closure ratio 0.967 (`semio-framework-3d-brep-construction`, test lines 3,867 of 4,000); no crate crosses 1.0. Static evidence only.

Re-measure (files moved with the cg1 carve): job root 1,286 lines (audit said 1,281). The other 14 counts equal the audit.

| tag | file (tail under `🧰️framework/🔨️modules/`) | owning crate | before | root after | children | max child |
|---|---|---|---:|---:|---:|---:|
| ge | `🧊️3d/📐️brep/⚙️engine` | `semio-framework-3d-brep-engine` | 1,817 | 122 | 9 | 406 |
| mp | `🧊️3d/📐️brep/💡️queries/📏mass-properties` | `semio-framework-3d-brep-queries` | 1,750 | 324 | 6 | 357 |
| bl | `🧊️3d/📐️brep/🛠️operations/🎨️blend` | `semio-framework-3d-brep-construction` | 1,449 | 125 | 6 | 316 |
| bo | `🧊️3d/📐️brep/🛠️operations/🔀️boolean` | `semio-framework-3d-brep-boolean` | 2,492 | 228 | 10 | 484 |
| mm | `🧊️3d/🥽️mesh/🛠️modeling` | `semio-framework-3d` | 1,867 | 117 | 6 | 486 |
| ss | `🧊️3d/📐️brep/💡️queries/✂️intersect/🏄️surface-surface` | `semio-framework-3d-brep-intersect` | 1,138 | 113 | 4 | 297 |
| te | `🧊️3d/📐️brep/💡️queries/🧩tessellation` | `semio-framework-3d-brep-tessellation` | 1,348 | 157 | 4 | 411 |
| to | `🧊️3d/📐️brep/📸️representation/🕸️topology` | `semio-framework-3d-brep-representation` | 1,040 | 232 | 5 | 248 |
| eu | `🧊️3d/📐️brep/🛠️operations/🔺️euler` | `semio-framework-3d-brep-construction` | 1,114 | 60 | 5 | 322 |
| md | `🧊️3d/🥽️mesh/📄️document` | `semio-framework-3d-mesh-document` | 1,236 | 174 | 4 | 373 |
| ap | `🎭️actor/💼️job/📨️publication` | `semio-framework-actor-job-publication` | 1,765 | 44 | 7 | 459 |
| av | `🎭️actor/📖️vocabulary` | `semio-framework-actor-vocabulary` | 1,570 | 49 | 6 | 367 |
| ar | `🎭️actor` | `semio-framework-actor` | 1,796 | 250 | 6 | 370 |
| jw | `🧵️job/👷️worker` | `semio-framework-job-worker` | 1,285 | 104 | 6 | 353 |
| jr | `🧵️job` | `semio-framework-job` | 1,286 | 471 | 4 | 260 |

88 new child folders (`<emoji>️<kebab>/🦀️.rs`, `//!` docstring starting with the folder emoji, unique per sibling set, mounted by `#[path]` from the root). Total lines 22,953 before, 23,666 after (docstrings, `use super::*;`, wiring).

## 1. Method and decisions

* Census-5 Rust method (`modsplit.py` stage/lossless/install, `vmod.py`, `defscan.py`, `modclash.py`, `widen2.py`), with three extensions in `c6fw-tools/modsplit.py`:
  1. **No glob re-exports.** The old `pub use child::*;` is replaced by explicit lists computed from usage: `pub use child::{..}` for every `pub` item (public API unchanged), `pub(crate) use` only for `pub(crate)` names that another file of the crate names (crate scan excludes the split's own tree), plain `use child::{..}` for private names another file or an external mount (tests, sibling modules) names, and nothing for names no one uses. Names used only from `cfg(test)` mounts or items get `#[cfg(test)] use ..` (bl `edge_two_faces`, ss `steinmetz`, te `collect_loop_uv`). Private traits are imported and widened when the trait name or one of its method names is used elsewhere. Children keep `use super::*;` (glob import, not re-export; same as every earlier slice).
  2. Region repair by `pre`: `av` (3 unclosed regions) and `jr` (1) get a synthetic `//#endregion` so items can move with balanced markers; checked by `region_fix` in the lossless oracle.
  3. `rebase_super` for `ss` (the moved fns named `super::shared::..`, now `super::super::shared::..`); `nest` for `mm` (`pub(super)` method became `pub(in super::super)`).
* Impl blocks above 400 lines were cut at method boundaries into several `impl` blocks (`ge` 5 pieces, `ar` 2, via `split_impl`).
* Kept in the roots on purpose: macro invocations next to their type, inline `mod`s (`mp::oracle`, `to::history`, `bo::testkit`, `jr::testkit`, `ar::schema_metadata`), pinned `use`/`mod x;` lines, `cfg(test)` mounts, `StepContext` and its loans (`jr`, it owns the private knot and a TS law asserts `self.ledger.is_none()` there).
* `jr`: `JobPayloadOperationLedger` stays `#[doc(hidden)] pub` as cg1 left it; cg1's `testkit` module is untouched.

## 2. Oracles (all run on the live tree after install)

| check | result |
|---|---|
| `modsplit.py lossless` (15 specs) | `LOSSLESS OK`: `missing 0`; every extra line is wiring (`#[path]`, `mod`, explicit `use child::{..}`), a child docstring, `use super::*;`, a declared impl-split pair or a region marker; regions orig/new balanced (`av` 3, `jr` 1, `ar`/`ap` orphan end markers kept) |
| `vmod.py live` | `VMOD OK` for all 15: every child mounted once, docstring emoji first/equal to folder emoji/unique/different from root/not clashing with existing sibling folders, path lengths (bytes <= 240, abs chars <= 256), brace depth, rustfmt parse, include targets equal (mp 2, to 1), `#[test]` multiset equal (0 tests in these 15 roots; test files untouched) |
| `defscan.py` | 0 cross-file private leftovers in all 15 |
| `modclash.py` | OK (two child module names that clashed with an fn or import were renamed: mp `solid_measures`, ss `marching_tracer`) |
| `rgi.mjs` on the 88 folder names | `bad=0` |
| `tax.ts` taxonomy resolution | not run: the live taxonomy is invalid right now (`generatorContracts["actor-typegen"|"flow-browser-package"|"repo-entity-kinds"].inputPatterns must be unique and lexically ordered`, a peer defect) |
| typeck oracle (`cg1-tools/pos/pos.py`, families `c6fw-3d`, `c6fw-actor`, `c6fw-job`, cfgs lib+test, label `pre` before install, `post`/`post2` after) | canary (E0308, E0616, E0624) reached before and after. Non-stub rows per category equal: privacy 0/0, trait-scope 0/0 in all; 3d resolve 7/7, type 174/174 (lib) and 202/202 (test), other 2/2, total errors 5361/5361 and 7278/7278 (identical); actor resolve 0/0, type 1/1 and 19/19; job resolve 2/2, type 15/15 and 34/34, other 3/3. Only job and actor show stub-layer row renames (`Tr`/trait-name and `pack` stub shape), classified stub noise by the tool (non-stub counts equal). The oracle caught one defect, fixed before the final run: ss `super::shared` (23 new resolve rows, `rebase_super`) |
| public API | for each root the set of `pub`/`pub(crate)` names of the original equals the set reachable from the new root (as definition or `pub use`), except three `pub(crate)` names no one else names (`MOMENT_COMPONENTS`, `JobPublicationOwner`, `WorkerJobSessionInner`), now plain private imports |
| source-boundary gate (`census-own.ts`, canonical `readRustSourceClosureV1`, lib / test ratio) | ge 0.733/0.866, mp 0.746/0.934, bl 0.726/0.967, bo 0.849/0.893, mm 0.809/0.809, ss 0.610/0.741, te 0.343/0.499, to 0.494/0.612, md 0.757/0.931 (29 files, 243,942 bytes), ap 0.416/0.683, av 0.681/0.823, ar 0.465/0.813, jw 0.487/0.925, jr 0.823/0.875. The 3d closure law `🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🟦️.ts` (bytes/files/lines per entry) re-derived in Python: md 29 files, 243,942 bytes, 3,218 lines, all inside 262,144/32/4,000 |

## 3. Text readers re-pointed (`c6fw-tools/tsedit7.py`, expected counts asserted, all edited files parse with `Bun.Transpiler`)

`readFileSync(<root>.rs)` became `readModuleTreeSource(<same url>)` (import of the module-tree reader added with a relative path) where the asserted text moved; the emulator `readers4.py` checked the direct `includes`/`toContain` assertions against original, new root and tree, and the slice markers were re-measured on the tree text (lengths equal the original slices):

* actor: `🎟️retained-turn/🎟️supplier`, `📃️policy` (actor root and the vocabulary root), `🫴️receiving`, `🧪️tests/📄️checkpoint` (actor root); the two publication reads of `📄️checkpoint` now read the child `📨️publication/🌉️publication-error-and-bridge/🦀️.rs` that holds `JobOutcomeProjection`/`JobTurnBridge`.
* job: `⏱️context/📦️owner`, `♻️retirement/📄️payload` (`self.ledger.is_none()`), `👷️worker/🎟️turn/🤝️receipt`, `👷️worker/🔀️erasure`, `📬️outcome/🎟️descriptor`, `📬️outcome/📦️builder/📏️extent`, `📬️outcome/🤝️loan`.
* plugin: `🖥️host/🧵️shard/🎟️grant/🧪️tests/🎟️supplier` (actor root).
* repo laws: `⚖️laws/🎯️tool-job-coverage/🏁️run` (`actor`, `jobRuntime` via `policyReadModuleTreeSource`) and `🎛️commands/✅️verify/🎯️tool-job-lane` (`actor`, `jobRuntime`, the fixed-operation `source`). The old `✅️verify/🟦️.ts` no longer holds these reads (a peer moved them).
* Not changed on purpose: `🧪️tests/📏️close-demand` (job; fails before the moved assertion, as cg1 reported), `⏱️context/📦️owner` line 60 (its literals exist in neither the original nor the new tree), the `fem` script (no moved literal), the JSON owner lists (path lists only), `tool-machine/🧪️history-closure-policy` (synthetic repo, no real read).

## 4. Crates to compile (`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests`)

Split crates (no manifest, lock or workspace change; no new crate):
`semio-framework-3d-brep-engine`, `semio-framework-3d-brep-queries`, `semio-framework-3d-brep-construction`, `semio-framework-3d-brep-boolean`, `semio-framework-3d`, `semio-framework-3d-brep-intersect`, `semio-framework-3d-brep-tessellation`, `semio-framework-3d-brep-representation`, `semio-framework-3d-mesh-document`, `semio-framework-actor-job-publication`, `semio-framework-actor-vocabulary`, `semio-framework-actor`, `semio-framework-job-worker`, `semio-framework-job`.
Companions that reach the cut crates through `testkit`: `semio-framework-3d-brep-boolean-unit-tests`, `semio-framework-job-unit-tests`; also `semio-framework-job` with `--features testkit` and `semio-framework-3d` with `--features mesh-test-observation`.
Dependents (public paths are unchanged, one sample is enough): `semio-framework-os-store-command`, `semio-framework-plugin-reactor-turn`, `semio-framework-plugin-job`, `semio-s-artifact-trinity-jack-inferences`.
Text laws to run after the Rust window: the 15 re-pointed TS tests of section 3 and `verify interactivity tool-jobs` (self-test, p2c, fixed-operation-fixture).

## 5. Open risks

* Nothing compiled and no TS test ran (`@semio-tech/*` does not resolve without the dependencies image). The typeck oracle covers privacy, resolution and types; unused-import and dead-code warnings are not visible to it (`--cap-lints allow`). Plain private imports were derived from identifier usage; a name used only as a field or method name could leave one unused import. Look at `unused_imports` of the first cargo window.
* The repo laws behind `tool-job-lane`/`tool-job-coverage` read `actor`/`jobRuntime` as module trees now; hostile-mutation self-tests were not run. `toolJobActorProgressOverlayExact` looks for `//#region 🎨️JobProgressOverlay` in the actor root text, which already lived in the publication crate before this slice (stale before and after).
* Taxonomy resolution of the 88 folders is unverified until the taxonomy schema defect is fixed.
* `jr` root is 471 lines because `StepContext` (private knot, text-asserted) stays; `semio-framework-3d-brep-construction` test closure is at 3,867 of 4,000 lines (133 lines headroom); `md` test closure has 29 of 32 files.
* `ar`: `pub use semio_framework_actor_vocabulary::*;` and `pub use semio_framework_actor_job_publication::*;` in the actor root, and `use super::super::*;` in two existing `mm` children, are pre-existing globs, untouched.
