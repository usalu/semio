# Artifact reader "Original artifact bytes or identity changed" (2026-11-27 04:21)

## Verdict
The writer-side hypothesis is not the cause. It is not a writer/reader race.

- Every writer already takes the exclusive lease on `"artifact:"+path`
  (`stageArtifacts`/`executeArtifactPublicationV1`, `withFreshArtifactPublicationWritersV1`,
  `stageOwnedArtifactsV1` in `framework/modules/process/artifacts/publication`), and the repo
  artifact destination (`withRepositoryArtifactDestinationV1` -> `stageRepositoryArtifacts`) goes through it.
  The existing proof `reading/tests/physical` already asserts the writer is blocked while a reader is live.
  Publication is also rename-swap of a staged directory, and identical content is not replaced.
- The failing step is the settlement `recheck` in `withRepositoryArtifactV1` (reading/index.ts:34). It compares
  `inspectArtifactV1` fingerprints, and directory entries were fingerprinted with
  `dev:ino:size:mode:mtimeNs:ctimeNs`.
- On macOS the published directory `.../tools/descriptor-emitter/darwin` showed
  `mtime 04:21:11` but `ctime 04:21:27` and carries `com.apple.macl` and `com.apple.provenance` xattrs.
  The OS adds these lazily and bumps the directory change time without any entry change.
  The shared lease was held the whole time and the entries (names, inodes, bytes) were unchanged.

## Root cause
Directory identity included `ctimeNs`, a metadata-change time that the platform changes for extended
attributes, chmod/chown-to-same and similar, so a reader's own recheck could fail with no writer involved.

## Fix
`reading/inspection/index.ts`: directories are now bound by `dev, ino, mode, mtimeNs`
(`directoryKeys`), both in the inventory fingerprint and in the during-inventory check. Entry
additions, removals and replacements are still caught: mtime changes on entry changes, the declared
file inventory is compared by name on each inspection, and every file keeps its full
`dev:ino:size:mode:mtime:ctime` plus sha256. Files are unchanged. The group reader shares the same function.

## Test
`reading/inspection/tests/identity/` (`fixture json` + `index.ts`): language-agnostic fixture cases
(directory ctime bump stable; same-size rewrite, stray entry, removed file detected), schema-checked with Ajv, digests
cross-checked against an independent hasher (`Bun.CryptoHasher` vs `node:crypto`).
Before the fix: `directory change time bump is stable` failed (directory ctime differed). After: 5/5 pass.
Also green: inspection/tests (3), reading/tests (1) via `repo test-file`.

Not run: the full physical reader proof (needs the repo script harness), and the Base chain re-publication.
