# PDF save readback

## Why save reported `natural-file.archive-not-persisted`

`persistAttachedProgramDocument` waits in `awaitVerifiedReadbackV1` until an archive event carries the bytes just written. A persisted status only asks the worker to re-read the folder (`externalChanged` → `pollFolderOnce`).

`pollFolderOnce` treats bytes equal to `state.folderArchive` as an own-write echo and does not emit `documentArchiveReplaced`. That echo rule is required: `folder-archive-restore` asserts a `localDocumentArchive` followed by `revalidateFolder` emits no `documentArchiveReplaced`. The files are on disk, and the shell never sees them, so the wait ends as `natural-file.archive-not-persisted`.

## Fix

A persisted status is still not proof by itself. When the caller supplies `read`, that function is the fresh readback: if it returns bytes `accepts` keeps, the wait succeeds. `persistAttachedProgramDocument` reads `GET /semio-backbone?uri=folder://<binding>&documentId=<id>` for the session's persisted-local folder binding. The echo event is unchanged, so a save does not reload the document.

## Test

`engine/tests/catalog-acknowledgment/readback.ts`, 8 passed, including the new case: a persisted status plus a fresh read of the expected bytes acknowledges the write. The older corpus still proves that a persisted status alone is not content proof. Ran with this ticket's `vitest-readback.config.ts`.
