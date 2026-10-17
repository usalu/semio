# Result CENSUS4-PKG-35 (prefix `c4p35`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `M` = `🧰️framework/🔨️modules`. Tools: `F/c4p35-tools/` (descendant of `c4p34-tools`; snapshots in `orig/`, hashes in `SHA256.txt`). Logs: `F/🗑️generated/c4p35-*` (nothing deleted). Stable sandbox/stage root (survives a scratchpad wipe): `/Users/ueli/.cache/c4p35/` (`stage/`, `sbx/<tag>/{orig,new}`, `orc/`).

## 0. Verdict

**DONE (static evidence; the real `bun repo cargo` check is queued).** Last update 2026-10-17 08:59.

- All 8 files of Package 35 (9,835 lines) are split: 8 wiring roots (12 to 242 lines) plus 37 children (largest 453 lines). Lossless by line multiset, region markers balanced per file, test names unchanged (62 = 62), `rustfmt` parses all 45 files, the scoped patched differential is identical for all eight crates in every configuration, definition scan is clean, taxonomy resolves 37/37, every crate stays well inside the source gate.
- Three source-text readers were re-pointed (section 4): two lines in `POLICY_SPLIT_SOURCE_ROOTS`, the replication ownership law (`consumers`) and the replication `📜️script.ts` presence oracle. The ingress ownership law needed no edit because the ingress root keeps its 14 public types and 5 constants.
- No `COMPILE ADMITTED` line in `📓️FLEET.md` at 08:58 (the fleet mutex `🗑️generated/cargo-mutex.lock` has been held by a peer since 08:42), so the real check is recorded as a command (section 7).

## 1. Found state (fresh slice, 2026-10-17 08:00)

- No earlier c4p35 work: no `c4p35-tools`, no `🗑️generated/c4p35-*`, no RESULT file.
- The 8 files were byte-identical to `HEAD b84bf526c7b` (git blob hashes equal) and had the census line counts. Pre-edit snapshots: `F/c4p35-tools/orig/<tag>.rs`, `F/c4p35-tools/SHA256.txt`. The installer asserted per target that the live file still equalled its snapshot.

| tag | file (tail under `M/`) | lines before | SHA256 (first 10) | owning crate |
| --- | --- | ---: | --- | --- |
| fmt | `🎒️pack/📐️format/🦀️.rs` | 1,376 | `aa73267105` | `semio-framework-pack-format` (lib root) |
| jtu | `🎒️pack/🔤️json/🧪️tests/🔬️unit/🦀️.rs` | 1,079 | `7dcda7eacd` | `semio-framework-pack-json-tests` (`mod tests` of the companion lib root via `#[path = "../🔬️unit/🦀️.rs"]`) |
| onb | `🎓️onboarding/🦀️.rs` | 1,313 | `65f04944b7` | `semio-framework-onboarding` (lib root) |
| mut | `📡️replication/🎮️mutation/🦀️.rs` | 1,248 | `97b338010d` | `semio-framework-replication-mutation` (lib root) |
| ing | `📡️replication/📡️wire/🎮️command/📥️ingress/🦀️.rs` | 1,115 | `ac7b7123dd` | `semio-framework-replication-command-ingress` (lib root) |
| prs | `📡️replication/📡️wire/👥️presence/🦀️.rs` | 1,314 | `a18bd51acd` | `semio-framework-replication-presence` (lib root) |
| wire | `📡️replication/📡️wire/🦀️.rs` | 1,051 | `ea8f699221` | `semio-framework-replication-frames` (lib root) |
| exp | `🔄️machine/✨️derive/⚙️expansion/🦀️.rs` | 1,339 | `9a3b3cc0be` | `semio-framework-machine-derive` (proc-macro, `mod component` of its lib root) |

## 2. What was cut (before -> after, children in lines)

