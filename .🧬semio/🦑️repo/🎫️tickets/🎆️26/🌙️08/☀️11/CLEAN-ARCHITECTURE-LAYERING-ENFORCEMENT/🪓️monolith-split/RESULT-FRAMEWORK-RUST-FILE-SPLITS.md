# Framework Rust File Splits (slice FRAMEWORK-RUST-FILE-SPLITS, prefix `frs`)

Executor: Sonnet. Date: 2026-10-16. Folder `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`.

## 1. Verdict

All four targets are split, installed and proven by static evidence. The real `bun repo cargo` check is still refused (`Insufficient physical compiler initial headroom`), so no compile claim is made.

| Target | Lines before | Parent after | Children (lines) | Largest child |
| --- | --- | --- | --- | --- |
| `🧰️framework/🔨️modules/🔢️number/🦀️.rs` | 3,271 | 64 | 8 (249 to 573) | 573 |
| `🧰️framework/🔨️modules/⏳️async/🦀️.rs` | 2,931 | 222 | 17 (39 to 511) | 511 |
| `🧰️framework/🔨️modules/🗣️dsl/📖️grammar/🦀️.rs` | 2,779 | 52 | 6 (189 to 976) | 976 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` | 3,323 (census said 3,325; the live file had a peer edit of 2 lines) | 104 | 7 (97 to 1,205) | 1,205 |

Public paths are unchanged. Every item a caller names stays reachable at the old crate path through explicit `pub use`. No caller changed.

## 2. What was cut

Children are `<emoji>️<kebab>/🦀️.rs`, wired in the parent with `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus explicit `pub use` (public items) and explicit private `use` (items other files still name). Existing mounted children (`cancel_waiters`, `maintenance`, `literal`, `archive_schema`, ...) stay mounted from the parent, so their `super::` paths are untouched. Every `//#region` marker moved with its cluster; no marker was dropped.

### number (`🔢️number/`)
`💍️traits`, `🧱️natural`, `➕️integer`, `➗️rational`, `🕰️modular`, `🔑️primes`, `📏️interval`, `🧮️algebra`. These are the eight inline `pub mod` blocks lifted verbatim (module identity unchanged, so no visibility change). Outer region markers stay in the parent around each mount. Each `include!("🧪️tests/...")` became `include!("../🧪️tests/...")`. Two docstring emoji were changed to stay unique (`natural` 🔢 to 🧱, `primes` 🧮 to 🔑).

### async (`⏳️async/`)
Inline modules (identity unchanged): `🌐️browser`, `🏷️schema-metadata`, `🪢️oneshot`, `🖥️native-pool`, `🕸️wasm-pool`, `🎭️testkit`.
Clusters moved into new modules: `🪪️operation-context`, `⛔️cancel-token` (`CancelState`, `CancelNode`, `CancelToken`, `Cancelled`), `🌳️scope` (scopes, channel policy, `HostAsyncRuntime`), `🌉️block-on` (cfg `any(test, feature = "entrypoint")` on the mount), `🛣️lane` (`ProcessKind`, `Lane`, `PermitLedger`), `⏰️timer-wheel`, `🚦️sync-primitives`, `🔀️combinators`, `🗺️sharded-map`, `🧱️boxed-slots`, `🧵️worker-pool`.
Root import hygiene: dead std imports dropped; names only tests/testkit need are cfg-gated (`Future`, `Pin`: `any(test, feature="testkit")`; `Context`, `Poll`, `AtomicU32`: `test`); `native_pool` got its own `use std::sync::atomic::AtomicU32;`.

### grammar (`🗣️dsl/📖️grammar/`)
`🧱️model` (dialect, productions, protocol model), `🔡️lexer`, `🌳️parser` (one child because the `Parser` region is one region and the shared `Cursor` serves both parsers), `🖨️writer` (incl. the empty `FromRecordSpec` region), `🧭️recognizer`, `🧮️walker`. The `📡️literal` mount stays in the parent.

