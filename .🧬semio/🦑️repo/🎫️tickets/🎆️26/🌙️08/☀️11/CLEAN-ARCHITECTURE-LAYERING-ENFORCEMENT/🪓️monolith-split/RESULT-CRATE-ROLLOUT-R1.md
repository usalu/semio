# Result: CRATE-ROLLOUT-R1 (prefix `r1`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r1-tools/`. Logs: `F/🗑️generated/r1-*`. Started 2026-10-17 08:39.

## 0. Verdict (DONE for all four, see section 7)

| Package | Verdict | Notes |
| --- | --- | --- |
| deflate | DONE (static) | 6 crates plus facade; every crate inside the gate for lib and test; `--check-applied` exit 0, 0 extras |
| xml | DONE (static) | 8 crates plus facade; 3 source moves broke the snapshot/diff/valid cycle; testkit gating of demo helpers; `--check-applied` exit 0, 0 extras; all crates inside the gate |
| json | DONE (static) | 6 crates plus facade; i-json and geojson conformance moved below the SQLite snapshot guard; `--check-applied` exit 0, 0 extras; all crates inside the gate |
| zip | DONE (static) | 7 crates plus facade; one generator bug fixed with a test; `--check-applied` exit 0, 0 extras; all crates inside the gate |

Order: deflate, xml, json, zip (dependency order: zip depends on deflate and xml; xml and json depend on txt, contract; deflate depends on binary, contract; those leaves are already inside the gate: txt 1/7,050/162, binary 1/7,690/167, contract 1/6,431/57).

## 1. Baseline census (cgc7, host cfg)

| crate | lib files/bytes/lines | worst test closure |
| --- | --- | --- |
| semio-s-artifact-stdio-deflate | 74 / 352,521 / 5,701 | 97 / 479,029 / 6,903 |
| semio-s-artifact-stdio-zip | 109 / 503,942 / 7,746 | 155 / 744,124 / 10,436 |
| semio-s-artifact-stdio-xml | 122 / 636,835 / 9,359 | 171 / 902,700 / 11,839 |
| semio-s-artifact-stdio-json | 116 / 568,082 / 8,135 | 155 / 848,071 / 10,496 |

Scoped census tool: `F/r1-tools/r1-census.ts` (a copy of `cgc7-census.ts` that measures only the crates under `SCOPE=<dir,...>` and writes `r1-census-<TAG>.tsv`); step runner `F/r1-tools/step.sh <tag> <cmd...>` (niced, `ulimit -n 65536`, log and exit code to `🗑️generated/r1-<tag>.{log,exit}`).

## 2. Per package

### 2.1 deflate (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate`, 08:40 to 09:25)

- Precondition: the artifact had staged edits of earlier waves (against HEAD) and one unstaged `📜️script.ts` edit of a peer (`runArtifactRustTests(..., this.invocation)`); no overlapping in-flight edit at plan time. During my verify run a peer slice (`c4p34`, zlib file split) installed 9 children under `🚪️io/🧮️zlib/` (08:43, before my apply at 08:51). The apply re-planned on that tree, so the zlib crate is 12 files instead of 3. c4p34 reports DONE for that file.
- `new crates <pkg> --dry-run --explain` (log `r1-deflate-plan0.log`): plans 6 crates, no problem lines (the survey of RESULT-PLUGIN-WAVE0-GENERATORS §16.5 that called deflate blocked is obsolete).
- `new crates <pkg> --verify --oracle --sandbox=...` (`r1-deflate-verify.log`, exit 0): census all within the hard budget; module trees equal, visibility 0, orphan 0, glue 0, unresolved crates 0; oracle native 593 vs 593, `--test` 911 vs 894 (`onlyCandidate` 0); layout `onlyCombined` 0 / 0.
- Apply with `--backup=F/🗑️generated/r1-backup-deflate` (`r1-deflate-apply.log`, exit 0): 50 files, 6 crates, 6 owner rows.
- `--check-applied` (base HEAD) first run (`r1-deflate-check.log`, exit 1): native 0 extras, test 2 extras: `E0425 cannot find value HASH_SIZE / WINDOW` in `🚪️io/🧪️tests/🔬️codec/🦀️.rs:21`. Cause: the test used `super::*` of the zlib module; c4p34 had moved the two constants into the private child `lz77_matching` as `pub(super)` (reached in zlib through a private `use lz77_matching::*`), and the generator's test detach only widens items that are defined in the host file.
- Source fix (minimal, in the rightful owner): `🧮️zlib/🔎️lz77-matching/🦀️.rs` `pub(super) const WINDOW` and `HASH_SIZE` became `pub const`; `🧮️zlib/🦀️.rs` gained `pub use lz77_matching::{HASH_SIZE, WINDOW};` next to the private glob.
- `--check-applied` second run (`r1-deflate-check2.log`, exit 0): native base 593 / combined 593 / extra 0; test 911 / 894 / extra 0; `admission`, `shared`, `sharedForeign` empty; `healedBase` = the artifact root (the base has the old `use` form that does not parse).
- Repo checks (`r1-deflate-checks.log`): both launch JSONs parse; owners scan 2588 packages / 2721 owners, no stdio row in `missing` (`missing` lists only framework 3d, wgpu and plugin rows of peers); taxonomy (`pw0-taxcheck.ts` over stdio) mounts 0, tree 1 (the old `gif` sqlite finding); `lib-wiring-budget`: the deflate root is now 140 lines, so its ratchet pin (416) was removed with `r1-tools/unpin.py`; the remaining failure of that test lists two foreign roots that grew (`📸️jpg` 527 to 551, energy model schema 165 to 214).
- Census after (`r1-census-deflate-after.tsv`, `r1-tools/r1-census.ts`, host cfg, lib / test cfg, files/bytes/lines):

