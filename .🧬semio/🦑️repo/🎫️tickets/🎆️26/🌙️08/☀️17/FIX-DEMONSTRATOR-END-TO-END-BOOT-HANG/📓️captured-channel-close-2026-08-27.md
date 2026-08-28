# Captured Channel Close

## Executed Repair

The binary-channel adapter previously disposed and removed the instance's AppChannelClient before awaiting native destroy. Two language-neutral fixture cases reproduced premature disposal: **2 failed/624 skipped**, 6.49s (`🧪️guest-channel-close-red-1.log`).

The adapter now captures the original channel, awaits the underlying close, disposes only that captured subscription after success, and removes the map entry only if it still refers to the captured channel. Failure leaves the channel available for retry. A replacement channel with the same numeric instance is not removed by the old close's later settlement.

The document-wire plus captured lifecycle scheduler cohort passed **7/7**, 619 skipped, 8.74s, start 19:40:01 (`🧪️guest-channel-close-green-1.log`). Strict Ajv validates the neutral fixture; Immer independently reproduces the expected close trace. These are actual adapter/controlled transport tests, not fresh guest retirement or AppChannel payload-retirement timing proof.

## Strict Boundary

Fresh strict rerun2 now reports exactly the seven known tutorial diagnostics (`🧪️guest-receipt-channel-strict-2.log`). UI's nullable fixture helper and taxonomy's two callback types are cleared by their owners. This is not a full strict pass.

The strict renderer run reached ten diagnostics: seven known tutorial joins, one nullable lifetime in the UI-owned native fixture helper, and two implicit callback types in the taxonomy-owned discovery provider. No adapter close, receipt codec, scheduler or output-envelope typing diagnostic appeared. The helper's new empty-array trace explicitly uses number[]; the earlier push-to-never issue is cleared in this output. Full log: `🧪️guest-receipt-channel-strict-1.log`.

The UI and taxonomy owners received their exact diagnostics. No overlapping source repair, suppression, compatibility default or source restore was made here.

## Live Cutover Inspection

The current loadPluginModule path still stores actor names in actorIdByInstance, allocates unchecked numeric IDs, sends an ordinary uncaptured Open, and applies content into actor-name retained UI maps. Ordinary commands, refresh and extension completion all need original-turn intake before result combination. The rich adapter's channel fix does not by itself replace these paths.

The live replacement must allocate a checked host instance ID, capture its dedicated lifecycle owner before Open, bind the single actual OwnedUiInstance only to the guest-issued Captured lifetime, preserve open-fault output, and drive each original patch through OwnedUiPatchIntake. Native ACK-returned results are new original outputs. Final disposal requires native descendants, actual UI read roots, raw returned-output roots, and the captured channel's required pending work, not a map-delete or empty participant.

UI owns response projection and Shell/Interpreter read-facade consumers. Demonstrator owns create/destroy, original-instance scheduler and raw output; native WIT and retained producer handoff remain with Dag. No live or all-app success is claimed until that coordinated cutover is executed and rebuilt.
