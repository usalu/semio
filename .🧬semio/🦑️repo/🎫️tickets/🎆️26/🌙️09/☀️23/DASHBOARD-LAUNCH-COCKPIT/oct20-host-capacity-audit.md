# Host Capacity Audit

A read-only store snapshot at 2026-10-20T17:04Z found two admitted grants from the current boot (14:00:09Z), with recent heartbeats. This establishes current live holders, not forward compiler progress; no stale-reboot cause is proved for the present wait. Queued matrix and reset requests remain protected. No peer signal, lease/store mutation, status-prune call, or capacity weakening was performed.

The read-only owner review identified a separate inherited-grant fast path that checks schema and PID liveness but omits the boot and heartbeat freshness checks used by ordinary grant enumeration. The potential prior-boot reused-PID adoption requires a bounded schema-first refusal fix and actual tests; it is not claimed to explain the current two fresh grants.

## Bounded Source Repair

One closed `GrantLiveness` observation now determines both ordinary roster accounting and inherited fast-path admission. Eleven neutral vectors cover current state, reboot/PID reuse, stale heartbeat, dead PID, inclusive boot/heartbeat boundaries, invalid records, mismatched IDs, path identifiers and extra authority fields. Existing boot tolerance, staleness policy, memory floors, reservations, queue ordering and current grant lifetimes are unchanged. An invalid inherited record proceeds into normal admission; the fast path does not unlink or release it. The inherited environment ID is validated before preparing directories or reading files.

The new law compares the pure predicate with Ajv and an independent SQLite conjunction, then evaluates the actual `withHostCapacityV1` body through explicit unit observation ports under the genuine suite Original. Valid current inheritance avoids queue/sample effects; invalid inheritance reaches ordinary admission; malformed IDs are refused before filesystem effects. It is mounted in existing `test-runtime-environment`. Syntax/import checks passed; no Source runtime pass yet.

Independent review accepted the bounded production predicate and eleven-row unit boundary law with no concrete source blocker. It explicitly confirms that the full-gate AST test reaches the inheritance versus queue boundary only; it does not establish actual queue admission. Existing current foreign grants remain untouched. The actual accounting owner is TypeScript host-capacity gate, so the earlier rough label “native Go capacity” must not be used to infer whether RootTS was already loaded.

Independent existing Ajv data audit validates the updated schema and all11 neutral case envelopes. The intentionally malformed grant, invalid path identifier and extra authority observation are rejected by the observation schema. This is a JSON data audit only; it does not execute the owned Source regression law or queue.

The exact Root cancellation log confirms capacity waiting occurs through `createCapacityControlV1` before a ScriptInvocation exists; its invocation budget starts only after admission. That still does not prove imported Source modules were absent. Root waited for actual terminal exit1 before releasing the reset source boundary. No real reset effects or test bodies were observed.

## Fresh Shared-Source Reconciliation

Fresh physical schema/decision/gate had lost the previously reviewed GrantLiveness definitions and predicate while adjacent authored tests remained. Restored the same closed observation and shared ordinary/inherited predicate against current source, preserving peer code, boot tolerance, inclusive heartbeat age, all reservation/floor/queue policies and actual live grants. Inherited identifier is checked before preparing directories or reading grant paths. Added the refusal to existing English/German catalogue and supplied that actual error port to the bounded gate law. No status/prune/store operation ran, and this does not explain the present live-capacity wait. Actual Source law remains pending.
