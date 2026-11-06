# SQLite Receiving Port Seam

The current `Port::work(depth, copy)` preposts a copied-item receipt before the caller mutates the retained slot. It also increments the SQL work counter without reserving work on the original native recipient. It cannot serve as evidence of a paid mutation.

A retained domain writer needs a quote/perform/post operation per concrete move, with a live native work loan and the same original physical recipient. Quote checks SQL limits, native work and the actual original physical grant. The effect writes into a pre-existing retained frame slot. Commit records the actual work and exact physical move even when a later native, SQL or recipient observer refuses; independent refusals remain distinct owned causes. Cancellation or a refused quote enters no effect. Vector, string and box births require their actual requested allocator receipts and retained partial outputs rather than a returned temporary.

Decode may use the construction recipient's submitted counter and accept-only port. Encode needs the corresponding original physical construction recipient; the current flat Source span grant and retirement slot do not fund general relational frame/row allocations. SQL-only allocations must be accounted separately from native admitted bytes so settlement does not subtract unrelated SQL charges.

FEM's authored traversal can remain a borrowed domain-neutral RowSink. Its retained writer must bind this port directly, not wrap the existing cold RowWriter result or infer work from copied items. The generic ArtifactSqliteSnapshotHost defaults still delegate to Result-only cold hooks and should be replaced by actual owner implementations or a real shared retained traversal; a fabricated zero-work default would misstate effects.