| crate | lib | test |
| --- | --- | --- |
| semio-s-artifact-stdio-deflate (facade) | 1/6,381/137 | 4/61,769/365 |
| semio-s-artifact-stdio-deflate-examples | 14/24,113/374 | 15/24,284/391 |
| semio-s-artifact-stdio-deflate-zlib | 12/111,106/2,635 | 12/111,106/2,635 |
| semio-s-artifact-stdio-deflate-schema | 24/56,972/288 | 29/89,514/699 |
| semio-s-artifact-stdio-deflate-snapshot | 12/65,210/567 | 12/65,210/567 |
| semio-s-artifact-stdio-deflate-mutations | 25/98,830/1,611 | 28/103,803/1,729 |
| semio-s-artifact-stdio-deflate-tests | 6/37,131/695 | 14/67,865/1,165 |
| semio-s-artifact-stdio-deflate-test-oracle (unchanged) | 2/11,853/256 | 2/11,853/256 |

Before: facade 74/352,521/5,701 lib, 97/479,029/6,903 test. After: no crate is over any limit (largest 25 files, 111 KB, 2,635 lines; schema test 29 files, mutations test 28 files, both under 32).

### 2.2 xml (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml`, 09:20 to 09:55)

- Precondition: the artifact had only staged edits of earlier waves; no peer edit in flight (`r1-xml-status-before.txt`).
- First plan (`r1-xml-plan0.log`, exit 1): `problem: crate ...-xml-schema exceeds the hard budget: lib 74/478,792/6,725`. `--explain` showed one cycle of 14 groups (99 files, 529,871 bytes, 6,882 lines). Only two upward edges tie the snapshot group to the groups above it; all other edges point downward:
  1. `io::binary::snapshot` and `io::text::snapshot` used the node codec helpers (`enc_xml_node_bin`, `write_str_lp`, ..., `enc_xml_node`, `enc_str`, ...) that lived in the binary and text diff codecs, so snapshot depended on diff and diff on snapshot.
  2. `io::sqlite::snapshot` (`validate_sqlite_snapshot_subset`, subset `valid`) called `valid::io::check_valid_conformance_controlled`, which dragged the valid io, the base io composer and the valid mutations into the snapshot cycle (`valid::io` and `base::io` call each other).
