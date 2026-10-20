# Nonempty Directory Command Design

## Current implemented scope

The authored backend boundary has descriptor-relative native move_no_replace ports for Unix and Windows. Guided UI commands intentionally restrict directory rename/retention to empty directories. Nonempty directories receive a localized NotEmpty outcome. This report proposes a separate bounded capability; it does not claim implementation or a host-qualified result.

## Proposed observed-tree contract

Add a distinct opaque TreeRevision in the existing filesystem Workspace owner. It retains the exact selected directory identity, every ancestor identity and a complete bounded relative tree listing with entry kinds/stamps. A schema-authored total entry bound (proposed4,096 across the whole operation, not4,096 per directory), bounded explicit stack and existing256 UTF16 physical path limit prevent unbounded recursion. No source-content copy is needed for directory movement; no symlink/reparse traversal is permitted. Binary/large-file descendants may be moved without decoding their contents, provided the contract is identity/metadata observation rather than a content snapshot.

Capture progresses and cancels before publication. Destination is explicitly absent. Before publication revalidate complete retained listings/identities and every projected descendant destination path. Persist a bounded command intent before the actual no-replace move and its outcome afterwards. The same command id binds actual source/destination and final observed outcome. A postmove mismatch or new child is a typed conflict with the actual moved object retained and an inspectable outcome path, without rollback, unlinking or claiming filesystem CAS. Late cancellation settles the actual publication.

## Required qualification before enabling

No-replace protects an occupied destination; it does not atomically assert the source revision or freeze descendants against a noncooperating writer. A tree can change after the last observation. The outcome therefore must distinguish a verified observed-tree move from an actual moved tree that raced, and retain all bytes in both cases. A recovery location must be usable through the actual read-only recovery/inspection API rather than merely printed.

Path growth is an additional boundary: a newly created descendant between the last check and a move can make a longer destination prefix exceed256 even if every observed path was valid. A non-increasing destination prefix avoids introducing this growth for existing descendants. Longer-prefix rename and event-history retention require an explicitly qualified path/custody contract; do not silently claim the preflight snapshot is an atomic guarantee. If that contract cannot be established for a platform, refuse that operation before effects rather than recursively copy or relax the path limit. The current empty-directory capability must also classify a raced newly nonempty directory honestly.

## Neutral and actual gates

Shared cases should cover4,096 versus4,097 total entries, depth/path256 edges, Unicode prefixes, symlink/reparse children, missing/replaced ancestors, changed child stamps/listings, occupied destination, new child at final publication, cancellation at every durable cut, committed late cancellation and retained partial outcomes. Independent SQLite evaluates the bounded roster/path projection and Node filesystem reference tests compare isolated actual no-replace move/retained tree contents. Native kernel adapter tests must remain separate from those oracles. Installed PTY tests need bounded progress, cancel, conflict/recovery inspection, and complete final roster/byte checks.

## Decision

Keep the current empty-only UI truthful while qualifying this separate tree capability. There is no recursive delete/copy fallback and no unbounded traversal in the current feature. Completion of the full guided nonempty-folder workflow remains an explicit pending scope.

## Descendant Document Settlement

The current empty-directory UI protects exact paths. Enabling nonempty trees requires a segment-aware same-or-descendant relation for both source and destination: `a` contains `a/b`, never `ab/b`. Dirty descendant buffers refuse command issuance. Live jobs retain prefix protections through actual settlement even when cancellation is requested. Successful directory rename remaps every clean open descendant to the committed destination; its old path-bound revision cannot authorize save, so the buffer remains readonly until an actual reload issues the new revision. Retention must close/remap clean descendants only after a committed outcome. Ordinary precommit errors retain their original paths. Late-commit outcomes remain authoritative.

A retained nonempty directory also needs an authenticated recovery browse/load or restore route. A displayed history directory path alone is insufficient: current load_recovery is a document port and the ordinary Files browser excludes history. The backend capability must authenticate the retained root and bounded descendant roster before UI navigation is enabled. These are design requirements, not implemented or tested behavior.