### channel (`📡️spr/🧵️channel/`)
`🪪️wire-entries`, `🚚️archive-load` (load state/status types plus `DocumentArchiveLoadHost`), `👥️presence-roster`, `🧩️command-decode` (the one `PagedAppCommandDecode` region), `📨️app-command`, `🎞️app-frame`, `🗜️codec` (one `Codec` region incl. the trailing `encode_local_interaction_query_frame_into`). Parent keeps version, handshake, the archive mounts, tests and the allocator static. Two folders were renamed after the taxonomy check (section 6): `🔣️codec` to `🗜️codec` (the 🔣 emoji resolves to the structural `json-fixture-case` kind) and `👥️presence` to `👥️presence-roster` (the plain slug resolves to the structural `presence` kind). The `app-frame` docstring starts with 🎬 because the parent docstring already starts with 🎞.

### Visibility
A private item/field/method moved into a new module is widened to `pub(super)` when its name is referenced in another file (token scan, conservative). For a root-level private item this is exactly its previous reach (the parent module and its descendants). Counts of widened lines (each is one removed line plus the same line with `pub(super)`): async 34, grammar 16, channel 46, number 0. Fields and methods use separate access-form scans (`.name`, `name:` versus `.name(`, `::name`), so unrelated common names widen less. The widened lines are listed class-by-class in `🗑️generated/frs-lossless-final.log`.

## 3. Verification

### 3.1 Lossless (line multiset, live tree)
`python3 frs-tools/lossless.py` (log `🗑️generated/frs-lossless-final.log`). Stripped non-empty line multiset of the original versus all installed files of the target:

| Target | removed | added | explained by |
| --- | --- | --- | --- |
| number | 26 | 27 | 8 mount heads (`pub mod x {`), 8 closing braces, 8 re-based `include!`, 2 docstring emoji edits; added: 16 wiring lines, 8 `include!`, 2 docstrings, plus 1 `// #endregion 🔖️StringIo` (the original `natural` module opened the region without closing it; HEAD was already unbalanced) |
| async | 51 | 152 | 34 lines to `pub(super)` variants, 6 inline-mod braces, 2 `include!`, 9 removed/changed imports; added: 99 wiring lines, 17 docstrings |
| grammar | 17 | 52 | 16 widened lines, 1 import line replaced by pruned form; added 30 wiring lines, 6 docstrings |
| channel | 47 | 88 | 46 widened lines, 1 import line replaced; added 35 wiring lines, 7 docstrings |

No other line is added or removed. Dedent of inline modules (number, async) is whitespace only and was checked by a lexer-based scan that none of the files has a multi-line string literal.

### 3.2 Patched full-resolution differential (RESOLUTION-ORACLE.md)
Tool: `frs-tools/oracle.py` (baseline sandbox from the pre-edit snapshot versus a sandbox copy of the live tree, `rustc +nightly-2026-07-20 --emit=metadata`, output file in the session scratchpad and deleted at once, never in the repo; `-o /dev/null` is refused by the tool sandbox on `/dev/rmeta*`). Compared: sorted multiset of (error code + message, primary-span label), module path prefixes and the sandbox root normalised.

Deviation to record: with the standard patches alone, rustc aborts before type-check for async, grammar and channel (unresolved external crates in item signatures), so field/method privacy (E0616/E0624/E0451) was invisible. I proved this by mutation (a type error in a body was not reported). I therefore added a sandbox-only stub layer (`frs-tools/stubgen.py`, applied identically to baseline and current sandbox, never to the repo): paths rooted at external crates are rewritten to `crate::__ext::<crate>::...`, a generated stub module is appended to the lib root, external derives/serde attributes are stripped, and (grammar only) the names the neutralised `pub use lexer::*` globs provided are stubbed. With the stubs rustc type-checks, and negative controls prove it sees visibility errors.

Configurations and results on the live tree (final runs):