| tag | before | root after | children |
| --- | ---: | ---: | --- |
| `fmt` | 1,376 | 62 | 🧮️controlled-allocation 95, 🧱️header-and-footer 174, 🧩️segments-and-symbols 222, 📜️chunk-table-and-manifest 165, ✍️pack-writer 296, 📖️pack-reader-and-recovery 414 |
| `jtu` (include fragments) | 1,079 | 12 | 🧯️retirement-laws 198, 📐️grammar-laws 192, 🌉️value-bridge-and-macro-laws 132, 🎲️property-and-differential-laws 301, 🫳️borrowed-source-laws 258 |
| `onb` | 1,313 | 45 | 🎬️introduction-definition 132, 🖱️introduction-interaction 357, 📚️tutorial-definition 213, 🎞️tutorial-events-and-ui 321, 🎥️tutorial-engine 283 |
| `mut` | 1,248 | 183 | 📋️mutation-metadata-enums 135, 🪪️leaf-descriptor-validation 368, 🔎️leaf-source-scope 246, 🚦️outcome-and-message 349 |
| `ing` | 1,115 | 240 | 📄️command-page-behavior 115, 🗃️page-set-and-reader-behavior 202, 📨️envelope-and-rejection-behavior 191, 🧰️driver-registry-behavior 181, 🚚️batch-driver-behavior 229 |
| `prs` | 1,314 | 68 | 🪟️view-records-and-codec 256, 👤️peer-records 453, 📤️peer-wire-encode 174, 📥️peer-wire-decode 243, 🕹️interaction-and-domain 174 |
| `wire` | 1,051 | 97 | 🥾️artifact-bootstrap 346, 🛰️server-frame 185, 🧵️codec-combinators 218, 🔌️frame-codec 239 |
| `exp` (inline modules to file modules) | 1,339 | 242 | 🔬️analyze 365, 🏗️codegen 294, ✂️parse 443 |

Total 9,835 lines before; after 10,108 lines: 949 in the eight roots plus 9,159 in the 37 children (the growth of 273 lines is wiring, docstrings and duplicated region markers). Each child sits next to its root as `<emoji>️<kebab>/🦀️.rs`; the longest repo-relative path is 142 UTF-8 bytes (limit 240).

### Mechanism and decisions

- **Item-level splitter** (`c4p35.py`, descendant of `c4p34.py`): whole items (with attributes and docs) move; each child starts with a `//! <emoji> ... See [`super`].` docstring and `use super::*;` (dropped where the child uses nothing from the root: `mut` metadata enums, `wire` artifact bootstrap), the root keeps its `use` lines and the `#[path = "<folder>/🦀️.rs"] mod x;` plus `use x::*;`/`pub use x::*;` at the position of the first moved item. Re-exports are omitted where unneeded (`ing` command-page-behavior). Existing test, borrowed and peer-overlay mounts stay in their roots, unchanged. No qualified paths were introduced, no compatibility re-export or alias.
- **Regions**: items moved with the region stack they sat in; every child is balanced, empty region pairs in the root were pruned. Region markers 118 -> 156 in total (orig 118 = `#region` plus `#endregion` lines of the eight files; the extra ones are the re-opened regions in children and the two completed `Entry` markers; every file is balanced).
- **Visibility**: `widen2.py` makes a moved private item, field or inherent method `pub(super)` when its name occurs in another file of the split (siblings, root, mounted tests); `fmt` 58 lines, `onb` 1, `mut` 11, `ing` 1, `prs` 23, `wire` 18. `jtu` and `exp` widen nothing (include fragments and file modules keep the module identity).
- **`ing` keeps the public type surface in its root** (14 pub types and 5 pub constants, in the order of `🧪️tests/🧫️fixtures/🔣️.json` `exports`); children hold only the `impl` blocks and the one private fn. Reason: `ingress/🏛️ownership/🧪️tests/🟦️.ts` parses the root AST and asserts `publicNames == exports`, mutates the root text for its hostile cases and checks the test mount, so the root is the declared "public neutral ownership" (section 4). `trait Mutation` also stays in the `mut` root (a `syn` test and the foreign-output law read it there).
- **`exp`**: the three inline modules `mod analyze/codegen/parse { .. }` became file modules with identical module paths (body dedented by four, their inner `//!` doc is the file doc). Two orphan markers that existed before were completed so each file is balanced: an `//#endregion 🔖️Entry` without opener at the end of `analyze` got its `//#region 🔖️Entry` before `pub fn analyze`, and the root's `//#region 🔖️Entry` (never closed) got its `//#endregion 🔖️Entry` before `//#region 🧪️Tests`. These are the only two added lines that are not wiring or docstrings.
- **`jtu`** (test file, 62 `#[test]` fns): five `include!` fragments spliced back into the same module so test paths, visibility and `use super::*;` scope are unchanged. Fragment cuts follow region boundaries (`FloatParity` region ends at line 822 and moved with `PropertyTesting`). Each fragment starts with an outer `///` line whose first character is the unique emoji. The 17 existing `[DEBUG]` eprintln lines are untouched (count 17 = 17).

