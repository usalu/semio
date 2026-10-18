# Result CENSUS5-A (prefix `c5a`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Tools: `F/c5a-tools/` (descendants of `c4p47-tools` and `pos-tools`). Logs: `F/🗑️generated/c5a-*` (nothing deleted). Sandboxes and stage trees: `/Users/ueli/.cache/c5a/`.

## 0. Verdict

**DONE for packages 1, 2 and 3** (static evidence; the real `bun repo cargo` check is queued, no `COMPILE ADMITTED` line in `📓️FLEET.md`, checked 23:3x). Last update 2026-10-17 23:4x.

- Package 1 (6 files, 9,780 lines), package 2 (5 files, 9,528 lines) and package 3 (5 files, 9,197 lines): all 16 files are split into 119 cohesive children in 119 new folders. 11 flat test files (packages 1 and 2) became `include!` fragments (84 children, one `include!` line per child in the root, test paths unchanged); 5 plugin lib roots (package 3) became `#[path]` child modules with `pub use child::*` (35 children). The census listed 16 files, not 17. 27,144 lines became 27,662 lines (wiring, one docstring per child, repeated region markers); roots 12 to 132 lines (test roots) and 62 to 125 lines (lib roots); largest child 966 lines (`⚛️reactor/🔄️turn/🌀️poll-kernel-turn`, one 962-line function that cannot be cut without rewriting it), all other children at most 516 lines.
- Lossless: line multiset and, for the test roots, the spliced sequence are exact (`gone=0`, `extra(non-marker)=0`, 11 of 11); `#[test]` fn multiset identical (330 tests: 23, 47, 0, 39, 27, 36, 17, 16, 34, 29, 62 per file in the order of the table); the module splits are lossless modulo the declared wiring, docstrings, markers, `pub(super)` widening and the four declared impl re-openings (5 of 5). Region markers are balanced in every file (markers added where a region spans a cut: ct 6, tr1 6, tt1 6, c1x 16, tt 6, tr 6, rt 2). Every child is mounted exactly once; 119 of 119 folder names RGI, used as folder prefix elsewhere in the repo, unique among siblings (existing siblings included) and resolved by the live taxonomy; longest path 208 UTF-8 bytes relative, 191 characters absolute.
- Gate: every owning crate is inside the gate (lib and test): worst ratio 0.902 (`semio-framework-plugin-tests-15`, 26 of 32 files); the plugin crates tt, tti, tr, tri, rt grew from 3 / 2 / 6 / 2 / 6 to 9 / 10 / 12 / 8 / 15 files in the lib closure (section 3.2). The full-workspace census (3,068 crates) shows all 16 crates compilable with 0 oversized dependencies (section 3.2).
- Differential: the scoped patched resolution differential (`oracle5.py`, original restored in one sandbox against the live tree in the other, lib and `--test`) is **identical** in 31 of 32 comparisons (equal error and warning multisets); the 32nd (tests-15 `--test`) differs in one error line that names an include path, with the same missing fixture target on both sides (section 3.3). The typeck-level `pos` oracle was run before and after for the five lib crates: privacy rows 8 = 8, trait-scope 0 = 0, canary reached; its resolve rows grow by 41 names that are external (glob and alias names the oracle cannot propagate into child files), none defined inside the crates (section 3.4).
- Source-text readers handled (section 4): preservation records re-recorded with proof (195 law sources moved, 20 shas re-recorded, assertion total 1115 = 1115), 11 TS readers re-pointed to the module tree, one policy root registered. No compatibility re-export, alias or shim; no crate, manifest, launch route, owner row or lock changed.

## 1. Found state (fresh slice)

- No earlier c5a work: no `c5a-tools`, no `🗑️generated/c5a-*`, no RESULT file. Pre-edit snapshots: `F/c5a-tools/orig/<tag>.rs`, hashes in `orig/SHA256`. Re-measured line counts equal the census (no peer touched the 16 files between the census and the first edit).
- Mount facts: the 11 test files are `#[path]` modules of the companion test crates `tests-01 .. tests-16` (`PL/🧷️companions`) or `include!` fragments of them (`ct` ends with `}include!("../…-contract-x/🦀️.rs");` on one line, `c1x` is itself the `-x` part that part-1 includes, `atc` is `mod typed_command_full_operation_tests { … }` included inside the retained-command host module); the 5 lib files are the lib roots of the crates `semio-framework-plugin-time-travel`, `-time-travel-turns`, `-tool-run`, `-tool-run-turns`, `-reactor-turn`.
- Baseline census rows: `F/🗑️generated/c5a-census-own-before.jsonl`; baseline typeck oracle (`pos`, lib and test): `F/🗑️generated/c5a-pos-pre.log`, taken on the unsplit tree before the first lib install.

