# Result CENSUS4-PKG-44 (prefix `c4p44`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `M` = `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`. Tools: `F/c4p44-tools/` (descendant of `c4p37-tools`). Logs: `F/🗑️generated/c4p44-*`. Sandboxes: `/Users/ueli/.cache/c4p44/`.

## 0. Verdict

**DONE** (static; real compile queued). All seven files are split into 55 cohesive children in 55 new folders (45 `#[path]` modules for six files, 10 `include!` fragments for the test file `wsq`; largest child 320 lines, roots 52 to 84 lines), installed 14:53 and re-checked on the live tree: losslessness exact 7 of 7, test-fn multiset identical (56 = 56), region pairs balanced per file, every `include_str!` target preserved, rustfmt parse clean, taxonomy 55 of 55 resolved, every folder mounted exactly once, sibling emoji unique, longest path 166 UTF-8 bytes, census 0 over the gate (lib and test), scoped patched differential **identical** (lib, `--test`, wasm32-unknown-unknown lib and test: 24 of 24 comparisons, 0 differences, warnings identical), full-crate typeck oracle: 0 privacy, 0 trait-scope, 0 other non-stub rows before and after, resolve rows equal, the only row delta is stub-trait emulation (section 5). Two source-text readers were handled (section 3). No `COMPILE ADMITTED` line in `📓️FLEET.md` (checked 15:00), so the launcher check is queued (section 8).

## 1. Found state (fresh slice)

- No earlier c4p44 work: no `c4p44-tools`, no `🗑️generated/c4p44-*`, no RESULT file.
- Snapshots: `F/c4p44-tools/orig/<tag>.rs`, hashes in `F/c4p44-tools/orig/SHA256`. Census counts were re-measured; `wsq` (1,099 to 1,098) and `cat` (1,430) differ from `HEAD` by peer edits that are part of the snapshot.

| tag | file (tail under `M/`) | lines | SHA256 (first 16) | owning crate | mount |
| --- | --- | ---: | --- | --- | --- |
| pac | `🏠️workspace/📨️plugin-artifact-channel/🦀️.rs` | 1,217 | `22e35969176bdab9` | `semio-framework-os-mcp-plugin-runtime` | `pub mod plugin_artifact_channel` (cfg not wasm32) of `⚙️plugin-runtime/🦀️.rs` |
| gwb | `🏠️workspace/🛂️gateway-backend/🦀️.rs` | 1,206 | `73dc85f77c3c0f37` | `semio-framework-os-mcp-workspace` | `pub mod gateway_backend` of `🪟️workspace-host/🦀️.rs` |
| wsq | `🏠️workspace/🧪️tests/🔬️quick/🦀️.rs` | 1,098 | `21ac09200832867` | `semio-framework-os-mcp` (glue) | `#[cfg(test)] mod workspace_quick` of `📦️packages/🦀️rust/🦀️.rs` |
| inf | `💡️inference/🦀️.rs` | 1,318 | `d59eb2fc1511d8f3` | `semio-framework-os-mcp-tools-inference` | `pub mod inference` of `🔭️inference-tools/🦀️.rs` |
| cat | `🗂️catalog/🦀️.rs` | 1,430 | `a06b2e0345603473` | `semio-framework-os-mcp-catalog` | `pub mod catalog` of `📚️catalog-compiler/🦀️.rs` |
| sch | `🧬️schema/🦀️.rs` | 1,288 | `6017b96599d6ad0` | `semio-framework-os-mcp-schema` | `pub mod schema` of `📐️schema-contract/🦀️.rs` |
| ses | `🧵️bridge/🪢️session/🦀️.rs` | 1,232 | `9cbc9e1d6d0de01b` | `semio-framework-os-mcp-bridge` | `pub mod session` of `🪢️bridge-session/🦀️.rs` |

## 2. What was cut

Six files are cut into `<emoji>️<kebab>/🦀️.rs` children wired in the parent with `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus `pub use <snake>::*;` (or `pub(crate) use` / plain `use` by the best visibility of the child's named items; no re-export when the child holds only `impl` blocks). Each child starts with an `//!` docstring whose first character is the folder emoji, then `use super::*;`. The test file `wsq` is a flat test module whose test paths (`workspace_quick::<test>`) must not change, so it uses `include!` fragments (the PKG-15/28/39 mechanism, `F/c4p44-tools/frag/`, descendant of `c4p39-tools`), each fragment starting with a `///` docstring with its unique emoji.

