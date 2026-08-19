# 📓️ terra wide-rollup report

Independent measurement-only pass over 12 siblings' claims. No source files edited (packet
scope: measurement only). All commands run FOREGROUND, real exit codes captured directly
(never through a `tail`/`echo` pipe). `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-rollup`
for every cargo command. Raw logs: `terra-rollup-*.txt` in this ticket folder is where the
`.md` lives; the raw `.txt` command outputs are in the session scratchpad
(`/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/terra-rollup-*.txt`)
per the "cargo target dirs live in scratchpad" rule — the census scripts are also there
(`dyn_census.py`, `async_census.py`, `banned_symbol_census2.py`, `attribute_errors.py`,
`scan_vitest_doublecount.py`).

---

## 1. Rust compile state

| target | command | exit | notes |
|---|---|---:|---|
| `semio-framework-os-kernel` | `--lib` | **0** | ✅ confirmed, NOT regressed. 417 warnings. |
| `semio-framework-os-kernel` | `--all-targets` | **101** | only `lib test` fails, **exactly 2746 errors** — bins and bin-tests compile clean. Matches `rs:alltargets-kernel`'s claim exactly. |
| **`semio-framework-plugin`** | `--lib` (**headline**) | **101** | ❌ **NOT green.** See §1a below — this is the single most important finding in this report. |
| `semio-framework-ui` | `--lib --features wgpu` | **0** | clean, 0 errors |
| `semio-framework-schema` | `--lib` | **0** | clean |
| `semio-framework-replication` | `--lib` / `cargo test` | **0** / **0** | 184 passed — matches `rs:alltargets-green` |
| `semio-framework-pack` | `--lib` / `cargo test` | **0** / **0** | 44 passed — matches |
| `semio-framework-geometry` | `--lib` / `cargo test` | **0** / **0** | 57 passed, 0 warnings — matches |
| `semio-framework-math` | `--lib` / `cargo test` | **0** / **0** | 191 passed — matches |
| `semio-framework-async` | `--lib` / `cargo test` | **0** / **0** | 17 passed, 0 warnings (confirms the `#![allow(async_fn_in_trait)]` fix landed: 6→0) |
| `semio-framework-dispatch-macros` | `--lib` / `cargo test` | **0** / **0** | 28 passed across lib + 4 integration binaries (22+3+1+1+1+0) — matches |

All 6 sibling-verified crates (`rs:alltargets-green`) reproduced exactly. No regressions there.

### 1a. THE HEADLINE IS NOT GREEN — `semio-framework-plugin --lib` is EXIT 101

The packet brief states: *"`semio-framework-plugin --lib` (the headline — EXIT 0 means the
guest SDK is green and the whole fleet unblocks)"*. Measured: **EXIT 101, 161 errors.**

The failure is **not in the plugin crate's own code** — the build aborts compiling its
dependency, the crate literally named **`semio-framework`** (root `🧰️framework/📦️packages/🦀️rust/Cargo.toml`,
`package.metadata.semio.role = "framework"`). Primary-error attribution (span-keyed, one
location per diagnostic, not multi-span noise):

| file | errors |
|---|---:|
| `🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` | **135** |
| `🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/🦀️component.rs` | **26** |
| **total** | **161** |

