# UI Handback and Runtime Transfer — Independent Review

## Executed Evidence Reviewed

The coordinator read the actual current logs, not only executor summaries:

| Gate | Actual result |
| --- | --- |
| UI contract full R10 |115passed,0skipped,1.579seconds; successful canonical Nx target |
| Surface backing transfer R5 |0passed,1failed,85skipped; moved-from backing remained64bytes instead of0 |
| Transfer with rejected payload R8 |1passed,86skipped,.080seconds; exact pointer/value/capacity returned and moved-from backing remains0 |
| Active reconciler finalization R7 |1passed,86skipped,.051seconds; exact record/index allocations transferred, no replacement backing, explicit close completed |
| Runtime regression R10 |86passed,1skipped,2.345seconds; the known inline-byte double-charge test is explicitly excluded |

The runtime result is not a full87-test pass. Process's physical ownership fault and current inline double-charge RED remain open.

## Source Review

The nested shared-alias test now proves the serializer's separately retained reader: aliases before and after its queued one-item/zero-byte release are asserted. The original descendant release remains two items/ten bytes. The test did not merely change a total to match production.

SurfaceFixedVec::take_all replaces the source with an empty boxed slice, transferring the original allocation without default-allocating an entire replacement. try_push refuses a moved-from owner and returns the exact rejected String with unchanged pointer/capacity. Active finalization uses the same transfer for retained records and key indexes, with explicit cursor/reconciler/patch retirement in the test.

UiPatchOps still reserved7,286,960bytes for a first payload at the subsequent true RED boundary. Dag owns specialized paged storage and the runtime executor owns resident/work metering. Directory/page allocation must be admitted before reservation; actual6320-byte payload placement cannot be accounted as4096bytes. Existing32768-byte reconcile work and4096-byte typed-retirement grants remain distinct. No8MiB or8ms ceiling increase is authorized.

## Exact Logs

- `🧪️member-ui-full-green-r10-native-2026-08-27.txt`
- `🧪️surface-ownership-transfer-red-r5-native-2026-08-27.txt`
- `🧪️surface-ownership-rejected-payload-r8-native-2026-08-27.txt`
- `🧪️surface-ownership-finalize-green-r7-native-2026-08-27.txt`
- `🧪️surface-runtime-regression-r10-native-2026-08-27.txt`

All existed and were read at this checkpoint. No deletion or recovery was performed.

