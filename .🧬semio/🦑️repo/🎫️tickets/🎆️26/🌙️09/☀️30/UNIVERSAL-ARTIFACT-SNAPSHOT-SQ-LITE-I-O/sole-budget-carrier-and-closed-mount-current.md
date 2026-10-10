# Sole Budget Carrier and Closed Mount Audit

Read-only fresh current sources; JSONC parse only, no build/runtime launch.

## Semantically clean current convention

Current Actor Budget647 contains sole original RetainedTurnInput. scaled664 preserves it unchanged; lane budget_for707 accepts original retained and forwards it. ScheduledActor has budget/issued/returned but no second retained field; TurnGrant carries Budget only. Drain4202 validates wallet, rejects issued/empty/zero work, checked-adds epoch after return before mailbox pop, updates original budget retained and issued witness once actual nonempty batch exists. Complete4886 validates receipt exactly for issued before metrics/state, then updates remaining grant preserving issued identity; issued clears and returned sets. Meta budget retained mirrors scheduler after completion; scheduler remains issuance authority. Its lagging epoch during in-flight turn is metadata, not an independent authorizer. Field location does not itself violate conservation.

Canonicalize all schema/fixture/source/native caller laws to this one convention if chosen. Current policy fixture/schema still says TurnGrant.retained/resourceOnly and Source policy insists budget_for(lane) with no retained, while public receiving Native already uses first.budget.retained. Receiving Source still contains mutually exclusive Budget assertions. These are exact contract inconsistencies, not reason to duplicate wallet or retain compatibility.

Packing caveat: Decision::pack_encode validates each budget retained; standalone TurnGrant::pack_encode directly writes actor/shard/Budget with no corresponding validation, and Budget pack manually writes identity/grant without validation. Decode uses RetainedTurnInput::pack_decode which validates. To preserve public direct packing preadmission law, validate original input before any writes at its actual public encode boundary.

## Current selected mounts

Existing jsonc-parser read-only parse returns zero errors for launch and seed; each contains exact eight selected Actor/Value/CSV/TSV rows. Every selected capability now has closed NativeOwner fields, exact artifactDirectory equals row CARGO_TARGET_DIR, child660000ms and explicit64MiB/65536 transport/offline. Actor/Value wrapper cwd framework, program bun with absolute Process script argument plus exact config/cwd/nested package command; Native adds manifest and actual selected lib name. CSV/TSV directly invoke absolute package script with test-snapshot-sqlite source or actual subset validation lib selector. Root and path placeholders remain original GUI substitutions; this audit did not execute wrapper equality or VSCode substitution. Mounted valid shape does not establish runtime completion.
