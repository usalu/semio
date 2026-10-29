# Native Artifact Layout Memoization Review

Read-only work-in-progress review, 2026-10-29. No compiler launches, queries, tests or Source edits by reviewer.

ArtifactLayout stores root anchor, copied boundary identities and per-directory kind/ancestor-ID observations in an instance. NewArtifactIndex copies bundle slice and constructs a new layout; codebase builder creates one layout per request and shares it across its folder/file producer pair. Both canonical and duplicated stream implementations allocate a layout if none supplied and refuse a supplied layout whose root differs. No new global layout cache found. Folder-kind and parent-ID mutable maps share a mutex; boundary reads are concurrent-safe because construction finishes before publication and boundary map is not subsequently modified. Parent-ID computation calls the private folderKind helper under the same held mutex, avoiding recursive locking.

Folder observations now use one Lstat and ReadDir per uncached ordinary directory; symlink directories and nonregular indicator entries are excluded. Subsequent invocation gets a new instance, so memoization alone does not preserve stale filesystem observations across requests. Existing context checks remain in stream walks and cancellable producer sends; the synchronous directory observation itself has no new asynchronous cancellation API. Do not claim hard interruption during os.ReadDir.

Early edge communicated to owner: boundary Root="." is not recognized for descendants by boundaryByPath’s normalized-prefix condition ("./" does not prefix "a/b"). Older GetBundleByPath appears to share this behavior, so this is not established as an introduced regression; require explicit root-boundary semantics before claiming arbitrary root parity. The implementation remains owner-in-progress and no final freeze verdict is asserted here.


## Current Constructor and Wire Review

Read current working Source after nearest-boundary changes. All production NewArtifactIndex constructors found pass an explicit root (codebase context ctx.RootDir; GraphQL c.rootDir; global fallback workspace root). Stream layouts are json:"-" in canonical model StreamOptions and duplicated client options, avoiding serialized mutex/layout leakage. No global layout memo found. Legacy convenience entrypoints construct a fresh instance; optimized paired streams explicitly share one invocation instance.

A concrete follow-on root case was sent to owner: boundaryByPath now recognizes Root=".", but ResolveParentIDFromPath still initializes the boundary ID only for exact equality or prefix root+"/", so a nested relative descendant can omit the fallback boundary ID. Require folder and file ID vectors, not only boundary lookup parity. Owner remains implementing; no final freeze verdict yet.


## Latest Boundary and Identity Delta

Reviewed actual ready implementation after owner’s correction. Root="." fallback now initializes the parent boundary ID in ResolveParentIDFromPath, resolving the concrete descendant omission. namespacePath converts absolute observations/boundaries inside the explicit anchor to relative namespace coordinates and rejects outside-root observations; outside-root boundaries are omitted. Nearest lookup and bundleRoots use the same normalized first-wins boundary selection. New tests exercise folder and file IDs for root fallback, absolute coordinates, nearest nesting, equal-root ties and prefix near-misses. URI hydration now reuses the resolved ID, avoiding a second global constructor lookup.

No new concrete blocker found statically in this delta. All mutable derived maps remain protected and scoped to the layout instance; no global memo introduced. Owner reports model and independent Python/Ajv gates green; final formatter/cancellation compilation remained owner-managed at notice time. Reviewer did not run them or claim fresh installed-native behavior.