| tag | owning crate (lib root) | file (tail under `PL/`) |
| --- | --- | --- |
| c6, c7, ct, c1x (via part-1), c3, c4, c5 | `…-tests-11-2`, `-12-2`, `-05`, `-08`, `-16`, `-14-2`, `-09-2` | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract*` |
| tr1, tt1, tt2, atc | `…-tests-13`, `-07-2`, `-15`, `-01` | `🧪️tests/🔬️tool-run-part-1`, `🧪️time-travel-part-1`, `-part-2`, `🔬️app-typed-command-full-operation` |
| tt, tti, tr, tri, rt | `semio-framework-plugin-time-travel`, `-time-travel-turns`, `-tool-run`, `-tool-run-turns`, `-reactor-turn` | `⏪️time-travel`, `⏪️time-travel/🧵️instance`, `⏯️tool-run`, `⏯️tool-run/🧵️instance`, `⚛️reactor/🔄️turn` |

## 2. What was cut (before → after, lines)

| tag | file (tail under `PL/`) | before | root | children | largest child | total after |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| c6 | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6/🦀️.rs` | 1,405 | 123 | 6 | 268 | 1,416 |
| c7 | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-7/🦀️.rs` | 1,372 | 132 | 7 | 378 | 1,382 |
| ct | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | 2,035 | 104 | 8 | 367 | 2,059 |
| tr1 | `🧪️tests/🔬️tool-run-part-1/🦀️.rs` | 1,800 | 100 | 8 | 275 | 1,823 |
| tt1 | `🧪️tests/🧪️time-travel-part-1/🦀️.rs` | 1,839 | 77 | 8 | 327 | 1,861 |
| tt2 | `🧪️tests/🧪️time-travel-part-2/🦀️.rs` | 1,329 | 74 | 6 | 261 | 1,341 |
| c1x | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🦀️.rs` | 2,478 | 12 | 10 | 295 | 2,518 |
| atc | `🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` | 1,569 | 32 | 8 | 349 | 1,584 |
| c3 | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🦀️.rs` | 1,865 | 123 | 8 | 341 | 1,881 |
| c4 | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/🦀️.rs` | 1,473 | 115 | 7 | 245 | 1,488 |
| c5 | `🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-5/🦀️.rs` | 2,143 | 106 | 8 | 361 | 2,159 |
| tt | `⏪️time-travel/🦀️.rs` | 1,458 | 92 | 6 | 429 | 1,511 |
| tti | `⏪️time-travel/🧵️instance/🦀️.rs` | 1,241 | 81 | 8 | 195 | 1,305 |
| tr | `⏯️tool-run/🦀️.rs` | 1,720 | 118 | 6 | 516 | 1,789 |
| tri | `⏯️tool-run/🧵️instance/🦀️.rs` | 1,096 | 62 | 6 | 246 | 1,145 |
| rt | `⚛️reactor/🔄️turn/🦀️.rs` | 2,321 | 125 | 9 | 966 | 2,400 |

Children (folder, lines), in include or mount order:

