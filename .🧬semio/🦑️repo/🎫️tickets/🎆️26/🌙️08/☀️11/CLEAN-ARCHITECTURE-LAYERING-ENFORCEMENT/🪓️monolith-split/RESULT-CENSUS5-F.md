# Result CENSUS5-F (prefix `c5f`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool, no cargo/bun repo/nx. `F` = this folder. `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Tools: `F/c5f-tools/` (copy of `c5b-tools` with the prefix and sandbox renamed to `c5f` and `/Users/ueli/.cache/c5f/`; `orig/` keeps only the five snapshots). Logs: `F/🗑️generated/c5f-*`.

## 0. Verdict

**DONE for package 5 (5 files, 8,920 lines, 36 new children), static evidence only; the real `cargo check` is the coordinator's.**

- Found state: the five splits were already installed on the live tree by the earlier slice `c5b` (installed 2026-10-17 23:46, snapshots equal `git HEAD`, byte for byte). `RESULT-CENSUS5-B.md` still says "package 5: not started", so it is stale (see section 5). This slice did not redo the cut. It re-verified every file on the live tree with the oracles, re-ran the differentials, found three source-text readers that the earlier slice had not re-pointed and re-pointed them.
- The `c5b` run of the package 5 typeck oracle predates its last edit of `specs/ip.py` (the two `reexport=False` children). All numbers below are fresh runs of this slice.
- Package 6 (`he`, `hg`, `hc`, `hr`, `hs`, the files of slice c5g) is also already installed by `c5b` (live roots 78, 94, 20, 104, 89 lines). Not touched here.

## 1. Verdict table

`PL` tail. Before = snapshot lines. Root and largest child are live line counts. All files are far below 1,000 lines.

| tag | file (tail under `PL/`) | before | root | new children | largest child | mechanism | owning crate |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- |
| su | `🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs` | 1,656 | 28 | 9 | 263 | `include!` fragments (flat test module, 49 tests to 49) | `semio-framework-plugin-host-shard-laws` (test) |
| dd | `🗃️document/🦀️.rs` | 1,856 | 87 | 6 | 468 | `#[path]` modules; the 464-line `impl ActiveArtifactStoreReplacement` is cut into 3 `impl` blocks of the same header at method boundaries | `semio-framework-plugin-document-media` |
| ir | `🧠️interpreter/🏃️runtime/🦀️.rs` | 1,202 | 89 | 6 | 355 | `#[path]` modules; `include_str!("🦀️.rs")` became the `OWNED_RUNTIME_SOURCE` concat of root plus children | `semio-framework-plugin-interpreter-runtime` |
| ip | `🧠️interpreter/🦀️.rs` | 3,172 | 119 | 11 | 438 | `#[path]` modules; `INTERPRETER_SOURCE` is the `concat!` of root plus children | `semio-framework-plugin-interpreter` |
| ab | `🧪️tests/🔬️app-app-builder/🦀️.rs` | 1,034 | 11 | 4 | 325 | `include!` fragments (inner-doc root, 62 tests to 62) | `semio-framework-plugin-tests-02` (test) |

Children (folder, mount order): see `c5f-tools/specs/{dd,ir,ip}.py` (module splits) and `FRAG["su"]`, `FRAG["ab"]` in `c5f-tools/specs5.py` (fragment splits). 36 new folders, all next to their root, all RGI (`rgi.mjs` bad=0) and resolved by the live taxonomy (`tax.ts`: 41 dirs including 5 existing mounts, 0 unresolved). Longest repo-relative path of a child: 218 UTF-8 bytes (limit 240).

## 2. Oracle results (all run on the live tree in this session)

