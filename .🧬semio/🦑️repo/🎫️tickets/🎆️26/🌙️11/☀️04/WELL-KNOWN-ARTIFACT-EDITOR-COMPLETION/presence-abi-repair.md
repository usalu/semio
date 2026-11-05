# Presence Fixture ABI Repair

Updated the two-turn presence/root-retirement leaf against the current controlled APIs while preserving native describe session 12698. Preparation receives its caller-owned NativeDecodeControl, reads the typed scalar revision directly, refuses insufficient copy/capacity grants before construction, charges the exact Arc backing allocation and reports the real copied bytes/capacity. Blocked or refused construction retains its original request; successful preparation consumes it only after all fallible admission and Arc construction, matching the original input-owner lifetime. Repeated prepared queries report no new work, and a transferred root cannot be manufactured again.

The retirement factory returns the real admit_shared_retirement RetainedWorkOutcome without extracting or replacing its work. Removed an unused custom Arc-drop retirement cursor; all factory use already goes through the owned shared retirement implementation. Mounted/app close helpers now accept and forward the original StepContext; parent owns updating callers in adjacent leaves.

The existing language-neutral full-operation fixture remains the behavioral specification. No new heavy build/test was started, and the in-flight native source snapshot was not restarted. Compilation of this test leaf remains pending the parent’s broader ABI repair pass.

## Cancellation Boundary Leaf

The cancellation-publication-boundaries leaf now admits genuine app cancellation issuers and regular leases through the existing original context. Standalone linearization rows reuse the audit-owned KeyedCancellationFixture (one additional regular begin method), retire every lease through its real public cursor, and close the issuer. No cancellation handle Clone or Default remains.

Presence preparation forwards the same source’s native receiving control and reconciles physical construction against returned semantic receipts. Pending publication close uses native retirement authority and consumes actual returned progress, including on refusal. Mounted publication uses the shared financed publish_mounted_fixture_unit; consumer pages are presented and physically closed before exact ACK, and producer pages retire through the public advance_acknowledged_result_page path via the shared helper. The existing language-neutral cancellation fixture remains authoritative. No heavy compilation/check was started; broader test compilation is pending parent orchestration.
