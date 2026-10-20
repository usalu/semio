# Final Metadata Observer Independent Review

Read-only review of held14-input roster `aa955fdf4a2707a7e968e15e00b43d82f15be060dacccebdc05e8322f7078a94`: all sizes/hashes matched actual bytes. No tests, Docker calls or guest operations executed.

The actual core verifies managed21 terminal and canonical/current plan binding before reads, authenticates engine and exact builder custody four times, obtains terminal-bound raw bytes, and requires every actual build-ID line to agree. Tag and immutable config ID are both inspected before/after; Linux amd64/config/rootfs and pinned Caddy digest remain required. It does not infer a producer receipt or final Source equality. Actual guest read code has no guest writes/lifecycle commands, validates physical owner/terminal/ancestors/single-link nofollow output, awaits each stdout callback and uses bounded deadlines. Host evidence is exclusive under generated.

Neutral tests exercise actual core ports and AST-extracted guestLog against generated files, including owner/terminal mismatch, mutation, hardlink/junction, raw Unicode bytes, delayed callback, repeated/conflicting IDs, custody/tag/config/digest negatives. Python hashlib/SQLite and Ajv are independent prepared oracles; no executed success claimed.

Two corrections pending consolidated reseal: retain signal listeners through receipt publication and check issued control/pins/output again afterward, so observed abort cannot end with actual0. Also replace small() readFileSync(fd) with initial-size bounded reads, retaining final descriptor/name fences; stat-only bounds do not prevent concurrent metadata growth allocation. Release acknowledged both and is preparing actual vectors. No other material blocker found in current bounded review.

## Corrected Exact Candidate

Reread corrected14-file roster `565a4ee7094bf47630b68ec0486cfe6a30c5425b57457a9b15e0d1971f12e1d7`: all sizes/SHA match actual bytes. Actual guest small metadata reads now allocate only initial admitted size and read within it; final descriptor/name fences reject growth. Actual AST-extracted metadata-growth vector forbids readFileSync and asserts all read buffers≤65536. Signal listeners now remain through publication; actual exported publishFactsReceipt invokes the final checkpoint afterward. The cancel-at-publication case injects abort within that actual helper and requires refusal.

No remaining material blocker found for exact34-case native neutral testing. No tests, Docker reads, private effects, artifact approval or actual observer success occurred during audit. An accepted receipt alone remains insufficient: actual owned process0 and unchanged evidence are required.
