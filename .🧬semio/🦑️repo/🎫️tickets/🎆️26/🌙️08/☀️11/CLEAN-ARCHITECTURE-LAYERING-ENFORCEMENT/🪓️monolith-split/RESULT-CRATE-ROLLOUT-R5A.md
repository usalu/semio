# Result: CRATE-ROLLOUT-R5A (prefix `r5a`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r5a-tools/` (`step.sh <tag> <cmd...>`, `r5a-census.ts`). Logs: `F/🗑️generated/r5a-*`. Sandboxes: `/Users/ueli/.cache/r5a/`. Started 2026-10-17 08:55.

## 0. Verdict (kept current)

| Package (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<dir>`) | Verdict | Notes |
| --- | --- | --- |
| 🖋️dxf | DONE (static): 5 crates + facade inside the gate (section 3.3); verify --oracle exit 0; check-applied exit 0, 0 / 0 | depends only on txt/contract |
| 📸️jpg | DONE (static): 6 crates + facade, all inside the gate (section 3.1); verify --oracle exit 0; check-applied exit 0, 0 / 0 | depends on binary/contract |
| 🖼️tiff | DONE (static): 6 crates + facade inside the gate (section 3.2); verify --oracle exit 0; check-applied exit 0, 0 / 0 | depends on jpg, png, deflate (R1) |

| 📷️png (added by the coordinator) | DONE (static): `png-mutations` split into 3 crates (io 12, schema 25, facets 3 files), every png crate inside the gate (section 3.4); check-applied on the png facade exit 0, 0 / 0 | crate was 37 / 39 files after the c4p34 io split |

Order worked: jpg, tiff, dxf, png.

**Overall verdict: DONE (static evidence only).** All 40 crates of the four artifact directories (including the three test oracles) are inside the gate for lib and test closure (`r5a-census-final.tsv`). Nothing was compiled by cargo (the launcher refuses compile admission); the evidence is the rustc analysis-only differentials, the census and the checks listed per package. Compile queue: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-stdio-jpg -p semio-s-artifact-stdio-tiff -p semio-s-artifact-stdio-dxf -p semio-s-artifact-stdio-png --lib --tests --keep-going --message-format=short` (one call, under the compile mutex).

## 1. Baseline census (cgc7 reader, host cfg, `F/🗑️generated/r5a-census-base0.tsv`, taken 08:57 after the c4p34 file splits landed)

| crate | lib files/bytes/lines | test closure files/bytes/lines |
| --- | --- | --- |
| semio-s-artifact-stdio-jpg | 104 / 419,864 / 6,440 | 166 / 653,209 / 8,173 |
| semio-s-artifact-stdio-jpg-test-oracle | 3 / 33,784 / 672 | 5 / 520,017 / 715 (OVER bytes) |
| semio-s-artifact-stdio-dxf | 99 / 632,365 / 8,637 | 120 / 697,897 / 9,702 |
| semio-s-artifact-stdio-dxf-test-oracle | 2 / 26,782 / 518 | 4 / 40,004 / 577 |
| semio-s-artifact-stdio-tiff | 116 / 487,946 / 5,819 | 175 / 790,035 / 7,370 |
| semio-s-artifact-stdio-tiff-test-oracle | 3 / 52,657 / 890 | 12 / 17,545,228 / 1,069 (OVER bytes) |

The two oracle crates (jpg, tiff) are test-only oversized by bytes (embedded fixtures); they are not in rows 22/24/27 of the audit but are in the same artifact directories (see section 4).

## 2. Findings that drive the work

Planner baseline (`new crates <pkg> --dry-run --explain`, `F3_SLICES=1` prints the slice graph; added by me to `🗺️plan/🟦️.ts`, 1 line): none of the three plans within budget.

- jpg: one 20-group dependency ring (110 files, 5,608 lines). Upward edges: the artifact facets in `🚪️io/🦀️.rs` (builder, analyzer, composer, `io_registry`, `blank_jpg_snapshot`, `inspect_*`) name `JpgDiff`, `JpgMutation` and the baseline composer; io unit tests reach binary diff and text mutation constants; the schema root (descriptor, 21 files) is reached by `demo_jpg_snapshot`; mutation leaf codecs build `JpgMutation` while the parent registry mounts the leaves.
- dxf: the schema root embeds the whole diff module tree as the descriptor `rust` facet (29 files, 241 KB); snapshot codecs use helper functions defined in the diff codecs; the diff coherence group alone is 4,037 lines.
- tiff: io root names diff/mutation/baseline composer; sqlite snapshot codec reaches io root and the baseline conformance check; `paint_region` mutation reaches the schema root (descriptor, 21 files).

## 3. Per package

### 3.1 jpg (DONE, static evidence)

Source changes (scripts `r5a-tools/jpg-j1.py`, `jpg-j2.py`, `jpg-j5.py`, originals in `r5a-tools/orig/jpg-*`; none is a compat layer):
- The artifact facets (builder, analyzer, composer) of `🚪️io/🦀️.rs` moved to the new module `🧾️document/🚪️io/🎹️facets/🦀️.rs` (callers use `io::facets::…`); the `io_registry` table moved to `🧱️baseline/🚪️io/🦀️.rs` (it names the baseline composer, which wraps the document composer), root `declaration()` and the root `io_registry` wrapper updated.
- The cross-component conformance and publication laws of the io unit tests moved to `🎹️facets/🧪️tests/🔬️conformance/🦀️.rs` (include paths rebased by one level).
- `demo_jpg_snapshot` moved from the schema root (descriptor, 21 embedded files) to `🧬️schema/📸️snapshot/🦀️.rs` and became a plain `pub fn` (it was `#[cfg(test)] pub(crate)`, used from other modules); `demo_diff_cases` became a plain `pub fn` as well (coordinator rule: no cross-crate cfg(test) items).
- The mounts of `facets`, `binary`, `text`, `sqlite` moved out of the io file into namespace wrappers of the root nest (`pub mod facets|binary|text|sqlite { mod component; … }`); the three declaration-only root files `💾️binary/🦀️.rs`, `📝️text/🦀️.rs`, `🪶️sqlite/🦀️.rs` are mounted by those wrappers unchanged. `decoded-components` and `encode-options` are mounted from a `snapshot` wrapper instead of from the snapshot file. This removes the mount edges from the snapshot/codec crate to the diff, mutation and facet crates (a parent crate cannot mount a child that lives in a higher crate).
- Codec internals the decoded-components child reaches: `pub(super)` → `pub(crate)` in the five codec children, `pub(crate) use huffman_and_bit_io::*`; `super::super::super` paths of decoded-components/encode-options made absolute.
- Test imports: the four text-mutation direct tests import `JpgMutation`/`JpgSnapshot` explicitly (they used the leaf's private glob); the receiving tests use absolute paths so the generator can hoist them; `pub mod receiving`; the sqlite tests refer to `text::snapshot` instead of the private `owned_text` alias; the dangling doc comment left by the demo move was removed by a peer sweep.

Result: operations 20/33,567/525, inferences 24/160,789/2,222, schema 21/59,035/97, schema-2 17/66,232/1,292, mutations 26/101,572/1,909, io-registry 4/51,638/906, facade 1/10,128/204 (lib files/bytes/lines); test closures 20/28/22/29/31/12/3 files, all under 32 files, 262,144 bytes and 4,000 lines (`r5a-census-jpg1.tsv`).

Commands run: `new crates <pkg> --verify --oracle --sandbox=…` exit 0 (module trees equal, visibility/orphan/glue 0, single-crate oracle native 876/876 and test 1,377/1,334 with 0 extra, layout 0 / 0); apply with `--backup=F/🗑️generated/r5a-backup-jpg/b` (63 files, 6 owner rows); `new crates <pkg> --check-applied --base=dir:/Users/ueli/.cache/r5a/jpg-pre --sandbox=…` exit 0 (native 876/876/0, test 1,371/1,328/0; the pre-carve snapshot was rebuilt by `r5a-tools/precarve.py` + `jpg-post-apply-edits.py`, see section 4: `HEAD` lacks every uncommitted pre-carve edit of this fleet and cannot be a base). Owners scan: no jpg row missing (the only missing rows are peer scratch under `ctc-tools/selftest-work`). `pw0-taxcheck` stdio: 0 mount findings, 1 tree finding (gif, existing). Both launch JSON files parse (no launch route was added; the carve registers owner rows only). `lib-wiring-budget`: two failures, none of them jpg (svg root grew 605→615, equation 1047→1054, flow brep mesh 1212 lines, graph pin stale; all peers).


### 3.2 tiff (DONE, static evidence)

Source changes (scripts `r5a-tools/tiff-t1.py`, `tiff-t2.py`, originals in `r5a-tools/orig/tiff-*`): same facets/io_registry/root-nest moves as jpg (new `🧾️document/🚪️io/🎹️facets/🦀️.rs`, `io_registry` in `🧱️baseline/🚪️io/🦀️.rs`, wrappers for `facets|binary|text|sqlite` in the root nest, the three declaration-only root files mounted unchanged); the `NativeFieldType` trait moved from the io root to the snapshot component beside `TiffFieldType` (the sqlite snapshot codec used it upward); the baseline `check_tiff_baseline_conformance_controlled` wrapper moved from baseline io to `baseline::schema::conformance` (the sqlite snapshot codec used it upward); `demo_diff_cases` became a plain `pub fn` (cfg(test) rule); the five fixture helpers of the text mutation registry read their JSON at run time (`fixture_text`, path joined onto `CARGO_MANIFEST_DIR`, rebased by the generator) so the mutation crate test closure is 28 files instead of 33; the baseline mutation unit test imports the schema globs explicitly.

Result (lib / test closure files/bytes/lines): snapshot 29/83,463/734 both, schema 28/118,504/846 both, inferences 22/135,052/1,599 and 29/165,371/1,897, schema-2 5/10,228/199 both, mutations 26/53,053/949 and 28/92,425/1,572, mutation-regressions 13/103,677/1,744 and 25/142,663/2,322, facade 2/19,355/322 and 5/39,121/488 (`r5a-census-tiff1.tsv`); `tiff-test-oracle` is now 3/53,569/894 and 6/68,648/1,091 (inside the gate).

Commands: `--verify --oracle` exit 0 (oracle native 1,039/1,039, test 1,504/1,506 with 0 only-candidate, layout 0 / 0); apply with `--backup=F/🗑️generated/r5a-backup-tiff/b` (57 files, 6 owner rows); `--check-applied --base=dir:/Users/ueli/.cache/r5a/tiff-pre` exit 0 (native 1,039/1,039/0, test 1,504/1,506/0). Kept root file modules: only the `cfg(test)` hoisted test module; no `crate::hoisted_*` path exists (jpg the same).

### 3.3 dxf (DONE, static evidence)

Source changes (script `r5a-tools/dxf-d1.py`, originals `r5a-tools/orig/dxf-*`): the shared snapshot-entity text codecs (about 380 lines: strings, numbers, lists, group codes, vertices, entities, header variables, layers/styles/linetypes, blocks, tags, other tables, tables) moved from `📝️text/🔺️diff/🦀️.rs` into the new child `📝️text/📸️snapshot/🏷️entity-codec/🦀️.rs` (`pub fn`), because the text snapshot codec used them upward; the callers in the two mutation registries and the text snapshot import them from there; the dead import-only `mod diff_codec` of the binary snapshot and the `schema::diff` glob of the text snapshot test re-export were dropped; the inline `mod tests` of the sqlite snapshot (it needs the root-bound `register_sqlite_test_declaration`) moved to the mounted file `🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` with explicit absolute imports (so the generator hoists it to the facade); `demo_diff_cases` and `demo_mutation_cases` became plain `pub fn` (cfg(test) rule; they merged diff, mutations and the schema unit tests into one 50-file group); the handcrafted diff-codec test imports the diff globs explicitly.

Result (lib / test files/bytes/lines): snapshot 11/138,270/2,154 both, inferences 24/42,453/695 and 29/56,814/922, schema 29/242,139/193 both (the descriptor embeds the diff module tree as its `rust` facet, hence 29 files and 242 KB; it is below the 262,144-byte limit but has no headroom), diff 21/208,233/3,693 both, mutations 26/89,060/1,480 and 28/90,629/1,521, tests 7/39,718/788 and 13/87,993/1,526, facade 1/14,838/266 and 3/38,753/418 (`r5a-census-dxf1.tsv`); `dxf-test-oracle` 2/26,782/518 and 4/40,004/577.

Commands: `--verify --oracle` exit 0 (oracle native 1,153/1,153 and 0 extra, layout native 0, test 0); apply with `--backup=F/🗑️generated/r5a-backup-dxf/b` (47 files, 6 owner rows); `--check-applied --base=dir:/Users/ueli/.cache/r5a/dxf-pre` exit 0 (native 1,153/1,154/0, test 645/1,354/0; the base test side reports fewer raw diagnostics than the combined side, the comparison is on the normalized set and has 0 only-combined). The facade keeps only `cfg(test)` root modules (`original_owner_tests`, `hoisted_snapshot_0`); no `crate::` path refers to them.

### 3.4 png (DONE, static evidence)

`semio-s-artifact-stdio-png-mutations` (a crate carved earlier whose lib root is a mirror root; it holds the mutation family and the png io codec root) was re-carved by running `new crates` on its own package directory (the planner takes the mirror root as the artifact root). Source changes (script `r5a-tools/png-p1.py`): facets, `io_registry` and `artifact_items_5` of the io root moved to `✳️any/🚪️io/🎹️facets/🦀️.rs` (callers: editor, facade root, inferences-tests mirror, mutations root updated); `binary`/`text`/`facets` mounted by namespace wrappers of the crate root nest; the io root and native-encode import the snapshot and operations items from the `png-snapshot`/`png-diff` crates directly (`semio_s_artifact_stdio_png_snapshot::…`, `semio_s_artifact_stdio_png_diff::…`) instead of through the crate's own `schema` wrapper, which made the codec depend on the mutation group.

Result: png-mutations is now a 1-file facade (2,322 bytes) over `png-mutations-io` 12/71,995/1,376, `png-mutations-schema` 25/43,863/700 (test 27 files) and `png-mutations-facets` 3/12,565/262; every other png crate unchanged and inside the gate. `--verify --oracle` on the mutations package exit 0 (oracle 362/362 and 403/403, layout 0 / 0); apply with `--backup=F/🗑️generated/r5a-backup-png/b` (12 files, 3 owner rows); `--check-applied` on the png facade package with `--base=dir:/Users/ueli/.cache/r5a/png-pre` exit 0 (9 crates, 6 already carved in the base: native 877/877/0, test 1,218/1,218/0). (Run on the mutations package itself the check sees 0 crates and proves little.)

### 3.5 cfg(test) rule (coordinator message 11:50)

Cross-crate `#[cfg(test)]` items found by `r5a-tools/cfgtest-cross.ts` (cfg(test) pub/pub(crate) item whose name is used by a file of another planned crate) and removed: jpg `demo_diff_cases` (and `demo_jpg_snapshot`, moved to the snapshot component), tiff `demo_diff_cases`, dxf `demo_diff_cases` and `demo_mutation_cases`: each is now a plain `pub fn` (a pure demo-data function, no `cfg(any(test,…))`, no feature); the script reports 0 remaining for jpg, tiff, dxf and png. Same-crate cfg(test) helpers stay (the text-mutation `fixture_text`/`*_test_case` of tiff, dxf `register_sqlite_test_declaration`, which lives at the facade root together with its hoisted users). Caveat: the script finds items by a name match on `#[cfg(test)] pub…` declarations only.

### 3.6 Oracle blind spot (brief 12:10)

Kept root file modules after the four carves: jpg `hoisted_receiving_0`, `hoisted_snapshot_1`; tiff `hoisted_snapshot_0`; dxf `original_owner_tests`, `hoisted_snapshot_0`; png `hoisted_snapshot_0`, `hoisted_tests_1` (all `cfg(test)` hoisted test modules); `git grep 'crate::hoisted_|crate::original_owner_tests'` finds no path into any of them, so no `crate::<kept-root-mod>::<item>` path can point at a moved item.

## 4. Generator changes (all in `L/🏗️authoring/📦️crate-tree/`)

- `📝️render/🟦️.ts` `rewriteMountPaths(…, widenPrivate)`: a file module that a carve cuts out of its parent and mounts at the top of a crate's mirror root is now declared `pub mod` (it was copied as `mod` and the parent crate's `pub use <crate>::…::m;` failed with E0603; found by the layout differential on jpg `transforms_and_tables`).
- `🗺️plan/🟦️.ts`: a crate that hosts moved root items but whose attach points are all inline wrappers (mutation aggregate wrapper with `mod top_level; pub use top_level::*;`) now hosts them in its first file member and imports them through the glob parent path (was "owns the moved root items but mounts no file"). `F3_SLICES=1` prints the slice graph of a blocker.
- `🧪️test-closure/🟦️.ts` aggregate (T3): the carried imports go after the inner doc comments of a leaf test (`prependAfterInner`); before, a leaf test starting with `//!` produced E0753 in the aggregated file.
- `🧫️sandbox/🟦️.ts` `verifyAppliedLayout`: `--base=dir:<snapshot>` takes the pre-carve tree from a directory (`<snapshot>/<artifact dir>`) instead of a git ref, because the fleet carves trees with uncommitted pre-carve edits.


## 5. Open items / hand-offs

1. `--base=dir:` is needed whenever the pre-carve tree has uncommitted edits (always, in this fleet); `r5a-tools/precarve.py` rebuilds that tree from the working tree, the apply log and the `--backup` directory, `jpg-post-apply-edits.py` replays the later source edits onto it (jpg only). A peer may want to make `precarve` part of the CLI (`--check-applied --backup=<dir>`).
2. The planner heuristics `usedBelow`/`referencedFromOutside` make a parent crate depend on any child module whose name occurs in a sibling; the root-nest wrappers (`pub mod binary { mod component; pub use component::*; }`) are the workaround used here. A generator pass that treats declaration-only file modules (`binary/🦀️.rs`, `text/🦀️.rs`, `sqlite/🦀️.rs`) as namespaces would remove the manual step for the other stdio artifacts.
3. Not covered by a generator test: the hosting-anchor fallback in `🗺️plan/🟦️.ts` (inline-wrapper attach points, found by dxf/jpg) is exercised only by the three live carves; `lib-wiring-budget` has 2 failing tests caused by peers (svg, equation, flow brep mesh roots, stale graph pin), none by these four artifacts.
4. dxf-schema has 242,139 of 262,144 bytes (the descriptor embeds the diff module tree as its `rust` facet); a later growth of the diff tree needs the S3 channel (`new leaves --json-schema`/served leaf documents, not applied because no host installs a `SchemaLeafReader`) or a smaller descriptor facet.
5. The dxf `--test` base side reports far fewer raw diagnostics than the combined side (645 vs 1,354); the comparison is on the normalized set and has 0 only-combined, but the base test analysis is cut short by an early error, so the dxf test layout evidence is weaker than the native one.
6. Tools: `F/r5a-tools/` (`step.sh`, `plan.sh`, `ring.py`, `verify-samples.py`, `precarve.py`, `cfgtest-cross.ts`, `units.ts`, `crate-files.ts`, `testonly.ts`, `reach.ts`, `r5a-census.ts`, per-package edit scripts `jpg-j*.py`, `tiff-t*.py`, `dxf-d1.py`, `png-p1.py`, originals in `orig/`). Logs `F/🗑️generated/r5a-*`; sandboxes in `/Users/ueli/.cache/r5a/`. Generator tests: `cd L && bun test ./🧪️tests/📦️crate-tree/🟦️.ts` 73 pass (3 new), `🧬️mutation-leaf-generics` 9 pass; `tsc --noEmit` over the five touched TypeScript files: 0 errors in `crate-tree` (`r5a-tools/tsconfig.json`).