- Source changes (minimal, each in the rightful owner, no re-export, originals in `F/r1-tools/orig-xml/`):
  1. `🧱️base/🚪️io/💾️binary/🔺️diff/🦀️.rs` loses 182 lines (`enc_prolog_bin` .. `dec_xml_node_bin`, 14 functions); `🚪️io/💾️binary/📸️snapshot/🦀️.rs` gains them as the file module `node_wire_codec` (`pub use node_wire_codec::*`), the wire codec imports them from `super::node_wire_codec`, the diff codec imports the 10 it uses from `binary::snapshot`.
  2. The same for the text codecs: `🚪️io/📝️text/🔺️diff/🦀️.rs` loses `hex_encode` .. `dec_xml_node` (21 functions); `🚪️io/📝️text/📸️snapshot/🦀️.rs` gains the module `node_text_codec`; the diff imports what it uses from `text::snapshot`.
  3. `✅️valid/🧬️schema/🏅️conformance/🦀️.rs`: `pub(crate) fn check_conformance` became `pub fn` (removes the visibility coupling that merged the conformance slice with the valid io slice), and it gained `pub fn check_valid_conformance_controlled` moved from `✅️valid/🚪️io/🦀️.rs` (8 lines removed there, nothing else used it); `🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` now calls `valid::schema::conformance::check_valid_conformance_controlled`.
- Second plan (`r1-xml-plan2.log`, exit 0, no problem): 8 crates plus facade. `--verify --oracle` (`r1-xml-verify.log`, exit 0): module trees 0 problems, visibility 0, orphan 0, glue 0, unresolved crates 0, oracle native 1151 = 1151 and `--test` 1706 vs 1712 (`onlyCandidate` 0), layout `onlyCombined` 0 / 0.
- Apply with backup `F/🗑️generated/r1-backup-xml` (80 files, 8 owner rows). `--check-applied` (base HEAD, `r1-xml-check.log`): **exit 0**, native base 1151 / combined 1151 / extra 0, test 1706 / 1712 / extra 0, `healedBase` empty, `admission`, `shared`, `sharedForeign` empty.
- Repo checks (`r1-xml-checks.log`): launch JSONs parse; owners scan has no xml row in `missing`; taxonomy mounts 0, tree 1 (old gif finding); `lib-wiring-budget`: the artifact root went from 544 to 194 lines, which is still above the 150-line budget, so the ratchet pin was lowered from 544 to 194 (`r1-tools/unpin.py` then `repin.py`; I first removed it by mistake and restored it at 194 right after); the remaining test failures list foreign roots (`📸️jpg` grew 527 to 600, `🧿️semio/.../🕸️graph` now complies but is still pinned).
- Census after (`r1-census-xml-after.tsv`):

| crate | lib | test |
| --- | --- | --- |
| semio-s-artifact-stdio-xml (facade) | 1/8,597/192 | 2/61,880/533 |
| ...-xml-snapshot | 22/215,306/3,587 | 23/218,128/3,634 |
| ...-xml-schema | 26/186,526/1,514 | 28/205,642/1,863 |
| ...-xml-inferences | 28/36,052/486 | 29/36,255/494 |
| ...-xml-mutations | 23/62,425/1,184 | 30/79,763/1,536 |
| ...-xml-schema-2 (valid) | 23/90,224/1,583 | 29/104,769/1,850 |
| ...-xml-tests | 8/9,649/163 | 15/11,909/223 |
| ...-xml-tests-2 | 4/79,321/1,386 | 8/122,806/2,022 |
| ...-xml-tests-3 | 1/2,599/52 | 14/116,699/587 |
| ...-xml-test-oracle and quick-xml-oracle-codec (unchanged) | 3/69,280/1,358 and 1/32,579/694 | 4/74,299/1,454 and - |

Before: facade 122/636,835/9,359 lib, 171/902,700/11,839 test. After: no crate over any limit (largest bytes 218,128 for the snapshot test closure, 90% of the 262,144 limit; largest files 30; largest lines 3,634).

### 2.3 json (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json`, 09:55 to 10:30)

- Precondition: only staged edits of earlier waves plus a peer's unstaged `📦️packages/🦀️rust/📜️script.ts` (`runArtifactRustTests(..., this.invocation)`), which the apply kept (`r1-json-status-before.txt`).
- First plan (`r1-🧾️json-plan0.log`, exit 1): `problem: crate ...-json-schema exceeds the hard budget: lib 47/333,060/4,983`. `--explain`: one cycle of 14 groups (67 files, 340,554 bytes, 5,319 lines). The upward edges out of the base snapshot group were all in the SQLite snapshot guard (`validate_sqlite_snapshot_subset` dispatching on the subset):
  1. `i-json`: `i_json::io::check_i_json_conformance_controlled` (the conformance engine sat inside `i-json/🚪️io/🦀️.rs`, 90 lines of the `derived_analysis` module);
  2. `geojson`: `geojson::io::sqlite::snapshot::check_geojson_conformance_controlled` (an adapter file in the geojson io) and `geojson::schema::CODE_*` in the receiving guard.
