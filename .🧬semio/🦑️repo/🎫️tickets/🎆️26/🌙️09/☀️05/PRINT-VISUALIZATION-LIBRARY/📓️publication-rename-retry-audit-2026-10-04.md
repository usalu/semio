# Publication Rename Retry Read-Only Audit — 2026-10-04

Inspected current generic artifact publication owner and neutral replacement test. No production/test edits or runtime execution.

No new concrete rollback/cancellation/path-scope failure demonstrated. Retry admits only EPERM/EACCES/EBUSY, measures5s deadline, delays asynchronously25→50→100→200ms and passes AbortSignal to delay. Cancellation is checked before rename, before delay and after rejected delay; stage checks after identical-byte admission and before entering replacement. Wait progress uses the existing repository-owned LeaseWait interface. Imports are system fs/path/timers and first-party resource leases; no external runtime dependency added.

If retiring the previous directory fails, original staging remains and temporary cleanup executes. If installing temporary fails or cancellation interrupts its wait after previous retirement, catch restores the sibling previous directory with a fresh non-cancelled signal. Recovery failure reports AggregateError containing both causes and leaves previous sibling outside temporary cleanup for recovery. Exclusive lease stays held through retry/recovery and finally releases even if temporary cleanup fails. Successful rename is synchronous; no asynchronously observable cancellation point exists inside it. Async caller cancellation resumes at guarded boundaries.

Paths remain the existing admitted staging sibling scope: caller output directory, unique stage sibling, previous sibling. Existing destination ancestor symlink checks and rejection of absolute/parent-traversal artifact names precede copy/publication. No new computed deletion outside those existing sibling paths introduced by the retry change.

Neutral tests patch only destination-install rename through test-only bun:test module mock, invoke production stageArtifacts, and restore node:fs mock in finally. They cover transient EPERM two failures with success, permanent EIO restoration, and EBUSY abort restoration; independent Python bytes compare published content. Persistent contention deadline coverage is pending addition at the inspected revision. Actual original API publication remains necessary to confirm Windows contention recovery; these source/test observations do not claim GREEN.

## Expanded Initial-Publication Evidence Binding

Coordinator reports registered83665 terminal0 8.3s with absent-destination EPERM2 failures recovered and seven independent Python inventories; production owner unchanged since34071 freeze. This audit source-binds current inputs only; no runtime rerun. Actual final API publication remains pending.

| Path | SHA256 |
|---|---|
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts` | 29953B7D7CB329020A0B8CDCE419F89DCE235AEDC956CECB5FC0F761B065DF16 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🧫️fixtures/🔣️.json` | A1770C888FCF20BF2F19F33A5D70FB6E9759B51D4DF2C5566B23B2F3F6811263 |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🧬️schema/🔣️.json` | 4603D8A7E6ADE082D3EEDB061C560E2F8AD08E9765CDCD5BEB45B69168CF359C |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🧪️tests/🟦️.ts` | 8AD7BA722863910DCB4D22A8DA3E2AF8658309D57176BF5603601E906EEB56BE |