## 3. Verification (commands and real results)

| Check | Command | Result |
| --- | --- | --- |
| Lossless line multiset | `python3 c4p35-tools/live.py` (`c4p35-lossless-live.log`) | `LOSSLESS OK` for all 8, `missing 0`; every file byte-equal to its stage; extras are only wiring (`#[path]`, `mod`, re-export, `use super::*`, docstring, `include!`, markers) |
| Structure | `python3 c4p35-tools/check35.py live` (`c4p35-check-live.log`) | `CHECKS OK`: each child mounted once, first line is a doc with the folder emoji, unique among siblings and different from the root emoji, no clash with existing sibling folders, emoji used as folder prefix in the repo (771-emoji scan), regions balanced per file, test fn multiset equal (jtu 62 = 62), DEBUG count equal (17 = 17), longest child 453 lines, longest path 142 bytes |
| rustfmt parse | `python3 c4p35-tools/fmt.py live` | 45 files parsed, 0 failures (a deliberately broken file does fail) |
| Scoped patched differential, stage and live | `python3 c4p35-tools/oracle.py <tag>` (stage) and `C4P35_LIVE=1 python3 c4p35-tools/oracle.py <tag>` (live tree), logs `c4p35-oracle-stage-*.log`, `c4p35-oracle-live-*.log` | `ORACLE <tag> CLEAN` for all 8 on the live tree: errors and warnings identical on lib, `--test`, wasm32-wasip2 lib and test (fmt additionally with `feature="deflate"`, exp native only). Error counts orig = new: fmt 59/98/59/98/65/104, onb 273/378, mut 193/240, ing 209/221, prs 370/463, wire 184/259, jtu 1/1334, exp 144/160. Patches as in `📓️RESOLUTION-ORACLE.md` (extern crate lines removed, external glob imports replaced, sandbox paths and the random `/dev/rmeta` temp name normalised) |
| Oracle sensitivity (negative control) | removed `pub(super)` from `controlled_vec` in the fmt sandbox, `C4P35_REUSE=1 oracle.py fmt lib,test` | `+3x E0425 cannot find function controlled_vec` in both configs, `ORACLE fmt DIFFERENCES` (restored afterwards) |
| First-round findings fixed before the install | stage runs | `mut` and `wire`: new `unused import: super::*` (children that use nothing from the root: `glob=False`); `ing`: new `unused import: command_page_behavior::*` (`reexport=False`); `prs`/`wire`: sandbox path text in `couldn't read` messages (normalised); `jtu` lib: the env error `couldn't create a temp dir /dev/rmeta*` appears when a configuration has no error at all (normalised) |
| Definition scan | `python3 c4p35-tools/defscan.py <tag>` | 0 cross-file leftovers for fmt (6 declarations), onb (8), mut (20), ing (6), prs (10), wire (15). For `jtu` and `exp` the scan reports same-module items; it is not applicable there (include fragments and file modules keep one module), the differential covers them |
| Module name clash | `python3 c4p35-tools/modclash.py fmt onb mut ing prs wire` | `modclash OK` |
| Taxonomy | `bun c4p35-tools/taxcheck.ts c4p35-tools/newdirs.txt` (`c4p35-taxonomy.log`) | `checked=37 unresolved=0` (32 `crate-module-children`, 5 `members-of-tests`) |
| RGI emoji | `node c4p5-tools/rgi.mjs c4p35-foldernames.txt` | 37 `RGI`, `bad=0` |
| Census (canonical `readRustSourceClosureV1`, `census.sh`, lib) | `c4p35-tools/census.sh before` / `after` (`F/🗑️generated/census.ts`) | machine-derive 2 files / 55,507 B / 1,372 lines -> 5 / 51,697 / 1,380; onboarding 1 / 69,649 / 1,310 -> 6 / 71,390 / 1,353; pack-format 2 / 96,471 / 1,378 -> 8 / 98,893 / 1,436; command-ingress 1 / 56,780 / 1,114 -> 6 / 58,384 / 1,162; frames 1 / 52,389 / 1,050 -> 5 / 53,779 / 1,088; mutation 4 / 108,411 / 1,802 -> 8 / 109,813 / 1,839; presence 3 / 95,564 / 1,684 -> 8 / 97,495 / 1,743; pack-json (not touched) 9 / 228,065 / 3,381. Max ratio of the touched crates 0.460 (mutation); none over the gate |
| Own-closure census, lib and cfg(test), incl. the companion crate (`census-own.ts`, sandbox root for "before") | `bun c4p35-tools/census-own.ts <root>` (`c4p35-census-own-{before,after}.jsonl`) | e.g. companion test closure 16 files / 216,317 B / 2,459 lines -> 21 / 217,566 / 2,478; presence test 7 / 149,206 / 2,369 -> 12 / 151,137 / 2,428; frames test 4 / 83,114 / 1,517 -> 8 / 84,504 / 1,555; mutation test 14 / 164,458 / 2,230 -> 18 / 165,860 / 2,267; fmt test 5 / 128,045 / 1,915 -> 11 / 130,467 / 1,973; all far below 32 files / 4,000 lines / 262,144 bytes |
| Replication partition laws | `bun test ./M/📡️replication/🧪️tests/🏛️crate-partition/🟦️.ts -t ...` (three runs, `c4p35-law-crate-partition-*.log`) | "exactly one owning crate" 1 pass; "foreign-trait impl" 1 pass; "source budget" (all targets and the deflate feature) 1 pass |
| Replication IO ownership law | `bun test ./M/📡️replication/🚪️io/🧪️tests/🏛️ownership/🟦️.ts` | before 21 pass 0 fail; after the re-pointed `consumers` read 21 pass 0 fail |
| Ingress ownership law | `bun test ./M/📡️replication/📡️wire/🎮️command/📥️ingress/🏛️ownership/🧪️tests/🟦️.ts` | before 1 pass 3 fail, after 1 pass 3 fail, the two logs are identical after removing timings (the 3 failures are inherited, section 6) |
| Policy evidence law | `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence/🟦️.ts` | 6 pass 0 fail, including "every declared split source root names a real file that reads as one text" (now covers the two new roots) |
| Policy reader text equals the unsplit text | `bun c4p35-tools/policy-read.ts <root> <snapshot>` | fmt: 58 lines missing = exactly the 58 widened lines (their `pub(super)` twins are present); wire: 18 missing = the 18 widened lines; extras are the appended test evidence and wiring |
| Presence oracle source read | `bun /Users/ueli/.cache/c4p35/probe-presence.ts` | tree text contains `PRESENCE_PEER_WIRE_LIMITS_V1` and `reader.position != bytes.len()`; the root alone contains neither (so the re-point is needed) |

