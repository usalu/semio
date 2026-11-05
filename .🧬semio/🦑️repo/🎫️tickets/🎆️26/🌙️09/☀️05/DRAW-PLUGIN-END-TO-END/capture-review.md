# Immutable Capture Repair Review

Read-only source review of the ancestry fixture, schema, tests and storage helper on 2026-11-04. No build or regression was duplicated.

The real-descriptor test uses an independent recursive SQLite path projection and Ajv validation. Its wrapper requires each unique directory admission exactly once in each capture/replay phase, compares complete receipt directory/file membership, includes multiple roots, sibling files, a multi-chunk file and an empty file, and checks custody returns to baseline. This is a meaningful complexity regression rather than a budget increase. The unchanged 4194304 limit is explicitly asserted.

The retained-callback helper uses named identity, directory kind, same-device validation and issued withDirectoryPath callbacks. It does not retain capabilities outside callbacks. Maximum depth bounds the ancestor stack. The original file capture and replay still check full bytes, EOF, metadata and final named identity, and final replay remains mandatory.

At this snapshot the input-parent grouping helper is not yet present although the new tests extract it. Existing capture/replay still independently reacquire prefixes and sibling parents, so the strict native regression must remain red until both phases are grouped. Source review of the completed grouping remains required. Stage still uses independent sibling acquisition and needs separate capacity evidence if the actual failing workload reaches fresh staging.

Actionable test follow-ups: the existing helper docstring says proofs refresh between mutations; recursive create now retains all ancestors while creating descendants. Confirm genuine Windows create-deep and create-tail descriptors accept this ownership lifetime rather than relying only on the in-memory model. Native refusal coverage exercises capture-time hash/replacement/cancellation and fresh replay roster drift; add or retain an explicit non-fresh replay hash/replacement drift assertion after a successful capture so later fence replay remains independently protected. The aggregate progress callback count bound is intentionally cross-platform loose; the strict unique-admission oracle is the useful complexity contract.

No completed-runtime claim is made. The normal admitted bootstrap regression must establish platform custody and unchanged-work-budget behavior.