Large inherent `impl` blocks were split by member (new `split_impls` in `specs/lib.py`): the impl header (attributes included) is repeated verbatim for each group of members, so `pac` (843-line `impl PluginArtifactChannel`, 4 blocks) and `gwb` (904-line `impl HeadlessWorkspace`, 5 blocks) have cohesive children. The repeated `}` and header lines are the declared `pre_extra` of the lossless check.

| tag | File | Before | After: root + children (max child) | Children (lines) |
| --- | --- | ---: | --- | --- |
| pac | `🏠️workspace/📨️plugin-artifact-channel` | 1,217 | 52 + 7 (293) | `🧱️channel-record` 85, `📬️pending-exchanges-and-agent-ops` 184, `💰️budgets-and-limits` 89, `📂️documents-and-backbone` 264, `💡️inference-route-and-actions` 180, `🧩️instances` 136, `🔁️exchange-turns` 293 |
| gwb | `🏠️workspace/🛂️gateway-backend` | 1,206 | 81 + 7 (264) | `🏗️open-and-discovery` 264, `🔬️probe-documents` 97, `🔌️plugin-activation-and-bindings` 206, `🌐️hub-session-documents` 245, `🧩️artifact-creation-and-routing` 116, `🏛️gateway-backend-impl` 208, `🪪️arc-delegate` 39 |
| wsq | `🏠️workspace/🧪️tests/🔬️quick` | 1,098 | 64 + 10 (183) | `📡️hub-relay-and-workspace-guards` 173, `🔐️authenticated-hub-workspace` 136, `🤖️headless-agent-and-pending-exchanges` 124, `📂️folder-workspace-and-probe-documents` 82, `🧭️plugin-capability-routing` 183, `📖️artifact-resources-and-probe-edits` 64, `🧵️document-backbone-egress` 60, `🗂️guest-document-codec` 51, `🔗️inference-artifact-binding` 89, `🏁️child-heads-hub-commit-and-probe-pack` 102 |
| inf | `💡️inference` | 1,318 | 84 + 9 (250) | `🔖️declared-inference` 175, `🪝️execution-and-job-seams` 88, `🛠️tools-and-resources` 135, `🔐️service-resolution-and-authorization` 137, `📤️submit-and-guest-jobs` 150, `📡️events-cancel-and-approval` 164, `✅️request-validation` 126, `🏃️run-handler` 250, `🔭️job-tool-registration-and-hub-read` 66 |
| cat | `🗂️catalog` | 1,430 | 50 + 7 (288) | `🪪️capability-identity` 229, `🧰️presentation-and-definition` 142, `♻️framework-dedup` 112, `🏭️capability-builders` 288, `📚️catalog-compile` 202, `🔍️catalog-audit` 228, `💬️description-law` 190 |
| sch | `🧬️schema` | 1,288 | 63 + 8 (237) | `🧭️core-and-mutation-tool-schemas` 162, `🛡️untrusted-content-schemas` 136, `📄️artifact-tool-schemas` 166, `🖥️ui-and-catalog-tool-schemas` 214, `💡️inference-tool-schemas` 130, `📡️artifact-inference-wire` 100, `📚️schema-catalog` 237, `🧾️scope-exports-and-validation` 117 |
| ses | `🧵️bridge/🪢️session` | 1,232 | 47 + 7 (320) | `🆔️connection-identity-and-limits` 79, `📢️broadcast-cursor` 106, `⏳️async-authority` 320, `🎞️encoded-frames` 188, `📮️outbox` 193, `🤝️handle` 281, `💬️agent-conversation` 73 |

Totals: 8,789 lines in 7 files became 441 lines in 7 roots plus 55 children. Children sit next to their root as siblings, e.g. `M/💡️inference/🏃️run-handler/🦀️.rs`. No new crate, test target or command was added, so no manifest, build-owners row or launch route changed.