No tool died with `ENFILE`/`EMFILE` or a Bun segfault; the machine only made several foreground calls slow enough that the harness moved them to the background (no result was lost, the logs were read from disk). The `COMPILE` check was not attempted (no admission).

## 4. Source-text readers

Search method: `git grep -F` of each root's path suffix over `.ts/.tsx/.rs/.py/.mjs/.sh/.json/.toml/.md` (log `c4p35-readers-grep.txt`), a recursive scan of the target trees for relative `🦀️.rs` reads, and a read of every candidate. Findings:

| Reader | What it reads | Action |
| --- | --- | --- |
| `📚️library/🔍️discovery/📖️source-access/🟦️.ts` `POLICY_SPLIT_SOURCE_ROOTS` | policy evidence readers (`policyReadRustPolicySource`, `policyReadRustSourceEvidence`) read a root as one text only if the root is listed. The interactivity audit (`⚖️laws/🔬️interactivity-audit/🏁️run`, `🎛️commands/✅️verify` `runInteractivityP1q/R4`) reads the pack format root (`INTERACTIVITY_AUDIT_PACK_FORMAT_FILE`) and `verify` p1z reads the frames root | **edited**: added `🎒️pack/📐️format/🦀️.rs` and `📡️replication/📡️wire/🦀️.rs` after the `⏳️async` entry (2 lines; the peer lines of that file are not mine) |
| `M/📡️replication/🚪️io/🧪️tests/🏛️ownership/🟦️.ts` (causal consumers, `fixture.causal.consumers = ["📡️wire/🦀️.rs"]`) | regex `semio_framework_replication_causal::binary::(encode\|decode)_` on the wire root; after the split the calls live in `🧵️codec-combinators` and `🔌️frame-codec` | **edited**: `readFileSync(...)` -> `readModuleTreeSource(...)` plus the import `@semio-tech/framework-compiler-ts/syntax/module-tree` (precedent: `🎒️pack/🌱️value/🧪️tests/🧱️ownership`); 21 pass before and after |
| `M/📡️replication/📦️packages/🦀️rust/📜️script.ts` presence oracle (line 391) | `source.includes("PRESENCE_PEER_WIRE_LIMITS_V1") && source.includes("reader.position != bytes.len()")` on the presence root | **edited**: `readModuleTreeSource(join(this.root, "../../📡️wire/👥️presence/🦀️.rs"))` plus a relative import of the module-tree reader (the script cannot be run directly: `Original parent Script process invocation required`; the probe above proves the two strings) |
| `M/📡️replication/📡️wire/🎮️command/📥️ingress/🏛️ownership/🧪️tests/🟦️.ts` | AST of the ingress root: `publicNames == exports` (14 types, 5 constants, in order), constant token values, the `unit_tests` mount path, and hostile text edits (`component.replace("pub struct FixedCommandPage", ...)` and similar) | **no edit**: the root keeps those 14 types and 5 constants (verified: `pub (struct\|enum\|type\|const)` list equals the fixture), the test mount stays; law output identical before and after |
| `M/📡️replication/🧪️tests/🏛️crate-partition/🟦️.ts` | follows `#[path]` and `include!("..rs")` for ownership, budget and foreign-trait laws | **no edit** (3 runs pass) |
| `🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🪪️mandatory-mutation-descriptor/🦀️.rs` | `syn` parse of the mutation root: exactly one `pub trait Mutation` with `DESCRIPTORS` and `descriptor` | **no edit**: `trait Mutation` stays in the `mut` root |
| `M/📡️replication/🎮️mutation/🌐️foreign/🧪️tests/🟦️.ts` | regexes `fn foreign_step_source<'a>` (present) and `fn foreign_steps(` (absent) on the mutation root | **no edit**: `foreign_step_source` is a `Mutation` trait method and stays in the root |
| `M/📡️replication/🚪️io/🧪️tests/🏛️ownership/🟦️.ts` (negative regexes on the mutation root) | `doesNotMatch(semantic, /pub trait X\|pub use[^;]*X/)` | **no edit**: only becomes easier after the split |
| `pack/⚠️error/🧪️tests/🧫️fixtures/🧭️cause/🔣️.json` | `source.path`/`sha256`/`predicate` of the pack format root | **no edit**: no test reads it (grep of the whole tree finds no consumer of `sourcePredicate`); its `sha256` (`9784b9...`) was already stale before this slice (the file hashes to `aa73267...`) |
| `🕹️interaction/🧪️tests/🧫️fixtures/🎯️merge-modes.json` (`wiring.codec.file = ...📡️wire/🦀️.rs`, anchors `pub fn wire_label`) | stale anchor: `wire_label` is not in the frames root before or after | **no edit** (not a live reader of this file) |
| Doc mentions, `.protocol.semio` comments, `crate-partition`/`ownership` fixture paths, `🧱️framework-source-topology` (`implementation` file only needs name and > 10 lines; exp root has 242), manifests, build owners | paths only | **no edit** |

