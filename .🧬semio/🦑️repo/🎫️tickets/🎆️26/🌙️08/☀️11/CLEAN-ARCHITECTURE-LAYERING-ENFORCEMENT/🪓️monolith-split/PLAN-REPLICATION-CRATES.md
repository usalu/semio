# Plan: Crate Partition of `semio-framework-replication`

Planner: Sonnet 5.5, read-only, foreground, no sub-agents. Consumers: wave-4 executors. This is a plan: nothing was edited, compiled, committed or run through cargo. Every number was measured on the live working tree on 2026-10-15 (about 17:30 to 18:45 CEST) with the canonical counter `readRustSourceClosureV1` (host cfg `aarch64-apple-darwin` plus `debug_assertions`, wasm32 cfgs, `test`, feature `deflate`) and a token-level item reference graph of the 57 library files (1,477 top-level items). Section 3 lists the method and its blind spots so any executor can re-measure in minutes. Peers edit the tree while this plan is used: re-run the census at WP0 start and anchor every cut on region markers and item names, never on the line numbers quoted here.

Notation. `R/` = `🧰️framework/🔨️modules/📡️replication/`. `P/` = `📦️packages/🦀️rust`. A crate written `x` is `semio-framework-replication-x` (project `@semio-tech/replication-x-rs`, `[package.metadata.semio] id = "replication-x"`). "Lines" are the gate's active lines (blank and comment lines count). "WP" is a work package (section 11). Paths are relative to `R/` unless they start with `🧰️`, `✏️s`, `🌎️hub` or `.`. `<folder>/P/…` means `<folder>/📦️packages/🦀️rust/…`.

## 0. Verdict in fourteen lines

1. Today `semio-framework-replication` (lib name `protocol`, tier 0) has a lib closure of **57 files / 1,039,653 bytes / 16,464 active lines** (identical on `wasm32-unknown-unknown` and `wasm32-wasip2`; 16,667 lines with feature `deflate`) and a test closure of **165 files / 1,868,093 bytes / 23,721 lines** (23,940 with `deflate`; 106 source files plus 59 embedded fixtures of 285,615 bytes). Limits: 3,500 lines / 28 files / 230,000 bytes per closure (hard 4,000 / 32 / 262,144). The binding limit is **lines and bytes of both closures**, not file count: 16,464 / 3,500 = 5 crates minimum for the lib, 23,721 / 3,500 = 7 for the tests.
2. The plan carves **16 new producer crates plus a thin facade** `semio-framework-replication` (lib name `protocol` kept, about 600 lines of explicit re-exports). The largest lib closure is 1,808 lines (`presence`), the largest test closure 2,482 lines (`presence`) and 230 KB (`history`, the one tight crate, with a prescribed fallback in 6.11); the largest file count is 23 (`wire-core`, `mutation-messages`).
3. The crates are the existing domain folders, not new abstractions: `ids`, `codec`, `wire-core` (🧾️wire + scalar + dictionary + retained codec), `format`, `mutation`, `edit`, `causal`, `conflict`, `mutation-messages`, `history-index`, `history`, `frames`, `interaction`, `presence`, `local-interaction`, `command-ingress`. Existing `replication-error`, `replication-crypto` and `mutation-diff` stay untouched and become the bottom of the DAG.
4. The module-level graph is **one 12-file knot plus six smaller ones** (section 5.1). Every knot is cut by moving a few items to the lowest crate that needs them; no cycle needs a trait, a port or an inversion. Every cycle-forming edge is listed with its cut (5.1, 5.2); seven further edges the token scan reports are false (an enum variant named like a type, mount-only re-exports, `Header` as a phase name) and are documented in 5.1.
5. **Causal schema types** `MutationEnvelope`, `ArtifactDiff`, `InverseMutation`, `FrontierSummary`, `FrontierComparison` (and `MutationDag`, `MutationTransform`, the edit-to-envelope bridge) live in the lowest causal-model crate `causal`, together with their value impls, their binary codec (`io/binary/causal`) and their text codec (`io/text/causal`); `frames` (ClientFrame/ServerFrame/artifact bootstrap), `conflict` (`ConflictKind::Quarantined`) and `history` all depend downward on it. The cut that makes this possible: `mutation` (message and edit vocabulary) sits below `causal`, `conflict` sits above it, and the message-ledger machinery (`mutation-messages`) sits above both.
6. **13 true orphan-rule or inherent-impl violations** (15 flagged by the check, two of them split-line artifacts) would be created by a naive file-level carve: a foreign-trait impl or an inherent impl living in a crate that owns neither the trait nor the type. They sit in two files (`🎮️mutation/🧵️canonical/🦀️.rs`, `🚪️io/📝️text/🧵️canonical/🦀️.rs`); in addition the whole `🧬️retirement` integration file (about 40 impls of the foreign trait `RetireOwned`) must be redistributed to the type owners. All are fixed by moving impl fragments to the crate that owns the type (5.3, 5.4). `ids`, `wire-core` and `mutation` therefore gain a dependency on `semio-framework-pack-json` (first-party, depends only on `value`).
7. DAG: **7 levels** (section 4). Level 0: `ids`, `codec`, `interaction`, `history-index`, `command-ingress`; the longest chain is `codec` > `wire-core` > `mutation` > `edit` > `causal` > `conflict` > `mutation-messages`.
8. **God-object rule:** no struct had to be decomposed. The largest inherent impl blocks are 233 lines (`ArtifactMessageComposeCursor`), 223 (`CommandBatchDriver`), 177 (`PresencePeerReader`), 159 (`DeflateRetainedCursor`), 157 (`ArtifactBootstrapAssembler`); each sits whole in one crate with its type. No extension trait is introduced.
9. The public surface (about 490 top-level `pub` names plus the module paths `protocol::{codec,ids,scalar,dictionary,io,mutation,causal,conflict,wire,format,peer_overlay,crypto}`) is kept by the facade; **no consumer needs an edit**. Consumers may be retargeted afterwards (section 9.4); the pack family, which uses only `protocol::codec` and `protocol::io::binary`, is then freed from the whole replication closure, which is what unblocks pack.
10. Parallelism: WP0 (serial, first: skeletons and registries), then **WP1..WP6 in parallel on disjoint source files**, then WP7 (facade, scripts, TS readers, locks, gate law, census).
11. Tests move with their subject. Exactly **one** test file must be split (`🧾️wire/🧪️tests/🔬️unit`, 531 lines: hlc/identifier tests go to `ids`, one schema-registry test to the facade); one test file (`🔗️causal/🔀️transition/🧪️tests/🔬️unit`, 793 lines) is the size risk and has a pre-agreed fallback.
12. Nothing here could be compiled: the compiler-ingress gate refuses the old crate before rustc starts. Verification is the patched full-resolution differential in the pack-carve union sandbox, orphan and privacy scans, census rows per crate (lib, test, wasm, `deflate`), TOML/JSON parsing, a new neutral crate-partition law with independent oracles, and one honest attempt at the Nx producer route per crate (section 12).
13. Pre-existing defect the split must not hide: the old facade root exports **`read_varint_u64` and `write_varint_u64` from two glob re-exports** (`codec::*` and `wire::*`), which is a hard ambiguity error at every flat use (`📡️spr/🧵️channel` uses `protocol::read_varint_u64` flat). The facade resolves it with two explicit `pub use` lines (5.5).
14. Biggest risks: first real compile (privacy and trait-in-scope are invisible to the oracle), the 144 per-workspace `wasm-prototype` profile rows that name only the old facade, ten `Cargo.lock` files, and the TS ownership law `🚪️io/🧪️tests/🏛️ownership` that pins file paths (this plan keeps every file at its path except three new fragment files, but the fixture and schema need two edits, 9.3).

## 1. Baseline, budgets, what binds

### 1.1 Closure measurements (gate counter, `debug_assertions`)

| closure | cfg | files | bytes | active lines |
|---|---|---:|---:|---:|
| lib | native (aarch64-apple-darwin) | 57 | 1,039,653 | 16,464 |
| lib | `wasm32-unknown-unknown` | 57 | 1,039,653 | 16,464 |
| lib | `wasm32-wasip2` | 57 | 1,039,653 | 16,464 |
| lib | native + feature `deflate` | 57 | 1,039,653 | 16,667 |
| lib + tests | native, `--cfg test` | 165 | 1,868,093 | 23,721 |
| lib + tests | wasm32 (both), `--cfg test` | 165 | 1,868,093 | 23,721 |
| lib + tests | native, `test` + `deflate` | 165 | 1,868,093 | 23,940 |

The census (`🗑️generated/census.ts`, `ONLY=semio-framework-replication`) reports 57 / 1,039,653 / 16,464, `overLimit true`, ratio 4.116, binding `lines`. Manifest `P/Cargo.toml` declares no `rust-source-boundary`. The lib has **no `target_arch` or `target_os` gate**: the only cfgs are `feature = "deflate"` (codec, format), `target_pointer_width = "64"` (two `cfg_attr(expect(clippy::large_enum_variant))` in `🔗️causal/🦀️.rs`) and `test`. Linux and Windows host cfgs were not run; with no `target_os` gate they cannot differ. The 49 test source files and 59 embedded fixtures of the test closure are fully attributed in Appendix A.

Per-file numbers used in the tables below were taken under `--cfg test` (a lib file that mounts a test module counts the mount lines), so they exceed the lib numbers by at most 0.4 percent.

### 1.2 Per-crate budget

At most **3,500 active lines, 28 files, 230,000 bytes** for the lib closure and for the test closure (embedded `include_str!`/`include_bytes!` fixtures count as files and bytes), per `📓️WAVE4-BRIEF.md`. Manifests declare the recipe's exact three keys at the ceiling (`maxSourceBytes = 262144`, `maxSourceFiles = 32`, `maxActiveLines = 4000`); the internal target is enforced by the census. Every path stays at most 256 characters and 240 UTF-8 bytes (`📓️EXECUTOR-RULES.md`); the longest new manifest path is 101 characters / 146 bytes, the longest existing test path 114 / 169 bytes.

### 1.3 What is in flight (do not collide)

* `R/⚠️error`, `R/🔐️crypto`, `R/🎮️mutation/🔺️diff` were carved by peers (explicit Framework workspace members, owner rows, own `📜️script.ts`). They are reused unchanged. `replication-crypto` keeps its "ports only" shape (24 lines, cap 250): this plan does **not** add `Blake3Hasher` to it (5.3, M6).
* `semio-framework-pack` is already a facade over twelve producers (RESULT-W4-PACK); its crates still depend on the replication facade for `protocol::codec`/`protocol::io::binary`. The os-kernel, plugin, ui, job, 2d carves are peer-owned; touch their manifests only for the consumer retargets of 9.4, and only after the facade has landed.
* The replication facade has **ten Cargo.lock files** in the working tree (root, `🧰️framework`, `🎓️teaching`, `✏️s`, `🌎️hub`, `✏️s/🔌️plugins/🀄️wfc`, `✏️s/🔌️plugins/🧩️puzzle`, `✏️s/🧑‍💻dev/🧩️composition`, `✏️s/🧑‍💻dev/🧹️fixture-sweep`, `✏️s/🧑‍💻dev/🌊️flow`) and **144** Cargo.toml files with `[profile.wasm-prototype.package.semio-framework-replication]` (section 9.5).

## 2. Inventory

### 2.1 Crate layout and mounts

* `P/Cargo.toml`: `name = "semio-framework-replication"`, `[lib] name = "protocol" path = "🦀️.rs"` (the package-dir glue file), workspace `../../../..`. Features: `deflate = ["dep:semio-framework-deflate"]` only. Dependencies: the lower owners `replication-error`, `replication-crypto`, `pack-source`, `mutation-diff`, `pack-json`, `pack-error`, `trace`, `diagnostic`, `schema-registry`, `schema-state`, `value`, `hash`, `deflate` (optional), `io-base64`, `value-derive`; third party `serde`, `serde_json`; target-gated `wasm-bindgen` (not used by any lib file: dead, see 10.3). Dev: `trace-memory-testing`, `blake3`, `async-macros`. One `[[test]] paged_seal_digest` at `🚪️io/📝️text/🧵️canonical/🔏️seal/🧪️tests/📄️paged-digest/🦀️.rs`.
* `P/🦀️.rs` (114 lines) mounts every domain file with `#[path]`: `codec` (+`codec::ids`), `ids`, `scalar`, `dictionary`, `io`, `mutation`, `causal`, `conflict`, a composite `wire` module (`📡️wire/🎮️command/📥️ingress`, `🧾️wire`, `📡️wire`, `📡️wire/🏠️local-interaction`, re-exporting `crypto`, `dictionary`, `ids`), `format`, the module-root file `🦀️.rs` (the "Sync" slice helpers) as `component`, `peer_overlay`, and `🧬️retirement` as the private `retirement_integration`; it re-exports `pub use crate::{causal,codec,conflict,format,mutation,scalar,value,wire}::*`, `ProtocolError`, `PackRefusal`, the command-ingress items, `crypto`, `value`, `dsl_value`; it installs the test `#[global_allocator]`; attributes `#![allow(async_fn_in_trait)]`, `#![allow(ambiguous_glob_reexports, unused_imports)]`.
* Domain files reach each other through that flat root namespace and, in child files, through `use super::*`: **1,614 `crate::value` paths, 128 `crate::wire`, 127 `crate::ProtocolError`, 94 `crate::ids`, 70 `crate::write_str`, 44 `crate::codec`, 30 `crate::read_str`, 30 `crate::io`, 21 `crate::causal`, 18 `crate::mutation`** and about 80 smaller ones in the 57 lib files (rewrite table in 6.0).

