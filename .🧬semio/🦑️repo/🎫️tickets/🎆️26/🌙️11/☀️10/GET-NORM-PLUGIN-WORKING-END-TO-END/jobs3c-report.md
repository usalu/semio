# jobs3c report: job framework crate test targets

All under `🧰️framework/🔨️modules/🧵️job`. Tests re-expressed under the post-merge effect-threaded API; no tests deleted, no floors, budgets from framework quotes.

| Crate / target | Status |
|---|---|
| job-clock (lib) | compiles, 3/3 pass |
| job-fault-publication (lib) | compiles, 2/2 pass |
| job-payload: unit tests (17), dense-page-prefix, original-page-transfer, original-builder-close, original_payload_admission, doctest | compile and PASS |
| job-payload original-header-initialization | PASS. The heap check after `caller.retire(stock)` is now net-release (released - requested == receipted birth bytes): the fixture's retire step allocates+frees 2930 B transiently; the retirement observer cannot be nested around `retire` |
| `semio-framework-job` target original-page-return (not a payload target) | compiles, 2/5 pass. 3 FAIL: (1) contiguous-bytes birth: heap (16385,0) vs expected (0,0) in `🧪️testing/📬️payload` observe; (2) supplied-page return: progress `copied_items:1, copied_bytes:256` where test expects default; (3) registry returns: fixture close releases 21448 of 14584 born root bytes. Not root-caused (accounting expectations vs the new quote-charged transfers); belongs with the job-context/testing-caller owner |
| job-return-escrow-publication publisher-birth | compiles, 4/4 pass |
| job-return-escrow-publication lib | PASS 11/11 (header-failure close via with_original_invocation, work quote, quote-funded grant and per-step wallet depth) |
| job-scope (lib) | compiles. 1 test FAILS: first child close turn reports `copied_items:1` where the test expects empty Pending ("child begin-close transfers control without claiming an owner release"). Unresolved: either the fixture child closure (returns real progress) or the pump contract changed |
| job-native-encoding (original-source-native-encoding) | compiles; 4/11 pass, 7 FAIL, blocked in production |

## Changes of note
- Payload testing helper `📬️outcome/📦️builder/🧪️testing/🦀️.rs`: PayloadCase owns an EffectBank; `close`/`close_with`; work is u128.
- Law: `authorityBirthWork` 1 -> 2 (shared allocation admission charges allocate + write) in the builder testing `case.json`/schema.
- `RetainedPayloadBuilder::close_step` doctest expects E0308.
- Page transfer: transfer progress asserts `copied_bytes == quoted copy demand` (previously hard-coded 0); close heap observed through the retirement observer.
- Added production quotes: `OriginalSourceEncodingOwner::next_authority_copy_bytes`, `OriginalEscrowPublisher::close_header_failure_work_demand`.
- Scope test: Source stock derived from the Source birth quotes; fixture fuel derived from child slot count.
- Dev-deps added: effect-bank (scope, fault-publication), allocator (native-encoding).
- Publisher birth test read `law[2]` from the wrong fixture; now reads the builder birth rows. Header guard test uses `header.len()`.

## Needs a framework decision: native-encoding
`NativeEncodeControl::allocate_effect` calls the local `checkpoint()`; recorded-Source controls have no local observer ("local checkpoint requires its original observer"), and `with_native_encoding` exposes no Source witness, so tests cannot drive `checkpoint_source`/`begin_source_stage`/`advance_source`. `with_native_encoding` should hand the callback the Source witness and `allocate_effect` should route recorded controls through the Source checkpoint protocol. Also `source_encoder_construction_posts_to_the_original_caller` sees 2 copied items where 3 were expected (accounting change, undiagnosed).

## Notes for other agents
- testing-caller `retire` allocates transient bytes under the plain trace observer (affects heap-equality assertions in callers).
- Admission queue waits exceeded one hour per run.
- The framework Cargo.lock was behind its manifests after dev-dep additions; refreshed with `repo cargo -- update --workspace --offline`.
