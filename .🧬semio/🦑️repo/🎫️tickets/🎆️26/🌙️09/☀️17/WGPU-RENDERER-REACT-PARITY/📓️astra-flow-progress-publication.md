# Flow Publication Progress

Flow20's one-byte semantic preparation test failed after progress moved from 22,013 bytes to zero during the `delete-widget` case, at step 22,213. The domain preparation checkpoint never resets. The shared `ArtifactStoreBatchPublication::progress` previously derived all folded work from its owned stage; the Publishing phase transferred that stage into the live store, so the subsequent observation returned a default checkpoint.

The repair retains the stage's fixed-width checkpoint when any of the three publication paths transfers its ownership: ordinary publication, coalesced amendment, and group-atomic staged handoff. Beginning closure freezes the current preparation checkpoint so incrementally destroying its owners cannot erase progress. The Flow law additionally checks that checkpoint throughout close. No domain state, action wire format, byte grant, or closure budget changes.

The fail-first receipt is `🗑️generated/astra-runtime/flow-native20-full/run.log`. The repair has not yet compiled or run; Flow21 is queued after the current renderer/UI verification lane.

Flow21 stopped before tests after Nx 2m55s: the test fixture's two `retire_cold` calls lacked the public `ColdRetire` trait import. The import is now explicit through the existing first-party neural facade. The checkpoint repair reached this compile boundary but no Flow behavior ran. Flow22 is required; receipt: `🗑️generated/astra-runtime/flow-native21-full/run.log`.
# Flow23 Compile Receipt

Flow23 executed no tests: the fixture import addressed the internal `neural_engine` module rather than the artifact's public `neural` facade. Three compile diagnostics (E0432 plus two missing-trait E0599s), Nx 2m37s. The source now imports `flow::neural::ColdRetire`, the same public facade that provides the fixture's OperatorInfo and Schema types. The progress and one-byte close runtime results remain pending.


## Flow 24 and UI 10

UI 10 passed all 664 tests in 4.249 seconds (Nx 30.3s), including the strict capture fixture with divergent candidate history.

Native 112 did run the three focus laws, but each stopped in a fixture precondition: the first accepted A document has no B editor, so a helper cannot map from a presented B before B is introduced. The helper now stages initial B directly from the candidate and reserves mapping for the later same-host ACB transition. Native 113 is running the corrected 46-test ownership selection, including root capture and diagnostic repairs and the stale Text-close and second-retirement admission repairs.

Flow 24 ran 253 tests: 246 passed, 7 failed, in 5.575s. The ColdRetire fixture repair eliminates the previous Once poison cascade. The retained publication progress law now passes its progress assertions and fails at terminal close, confirming the separately audited one-byte authority-close deadlock. The publication now transfers authority disposal into the existing bounded retirement owner; this production change is not yet rerun. An extended canonical authority law accounts for stamped edit ID bytes and still awaits its fail-first run before the three-string disposer repair.


## Stamped Authority Verification

The fail-first single-byte law released 19 bytes rather than the required 31. The canonical retirement owner now retains and incrementally releases actor id, group id, and stamped edit id. Its GREEN rerun passed 1/1 in 98ms (Nx 52.0s), with 1174 unrelated tests skipped. Flow's broader publication/close gate is still pending.

Three Flow fixture defects from the read-only audit were synchronized: the replica pair binds distinct instance ids and uses matching dispatch/settle metadata; the direct host edge test installs its own required extension registry; window publication accounting expects two coalesced config pages and one transient page. The content-command/child-store repair is delegated to Sol and remains in progress.


## Flow 25 Full Receipt

Flow 25 ran 253 tests: 244 passed and 9 failed in 11.304s (Nx 2m52s). The Store publication progress/one-byte-close law now passes, as does direct host edge selection after local registry installation. The child route repair compiles but still needs semantic fixes and exact route-catalogue expectation updates (seven owned by Sol). Distinct replica identities alone did not restore child convergence; parent-only backbone attachment needs a deeper composed-child audit. The window law now reaches its semantic settings assertion and fails there, so coalesced lane accounting alone was not the full issue. Root retained its desired settings values and added exact observed grid values to the existing failure diagnostic.
## Flow26 Receipt

The full Flow26 target ran 253 tests: 251 passed, two failed in 4.424s (Nx 2m48s). All seven content-child route and observer repairs now pass. Disjoint replica edits still do not converge through composed child transport. The window ownership fixture was concurrently updated to settle each command independently; all four config pages and the transient page publish, and camera/grid values pass. It now fails later because same-byte document reload leaves the generation transient alive. This does not establish that back-to-back config commands preserve both amendments; that separately observed issue remains recorded.

The composed replication audit is in `📓️terra-flow25-composed-replication-window-config-audit.md`. These framework gaps remain open, while the next renderer gate and WGPU19 activation take priority for actual parity evidence.