No other reader exists for `onb`, `prs` (besides the script), `jtu` or `exp`. No test or fixture file name changed; no `launch.json`, registry, manifest or lock edit was necessary (no crate, test target or command added or renamed).

## 5. Files

- Edited (8 targets): `M/🎒️pack/📐️format/🦀️.rs`, `M/🎒️pack/🔤️json/🧪️tests/🔬️unit/🦀️.rs`, `M/🎓️onboarding/🦀️.rs`, `M/📡️replication/🎮️mutation/🦀️.rs`, `M/📡️replication/📡️wire/🎮️command/📥️ingress/🦀️.rs`, `M/📡️replication/📡️wire/👥️presence/🦀️.rs`, `M/📡️replication/📡️wire/🦀️.rs`, `M/🔄️machine/✨️derive/⚙️expansion/🦀️.rs`.
- Edited (readers, section 4): `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` (+2 lines), `M/📡️replication/🚪️io/🧪️tests/🏛️ownership/🟦️.ts` (2 lines), `M/📡️replication/📦️packages/🦀️rust/📜️script.ts` (2 lines).
- Created: the 37 children listed in section 2 (list in `F/c4p35-tools/newdirs.txt`).
- Tools, kept in `F/c4p35-tools/`: `c4p35.py` (stager, lossless, installer), `c4p35x.py` (custom builders for exp and jtu), `widen2.py`, `rsitems.py`, `specs/*.py` (one spec per file), `oracle.py`, `check35.py`, `fmt.py`, `defscan.py`, `modclash.py`, `testnames.py`, `live.py`, `census.sh`, `census-own.ts`, `policy-read.ts`, `taxcheck.ts`, `tax.ts`, `newdirs.txt`, `orig/` and `SHA256.txt`. Logs: `F/🗑️generated/c4p35-*` (install journal `c4p35-installed.json`).
- No `[DEBUG]` log was added; the 17 pre-existing `[DEBUG]` strings of the `jtu` file are untouched.

