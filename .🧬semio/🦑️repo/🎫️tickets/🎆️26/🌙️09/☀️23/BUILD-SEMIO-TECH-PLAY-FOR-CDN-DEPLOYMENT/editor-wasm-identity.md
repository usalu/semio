# Editor WASM release package identity

Run `wCsXqmg` reached editor package publication. The stack in `.cache/b/r/m/components/runs/wCsXqmg/l/job-editor-wasm.log.tail` originates at the explicit package recheck after `stageRepositoryArtifacts` in the repository WASM policy. The editor-specific consumer had yet to run. The guard is comparing a compiler output file's filesystem metadata. The error carries neither the file name nor the mismatched field, so the retained log is insufficient to identify which identity fact changed.

The generic publisher opens its input read-only, verifies the input descriptor and path, opens a distinct output inode with `wx`, copies the bytes, and renames the output directory. The editor consumer subsequently verifies the produced package. The current source provides no evidence that the compiler package name, manifest settings, or the generic publisher changed the guarded source.

Isolated macOS probes using Bun 1.4.2 exercised fresh and identical publication with editor-sized and editor-shaped files. The generic and funded staging probes preserved guarded identities, while ordinary reads advanced atime. An isolated same-mode chmod advanced ctime with the bytes intact. These probes ran on the ticket filesystem. The release ran through mounted stores and its initiating mutation remains unconfirmed.

## Change and verification

The produced WASM package guard now names the logical file and reports changed exact fields or a missing path. It matches the bigint identity on both sides of digest reads, including the first admission, and continues to refuse metadata or byte changes. It ignores atime as before. The byte check identifies its file too.

The language-neutral fixture exercises stable content with ordinary reads, explicit retiming, same-byte inode replacement, hardlinking, and disappearance. The Bun test applies actual filesystem operations and checks unchanged bytes through Bun's file reader. The original fixture test failed without the diagnostic implementation. After the change the new test, schema admission, renewable deadline, and lease-order tests passed. The three existing neutral Node bundle tests stop at esbuild resolving a literal `bun:ffi` import in the host-capacity native-memory module. The module is outside this change.

No Cargo build or live release replay was run. The initiating release mutation remains unverified. A subsequent refusal will identify the affected file and fields, and ownership failures remain fatal.

## Bounded observation retry

Each member observation can retry once on a timestamp-only identity difference or on a typed physical observation change. Both attempts use the originally admitted bigint identity. The retry must observe the same device, inode, size, mode, link count, mtime and ctime before and after hashing, and consumer rechecks compare the digest and length with the originally admitted bytes. Permanent timestamp differences, inode replacement, link or mode differences, and missing paths still fail. Read failures and digest disagreements still propagate immediately. The retry checks the existing operation budget and cancellation before reading.

The language-neutral fixture runs the actual package consumer and forces a single timestamp discrepancy, an observation interruption, persistent differences, an inode discrepancy, an I/O error and a different digest. It counts attempts and compares SHA-256 from Node crypto with Bun's independent hasher. The transient timestamp case reproduced the refusal before the fix. The final Bun WASM compiler owner suite passed all eight tests, including the Node bundle cases that had failed in the earlier run. The two identity tests passed in 25 fresh Bun processes, and the combined WASM compiler owner and Windows native memory suites passed all 12 tests. No concurrent Cargo, release, or describe run was started for this fix. The original release failure still lacks an identified member and a reproduction.
