# Current Bundle Source Input Refresh Review

Read-only review of actual current bytes on 2026-10-20. No commands, tests, image actions or Source edits were performed. The previous producer bytes are unavailable in this review; no exact historical diff is asserted.

- `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🎁️stack-bundle/🟦️.ts`: 25343 bytes, SHA-256 `5e2609c9ade77ea8c039be6feb3605814389e8074697c8beae9b485ef8628ef4`.
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts`: 26345 bytes, SHA-256 `1404302a1bf58e00ce6e32f9ea33531d3b98a74f0095a27209bc808dc5e9836a`.
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚛️bundle/🔣️.json`: 4657 bytes, SHA-256 `8b020274b80c06faa449ee11b7d5f586321660c909636197c254fb9ab351f63b`.
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚛️bundle/🧬️schema/🔣️.json`: 3628 bytes, SHA-256 `a35c072c8f093372542eb06cdec5c8dcf10aa5adaff0bfe735ac2e983c16c0a5`.

The fixture and schema match the prior review and retain all 23 export vectors. The actual test file has a different current hash and size from the historical review, so it was reread rather than treated as unchanged. Its exported test registration executes the actual saveStackBundleImages and verifyStackBundleArchive implementations. Real tar-stream encoding/decoding, noble SHA-256 and Ajv provide the independent archive/config/tag/platform oracle, including the mutable-tag ABA negative, before/after retag negatives, malformed framing, shared configs, Linux-platform refusal, OCI mapping and cancellation cases. This is prepared test coverage, not an observed pass.

Current producer lines 150–167 capture immutable inspected proctor/Caddy config IDs, use the proctor ID for labels and extraction, and create the offline Caddy alias from its immutable ID. Lines 42–44 inspect all three references before save, after save and after archive verification. Saving the required tags retains offline RepoTags; verification of the actual archive config bytes and exact required-tag associations at lines 61–66 rejects an ABA-selected replacement even if the live tag returns to its original ID. Linux/amd64 config checks and optional OCI digest/size/config/layer-map/tag checks remain present.

The archive reader remains bounded and retains no-follow descriptor/name/ancestor stability checks, duplicate/ancestor conflict refusal and framing checks. It does not unpack arbitrary layers or establish layer-content hashes/ELF correctness; those remain separate final artifact checks. Staged proctor and Caddy pull policies remain never, while operator settings, certificates and volumes retain their existing contract.

Disposition: no material blocker found in current producer 5e2609c9ade77ea8c039be6feb3605814389e8074697c8beae9b485ef8628ef4 or the reread current regression implementation. These exact bytes are clear for inclusion in a fresh native verification seal. This is static readiness only, with no native test or artifact approval claim.
