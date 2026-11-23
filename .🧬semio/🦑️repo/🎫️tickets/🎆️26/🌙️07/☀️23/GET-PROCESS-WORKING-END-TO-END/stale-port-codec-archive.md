# Stale Port: Store Codec and SPR Channel Archive

Prefix `🧰️framework/🛍️products/💻️os/🔨️modules/`. Build via `stale-cargo.sh` (labels `codec-*`, `archive-*`).

## Shared test helper

`🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🪙️close/🦀️.rs` is mounted `#[cfg(test)] original_close` by space-history, codec and archive roots (one file, no copies). Added for this slice: `CloseOriginal` (one `close_original` for encode and decode controls), `with_port` (NativeAllocationAuthority port, now ending in `into_port()` because the authority's Drop panics otherwise), `WorkStock`, `ConstructionStock`, `funded`, `funded_work`, `funded_invoked` and the decode birth-step branch in `close_decode` (`requires_original_birth` then `close_original_birth_step`).

## 🏪️store/📦️codec (16 errors, then more once names resolved)

| Test | Before | After |
| --- | --- | --- |
| `🪶️snapshot-capability` encode/decode `🚪️control` | `NativeEncodeWorkRecipient`/`NativeDecodeWorkRecipient`, issuers, `|output,owner|` receiving closures, `step(grant)` | `OriginalWorkAuthority`+`OriginalConstructionAuthority` loans, `construct(native)`/`construct(record,retained,native)` with output slot, `retire`, `close_original`; accepted path now drains retirement before asserting empty |
| `📸️native-snapshot/writing-tests.rs` | grant-taking `advance`/`take`/`close`/`step` | `funded(grant, effect)` wrappers; `last_turn_work` replaced by the recorded work of `funded_work` |
| `📸️native-snapshot/🫴️receiving/🧪️tests` | job caller turns, `receive_retained_work` | rewritten on `NativeDecodeControl` + `funded_invoked` effects; heap equality kept via `observe_backing` around the step |
| `📥️lifted/🪆️workspace-receiving` | `close_step(grant)` | `close_step(0, effect)` loop under `funded_invoked`; the work-equals-1 assertion dropped (empty reserved workspace performs no work) |
| `🫴️failure/🧪️tests.rs` (integration) | `step(grant)` | `AuthoringWallet::run_settled` inline |
| remaining files | `close_retirement_recipient(grant)` | `close_original(grant)` |

## 📡️spr/🧵️channel/📦️archive (27 errors)

Dev-deps added: `semio-framework-trace-memory-testing`, `semio-framework-value-testing-allocator` (the latter must be linked, otherwise `observe_backing` reads 0). `🛬️decode/🧪️tests`: forwarded allocation closures became `with_port` event ports (Admit asserts the ledger, Post updates it), work and construction loans through `scoped`, the job-caller denial test rewritten with `WorkStock(0)`.

## Findings

- Forwarded decode retirement close: `"forwarded retirement allocation requires actual receipt admission"` (known gap of the decode `close_retirement_recipient` charge under a forwarded binding). The codec decode `🚪️control` test therefore uses a local ledger; the forwarded-ledger law is not covered.
- Forwarded encode: `charge` outside a quoted effect is refused (`"forwarded native storage requires its quoted allocation effect"`), so the encode control test also uses a local ledger.

## Results

- `📦️codec`: `cargo check --tests` clean; lib 7/7 and integration 2/2 pass (`codec-t13`). The decode `🚪️control` test closes under fresh loans (`closed`) because the old close consumed no native work and the new one does (row `frameOnly` has a work stock of 1).
- `📦️archive`: `cargo check --tests` clean; `--lib` 2 of 5 pass (`archive-t7`). Open: `original_composed_carrier_reservations_...`, `archive_partial_field_refusals_...` and `archive_interior_utf8_cancel_...` close pending decode owners under a forwarded allocation port, which the decode retirement `charge` refuses (`🌱️value/🛬️decode/♻️retirement/🦀️.rs`, forwarded binding, same gap as the earlier ledger note). Product fix needs actual receipt admission for forwarded close births; not attempted.
- No product code changed. Cargo.toml: archive gained two dev-deps (trace-memory-testing, value-testing-allocator). Not seen: the parent's 392 B marker mismatch did not occur in these tests.