Specific handling (each verified in the staged and live text):
- **Regions.** A region that spans several children gets a balanced marker pair in each child that holds a part; the parent keeps the empty `🧪️Tests` / `💼️Jobs` / `🧪️InferenceJobTests` pairs (`keep_empty`). Original pairs per file (pac, gwb, wsq, inf, cat, sch, ses): 1, 0, 3, 11, 20, 15, 2; all balanced in every new file.
- **Widening.** Lexical, name-based `pub(super)` insertions for private items, fields and inherent members whose name occurs in another file of the split (`widen2.py`, conservative): 22 (pac), 0 (gwb: all members were already `pub`/`pub(crate)`), 29 (inf), 10 (cat), 5 (sch), 20 (ses). Trait-impl methods are never widened; no qualified path was used to fix visibility. The three original `pub(super)` declarations of `pac` (two fields and `PluginInferenceRoute`) are consumed only inside the module, so their narrower meaning in the child (parent module) is enough (checked by search; no other file of the crate names them).
- **`include_str!` re-based.** One more `../` on the 10 relative literals of `cat` (1) and `sch` (9, spread over the children): every absolute target equals the original (`incl_mod.py`).
- **`sch` rust leaf.** `LEAVES.rust` was `include_str!("🦀️.rs")`, i.e. the registry embedded the file's own source. It is now `concat!(include_str!("../🦀️.rs"), include_str!("../<child>/🦀️.rs"), ..., include_str!("🦀️.rs"))` over the root and all eight children (the repo's existing idiom for split schema roots), so the published Rust leaf still holds every exported `pub fn`.
- **`sch` core child.** `use super::*;` is omitted in `🧭️core-and-mutation-tool-schemas` (`glob=False`): it uses no name of the parent and the glob would be an unused import (seen in the first oracle run).
- **`gwb` macro.** `dyn_enum_close!` stays in the parent (an external macro expansion defines the public enum `GatewayBackends`; leaving it in place keeps the public item where its imports are).
- **Module names.** `sch` child `untrusted_content_schemas` (a child `untrusted_content` would have shared a name with `pub fn untrusted_content`); folder emoji unique among siblings (existing siblings of `💡️inference`: `💼️ 🔌️ 🧪️ 🧫️ 🧬️ 🧰️ 🧾️`) and different from each root's emoji; all RGI and already used as folder prefixes (taxonomy count at least 2).

## 3. Source-text readers

`git grep -F` over the repo (excluding tickets) for each file path, each folder name and the `🌉️mcp` path with `🦀️.rs`; plus the registry readers of the module tree.

| reader | before | after |
| --- | --- | --- |
| `M/🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🟦️.ts` (two tests, lines 119 and 125) | `readFileSync(new URL(".../🌉️mcp/💡️inference/🦀️.rs", import.meta.url), "utf8")` | `readModuleTreeSource(new URL(...))` (import already present) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` `POLICY_SPLIT_SOURCE_ROOTS` | listed the bridge root, not the session file | added `.../🌉️mcp/🧵️bridge/🪢️session/🦀️.rs` (the interactivity audit reads it through `policyReadRustPolicySource`) |

Probes: `c4p44-tools/probe-tree.ts` (module-tree reads of 6 files: 8, 8, 10, 8, 9, 8 files; the four needles of the inference tests are true and the two negative needles hold), `c4p44-tools/probe-audit.ts` (`interactivityMcpHttpTransportFailures` with the new session tree text and with the snapshot text: 30 failures each, **equal**; the 30 are pre-existing), and `bun test ./🧪️tests/🔬️unit/🟦️.ts -t "original MCP"` in the wasm extension: the inference authority test passes (it reads the module tree); the second test fails at `expect(validate(source)).toBe(true)` (schema validation, before any Rust read, so not caused by this slice; its Rust needles are covered by the probe).

Not readers: `taxonomy.json` (folder names only), `🗂️schema-document-index.json` and `🔣️schema-catalog.json` (JSON documents, not the Rust file), `📋️project.json` inputs (`**/🦀️.rs` glob), the descriptor-digest fixture (`rustReceivers` lists `🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`, which is only tree-sitter parsed and still parses), docstring mentions in `🗂️catalog/🟦️.ts`, `🧬️schema/🟦️.ts`, `📜️script.ts`, `🔄️hub-agent-participant`. No reader of `🛂️gateway-backend`, `📨️plugin-artifact-channel` or `🗂️catalog/🦀️.rs` text exists. No compatibility re-export: parents hold `#[path] mod x; <vis> use x::*;` wiring and genuinely own items only.

