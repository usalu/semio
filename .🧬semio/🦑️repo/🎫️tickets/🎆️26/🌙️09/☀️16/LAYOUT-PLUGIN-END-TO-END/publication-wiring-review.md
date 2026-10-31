# Publication Wiring Review

Read-only source audit, 2026-10-29. No builds, tests, cache operations, publication, receipt changes or active-process interventions were performed. The existing native2163 lane remains independent. Read the build skill, persistent-native-publication-review.md and typescript.md before tracing.

## Conclusion

No verified declared CLI recovery route currently reconciles the stale dependency publication. `repo --service=off tools setup all` has the exact Original purpose accepted by the publication API, but its production dispatcher does not call that API. Worse, its dependency reader opens before setup handling, so the same stale receipt can refuse before any setup implementation executes. Repeating setup-all cannot be claimed to repair this state from the inspected wiring.

Envelope-only publication *can* satisfy the shown filesystemBytes equality without resizing. The field in the newly published receipt is the configured envelope, not measured statfs geometry. The publisher explicitly writes options.filesystemBytes and preserves the old backing size. Its mounted observation separately checks actual statfs bytes are positive and at most that envelope. Thus the old 8 GiB backing plus a new approximately 36 GiB envelope is internally admissible at these boundaries. This does not create 36 GiB of actual filesystem capacity or prove that future large writes fit.

## Production Dispatch Evidence

Paths below are relative to the repository root; line numbers refer to this inspected workspace.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts:68-76`: imports the ordinary Store, enters `withRepositoryCacheStoreV1(root,"dependencies",...)` at 69, then checks tools/setup grammar at 70-74. Toolchains run at 75; registered backend setup methods run at 76. No publication step precedes the reader.
- `.../🏗️build/🔌️backends/🟦️.ts:94-104`: dynamic backend loading uses exactly the registry module and checks exported backend id. Its BuildBackendV1 interface provides setup, reset, prepareStorage, cache, remoteCache and execute, with no hidden image-publication hook.
- `.../🏗️build/🔌️backends/📋️registry/🔣️.json`: the actual registered implementations are Nx, Bazel and Semio. Their setup methods were read, not inferred from interface names.
- `.../🏗️build/🔧️tools/🧶️nx/🔌️backend/🟦️.ts:14-16`: setup delegates to runOwnedNxInvocationV1 with --version.
- `.../🏗️build/🔧️tools/🏗️bazel/🔌️backend/🟦️.ts:12-21`: setup uses withBazelSetupStorageV1, prepares the pinned binary/modules, then reports readiness.
- `.../🏗️build/🔧️tools/🐙️semio/🔌️backend/🟦️.ts:135`: setup only reports that no external tool needs provisioning and checks control.
- `.../🏗️build/🧰️tool-setup/⌨️command/🟦️.ts:136-169`: runToolchainsSetupV1 provisions tools through existing stores/installers and optionally warms compiler helpers. It has no canonical image-publication dispatch.
- `.../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts:135-137`: ordinary opening recovers an *unsupported* inactive publication. The inspected reader rejects this schema-valid older envelope instead. It is not an automatic envelope-republication route.

A repository-library rg search for publishRepositoryCacheImageV1 and repositoryCacheImagePublicationV1 found declarations and calls in the prepared-worker test asset-publication helper, not production setup. The helper is a real explicit test/qualification worker with physical census and custody checks; its existence does not connect production tools setup all to publication and is not general recovery authorization. Rust native subtree searching for tools/setup and asset-publication found no interception; this is a bounded source audit, not an exhaustive proof of every possible external launcher.

## Receipt And Geometry Evidence

- `.../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts:77`: repositoryCacheImagePublicationV1 authenticates the genuine Original and actual phase, requires original arguments exactly tools/setup/all, selects only a declared partition, and derives filesystemBytes/capacity from current repositoryCacheBudgetV1.
- Same file line 81 delegates to publishNativePlatformImageV1 with that closed request and optional awaited progress.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🎛️owner/🟦️.ts:65`: publishNativeImageReceiptCandidateV1 writes filesystemBytes:x.filesystemBytes and maximumPhysicalBytes:x.maximumPhysicalBytes while size remains scope.request.backing.size. It CAS-checks the old receipt and atomically publishes the new receipt. It does not resize the backing.
- Same owner line 69: observeMacPublication accepts actual filesystem bytes in (0, x.filesystemBytes]; rejects mounted descendants and ambiguous mapping.
- Same owner line 81: publisher takes exclusive native-fixed-image custody with timeoutMs 0, observes, witnesses, detaches if needed, publishes, attaches, verifies payload and settles owned cleanup. These are real physical effects and must not run against active peer custody.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:139`: inspectPersistentReceiver requires authored.filesystemBytes === requested.filesystemBytes, allows authored.maximumPhysicalBytes <= requested.maximumPhysicalBytes, binds actual backing identity/size to receipt, and later accepts mounted statfs <= requested.filesystemBytes. Therefore the recorded 8588886016 versus 38653657088 equality would be repaired by the publisher-authored envelope, even with unchanged backing size. All other identity, ancestry, namespace, witness and custody checks still apply.

## Existing Test Evidence And Limits

Inspected existing tests, not executed here:

- `.../📤️publication/🧪️tests/🏗️native-build/🟦️.ts:15` verifies genuine Original/phase identity, rejects copied authority and forged purpose, and preserves issued envelope identity. Lines 13-14 compare request/stage/failure models with Ajv and SQLite. Line 21 exercises actual extracted reader logic to refuse pending publication before kernel commands.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧪️tests/📥️inspection/🟦️.ts:20` exercises actual extracted receiver functions with Ajv and SQLite for published device reassignment and changed custody. It does not establish production setup publication orchestration.
- `.../📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:61-63` constructs a fresh observed request, calls canonical publication and performs an ordinary reopen after success. This demonstrates the intended owner protocol in the qualification helper, not a wired setup command.

No current test outcome is claimed. Prior TypeScript direct 75-test and strict compiler results remain those in typescript.md; the declared target is still unverified due to the recorded pre-test receiver refusal.

## Required Owner Work

The missing integration belongs to the genuine setup-all owner: route canonical observed publication before the stale dependency ordinary reader, preserve original issued authority, acquire actual exclusive custody, refuse descendants or active holders, publish through the canonical API, then open the ordinary receiver. Add a language-neutral stale-envelope fixture and independent oracle for the complete dispatcher ordering and unchanged-backing/envelope semantics before implementation. No cap relaxation, receipt editing, cache deletion, forged envelope or test-helper substitution is needed or justified.

This audit provides a concrete missing-orchestration finding and a source-derived explanation of envelope semantics. It does not authorize live recovery, establish quiescence, prove another platform end to end, or alter the parent native compiler wait.
