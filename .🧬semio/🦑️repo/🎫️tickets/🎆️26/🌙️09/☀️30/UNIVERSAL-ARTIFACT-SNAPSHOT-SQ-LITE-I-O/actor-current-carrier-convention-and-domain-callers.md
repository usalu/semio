# Current Actor Carrier Convention and Domain Callers

Read-only fresh audit during concurrent edits. No compiler or runtime claims.

## Single authority convention

Latest Actor Rust has resource-only Budget, ScheduledActor.retained, TurnGrant.retained, and exact issued/returned scheduler witness. Drain validates current retained input and checked next epoch before mailbox pop. Complete validates exact issued receipt before replacing entry.retained with original identity plus returned remaining grant. Keep this single convention across metadata, packing, fixtures and callers; resource scaling must neither copy nor renew retained grant. Initial earlier read observed Budget.retained restored, while subsequent read observed separation again, so files are actively changing; this report does not infer stable compile status.

Concrete receiving Source line14 currently contains both true and false assertions for the identical Budget retained predicate. It also expects absent ScheduledActor/TurnGrant retained and old entry.budget.retained updates. These contradict latest production and policy Source, which correctly requires resourceOnly + TurnGrant.retained. Root owns that Source repair. Native retained-turn laws use first.retained and entry.retained and align with current convention. StepBudget.retained in test-only JobTurnBridge refers to the separate Job StepBudget; it is not Actor Budget and must not be mechanically removed.

## Exact CSV/TSV callsite correction

Correction to earlier report: physical `export_sqlite_database(database,limits,callback)` and `import_sqlite_database(bytes,limits,callback)` are still actual three-argument public APIs at SQLite main320/325. Their CSV53/54 and TSV73/74 uses are not missing owner arguments merely by arity. Actual stale domain provider calls are CSV snapshot test74 and TSV94: provider.export lacks original IoRunControl and provider.import transfers database by value instead of original mutable Option plus control. Direct decode/encode calls also omit current original native control; CSV corpus186–190/TSV207 onwards contain fake retire_sqlite_snapshot cleanup. Repair these exact boundaries from actual declarations rather than replacing physical SQLite wrappers indiscriminately.

## Genuine test control custody

Handcraft declared finite bodyGrant, closeGrant, deniedCloseGrant, nativeMaximum and close turn bound; keep them independent of payload length/descriptive demand. Name callback, allocator, original NativeOwned recipient and SQL/native controls outside borrowed operation. Preserve original database Option before admission. Drop loan before inspecting same recipient, then use actual RetireOwned controlled cleanup for retained snapshot/database/payload/IoError and pending owner. No compatibility helper API, invented maximum/default grant or unchecked producer drop. Existing physical unowned wrappers remain an implementation ownership frontier until the actual API itself changes; test arity cannot confer receiving ownership.
