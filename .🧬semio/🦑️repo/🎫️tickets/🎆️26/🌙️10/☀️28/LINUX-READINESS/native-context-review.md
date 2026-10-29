# Native Context and Snapshot Review

No query/build/test/process intervention performed. Nine production BuildMonorepoTreeCached callers across canonical CLI search/render and duplicated client query/serve/render now receive tree,error and return errors or explicit ToolResultError. Cache delegates to shared snapshot backend, preserving context beforebuild/write/publication, kernel generation custody and staged rollback. Streamproducer errors are collected and drained before returning; shared files/folders now WalkDir, cancelcheck and prune generated roots including explicitscope. Prior cloneancestor issue is corrected by repository-coordinate normalization before component scan.

Concrete correctness issue sent owner: tree computeCompositeFingerprintContext still hashes HEAD and porcelainstatus text, not modified authoredfile bytes. Two edits while status remains M preserve fingerprint and can serve stale warm tree/index. Existing TestComputeCompositeFingerprintIgnoresUntrackedFiles explicitly asserts untrackedignored even though fulltree StreamFiles enumerates untracked authoredsource. New snapshottests566–613 exercise caller-suppliedfingerprints, lifecycle/cancel/checksum, not actualsame-status freshness. This needs a semantic contract correction and realGit byteedit oracle, not a missing-test-only claim.

Concrete cancellation identity issue sent owner: snapshot readSnapshot secondJSONDecode EOF check maps any nonEOF error to ErrCorrupt trailingdata. Cancellation between firstvalue andEOF returnscontextcanceled/deadline at that point but is replaced before finalctx check. Preserve actualcontext error identity.

Responsiveness/resource notes: fulltree assembly checksctx at138 then after large loops471, so cancellation during assembly waits for allnodes; snapshot JSONEncoder materializes encoding before boundedwriter seesbytes, so writerlimits cap emitted size but not pre-encoding allocation. These are narrower boundaries than an unbounded diskread and should not be described as proven boundedmemory cancellation throughout. No measuredhotspot or runtimefailure claimed from these static observations.

Resourcechannel correction is in native-source-pruning-review.md: actual declaration is metadata.semio.closure.resources, not unsupported runtimeSourceResourceEntries; owner verified against real schema/readers. Full readiness remains pending ownerfreeze and actual nativequery.

## Frozen Candidate Follow-up Review

Prior source-file freshness defect repaired: shared FingerprintSource lexically streams authored relative identities and bytes, with perentry/read cancellation and generated exclusion; canonical/client bind digest, and owner reports actual realGit unchanged-M/untracked tests green. Trailing snapshot payload cancellation now joins originalcontext error, preserving errors.Is. Snapshot ancestor creation validates ordinary ancestors before child mutation; symlink regression was red then green. Production cachedtree callers still propagate typederrors. No query/build/test rerun by reviewer.

Concrete remaining persistentdomain freshness issue: canonical compositefingerprint calls FingerprintSource with workspace.IsRepoExcludedPath, which excludes all .🧬semio. hashComposeMetaState still hashes top directorymtime and ticketdirectorynames only. Existing already-M goal/ticket title/status/body byteedits leave porcelainstatus, topmtime and ticketlist unchanged but change records BuildMonorepoTreeFromRepo projects. Therefore warm domain search can stay stale. Reported to owner as a persistent-record projection identity problem; preserve intentional ephemeral/cache exclusions rather than hashing every log.

Small malformedmetadata gap: metadataDecoder disallows unknownfields but lacks trailingEOF validation, so validmetadata followed by anotherJSONobject is accepted. Existing indexmetadata reader already rejects trailingdata. Sent owner for parity.

CrossOS physicalguard note: standardmacOS /var and/tmp aliases can be ancestors of t.TempDir/os.TempDir. Canonicalize an authenticated owned temporary/systemroot before appending children (existing repository pattern realpathSync(tmpdir())); do not silently follow arbitrary cache-descendant symlinks or weaken ordinarySearchDirectory. Compile-only crossOS checks do not execute this lifecycle.

JSONEncoder pre-encoding allocation limitation remains acknowledged and must not be described as a proven bounded heap. Full native rebuild/query proof remains owner-controlled after these concrete findings.


## Frozen Candidate Domain and Metadata Delta

Read-only review after owner freeze; no builds, queries, tests, or Source changes. No new concrete release blocker found in this delta. Canonical tree cache stores buildRepositoryCodebase only and invokes fresh buildRepositoryDomains after snapshot loading. Domain discovery adds exactly six workers and starts exactly six; ticket streaming happens separately and drains its completion error, avoiding the former join count mismatch. Worker errors are joined after completion. Metadata decoding rejects unknown fields and requires EOF on a second decode. Payload trailing decode retains both the decoding error and cancellation identity. Production root canonicalization remains owned by workspace selection; physicalSearchTempDir canonicalizes newly allocated standalone test anchors without relaxing descendant symlink refusal.

This is a static delta review, not independent execution of the owner’s pending final gates. The previously acknowledged whole-value JSONEncoder allocation limitation remains an observation, not proof of bounded heap consumption.