- Source changes (minimal, originals in `F/r1-tools/orig-json/` with `INDEX.txt`):
  1. New file `🛜️i-json/🧬️schema/🛡️conformance/🦀️.rs` (docstring emoji 🪪️) holds the I-JSON conformance region moved out of `🛜️i-json/🚪️io/🦀️.rs` (`CODE_*` constants, `check_i_json_conformance`, `check_i_json_conformance_controlled` and their helpers); mounted in `🛜️i-json/🧬️schema/🦀️.rs` as `pub mod conformance; pub use conformance::*;`. The io module imports `i_json::schema::check_i_json_conformance` (composer, builder, analyzer) and, under `cfg(test)`, the four `CODE_*` constants its unit tests use. Callers updated: i-json editor, the demo example test, the base SQLite snapshot guard and its tests, and the receiving guard (`i_json::schema::conformance::*`). The guard calls the module path `i_json::schema::conformance::...` (not the schema root) so the base schema root stays out of the cycle.
  2. `🌍️geojson/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` (SQLite adapter `SqliteGeoJsonConformance` and `check_geojson_conformance_controlled`) moved into `🌍️geojson/🧬️schema/🛡️conformance/🦀️.rs` as the inline module `sqlite_control`, exported from the geojson schema root next to `check_geojson_conformance_with_control`; the Rust file and the `pub mod sqlite { snapshot }` mount in `🌍️geojson/🚪️io/🦀️.rs` were removed (the sibling `🟦️.ts` stays); the guard and its tests call `geojson::schema::check_geojson_conformance_controlled`.
- Second plan (`r1-json-plan2.log`, exit 0). `--verify --oracle` (`r1-json-verify.log`, exit 0): module trees 0, visibility 0, orphan 0, glue 0, unresolved crates 0, oracle native 1219 = 1219 and `--test` 1786 vs 1794 (`onlyCandidate` 0), layout `onlyCombined` 0 / 0.
- Apply with backup `F/🗑️generated/r1-backup-json` (73 files, 6 owner rows). First `--check-applied` (`r1-json-check.log`, exit 1): native 0 extras, test 2 extras, both in the SQLite snapshot test file (`E0425 check_i_json_conformance` in module `i_json::io` at line 90 and `E0433 cannot find sqlite in io` at line 160), two call sites my first replace missed (the non-`_controlled` function and the geojson adapter path). Fixed both call sites. Second `--check-applied` (`r1-json-check2.log`): **exit 0**, native base 1219 / combined 1219 / extra 0, test 1784 / 1792 / extra 0, `healedBase`, `admission`, `shared`, `sharedForeign` empty.
- Repo checks (`r1-json-checks.log`): launch JSONs parse; owners scan has no stdio row in `missing`; taxonomy mounts 0, tree 1 (old gif finding, the new `🛡️conformance` folder resolves); `lib-wiring-budget`: the json root went from 538 to 186 lines (still above 150), so the ratchet pin was lowered from 538 to 187; the only remaining failure there is the foreign `🧿️semio/.../🕸️graph/🦀️.rs` pin.
- Census after (`r1-census-json-after.tsv`):

| crate | lib | test |
| --- | --- | --- |
| semio-s-artifact-stdio-json (facade) | 1/7,973/185 | 4/60,046/496 |
| ...-json-schema (geojson schema plus snapshot, i-json conformance) | 26/222,448/2,838 | 29/229,767/3,009 |
| ...-json-schema-2 (base schema root) | 26/99,382/371 | 28/101,404/413 |
| ...-json-number | 19/14,462/275 | 20/14,633/283 |
| ...-json-mutations | 22/100,912/1,959 | 29/144,763/2,708 |
| ...-json-mutations-2 (i-json) | 23/131,473/2,202 | 28/178,811/2,866 |
| ...-json-tests | 12/68,373/1,070 | 24/118,547/1,575 |
| ...-json-test-oracle and json-rfc8259-base-serde-json-engine (unchanged) | 2/17,132/319 and 1/2,697/59 | 3/23,268/423 and - |

Before: facade 116/568,082/8,135 lib, 155/848,071/10,496 test. After: nothing over any limit (largest bytes 229,767 for the schema test closure, 88% of the limit).

