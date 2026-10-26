# Prepared Taxonomy Finalizer Review

PASS by read-only source review of `finish-taxonomy.ts`; it was not executed. No actionable bug was identified for its stated parent-controlled invocation sequence.

The baseline index map is read before waiting for closed metadata, so later generated-output removal cannot erase the comparison authority. Polling is bounded by a four-minute deadline. Git index-lock retries wait without removing the lock. Ticket staging enumerates direct root files and uses explicit argument arrays/batches, excluding generated/important subdirectories. It does not restage product/shared files wholesale.

All canonical source hashes are recomputed with Git blob framing and optional LF normalization, and every old mapped coordinate must be absent. Outside-scope entries are compared against the frozen baseline and reported; the finalizer makes no write to them. The final Markdown itself is explicitly staged after being written, and whitespace validation runs again. No commit, checkout, stash, worktree or broad Git add operation occurs.

Required external preconditions remain the parents responsibility: invoke from repository root, load the script before cleanup, finish all render/probe/report writers before closure, and run final cleanup only after this command exits. The finalizer records those process-completion facts; it does not itself enumerate or terminate render processes. The separate independent product index audit already checked current stage-zero source entries and exact shared projections. New ticket evidence can continue changing before the parent signals this prepared finalizer to run.

This review launched no subprocess and has no owned process.
