# Lane `revision-cas`: MVCC revision pinning over the shared CAS

`REV` = `BUILD/🧊️revision`. Status: schema, tree, staging, pin lifecycle, frozen view, materialisation and tests done and green. Cut-over of the planner/executors is left to `daemon`, `scoped-plan` and `integration` (requests filed in `📌️requests.md`).

## Design
- Revision = immutable snapshot of exactly one build's input closure. `RevisionV1 {id, workspace, journalCursor, parent?, roots:[{path,tree,size}]}`; its `id` is the sha256 of its canonical bytes, so the revision is itself a CAS object (`loadRevisionV1`).
- Content is a Merkle tree: one canonical-JSON tree object per directory (entries `{digest,kind,name,size}` sorted by code point), stored in the shared `CasStoreV1` (`BUILD/🗄️cas`). `InputManifestV1 {revision,digest,entries}`: `digest` is the root tree digest. Canonical JSON and all digest rules are written down in `REV/🧬️schema/🔣️.json` (`$comment`); the Python oracle re-implements them from scratch.
- Staging (`pinRevisionV1`): each input is streamed once into the CAS (bounded chunks, parallel `concurrency` 16), then re-stat'ed; changed during the read -> retried up to `maximumAttempts` (3); if it keeps changing the revision records the content actually read (`torn` lists it). Missing inputs are listed in `absent`. Symlinks are inputs (digest of target text); other file types are refused (`unsupported-entry`).
- O(changed): a `parent` revision plus the journal `DirtySet` reuse unchanged entries without reading (only if the object is still in the store); `overflow` restages all.
- Pin = `cas.pin(holder, all digests)` (files, tree objects, revision object), one holder per pin, so refcount = live holder files; `releasePinV1`, `renewPinV1` (fails `unknown-revision` if an object is gone), `withPinnedRevisionV1` (auto-renew every 5 min, release on success, failure and cancellation). cas-gc needs no extra rule: pins are its roots.
- Frozen reads: `openPinnedWorldV1` implements the derived-dag `WorldPortV1` (`read`, `files`) over pinned content (optional re-hash `verify`); `materializeRevisionV1` places a sandbox dir (reflink else copy, never a writable hardlink, path-escape check). There is no "source changed -> re-plan" for a pinned build: the workspace is not read after staging.
- Results by content: `actionKeyV1({tool,command,environment,manifest})` has no workspace/path/time; equal trees in two workspaces give equal manifest digests and action keys, and the CAS stores the content once (test: second workspace adds exactly 1 object, the revision object).
- `stageRevisionV1 {revision,paths} -> {refs:[{hash,size}]}` re-ensures content; evicted content is restaged only if the workspace still has exactly that digest (else `content-evicted`).
- Typed errors with EN+DE texts (`revisionTextV1`): unsafe-path, content-evicted, too-many-paths, unknown-revision, unsupported-entry, invalid-revision.
- Executable bit is recorded only where the platform has one (not on Windows), so a manifest can differ between OSes by `mode`.

## Verified
- `repo --service=off tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧊️revision/🧪️tests/🟦️.ts`: 18 pass, 0 fail (242 expects), three consecutive runs. `🗄️cas` suite still 13 pass.
- Independent oracle `REV/🧪️tests/🐍️oracle.py` (stdlib `tarfile`, `hashlib`, `json`): manifest entries, tree digest and action key equal for the fixtures (emoji, astral, fullwidth names, executable, empty file, symlink) and for 25 seeded random trees, including digest of a tarball snapshot taken at R before edits.
- Ajv validates policy, fixtures, `Pin`, `Revision`, `InputManifest`, `StageRequest/Reply`, path rejection cases, EN/DE message keys.
- Concurrency: edits after the pin never change content, manifest, materialised files or action key; edits injected before/after read at each staging attempt give a valid revision equal to a real snapshot of that file; perpetual edit gives bounded attempts and the content read; touch-only changes keep the digest; reuse/dirty/overflow/lost-object cases; refcounted pins, lease lapse and renewal; `withPinned` release on success/failure/abort; progress, cancellation, path bound.

## Not done
- Acceptance numbers (0 "source changed" restarts under `n-agents-edit-storm`, staging <= 5% of cold build) need the cut-over and the bench lane; the old restarts live in `🕸️graph/🟦️.ts` and `owner-discovery` (outside this lane).
- `🧷️pin` (generation pin) left untouched: it pins the fact-index generation, the revision pin the input content; a plan request carries both.
- No stat-keyed digest memo across processes (reuse comes from `parent`+dirty); a resident daemon can hold the previous `PinnedRevisionV1` per workspace.
- Windows/macOS not run (only `node:` APIs); TypeScript type check not run.

## Daemon wiring (added)
- `BUILD/🏡️resident/🧬️schema/🔣️.json`: `CasRef` is now `{hash,size}` (the revision shape); `RevisionId` documents `RevisionV1.id`; new `Dirty` (journal `DirtySet` + `cursor`) and `Pin`; `PlanRequest` gains `parent`, `dirty`; `PlanReply` gains `manifest`, `pin`. Fixtures regenerated by the Python oracle (2224 frame cases).
- Host service passes `parent/dirty/cursor/agent/root` to the planner port, never memoises plans with a dirty set or a pin, returns `manifest`/`pin`; client `requestPlanV1` accepts `parent`/`dirty` and resolves spills by `hash/size`.
- Planner port takes a `pin` hook; `BUILD/🧊️revision/🔌️plan` `createRevisionPinHookV1({cas,inputsOf})` implements it (parent loaded from the store, ignored if gone).
- Verified: `🏡️resident/🧪️tests/🌐️host/🟦️.ts` + `📨️protocol/🟦️.ts` 34 pass 0 fail (includes new end-to-end test: reply carries pinned revision, edits after the reply do not change pinned content, parent+dirty gives a child revision with the journal cursor); revision suite 18 pass; strict `tsc` (strict, exactOptionalPropertyTypes, noUncheckedIndexedAccess) over `🧊️revision` incl. tests: 0 errors (3 pre-existing errors in unrelated `esbuild` and `schema/validator` files pulled in via imports).
