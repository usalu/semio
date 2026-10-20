# Dashboard Source Workspace

Status: implementation in progress; no new runtime pass claimed.

The native dashboard workflow audit found no built-in file browser/editor and a reachable external IDE delegation. The execution owner is adding a reusable multiline terminal widget, dashboard browser/editor workers, localized customizable actions and real edit/save/conflict journeys. The controls owner is retiring external source-open delegation.

The coordinator owns the domain-neutral filesystem workspace: schema-first16portable path/text vectors, Rust physical implementation, TypeScript port implementation, independent Ajv/SQLite oracle and actual Rust filesystem laws. The fixed limits are1MiB UTF-8 files,4096direct children,256UTF-16physical path units and64MiB retained command history. Revision tokens include authoritative identity and exact bytes. Pre-publication conflict/cancellation retains original source; rare kernel-exchange races retain their foreign predecessor and expose a structured recovery path. The UI must explain that the local version may already be published in this latter case. No atomic compare-and-swap guarantee against arbitrary noncooperating writers is claimed.

Publication uses a private native boundary with [Apple swap rename](https://developer.apple.com/de/documentation/foundation/urlresourcekey/volumesupportsswaprenamingkey), [Linux rename exchange](https://www.kernel.org/pub/linux/docs/man-pages/book/man-pages-6.16.pdf) and [Windows ReplaceFileW backup](https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-replacefilew). Windows file identity uses a stable system call because [Rust MetadataExt identity methods remain nightly-only](https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html). Those references support platform API choice; host runtime evidence is still pending.

Independent review identified the browser root spelling mismatch (fixed in UI), worker channel disconnect retirement and late cancellation after an actual commit (fixed in UI), and a path-ancestry race requiring descriptor anchoring (being corrected by the coordinator). Source-member rows for filesystem workspace and snapshot projection were added to both physical Cargo workspaces. Final lock refresh waits for all collaborating manifest sets to settle.

## Physical Ownership And Native Adapter Review

Both new libraries are physically owned by the framework Cargo workspace. The first Root lock attempt correctly refused101 after the coordinator incorrectly also listed them as Root members; those two added Root member rows were removed, retaining Root dependency rows for cross-workspace consumers. Framework lock refresh28180 returned0; offline resolution updated three existing registry versions, so exact supported precise updates are restoring the prior pins before final readiness. No native matrix was released during this correction.

Unix source operations are now descriptor-anchored through root/parent directory handles; listing uses a fresh directory description and bounded cancellable native records. Windows pins physical parent handles without delete sharing, rejects reparse points and uses actual handle paths. Independent bounded review accepted their direction and caught the partial-retention edge: post-exchange failure preserves .new/predecessor bytes, now exposes a structured recovery path and an authenticated readonly inspection path. UI distinguishes a known raced publication from an unsettled I/O publication.

The Unix publication path now uses one kernel exchange and retains the predecessor at its existing transaction path; there is no second post-exchange rename. This removes the reviewed retirement-failure window. Post-publication I/O failures expose whichever owned version remains, append a retained outcome where possible, and permit readonly inspection authenticated by the pre-existing immutable intent if the outcome itself cannot be written. Actual physical tests now cover outcome-write failure, readonly recovery, final-checkpoint ancestry replacement, repeated directory listing and cancellation after a successful commit. These bodies are authored but not yet executed.

## Coordinated Lock Readiness

Framework28180 returned0, then owned precise updates38532/30525/93417 each returned0, restoring all612registry tuples exactly. Corrected Root workspace-local update18629 returned0 with all706registry tuples unchanged. No registry dependency change remains in this editor/projection batch. The sole11package native matrix57413 is now compiling Rust after passing all strict Source admission, including the prior33rdfile refusal; no feature body or whole-matrix pass is inferred yet.

## Actual Native Filesystem Acceptance

Isolated official repo Cargo run53877 returned whole-command0. The actual semio-framework-filesystem-workspace library executed10named tests with10passed,0failed,0filtered in0.46seconds; build took12.49seconds. Tests exercised sharedportablevectors, real Unicode/CRLF save/reopen and retained journal, foreign/pre-publication conflict, cancellation before publication, cancellation after committed save, actual outcome-write failure and readonly recovery, binary/extent/revision refusal, repeated directory listing/cancellation, nofollow source/state links, and ancestry replacement. No extra Source limit or ownership exception was used: actual native source admission observed1unit,4reads,3files,55655bytes. These are macOS host results; cross-platform native adapter source and neutral policy cases do not claim Windows/Linux host execution.


Registered the new Source/native workspace owners and the companion snapshot-projection owner in the existing static build-owner registry. These are exact three bounded rows; other agents’ registry content was preserved. Actual native10-law pass predates this metadata-only registration and is unaffected; Source target invocation remains in current25-task retry.