Dominant error shapes: 54× "method should be `async` or return a future, but it is
synchronous"; 26× `no method named 'at' found for opaque type impl Future<Output =
MutationApplyError>` (missing `.await` before `.at(...)`, an error-builder chain); plus
`E0308`/`E0277`/`E0382` mismatches. This reads as an in-flight or stalled asyncify pass over
`workflow/component.rs` and `manifest/component.rs` — nobody in the 12 sibling summaries
claims ownership of either file. **Coordinator: this blocks the "whole fleet unblocks"
milestone and needs an owner.** Full error log: `terra-rollup-plugin-lib.txt`.

(Aside: `semio-framework-ui --lib --features wgpu` is genuinely EXIT 0 — the `icon_name.rs`
mentions inside the plugin-lib error log are secondary/`help:` spans on a `catalog_action_icon_id`
call site inside `workflow/component.rs`, not a UI-crate defect.)

---

## 2. TypeScript roll-up

All runs: real consumer command (`bun ./📜️script.ts test --reporter=verbose`), unique counts
confirmed against `🧪️vitest.config.ts` for the include/includeSource double-count trap.

| package | exit | tests | vs. stated baseline |
|---|---:|---|---|
| `@semio-tech/framework` | 0 | **87 passed** | = 87 ✅ |
| `@semio-tech/framework-actor` | 0 | **46 passed** (3 files) | baseline row said "40→46"; confirmed 46, config clean (glob-safe check: independently grepped `🖼️wire-turn.ts`/`🧵️shard-runtime.ts` for `import.meta.vitest` — 0 hits each, so the 3-filename `includeSource` array is NOT silently excluding anything) ✅ |
| `@semio-tech/framework-kernel` | 0 | **33 passed** | ⚠️ moved from stated "29" — but this is EXPLAINED: `ts:exchange-removal` added 4 tests for the new `TurnOutcomeBroadcast` primitive (29+4=33), confirmed by their own report. Not a silent drift. |
| `@semio-tech/framework-os` | 1 | **206 passed / 1 failed** | = 206/1 ✅, same named failure (`matches the Rust plan_workflow … decoded via wasm`), routed out-of-band per rule 391 |
| `@semio-tech/framework-renderer-react` | 1 | **321 passed / 15 failed** (336) | baseline row says 325/336 (11 failed); CURRENT state is 15 failed. This is ALSO explained: `ts:exchange-removal`'s foreign-file impact added 4 new failures (all `adaptPluginHandle …` names, exactly matching their filed lease-request) on top of the 11-name pre-existing baseline subset. 11+4=15, 321 passed — matches `ts:exchange-removal`'s own math exactly. **Both lease-requests from that packet are still outstanding** — see §2a. |
| `@semio-tech/framework-os-dev` | 0 | **27 passed** | ⚠️ moved from stated "17" — matches `ts:dev-and-modules`'s own measurement (27), but **no one has explained the 17→27 delta**; flagged, not resolved. |
| `os-hub-ts` | 0 | 1 skipped (gated `HUB_E2E=1`) | matches `ts:hub` |
| `@semio-tech/hub-admin` | 0 | **5 passed** | matches `ts:hub` |
| `@semio-tech/flow-js` | 0 | 0 tests | matches `ts:fleet` |
| `@semio-tech/trinity-jack-lsp-worker` | 0 | 0 tests (2 files) | matches `ts:fleet` |
| `@semio-tech/puzzle-js` | 0 | **15 passed** (9 files) | matches `ts:fleet` |
| `@semio-tech/animate-js` | 1 | 0 tests, collection error (dangling `animate-present-core` alias) | matches `ts:fleet` |
| `@semio-tech/cad-js` | 1 | 0 tests (`Export named 'join' not found` in shared library package) | matches `ts:fleet` — **still broken now**, shared-library churn unresolved |
| cad ext. `aec-building-structure`/`aec-building`/`spatial-shape`/`aec-building-energy` | 1 each | 0 tests each (collection error) | matches `ts:fleet`'s "0/5" exactly |
| `@semio-tech/s-2d-js` | 0 | **4 passed** | matches `ts:dev-and-modules` |
| `@semio-tech/s-3d-js` | 0 | **1 passed** | matches `ts:dev-and-modules` |
| `@semio-tech/framework-replication` (ts) | 0 | **1 passed** | matches `ts:dev-and-modules` (the `includedScripts` recursion fix holds) |
| `@semio-tech/framework-os-mcp` | 0 | **20 passed** (4 files) | matches `ts:dev-and-modules` |
| `@semio-tech/framework-os-shell` | 0 | **3 passed** | not previously reported by name this session; clean config |

**No suite is currently double-counting.** Repo-wide scan (`scan_vitest_doublecount.py`) for
`include`/`includeSource` naming the same file(s), excluding the 4 packages already fixed
per rule 18, found **3 remaining instances**:

1. `compose/dev/algorithm/js/vitest.config.ts` — **out of scope per O3**, ignore.
2. `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/🧪️vitest.config.ts` —
   downstream app, not this ticket's surface.
3. **`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️vitest.config.ts`** — **in scope**
   (`🧰️framework/**`), `include`/`includeSource` both list `🟦️vite-elements-assets.ts` +
   `📜️script.ts`. Ran it: **16 reported, real unique is doubled** (not yet independently
   halved by file — flagging for whoever owns `🖱️ui/🎨️styling` to apply the same `include: []`
   fix rule 18 already applied elsewhere). No sibling this session touched this package.

**Stale doc-comment (cosmetic, not a real bug):** `🎠️kernel/📦️packages/🟦️typescript/🧪️vitest.config.ts`'s
own comment block claims `framework-actor`/`framework-os`/`framework-os-mcp`/`framework-os-shell`
"all... make vitest collect each in-source file through BOTH... paths, doubling every test's
run count" and "Not fixed there". Independently re-checked all 4 named configs: **all 4 are
currently correctly configured** (`include: []` or a disjoint explicit list vs.
`includeSource`), no doubling reproduced in any of them. The doc comment describes a past
state that has since been fixed elsewhere and was never updated. Not functionally harmful
(nobody is trusting the comment for a build decision) but worth a one-line fix by kernel's owner.

### 2a. Outstanding lease-requests from `ts:exchange-removal` — confirmed still open

- `📺️renderer/…/⚛️react/🧪️index.test.ts` — the 4 new `adaptPluginHandle …` failures are
  confirmed present by name (see table above), fix not yet applied.
- `📺️renderer/…/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts` — independently re-grepped: still
  constructs `const exchangeHandle: Pick<KernelPluginWasmHandle, "exchange"> = { exchange: async (...) => {...} }`
  at lines 246–247. `KernelPluginWasmHandle` no longer has an `exchange` member post-rewrite,
  so this is a live type error, not yet converted to the `enqueue`/`outcomes` pattern.

---

## 3. Banned-symbol census (source vs. generated, separately)

Method: python3, word-boundary regex, excludes `.git`/`node_modules`/`target*`/`.venv`.
"Generated" = paths under `🤖️generated/`, `/dist/`, `/pkg/`, `_vendor/`,
`🔌️extension-modules/`, `🔌️plugin-modules/`, or filenames matching wasm-bindgen/jco output
patterns (`semio_s_plugin_*.js`, `*_component.js`, `*_bg.js`, `🟨️plugin-worker.js`) — a flat
grep without this split reported **3112 generated hits for `exchange` alone**, which would
have kept a naive gate red for the wrong reason (per the packet's own warning).

| symbol | source hits (real, non-comment where checked) | generated hits | verdict |
|---|---:|---:|---|
| `exchange` | see below — mixed real code + false positives (English word) | 3112 (676 files, stale jco/wasm-bindgen glue) | ⚠️ **see finding below** |
| `PluginWorkerClient` | **0** live code (2 comment-only mentions in `kernel/component.ts` describing its removal) | 18 (5 files — one stale `.stage/🟨️boot.js` cache copy) | ✅ clean — both real source copies (`kernel/component.ts`, `wgpu/typescript/boot.ts`) confirmed zero |
| `WasmPluginRuntime` | **0** live code (all 38 "source" hits are doc-comments describing "the deleted `WasmPluginRuntime`") | 5 (1 file) | ✅ clean |
| `ExtensionRuntime` | 2 (doc-comment) | 7 | ✅ clean |
| `ProgramSupervisorState` | 1 (doc-comment) | 0 | ✅ clean |
| `PLUGIN_FUEL_BUDGET` | 3 (2 files, doc-comment only — 1 in `plugin/🖥️host/component.rs`, 1 in `actor/component.rs`) | 1 | ✅ clean |
| `INSTANCE_GUARD` | 1 (doc-comment) | 5 | ✅ clean |
| `runSerialized` | 2 (`kernel/component.ts` + `plugin-web-materialize.ts`, need eyeballing — not verified line-by-line) | 1001 (197 files — almost certainly false-positive-prone generic name, not independently confirmed banned-shaped) | not fully resolved — low confidence either way, needs a follow-up look at the 2 real-source hits |
| `loadPluginModuleUncached` | 2 (`kernel/component.ts`) | 10 | not independently line-checked — flagged for follow-up |
| `poll_ready` | 10 (5 files, all Rust) | 0 | not independently line-checked — flagged for follow-up |
| `set_host_backbone_channel` | 6 (5 files) | 6 | not independently line-checked — flagged for follow-up |

### ⚠️ Finding: `exchange` (the RPC method) survives on the Rust HOST side — not removed

`ts:exchange-removal`'s own report is accurate and narrow: it cleaned exactly its 3 owned TS
files (`🎠️kernel/🟦️component.ts`, `💻️os/🟦️component.ts`, `PluginRuntime/🟦️component.tsx`),
independently re-verified here at **zero** `exchange` occurrences in all 3.

But the ticket's binding rule says **`exchange` (WIT + all callers)** must not exist at exit —
and on the **Rust** side it plainly still does, as live (non-comment) code, not doc-comment
residue:

| file | non-comment `exchange` lines | shape |
|---|---:|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` | **18** | closure parameter `mut exchange: impl FnMut(&str, u32, AppCommand) -> Result<Vec<AppFrame>, TransactionError>` threaded through `run_transaction`/`undo_group`/`redo_group`, plus a `fn exchange(&self, ...)` test impl |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` | **12** | `async fn exchange(&mut self, ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError>` — a genuine **trait method** literally named `exchange`, plus `self.host.exchange(&ctx, handle, commands).await?` call sites |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs` | **7** | `async fn exchange(client: &KernelClient, instance_id: u32, commands: Vec<AppCommand>) -> Result<ExchangeOutcome, String>` wrapping `client.exchange_commands(...)` |

These are the same "send `AppCommand`s, get `AppFrame`s back" round-trip semantics the WIT
`exchange` function had — just re-implemented natively as a trait method / closure parameter
rather than removed. **No sibling report claims ownership of this cleanup on the Rust side.**
This is the single largest gap in the banned-symbol program as measured today.

(The many other `exchange` "source" hits from the raw census — `heat_recovery`/`air_exchange`
in the energy plugin, `change-exchange-process`/`inverse` mutation names in the ISO16757
schema, `envelope`/prose mentions in doc comments — are the **English word**, not the banned
identifier, and are correctly not part of this finding.)

---

## 4. Repo-wide first-party `dyn` census

Method: python3, `dyn <path::>*Trait` pattern per non-comment line, `.rs` files only,
**excludes `.🧬semio/` (ticket scratch/backup files — confirmed by inspection these are dead
files in CLOSED tickets, not live source) and `compose/` (out of scope per O3)**. HRTB
`dyn for<'a> Fn(...)` correctly classified as `dyn Fn` (std-legal), not a false first-party hit.

| | count |
|---|---:|
| **First-party `dyn` (R1-banned)** | **90** (framework 85, fleet 5) |
| **std/lang `dyn` (R1-legal baseline)** | **135** (`Fn` 51, `Future` 27, `FnMut` 23, `Any` 14, `FnOnce` 12, `Error` 8) |

⚠️ **This is sharply lower than the packet's stated "previous: 173 (framework 148, fleet 25)"
and "std baseline previous: 138"** — first-party down 173→90 (48% further drop), std roughly
flat (138→135). Two possible explanations, not adjudicated here: (a) genuine further de-dyn
progress by today's concurrent siblings as a side effect of their async/trait work (several —
`rs:fleet-modules-sweep`, `rs:alltargets-kernel`, `schema:world-collapse-prep` — touched
exactly the kind of files this would move), or (b) the "173" baseline was measured with a
methodology that included `.🧬semio/` scratch files (my raw, unfiltered first pass, before
excluding ticket folders, found **205** first-party hits — much closer to 173, with the
biggest scratch-inflated traits being `dyn Backbone` 30→1, `dyn StudioMember` 20→0). **(b)
looks more likely** given how cleanly the scratch-inflated numbers explain most of the gap,
but this needs the coordinator to compare against whoever measured "173" and how.

Top first-party traits still `dyn`-dispatched (post-filter, real production code):
`dyn Emit` (15, all in `🛢️db/**`), `dyn HostAsyncRuntime` (10, `🔌️plugin/🖥️host/⚡️effects/component.rs`
— arguably R3-sanctioned erasure-adjacent, needs an owner's judgment, not mine), `dyn HttpBody`
(7, `🛎️services/component.rs`), `dyn HttpTransport` (5), `dyn RouterEffectHandler` (5),
`dyn Operator`/`dyn Iterator` (4 each), `dyn OsBackbonePort` (4, `✏️s/🔌️plugins/🪐️space/component.rs`
— fleet's only cluster). Full site list: `terra-rollup-dyn-census.txt`.

---

## 5. Async census

Method: python3, `.rs` files, same exclusions as §4, line-anchored `async fn` / `fn` matcher
(fn-pointer-type signatures like `fn(...)` excluded by requiring an identifier after `fn`).

| | count |
|---|---:|
| `async fn` | **67,749** |
| plain `fn` | **9,685** |
| **total** | **77,434** |
| **async %** | **87.49%** (previous: 86.8%) |
| `// 🚫️async:` tags | **423** (previous: 337) — E1 157, E3 60, E4 112, E5 15, unclassed (R9 "no suspension point" style, no `E<n>` code in the tag text) 79 |

Both numbers moved up, consistent with a day of active asyncify work; no red flags. (First
census attempt undercounted tags at 2 due to a comment-line-skip ordering bug in my own
script — caught and fixed before reporting; corrected number is 423.)

---

## 6. Discrepancy summary (§6 = the actual point of this packet)

1. **HEADLINE FALSE: `semio-framework-plugin --lib` is EXIT 101, not EXIT 0.** Blocked by a
   dependency crate (`semio-framework`, the top-level framework kernel) with 161 errors
   concentrated in `workflow/component.rs` (135) and `manifest/component.rs` (26). No sibling
   report claims either file. **This is the top priority for the coordinator.**
2. **Banned symbol `exchange` survives on the Rust host side** (`plugin/🖥️host/component.rs`,
   `run/component.rs`, `ProgramBridge/component.rs` — 37 non-comment lines total) even though
   the TS side (`ts:exchange-removal`'s 3 owned files) is genuinely clean. The binding rule
   ("`exchange` (WIT + all callers)") is not yet satisfied repo-wide.
3. **`framework-kernel` ts test count moved 29→33** — explained (exchange-removal's +4), not a
   silent regression.
4. **`framework-renderer-react` moved from the stated 325/336 (11 failed) baseline to
   321/336 (15 failed)** — explained (exchange-removal's 4 new foreign-file failures, both its
   filed lease-requests confirmed still outstanding by direct inspection).
5. **`framework-os-dev` ts test count moved 17→27 with no explanation on record.** Matches
   `ts:dev-and-modules`'s own measurement exactly, but nobody has said why the ticket's
   long-standing "17" baseline grew to 27; not investigated further here (out of this
   packet's scope to fix, in scope to flag).
6. **A 3rd in-scope vitest double-count instance found**, not caught by any of the 12
   siblings: `🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️vitest.config.ts` (16 reported, doubled).
7. **`dyn` census (90 first-party) is far below the stated "previous: 173."** Most likely
   explained by the "173" baseline having been measured including `.🧬semio/` ticket-scratch
   `.rs` files (my own unfiltered first pass landed at 205, much closer) — flagged for the
   coordinator to reconcile methodology, not treated as either a regression or confirmed progress.
8. **`os-kernel --lib` warning count (417) is NOT the "9" `rs:fleet-modules-sweep` observed** —
   that packet explicitly described 417→9 as a live snapshot of another session mid-edit, not
   a claimed final state, so this is not a contradiction, just confirming which number is current.
9. Four banned symbols (`runSerialized`, `loadPluginModuleUncached`, `poll_ready`,
   `set_host_backbone_channel`) have real source hits that were **not individually
   line-verified** in this pass (time-budgeted away after the `exchange`/`PluginWorkerClient`/
   `WasmPluginRuntime` deep-dives) — flagged as open, not asserted clean or dirty.

No other sibling claim in the 12 summaries failed to reproduce.
