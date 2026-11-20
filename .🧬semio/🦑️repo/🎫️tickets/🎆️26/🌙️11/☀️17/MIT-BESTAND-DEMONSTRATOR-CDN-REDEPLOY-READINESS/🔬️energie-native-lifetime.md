# 🔬️ Energie Pane: `plugin-ui.native-lifetime-required`

## Reproduction
`energie-console.mjs <url> <ms>` (playwright, chromium from the repo tool cache) loads `http://127.0.0.1:8766/#energie`, logs console errors, network order and, via route interception of the built host bundle only (nothing in source or `.cache` changed), the open turn state.

## Findings
- The failing plugin is `energy` itself (the first and only hub opened in the pane): the network order ends at `🔋️energy/semio_hub_energy_component.core.wasm`, then the boot error.
- `module-loader/🟦️.ts:129`: the single `open` crossing returns `status: more-work`, `lifecycleReceipt` absent; `lease.progress()` stays `opening`, `lifetime` null. The guest never ran `install_owner` (reactor `🚪️lifetime/🦀️.rs`: the captured receipt exists only after the instance owner is installed).
- Not a host bug and not a missing export: the open crossing is correctly one guest drive. Polling the lease up to 160+ more times (probe patch) never captures (`more-work`, ~190 ms each), so a host-side poll loop would not help; the guest livelocks.
- The staged `energy` wasm (built 2026-11-18 11:08, receipt sourceTreeDigest `1901b30e...`) contains the temporary guest probe `[DEBUG] settle table_false: state=(Absent, Absent, Absent, true, false) birth_requested=true` printed at open. That string no longer exists in source: `🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs` `admit_original_return_table` was rewritten after that build (file mtime 2026-11-18 23:11, plus further uncommitted edits). The original-return escrow table cannot be born for the open turn (`birth_requested=true`, table/origin/publisher absent), so the owner is never installed.
- Same framework-guest family as the procedural-hub invariant (original return admission): other panes (`generator`, `koordinator`, `aussuchen`) fail at boot with a structured error payload instead.

## Fix
No host source change is justified. The root cause lives in the guest original-return admission code, owned by the other agent. After that fix lands, the guest wasm must be rebuilt.

Rebuild (browser facet): `energy` first (verifies this pane), then every module built before the 2026-11-18 23:11 framework edit that shares the guest runtime: `fem`, `procedural`, `demonstrator`, `cad`, `playbook`, `puzzle`, `sourcing`, and the extension modules. `cad-extension-aec-building-energy` (built 11-19 07:35) is newer than the edit and only needs a rebuild if the fix touches it. No `components describe` or site build was run here.

## Re-diagnosis (energy rebuilt 2026-11-19 15:00, shard worker 15:16, overlay :8767)
Method: `energie-console.mjs` (`POLL=<n>` additionally patches the served host bundle in-browser to keep polling the open lease; per-turn `original-turn-receipt` messages are read off the worker).
- The `table_false` probe is gone from this pane; the open now runs ~1000 guest turns per crossing (steps ceiling of the worker drive), status `more-work`, no `lifecycleReceipt`, lease `opening`.
- Genesis/original-return birth now progress: the first ~21 600 epochs (about 10 800 turns) show varying copied items and retained capacity (real work: store genesis, app construction). Polling the lease 30 further crossings (about 31 700 epochs) never yields the receipt.
- From epoch 21 629 to the end (5 000+ turns) every turn is identical (9 items, 4096 bytes, 19 work units, nothing retained, no fuel/grant exhaustion: fuel 49.4M, items 7.9M, copy 402M left). This is the baseline turn overhead with the opening step paying nothing: the open state machine (`🚪️opening/🦀️.rs` `plugin_advance_actor_opening`, one `return Ok(false)` per step) reached a step that makes zero progress forever, or finished and the receipt is never published. Candidates in order of likelihood: the tail of the original app construction (`advance_original_app` after the last genesis phase / `EnergyNativeInitialSource::advance` take branch when `cursor.ready()` and its quote does not fit), `advance_instance_birth` (`admit_cancellation_issuer` -> `OriginalCallerChildSource::admit_step`, which returns `Ok(false)` without progress while `should_yield`/metadata does not fit), and the final loan return.
- The open is not a host bug: a host poll loop does not help (guest never advances), and exports/bridge are fine. The between-action re-binding is not implicated by the evidence (no error is thrown; the guest quietly stalls).

## Not done
No source change: the stalled step could not be pinned from outside the guest, and the native check (`repo cargo --store=shared --configuration=prototype -- test -j 2 -p semio-s-artifact-energy-model-initialization`) never got past `source.served` within 580 s (machine saturated), so the genesis test `original_energy_initial_source_transfers_the_real_persistent_model` (128-turn bound with a 4 KiB grant) is unverified.
Next step: add a `[DEBUG]` step label (step name per `plugin_advance_actor_opening` branch) to the guest, rebuild `energy` only, rerun `POLL=2 node energie-console.mjs http://127.0.0.1:8767/#energie 40000`; or run that native test to prove or exclude the genesis take branch.
