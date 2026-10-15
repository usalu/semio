# Bounded SDK Package Archive

Status: The first-party gzip/tar package reader passed actual Original-controlled prototype and release execution. Integration publication is still pending; this report does not qualify a fresh SDK or an ordinary build pair.

## Contract and Boundaries

Production owner: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🗃️archive`.

The closed policy admits at most 32 MiB compressed input, 64 MiB inflated material, 4,096 tar headers, 16 MiB per regular file, 256 UTF-8 bytes per archive path and 64 KiB per PAX metadata body. Both the first-party schema subset validator and an independent Ajv validator enforce the policy shape. The reader uses Node framework crypto/zlib/stream libraries and adds no external runtime dependency.

The reader copies bounded compressed input before hashing and inflation, returns the SHA-512 identity of that exact private copy, and exposes borrowed regular-file bodies from one zeroed 64 MiB arena. It no longer retains all inflated chunks and concatenates a second full decoded payload. Caller-owned input, zlib/runtime memory and stream buffering are separate; this is not an exact process RSS quota. The 32+64+16=112 MiB coupling check against the 128 MiB publication allowance concerns physical publication writes, not a hard memory ceiling.

Only regular files and directories with checked POSIX tar framing are admitted. The reader checks checksums, octal fields, whole 512-byte blocks, zero body padding, a complete zero footer, bounded PAX/GNU long names and exact metadata semantics. It refuses traversal, foreign prefixes, special entries, links, sparse files, Windows reserved names, unsafe separators, duplicate normalized/case-insensitive paths and file/ancestor collisions. Unknown PAX semantics fail closed.

Actual progress/cancellation comes from `scriptCompilerAdmissionControlV1` derived from the received invocation. The inflater observes and drains its stream completion before the scope returns, including failure and abort. The decoder performs no filesystem writes and grants no storage authority. Its integration must separately authenticate the lock SRI and retain native stage/path/shared-inode custody.

## Test Development and Evidence

Language-agnostic fixtures and a closed fixture schema live under `🧪️tests`. Bun.Archive provides independent tar creation/reading, Ajv validates schemas and Bun.CryptoHasher independently verifies SHA-512.

- Original 26372 was genuine RED because the production decoder did not exist. Initial implementation Original 68586 passed the first oracle law.
- Original 87045 exposed a strict type error passing a ScriptControl where a ResourceLeaseControl was required; the test was corrected to derive the actual admission control. This was a test-setup defect, not an archive runtime claim.
- Hostile framing execution79552 and strict99270 passed after checksum/path/link/sparse/padding/truncation/footer/duplicate coverage was added.
- Original 5612 was genuine RED: the decoder accepted an unaligned tar footer. Production now requires whole 512-byte blocks. Normalized/case-insensitive collisions and ancestors were covered in the same closed corpus. Original 45089 and strict73454 passed.
- Expanded Original 2614 passed six laws before the arena refinement; strict93425 exited zero.
- Final zeroed-arena Original 71758 prototype: **6 pass, 0 fail, 40 assertions**, 787 ms test runtime. Original 30281 release: **6 pass, 0 fail, 40 assertions**, 1.77 s test runtime. Nx targets completed successfully with actual execution and zero cache hits.
- Final strict Originals 10655 prototype and 38805 release both exited zero through the actual managed tools/lease/SDK harness.

The six laws cover independent regular/Unicode/NUL/empty-file equality, long paths, closed byte/publication ceilings and cancellation, PAX/GNU equality and unsafe metadata refusal, file/entry/metadata/path ceilings, and fourteen hostile framing/namespace cases. `[DEBUG]` runtime records confirmed the respective branches under real invocation control.

The Nx execution owner separately authenticated the actual locked Nx registry artifact: 5,194,505 compressed bytes, 19,339,264 inflated bytes, 1,347 regular files, largest file 5,645,406 bytes and longest path 92 UTF-8 bytes. The current decoder accepted that artifact; all 34 patch postimages matched an independent diff oracle. See [SDK Pristine Dependency Base](sdk-pristine-dependency-base-current.md) for that separately owned evidence and integration state.

## Remaining Qualification

A new pristine dependency base and derived SDK must publish through two distinct one-use scoped quotes with fresh physical headroom observations, then pass all 31 mandatory authorizer hooks. The graph-independent Native cache-index regression and ordinary seven-phase prototype/release sequence remain mandatory after that publication. Whole-checkout predictability, historic unmanaged data and unqualified platform enforcement remain open in [Repository Cache Budget](repository-cache-budget.md).
