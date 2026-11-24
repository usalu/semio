# Work-limit refusals on the staged demonstrator (overlay 8775)

## 2) aggregator: `WorkLimit: original Window initialization retains another mounted address` -- fixed

`TypedWindowConfigStoreOwner::ensure_original_partition(window, cx)` (`🪟️window/🎚️config/🗄️store-owners/🦀️.rs` ~286) runs ONE resident partition initialization at a time (`self.initializing` and the staged `initialized_*`, `registry_address`, `pending_partition` fields). When a second window asked while another window's initialization was in flight, it answered a hard `WorkLimit` error that surfaced as `original-turn.semantic` and failed the boot. That is a wait condition, not a refusal: the request is not ready yet (`Ok(false)`), the in-flight initialization finishes and inserts its partition, and the later window starts its own on a following call.

Fix: the guard is removed; a call for another window now drives the in-flight initialization one step and answers `Ok(false)`. Driving it from either caller also removes the deadlock where the first window's caller never returns. The staged fields carry the address themselves, so the pipeline does not depend on which caller steps it.

Not run: `check -j 1 -p semio-plugin-window-config` is refused before the compiler by a peer's in-flight change (`semio-framework-value-native-owned` is 262180 of 262144 bytes). No test: a partition initialization fixture needs a full store owner.

## 1) koordinator/bearbeiten: `native-return-refused: authority (work-limit)` -- NOT root-caused

Evidence is the same family as `🔬️return-source-authority.md`: the return ledger refuses when `work + held_fuel > fuel` ("original return source CPU exceeds spent and held authority", kind WorkLimit).

Static reading of the hypothesis (a single action reserving the whole budget): `StepContext::with_original_effect_at` (`🧵️job/⏱️context/🦀️.rs` ~348) quotes `maximum = fuel_remaining` (the whole remaining turn fuel, ~50M here) for every plain original effect. A quote is only an admission ceiling (`observed + additional <= fuel`); the commit charges the performed work, so the quote alone does not enter the ledger. The ledger term grows through `reserved_return_work` / `record_work` of the actually performed work and the held return credits, so an oversized quote cannot by itself exceed `fuel`. I could not find a path that holds the quote, and I did not change the quote extent: narrowing it to a per-effect bound changes admission for every caller of `with_original_effect` and needs the failing state to choose the bound.

Next evidence (no print needed in `loan/🦀️.rs`): the already existing `[DEBUG] return ledger refusal` print is gone with the cleanup; re-add one non-capturing line in `OriginalReturnBudgetSnapshot::advance_retained` naming `work`, `held_fuel` and `fuel` for the first refusal, rebuild once, and read which term crosses. The secondary-drain note of `🔬️return-source-authority.md` still applies: any remaining stall or refused publication makes the ledger fail later with this same text.
