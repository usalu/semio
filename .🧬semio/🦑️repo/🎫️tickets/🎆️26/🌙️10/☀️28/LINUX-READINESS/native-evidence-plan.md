# Stable Native Evidence Input

The retained cold/warm acceptance input currently hashes snapshot payloads but records only inode/size/mtime for index shards. Add bounded streaming SHA256 and descriptor/leaf identity checks for every actual shard, checking each observed digest and length against its admitted manifest row. Preserve exact64 occupied-shard coverage, real Native reader validation, existing cancellation/query budgets and protected cache content. This changes ticket verification inputs only; it does not establish new Native runtime success.

Test first with language-neutral vectors, actual ordinary/symlink/hardlink/mutating files and cancellation, using existing Ajv and independent @noble/hashes SHA256. All test artifacts stay in this ticket's generated directory. No Source copies, new runtime dependencies or compiler execution.

## Streaming Evidence Implemented

The retained input now hashes every snapshot file, index metadata/manifest and all64 admitted shard files with256KiB streaming reads. It validates exact single-link ordinary-file identity before open, from the live descriptor during progress, and again from descriptor and leaf after reading (dev/ino/mode/nlink/size/mtimeNs/ctimeNs). Each shard's actual byte length and SHA256 must equal its admitted manifest row. Metadata capture is capped at64KiB; payload/shard caps come from the current Source protocols. No protected cache was read or changed in these focused tests.

Actual Node24 tests:9passed,0failed. Language-neutral cases cover empty/small/multiple-block files, byte-limit refusal, symlink and hardlink refusal, mutation during a read, cancellation and bounded metadata capture. Existing Ajv validates the corpus; independent existing @noble/hashes SHA256 agrees for each positive output. The initial helper test first failed because the new module was absent; bounded capture additionally had a genuine behavioral red8passed/1failed before implementation. Input syntax check passed. Temporary fixture directories were removed by finally blocks; retained test/schema inputs remain in the ticket.

Snapshot/index observations are async, report progress every16MiB, and accept cooperative SIGINT during observations without altering the existing Native query's cancellation owner or limits. This is verification-input readiness only. No real Native cold/warm/cancellation query or new installed-artifact success is claimed.

Changed inputs: native-evidence.cjs, native-evidence.test.cjs, native-evidence-vectors.json and native-layout-acceptance.cjs. Existing physical identity comparisons now include hashes for all shards, rather than relying on Native replay alone. Per-file stability does not create an atomic multi-file tree snapshot: concurrent generation changes must still cause the acceptance run to be rejected.

## Actual Shard Observation Integration

A further9tests passed for the actual retained indexState/evidence/observe function bodies, selected by the existing third-party TypeScript AST parser without copying Source files. The language-neutral corpus covers valid sparse and full64-shard manifests, incorrect hash/length, duplicate/unsorted/out-of-range shard names, incomplete full-workload coverage and document-count mismatch. An independent SQLite relation computes acceptance; independent @noble/hashes supplies actual event digests. All18focused evidence tests are green; no Native query was executed. Temporary fixture files were generated under the ticket and removed after each test.
