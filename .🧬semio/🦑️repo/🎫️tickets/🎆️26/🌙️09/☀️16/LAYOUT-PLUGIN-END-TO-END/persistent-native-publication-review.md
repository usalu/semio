# Persistent Native Publication Review

Read-only owning-source diagnosis using the build skill and first-party contracts. No cache, cleanup, resize, attach/detach, receipt/schema/intent/policy changes, jobs or tests were performed. The live root native2163 and peers were untouched.

## Actual Refusal

Actual retained log is ticket/🗑️generated/typescript-declared-retry.log. Declared Layout TypeScript retry exited 1 before tests. Its persistent receiver diagnostic for .cache/b/d is:

- schema issues: []
- filesystemBytes: authored 8588886016, requested 38653657088
- maximumPhysicalBytes: authored 8589934592, requested 38654705664
- backing: device 16777229 matches authored; inode 422334443 matches authored; size 8588886016 matches authored; one link; allocated 8588886016.

Thus the diagnostic is a current envelope mismatch, not a malformed receipt or observed backing identity mismatch. Actual physical/kernel custody is still rechecked by the owner; this log alone grants no mutation authority.

inspectPersistentReceiver in process/native-build/storage/cleanup/command source line 139 requires authored.filesystemBytes === requested.filesystemBytes. Its maximumPhysicalBytes comparison permits an authored maximum below the requested maximum, so the filesystem field is the decisive shown failing equality. Retrying the ordinary declared test does not rewrite a valid older publication into the new envelope. Device rebinding or boot recovery does not repair this numeric mismatch.

## Supported Owning Publication Protocol

repositoryCacheImagePublicationV1 in repo/library/caching/storage/enforcement/budget/opening/publication source line 77 binds publication to genuine original argv exactly [tools, setup, all]. It derives dependency directory and both envelope limits from the canonical current repository budget; callers cannot supply a replacement path or cap. publishRepositoryCacheImageV1 line 81 delegates to the first-party Native publisher with a closed freshly observed request.

The request contract requires current receipt SHA256 and actual directory/backing/root identities, sizes, allocated bytes and mounted/inactive state. Those observations are preconditions, not standalone authority. The publisher owner line 81 requires exclusive native-fixed-image custody, captures the payload witness, detaches the actual image, proves inactivity, stages and CAS-publishes a fresh receipt, attaches and verifies, compares preserved application/journal/service identity and settles its owned cleanup. Its line 69 rejects mounted descendants and only requires observed filesystem bytes <= the new envelope. Line 65 publishes the configured envelope with the actual unchanged backing size/identity. Thus supported republication need not resize the 8GiB physical backing merely to publish its larger allowed envelope.

The command whose original authority this API recognizes is:

```sh
repo --service=off tools setup all
```

This is the authorized publication purpose, not a verified automatically working repair invocation. Scoped inspection of current repository library runtime callers found no invocation of publishRepositoryCacheImageV1 beyond its declaration/export. The inspected tools command dispatcher implements list/exec, while setup is handled through backend/bootstrap routing. I did not establish an end-to-end setup-all call path that constructs the actual publication request and dispatches this API. Therefore do not claim that running setup-all alone will repair the stale dependency receipt. If the declared setup owner does not already dispatch this API, its missing orchestration must be completed at that owner under the same genuine setup-all purpose; an ad-hoc script or fabricated request is not a supported substitute.

## Coordination And Limits

The publisher acquires an exclusive image lease with timeoutMs 0 and refuses mounted descendants. It must not be attempted while root's live original native2163 or peers retain relevant images/custody. Wait for those operations to complete naturally, then obtain fresh owner observations and use the supported setup owner path. Do not cancel holders or detach their images. This review has not taken current lease/kernel inventory and does not claim the partition is quiescent.

The fixed inactive reset schema profiles are graph, nx, rust and products; dependencies is not a supported reset profile. The inactive reset targets cannot repair .cache/b/d. Cache retirement, dependency sync, receipt hand-editing, schema/cap changes and manual image operations are not substitutes for current publication. The build skill explicitly says to retry through the declared owner, preserve receipts, and never detach, move intents, rewrite receipts or delete partitions manually.

No recovery action or native proof is claimed. Root must decide the necessary owner-level fix after preserving the active lane and checking the current setup orchestration.