## 4. Verification (commands, real results)

Tools in `F/c4p44-tools/`, logs in `F/🗑️generated/c4p44-*`; all long runs used `ulimit -n 65536` and `nice -n 10`.

| Check | Command | Result |
| --- | --- | --- |
| Install precondition and apply | `python3 c4p44.py install <tag>` (six tags), `cd frag && python3 install.py check; apply` | each live file equalled its snapshot; `installed pac gwb inf cat sch ses`; frag `children 10, roots 1, problems 0`, `installed 11 files` |
| Lossless (live, six module files) | `python3 live.py pac gwb inf cat sch ses` (`c4p44-live.log`) | 6 of 6 `bytes-equal-to-stage` and `LOSSLESS OK`: missing 0; every extra line is a docstring, `use super::*;`, `#[path]`/`mod`/`pub use` wiring, a region marker, a declared impl header/brace line or the declared `LEAVES` line |
| Lossless (live, wsq) | `cd frag && python3 lossless.py; python3 verify.py live` (`c4p44-frag-live.log`) | `gone=0 extra=0 sequence-equal=True`; `VERIFY OK live`: test fns 56 = 56 identical, regions balanced, docstring emoji unique, rustfmt parse clean, each fragment included once |
| Structure | `python3 check.py pac gwb inf cat sch ses` (`c4p44-check-live.log`) | `CHECKS OK`: every child mounted exactly once, doc emoji equals folder emoji, unique among siblings and different from the root's, region counts equal, no `[DEBUG]`, longest child 320 lines, longest path 166 bytes |
| Mount scan | inline Python over `M/**.rs` | 55 of 55 folders named exactly once |
| Private declarations | `python3 defscan.py <tag> <live dir>` (`c4p44-defscan-live.log`) | 0 cross-file leftovers for all six (6, 0, 23, 16, 7, 23 private declarations scanned); `modclash OK` |
| Include targets | `python3 incl_mod.py` (`c4p44-incl-mod.log`) | every original target present; only difference is the declared eight own-tree entries of the `sch` concat; `cd frag && python3 incl.py`: `INCLUDES OK` |
| rustfmt | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout` over the staged files | 0 `error` lines |
| Taxonomy, RGI | `bun tax.ts newdirs.txt`; `bun rgi.mjs newnames.txt` | `dirs 55, unresolved []`; `bad=0` |
| Scoped patched differential, staged | `C4P44_NORM=1 python3 oracle.py all all` (`c4p44-oracle-stage{1,2,3}.log`) | stage1 showed three textual or glob artefacts (an `unused import` of an empty-by-oracle glob for the `dyn_enum_close!` child, an unused `super::*` in the `sch` core child, a printed type path), fixed or normalised; stage3: `ORACLE all CLEAN` |
| Scoped patched differential, live (final) | `C4P44_LIVE=1 C4P44_NORM=1 python3 oracle.py all all` (`c4p44-oracle-live.log`) | `ORACLE all CLEAN`: 24 of 24 comparisons (pac lib/test; gwb, inf, cat, sch, ses lib/test/wasm/wasmtest; wsq lib/test), 0 only-orig, 0 only-new, warnings identical; per-crate error counts equal (e.g. pac 512/561, gwb 160/209/105/155, wsq 132/483, inf 175/295/174/294, cat 104, sch 259, ses 24/31/24/31) |
| Census (canonical `readRustSourceClosureV1`) | `bun census-own.ts <root>` (pre 14:3x on the unsplit tree, post live); `zsh census.sh post` (`census.ts` of `F/🗑️generated`) | 0 of 7 over the gate; see the table below; canonical `census.sh post` agrees with the own rows (lib) |
| Wiring budget law | `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` (library module) | 2 pass, 1 fail; the three breaches are plugin crates of peers (`🌊️flow` 783 to 790, `🎬️sequence` 275 to 276, `✒️writer` 688 to 692, `🏙️bim` 440 over 150), none in `🌉️mcp` |

Census (files / bytes / active lines; ratio = worst of files/32, bytes/262,144, lines/4,000; before = `cgc7-crate-census.csv` and the pre run of `census-own.ts`, after = live):

| crate | lib before | lib after | cfg(test) before | cfg(test) after |
| --- | --- | --- | --- | --- |
| plugin-runtime (pac) | 6 / 187,261 / 2,811 (0.714) | 13 / 190,230 / 2,884 (0.726) | 7 / 198,351 / 2,867 | 14 / 201,320 / 2,940 (0.768) |
| workspace (gwb) | 5 / 119,598 / 1,704 (0.456) | 12 / 121,610 / 1,761 (0.464) | 6 / 158,058 / 2,306 | 13 / 160,070 / 2,363 (0.611) |
| os-mcp glue (wsq) | 3 / 85,420 / 1,344 (0.336) | 3 / 85,420 / 1,344 (0.336) | 8 / 228,816 / 3,461 (0.873) | 18 / 231,199 / 3,501 (0.882) |
| tools-inference (inf) | 3 / 100,401 / 1,576 (0.394) | 12 / 103,814 / 1,642 (0.410) | 5 / 146,955 / 2,339 | 14 / 150,368 / 2,405 (0.601) |
| catalog (cat) | 8 / 123,870 / 2,130 (0.532) | 15 / 125,957 / 2,148 (0.537) | same as lib | same as lib |
| schema (sch) | 5 / 165,236 / 1,294 (0.630) | 13 / 168,545 / 1,339 (0.643) | same as lib | same as lib |
| bridge (ses) | 3 / 61,604 / 1,298 (0.325) | 10 / 64,012 / 1,360 (0.340) | 4 / 87,855 / 1,862 | 11 / 90,263 / 1,924 (0.481) |

Bytes bind for pac, gwb, wsq and sch; the highest file count is 18 of 32 (wsq test). The glue test closure grew from 0.873 to 0.882 of the byte limit; nothing crosses a limit.

## 5. Full-crate typeck oracle (extra evidence)

`posrun.py` (the privacy and type oracle `F/pos-tools/pos.py` with a stub layer for everything outside the family, sandbox `/Users/ueli/.cache/c4p44/pos-work`), family `families/c4p44.json` = the seven crates, no base revision: `pre` ran on the unsplit live tree before the install and `post` after it. Commands: `cd F/c4p44-tools && ulimit -n 65536 && PYTHONHASHSEED=0 POS_PREFIX=c4p44 python3 posrun.py run families/c4p44.json --cfgs lib,test,wasm --label {pre,post} --no-base` (`c4p44-pos-pre.log`, `c4p44-pos-post.log`, a few minutes each, two iterations), then `python3 posdiff.py c4p44-pre c4p44-post lib test wasm` (`c4p44-posdiff-1.log`).

| cfg | errors pre / post | privacy | trait-scope | resolve | type | other | canary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| lib | 3,018 / 3,016 | 0 / 0 | 0 / 0 | 13 / 13 | 55 / 54 | 0 / 0 | `E0308 E0616 E0624` reached on both sides |
| test | 4,669 / 4,667 | 0 / 0 | 0 / 0 | 15 / 15 | 122 / 121 | 0 / 0 | reached |
| wasm | 1,917 / 1,915 | 0 / 0 | 0 / 0 | 8 / 8 | 36 / 35 | 0 / 0 | reached |

So no new `E0603`, `E0616`, `E0624` or unresolved name is introduced. The row delta (16 rows only in `post` and 18 only in `pre`, identical counts in lib, test and wasm) is stub-layer emulation, all about two stubbed external traits: in `pre` the stub knows `GatewayBackend` and `FromValue` as traits at their `impl ... for` sites and reports their stub-signature consequences (E0050, E0061, E0277, E0308 on the `from_value` calls, E0407 naming the trait); in `post` the `impl` sites sit in children that reach the parent's `use` line only through `use super::*`; consistent with a per-file trait-position scan in the stub layer (not verified inside the tool), the stub then types them as the placeholder `Tr` and reports E0404/E0407/E0576 at the same sites (children `🏛️gateway-backend-impl`, `🪪️arc-delegate`, `🪪️capability-identity`) and one fewer E0277 or E0308 type row. This is the same class of artefact PKG-37 recorded (stub-layer rows). Not covered: borrowck, trait coherence against the real externals, lints, and the cfg-gated wasm session of other crates; the real check in section 8 covers them.

## 6. Foreign errors and observations

- `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts`: breaches only in plugin crates of peers (section 4).
- The interactivity audit MCP bridge law returns 30 failures on the unsplit and on the split text alike (pre-existing; the probe shows the lists equal).
- The wasm extension test "original MCP source requires the exact retained turn and work identity" fails at its first schema `validate(source)` assertion (unrelated schema or fixture drift in `InferenceRunInput`).
- `git` printed `daemon terminated` on several read-only calls (fsmonitor noise). No `ENFILE`/`EMFILE` or Bun segfault occurred. Two long shell calls were moved to the background by the tool timeout and finished normally.
- `wsq` and `cat` differed from `HEAD b84bf526c7b` by peer edits that are part of the snapshots (`git hash-object` differs; the snapshots are the working-tree files).

## 7. Files

- Edited (7 roots): `M/🏠️workspace/📨️plugin-artifact-channel/🦀️.rs`, `M/🏠️workspace/🛂️gateway-backend/🦀️.rs`, `M/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`, `M/💡️inference/🦀️.rs`, `M/🗂️catalog/🦀️.rs`, `M/🧬️schema/🦀️.rs`, `M/🧵️bridge/🪢️session/🦀️.rs`.
- Created (55 children, one `🦀️.rs` per new folder): the folders of section 2, next to their roots.
- Edited outside the package (readers, section 3): `M/🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🟦️.ts` (2 expressions) and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` (1 line).
- Tools, kept: `F/c4p44-tools/` (`c4p44.py` splitter, `specs/*.py`, `frag/` fragment pipeline with `specs/1.json`, `oracle.py`, `live.py`, `check.py`, `defscan.py`, `modclash.py`, `globuse.py`, `incl_mod.py`, `census.sh`, `census-own.ts`, `tax.ts`, `rgi.mjs`, `probe-tree.ts`, `probe-audit.ts`, `posrun.py`, `posdiff.py`, `families/c4p44.json`, `newdirs.txt`, `newnames.txt`, `installed.json`, `orig/` snapshots with `SHA256`). Do not re-stage after install: the specs read the original snapshots, but `install` refuses when the live file differs from them.
- Logs, kept in `F/🗑️generated/`: `c4p44-oracle-{stage1,stage2,stage3,live}.log`, `c4p44-static-stage.log`, `c4p44-install.log`, `c4p44-live.log`, `c4p44-check-live.log`, `c4p44-frag-live.log`, `c4p44-defscan-live.log`, `c4p44-incl-mod.log`, `c4p44-tax-stage.log`, `c4p44-probe-tree.log`, `c4p44-probe-audit.log`, `c4p44-census-own-{pre,post}.log`, `c4p44-census-post.log`, `c4p44-pos-{pre,post}.log`, `c4p44-posdiff-1.log` with the `c4p44-c4p44-*` diagnostic dumps.

## 8. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md` at 15:00 and no `bun repo cargo` call was made by this slice. One batched call under the compile mutex of `📓️WAVE5-BRIEF.md` (framework workspace members need the manifest path):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-mcp-plugin-runtime -p semio-framework-os-mcp-workspace -p semio-framework-os-mcp -p semio-framework-os-mcp-tools-inference -p semio-framework-os-mcp-catalog -p semio-framework-os-mcp-schema -p semio-framework-os-mcp-bridge --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```

then `--target wasm32-unknown-unknown` for `-p semio-framework-os-mcp-workspace -p semio-framework-os-mcp-tools-inference -p semio-framework-os-mcp-catalog -p semio-framework-os-mcp-schema -p semio-framework-os-mcp-bridge --lib`. If a build shows a missing `pub(super)`, widen that single definition line in the child (never with a qualified path).