| Crate | configurations | result |
| --- | --- | --- |
| number | lib, `--test`, wasm32-unknown-unknown, wasm32-wasip2, both with `--test` | 0 errors in baseline and current: a real type-check, IDENTICAL x6 (log `frs-run-number-live2.log`) |
| async | the same six plus `testkit`, `entrypoint` and both features, each lib and test | baseline 163 to 499 stub-noise errors, IDENTICAL x12 (`frs-run-async-live2.log`) |
| grammar | six | IDENTICAL x6 (`frs-run-grammar-live2.log`) |
| channel | six | IDENTICAL x6 after the two folder renames (`frs-run-channel-final.log`) |

Negative controls (mutating the current sandbox only; logs `frs-*-oracle-neg*.err`, `frs-*-oracle-cneg*`, `frs-grammar-oracle-gneg*`):
- number: `pub fn from_u64` made private gave 19x E0624; a wrong `include!` path gave "couldn't find file".
- async: removing `pub(super)` from `CancelNode.local`, `TimerWheel::schedule_callback`, `LANE_COUNT` gave E0616, 5x E0624, E0603.
- grammar: un-widening `GToken.kind` and `mismatch` gave 51x E0616, E0603 plus E0432.
- channel: un-widening `malformed` and `MediaExportStateWire::wire` gave E0603 and E0624.
Un-widening names that no other file accesses (for example `Cursor.pos`, `PresenceCommandCursor.reader`) gave no error, which shows the scan over-approximates slightly; this is the safe direction.

Limits that remain: expressions whose type derives from a stub type are error-typed, and rustc does not check privacy on them; method calls that need a trait in scope on stub traits are invisible. The conservative token scan and the explicit trait-import handling compensate; only the real cargo check closes this fully.

### 3.3 Warnings
`oracle.py --warn` compares unused-import, private-interface and unreachable-pub warnings (message-level, a message that only moved files is not new) across all configurations: number clean; async clean; grammar clean; channel clean (final logs `frs-run-*-finalw.log`).

### 3.4 rustfmt and parse
`frs-tools/parsecheck.py <crate> live`: `rustc -Zunpretty=normal` (loads every out-of-line module) parses with 0 errors for all four crates; `rustfmt --check` reports 0 parse errors. number is rustfmt-clean (0 diff hunks, same as baseline). async/grammar/channel were already not rustfmt-clean (39/17/63 diff hunks at baseline), now 54/23/68: the extra hunks are formatting of the added mount and import lines. Nothing was reformatted. Log `frs-parsecheck-live.log` and `frs-parsecheck-base.log`.

