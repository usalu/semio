# Windows Conditional File Publication

## Scope

The first-party filesystem descriptor interface already exposed conditional file publication, but Windows refused it before effects. Dependency synchronization and lock publication need this primitive. The Windows implementation now captures bounded source facts and SHA256 under the original rooted directory, accepts only its privately issued single-use expectation, stages and flushes captured bytes, retains an existing inode under an absent private name, and publishes through a no-replace rename. It verifies named identities, complete digests, directory roster and actual flushes after handles close. Failure retains prior and staging bytes without deleting unknown content or rolling back over another writer.

The Windows schema explicitly describes preservation followed by absent-name publication under domain exclusivity. These are separate no-replace rename operations; it does not claim the POSIX exchange operation. All native handles remain private to the existing descriptor custody registry. No runtime library was added.

The native rename structure and DELETE access requirements were checked against [Microsoft's FILE_RENAME_INFORMATION specification](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_file_rename_information) and [NtSetInformationFile specification](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntsetinformationfile). The implementation uses a relative target anchored to the actual parent handle, with replacement disabled.

## Test-Driven Evidence

The initial runtime test failed on the existing Windows publication refusal. The implementation then passed five shared language-neutral positive cases, including missing/empty files, replacements and Unicode bytes. The source/caller corpus covers copied and proxied expectations, proxied bytes, parent and source changes, hardlinks, directories, cancellation and byte bounds. All eleven selected hostile cases refused before publication effects.

`bun test ./🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/📄️publication/🟦️.ts`, with explicit ticket-generated test output: **3 passed, 0 failed, 65 assertions**. Actual Windows behavior emitted `[DEBUG]` test evidence. AJV admits the shared JSON cases, and the independent Noble SHA256 implementation agrees with published and retained bytes. Both defining suites are included in the existing filesystem observation test command.

The read-only integration auditor found no reproducible P1/P2 defect in the publication fences. Final strict TypeScript checking of the descriptor implementation and both descriptor/publication test suites passed without diagnostics; the source file branch narrows the shared corpus's nullable prior string before passing it to the independent matcher. The combined execution-owned gate passed 18 tests with 1,051 assertions. This primitive qualification does not claim full dependency setup passed: mounted cache storage is still subject to the host prerequisite documented in `storage-2026-10-19.md`.

The publication suite's disposable output directory was verified empty and removed using nonrecursive deletion after validation. Input fixtures and this report remain intact.