- **c6**: `🏭️tool-factory-keys-and-shared-actions` 240, `🧯️retained-proof-and-app-maintenance` 225, `📥️command-ingress-pages` 254, `🔀️merge-heads-and-media-import` 185, `↩️view-action-undo-and-history-delivery` 268, `🧵️spawned-task-resume` 121
- **c7**: `🆔️request-context-serializer-and-shared-job` 165, `🏗️media-result-close-destructor-and-builder` 132, `🗒️peers-clipboard-and-history-panel` 378, `🧭️ribbon-menus-and-surface-context` 219, `🪪️command-handler-and-registry-laws` 134, `🧩️task-admission-and-cancellation` 143, `📜️mounted-history-and-reserved-job-currency` 79
- **ct**: `👶️child-grant-and-maintenance-fixtures` 102, `⌨️test-command-and-test-app-type` 130, `📋️clipboard-and-retained-command-fixtures` 364, `🎛️test-app-artifact-app` 276, `🗝️keyed-dispatch-fixtures` 367, `🗃️contract-registries-and-reserved-helpers` 205, `🏃️contract-app-drivers` 226, `👥️interaction-presence-and-child-fixtures` 285
- **tr1**: `🧸️toy-job-ticks-abort-and-finalize` 237, `🚰️reclaim-ingest-pause-and-reconfigure` 275, `🔌️integration-seams` 275, `🔢️action-arg-carriers-and-read-only-runs` 110, `🎯️base-change-chords-and-retarget` 207, `🪟️trace-requests-and-window-config` 245, `📖️reader-windows-and-panel-fixtures` 247, `🛑️terminal-run-and-retained-config-close` 127
- **tt1**: `🎬️fixture-scenarios-sessions-and-drafts` 165, `🚚️long-transactions-and-single-controls` 196, `🔁️unchanged-input-stage-and-alternatives` 189, `🪞️replicas-and-history-edit-rows` 220, `✅️acceptance-blocked-review-and-long-history` 214, `⏲️interior-undo-pure-lane-and-wall-deadline` 221, `🔄️opened-instance-edit-drafts-and-withdraw` 252, `🧨️inert-label-op-and-hostile-input` 327
- **tt2**: `🗑️retirement-custody-and-transaction-rows` 180, `🔢️input-shapes-lists-unions-and-sources` 240, `🎨️references-colours-nullable-and-replay` 260, `🧮️ledger-replay-chips-and-alternatives` 181, `🗝️editor-keys-history-patch-and-actors` 145, `🪣️shared-schemas-filters-and-backbone` 261
- **c1x**: `🧱️composed-parent-snapshot-and-recursive-mutation` 196, `🔓️decode-authority-and-field-decoders` 254, `🌳️field-catalog-parent-app-and-branch-snapshot` 292, `🧵️recursive-wire-open-and-producer` 270, `🚀️ready-parent-initialization` 295, `🔧️replacement-fixtures-and-recursive-documents` 272, `📦️document-archive-and-replacement-publication` 225, `🏁️replacement-close-cancellation-and-nested-window` 247, `🗂️recursive-archive-rejection-and-stale-authority` 228, `📨️envelope-decode-ladder` 227
- **atc**: `🔎️census-oracles-and-fixture-machine` 134, `🔐️two-turn-presence-and-root-retirement` 187, `🚫️cancellation-publication-boundaries` 227, `📌️document-cancellation-and-latest-wins-keys` 349, `⚖️slot-and-publication-fairness` 222, `🌐️neutral-hostile-rows` 136, `👪️child-genesis-wire-and-group-owner` 142, `🧾️source-rejection-anchors-and-host-lane` 155
- **c3**: `🔚️close-turn-wire-input-and-no-state-disposers` 149, `🎟️typed-operation-slots` 118, `🛂️continuation-publication-and-host-crossing` 203, `📍️retained-operation-close-and-checkpoint` 292, `🪦️snapshot-retirement-gesture-and-agent-transaction` 239, `🧷️composed-checkpoint-commit` 190, `🧹️child-root-maintenance-and-rates` 226, `↪️reserved-undo-ui-stamping-and-selection` 341
- **c4**: `🎞️host-events-media-cancel-and-download-seam` 188, `👶️child-emit-prepublication-and-ephemeral-lanes` 189, `📚️archive-load-reload-and-revert` 187, `↩️reserved-undo-and-note-shell-command` 245, `📡️host-events-intents-and-streamed-tick` 184, `🖱️interaction-hover-and-select-laws` 226, `🔀️selected-ids-merge-vocabulary-and-checkpoint` 154
- **c5**: `🎡️maintenance-rotation-download-and-ingress` 163, `🥾️operation-action-stack-and-peer-presence` 257, `📎️member-owners-media-edge-and-clipboard` 301, `🧶️streamed-ticks-and-reserved-undo` 153, `📔️command-log-laws` 361, `🔭️empty-undo-ingest-and-context-menu` 260, `🧮️gestures-manifest-commands-and-selection-persistence` 219, `🎚️selection-modes-verbs-and-hover` 339
- **tt**: `⏱️limits-and-outcomes` 93, `✏️editor-and-member-subjects` 73, `📒️time-travel-ledger` 417, `🗄️store-state` 429, `🧱️store-owners-and-member-visitors` 159, `🔀️supersession-ledger` 248
- **tti**: `🧹️owners-and-retirement` 133, `🔄️reprojection-turns-and-base-watch` 141, `🎬️action-dispatch-and-effects` 138, `🔎️preview-replay-and-session-start` 164, `✍️draft-inputs` 141, `🏃️turn-driving-and-replay-steps` 131, `📖️ref-reads` 181, `🪄️supersede-authoring-and-extension` 195
- **tr**: `⏱️limits-and-job-slot` 208, `🔁️refold-finalize-and-entry` 331, `📒️driver-and-ledger` 516, `👥️member-owners-and-visitors` 338, `🏷️action-outcome-and-labels` 97, `🪧️panel-groups` 181
- **tri**: `🚀️action-dispatch-and-start` 246, `👀️generation-watch-and-turn-driver` 204, `🎟️job-admission-and-finalize-turn` 184, `📤️publication-and-member-refresh` 210, `🧹️member-retirement-and-close` 119, `🔌️ref-reads-and-extension` 120
- **rt**: `🔌️extension-invocation-and-command-ingress` 198, `🔦️more-work-sources-and-tracing` 174, `🎯️dirty-poll-surfaces` 150, `⏱️turn-execution-and-poll-entry` 168, `🌀️poll-kernel-turn` 966, `🧊️cold-pair-loads-and-process-pool` 131, `🧯️retirement-drivers-and-turn-limits` 126, `🩹️reconcile-drive-and-publication-receipts` 250, `📬️task-resumes-and-wire-effects` 112

### Mechanism and decisions