### 3.5 Path check
`frs-tools/pathcheck.py` (log `frs-pathcheck-final.log`): every `#[path]` and `include!` target of the four crates exists; each new file is reachable exactly once; longest repo-relative path 116 bytes (limit 240); no new file over 1,205 lines; every new file starts with a `//! <emoji>` docstring; the emoji is unique among the siblings of one parent; `//#region` and `//#endregion` balance in every new file (number's `natural` is balanced only after the added StringIo end marker).

### 3.6 Taxonomy
`frs-tools/taxcheck.ts` calls the repo's own `matchDirectoryKind` over the whole ancestor chain for all 42 module folders, i.e. the 38 new ones plus the 4 parents (log `frs-taxcheck-final.log`): 0 unresolved, 0 ambiguous. All resolve through the existing `crate-module-children` pattern row (`memberSlugPattern`), so no taxonomy edit was needed. The check found two names that resolved to structural kinds (`🔣️codec` to `json-fixture-case`, `👥️presence` to `presence`), which is why they were renamed.

## 4. Source-text readers updated
- `🧰️framework/🔨️modules/🗣️dsl/🧪️tests/🧱️ownership/🟦️.ts`: reads the grammar through `readModuleTreeSource`. Checked with a bun script: the `compile` signature and `unregistered grammar fragment` strings are found. The test itself still fails at its own line 94 (Cargo dependency list expectation, unrelated, fails before my lines).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts`: `channel` read through `policyReadModuleTreeSource`.
- `✏️s/🔌️plugins/📐️cad/.../👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts`: channel source read through `readModuleTreeSource` (the substring laws need `PresenceCommandCursor` and the codec text). That function fails at its own line 88 (ENOENT on another file) before and after, so it was not exercised.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🏛️ownership/📥️command-ingress/🧪️tests/🧫️fixtures/🔣️.json`: the AST-based corpus now has one `private-import` probe per file that imports from `::protocol::wire::command_ingress` (root, presence-roster, command-decode, codec) and a `retired-public` probe per child (4 cases to 14). `bun test` of that folder: 2 pass, 0 fail (it also passed before the change).
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json`: `test-command-ingress-consumer` inputs gained `.../🧵️channel/*/🦀️.rs`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts`: `⏳️async/🦀️.rs` added to `POLICY_SPLIT_SOURCE_ROOTS`. The interactivity p1x reader for async throws `conditional external module original_cancel_admission_tests is not canonical` before and after my change (the `🛑️cancel/🎟️admission/🧪️tests/🦀️.rs` mount is non-canonical), so the register is correct but unobserved.
- Checked and unchanged: `🧑‍💻dev/🧪️tests/🔖️channel-version` (CHANNEL_VERSION stays in the parent; 4 pass, 0 fail), `🛂️descriptor-verification` test (reads only `CHANNEL_VERSION`), `📦️crate-boundaries` fixture (entry file only). No reader of number or of the grammar file other than the ownership test exists.
- `.vscode/launch.json` and `🧩️launch.seed.jsonc`: no new executable command, no change.

## 5. Real check for the compile queue
Not run to completion. Two attempts under the mutex (`frs-real-number-1.log`, `frs-real-check-2.log`) were refused with `Insufficient physical compiler initial headroom`; the mutex was released each time.

Framework workspace (number, async, dsl); run once with the default features and once with `--features testkit,entrypoint` for async:
```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-number -p semio-framework-async -p semio-framework-dsl --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```
Root workspace (channel is a member of `Cargo.toml`, not of `🧰️framework/Cargo.toml`):
```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-channel --lib --tests --keep-going --message-format=short
```
Add `--target wasm32-wasip2` and `--target wasm32-unknown-unknown` variants for number and async (`--lib` only). Hand-off: on any E0603, E0616 or E0624 in these crates widen the named item to `pub(super)`; on E0599 for a trait method add the trait import to the child.

## 6. Foreign and pre-existing findings
- Fail before my change and are not mine: dsl ownership test line 94, cad-presence-retirement line 88, async interactivity reader non-canonical test mount.
- The compiler-ingress source-boundary gate (4000 active lines, 32 files, 256 KiB per crate closure) is unchanged by a split inside one crate; it needs the later crate carve.

## 7. Files
Created (tracked after install): `🔢️number/{💍️traits,🧱️natural,➕️integer,➗️rational,🕰️modular,🔑️primes,📏️interval,🧮️algebra}/🦀️.rs`; `⏳️async/{🌐️browser,🏷️schema-metadata,🪪️operation-context,⛔️cancel-token,🌳️scope,🌉️block-on,🛣️lane,⏰️timer-wheel,🪢️oneshot,🚦️sync-primitives,🔀️combinators,🗺️sharded-map,🧱️boxed-slots,🧵️worker-pool,🖥️native-pool,🕸️wasm-pool,🎭️testkit}/🦀️.rs`; `🗣️dsl/📖️grammar/{🧱️model,🔡️lexer,🌳️parser,🖨️writer,🧭️recognizer,🧮️walker}/🦀️.rs`; `🧵️channel/{🪪️wire-entries,🚚️archive-load,👥️presence-roster,🧩️command-decode,📨️app-command,🎞️app-frame,🗜️codec}/🦀️.rs`.
Rewritten parents: the four target files. Edited elsewhere: the six files of section 4.
Tools (kept): `frs-tools/` (`rslex.py`, `frslib.py`, `snapshot.py`, `splitter.py`, `stubgen.py`, `oracle.py`, `parsecheck.py`, `pathcheck.py`, `lossless.py`, `taxcheck.ts`, `install.py`, `plan_{number,async,grammar,channel}.py`). Baseline snapshots, sandboxes and logs: `🗑️generated/frs-*`.
