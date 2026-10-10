# Window Snapshot Custody Plan

## Current Original Owners

WindowConfigSnapshot stores an erased raw Arc payload and WindowTransientSnapshot adds a raw Arc around an original ErasedSnapshotRead. Both snapshots and authorities derive Clone. The Plugin root clones these snapshots into command/tool contexts at the current source lines 32860–32861, 32886, 34548–34549 and 34600. Refresh assigns a new snapshot over the previous original. Two retained-window native tests also clone a snapshot while comparing context identity.

Config previews currently call fold_leaf with snapshot_owner and keep displaced intermediate Arc roots in the original partition. They do not acquire an actual Store projection read. Store already exposes derived_snapshot_head, canonical derived snapshot operations and snapshot_read_derived. A complete change must use those original projection capabilities and retain displaced projection owners until caller-funded closure.

## Intended Authority Boundary

Carry the actual ErasedSnapshotRead directly in each owned snapshot, avoiding a second opaque shared allocation. Capture must acquire the original typed Store read and erase it by an ownership move preserving its exact lease. Duplicate access must acquire a new real lease from the existing issuer, with fallible admission rather than unconditional Clone; callers can borrow the same immutable snapshot when their lifetime permits. Refresh must transfer the previous original snapshot into retained child custody instead of dropping it during publication work.

Config preview should create a canonical Store-derived read from the exact captured authority and typed mutations, then retain original intermediate projection owners. A borrowed tool context should not fabricate a SnapshotRead from projected data or reconstruct the retirement catalog. Window identity Strings remain original owned metadata, closed through their typed controlled facets.

The owning read close must return the original lease under a locked registry generation check before releasing its local alias while the original registry slot remains held. The actual returned-read witness then closes through the first-party controlled registry owner, separately funding registry frame and backing. Rejection must retain the original read and lease unchanged. Root owns this Store boundary repair.

## Open Proof Obligations

The existing generic raw Arc lease retirement remains an unaccepted physical-release gate where Weak aliases can race header release. Therefore these window authorities currently refuse physical publication retirement; no outer supported marker or generation-completion predicate substitutes for original root custody. The new direct read boundary must be tested with actual allocator receipts, denied copy/capacity/release/depth, original pointer equality on refusal, interrupted refresh/capture/preview and exact final frame conservation.

Removing Clone changes the actual Plugin context construction sites, so integration must coordinate with the command agent's mounted authority handback and root's global lifecycle frontier. The original read capability and metadata must survive unsupported paths. No native window authority execution is claimed.

## Original Read Facet Milestone

The Store read-retirement module now uses one original capability cursor for typed SnapshotRead and ErasedSnapshotRead. It holds the exact read until the locked original return succeeds; Busy and other refusal restores the same read and emits zero progress. Successful return transfers the original witness to a caller-funded controlled child; the original child body and erased frame remain held until exact close demands are funded. The previous typed raw Arc lease path in this module was removed. Other generic raw lease paths and normal eager return/Drop remain separate open boundaries.

SnapshotRead::into_erased moves the original owner and lease without cloning or allocating. The new native law invokes that actual API and checks original pointer/index/generation, denied axes, interrupted closure, allocator birth/release equality and zero final cursor Drop. Those laws are authored but unexecuted while the native lib-test graph remains blocked. Root production gate is replaying against the current source.

A retirement-local neutral schema and fixture cover typed/erased variants, interruption points 0/1/2 and copy grants 1/7/4096. The portable original capability implementation is validated against an independent SQL custody ledger and strict Ajv2020 schema. Existing Window source/native commands include this proof; their existing launch entries are reused.

The first new proof attempt failed because this lane used a parent-relative fixture/import path with one extra directory traversal. Rust and TypeScript paths were corrected and their resolved targets were checked directly. The subsequent actual isolated Nx window-source-test completed: 3 tests, 438 assertions, strict TypeScript checking, owner exit 0. This portable milestone does not establish native allocator law execution or window authority integration.

## Direct original read and refresh continuation

The current native WindowConfigSnapshot and WindowTransientSnapshot carry the original issued ErasedSnapshotRead. Cold context duplication asks the original registry for a new lease; no raw Arc facade or synthesized SnapshotRead is constructed. Required fallible reverse ownership erasure preserves the original read/registry on type mismatch. Canonical derived preview reads use their sealed original Store registry.

Refresh retains the pending original read and the displaced original snapshot until an exact queue admission grant is available. Six portable fixture variants cover three copy grants and issuer rejection recovery, using Ajv and SQLite owner/address witnesses. The latest source gate executed four tests and 673 assertions with strict source checks successful. This includes window wrappers, prepared owners, original read closure and refresh.

A native prepared candidate law failed because its authority fields used an unsupported four-tuple; the fields now use actual supported nested two/three-tuples. The subsequent native replay ran the prepared law successfully, then failed the original-read terminal law because its test had omitted the original issuer pump. The native law now interleaves actual typed returned-root admission, verifies original pointers and zero allocator cost at that handoff, and closes the physical witness only afterward. The latest native replay reached both native Kernel stages successfully and is currently preparing Plugin tests; final native wrapper/refresh results are pending.

Window Config now retains an original returned-read child per typed partition and services it through its existing maintenance_retirements_demands/step methods before ordinary displaced work. The same child and preview owners are included in partition and retained-load cancellation closure. These source edits remain uncompiled by the full Plugin production gate at the time of this entry.

Native fixture migration continues under the actual four-currency APIs. No old two-axis adapter or fabricated snapshot authority is retained. The shared native window registry still owns opaque BTreeMap node backing and clones traversal keys without physical admission; its final map/node cleanup is not an accepted bounded allocation witness. The retained pack loader's value/catalog/segment child APIs also retain older two-axis closure seams. Neither gap is concealed by the new original-read facets.

## Exact native replay receipt

The latest Kernel exact gate completed both selected laws successfully. The receipt is 🗑️generated/window-mutation/exact-cargo-laws-ifb1zc/00/receipt.json. The selected laws cover original prepared candidate owners and original typed/erased read closure with the real issuer pump. The latter exercises denied currencies, interruptions, wrong-type owning erasure refusal, successful owning erasure, original pointers/index/generation, actual allocator receipts and final zero-cost cursor destruction.

The same invocation then failed Plugin lib-test compilation with 1,463 compiler diagnostics before executing window wrapper and refresh laws. 🗑️generated/window-native-error-index.json preserves the complete indexed errors. Narrow window fixture errors concerning module namespace, fallible owner catalog construction, displaced actor fields, four-grant refresh/close and physical read issuer pumping have been repaired afterward. These repairs are not a native passing receipt.

The Config returned-read service uses the existing maintenance_retirements_demands/step/terminal methods. Root confirmed the actual maintenance ladder already invokes these methods at stage 24 with the unchanged original four-grant budget. No duplicate mounted maintenance path was introduced.