- **Test roots (packages 1 and 2, `frag.py`)**: verbatim depth-0 line ranges (atc: depth 1) move into `<emoji>️<kebab>/🦀️.rs` next to the root; the root keeps its docstring, its `use` block, its `#[path]`/`include!` mounts and gets one `include!("<folder>/🦀️.rs");` per child, so every item keeps its module identity, visibility and `use` scope and nothing is widened. Boundaries follow the test names (topic groups of 60 to 380 lines); a chunk starts at the first non-blank line after the previous item and takes its `//#endregion` lines with the previous chunk. A region that spans a cut is re-opened and closed with a balanced marker pair in every fragment it covers, the name taken from the opening marker (tt1, ct and tt2 carry name-mismatched marker pairs in the original; pairing is by nesting, as the compiler-free tools of earlier slices did). Relative `include_str!`/`include!` literals of a fragment gained one `../` (40 literals; their absolute targets are proven equal and existing, section 3.1); `concat!(env!("CARGO_MANIFEST_DIR"), …)` forms are manifest-relative and need no change. A fragment starts with a `///` docstring where its first line is an item, otherwise with `//`; `c1x` (a root that is only items) got one `//` root docstring line. `ct` needed one declared pre-edit: the closing `}` and the trailing `include!` shared a line, so the `include!` moved to its own line. `tr1`, `c3` and `c5` keep their trailing file `mod`/`include!` items in the root (`end_line`). No `#[path]` file module sits inside a fragment (asserted).
- **Lib roots (package 3, `modsplit.py`, descendant of the PKG-47 stager)**: whole items move into `#[path = "<folder>/🦀️.rs"] mod <snake>;` children (wiring at the first item each child took, so the root keeps its region skeleton, imports, `#[path]` test mounts and external re-exports), each child starts with `//! <emoji> … See [`super`].` and `use super::*;` (omitted in `🧯️retirement-drivers-and-turn-limits`, which names nothing of its parent: the oracle reported the unused glob, fixed). The root re-exports with `pub use child::*` (visibility from the widest named item). Private items, fields and inherent methods whose name occurs in another file of the split scope became `pub(super)` (tt 17, tr 13, tri 18, tti 28, rt 40 edits, reports `/Users/ueli/.cache/c5a/stage/<tag>.widen.tsv`); the definition scan finds 0 cross-file private leftovers (tt 4, tr 9, tri 11, tti 10, rt 10 declarations scanned).
- **Impl re-opening (tri, tti)**: the two instance-turn crates hold one inherent `impl` of 814 and 935 lines. They were cut by method group into several `impl<…> Ctx<…> { … }` blocks of the same header (declared pre-edit, 4 cuts in tri, 5 in tti; only the closing brace and the header line are new), then each block moved with its group.
- **Statics stay in the root of the reactor turn**: `thread_local!` and `component_persistent_local!` blocks (`RETAINED_EXTENSION_*`, `COMMAND_INGRESS`, the trace block, `ARENA_STARVED`, `TURN_EXECUTION`, `PROCESS_POOL`, `REFUSED_PUBLICATION`) are the reactor's own state and are read from several children; they were not widened or moved (the macro items are outside every child range), and the types they name are widened where needed. The child that holds `poll_kernel_turn` is mounted as `poll_turn` to avoid a module with the name of its function. The textual `selected_entry!` macros of tool-run stay in the root before the children that use them.
- **Roots keep what is genuinely theirs**: file mounts (`#[path]` modules of tests and staged pieces), `extern crate` aliases, the external globs, `pub use` of lifted modules, macros.

## 3. Evidence (all commands run in this session on the live tree unless noted)

### 3.1 Structure and losslessness

| Check | Command (tools in `F/c5a-tools/`) | Result |
| --- | --- | --- |
| Install precondition | `python3 install.py check <11 tags>` then `apply`; `python3 modsplit.py install <tag>` | each asserted the live file equals its snapshot and the child folders do not exist; `problems 0`, `installed 95 files` (11 roots + 84 children); the five lib roots installed one by one |
| Test roots exact | `python3 chk.py live c6 c7 ct tr1 tt1 tt2 atc c1x c3 c4 c5` | `CHK OK live`: `gone=0 extra(non-marker)=0` and sequence equality (fragments spliced back at their `include!` lines, one `../` removed, balancing markers and docstring dropped) for all 11; test fn multisets equal; include targets equal and existing; regions balanced; docstring emoji equals folder emoji, unique among siblings and different from the root's; depth 0 at the end of every file; rustfmt parses every file (the `trailing whitespace` formatter note of `error[internal]` is not a parse error) |
| Include literals | inside `chk.py` | the multiset of absolute `include*!` targets of root plus children equals the snapshot's, every target exists (40 rebased literals) |
| Module splits lossless | `python3 modsplit.py lossless <tag> <live dir>` | `LOSSLESS OK` x5, `missing 0`; the extras are only wiring, docstrings, markers, the declared pre-edit lines; regions orig/new tt 7/10, tr 7/10, tri 0/0, tti 0/0, rt 1/2 |
| Module structure | `python3 chkmod.py tt tr tri tti rt`, `C5A_LIVE=1 python3 modclash.py …`, `python3 defscan.py <tag> <live dir>` | `CHKMOD OK` (mounts once, docstrings, regions, depth, rustfmt, path length, no `[DEBUG]`), `modclash OK`, 0 cross-file private leftovers |
| Taxonomy and names | `node ../c4p5-tools/rgi.mjs newnames.txt`, `bun tax.ts newdirs.txt`, emoji list `/Users/ueli/.cache/c4p43/folder-emojis.txt` | `bad=0` for 119 names; `{"dirs": 119, "unresolved": []}`; every emoji already used as a folder prefix in the repo (one replacement, `🚆` to `🚚`, before installing) |
| Paths and debug | inline scan over the 119 files | longest repo-relative path 208 UTF-8 bytes (limit 240), longest absolute 191 characters (limit 256), 0 files with `[DEBUG]`, 0 inner attributes or inner docs in a test fragment |
| Formatting debt | `rustfmt --check` hunks and changed-line multiset, orig versus root plus children (lib files) | tt 36 / 36, tti 30 / 35, tr 28 / 30, tri 36 / 39, rt 61 / 60 hunks; the changed-line multisets differ only by the same count of `pub(super)` prefixes (4 / 20 / 10 / 12 / 34 lines each way): no new debt kind |