### 2.2 Families (lib closure, native, active lines under `--cfg test`; the lib closure is up to 0.4 percent smaller)

| family | files | lines | KB |
|---|---:|---:|---:|
| `📡️wire/🦀️.rs` (frames + presence + interaction in one file) | 1 | 3,253 | 166 |
| `🔗️causal/🔀️transition` (history transitions, fold, drafts) | 3 | 1,296 | 104 |
| `🔗️causal/🔀️transition/🔁️fold/🗂️index` (fold index, insert) | 2 | 344 | 46 |
| `🚪️io` (traits, binary bytes, operation sequence, text/binary causal, canonical + seal, presence) | 10 | 1,460 | 98 |
| `🎮️mutation/🦀️.rs` (Mutation trait, leaf descriptors, messages, meta, origin) | 1 | 1,658 | 88 |
| `🎮️mutation` messages / replay retirement / prepared | 8 | 949 | 83 |
| `📡️wire/🏠️local-interaction` | 4 | 892 | 64 |
| `🎮️mutation` map / foreign / canonical / admission | 6 | 692 | 63 |
| `⚙️codec` (core, retained, history limits, codec ids) | 5 | 773 | 62 |
| `📡️wire/🎮️command/📥️ingress` | 1 | 1,115 | 57 |
| `📐️format` (+ verification, record) | 3 | 1,099 | 55 |
| `🔗️causal/🦀️.rs` | 1 | 774 | 38 |
| `⚔️conflict` | 3 | 540 | 32 |
| `🧾️wire` (limits, record kinds, flags, policies, primitives) | 1 | 335 | 15 |
| `📖️dictionary` (+ decode) | 2 | 198 | 15 |
| `🔢️scalar` | 1 | 362 | 14 |
| `🆔️ids` | 1 | 257 | 12 |
| `🧬️retirement` (integration glue) | 1 | 116 | 10 |
| `👕️peer-overlay` | 1 | 207 | 9 |
| `🦀️.rs` (Sync: `extract_range`, `verify_slice`) | 1 | 94 | 5 |
| `P/🦀️.rs` (glue) | 1 | 114 | 4 |

Three files are above 1,000 lines: `📡️wire/🦀️.rs` 3,253 (166 KB), `🎮️mutation/🦀️.rs` 1,658 (88 KB), `📡️wire/🎮️command/📥️ingress/🦀️.rs` 1,115 (57 KB, self-contained). The first two are split by region (5.3, M1 and M2).

### 2.3 Cross-cutting facts that shape the plan

* **Statics / macros:** none with state in the lib (no `thread_local!`, `OnceLock`, `lazy_static`, no `macro_rules!`; the two macros in `🎮️mutation/🔺️diff` belong to the separate diff crate). The only `static`s are three string constants in `🚪️io/📝️text/🧵️canonical` and the `cfg(test)` global allocator of the facade glue, a per-test-binary item (6.0).
* **Embedded assets:** no `include_bytes!`/`include_str!` in any lib file; all 59 embedded files are test fixtures. Two fixtures live outside `R/` (3,950 B in `🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧪️tests/🧫️fixtures`, 108 B in `…/🏪️store/📦️prepared/🪪️identity/🧪️tests/🧫️fixtures`); they stay where they are and count towards `mutation-messages` and `edit`.
* **Orphan-rule structure:** 494 impl and retire-macro items were checked against the proposed crate map (5.4).
* **Third-party:** `serde` derives or impls in `ids` (4 derives), `interaction` (10 derives), `causal` (`FrontierSummary`), `codec` (`HistoryReadLimits`, `OwnedOperationBytes`), `command-ingress`, `history-index`; `serde_json` and `wasm-bindgen` are used by no lib file (comments and `cfg(test)` only). This is pre-existing debt against the "no external runtime libraries" rule; the split carries it per crate and does not widen it (10.3).
* **Public surface:** about 490 top-level `pub` items (fn/struct/enum/trait/type/const/static/mod) across the 57 files, plus `pub use` re-exports, methods and the module paths of 7.1.
* **Mounts from outside the tree:** none. No other crate `#[path]`-mounts a replication file (`git grep` over `*.rs` outside the ticket folders); consumers only `include_str!` fixtures by relative path, and those files do not move.
* **TS ownership law:** `🚪️io/🧪️tests/🏛️ownership/🟦️.ts` plus fixture `🚪️io/🧪️tests/🧫️fixtures/🏛️ownership/🔣️.json` and schema `🚪️io/🧬️schema/🏛️ownership/🔣️.json` pin the owner paths of the five `io` traits, the binary owner, the 12 canonical-tree types, the causal binary owner and the presence owner. The plan keeps all these files at their paths (the only fixture change is the canonical block, 9.3).

## 3. Method (what the numbers mean)

1. **Closure.** `readRustSourceClosureV1([P/🦀️.rs], repoRoot, cfg, readFile, {manifestDirectory: P, generated: () => undefined})` with the host cfg from `rustc --print cfg`, `debug_assertions`, optional `test`, optional `feature=deflate`; wasm cfgs from `rustc --print cfg --target …`. Same call shape as `🗑️generated/census.ts` and `pack-carve-tools/census_stage.ts`.
2. **Items.** A small Rust lexer (strings, raw strings, char literals, nested comments) and a brace-depth walker cut every library file into 1,477 column-0 items (fn, struct, enum, trait, type, const, static, impl, use, mod, macro invocation) with doc and attribute lines attached and the nearest `//#region` recorded. An impl is bound to the type it implements for; `artifact_retire_struct!`/`artifact_retire_leaf!` invocations are bound to the types they name.
3. **Reference graph.** An item references another when its masked text mentions a name defined by exactly one item crate-wide, that is not a method call (`.name(`) and, for lower-case names, is called, path-qualified or imported; private definitions count only for files below the defining file's directory. Duplicates (`read_varint_u64`, `write_varint_u64`, `read_u8`) are reported separately (5.5).
4. **Partition.** A file-to-crate map (Appendix A) plus region and item selectors for the three split files; the same graph at crate level gives the edges, the levels, the cycle check, the visibility audit (cross-crate use of non-`pub` definitions) and the orphan audit.
5. **Tests.** Each of the 49 test source files and 59 fixtures is attributed to the crate that owns the lib file that mounts it (fixtures: the test file that embeds them).
6. **Limits of this evidence** (carry into verification): references through method syntax, trait dispatch, macros, `Deref` and generic bounds are invisible to the item graph; private fields and `pub(crate)` methods called across a new crate boundary are only partly found (the scans of 5.5 found three); sizes are plus or minus 8 percent after import rewriting; the analysis scripts lived in the planner's scratch directory and are not kept: executors rebuild the graph with the pack-carve tools (`union.py`, `oracle.py`, `privscan.py`, `privscan2.py`, `orphan.py`, `lossless.py`, `census_stage.ts`).

## 4. Layers and the crate DAG

### 4.1 Rules the partition obeys

1. Lines and bytes decide the cut, cohesion decides the shape: a crate is an existing domain folder (or two siblings that share a type family), never a size bucket.
2. Dependencies point down only; the levels of 4.2 are the topological order. A reference to a higher crate is a defect and appears in section 5.
3. A type, its inherent impls and every impl of a foreign trait for it live in the crate that defines the type. No re-mounting: each `.rs` file has exactly one owning crate.
4. The former crate becomes a thin facade with explicit `pub use` lists and module-shaped re-exports (7); it adds no behaviour.
5. No compatibility alias, no deprecation, no shim (greenfield). `ids`, `wire-core` and `mutation` take a dependency on `pack-json` rather than a wrapper type.

### 4.2 Crates, levels and dependencies (first-party edges derived from the item graph; third-party in Appendix D)

| level | crate | domain folders it owns | depends on (replication crates) |
|---:|---|---|---|
| 0 | `ids` | `🆔️ids` | - |
| 0 | `codec` | `⚙️codec` (core, history limits, codec ids), `🚪️io` traits + `💾️binary/📦️bytes` + `📑️operation-sequence` | `replication-error` |
| 0 | `interaction` | interaction regions of `📡️wire/🦀️.rs` (new `📡️wire/🕹️interaction`) | - |
| 0 | `history-index` | `🔗️causal/🔀️transition/🔁️fold/🗂️index` (+ `🪴️insert`) | - |
| 0 | `command-ingress` | `📡️wire/🎮️command/📥️ingress` | - |
| 1 | `wire-core` | `🧾️wire`, `🔢️scalar`, `📖️dictionary`, `⚙️codec/🛬️retained` | `codec`, `replication-error`, `replication-crypto` |
| 1 | `local-interaction` | `📡️wire/🏠️local-interaction` | `interaction` |
| 2 | `format` | `📐️format`, module-root sync file | `codec`, `wire-core`, `replication-error`, `replication-crypto` |
| 2 | `mutation` | `🎮️mutation` root (Mutation trait, leaf descriptors, messages), `🗂️map`, `🌐️foreign` | `ids`, `wire-core`, `mutation-diff` |
| 2 | `presence` | presence regions of `📡️wire/🦀️.rs` (new `📡️wire/👥️presence`), `🚪️io/👥️presence`, `👕️peer-overlay` | `interaction`, `wire-core` |
| 3 | `edit` | `TransactionRef`/`MutationMeta`/`Edit`/`MutationOrigin` (new `🎮️mutation/✏️edit`), `🧵️canonical`, `🧾️transaction`, `🚪️io/📝️text/🧵️canonical` | `mutation`, `ids`, `wire-core`, `replication-crypto` |
| 4 | `causal` | `🔗️causal` root, `🚪️io/💾️binary/🔗️causal`, `🚪️io/📝️text/🔗️causal` | `mutation`, `edit`, `ids`, `codec`, `wire-core` |
| 5 | `conflict` | `⚔️conflict` | `causal`, `mutation`, `ids`, `wire-core` |
| 5 | `history` | `🔗️causal/🔀️transition` (+ fold, drafts) | `causal`, `edit`, `ids`, `history-index`, `wire-core`, `replication-crypto`, `replication-error` |
| 5 | `frames` | `📡️wire/🦀️.rs` frames part | `causal`, `ids`, `wire-core` |
| 6 | `mutation-messages` | `📨️messages/*`, `♻️retirement`, `🧳️prepared` | `conflict`, `mutation` |
| 7 | facade | `P/🦀️.rs` | all |

```
replication-error, replication-crypto, mutation-diff (peer crates)
ids   codec   interaction   history-index   command-ingress          (level 0)
        |  \        |
   wire-core(codec)  local-interaction(interaction)                  (1)
     |      \________________________
 format  mutation(ids,wire-core)   presence(interaction,wire-core)   (2)
            |
          edit(mutation,ids,wire-core,crypto)                        (3)
            |
          causal(mutation,edit,ids,codec,wire-core)                  (4)
        /     |          \
 conflict  history(+history-index)  frames                           (5)
    |
 mutation-messages(conflict,mutation)                                (6)
facade -> everything                                                 (7)
```

