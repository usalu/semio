# 1107 opendebt result

## Confirmed already fixed by peers (read in the current tree, not changed)
- Tick receive constant `work:1` (`tool-run/🎞️tick/🛬️receive/🦀️.rs`): `advance` enters via `control.receive_effect(demand,1,&mut frame,..)` with a resident `NativeReceivingEffect`; outcome work is `effect.performed_work()`. Not compiled by me.
- Presence JSON per-effect wallet (`store/👥️presence/📸️snapshot/🔤️json/🦀️.rs`): no `AuthoringWallet`; `effect_step`/`effect_poll` use `native.receive_effect` with a frame in the `Receiving<P>` slot tuple. Not compiled by me.
- io-control `retire_decode` Pending progress drop: now posted via `post(..)` before the typed pending refusal (reviewfix, 1107-result-reviewfix.md). Read only.
- tool-run presentation slot panics: fixed by reviewfix (typed invariants).

## Fixed here (verified)
- pack-json JSON writer pending-quote refusal (`pack/🔤️json/🎛️controlled-and-borrowed-bridge/🦀️.rs`): when `allocate_effect` returned `None` (quote reply pending) the writer returned `InvariantViolated "quote is still pending"`. Now `allocate_or_resume!` resumes a retained quote with `continue_allocation_effect`, and a pending quote yields the turn (`Ok(None)`); only a pending settlement after the producer ran stays an invariant error. The retained `allocation_progress` no longer trips the "unsettled" guard.
- Law: `pack/🔤️json/🧪️tests/🪢️pending-quote/🦀️.rs` (`[[test]] pending-quote-law` + dev-dep control-contract in pack-json Cargo.toml): a forwarded port replies pending to the first quote, then completes; the writer must yield at least once and produce the serde_json string. `opendebt-3` native rc 0, 1 passed. Before the fix the first turn would return the invariant error by code reading; I did NOT run the before state.

## Not fixed
- JPEG/source phases: image job charges 1 unit per action, which equals the decoders' own per-action `work` counters (jpeg `self.work+=1`, source `last_work+=1`); undercount remains for one action doing a whole chunk until the JPEG decoder is ported to effects. Port is larger than this lane.
- tool-run panics: about 230 `unwrap/expect` remain in non-test tool-run files (largest: progress presentation update 36, entities provenance 23, plugin publication-and-member-refresh 16, steps insert 13, tick operations source 12). They assert slot invariants of the plugin instance; not converted, no law written.
- Energy `Default` expect and `retire_decoded`: left for lane energie.

## Notes
- The pack-json companion test crate (`semio-framework-pack-json-tests`, lib test) does not compile (13 errors, stale retirement/native_decoding API in owned-bytes, retirement-laws, receiving, receipt, borrowed-source-laws lines ~324-333); not mine, hence the law lives in the pack-json package.
- Outputs: `🗑️generated/1105b/opendebt-{1,2,3}.txt`.