| check | result |
| --- | --- |
| losslessness | `modsplit.py lossless dd ir ip`: `LOSSLESS OK` x3, `missing 0`; the extras are only mounts, `pub use child::*`, `use super::*;`, child docstrings, the declared impl re-openings of dd (header, `where` bounds, braces) and the two declared `concat!` source constants of ir and ip. `chk.py live su ab`: `CHK OK`, `gone=0 extra(non-marker)=0` (su 4 region markers added where a region spans a cut), `#[test]` multiset 49 to 49 and 62 to 62 |
| structure | `vmod.py live dd ir ip`: `VMOD OK` (each child mounted once, docstring emoji first, equal to the folder, unique and different from the root, path lengths, brace depth, rustfmt parse, include targets 0/0, 1/1 and 1/1, `#[test]` multiset, no `[DEBUG]`) |
| definition scan | `defscan.py dd ir ip`: 4, 16 and 81 private declarations scanned, 0 cross-file leftovers |
| glob ambiguity | `ambig.py live dd ir ip`: `AMBIG OK` (dd: glob source `lifted_app`, 24 child item names, 0 candidates; ir and ip: no glob source) |
| module-name clash | `modclash.py dd ir ip` (live mode): `modclash OK` |
| scoped patched differential | `oracle5.py` groups dd, ir, ip, hl (laws crate with the su fragments), t02 (tests-02 with the ab fragments), lib and test: ir 137 / 286, ip 32 / 106, hl 1 / 528, t02 98 / 474 errors on both sides, warnings equal: **CLEAN**. dd: 498 original, 512 new, only-new 14, all `E0433 unresolved crate` (`semio_framework_value` x8, `semio_framework_os_kernel` x2, `semio_framework_schema_composition` x2, `store` x2): the sandbox has no extern crates, and the 14 extra rows are exactly the paths named in the 2 re-opened impl headers (4 + 1 + 1 + 1 paths, times 2 duplicated headers). No other message differs. Log `c5f-oracle-pkg5.log` |
| typeck oracle (`posrun.py`, family `c5f5`, 6 crates, labels `pre` = the unsplit tree from `c5b` and `post` = this session) | canary (E0308, E0616, E0624) reached before and after; **privacy 0 / 0, trait-scope 0 / 0, resolve 13 / 26 equal, type 73 / 254 equal** in lib / test; the non-stub class `other` goes from 2 (`E0576` on `ArtifactApp` associated types) to 1 (`E0404` on `ArtifactApp`). Row diff `c5f-posdiff-pkg5.log`: the extra post rows are the documented limit of the oracle's stub layer (traits and structs that arrive through the root's `use` lines and the child's `use super::*;` glob are stubbed as `Tr`, so `M: Tr`, `P: Tr`, `Mutation: Tr` and the `E0599 trait bounds not satisfied` rows of the re-opened `ActiveArtifactStoreReplacement` impls and the `ArtifactApp` row at `AppProjectionCache` appear once the code sits in a child). The root imports are real (`use semio_framework_plugin_decl_app::ArtifactApp;`, `use store::{MemberFactory, SpaceMember, ...};`, `use pack::snapshot::ArtifactPack;`). The real `cargo check` is the confirmation. Logs `c5f-pos5-post.log` |
| formatting debt | `fmt.py`: hunks original / new dd 61 / 60, ir 66 / 66, ip 18 / 19; changed-line multisets dd 1355 / 1352, ir 674 / 674 (identical), ip 242 / 241 (only removed lines): no new debt kind |
| census (canonical `readRustSourceClosureV1`, files / bytes / active lines, ratio of worst limit 32 / 262,144 / 4,000) | dd 6 to 12 files, 2,573 lines, 0.721; ir lib 8 to 14 files (0.530), test 23 to 29 files (0.906); ip lib 3 to 14 files (0.851), **test 6 to 17 files, 3,797 of 4,000 lines (0.949)**; hl test 14 to 23 files (0.723); tests-02 test 25 to 29 files (0.906). Every closure is inside the gate. Logs `c5f-census-own-after.jsonl`, baseline `c5b-census-own-before.jsonl` |
| source-text readers | see section 3 |

## 3. Text readers

Search: `git grep -F` of every path suffix and crate path over `*.ts *.tsx *.mjs *.py *.json *.jsonc *.sh *.toml *.rs *.wit *.md`, a direct grep of the two split directories for `"🦀️.rs"` and relative `../🦀️.rs` text reads, and `readers.py` (`c5f-readers.txt`). No sha pin, preservation record or schema row names any of the five files (`📦️units/*.json` checked for the crate and path words: only crate names). The two `include!` fixtures and the `#[path]` mounts that name the fragments resolve (chk and oracle `t02` / `hl`).

Re-pointed by `c5b` before this slice (verified present, not touched): `🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts` and `🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests/🟦️.ts` (both read `🗃️document/🦀️.rs` as a module tree), `🧠️interpreter/🧪️tests/🔌️exports/🟦️.ts` (reads the roots of ip and ir as module trees; top-level item names it looks up are unique in both trees: `OwnedSemioExport` 1, `impl OwnedSemioExport` 1, `impl OwnedOperation` 1, `decode_owned_result` 1, `fn export` 1).

**Re-pointed by this slice** (the marker text moved into a child, so the plain read of the cut root would have failed):

| reader | what it reads | edit |
| --- | --- | --- |
| `🖥️host/🎟️context/🤝️admission/🧪️tests/🟦️.ts` | `runtime.includes("OriginalActorFixedReceipt::read")` on `🧠️interpreter/🏃️runtime/🦀️.rs` | `readFileSync(...)` became `readModuleTreeSource(new URL(...))` (the module-tree import was already there) |
| `🖥️host/📦️packages/🦀️rust/📜️script.ts` (`owned.includes("result.map_err(|error| decode_guest_fault_bytes(&error))")`, line 198) | the same runtime root | `readFileSync(..., "utf8")` became `readModuleTreeSource(...)` (import already there) |
| `🧠️interpreter/🏃️runtime/🧪️tests/🔬️microsecond-clock/🟦️.ts` | slices the runtime root from `pub fn resume_snapshot(` to `/// 🪶️ The six owned`; the two anchors now sit in two consecutive siblings, so a slice inside one file would not do | `readFileSync` became `readModuleTreeSource`; import `@semio-tech/framework-compiler-ts/syntax/module-tree` added after the `ajv` import (same specifier as the exports test) |