Checked acyclic by a DFS over the item-level crate edges after applying the moves and the removals of 5.2 (the planner's scratch DFS: levels 0 to 6 plus the facade at 7). `frames` does not depend on `presence`: the frames carry presence only as opaque `Vec<u8>` blobs; the only upward-looking line is a `pub use crate::io::presence::{…}` re-export at the top of `📡️wire/🦀️.rs` (line 19), which moves to the facade.

## 5. Cycle cuts and ownership fixes

### 5.1 The real knots (module-level strongly connected components of the 57 files, after removing name noise)

| # | knot | what ties it | cut |
|---|---|---|---|
| K1 | `⚙️codec` / `🛬️retained` / `🔢️scalar` / `🚪️io/💾️binary/📦️bytes` | `ByteSpan`/`ByteReader` name `OwnedOperationBytes` (`ByteStorage::Paged`), `OwnedOperationBytes` uses `ByteSpan`/`PackEncodeOptions`; `scalar` uses `ByteReader`/`ByteWriter`; `retained` uses `scalar::civil_from_days`; `codec` only `pub use`s the retained types | `codec` owns core + `OwnedOperationBytes` together (they are one unit); `scalar`, `retained` and `dictionary` go **up** to `wire-core` (they use codec, never the reverse); the five `pub use` lines of retained types leave `⚙️codec/🦀️.rs` for the facade |
| K2 | `📐️format` / `🔎️verification` | `VerifiedSprSpan` vs `Header`, `MAGIC`, `COMMIT_FRAME_LEN` | same crate (`format`) |
| K3 | mutation / causal / conflict / io-binary root (12 files) | (a) `conflict` needs `MutationMessage` (mutation root) and `MutationEnvelope` (causal); `mutation/messages/{retire,accumulate}` need `conflict::{EditMessages,MutationReplayOutcome}`; (b) `causal` needs `Mutation`, `Edit`, `TransactionRef`; (c) mutation root mounts and re-exports the message ledger modules; (d) `TransactionRef` identity hashes through `format::Blake3Hasher` | (a)+(c): the ledger machinery (`📨️messages/*`, `♻️retirement`, `🧳️prepared`) is a separate top crate `mutation-messages`; the root drops its 18 mount and `pub use` lines 21 to 38 (they only re-export, the root never calls them); (b) `mutation` and `edit` sit below `causal`; (d) `Blake3Hasher` (8 lines) moves to `wire-core` (M6) |
| K4 | `📡️wire/🦀️.rs` vs `🚪️io/👥️presence/🔍️borrowed` | the wire file re-exports the borrowed presence reader (one `pub use` line) and the reader names `PresencePeerWireLimitsV1`, `PresenceToolRunState`, `PresenceHistoryEditStage`, `PresencePrincipalKind` from the wire file | both in `presence`; the re-export moves to the facade |
| K5 | local-interaction root / update / root-state (3 files) | `LocalInteractionRoot` vs `LocalInteractionRootUpdate` vs `LocalInteractionState` | same crate (`local-interaction`) |
| K6 | fold index / insert (2 files) | `HistoryFoldIndex` vs `HistoryFoldIndexInsert` | same crate (`history-index`) |
| K7 | history transition / fold / drafts (3 files) | `fold_history`, `fold_history_for`, `decode_history_transition` (thin wrappers in the transition file call the fold file's controlled forms); the fold file uses 19 names of the transition file; drafts use the fold index | one crate `history`; `index` leaves it for `history-index` (leaf: it uses nothing from the history files) |

Edges that look like cycles but are not (documented so nobody "fixes" them): `causal -> conflict` (token `Conflict` is the variant `TransformOutcome::Conflict(String)`), `mutation-edit -> format` (`Header` is a `Phase` enum variant in `seal`), `wire-core -> conflict` (the single line `artifact_retire_leaf!(MergePolicy, ConflictResolution)` spans two owners: split it, M5), `codec -> wire-core` (the retained re-exports), `mutation -> mutation-messages` and `edit -> mutation-messages` (mount and `pub use` lines only), `frames -> presence` (the line-19 re-export).

### 5.2 Edges removed or rerouted (all mechanical)

| edge | action |
|---|---|
| mutation root `#[path] mod` + `pub use` for compose, retire, accumulate, clamp, copy, replay retirement (lines 21 to 38) and `prepared` (1639 to 1641) | delete in `mutation`; `mutation-messages` glue mounts the files; facade re-exports `protocol::mutation::{ArtifactMessageComposeCursor, …}` |
| mutation root `canonical_fields` mount and `pub use` (1643 to 1646) | moves to the `edit` root |
| mutation root `transaction_admission` mount (1541 to 1542) and `transaction_ref_tests` mount (1545 to 1546) | move to the `edit` root |
| `⚙️codec/🦀️.rs` lines 7 to 10 (`retained_wire_field` mount and five `pub use`) | delete; `wire-core` glue mounts `⚙️codec/🛬️retained/🦀️.rs` |
| `🚪️io/🦀️.rs`: `pub mod presence`; `🚪️io/💾️binary/🦀️.rs`: `pub mod causal`; `🚪️io/📝️text/🦀️.rs`: `canonical` and `causal` mounts | delete from the three `io` files (codec owns them); the owning crates mount the children |
| `🔗️causal/🦀️.rs` `pub mod transition` (lines 12 to 13) | delete; `history` is a sibling crate |
| `🔗️causal/🔀️transition/🔁️fold/🦀️.rs` `mod index` (11 to 12) | delete; `history` imports `history-index` |
| `📡️wire/🦀️.rs` line 19 presence re-export | delete; facade |

### 5.3 Item moves (apply while carving; destination is the crate that must hold the item)

| id | what | from | to |
|---|---|---|---|
| M1 | frames / presence / interaction split | `📡️wire/🦀️.rs` | `frames` stays at the path; `presence` and `interaction` become new roots (Appendix B gives the region and line ranges) |
| M2 | `Meta` and `Origin` regions (`TransactionRef`, `MutationMeta`, `Edit`, `MutationOrigin`) + the `canonical_fields`/`transaction_admission`/`transaction_ref_tests` mounts | `🎮️mutation/🦀️.rs` 1252 to 1551, 1553 to 1635, 1643 to 1646 | new `🎮️mutation/✏️edit/🦀️.rs` (`edit`); the root keeps Mutation trait, leaf descriptors, messages, foreign mount, `map` mount, `apply_operation`, outcome-code and leaf-metadata test mounts |
| M3 | `ArtifactCanonicalJsonTree` impls and the inherent `canonical_field` helpers for `HybridLogicalTimestamp`, `MutationId`, `ActorId`, `ArtifactId`, `SchemaId`, `PayloadHash` (and the helpers `absent`, `selected`) | `🚪️io/📝️text/🧵️canonical/🦀️.rs` (lines 56 to 68 and 94 to 101) | new `🚪️io/📝️text/🧵️canonical/🆔️ids/🦀️.rs`, mounted by `ids` |
| M3b | same for `ForeignTarget` (lines 69 to 81) | same | new `🚪️io/📝️text/🧵️canonical/🌐️foreign/🦀️.rs`, mounted by `mutation` |
| M3c | same for `UndoPolicy` (lines 102 to 104) | same | new `🚪️io/📝️text/🧵️canonical/🧾️policy/🦀️.rs`, mounted by `wire-core` |
| M4 | retire impls: `ForeignTarget` (struct macro), `PayloadHash` (impl), `UndoPolicy` (leaf macro) | `🎮️mutation/🧵️canonical/🦀️.rs` lines 11 to 19 | `🌐️foreign/🦀️.rs` (mutation), `🆔️ids/🦀️.rs`, `🧾️wire/🦀️.rs`; `MutationMeta` and `Edit` retire impls stay |
| M5 | `🧬️retirement/🦀️.rs` dissolves; each impl moves to its type's crate (table 5.4b) | `🧬️retirement/🦀️.rs` | owners; the file and the `retirement_integration` mount are deleted in WP7 |
| M6 | `Blake3Hasher` struct + `impl RecordHasher` (region `🔖️Crypto`, `📐️format/🦀️.rs` 788 to 798) | `📐️format` | `🧾️wire/🦀️.rs` new region `🔖️Crypto` (`wire-core` takes `semio-framework-hash`) |
| M7 | generic combinators `write_opt_str`, `read_opt_str`, `write_opt_bytes`, `read_opt_bytes`, `write_vec_bytes`, `read_vec_bytes` (frames file 601 to 614, 616 to 629, 646 to 660) become `pub` | `📡️wire/🦀️.rs` | `🧾️wire/🦀️.rs` new region `🔖️Combinators`; `write_opt_frontier`, `read_opt_frontier`, `write_vec_envelope`, `read_vec_envelope` stay in `frames` (they name causal types) |
| M8 | visibility: `decode_hlc` (`pub(crate)`, used by the `history` fold) becomes `pub`; the six M7 combinators become `pub`; nothing else crosses a boundary by name (5.5) | `🚪️io/💾️binary/🔗️causal/🦀️.rs`, `📡️wire/🦀️.rs` | `causal`, `wire-core` |
| M9 | test split: hlc, identifier and document-version tests of `🧾️wire/🧪️tests/🔬️unit/🦀️.rs` (lines 310 to 377) | `wire-core` | new `🆔️ids/🧪️tests/🔬️unit/🦀️.rs` (mounted by `ids`); the kernel-inference-catalog test (lines 428 to 445) goes to the facade tests (needs `schema-registry`/`schema-state`) |

### 5.4 Orphan-rule audit (494 impl and retire-macro items checked against the crate map)

Violations in the naive file-level carve, all fixed by M3 to M5. The check flagged 15 items: rows 1 to 4 are the 13 true violations (3 + 7 + 2 + 1 impls), rows 5 and 6 are artifacts of separating one struct from its impl or one macro line between two owners.

| item (current file) | type owner | fix |
|---|---|---|
| `retire ForeignTarget`, `impl RetireOwned for PayloadHash`, `retire UndoPolicy` (`🎮️mutation/🧵️canonical/🦀️.rs`) | `mutation` / `ids` / `wire-core` | M4 |
| `impl HybridLogicalTimestamp` (inherent `canonical_field`, line 56) and `impl ArtifactCanonicalJsonTree for HybridLogicalTimestamp, MutationId, ActorId, ArtifactId, SchemaId, PayloadHash` (`🚪️io/📝️text/🧵️canonical/🦀️.rs`) | `ids` | M3 (this is also an inherent-impl violation: an inherent impl cannot live outside the defining crate) |
| `impl ForeignTarget` (inherent) and `impl ArtifactCanonicalJsonTree for ForeignTarget` | `mutation` | M3b |
| `impl ArtifactCanonicalJsonTree for UndoPolicy` | `wire-core` | M3c |
| `impl RecordHasher for Blake3Hasher` (format) | struct moved with it | M6 |
| `artifact_retire_leaf!(MergePolicy, ConflictResolution)` | `wire-core` + `conflict` | split the line (M5) |

Table 5.4b (destination of every integration impl in `🧬️retirement/🦀️.rs`; append to the owner file, keep the macro form):

| crate | impls |
|---|---|
| `ids` | `HybridLogicalTimestamp` (leaf), `MutationId`, `ActorId`, `ArtifactId`, `SchemaId` (hand impls), `PayloadHash` (from M4) |
| `wire-core` | `MergePolicy` (leaf), `UndoPolicy` (from M4) |
| `mutation` | `MapDelta<V>`, `MapEntryDelta<V>`, `MapEntryOperation<V>` (hand impls, append to `🗂️map/🦀️.rs`), `MapPresence` (leaf), `MutationMessage` (struct), `ForeignTarget` (from M4) |
| `edit` | `TransactionRef`, `MutationOrigin` |
| `causal` | `MutationEnvelope`, `ArtifactDiff`, `InverseMutation` |
| `conflict` | `ConflictResolution` (leaf), `Conflict`, `ConflictStatus`, `ConflictId`, `ConflictKind`, `EditMessages` |
| `history` | `HistoryFold`, `EffectiveSupersession`, `FoldChange`, `FoldCheckpoint`, `FoldAlternative`, `TransitionAuthor`, `TransitionPin`, `InputReplacement`, `FoldEdit`, `TransitionCheckpoint`, `TransitionSupersede`, `SupersededInput`, `HistoryTransition`, `ViewerHead` |

Everything else checked clean: all value/serde/OpBinary impls sit in the type's file; the only foreign-trait impls of local types are of `ToValue`, `FromValue`, `RetireOwned`, `ArtifactCanonicalJsonTree` and `serde` traits, all in the defining file after M3 to M5.

### 5.5 Visibility and name audit

* Cross-crate use of non-`pub` definitions (token-level, found by the visibility scan): `decode_hlc` (`pub(crate)`, used by `history` fold), the six M7 combinators (private, used by `frames` and `presence`). After putting scalar, retained and dictionary into `wire-core`, `RetainedWireUtf8Validation` and its `is_complete`/`consume` stay crate-local. No private struct field was found accessed across a new boundary by name; the privacy pass of the oracle cannot see field and method privacy, so the privscan scans of 12.4 are mandatory.
* Name collisions: `write_varint_u64` and `read_varint_u64` are defined in `codec` (return `PackRefusal`) and in `wire-core` (wrap the codec ones, return `ProtocolError`). The facade root must name one explicitly. Recommendation: `pub use wire_core::{read_varint_u64, write_varint_u64}` at the facade root (this is the variant `protocol::wire::` always exposed and the one `📡️spr/🧵️channel` calls with `?`), and keep the codec ones at `protocol::codec::…`. First check with the baseline oracle whether the flat name resolves at all today (two same-named globs are `E0659` on use, verified with a scratch `rustc` file): either `📡️spr/🧵️channel` is currently broken, or an explicit export exists that this scan missed.
* `read_u8` exists in `wire-core` (public) and as a private fn in `history`; no conflict.

## 6. The crates

### 6.0 Shared rules for every manifest and root

* **Manifest** (recipe 3.2): `workspace` points at `🧰️framework` (depth table below), `name`, `version = "0.1.0"`, `edition = "2021"`, `rust-version.workspace = true`, role `framework`, `id = "replication-<x>"`, exact three boundary keys at the ceiling, `[lints] workspace = true`, `[features]` only where stated, dependencies as in Appendix D.
* **Lib root.** `[lib] path = "../../🦀️.rs"` when the crate is exactly one domain folder (`ids`, `mutation`, `conflict`, `frames`, `history`, `history-index`, `local-interaction`, `command-ingress`, `interaction`, `presence`, `edit`); `[lib] path = "🦀️.rs"` with a package-dir glue file when it spans several folders (`codec`, `wire-core`, `format`, `causal`, `mutation-messages`), like `🎮️mutation/🔺️diff/P` and the facade. Glue files contain only `#[path]` mounts and `pub use` lines, no comments inside definitions.
* **Depth** (`workspace = "../" × n`): `ids`, `codec`, `wire-core`, `format`, `mutation`, `causal`, `conflict`, `frames` n = 5; `edit`, `mutation-messages`, `history`, `interaction`, `presence`, `local-interaction` n = 6; `command-ingress` n = 7; `history-index` n = 8.
* **Imports:** replace the flat root names with explicit imports per file: `crate::value::…` becomes `semio_framework_value::…`; `crate::ProtocolError` and the four retirement functions become `semio_framework_replication_error::…`; `crate::PackRefusal` becomes `semio_framework_pack_error::PackRefusal`; `crate::ids::X` becomes `semio_framework_replication_ids::X`; `crate::write_str`/`read_*`/`write_*`/`REC_*`/`FRAME_*`/`REQUIRED_*`/`ProtocolLimits`/`MergePolicy`/`UndoPolicy`/`RecordHasher` come from `semio_framework_replication_wire_core` (the hasher traits from `semio_framework_replication_crypto`); `crate::codec::X` and `crate::io::X` stay inside `codec` or point to it; `crate::mutation::X` to `mutation`, `edit` or `mutation-messages` by the table of Appendix A; `crate::causal::X` to `causal`. Do not use glob re-exports to hide the rewrite; the pack carve did exactly this at token level with strings and comments excluded.
* **Per-crate test allocator.** `codec`, `wire-core`, `causal`, `history`, `local-interaction` have tests that use `crate::test_allocation::observe_backing`; each lib root gets the three lines that the facade glue has today (`#[cfg(test)] pub(crate) use semio_framework_trace_memory_testing as test_allocation; #[cfg(test)] #[global_allocator] static …`) and a dev-dependency on `semio-framework-trace-memory-testing`. The facade glue drops its allocator unless a facade test needs it (the allocation tests move out with their subjects).
* **`async fn` in public traits:** copy `#![allow(async_fn_in_trait)]` with the R7 comment into every crate whose compile reports `async_fn_in_trait`; keep `#![allow(ambiguous_glob_reexports, unused_imports)]` only in the facade.
* **Region markers:** preserve every `//#region`/`//#endregion`; in split files each new file carries balanced markers (nested regions that become top-level, e.g. `🔖️PresenceInteraction`, are closed in the new file).
* **Docstrings:** every new file starts with `//!` whose first character is a unique emoji (rule of `AGENTS.md`).

### 6.1 `ids` (WP1, about 300 lines, 15 KB; test closure +70 lines)

Owns `🆔️ids/🦀️.rs` (`MutationId`, `ActorId`, `ArtifactId`, `ArtifactVersion`, `SchemaId`, `SchemaVersion`, `PayloadHash`, `HybridLogicalTimestamp`) plus fragment `🚪️io/📝️text/🧵️canonical/🆔️ids/🦀️.rs` (M3) and the retire impls (5.4b). Dependencies: `value`, `pack-json`, `serde` (4 derives). Tests: the hlc/identifier tests split out of `🧾️wire/🧪️tests/🔬️unit` (M9) into `🆔️ids/🧪️tests/🔬️unit`. Lowest crate, used by everything above; keep public API byte-identical.

### 6.2 `codec` (WP1, 1,075 lines / 68 KB lib; test 1,684 / 106 KB / 15 files)

Glue root `⚙️codec/P/🦀️.rs` mounts `../../🦀️.rs` as the codec module (pub use the items; `pub mod ids` + `pub use ids::*` as today), `history_limits`, and `../../../🚪️io/🦀️.rs` as `pub mod io` (traits `OpBinary`, `OpBinaryReceiving`, `OpBinaryReceivingCursor`, `DiffBinary`, `DiffText`, `OpText`, `DiffCodec`; `io::binary::operation_bytes` = `OwnedOperationBytes` and the operation-byte output/preparation family; `io::binary::operation_sequence`). Feature `deflate = ["dep:semio-framework-deflate"]` (the 15 `cfg(feature = "deflate")` sites are all in `⚙️codec/🦀️.rs`). Dependencies: `replication-error` (traits name `ProtocolError`), `pack-error`, `value`, `io-base64`, `diagnostic`, `serde` (history limits, `OwnedOperationBytes: Serialize`). Edits: remove the retained mount and re-exports (K1), the `io` children mounts (5.2), imports (6.0). Tests: `⚙️codec/🧪️tests/🔬️unit`, `🆔️ids/🧪️tests/🔬️unit`, `🚪️io/💾️binary/📦️bytes/🧪️tests` (+3 fixtures); the allocator tests need the per-crate allocator. This is the crate `pack` retargets to (9.4).

### 6.3 `wire-core` (WP1, 1,204 lines / 77 KB lib; test 1,892 / 139 KB / 23 files)

Glue root `🧾️wire/P/🦀️.rs` mounts `../../🦀️.rs` (limits, `REC_*`, flags, `UndoPolicy`, `MergePolicy`, the `WireCodec` primitives `write_varint_u64`, `read_varint_u64`, `read_u8`, `write_str`, `read_str`, `write_bytes`, `read_bytes`, `write_hash32`, `read_hash32`, `write_bool`, `read_bool`, `write_f64`, `read_f64`), `../../../🔢️scalar/🦀️.rs` as `scalar`, `../../../📖️dictionary/🦀️.rs` as `dictionary` (+ `🛬️decode`), `../../../⚙️codec/🛬️retained/🦀️.rs` as `retained` (+ `🔣️json-string`), and the fragment `🚪️io/📝️text/🧵️canonical/🧾️policy/🦀️.rs` (M3c). New regions in `🧾️wire/🦀️.rs`: `🔖️Crypto` (M6) and `🔖️Combinators` (M7). Retire impls for `MergePolicy` and `UndoPolicy` (5.4b). Dependencies: `codec`, `replication-error`, `replication-crypto`, `pack-error`, `value`, `diagnostic`, `hash`, `pack-json`. The test file `🧾️wire/🧪️tests/🔬️unit` shrinks by M9. Tests of retained, json-string, dictionary decode and their fixtures stay at their paths. This crate carries 23 of 28 files in the test closure: do not add test files here without moving something out.

### 6.4 `format` (WP1, 1,215 lines / 62 KB lib; test 1,902 / 106 KB / 10 files)

Glue root `📐️format/P/🦀️.rs` mounts `../../🦀️.rs` (header, frames, commit, writer, recovery; loses `Blake3Hasher`), `🔎️verification` + `🧾️record` (as `retained` and `retained::record`, unchanged), and the module-root file `../../../🦀️.rs` as the sync slice helpers (`RecordSlice`, `extract_range`, `verify_slice`; its `use crate::format::{Blake3Hasher,…}` becomes `wire-core` + `format` imports). Feature `deflate = ["semio-framework-replication-codec/deflate"]` (the two `cfg(feature = "deflate")` arms in `prepare_payload`). Dependencies: `codec`, `wire-core`, `replication-error`, `replication-crypto`, `pack-error`, `pack-source`, `hash`, `value`. Dev: `blake3` (the commit-chain tests), `async-macros`, `serde_json`.

### 6.5 `mutation` (WP2, 1,730 lines / 108 KB lib; test 2,202 / 165 KB / 14 files)

Lib root `🎮️mutation/🦀️.rs` (stays at its path), reduced to: the `map` mount, `Mutation` trait with the `mutation-diff` re-exports, leaf descriptors and `MutationLeaf` (regions `🪪️MutationLeafDescriptor`, `🪪️MutationLeaf`), the message vocabulary (`OutcomeCode`, `MutationMessage`, `MutationOutcome`, `worst_level`, `MUTATION_MESSAGE_*`), the `foreign` mount (+ `ForeignTarget` canonical fragment M3b, retire macro M4), `apply_operation`, and the mounts of the leaf-metadata and outcome-code tests. Also owns `🗂️map` (with the `MapDelta*` retire impls, 5.4b) and `🌐️foreign`. Dependencies: `ids`, `wire-core` (`UndoPolicy` in the `Mutation` trait, `MergePolicy` in `MutationOutcome`), `mutation-diff`, `value`, `diagnostic`, `schema-state` (`StateClass`), `pack-json`. Tests: `🧪️tests/🔬️mutation-leaf-metadata`, `🧪️tests/🧪️outcome-code`, `🗂️map/🧪️tests`, `🌐️foreign/🧪️tests` (+6 fixtures).

### 6.6 `edit` (WP2, 992 lines / 76 KB lib; test 1,261 / 130 KB / 20 files)

New root `🎮️mutation/✏️edit/🦀️.rs` holds regions `🔖️Meta` and `🔖️Origin` of the old root (`TransactionRef`, `MutationMeta`, `Edit<Op>`, `MutationOrigin`) and mounts `🧵️canonical` (identity, authority cursors; loses the three foreign retire impls, M4), `🧾️transaction/🎟️admission`, `🚪️io/📝️text/🧵️canonical/🦀️.rs` (canonical trees of `Edit`, `MutationMeta`, `TransactionRef`, `MutationOrigin`; helpers `absent`/`selected` stay, copies of them exist in the three fragments, which is acceptable duplication within one tree) and `🔏️seal`. Retire impls `TransactionRef`, `MutationOrigin` (5.4b). Dependencies: `mutation` (`ForeignTarget`), `ids`, `wire-core` (`UndoPolicy`, `write_str`, `Blake3Hasher`), `replication-crypto` (`RecordHasher`), `hash` (`Sha256`), `pack-json`, `value`. Dev: `blake3`, `serde_json`, `trace-memory-testing`. The `[[test]] paged_seal_digest` stays on the facade. Tests: transaction-ref, identity, authority, admission, canonical, seal (+6 fixtures, one 108-byte external).

### 6.7 `causal` (WP4, 1,381 lines / 76 KB lib; test 2,118 / 133 KB / 8 files)

Glue root `🔗️causal/P/🦀️.rs` mounts `../../🦀️.rs` (`MutationEnvelope`, `ArtifactDiff`, `InverseMutation`, `MutationDag` family, `FrontierSummary`, `FrontierComparison`, `frontier_delta`, `MutationTransform`, the edit-to-envelope bridge), `../../../🚪️io/💾️binary/🔗️causal/🦀️.rs` (hlc, envelope, frontier, batch codecs; `decode_hlc` becomes `pub`) and `../../../🚪️io/📝️text/🔗️causal/🦀️.rs` (the `ToValue`/`FromValue` impls of the five types). Retire impls for the three envelope types. Dependencies: `mutation` (`Mutation`), `edit` (`Edit`, `TransactionRef`), `ids`, `codec` (`OpBinary`), `wire-core`, `replication-error`, `value`, `serde` (`FrontierSummary` derive). The `target_pointer_width = "64"` `cfg_attr` stays. Tests: `🔗️causal/🧪️tests/🔬️unit` (737 lines, refs 19 causal / 12 mutation / 10 binary-causal names) with fixtures backbone-batch (18 KB), slots, causal-add.

### 6.8 `conflict` (WP3, 583 lines / 36 KB lib; test 766 / 64 KB / 8 files)

Lib root `⚔️conflict/🦀️.rs` unchanged apart from imports; `♻️retirement`, `♻️report`; retire impls (5.4b). Dependencies: `causal` (`MutationEnvelope` in `ConflictKind::Quarantined`), `mutation` (`MutationMessage`), `ids`, `wire-core` (`MergePolicy`), `value`, `diagnostic`, `hash`. Tests: `🧪️tests/🔬️unit`, `♻️retirement/🧪️tests` (+2 fixtures).

### 6.9 `mutation-messages` (WP3, 989 lines / 86 KB lib; test 1,446 / 173 KB / 23 files)

Glue root `🎮️mutation/📨️messages/P/🦀️.rs` mounts the five message modules (compose, retire, accumulate, clamp, copy), `../../♻️retirement/🦀️.rs` (replay retirement) and `../../🧳️prepared/🦀️.rs` (+ `♻️retirement`). Dependencies: `mutation`, `conflict`, `value`, `diagnostic`, `replication-error`. Tests: all of `📨️messages/*/🧪️tests`, `♻️retirement/🧪️tests`, `🧳️prepared/♻️retirement/🧪️tests` (+8 fixtures; the largest is 14 KB; one 3,950-byte fixture lives under `🏪️store`).

### 6.10 `history-index` (WP4, 374 lines / 48 KB lib; test 559 / 100 KB / 8 files)

Lib root `🔗️causal/🔀️transition/🔁️fold/🗂️index/🦀️.rs` (mounts `🪴️insert`). Dependencies: `value`, `pack-json`, `replication-error`, `serde` (manual `Serialize`/`Deserialize` for `HistoryFoldIndex`/`HistoryFoldSet`). Fixtures: insertion (18 KB), set, entry, index (4).

### 6.11 `history` (WP4, 1,371 lines / 111 KB lib; test about 2,400 / 222 to 230 KB / 14 files; **the tightest crate**)

Lib root `🔗️causal/🔀️transition/🦀️.rs` (vocabulary, codec, envelope bridge, `🔖️Fold` types) mounting `🔁️fold/🦀️.rs` (fold algorithm, controlled decode, `HistoryFoldJob`) and `📝️drafts`; the `index` mount in the fold file is deleted. Retire impls (5.4b). Dependencies: `causal`, `edit` (`TransactionRef`), `ids`, `history-index`, `wire-core` (`write_str`, `read_varint_u`, `Blake3Hasher`), `replication-crypto`, `replication-error` (`protocol_cause_retirement_demand`, `close_protocol_cause_one`), `hash`, `value`. Dev: `blake3`, `serde_json`, `trace-memory-testing`. Test closure: lib 104 KB + unit 53.5 KB + fold retirement 20 KB + drafts 6.5 KB + 7 fixtures including `history-transition` (11.8 KB), `supersede-fold` (16.6 KB), `durable-collaborative-redo-v1` (6.6 KB) = about 223 KB before import rewriting, 230 KB after (the planner's allowance). **Decision rule:** if the WP4 census shows more than 225,000 bytes, move the three fixture-driven laws (`durable_collaborative_redo_fixture_survives_reload_and_hub_restart`, `the_language_agnostic_fixture_matches_the_codec_byte_for_byte`, `the_supersede_fold_fixture_matches_the_fold_law`) and their fixtures into a `[[test]]` integration target `history_fixture_laws` (public API only: `fold_history`, `fold_history_for`, `decode_history_transition`, `encode_history_transition`, the vocabulary types); the unit closure then drops by about 50 KB and the integration closure is about 155 KB. Do not split the crate: `fold` and `transition` form a knot (K7) whose only link is the wrappers.

### 6.12 `frames` (WP5, 1,026 lines / 54 KB lib; test 1,492 / 84 KB / 5 files)

`📡️wire/🦀️.rs` stays at its path and shrinks to the frames: regions `🔖️Lane`, `🔖️ClientFrame`, `🔖️ServerFrame` (including the artifact-bootstrap assembler and `SnapshotChunkBytes`), `🔖️Codec` (including `🔖️Combinators` remainder and `🔖️NestedEnums`: `encode/decode_client_frame`, `encode/decode_server_frame`), `🧪️Tests` mounting `🧪️tests/🔬️unit`. Dependencies: `causal` (`MutationEnvelope`, `FrontierSummary`, envelope/frontier codec), `ids` (`ActorId`), `wire-core` (primitives and combinators), `replication-error`, `hash` (aggregate bootstrap hash), `value`. Tests: `📡️wire/🧪️tests/🔬️unit` (466 lines), fixtures `🚀️artifact-bootstrap` (3.3 KB) and the 63-byte legacy hello `.bin` (both under the replication root `🧪️tests/🧫️fixtures`, referenced by relative path). The TS interactivity audit reads this file for the hello frame (`📜️script.ts` line 8066) and keeps working.

### 6.13 `interaction` (WP5, 873 lines / 45 KB lib; test 1,191 / 65 KB / 6 files)

New root `📡️wire/🕹️interaction/🦀️.rs`: region `🔖️Interaction` of the old file without its nested `🔖️PresenceInteraction` (lines 2226 to 3073: `HoverSpec`, `SelectionSpec`, `HierarchyProvider`, `SelectionMode`, `SelectionMethod`, `MergeMode`, `InteractionTarget`, `DomainSelection`, `DomainHover`, `InteractionState`, topology, `next_selection`, `next_hover`, `validate_state`, `InteractionOutline`) and the empty `🔖️InteractionStorePack`. Mounts the three tests that stay in `📡️wire/🧪️tests` (interaction-wire, interaction, typed-field-order) by `#[path = "../🧪️tests/…"]`. Dependencies: `value`, `pack-json`, `serde` (10 derives). Leaf: it uses nothing else.

### 6.14 `presence` (WP5, 1,808 lines / 91 KB lib; test 2,482 / 144 KB / 8 files)

New root `📡️wire/👥️presence/🦀️.rs`: region `🔖️Presence` (lines 1100 to 2224: `PresenceWindowView`, `PresenceViewKind`, `PresenceUi`, `PresencePeer`, typing/tool-run/history-edit types, `PresencePeerWireLimitsV1`, the reader and the codec), region `🔖️PresenceInteraction` (lines 3075 to 3246: `PresenceInteraction`, `PresenceDomain`, their codec, `assemble_presence_interaction`), mounts of `../../🚪️io/👥️presence/🔍️borrowed/🦀️.rs` and `../../👕️peer-overlay/🦀️.rs`, and of the tests `presence-codec`, `assemble-presence-interaction` (stay in `📡️wire/🧪️tests`) and `peer-overlay/🧪️tests`. Dependencies: `interaction` (`InteractionState`, `HoverSpec`, `SelectionSpec` in the assembler), `wire-core` (primitives, `write_opt_str`), `replication-error`, `value`. Fixture `👥️presence-peer-codec-v1` (13 KB). The replication package script reads this code for `presence-peer-codec-check` (9.3).

### 6.15 `local-interaction` (WP6, 922 lines / 65 KB lib; test 1,204 / 180 KB / 13 files)

Lib root `📡️wire/🏠️local-interaction/🦀️.rs` (mounts `🌳️root`, `🩹️update`, `📡️transport`). Dependencies: `interaction` (`DomainSelection`, `SelectionMode`), `value`, `replication-error` (confirm), `serde_json` dev. The 91.7 KB fixture `🏠️local-interaction/🔣️.json` is the reason this crate cannot absorb `interaction`. Allocator tests in root and update.

### 6.16 `command-ingress` (WP6, 1,145 lines / 58 KB lib; test 1,283 / 67 KB / 4 files)

Lib root `📡️wire/🎮️command/📥️ingress/🦀️.rs` unchanged. Dependencies: `trace`, `diagnostic`, `value`, `value-derive`, `serde`. Fully independent (no replication edge at all). `🏛️ownership` TS tests and consumers (`kernel`, `channel`, `plugin`) keep the facade path `protocol::wire::command_ingress`.

## 7. The facade

### 7.1 What it keeps

The facade is `P/🦀️.rs` plus `P/Cargo.toml`: lib name `protocol`, package `semio-framework-replication`, features `deflate = ["semio-framework-replication-codec/deflate", "semio-framework-replication-format/deflate"]`, dependencies on the 16 crates and on the seven lower owners it re-exports. About 600 lines. Module-shaped re-exports (crate aliases where one crate owns a whole old module, composite modules otherwise):

| old public path | new source |
|---|---|
| `protocol::value`, `dsl_value`, flat `value::*` | `semio-framework-value` |
| `protocol::codec` (+ `codec::ids`, flat codec items, base64 re-exports, `HistoryReadLimits`, deflate cursors) | `codec` plus the retained types (`RetainedWireFieldDecode`, `RetainedWireFieldStep`, `RetainedWireByteSource`, `OriginalWireSourceIdentity`) from `wire-core`: a composite `pub mod codec` |
| `protocol::ids`, `protocol::scalar`, `protocol::dictionary` | `pub use replication_ids as ids`; `wire-core::scalar`, `wire-core::dictionary` |
| `protocol::crypto`, `ProtocolError` and the four retirement functions, `PackRefusal` | unchanged lower owners |
| `protocol::io::{OpBinary, OpBinaryReceiving, OpBinaryReceivingCursor, DiffBinary, DiffText, DiffCodec, OpText}`; `io::binary::{operation_bytes, operation_sequence}` | `codec` |
| `io::binary::causal::*`, root re-exports `encode_envelope`, `decode_envelope(s)`, `encode_frontier`, `decode_frontier`, `encode_ops_vec`, `decode_ops_vec`, `decode_document_backbone_envelopes_exact(_with_limits)`, `DocumentBackbone*` constants | `causal` |
| `io::text::canonical::ArtifactCanonicalEditSealCursor` | `edit` |
| `io::presence::*` and the `wire::borrow_presence_metadata` family | `presence` |
| `protocol::mutation::*` (+ `map`, `transaction_admission`, `prepared_retirement`) | composite of `mutation`, `edit`, `mutation-messages` |
| `protocol::causal::*` (+ `causal::transition`) | `causal`; `causal::transition` = `history` + `history-index` items |
| `protocol::conflict` | `pub use … as conflict` |
| `protocol::format` (+ `format::retained`, `retained::record`), flat format items, `extract_range`, `verify_slice`, `RecordSlice`, `Blake3Hasher` | `format`, plus `wire-core::Blake3Hasher` re-exported at `format::Blake3Hasher` |
| `protocol::wire::*` (limits, `REC_*`, flags, policies, primitives, `RecordHasher`, frames, interaction, presence, `crypto::*`, `dictionary::*`, `ids::*`) | composite of `wire-core`, `frames`, `interaction`, `presence`, `crypto`, `dictionary`, `ids` |
| `wire::command_ingress` and the flat command-ingress items | `command-ingress` |
| `wire::local_interaction` and flat items | `local-interaction` |
| `protocol::peer_overlay` | `presence` |
| `RuntimeFrontierComparison`, `RuntimeFrontierSummary` (aliases of `FrontierComparison`, `FrontierSummary` in the old glue) | `causal`, kept as today (pre-existing public names; removing them is a separate consumer change) |

Re-exports are **explicit item lists** generated from the pre-split public surface (rule 4 of `📓️CARVING-RULES.md`); `pub use crate as module` aliases are used where one crate owns a whole module. The facade root also contains: `#![allow(async_fn_in_trait)]`, `#![allow(ambiguous_glob_reexports, unused_imports)]` (only while flat re-exports remain), the explicit varint names of 5.5, and the facade tests below.

### 7.2 Facade tests

* `[[test]] paged_seal_digest` unchanged (uses `protocol::io::text::canonical::ArtifactCanonicalEditSealCursor`, `protocol::Edit`).
* New `🧪️tests/🔬️facade/🦀️.rs` mounted by the facade: the kernel-inference-catalog test moved out of `🧾️wire/🧪️tests/🔬️unit` (M9) plus a **public-path conformance test**: one `use` per consumer-visible path of 7.1, generated from the baseline surface, so an omitted re-export fails to compile.

## 8. Tests: what moves, what is split

* Every test file keeps its path and moves with its subject; the owning crate root mounts it (`#[path = "🧪️tests/…"]` from its own folder; from a new root with `../` where the subject moved up one folder: interaction, presence, edit). `include_str!` paths are relative to the test file and therefore unchanged.
* Split exactly once: `🧾️wire/🧪️tests/🔬️unit/🦀️.rs` (M9). The remaining 531-line file stays with `wire-core` (wire primitives, flags, policies, scalar timestamp and id codec, dictionary, record hasher object).
* Per-crate dev-dependencies are listed in Appendix D.
* Tests that use `use super::*` inherit the parent module namespace; after the move the parent is the new root, so each root re-exports (`use`) what its tests name, or the test imports explicitly. This is the main source of compile errors the oracle will show.
* New neutral law (WP7): `🧪️tests/🏛️crate-partition/🟦️.ts` asserting, with independent oracles, that (a) the 17 manifests parse identically under Bun TOML and `@iarna/toml`; (b) their path dependencies equal the DAG table of 4.2 and are acyclic (DFS plus graphlib SCC); (c) no crate except the facade depends on the facade; (d) every `.rs` file under `R/` is reachable from exactly one lib root through `#[path]`/`include!` (module-tree helper `listModuleTree`) or is a declared test; (e) lib and test closures of all crates satisfy 3,500 / 28 / 230,000 under native, `wasm32-unknown-unknown` and `deflate` on/off via `readRustSourceClosureV1`; (f) no foreign-trait impl targets a type owned by another replication crate (token oracle). Add its launch route to both launch files.

## 9. Consumer impact

### 9.1 What does not change

Facade package name, lib name `protocol`, every public path, the `deflate` feature, the workspace dependency rows (`🧰️framework/Cargo.toml` line 420, `Cargo.toml` line 565). Every crate that depends on `semio-framework-replication` (dozens of framework crates plus the plugin and hub workspaces) needs no edit. All fixtures embedded by consumers (`include_str!` of replication fixture paths in `🌎️hub`, `🏪️store`, `📡️spr`, `🔌️plugin`) stay valid because no fixture file moves.

### 9.2 Registries (WP0, once)

* `🧰️framework/Cargo.toml` `[workspace] members`: 16 rows (byte-sorted among `🔨️modules/📡️replication/…` rows next to the existing `⚠️error`, `🔐️crypto`, `🎮️mutation/🔺️diff` rows).
* `📋️build-owners.json`: 16 rows named `@semio-tech/replication-<x>-rs` with `root` = the package dir, `manifest` = its `📋️project.json`, `packages: []`, inserted by name.
* 16 `P/📋️project.json` (`name`, tags `scope:framework`, `language:rust`, no targets; targets are inferred).
* `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 96 routes each (6 per crate: build, check, test x prototype, release; groups `4_build`, `4_gate`, `4_test`; name `🦀️ semio-framework-replication-<x> 🏗️build 🛠️prototype` and so on; command `bun repo run @semio-tech/replication-<x>-rs:<target> --configuration=<c> --parallel=1 -- --lib --offline`). The order bands are dense (`900.97xxxx`, `903.70xxx`, `922.70xxx` end at 900.970242 / 903.70022 / 922.70022): take the next free keys after the last route and grep both files for uniqueness. One more route for the new law of section 8.
* `[profile.wasm-prototype.package.semio-framework-replication-<x>] opt-level = 2` rows in `🧰️framework/Cargo.toml` and `Cargo.toml` (see 9.5).
* New folders to admit in the taxonomy (all emoji reuse existing folder prefixes; no sibling repeats): `🎮️mutation/✏️edit`, `📡️wire/🕹️interaction`, `📡️wire/👥️presence`, `🚪️io/📝️text/🧵️canonical/{🆔️ids,🌐️foreign,🧾️policy}`, `🧪️tests/🔬️facade`, `🧪️tests/🏛️crate-partition`, and 16 `P` folders. Run the taxonomy and discovery validation before WP1..WP6 start; fallback emoji: `🎮️edit` (precedent `🌱️value/📋️list/🎮️edit`).

### 9.3 Scripts, TS readers, laws (WP7)

| item | change |
|---|---|
| `P/📜️script.ts` of the facade (`TestScript`, `LocalInteractionNativeTestScript`, the aggregate at line 407: `packages:["semio-framework-replication"]`) | list all 17 packages; retarget the `local_interaction_` selector to `local-interaction`; native selectors for presence/format/retained checks to their new owners |
| same script, `presence-peer-codec-check` (reads `📡️wire/🦀️.rs` and the presence-codec test, lines 381 to 382) | read `📡️wire/👥️presence/🦀️.rs` |
| `P/📋️project.json` | keep its custom targets; its `namedInputs.default` already globs `R/**/*.rs` |
| `🚪️io/🧪️tests/🏛️ownership/🟦️.ts`, `🚪️io/🧪️tests/🧫️fixtures/🏛️ownership/🔣️.json`, `🚪️io/🧬️schema/🏛️ownership/🔣️.json` | `canonical.borrowedTrees` (12 names, one `textOwner`) becomes rows `{type, owner}` over the four files `…/🧵️canonical/🦀️.rs` (Edit, MutationMeta, TransactionRef, MutationOrigin), `…/🆔️ids/🦀️.rs` (six id types), `…/🌐️foreign/🦀️.rs` (ForeignTarget), `…/🧾️policy/🦀️.rs` (UndoPolicy); the assertion loops over owners. The 5 trait owners, binary owner, causal and presence rows stay valid |
| root `📜️script.ts` readers: line 6266 (`🔗️causal/🦀️.rs`), 8066 (`📡️wire/🦀️.rs`, hello frame), 8921 and 9204 (`📐️format/🦀️.rs`), 20399 (outcome-code fixture), 20843 (`🧾️wire/🦀️.rs`, MergePolicy) | files stay at their paths; re-run the self-tests; the format audit no longer sees `Blake3Hasher` (check its needle list) |
| `🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/📋️native-law-plan/🟦️.ts` line 50 (`📡️wire/🧪️tests/🔬️unit/🦀️.rs`) | unchanged (frames tests stay) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🗜️deflate/🧪️tests` line 27 (reads `⚙️codec/🦀️.rs`) | unchanged (codec core stays at its path) |
| `🎒️pack/🚰️source/🧪️tests` line 19 (reads `P/🦀️.rs` for forbidden re-exports) | re-run |
| store `📦️owned` test line 108/147, `🧪️supersede-replay` (read `🔗️causal/🦀️.rs`, `🎮️mutation/📨️messages/📋️copy`, `🔗️causal/🔀️transition/🔁️fold`) | unchanged paths |

### 9.4 Optional retargets after the facade lands (separate tickets; each removes the replication facade from a closure)

| consumer | uses (from the scan) | new dependency |
|---|---|---|
| `🎒️pack/*` (12 crates + `pack`) | `protocol::codec::*` (`KIND_*`, `Crc32cCursor`, `ByteSpan`, `ByteRange`, `PackLimits`, deflate cursors), `protocol::io::binary::operation_bytes`, `protocol::ByteSpan` | `replication-codec` (feature `deflate` forwarded); frees pack from the whole replication closure |
| `⏯️tool-run` (+ actions), `🎓️onboarding`, `🗺️surface/🕸️node-graph`, `🛂️manifest/*`, `🕹️interaction` | `HistoryReadLimits`; `SelectionSpec`, `HoverSpec`, `DomainSelection`, `InteractionState`, … | `replication-codec`, `replication-interaction` |
| `⏪️time-travel` | `MutationId`, `ReplayReport`, `MutationReplayOutcome`, `MutationMessage`, `InputReplacement`, `SupersededInput` | `ids`, `conflict`, `mutation`, `history` |
| `🎠️kernel` (+ identity) | command ingress, `ArtifactDiff`, `UndoPolicy`, presence codec, `SchemaId` | `command-ingress`, `causal`, `wire-core`, `presence`, `ids` |
| `🏗️mesh-engine` | `causal::transition` | `history`, `history-index` |
| `🕸️graph/🛂️manifest`, `🧪️test/🎮️mutation/🏭️inventory` | `MapDeltaTarget`; `MutationOutcomeClass`, `MutationLeafDescriptor` | `mutation` |
| `🛠️tool-machine` | `TransactionRef`, `HybridLogicalTimestamp`, `ActorId` | `edit`, `ids` |
| `🖱️ui/🎬️scene/*` | `value` only | `semio-framework-value` (peer ticket already) |

### 9.5 Profiles and locks

* 144 Cargo.toml files name `[profile.wasm-prototype.package.semio-framework-replication] opt-level = 2`. The pack carve added the new crate rows only to `🧰️framework/Cargo.toml` and `Cargo.toml`. Do the same, and list in the RESULT file that the other 142 workspace manifests keep `opt-level = 2` for the facade only, which is a wasm-prototype speed risk, not a correctness one; their generator, if any, should emit the 16 crate rows.
* Locks: hand-derive the 16 packages and the facade dependency lists in `🧰️framework/Cargo.lock` and `Cargo.lock` (pack precedent: root lock normal dependencies only, framework lock with dev-dependencies), or run `bun repo cargo … -- update --workspace --offline` when the launcher works. The other eight lock files (`🎓️teaching`, `✏️s`, `🌎️hub`, `🀄️wfc`, `🧩️puzzle`, `🧩️composition`, `🧹️fixture-sweep`, `🌊️flow`) refresh with their owners' route.

## 10. cfg targets, features and dependencies

### 10.1 Targets

| target | what to run | notes |
|---|---|---|
| native (`aarch64-apple-darwin` host cfg) | census lib and test per crate | no `target_os` gates in the tree |
| `wasm32-unknown-unknown` | census lib (equal to native) and test, oracle `--target` | `target_arch = "wasm32"` TRUE here; `wasm-bindgen` dep is dead (10.3) |
| `wasm32-wasip2` | census lib, oracle `--target` | `target_env = "p2"`: the facade's only target-gated dependency is `wasm-bindgen` for `not(p2)` |
| `deflate` on/off | census lib/test for `codec`, `format`, facade; oracle with `--cfg 'feature="deflate"'` | features forwarded as in 6.2, 6.4, 7.1; consumers `pack` and `os` forward `replication/deflate` |

### 10.2 Per-crate census expectations (after the carve, native, `--cfg test`; estimates, plus or minus 8 percent; lib = lib closure)

| crate | lib lines / KB | test lines / KB / files | note |
|---|---:|---:|---|
| `ids` | 295 / 15 | 365 / 19 / 3 | |
| `codec` | 1,075 / 68 | 1,684 / 106 / 15 | +203 lines lib with `deflate` |
| `wire-core` | 1,204 / 77 | 1,892 / 139 / 23 | files tight |
| `format` | 1,215 / 62 | 1,902 / 106 / 10 | |
| `mutation` | 1,730 / 108 | 2,202 / 165 / 14 | |
| `edit` | 992 / 76 | 1,261 / 130 / 20 | |
| `causal` | 1,381 / 76 | 2,118 / 133 / 8 | |
| `conflict` | 583 / 36 | 766 / 64 / 8 | |
| `mutation-messages` | 989 / 86 | 1,446 / 173 / 23 | |
| `history-index` | 374 / 48 | 559 / 100 / 8 | |
| `history` | 1,371 / 111 | about 2,400 / 223 to 230 / 14 | fallback in 6.11 |
| `frames` | 1,026 / 54 | 1,492 / 84 / 5 | |
| `interaction` | 873 / 45 | 1,191 / 65 / 6 | |
| `presence` | 1,808 / 91 | 2,482 / 144 / 8 | |
| `local-interaction` | 922 / 65 | 1,204 / 180 / 13 | |
| `command-ingress` | 1,145 / 58 | 1,283 / 67 / 4 | |
| facade | about 600 / 40 | about 700 / 45 / 6 | plus `paged_seal_digest` as its own target |

Sums are higher than the old numbers because glue roots, import lists and the three duplicated canonical helpers are new; no crate is within 7 percent of a hard ceiling and only `history` (bytes) is within the soft target.

### 10.3 Dependencies worth a decision

* `wasm-bindgen` (target-gated) and `serde_json` (non-dev) are listed in the old manifest but no lib file uses either; drop them from the facade and do not carry them to any new crate (verify with the oracle's unresolved-extern list).
* `serde` stays where derives and manual impls exist (`ids`, `interaction`, `causal`, `codec`, `history-index`, `command-ingress`). It is the pre-existing "external runtime library" debt; replacing it requires the consumer-derive fan-out work already documented in the old manifest comment, not this carve.
* `semio-framework-pack-json` is new for `ids`, `wire-core`, `mutation` (canonical-tree fragments). It depends only on `value`; no cycle (checked against its manifest).

## 11. Work packages and protocol

### 11.1 Packages (disjoint source file sets; one executor each)

| WP | scope | crates | owns (source files; everything under the named folders incl. their `🧪️tests`) |
|---|---|---|---|
| 0 | infrastructure (serial, first) | all | the 16 `P/` dirs (Cargo.toml, project.json, glue roots as stubs), `🧰️framework/Cargo.toml`, `Cargo.toml`, `📋️build-owners.json`, both launch files, taxonomy admission of the new folders, the interface freeze of 11.2 |
| 1 | foundation | `ids`, `codec`, `wire-core`, `format` | `🆔️ids`, `⚙️codec`, `🔢️scalar`, `📖️dictionary`, `🧾️wire`, `📐️format`, `🦀️.rs` (module root), `🚪️io/🦀️.rs`, `🚪️io/💾️binary/🦀️.rs`, `🚪️io/💾️binary/📦️bytes`, `🚪️io/💾️binary/📑️operation-sequence`, `🚪️io/📝️text/🦀️.rs` |
| 2 | mutation | `mutation`, `edit` | `🎮️mutation/🦀️.rs`, `🗂️map`, `🌐️foreign`, `🧵️canonical`, `🧾️transaction`, `🎮️mutation/🧪️tests`, new `✏️edit`, `🚪️io/📝️text/🧵️canonical` (incl. the three new fragment folders) |
| 3 | messages and conflict | `mutation-messages`, `conflict` | `🎮️mutation/📨️messages`, `🎮️mutation/♻️retirement`, `🎮️mutation/🧳️prepared`, `⚔️conflict` |
| 4 | causal and history | `causal`, `history-index`, `history` | `🔗️causal` (whole tree), `🚪️io/💾️binary/🔗️causal`, `🚪️io/📝️text/🔗️causal` |
| 5 | wire split | `frames`, `interaction`, `presence` | `📡️wire/🦀️.rs`, `📡️wire/🧪️tests`, new `📡️wire/🕹️interaction`, new `📡️wire/👥️presence`, `🚪️io/👥️presence`, `👕️peer-overlay` |
| 6 | wire leaves | `local-interaction`, `command-ingress` | `📡️wire/🏠️local-interaction`, `📡️wire/🎮️command` |
| 7 | facade and closure | facade | `P/🦀️.rs`, `P/Cargo.toml`, `P/📜️script.ts`, `P/📋️project.json`, `🧬️retirement` (delete), `🧪️tests` (TS and root fixtures), TS readers, locks, census, the crate-partition law, `🔒️dependencies.json` if it lists replication consumers |

Size of work: WP1 about 4,800 moved lines across 4 crates; WP2 about 2,700; WP3 about 1,500; WP4 about 2,100; WP5 about 3,300 (the only real file cutting); WP6 about 2,100 (mounts and imports only). WP3, WP4 and WP6 are mostly mechanical.

### 11.2 Interface freeze (WP0 publishes it in the ticket; `📌️important.md` stays empty)

Crate names and ids of 4.2; per-crate root file and glue mount list (6.1 to 6.16); the new file paths (`🎮️mutation/✏️edit/🦀️.rs`, `📡️wire/🕹️interaction/🦀️.rs`, `📡️wire/👥️presence/🦀️.rs`, `🚪️io/📝️text/🧵️canonical/{🆔️ids,🌐️foreign,🧾️policy}/🦀️.rs`, `🆔️ids/🧪️tests/🔬️unit/🦀️.rs`); the visibility list of 5.5; the fragment ownership of 5.3 (WP1 writes the mount line in `ids` and `wire-core`, WP2 creates the fragment files; WP2 writes the `mutation` mount, the others never touch `🚪️io/📝️text/🧵️canonical`); retire-impl destinations of 5.4b (each WP appends its own impls).

### 11.3 Protocol

1. **Phase 0 (WP0):** create the 16 skeletons (manifest with the full dependency list of Appendix D, stub glue root with the mounts of 6.x commented out until the owning WP fills them, project.json), registries, launch routes; run the taxonomy validation; run the census for the baseline and record it. Do not touch the old facade glue.
2. **Phase 1 (WP1..WP6 in parallel):** each WP works in an overlay copy of the replication tree in the session scratchpad (the path-length rule forbids copies of checked-in sources in the ticket folder; scripts and logs go to `🗑️generated`, with `[DEBUG] ` prefixes removed at the end), runs the union-sandbox differential for its crates against the pristine baseline, then applies its diff to the live tree in one pass with content anchors (`assert text.count(anchor) == 1`) and a `git diff --stat` check, re-reading every file right before writing (peers edit concurrently). Order inside a WP is leaf-first as listed in section 4.2. Write `RESULT-W4-REPLICATION-WPn.md` with census rows. No WP edits another WP's files; the shared registries belong to WP0 and WP7.
3. **Phase 2 (WP7, serial):** facade glue with generated explicit lists and the explicit varint names, facade Cargo.toml, `Cargo.lock` updates, scripts and TS readers of 9.3, delete `🧬️retirement/🦀️.rs`, run the union differential for the whole set, the census for all 17 crates (lib, test, wasm, `deflate`), the crate-partition law, the TS tests of the replication tree, and the Nx/cargo attempt of 12.7. Write `RESULT-W4-REPLICATION.md`.

Executor rules that matter: no `git commit/stash/checkout`, no worktrees, no sub-agents, foreground builds only, no compiling around the ingress gate, `[DEBUG] ` prefix for temporary logs, no `ticket_close` from executors, no `🗑️generated` sweep, `/usr/bin/grep` instead of the aliased `grep`, macOS has no `timeout`.

## 12. Verification

Per crate and for the whole set; every command's actual output goes into the RESULT file.

1. **Baseline census before the first edit** (`ONLY=semio-framework-replication bun census.ts`, then `census_stage.ts` style rows for lib, test, wasm and `deflate`), and a pristine copy of the tree at HEAD for the differential.
2. **Patched full-resolution differential** (`📓️RESOLUTION-ORACLE.md` with its two sandbox patches; pack-carve `union.py`/`oracle.py`): simulate the 16 crates as modules of the facade crate, `pub(crate)` rewritten to `pub(in <crate module>)`, `rustc +nightly-2026-07-20 --edition 2021 --emit=metadata -o /dev/null` in native, `--test`, `wasm32-unknown-unknown`, `wasm32-wasip2` and `deflate` on/off; compare internal error multisets with the baseline; a seeded sensitivity check (make one widened item private again and expect `E0603`).
3. **Orphan scan** (`orphan.py`) over every impl of every new crate, plus a re-run of the 494-item check of this plan; target 0 violations. **Definition and privacy scans** (`privscan.py`, `privscan2.py`): every cross-crate private method or field; the oracle cannot report `E0616`, `E0624`, `E0451`, trait-in-scope or coherence.
4. **Losslessness** (`lossless.py`): trimmed non-blank line multiset over all touched `.rs` files; the differences must be exactly the path rewrites, imports, mounts, `pub` widenings and new docstrings.
5. rustfmt parse (`+nightly-2026-07-20`, `skip_children`) of every touched file; `#[path]` and `include!/include_str!` target existence for all 16 roots and the facade; longest new path.
6. **Static registry check**: manifests parse (Bun TOML), `workspace` resolves to `🧰️framework`, `[lib] path` and every path dependency exist, boundary keys exact, members present and byte-sorted, owner rows and project names unique, 96 launch commands present exactly once per file with unique order keys, `launch.json`, the seed and `📋️build-owners.json` parse, both lock files contain all 16 packages.
7. **Cargo and Nx attempt, once at the start and once at the end, foreground:** `bun repo run @semio-tech/replication-<x>-rs:check --configuration=prototype --parallel=1 -- --lib --offline` (and `test`), or `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-replication-<x> --message-format=short` with a `--tests` and a wasm `--target` variant. The recent failures were `Finite original maintenance authority required`, `Repository target execution contract refused`, `Active graph readers retain required Nx pool headroom`; if the launcher admits a crate, run check and test for real and fix every error; otherwise say so.
8. **TS tests** of the replication tree (`🧪️tests/**`, `🚪️io/🧪️tests/🏛️ownership`, `⚠️error`, command-ingress ownership, the new law) before and after; unchanged outcomes except the intended ownership-law update.
9. **Honesty:** the Rust unit and law tests are moved, not rewritten; unless step 7 runs them, the RESULT must say that they were checked by the oracle only (name resolution), not type-checked or executed.

## 13. Risks

1. **First real compile.** Privacy of fields and methods, trait-in-scope for method calls, inference changes caused by import rewrites and coherence cannot be seen by the oracle. Run check for all 17 crates with `--tests`, wasm32 and wasip2, `deflate` on and off at the first opportunity.
2. **Facade ambiguity and surface drift.** Two duplicate public names (5.5) and about 490 re-exported names: the generated lists and the public-path conformance test (7.2) are the guard; any consumer-visible name missed breaks a far-away crate. Compare the facade list with the baseline item table, not with memory.
3. **`history` test closure at the soft limit** (6.11) and `wire-core` / `mutation-messages` at 23 files; adding a test file to any of them needs an offsetting move.
4. **Orphan fixes change file ownership of three small impl groups** and touch the TS ownership law (9.3); if the ownership fixture is not updated in the same change, `🚪️io/🧪️tests/🏛️ownership` fails.
5. **Taxonomy admission of the new folders** is unverified here (the taxonomy file is 1.1 MB); fallback names are in 9.2.
6. **`wasm-prototype` profile drift** (144 manifests) and **ten lock files** (9.5).
7. **Peer churn.** `replication-error`, `replication-crypto` and `mutation-diff` are peer-owned; `pack`, `os-kernel`, `plugin`, `ui`, `job`, `2d` carves are in flight. Never edit them for the facade; consumer retargets (9.4) wait until the facade is green.
8. **Test allocator.** Exactly one `#[global_allocator]` per test binary; five crates get their own `cfg(test)` copy (6.0) and the facade none unless a facade test needs it. A crate whose tests do not need it must not get one.
9. **Duplicated helper code.** `absent`/`selected` (8 lines) exist in four canonical fragments; acceptable (close to each other, one tree) but flagged for a later lift into `pack-json` if its owner agrees.
10. **Method-level references** are the blind spot of the item graph (section 3, point 6): trait objects and generic bounds between `frames`/`presence`/`causal` were read manually for the three highest-risk pairs, not proven.
11. **Source drift.** This plan was written against HEAD `f856dff0813` plus the working tree of 2026-10-15; the Region and line anchors of Appendix B must be re-derived from markers.

## Appendix A. Files per crate (lib files with gate lines; test sources; fixtures with bytes)

Paths relative to `R/`. `(module root)` is `R/🦀️.rs`. Lines are active lines under `--cfg test`. Split files are in Appendix B. `retained` below belongs to `wire-core`.

##### semio-framework-replication-ids
- lib: 🆔️ids (257)
- tests: -
- fixtures: -

##### semio-framework-replication-codec
- lib: ⚙️codec/🎟️history-limits (13); ⚙️codec (476); ⚙️codec/🆔️ids (68); 🚪️io/💾️binary/📑️operation-sequence (12); 🚪️io/💾️binary/📦️bytes (383); 🚪️io/💾️binary (45); 🚪️io/📝️text (22); 🚪️io (16)
- tests: ⚙️codec/🧪️tests/🔬️unit (213); ⚙️codec/🆔️ids/🧪️tests/🔬️unit (32); 🚪️io/💾️binary/📦️bytes/🧪️tests (364)
- fixtures: 📦️bytes/🧪️tests/🧫️fixtures (637); 📦️bytes/🧪️tests/🧫️fixtures (420); 📦️bytes/🧪️tests/🧫️fixtures (97)

##### semio-framework-replication-wire-core
- lib: ⚙️codec/🛬️retained/🔣️json-string (17); ⚙️codec/🛬️retained (199); 📖️dictionary (87); 📖️dictionary/🛬️decode (111); 🔢️scalar (362); 🧾️wire (335)
- tests: ⚙️codec/🛬️retained/⏱️timestamp/🧪️tests (15); ⚙️codec/🛬️retained/🔣️json-string/🧪️tests (16); ⚙️codec/🛬️retained/🧪️tests (67); ⚙️codec/🛬️retained/🧾️source/🧪️tests (19); 📖️dictionary/🛬️decode/🧪️tests (21); 📖️dictionary/🛬️decode/🧾️source/🧪️tests (19); 🧾️wire/🧪️tests/🔬️unit (531)
- fixtures: ⏱️timestamp/🧪️tests/🧫️fixtures (352); 🔣️json-string/🧪️tests/🧫️fixtures (631); 🛬️retained/🧪️tests/🧫️fixtures (448); 🛬️retained/🧪️tests/🧫️fixtures (387); 🧾️source/🧪️tests/🧫️fixtures (491); 🪪️id/🧪️tests/🧫️fixtures (190); 🛬️decode/🧪️tests/🧫️fixtures (337); 🧾️source/🧪️tests/🧫️fixtures (259)

##### semio-framework-replication-format
- lib: 📐️format/🔎️verification (253); 📐️format/🔎️verification/🧾️record (46); 📐️format (800); (module root) (94)
- tests: 📐️format/🔎️verification/🧪️tests/🔬️unit (238); 📐️format/🔎️verification/🧾️record/🧪️tests/🔬️unit (77); 📐️format/🧪️tests/🔬️unit (372)
- fixtures: 🔎️verification/🧪️tests/🧫️fixtures (5637); 🧾️record/🧪️tests/🧫️fixtures (2851)

##### semio-framework-replication-mutation
- lib: `🎮️mutation` root (part of `🎮️mutation/🦀️.rs`, Appendix B.2);  🎮️mutation/🌐️foreign (326); 🎮️mutation/🗂️map (170)
- tests: 🎮️mutation/🌐️foreign/🧪️tests (109); 🎮️mutation/🗂️map/🧪️tests/🧪️shared-map-delta-native (66); 🎮️mutation/🧪️tests/🔬️mutation-leaf-metadata (175); 🎮️mutation/🧪️tests/🧪️outcome-code (122)
- fixtures: 🌐️foreign/🧪️tests/🧫️fixtures (1166); 🗂️map/🧪️tests/🧫️fixtures (8775); 🧪️tests/🧫️fixtures/⚠️value-refusals (480); 🧪️tests/🧫️fixtures/🤝️mutation-leaf-contract (559); 🧪️tests/🧫️fixtures/🧫️outcome-code (2441); 🧪️tests/🧫️fixtures/🧭️mutation-leaf-source-contract (5198)

##### semio-framework-replication-edit
- lib: new `🎮️mutation/✏️edit` root (part of `🎮️mutation/🦀️.rs`, B.2);  🎮️mutation/🧵️canonical (25); 🎮️mutation/🧵️canonical/🪪️identity (61); 🎮️mutation/🧵️canonical/🛂️authority (85); 🎮️mutation/🧾️transaction/🎟️admission (25); 🚪️io/📝️text/🧵️canonical/🔏️seal (124); 🚪️io/📝️text/🧵️canonical (127)
- tests: 🎮️mutation/🧪️tests/🧪️transaction-ref (59); 🎮️mutation/🧵️canonical/🪪️identity/🧪️tests (31); 🎮️mutation/🧵️canonical/🛂️authority/🧪️tests (38); 🎮️mutation/🧾️transaction/🎟️admission/🧪️tests (20); 🚪️io/📝️text/🧵️canonical/🔏️seal/🧪️tests (63); 🚪️io/📝️text/🧵️canonical/🧪️tests (58)
- fixtures: 🧪️tests/🧫️fixtures/🧫️transaction-ref (1481); 🧪️tests/🧫️fixtures/🛂️mutation-source-authority (2733); 🛂️authority/🧪️tests/🧫️fixtures (4529); 🎟️admission/🧪️tests/🧫️fixtures (263); 🔏️seal/🧪️tests/🧫️fixtures (669); 🧵️canonical/🧪️tests/🧫️fixtures (3734)

##### semio-framework-replication-causal
- lib: 🔗️causal (774); 🚪️io/💾️binary/🔗️causal (333); 🚪️io/📝️text/🔗️causal (231)
- tests: 🔗️causal/🧪️tests/🔬️unit (737)
- fixtures: 📦️slots/🧪️tests/🧫️fixtures (439); 🧬️mutations/➕️causal-add/🧪️descriptor (583); 🧪️tests/🧫️fixtures/🧮️document-backbone-batch-v1 (18224)

##### semio-framework-replication-conflict
- lib: ⚔️conflict/♻️report (4); ⚔️conflict/♻️retirement (33); ⚔️conflict (503)
- tests: ⚔️conflict/♻️retirement/🧪️tests (75); ⚔️conflict/🧪️tests/🔬️unit (108)
- fixtures: ♻️retirement/🧪️tests/🧫️fixtures (2674); 🧪️tests/🧫️fixtures/🧫️replay-report (5539)

##### semio-framework-replication-mutation-messages
- lib: 🎮️mutation/♻️retirement (126); 🎮️mutation/📨️messages/♻️retire (127); 🎮️mutation/📨️messages/✂️clamp (120); 🎮️mutation/📨️messages/📋️copy (38); 🎮️mutation/📨️messages/📦️accumulate (144); 🎮️mutation/📨️messages/🧵️compose (339); 🎮️mutation/🧳️prepared/♻️retirement (38); 🎮️mutation/🧳️prepared (17)
- tests: 🎮️mutation/♻️retirement/🧪️tests/🔬️unit (52); 🎮️mutation/📨️messages/♻️retire/🧪️tests (115); 🎮️mutation/📨️messages/✂️clamp/🧪️tests (24); 🎮️mutation/📨️messages/📋️copy/🧪️tests/🔬️unit (7); 🎮️mutation/📨️messages/📦️accumulate/🧪️tests (97); 🎮️mutation/📨️messages/🧵️compose/🧪️tests (130); 🎮️mutation/🧳️prepared/♻️retirement/🧪️tests (32)
- fixtures: ♻️retirement/🧪️tests/🧫️fixtures (458); ♻️retire/🧪️tests/🧫️fixtures (13961); ✂️clamp/🧪️tests/🧫️fixtures (6679); 📋️copy/🧪️tests/🧫️fixtures (621); 📦️accumulate/🧪️tests/🧫️fixtures (2165); 🧵️compose/🧪️tests/🧫️fixtures (10531); ♻️retirement/🧪️tests/🧫️fixtures (832)

##### semio-framework-replication-history-index
- lib: 🔗️causal/🔀️transition/🔁️fold/🗂️index (280); 🔗️causal/🔀️transition/🔁️fold/🗂️index/🪴️insert (64)
- tests: 🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests (185)
- fixtures: 🧪️tests/🧫️fixtures/🌱️entry (603); 🧪️tests/🧫️fixtures/🎟️insertion (18010); 🗂️index/🧪️tests/🧫️fixtures (3041); 🗂️index/🧪️tests/🧫️fixtures (494)

##### semio-framework-replication-history
- lib: 🔗️causal/🔀️transition/📝️drafts (51); 🔗️causal/🔀️transition/🔁️fold (651); 🔗️causal/🔀️transition (594)
- tests: 🔗️causal/🔀️transition/📝️drafts/🧪️tests (48); 🔗️causal/🔀️transition/🔁️fold/📏️retirement/🧪️tests (187); 🔗️causal/🔀️transition/🧪️tests/🔬️unit (793)
- fixtures: 📝️drafts/🧪️tests/🧫️fixtures (530); 📏️retirement/🧪️tests/🧫️fixtures (1362); 📏️retirement/🧪️tests/🧫️fixtures (432); 🔁️fold/🧪️tests/🧫️fixtures (1386); 🧪️tests/🧫️fixtures/🗄️durable-collaborative-redo-v1 (6560); 🧪️tests/🧫️fixtures/🧫️history-transition (11846); 🧪️tests/🧫️fixtures/🧫️supersede-fold (16643)

##### semio-framework-replication-frames
- lib: `📡️wire/🦀️.rs` frames part (B.1)
- tests: 📡️wire/🧪️tests/🔬️unit (466)
- fixtures: 🧫️fixtures/📡️wire/🚫️legacy-client-hello-rejected (63); 🧪️tests/🧫️fixtures/🚀️artifact-bootstrap (3345)

##### semio-framework-replication-interaction
- lib: new `📡️wire/🕹️interaction` root (part of `📡️wire/🦀️.rs`, B.1)
- tests: 📡️wire/🧪️tests/🔬️interaction-wire (89); 📡️wire/🧪️tests/🔬️interaction (203); 📡️wire/🧪️tests/🖱️typed-field-order (26)
- fixtures: 🧪️tests/🧫️fixtures/🖱️typed-field-order (1309)

##### semio-framework-replication-presence
- lib: new `📡️wire/👥️presence` root (part of `📡️wire/🦀️.rs`, B.1); 👕️peer-overlay (207); 🚪️io/👥️presence/🔍️borrowed (167)
- tests: 👕️peer-overlay/🧪️tests/🔬️unit (167); 📡️wire/🧪️tests/🔬️assemble-presence-interaction (79); 📡️wire/🧪️tests/🔬️presence-codec (428)
- fixtures: 🧪️tests/🧫️fixtures/👥️presence-peer-codec-v1 (13287)

##### semio-framework-replication-local-interaction
- lib: 📡️wire/🏠️local-interaction/🌳️root (40); 📡️wire/🏠️local-interaction/🌳️root/🩹️update (108); 📡️wire/🏠️local-interaction/📡️transport (291); 📡️wire/🏠️local-interaction (453)
- tests: 📡️wire/🏠️local-interaction/🌳️root/🧪️tests/🌳️root (69); 📡️wire/🏠️local-interaction/🌳️root/🩹️update/🧪️tests/🩹️update (89); 📡️wire/🏠️local-interaction/📡️transport/🧪️tests/📡️transport (78); 📡️wire/🏠️local-interaction/🧪️tests/🏠️local-interaction (46)
- fixtures: 🌳️root/🧪️tests/🧫️fixtures (554); 🩹️update/🧪️tests/🧫️fixtures (635); 📡️transport/🧪️tests/🧫️fixtures (624); 🧪️tests/🧫️fixtures/🏠️local-interaction (91686)

##### semio-framework-replication-command-ingress
- lib: 📡️wire/🎮️command/📥️ingress (1115)
- tests: 📡️wire/🎮️command/📥️ingress/🧪️tests/🔬️unit (138)
- fixtures: 🧪️tests/🧫️fixtures/📄️pages (1676)

##### Fragments created by the carve
- `🚪️io/📝️text/🧵️canonical/🆔️ids/🦀️.rs` (`ids`), `🚪️io/📝️text/🧵️canonical/🌐️foreign/🦀️.rs` (`mutation`), `🚪️io/📝️text/🧵️canonical/🧾️policy/🦀️.rs` (`wire-core`), `🆔️ids/🧪️tests/🔬️unit/🦀️.rs` (`ids`), `🧪️tests/🔬️facade/🦀️.rs` (facade), `🧪️tests/🏛️crate-partition/🟦️.ts` (law).
- Deleted: `🧬️retirement/🦀️.rs` (distributed, 5.4b).

## Appendix B. Split-file anchors (re-derive from region markers; line numbers as measured)

### B.1 `📡️wire/🦀️.rs` (3,253 lines, 166,382 bytes)

| part | content | regions / items | lines (measured) |
|---|---|---|---|
| keep (`frames`) | module doc, `use std::collections::BTreeMap` (move to `presence` if only the assembler uses it), `🔖️Lane`, `🔖️ClientFrame`, `🔖️ServerFrame`, `🔖️Codec` with the frontier/envelope combinators, `🔖️NestedEnums`, `🧪️Tests` mount of `🧪️tests/🔬️unit` | | 1 to 1,098 minus 601 to 614, 616 to 629, 646 to 660 (M7) and minus line 19 (re-export) |
| to `wire-core` (M7) | `write_opt_str`, `read_opt_str`, `write_opt_bytes`, `read_opt_bytes`, `write_vec_bytes`, `read_vec_bytes` | region `🔖️Combinators` (the other four stay) | 601 to 614, 616 to 629, 646 to 660 |
| to `presence` | `🔖️Presence` (including `🔖️PresenceView`, `🔖️PresenceViewCodec`) and the `presence_codec_tests` mount | | 1,100 to 2,224 except the `interaction_wire_tests` mount at 2,220 to 2,222 |
| to `presence` | `🔖️PresenceInteraction` (`PresenceInteraction`, `PresenceDomain`, `🔖️PresenceInteractionCodec`, `🔖️Assemble`) and the `assemble_presence_interaction_tests` mount | nested in `🔖️Interaction` today | 3,075 to 3,246 |
| to `interaction` | `🔖️Interaction` (HoverSpec to `default_pointer_channels`), `🔖️Runtime`, `🔖️Topology`, `🔖️SelectionMachine`, `🔖️HoverMachine`, `🔖️Validation`, `🔖️InteractionStorePack`; mounts `interaction_wire_tests` (2,220 to 2,222), `interaction_tests` (3,252 to 3,253), `typed_field_order_tests` (3,257 to 3,258) | | 2,226 to 3,073, 3,248 to 3,254 and the three mounts |

Items by name for the cuts: presence = `PresenceWindowView`, `PresenceViewKind`, `PresenceUi`, `PresencePeer`, `PresenceTyping`, `PresencePrincipalKind`, `PresenceToolRunState`, `PresenceToolRun`, `PresenceHistoryEditStage`, `PresenceHistoryEdit`, `PresencePeerWireLimitsV1`, `PRESENCE_PEER_WIRE_LIMITS_V1`, `PRESENCE_TYPING_EXCERPT_BYTES`, `PresencePeerReader`, `encode_presence_*`, `decode_presence_*`, `write_vec_presence_window_view`, `PresenceInteraction`, `PresenceDomain`, `write_vec_str`, `read_vec_str`, `assemble_presence_interaction`. Interaction = `HoverSpec`, `SelectionSpec`, `HierarchyProvider`, `SelectionMode`, `SelectionMethod`, `MergeMode`, `InteractionTarget`, `DomainSelection`, `DomainHover`, `InteractionState`, `TopologyNode`, `DomainTopology`, `InteractionTopology`, `SelectionInput`, `HoverInput`, `InteractionOutline`, `next_selection`, `next_hover`, `validate_state`, `dedup_preserving_order`, `visit_descendants`, `default_true`, `default_pointer_channels`.

### B.2 `🎮️mutation/🦀️.rs` (1,658 lines, 87,836 bytes)

| part | content | lines (measured) |
|---|---|---|
| keep (`mutation`) | header, `map` mount and re-export (11 to 13), `MUTATION_MESSAGE_*` constants (16 to 20), region `🔖️Mutation` (40 to 153), `🪪️MutationLeafDescriptor` (155 to 647), `🪪️MutationLeaf` (649 to 890), leaf-metadata test mount (893 to 894), `🔖️Message` (896 to 1,238), `🔖️Foreign` (1,246 to 1,250), outcome-code test mount (1,549 to 1,550), `apply_operation` (1,648 to 1,659) | |
| delete | message-ledger and replay-retirement mounts and re-exports | 21 to 38, 1,639 to 1,641 |
| to `edit` | `🔖️Meta` (1,252 to 1,551 minus the outcome-code mount), `🔖️Origin` (1,553 to 1,635), `transaction_admission` mount (1,541 to 1,542), `transaction_ref_tests` mount (1,545 to 1,546), `canonical_fields` mount and re-exports (1,643 to 1,646) | |

### B.3 Other cuts

* `🚪️io/📝️text/🧵️canonical/🦀️.rs` (127 lines): ids fragment lines 56 to 68 and 94 to 100 (inherent `HybridLogicalTimestamp::canonical_field`, trees for HLC and five ids and `PayloadHash`); foreign fragment 69 to 81; policy fragment 101 to 104. Remains: `selected`, `absent`, kind statics, `Edit`, `MutationMeta`, `TransactionRef`, `MutationOrigin`, the seal mount and test mount.
* `🎮️mutation/🧵️canonical/🦀️.rs`: lines 11 to 19 (`ForeignTarget`, `PayloadHash`, `UndoPolicy` retire impls) leave; `MutationMeta` and `Edit` retire impls stay.
* `📐️format/🦀️.rs` lines 788 to 798 (region `🔖️Crypto`, `Blake3Hasher` + impl) leave for `🧾️wire/🦀️.rs`.
* `🧾️wire/🧪️tests/🔬️unit/🦀️.rs`: lines 310 to 377 to `🆔️ids/🧪️tests/🔬️unit`; lines 428 to 445 to the facade tests.
* `🧬️retirement/🦀️.rs` (116 lines): distribute per 5.4b, delete.
* `🔗️causal/🔀️transition/🔁️fold/🦀️.rs` line 11 to 12 (`mod index`): delete; imports of `HistoryFoldIndex*`/`HistoryFoldSet*` from `history-index`.

## Appendix C. Reproducing the measurements

* Baseline and per-crate rows: `cd 🪓️monolith-split/🗑️generated && ONLY=<crate list> bun census.ts` (≈ 47 s, reads the whole workspace); test, wasm and `deflate` rows with the `readRustSourceClosureV1` call of section 3, point 1 (cfg lists: host + `debug_assertions` [+ `test`] [+ `feature=deflate`]; wasm: `rustc --print cfg --target wasm32-unknown-unknown`).
* Public surface: all `pub` top-level items of the 57 files (about 490) plus every `pub use`; the facade lists are generated from this table in WP7.

## Appendix D. Dependency table for the manifests (token scan plus the DAG; the oracle's unresolved-extern list is authoritative)

| crate | first-party (replication) | first-party (other) | third party | dev |
|---|---|---|---|---|
| `ids` | - | `value`, `pack-json` | `serde` | `serde_json` |
| `codec` | `replication-error` | `pack-error`, `value`, `diagnostic`, `io-base64`, `deflate` (opt) | `serde` | `async-macros`, `serde_json`, `trace-memory-testing` |
| `wire-core` | `codec`, `replication-error`, `replication-crypto` | `pack-error`, `value`, `diagnostic`, `hash`, `pack-json` | - | `async-macros`, `serde_json`, `trace-memory-testing` |
| `format` | `codec`, `wire-core`, `replication-error`, `replication-crypto` | `pack-error`, `pack-source`, `hash`, `value` | - | `blake3`, `async-macros`, `serde_json` |
| `mutation` | `ids`, `wire-core`, `mutation-diff` | `value`, `diagnostic`, `schema-state`, `pack-json` | - | `serde_json`, `trace`, `trace-memory-testing` |
| `edit` | `mutation`, `ids`, `wire-core`, `replication-crypto` | `value`, `hash`, `pack-json` | - | `blake3`, `serde_json`, `trace-memory-testing` |
| `causal` | `mutation`, `edit`, `ids`, `codec`, `wire-core`, `replication-error` | `value` | `serde` | `serde_json`, `trace-memory-testing` |
| `conflict` | `causal`, `mutation`, `ids`, `wire-core` | `value`, `diagnostic`, `hash` | - | `async-macros`, `serde_json`, `trace` |
| `mutation-messages` | `mutation`, `conflict`, `replication-error` | `value`, `diagnostic` | - | `serde_json`, `trace` |
| `history-index` | `replication-error` | `value`, `pack-json` | `serde` | `serde_json`, `trace` |
| `history` | `causal`, `edit`, `ids`, `history-index`, `wire-core`, `replication-crypto`, `replication-error` | `value`, `hash` | - | `blake3`, `serde_json`, `trace-memory-testing` |
| `frames` | `causal`, `ids`, `wire-core`, `replication-error` | `value`, `hash` | - | `async-macros`, `serde_json` |
| `interaction` | - | `value`, `pack-json` | `serde` | `async-macros`, `serde_json` |
| `presence` | `interaction`, `wire-core`, `replication-error` | `value` | - | `async-macros`, `serde_json` |
| `local-interaction` | `interaction`, `replication-error` | `value` | - | `serde_json`, `trace-memory-testing` |
| `command-ingress` | - | `trace`, `diagnostic`, `value`, `value-derive` | `serde` | `serde_json` |
| facade | the 16 crates, `replication-error`, `replication-crypto` | `mutation-diff`, `pack-error`, `pack-source`, `pack-json`, `hash`, `value`, `diagnostic`, `trace`, `value-derive`, `deflate` (opt) | `serde` | `trace-memory-testing`, `schema-registry`, `schema-state`, `serde_json`, `blake3`, `async-macros` |
