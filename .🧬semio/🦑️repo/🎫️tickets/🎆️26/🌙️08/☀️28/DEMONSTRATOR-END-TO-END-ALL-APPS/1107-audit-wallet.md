# 🏦️ 1107 Wallet Audit: AuthoringWallet / AuthoredEffect / Finite stocks

Lane `aud-wallet`. Read-only. No repo runs, no sub-agents, no edits outside C.
Scope: `🧰️framework`, `✏️s`, `🌎️hub` (`git -c core.fsmonitor=false grep -n`).
Raw hit list: `C/1107-grep-wallet.txt` (21 lines, snapshot at audit time).

Note: the first grep showed 23 hits. Two `🚰️source-cursor/🦀️.rs` hits (lines 60 and 200, `AuthoringWallet::new(grant,1).run(..)`) were gone on re-grep. A peer edited that file during the audit. Verdicts below use the current tree.

## Verdicts

| # | file:line | what | verdict | owner lane | fix |
|---|-----------|------|---------|------------|-----|
| 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📸️snapshot/🔤️json/🦀️.rs:9-13,35,41,47-48,53,65,67` | `effect_turn` / `effect_poll` / inline `AuthoringWallet::new(grant,TURN_UNITS).run(..)` inside `owner.receive(..)` and `native_snapshot_turn(..)` closures | **BYPASS** (inside a turn). Fresh 1024-unit wallet per effect, not the job's remaining fuel. Work is taken into the turn after the fact (`work.take`), so it is charged late, and admission is decided against the wrong budget. | runtime (provider), presence owner (os store, not in lane table) | Replace with job-side `StepContext::with_original_effect` / `close_original_retained_effect_step` (runtime addendum). Single debit = `receive_original_retained_work`. No `AuthoringWallet` in turn code. |
| 2 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/🌉️bridge/🦀️.rs:68-97` (`receive_stepped`) | Construction work runs under `OriginalWorkAuthority::new(StepWork{quantum:1<<30})` (line 97). Only physical progress is posted (`wallet.post`, lines 84/91). `BRIDGE_WORK` (2^36) is never debited for construction work. | **BYPASS** (work not debited). Synchronous entry, so not a turn, but the work stock is not honest. Per step up to 2^30 units, up to 2^24 steps. | energie | Debit construction work into the wallet (new `wallet.charge_work(u128)` or run construction through `wallet.run`). Set `StepWork` quantum from `wallet.remaining_work()`. |
| 3 | energy bridge `🌉️bridge/🦀️.rs:71` | `NativeDecodeControl::new(1<<40, ..)` | **BYPASS** (admit-all constant). Runtime addendum forbids `1<<40`-style admit-all. | energie | Derive the limit from the wallet stock (`bridge_stock()` bytes/items). |
| 4 | energy carrier `📜️carrier/🦀️.rs:42` | `impl Default for EnergyModelSnapshot { .. .expect("the canonical empty model is admissible") }` | **Panics on shortfall** (rule: refuse, never panic). Default routes through `receive_stepped` and its wallet. | energie | Build the empty owner without the wallet (infallible constructor) or drop the `Default` impl. |
| 5 | energy carrier `📜️carrier/🦀️.rs:47` | `fn retire_decoded(self){let _=retire_stepped(self);}` | **Swallows refusal** (typed error discarded). | energie | Surface the typed error through the trait, or document and log it as `[DEBUG]` only until fixed. |
| 6 | energy bridge `🌉️bridge/🦀️.rs:14,23,27` (`retire_stepped`) | `bridge_wallet()`, `wallet.run_settled(wallet.remaining(), BRIDGE_STEP_WORK, ..)`, `wallet.post(..)` | **LEGIT**. Physical progress posted, per-step work capped (2^16), steps capped (2^24), exhaustion typed. | energie | none |
| 7 | `✏️s/.../🔋️energy/.../🌉️bridge/🦀️.rs:14` stock | `bridge_stock()` = 2^20 items, 2^30 bytes (copy, capacity, release), depth 128; `BRIDGE_WORK` 2^36 | Bounded, generous (1 GiB physical per synchronous call). Acceptable as a declared document budget. | energie | Name it as the document budget constant. |
| 8 | Pack JSON `🧰️framework/🔨️modules/🎒️pack/🔤️json/🧮️parser-and-grammar-cursor/🦀️.rs:7-9` | `parse()`: `NativeDecodeControl::new(usize::MAX,..)`, grant = `isize::MAX` for copy/capacity/release, wallet stock = that grant, work = `(len+16)*64` | **BYPASS** (admit-all physical stock; the same grant is reused each loop and never debited, since `run` does not decrement the stock and there is no `post`). Work is sized sensibly; physical is not. `parse` is reached from many os callers (`🌊️flow/🌉️bridge`, `🌿️vcs`, `🎒️mesh`, `🌉️mcp`). | port-f (`pack-json-native-input`); aggregator (`pack-json-canonical`) | Size the stock from `input.len()` (items, bytes, capacity, release), `post` each step's progress, bound `NativeDecodeControl` by the same stock. |
| 9 | `🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🦀️.rs:70,91,93,97` | `decode_encoded_blocking` → `authoring_wallet(&limits)` (stock from declared limits), `retire_blocking` | **UNCLEAR**. Stock is sized from declared limits (bounded, sensible). Synchronous, but its callers are render/world/canvas paths: `🖼️canvas/🪪️icon-codec` (`:286`), `♾️infinite/.../face-overlay-and-fit` (`:151`), `.../brush-lod-and-style` (`:336`), `🎞️Scenes/.../packet-image-and-blend` (`:68`), `🗣️Interpreter/.../ui-image-loading` (`:93`). If any of those run inside a reactor turn, the decode is uncharged. Also `retire_blocking` line 97 calls `std::mem::forget(owner)` on stall (leak). | proc (pixels, canvas-scene) | Confirm each caller is a synchronous entry. Otherwise route through the job provider. Replace `forget` with a typed refusal that keeps the owner. |
| 10 | `🧰️framework/🔨️modules/🔲️pixels/🧩️compositing/🗂️layers/🦀️.rs:221-233` | `RasterStackRetirement::into_result(self, wallet: &mut AuthoringWallet)` pays each retirement step through `run_settled` + `post`, typed refusals | **LEGIT** (caller-owned wallet, physical posted, typed failures). | proc (pixels) | none |
| 11 | `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧩️compositing/🦀️.rs:101,104` | `advance` uses a new `RetainedWorkLedger::new(STOCK)` per call, and on completion `into_result(&mut AuthoringWallet::new(STOCK,WALLET_WORK))`; `self.job.take().unwrap()` on line 104 | **UNCLEAR → leaning legit**. Stock bounded (2^22 items, 2^28 copy, 2^25 capacity, 2^26 release, WALLET_WORK 1e9). Synchronous, reached only from raster-host. Physical stock resets on every `advance` call, so it is not cumulative. `unwrap()` breaks the no-unwrap rule. | port-c (ui-raster-owner) | Keep one ledger per composite, not per call. Replace `unwrap()` with a typed error. |
| 12 | `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🖥️raster-host/🦀️.rs:71,319` | `advance_composite(65536)` called from `build_current_scene` as `let _ = ...` | **UNCLEAR**. Synchronous scene build. The error is kept in `composite.error`, but the build ignores the refusal. | port-c | Propagate the error to the caller, or return it through the scene result. |
| 13 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🕸️mesh/🧱️mesh-owner/🦀️.rs:33-41` (`authored_step`) | Fresh wallet per step, stock = caller's grant, work 16. `run_settled` result `(turn,_)` discards `turn.work`. `Ok(None)` maps silently to zero progress. | **UNCLEAR** (minor bypass). Work up to 16 per step is never reported. A non-admitted quote looks like "no progress" rather than a refusal, so a caller loop can stall silently. Callers: `close_step` at `mesh-owner/🦀️.rs:229,238`. | port-e (3d-mesh-contract) | Return `turn.work` to the owner's work accounting. Make `Ok(None)` a typed refusal. |
| 14 | `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🏛️validation/🦀️.rs:38` | `SchemaValidationStorage::parse`: per-page `AuthoringWallet::new(grant,1).run(..)` for `units.reserve_one`, physical via `control.admit_allocation_bytes(quote)` | **LEGIT-with-note**. Synchronous import (`import_sqlite_database_controlled`, explicit `SqliteSnapshotControl`). Physical is admitted by control. The 1 work unit per page is not charged anywhere. | port-f (io-sqlite-snapshot) | Optional: return `done.performed` into the control's work accounting. |
| 15 | `🧰️framework/🔨️modules/◻️2d/🧱️regions/🦀️.rs:98-115` (+ consts :12-14) | `AuthoringWallet::new(STOCK, WALLET_WORK)` for retiring a finished `job`; `run_settled` + `post`, `work==0` refused | **LEGIT**. Stock 2^22 items, 2^28 copy, 2^25 cap, 2^26 release, depth 256, WALLET_WORK 1e9, CLOSE_WORK 2^20. Typed refusals. | port-e (2d-path) / unassigned for regions, coordinator to confirm | none |
| 16 | Core `🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️authoring/🦀️.rs:33-53` (`AuthoringWallet::run`) | Core API. (a) `run` never decrements `self.stock`; only `post` does, so callers that loop on `run` must post, and pack `parse` and presence do not. (b) Work is debited once at line 52, but early `?` returns at lines 41-47 (`claim_performed`, `settle_partial`, `commit`, `finish`) skip it after work may have run. (c) `grant.restricted_by(stock)` and `maximum_work > work` refuse typed (good). | runtime | Debit stock and work from the receipt before any early return, or on error paths. Make `run` debit the stock itself, or make the `post` requirement a compile-time shape. |

Also: `AuthoringWallet` is re-exported (`🌱️value/🎟️control/🧬️contract/🦀️.rs:11`, `🌱️value/📦️packages/🦀️rust/🦀️.rs:60`). That is fine, but the re-export is an API surface. Check it against the "no export needs a type outside the codebase" rule.

## Callers, turn vs synchronous

- Inside a turn: #1 (presence, `native_snapshot_turn` / `owner.receive`). This is the dishonest-admission case.
- Synchronous per addendum: #2-#7 (energy: `from_document`, `from_model`, `Default`, `FromValue`, `retire`), #8 (pack `parse`, widely used, verify each os caller), #10-#11, #14, #15.
- Unclear, need caller trace: #9 (decode callers in render/world), #12 (`build_current_scene`, which turn drives it?), #13 (`close_step` callers in mesh owner).
- `Finite` / `EffectBank` / `finite_work` in non-test code: no such identifiers exist in `🧰️framework`, `✏️s`, or `🌎️hub`. The `Finite` hits are `NonFinite*` float checks (`🕸️graph/⏯️layout-run/🕸️input-graph`, `🖱️ui/🖌️render/...scene-finish...`) and `sameFiniteBank` / `sameFiniteStock` JSON flags in value's work fixtures (`🌱️value/📋️list/🔗️persistent/📥️build/🎟️work/📐️.json`, `🌱️value/🛬️decode/🧳️owned/🧺️set/🎟️work/📐️.json`). None is a stock. There is no `EffectBank` and no `finite_work` anywhere.
- `🌎️hub`: no hits for `AuthoringWallet`/`AuthoredEffect`.

## Stock sizes

- Sensible and bounded: #7 (energy, generous), #9 (decode, from declared limits), #10/#11 (paint/pixels STOCK), #15 (regions), #13 (mesh, grant-bound), #14 (sqlite, per page).
- Not sensible: #8 (`isize::MAX` / `usize::MAX` admit-all), #3 (`1<<40` control limit).
- Presence (#1): stock = the body's remaining grant (bounded by job), work 1024 per call (bounded, but the wrong budget).

## Shortfall: refuse or panic?

- Refuses typed: `run` (`OwnershipLimit`, `WorkLimit`), `post`, `run_settled` (pending poll refused), regions, layers, sqlite, presence `unfunded()`, pack `parse` (`WorkLimit`).
- Panics: #4 (energy `Default` `expect`). Non-test `unwrap()`: #11 line 104. The `source-cursor` file had `expect("admitted retained Pack page")` in non-test code, but that file changed during the audit and its wallet hits are gone.
- Silent: #12 (`let _`), #13 (`Ok(None)` → zero progress), #5 (`let _` in `retire_decoded`).

## Priority

1. #1 presence in turn (bypass, runtime + presence owner). Blocks "uncharged work" claim.
2. #2, #3, #4, #5 energy (work not debited, admit-all, Default panic).
3. #8 pack `parse` admit-all stock (most callers).
4. #16 core debit gap on error paths (runtime).
5. #9, #12, #13 caller traces (proc, port-c, port-e).
