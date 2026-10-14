# Semio Text Neutral Receiving Review

Read-only actual fixture/schema/Source, 2026-10-13. No builds/edits; producer not present at inspection.

Current neutral cases cover emptyDocument1 row, emptyRun2 rows, complete9 rows (document1+runs3+marks5). Complete snapshot covers schema/language/content, all four mark tags and exact href including nonlink values. Parsed current fixture literal/schema contain true NUL (Python JSON read confirmed), not backslash-u text. Unicode/CRLF/long content and empty last run/marks are preserved. No missing actual model field found.

Independent SQLite renumbers run and mark IDs by0/-1/-1000 and adjusts mark.run_id, leaving document id1 and run.document_id1. This is correct for legal signed child surrogates and current one-document producer contract. Foreign keys disabled during edits then explicit foreign_key_check verifies final relations. Hex BLOB queries preserve NUL UTF8 rather than SQL length truncation. Output comparison rebuilds exact complete runs/marks and verifies authored row census.

Actual TypeScript identity currently rejects rowid<=0n despite DDL permitting signed run/mark ids. This is the genuine intended Source RED, not an invalid fixture. Remove only unnecessary positivity while retaining id-cell=rowid/arity, unique identities, exact parent references and contiguous ordinals. Document id=1 remains actual owning construction policy and malformed documentId case tests it.

Fourteen edits are distinct meaningful cases: document/parent identities, run/mark gaps, unknown kind, Blob instead of language/href Text invalid; changed schema/language/content/href/kind and deletion of one run's mark group semantically different. Each edit is actually applied to third-party SQLite before reconstruction; ignore_check_constraints allows intentionally invalid representation. No false same-value edit found.

Schema is closed fixture shape and includes inline originalGrant fields. Source currently admits this fixture schema only; add independent canonical Grant wire admission if claiming original resource authority schema qualification, rather than making this fixture copy its own authority. This is a proof-strength recommendation, not evidence current numeric grant is invalid. Current Native test file absent is intentionally checked by Source hook law before implementation.

Planned retained frame (Projection/candidate Snapshot/run-id+order/mark-id+order/status) fits all three tables. Share original paid schema checker and signed indexes; copy schema/Run strings/Mark href directly into retained candidate fields, price run/mark/header moves before pushes, map enum without allocating tags. Validate all groups/counts even empty, no local BTreeMap or reconstructed comparison snapshot. Runtime/allocator qualification remains pending authored producer and owning laws.