### 2.4 zip (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip`, 10:35 to 11:10; c4p16 file split already installed)

- Precondition: only staged edits of earlier waves (c4p16 split, DONE per `RESULT-CENSUS4-PKG-16.md`); no peer edit in flight (`r1-zip-status-before.txt` lists only my own edits).
- First plan (`r1-🎒️zip-plan0.log`, exit 1): `problem: crate ...-zip-mutations exceeds the hard budget: lib 39/202,401/3,417` (files). `--explain`: one cycle of 8 groups (52 files); the single upward edge was `io::sqlite::snapshot -> iso21320::io::check_iso21320_conformance_controlled` (the SQLite snapshot guard calling the iso21320 subset's conformance, which lived in `🌐️iso21320/🚪️io/🦀️.rs`).
- Source changes (originals in `F/r1-tools/orig-zip/` with `INDEX.txt`):
  1. New file `🌐️iso21320/🧬️schema/🛡️conformance/🦀️.rs` (docstring emoji 🛡️) holds the `FLAG_*` constants, `VERSION_NEEDED_SOFT_CEILING`, the `CODE_*` constants, `hard`, `soft`, `check_iso21320_header`, `check_iso21320_conformance` and `check_iso21320_conformance_controlled` moved out of the io module; mounted in `🌐️iso21320/🧬️schema/🦀️.rs` as `pub mod conformance; pub use conformance::*;`. The io module keeps `check_iso21320_entry_headers` and `check_iso21320_wire_conformance` (they need the base io header inspection) and imports the schema items; its unit tests import the constants under `cfg(test)` from the schema. Callers updated: the SQLite guard (`iso21320::schema::conformance::...`), its tests, and the composition unit test.
  2. Test-visibility widening found by the first `--verify` (10 `onlyCandidate` errors in `🧱️base/🚪️io/🧪️tests/🔬️codec/🦀️.rs`): `pub(super)` to `pub` for `u16_le`, `u32_le`, `u64_le`, `EXTRA_ZIP64`, `EXTRA_UNICODE_COMMENT`, `EXTRA_UNICODE_PATH` (in `🧮️byte-primitives-and-text`) and `SIG_LOCAL`, `SIG_CENTRAL`, `SIG_EOCD`, `SIG_DATA_DESCRIPTOR` (in `🔍️inspect-decode-and-sniff`); the io root already glob-re-exports both children with `pub use`.
  3. `🌐️iso21320/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `use crate::apply_mutation;` became `use semio_s_artifact_stdio_contract::apply_mutation;` (the item is the contract crate's; the root alias bound the test to the root).
- Generator bug (found by the layout differential, 7 `onlyCombined` errors, `cannot find type ZipIso21320Mutation` in the iso mutation test): when a detached test is placed in a crate other than the crate of its host module, the mirror emitted a local inline wrapper `pub mod mutations { pub mod tests; }` next to the component's `pub use <owner>::...::mutations;`; the local module shadows the re-export, so the test's `use ...::mutations::*` found nothing (a real build error, not only a harness artifact). Cause: `planArtifactCrates` skipped the import of the host module when the crate owned a prefix of its path (`attachPaths ... startsWith`). Fix in `L/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts`: for every detached test slice attached to a crate, the host module of another crate becomes a mirror import unless the crate owns exactly that path, so the wrapper gets `pub use <owner>::<host path>::*;`. Test added: `glob-exports the owner of a module into the wrapper that hosts its detached test in another crate, so the wrapper does not shadow the module` in `L/🧪️tests/📦️crate-tree/🟦️.ts` (fails with the old planner, passes now); whole file: 65 pass, 0 fail. Original planner copy: `F/r1-tools/orig-plan.ts.txt`.
- Second plan exit 0; `--verify --oracle` (`r1-zip-verify4.log`, exit 0): module trees 0, visibility 0, orphan 0, glue 0, unresolved 0, oracle native 1010 = 1010 and `--test` 1478 vs 1483 (`onlyCandidate` 0), layout `onlyCombined` 0 / 0.
- Apply with backup `F/🗑️generated/r1-backup-zip` (62 files, 7 owner rows). `--check-applied` (`r1-zip-check.log`): **exit 0**, native base 1012 / combined 1010 / extra 0, test 1481 / 1483 / extra 0; `healedBase`, `admission`, `shared`, `sharedForeign` empty.
- Repo checks (`r1-zip-checks.log`): launch JSONs parse; owners scan no stdio row in `missing`; taxonomy mounts 0, tree 1 (old gif finding); the zip root went from 547 to 220 lines, so its ratchet pin was lowered from 547 to 221; `lib-wiring-budget` failures that remain are foreign roots.
- Census after (`r1-census-zip-after.tsv`):

| crate | lib | test |
| --- | --- | --- |
| semio-s-artifact-stdio-zip (facade) | 1/10,043/219 | 3/49,667/447 |
| ...-zip-inferences | 25/84,793/981 | 29/91,401/1,115 |
| ...-zip-schema (iso21320) | 25/94,599/1,988 | 29/101,743/2,148 |
| ...-zip-schema-2 (base schema root) | 21/82,064/188 | 22/82,458/198 |
| ...-zip-mutations | 23/61,063/1,160 | 28/76,296/1,387 |
| ...-zip-opc | 16/138,051/2,408 | 23/194,425/3,401 |
| ...-zip-editor | 10/86,161/1,453 | 14/129,005/2,162 |
| ...-zip-tests | 1/3,811/82 | 6/21,579/415 |
| ...-zip-test-oracle (unchanged) | 3/30,958/608 | 4/34,293/671 |

Before: facade 109/503,942/7,746 lib, 155/744,124/10,436 test (the c4p16 split had made it 109 files). After: nothing over any limit (largest bytes 194,425, 74% of the limit).

## 3. cfg(test) rule (coordinator 11:30, hand-off 12:50) and kept-root-module check (12:10)

Method: a new tool `F/r1-tools/cfgtest-isolate.ts` compiles the kept combined layout of `--check-applied --keep` once per crate with `cfg(test)` true only in that crate's own files (every `cfg(...)` predicate of the other crates has `test` replaced by an unset name, and `feature = "testkit"` is switched off in every crate whose `testkit` the tested crate does not enable in its `[dev-dependencies]`, itself included) and diffs the raw rustc errors against the all-test baseline (no unlinked-cascade filter, which is blind for modules that glob-re-export unlinked crates). Positive control: a `#[cfg(test)] pub fn` in `deflate-examples` called from `deflate-schema` is reported as `E0425` (and was not reported by the filtered oracle). Second tool: the coordinator's `F/ctc-tools/ctc-scan.ts --only <35 crates>`.

Real findings and fixes (originals in `F/r1-tools/orig-testkit/`, manifests edited by `F/r1-tools/testkit.py`):

| owner crate | item | consumers | fix |
| --- | --- | --- | --- |
| deflate-mutations | `demo_mutation_cases`, `demo_diff_cases` (`pub(crate)` + `cfg(test)`) | deflate-tests, deflate-schema (hosted unit tests) | `#[cfg(feature = "testkit")] pub fn`; owner declares `[features] testkit = []` and the self dev-dependency `{ path = ".", features = ["testkit"] }` (the repo pattern of the infinite world crates); consumers enable it in `[dev-dependencies]` |
| xml-mutations | `demo_mutation_cases` and the test-only `use XmlNode` | xml-tests-3 | same |
| xml-schema | `demo_diff_cases` and the test-only `use XmlQuote` | xml-tests-3, xml-mutations | same (xml-mutations: self dev-dependency plus xml-schema with `testkit`) |
| zip-schema | `u64_le` (byte helper used by the base io codec test hosted in zip-opc) | zip-opc | same |
| deflate-examples | `Publication::applied_progress`, `Publication::commit_output` (ctc hand-off) | deflate-schema | `#[cfg(any(test, feature = "testkit"))]` as the ctc precedent says; `testkit = []`; deflate-schema enables it in `[dev-dependencies]` |

After the fixes: `cfgtest-isolate` reports no extra diagnostic for any of the 35 crates (deflate 8, xml 10, json 8, zip 9 crates); `ctc-scan.ts --only` reports 2 hits, both sanctioned (the deflate-examples methods, now feature-gated), 0 unsanctioned; `--check-applied` was re-run after the changes for all four: deflate (`r1-deflate-check3.log`), xml (`r1-xml-check3.log`), json and zip (`r1-keep-3.log`, `r1-keep-4.log`) all exit 0 with 0 extras (the generator already adds `--cfg 'feature="testkit"'` to the test run when a carved manifest declares `testkit`). json had no finding. Note: the `testkit` gating changes the census by a few lines only because the census uses default features.

Kept root file modules: the only `mod x;` items the four artifact roots keep are the generator's `hoisted_*` test modules (deflate 2, xml 1, json 1, zip 1; all `cfg(test)`), and no other file names them. `F/r1-tools/hoisted-check.ts` compares the raw rustc errors located in those hoisted test files between the pre-carve layout and the carved layout: the only extra errors are the harness noise categories (`env!("CARGO_MANIFEST_DIR")` of the runtime-fixture helper and `cannot find semio_framework_os_kernel` of an external crate unlinked in the sandbox); no unresolved `crate::` path.

## 4. Incident and corrections

- My first isolation tool resolved the repository-relative closure paths against the current directory instead of the sandbox, so it neutralized and restored real source files for a few minutes per crate (11:30 to 12:50), and a kill left 37 real files in deflate and zip with `test` replaced by `__r1_never` inside `cfg(...)`. The replacement was a pure token swap that cannot occur in the sources, so I reversed it (`__r1_never` back to `test`) in all four artifact trees and verified with `grep -rl __r1_never` (0 files in each artifact). The tool now maps every path into the sandbox and refuses any write outside the scratchpad (`guard`). If an auto-commit ran during those minutes it contains the swapped text and the next commit contains the restored text.
- Lessons for the generator work: (1) the unlinked-cascade filter hides unresolved names in modules that glob-re-export unlinked crates (the positive control); (2) the layout differential cannot see `cfg(test)` items across crates.

## 5. Generator changes (library authoring module, with tests)

1. `🗺️plan/🟦️.ts`: a detached test slice attached to a crate other than the one of its host module adds the host module to the crate's mirror imports even when the crate owns a prefix of its path (otherwise the local wrapper shadows the re-export). Test `glob-exports the owner of a module into the wrapper that hosts its detached test in another crate, so the wrapper does not shadow the module` in `L/🧪️tests/📦️crate-tree/🟦️.ts`; the whole file passes (65 tests).

## 6. Final verification

- Census after (host cfg, `r1-census-final.tsv`, `r1-tools/r1-census.ts`): 37 crates measured under the four artifacts, 0 over any limit for lib or test; the per-package tables are in 2.1 to 2.4 (numbers moved by at most 100 lines through the testkit gating: deflate-mutations test 28/103,819/1,707, xml-schema test 28/205,688/1,835, xml-mutations test 30/79,786/1,525, zip-schema test 29/101,758/2,145).
- `--check-applied` (base HEAD) final: deflate 593/593/0 and 911/894/0; xml 1151/1151/0 and 1706/1712/0; json 1219/1219/0 and 1784/1792/0; zip 1012/1010/0 and 1481/1483/0 (native/test: base/combined/extra), all exit 0, `admission`, `shared`, `sharedForeign` empty.
- Both launch JSONs parse; owners scan has no stdio row in `missing`; taxonomy mounts 0 (tree 1, the old gif finding); `lib-wiring-budget` has 1 failing test whose five findings are foreign roots (sequence, presentation, forms, brep mesh, puzzle-5d support); the four stdio pins were removed or lowered by me (deflate removed; xml 544 to 194; json 538 to 187; zip 547 to 221).
- Not run: real `bun repo cargo` check (compile admission is the coordinator's); the commands for the compile queue are the canonical form in `RESULT-CENSUS4-PKG-16.md` section 0c with `-p semio-s-artifact-stdio-{deflate,xml,json,zip}` plus their component crates, `--lib --tests`, and the artifact workspace manifests (`--features testkit` is enabled by the dev-dependencies).
- Foreign errors seen: none caused by these packages; the `lib-wiring-budget` failures above, and `git` daemon crashes (`error: daemon terminated`) that did not affect results.

## 7. Verdict

DONE (static) for all four packages. deflate: 1 facade + 6 crates; xml: 1 + 8; json: 1 + 6; zip: 1 + 7. Every produced crate is inside the gate for lib and test (census above), `--check-applied` exits 0 with 0 extras for each, no cross-crate `cfg(test)` item remains (isolation tool and ctc-scan), and the kept-root-module check found nothing.