Proof: `c5f-tools/tree_probe5.py` (a Python emulation of `readModuleTreeSource`: parent first, then `#[path]` children in declaration order, canonical `🧪️tests` mounts excluded) over the live runtime root: all six assertions hold (the slice is 707 chars, anchors ordered, `deadline_us: u64`, `fuel, deadline_us, OwnedDeadline::Original`, no `NoFuelProgress|deadline_ms|*1000`); the same anchors hold on the snapshot. The TS tests themselves could not run: `bun test` stops at `Cannot find package 'ajv'` and the workspace packages `@semio-tech/framework-compiler-ts` and `@semio-tech/compiler-syntax-ecma-ts` are not linked at the moment, so the `ts-probe5.ts` probe (kept in `c5f-tools/`) cannot load either. The edits are one-token swaps of an existing helper that the sibling readers already use.

Not readers: `🖥️host/📦️packages/🦀️rust/📜️script.ts` line 474 parses the 28-line su root only for syntax (a root of `include!` lines is valid Rust); `include_str!` of `🏃️runtime/🏗️startup/🧪️tests/...` fixtures in `🖥️host/🏗️instance` are not cut files; `🪝️mounts/📜️wp2-declaration/🧪️hosts/🏗️app-builder/🦀️.rs` does `include!` of the ab root (nested fragment includes resolve relative to the root file; oracle `t02` clean).

## 4. Crates the coordinator must compile

```
cargo check -p semio-framework-plugin-document-media -p semio-framework-plugin-interpreter -p semio-framework-plugin-interpreter-runtime -p semio-framework-plugin-host-shard-laws -p semio-framework-plugin-tests-02 --lib --tests
```

Add `-p semio-framework-plugin-host-shard` with package 6. The wasm-gated parts need no separate target here (no `cfg(target_arch = "wasm32")` item moved except inside whole children).

## 5. Open risks and findings

1. `semio-framework-plugin-interpreter` test closure is at 3,797 of 4,000 active lines (0.949, 17 files): 203 lines of headroom before the line gate trips. It was at 3,687 before. A later growth of the interpreter needs a crate carve first.
2. Engine identity: `owned_engine_identity()` digests `INTERPRETER_SOURCE` and `OWNED_RUNTIME_SOURCE`. Both are now the concatenation of the root and the children (the root alone before), so the digest value changes once with this split, like with any edit of the interpreter. No pinned digest exists (the hub tests use synthetic `owned-engine-v1:aa...` values); a persisted `GuestCodecVerificationCache` from an earlier build is simply not reused.
3. The `dd` oracle diff (14 rows, section 2) is the declared header duplication on top of missing extern crates, not a change of meaning. The `pos` row growth is the documented stub-layer limit; none of the new rows names a thing defined inside the five files.
4. `RESULT-CENSUS5-B.md` is stale (package 5 and 6 are installed). Its tools are the ancestor of `c5f-tools`.
5. The TS laws could not be executed (missing `node_modules` packages), see section 3.
6. Rust formatting: no new debt kind, the existing formatting debt of these files moved with the code.

## 6. Files

- Created by `c5b` and verified here: 36 child folders with one `🦀️.rs` each, next to the five roots (`PL/🖥️host/🧵️shard/🧪️tests/🔬️unit/`, `PL/🗃️document/`, `PL/🧠️interpreter/🏃️runtime/`, `PL/🧠️interpreter/`, `PL/🧪️tests/🔬️app-app-builder/`); the five roots are edited. Child lists: `c5f-tools/specs/{dd,ir,ip}.py`, `c5f-tools/specs5.py`.
- Edited by this slice: `PL/🖥️host/🎟️context/🤝️admission/🧪️tests/🟦️.ts`, `PL/🖥️host/📦️packages/🦀️rust/📜️script.ts`, `PL/🧠️interpreter/🏃️runtime/🧪️tests/🔬️microsecond-clock/🟦️.ts` (each one anchored swap of `readFileSync` for `readModuleTreeSource`, others' edits in the first two files left alone).
- Tools kept: `F/c5f-tools/` (verifiers, specs, `tree_probe5.py`, `ts-probe5.ts`, five snapshots in `orig/` with `SHA256`). Logs: `F/🗑️generated/c5f-oracle-pkg5.log`, `c5f-pos5-post.log`, `c5f-posdiff-pkg5.log`, `c5f-census-own-after.jsonl`, `c5f-readers.txt`; the `c5f-c5f5-pre-*` entries there are symlinks to the `c5b` pre-split oracle outputs. Sandboxes: `/Users/ueli/.cache/c5f/`.
- No `[DEBUG]` log added, no manifest, launch route, owner row or lock changed.