## 6. Foreign errors and inherited findings

- **Ingress ownership law**: 1 pass 3 fail at baseline and after; the failing assertion is the `closure` comparison (`fixture.closure` is stale against today's Cargo manifests: `semio-framework-pack-json`, a new `native-build-compiler-resources` row and others). It sits after every assertion this slice can influence, and the two logs are identical.
- **Stale provenance pin**: `pack/⚠️error/.../🧭️cause/🔣️.json` pins `sha256 9784b9...` for the pack format root (already not the live hash before this slice, no reader).
- **Stale anchors**: `merge-modes.json` `wire_label` anchors point at the frames root but are not there (before and after).
- **`new crates --check-applied`** (plugin gate) does not apply to framework crates and was not run.
- The `📜️script.ts` presence oracle needs the original parent process invocation (launcher), so it was proven by the module-tree probe instead of executed.

## 7. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md`; the mutex folder `🗑️generated/cargo-mutex.lock` is held by a peer (since 08:42). Exact commands for the compile queue (hold the fleet mutex around each launcher call):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-pack-format -p semio-framework-pack-json-tests -p semio-framework-onboarding -p semio-framework-replication-mutation -p semio-framework-replication-command-ingress -p semio-framework-replication-presence -p semio-framework-replication-frames -p semio-framework-machine-derive --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```

then the same with `--features semio-framework-pack-format/deflate` for the pack format crate and `--target wasm32-wasip2` for the seven non-proc-macro crates. Afterwards the native tests that read or run the split code: `cargo test -p semio-framework-pack-json-tests --lib` (62 `#[test]` fns of the `jtu` fragments, names unchanged), `-p semio-framework-replication-command-ingress --lib`, `-p semio-framework-replication-presence --lib`, `-p semio-framework-replication-frames --lib`, `-p semio-framework-replication-mutation --lib`, `-p semio-framework-onboarding --lib`, `-p semio-framework-machine-derive --lib`, `-p semio-framework-pack-format --lib`, and the `dsl-derive` test `mutation_trait_matches_adjacent_required_declaration_facts` (reads the mutation root). If a build exposes a missing `pub(super)`, widen that one definition line (the spec and `c4p35.py` reproduce the stage).

## 8. Residual risks

- Not covered by a compiler: borrowck, trait-method resolution through glob re-exports and the lint set beyond `unused-imports` and the default resolution lints (the differential compares all warnings, none new). The differential runs with unresolved external crates (59 to 1,334 error lines per configuration, identical before and after), so field and method privacy through external types is only covered by the definition scan and the widener; the real compile decides.
- `pub(super)` widening is name-based and conservative (generic names such as `new`, `parse`, `write_bytes` were widened on `Header`, `Footer` and similar impls).
- `include!` fragments (`jtu`) are invisible to rustfmt; they were parse-checked as files.