### 3.2 Census (canonical `readRustSourceClosureV1`)

`bun census-own.ts <root>` before (`c5a-census-own-before.jsonl`) and after (`c5a-census-own-after.jsonl`); files / bytes / active lines (ratio of the worst limit 32 / 262,144 / 4,000), `--test` closure for the test crates (their lib closure is the 1-file crate root):

| crate | before | after |
| --- | --- | --- |
| `tests-01` (atc) | 9 / 220,914 / 2,055 (0.843) | 17 / 222,867 / 2,078 (0.850) |
| `tests-05` (ct) | 6 / 189,919 / 2,789 (0.724) | 14 / 192,038 / 2,821 (0.733) |
| `tests-07-2` (tt1) | 3 / 168,191 / 1,904 (0.642) | 11 / 170,436 / 1,934 (0.650) |
| `tests-08` (c1x via part-1) | 14 / 227,900 / 2,751 (0.869) | 24 / 231,248 / 2,801 (0.882) |
| `tests-09-2` (c5) | 7 / 213,195 / 2,313 (0.813) | 15 / 215,144 / 2,337 (0.821) |
| `tests-11-2` (c6) | 10 / 213,367 / 1,902 (0.814) | 16 / 214,811 / 1,919 (0.819) |
| `tests-12-2` (c7) | 11 / 205,329 / 1,501 (0.783) | 18 / 207,215 / 1,518 (0.790) |
| `tests-13` (tr1) | 6 / 206,706 / 2,359 (0.789) | 14 / 208,893 / 2,390 (0.797) |
| `tests-14-2` (c4) | 10 / 195,470 / 1,686 (0.746) | 17 / 197,147 / 1,708 (0.752) |
| `tests-15` (tt2) | 20 / 234,983 / 2,183 (0.896) | 26 / 236,546 / 2,201 (0.902) |
| `tests-16` (c3) | 10 / 183,134 / 2,056 (0.699) | 18 / 185,127 / 2,080 (0.706) |
| `time-travel` lib | 3 / 120,384 / 1,678 (0.459) | 9 / 122,648 / 1,737 (0.468) |
| `time-travel-turns` lib | 2 / 104,496 / 1,332 (0.399) | 10 / 107,933 / 1,404 (0.412) |
| `tool-run` lib | 6 / 133,508 / 2,178 (0.544) | 12 / 135,679 / 2,253 (0.563) |
| `tool-run-turns` lib | 2 / 83,107 / 1,176 (0.317) | 8 / 85,749 / 1,231 (0.327) |
| `reactor-turn` lib | 6 / 217,415 / 2,688 (0.829) | 15 / 221,197 / 2,778 (0.844) |

(the brief's "tests-02..05 at 31 of 32 files" is out of date: the bins were re-split into `-2` crates before this slice; `tests-05` held 6 files and holds 14 now, and the largest file count after this slice is 26, `tests-15`.) Full-workspace census (`cgc-census.ts`, a copy of `🗑️generated/cgc7-census.ts` that reads the shared manifest list and writes only `c5acgc-*`; 3,068 crates, `readerRefusedCrates` 0, 1 oversized lib, none of mine: `semio-s-artifact-bim-model`): all 16 crates above have `libOver false`, `testOver false`, `libCompilable true`, `testCompilable true`, `libOverDeps 0`, `testOverDeps 0`, with the same files, bytes and lines as the table (`c5acgc-crate-census.csv`, `c5a-cgc-census.out`).

### 3.3 Scoped patched differential (resolution level, `oracle5.py`)

`python3 oracle5.py <group>` for the 11 test crates and the 5 lib crates (log `c5a-oracle-all.log`, rerun of rt and tests-15 in `c5a-oracle-rerun.log`): `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null` from the crate lib root, lib and `--test`, `CARGO_MANIFEST_DIR` set to the crate folder, `extern crate` lines of missing crates removed and external globs replaced by `const _: () = ();` identically on both sides, missing include targets pulled from the repo into the sandbox. A = the crate directories copied with the snapshot restored and the child folders removed, B = the live tree. Error counts equal on both sides (orig = new, only-orig 0, only-new 0) in every group: tests-01 421 / 557, -05 124 / 126, -07-2 1 / 457, -08 1 / 628, -09-2 1 / 738, -11-2 1 / 611, -12-2 1 / 467, -13 1 / 231, -14-2 1 / 517, -16 1 / 601, time-travel 373 / 425, time-travel-turns 207 / 211, tool-run 374 / 531, tool-run-turns 152 / 156, reactor-turn 542 / 542 (lib / test); warnings equal in all (the first run of reactor-turn reported one extra `unused import: super::*`: fixed, section 2). tests-15 `--test`: 257 = 257 errors; one error line differs only by its text because it names the include path of the `time-travel-band` fixture (`…/⚖️testing-laws/🧱️unit-4/../…/🧪️time-travel-part-2/[🗝️editor-keys-history-patch-and-actors/]../../../…/🔣️.json`): the sandbox cannot reach that file on either side (its path holds a ZWJ emoji), and `chk.py` proves the target the same and existing on the live tree. The oracle is weak where external crates are missing (it proves mount structure and crate-local name resolution); the typeck-level run follows.

### 3.4 Typeck-level oracle (`pos`) for the five lib crates

`PYTHONHASHSEED=0 POS_PREFIX=c5a python3 posrun.py run families/c5a.json --cfgs lib,test --label {pre,post} --no-base` (family `c5a.json`: the five crates; stub layer for everything outside). `pre` ran on the unsplit tree (`c5a-pos-pre.log`), `post` on the final lib tree before the last child tweak (`c5a-pos-post.log`). Both reach the typeck canary (E0308, E0616, E0624). Non-stub classes, lib / test: **privacy 8 / 8 before and 8 / 8 after** (the same four distinct E0616 messages, see section 5), **trait-scope 0 / 0 both**, type 141 / 216 before and 136 / 211 after, other 0 / 0 both, **resolve 29 before and 194 after (+165 rows, 41 new unresolved names)**. `namecheck.py` (log `c5a-namecheck.log`) classifies the new names: none is defined in any of the five crates; they are names behind the roots' external globs and aliases (`use semio_framework_ui_contract::*;`, `use … as ui_contract;`, the panel and command glob re-exports) that the oracle's stub layer learns only from the file holding the import, so they surface as `cannot find … in this scope` once the use sits in a child (the limit already documented for glob-held traits in CENSUS4-PKG-47 §3.3), plus the macro-generated statics `COMMAND_INGRESS` and `REFUSED_PUBLICATION` that the pre run reports too. A patched copy of the oracle (`pos-tools/pos.py`, fills also looked up in the parent file's globs, label `post2`) lost the typeck canary (alias paths such as `ui_contract::SurfaceId` are scanned per file) and was dropped; the real `bun repo cargo` check (section 7) is the confirmation.

### 3.5 Source-text laws that were run

TS laws that read the cut files, one `bun test ./<path>` each (log `c5a-ts-laws2.log`; the `units` rerun with its failure details is `c5a-ts-units-final.log`): `🧪️testing/🎟️caller` 8 pass; `⏪️time-travel/🧪️tests/🧩️history-partition` 4 pass (the reader memoises the module tree: the first run timed out at 5 s); `🛠️tool-machine/♻️retirement` 9 pass; `plugin-runtime-runtime-close-budget` 30 pass, 1 skip; `🧪️tests/📦️units` 10 pass, 3 fail (the preservation law, the surface-carrier law, the child-group law and the registration laws pass; the 3 failures are foreign, section 5; an earlier run of the same file had 7 pass, 6 fail while a peer was rewriting `🧰️framework/Cargo.lock`). The exported oracles (`oracles.ts`, no bun:test runner): `issuedPatchOracle` 10, `coldDocumentPairIngressOracle` 13, `timeTravelScenarioOracle` 15 pass; four fail on anchors that no longer exist in the unsplit files either (section 5), checked by running the anchors of the cut files directly against the module trees (`/Users/ueli/.cache/c5a/t/d.ts`: all present).

## 4. Source-text readers and records

Search method: `git grep -F` of every file path suffix and crate path over `*.ts *.tsx *.json *.rs *.py *.toml *.mjs *.jsonc` (excluding the ticket tree), logs `c5a-readers-a.txt` and `c5a-readers-b.txt`, plus a read of every candidate.

| Reader | What it reads | Action |
| --- | --- | --- |
| `PL/🧪️tests/📦️units/📋️preserved.json` (214 law rows) | the 195 laws of parts 3 to 7: `source` path and body sha256 | **re-recorded** by `rerecord.py`: every `source` points to the fragment that holds the law; 175 shas unchanged, 20 changed only by the `../` rebase of an `include_str!` literal, each proven (the new body with the rebase reversed equals the old body, and the old body hashes to the recorded sha); no law name appears twice; total assertions over the new source set 1,115 = `originalAssertions` + `addedAssertions` |
| `PL/🧪️tests/📦️units/🟦️.ts` | `family()` (surface-carrier law) and the `original` contract file of the preservation law | **edited**: both read the module tree (`readModuleTreeSource`), so the root's fragments are part of the text; the tree stops at the `../…-x` mount, as the old family filter did |
| `PL/🧪️tests/📦️units/📋️surface-carriers.json` | `remaining`: 469 function shas of the contract family | **re-recorded**: 22 shas and byte counts changed by the rebase (proof as above), 447 unchanged, 0 unmatched either way |
| `PL/🧪️tests/📦️units/📋️child-group-owner.json` and `🧬️child-group-owner.json` | `remaining[1]`: the 57 functions of `app-typed-command-full-operation/🦀️.rs` | **edited**: the entry became the root plus its 8 fragments in include order (9 entries); 11 of the 57 shas changed by the rebase (proved); the schema's `minItems`/`maxItems` went from 2 to 10 |
| `PL/🧪️testing/🎟️caller/🧪️tests/🟦️.ts` | `familyText` of the contract, tool-run and time-travel test families | **edited** (`readModuleTreeSource` per family root) |
| `PL/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts` | all `🔬️plugin-runtime-plugin-builder-contract*` roots | **edited** (module tree per root) |
| `✏️s/…/📐️cad/…/🔬️cad-presence-retirement/🟦️.ts` | the contract root, for two law names that live in part 5 (it was already broken: the root has no test) | **edited**: reads the module tree of part 5 |
| `PL/⏪️time-travel/🧪️tests/🧩️history-partition/🟦️.ts` (+ fixture) | items defined once in the files of the history partition (`⏪️time-travel/🦀️.rs`) | **edited** (memoised module tree) |
| `PL/🛠️tool-machine/♻️retirement/🧪️tests/🟦️.ts`, `PL/🏇️mounted-owner/🧪️tests/🟦️.ts`, `PL/🧩️extension/🧪️tests/📨️invoke/🟦️.ts`, `PL/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts`, `PL/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts`, `PL/🧪️tests/🧪️time-travel/🟦️.ts`, the planning test of `🏪️store/🕰️history`, the media-slots collect test of the renderer, `🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧬️retained-clone-check/🟦️.ts` | the roots of reactor-turn, tool-run (both crates) and time-travel | **edited**: `readFileSync` of the root became `readModuleTreeSource` (12 call sites, 8 imports added with the relative path; `tsedit.py`) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` `POLICY_SPLIT_SOURCE_ROOTS` | the interactivity audit reads `INTERACTIVITY_AUDIT_REACTOR_TURN_FILE` as a module tree only when the root is listed | **edited**: the reactor turn root is listed |
| `PL/🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts`, `PL/⚛️reactor/🧪️tests/🔬️command-ingress-terminal/🦀️.rs`, `pluginFamilyText()` users | tt/tr/rt text | **no edit**: they already read module trees (`readModuleTreeSource`, `read_module_tree_source`) or every non-test `.rs` file under the plugin folder |
| `🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json` | `contract/🦀️.rs` must contain `semio_framework_composition_admission::` | **no edit**: the unsplit file never contained the string (the row was already failing) |
| Cargo workspace, `📋️build-owners.json`, `🔒️dependencies.json`, launch files, `📋️project.json` | crate roots and manifests only | **no edit**: no crate, target, command or owner row changed; folder names resolve in the taxonomy |
| comments and doc mentions of the paths | prose | **no edit** |

## 5. Foreign errors and findings

- Pre-existing, not touched: (1) the typeck oracle reports four distinct E0616 messages before and after (`retirements`, `discarded_factory`, `discarded`, `discarded_pending` of `TimeTravelStoreState` are private but used by the turns crate `time-travel-turns`); a real cross-crate visibility defect of the PI2 crate cut, the same rows move with the struct into its child. (2) The 3 failing tests of `📦️units` are foreign: `actual feature verdict owner …` (dependency `semio-framework-2d` expected, `semio-framework-2d-compute` found), `actual private surface fixture owner …` (EISDIR: `🪟️surface-fixture/🦀️.rs`, edited by a peer today, mounts a directory) and its package-alias twin. (3) Stale anchors that were already absent from the unsplit files: `guestLifecycleOracle` (`pub use turn::poll_kernel;` in the reactor root), `documentBackboneBindingOracle` (`attach_hot_backbone` in the store root), `extensionInvokeCallerOracle` (`extension_invoke(&capability, &payload, cx)` exists nowhere), `toolJobLatestWinsSelfTests` (anchors in the plugin root), the retained-clone-check assertion `store.handoff_batch_publication_close(&mut owner)` (not in tool-run), the planning test and the media-slots collect test (other roots), `mounted-owner` (needs native result artifacts).
- Bun 1.3.14 crashes (segmentation fault) when `bun test` gets a bare path: it treats it as a filter and scans 7.5 million files; use `bun test ./<path>`.
- `bun repo`-independent census tool note: `cgc-census.ts` (copy) tolerates an unparsable foreign manifest (`✏️s/🔌️plugins/🗄️stdio/…/🔢️value/…/Cargo.toml` had a duplicate key at 23:2x).
- A peer sweep touched `🪟️surface-fixture/🦀️.rs`, `cooperative-pump` and other test files in the same folder during the slice; none of them is one of the 16 files.

## 6. Hand-offs

- `bd11` (plugin TS laws): the plugin TS laws listed in section 4 now read the cut sources through the module tree; the preservation records point at the fragments. Any new split of a contract part must re-run `rerecord.py` (or its successor).
- The four anchors that were already stale (section 5) belong to their owners.

## 7. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md` and no mutex folder `🗑️generated/cargo-mutex.lock` (checked 23:3x). Command for the compile queue (hold the fleet mutex around the single launcher call):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-plugin-time-travel -p semio-framework-plugin-time-travel-turns -p semio-framework-plugin-tool-run -p semio-framework-plugin-tool-run-turns -p semio-framework-plugin-reactor-turn -p semio-framework-plugin-tests-01 -p semio-framework-plugin-tests-05 -p semio-framework-plugin-tests-07-2 -p semio-framework-plugin-tests-08 -p semio-framework-plugin-tests-09-2 -p semio-framework-plugin-tests-11-2 -p semio-framework-plugin-tests-12-2 -p semio-framework-plugin-tests-13 -p semio-framework-plugin-tests-14-2 -p semio-framework-plugin-tests-15 -p semio-framework-plugin-tests-16 --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```

A `--target wasm32-wasip2` variant covers the wasm-gated parts of the reactor turn (`PROCESS_POOL`, `note_turn_events`).

## 8. Files

- Edited (16 roots): the files of the table in section 2.
- Created (119 children, one `🦀️.rs` per new folder, all next to their root): the folders listed in section 2; absolute list in `F/c5a-tools/newdirs.txt`, SHA256 of all 135 final files in `F/c5a-tools/final-SHA256`.
- Edited (readers and records): `PL/🧪️tests/📦️units/{🟦️.ts,📋️preserved.json,📋️surface-carriers.json,📋️child-group-owner.json,🧬️child-group-owner.json}`, `PL/🧪️testing/🎟️caller/🧪️tests/🟦️.ts`, `PL/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts`, `PL/⏪️time-travel/🧪️tests/🧩️history-partition/🟦️.ts`, `PL/🛠️tool-machine/♻️retirement/🧪️tests/🟦️.ts`, `PL/🏇️mounted-owner/🧪️tests/🟦️.ts`, `PL/🧩️extension/🧪️tests/📨️invoke/🟦️.ts`, `PL/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts`, `PL/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts`, `PL/🧪️tests/🧪️time-travel/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🧪️tests/🟦️.ts`, `…/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️media-slots/🎟️collect/🧪️tests/🟦️.ts`, `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧬️retained-clone-check/🟦️.ts`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts`, `✏️s/🔌️plugins/📐️cad/…/🔬️cad-presence-retirement/🟦️.ts`.
- Tools, kept: `F/c5a-tools/` (`frag.py`, `specs5.py` (chunk tables), `chk.py`, `install.py`, `modsplit.py`, `specs/*.py`, `widen2.py`, `rsitems.py`, `chkmod.py`, `defscan.py`, `modclash.py`, `oracle5.py`, `posrun.py`, `posdiff.py`, `pos-tools/`, `namecheck.py`, `fnrows.ts`, `rerecord.py`, `rerecord_cgo.py`, `tsedit.py`, `oracles.ts`, `tstests.sh`, `census-own.ts`, `cgc-census.ts`, `cgcloop.sh`, `tax.ts`, `ol.py`, `methods.py`, `report.py`, `show.py`, `ts-probe.ts`, `orig/` snapshots with `SHA256`, `staged-*.json`, `families/c5a.json`, `files.tsv`, `newnames.txt`, `newdirs.txt`, `final-SHA256`). Logs: `F/🗑️generated/c5a-*`; nothing deleted. Sandboxes: `/Users/ueli/.cache/c5a/` (`stage`, `stage-frag`, `sbx`, `orc`, `c5a-work`, `t`).
- No `[DEBUG]` log was added to the repo; `cgc-census.ts` keeps the progress lines of the shared census tool.
