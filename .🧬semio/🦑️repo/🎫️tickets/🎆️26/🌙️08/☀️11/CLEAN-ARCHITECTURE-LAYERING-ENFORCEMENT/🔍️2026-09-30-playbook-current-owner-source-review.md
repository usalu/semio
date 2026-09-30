# Current Playbook Geometry Owner Source Review

Read-only WIP review on 2026-09-30. No edits/builds/tests. Session close_step implementation is explicitly still owned by the execution agent; its pending API is not a defect claimed here. Root is already repairing old prefixed handle parsing and importing string-array handles, so those unchanged findings are not repeated as outstanding work.

## Actual Instance Routing

Inspected `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs`. ModuleGeometryOwner at lines 371–391 constructs one actual Session and one SharedRegistry, registers concrete Math and BREP operators against that same Session, and supplies the registry and sibling geometry authority to temporary FlowHost. Typed retained work at lines 748–771 carries request.instance_operation_owner supplied at line 916. Instance render at lines 989–1001 downcasts the same owner and performs preview through it; ownerless preview fails rather than silently constructing a new default. These are real production paths, not test-only composition.

Preview tessellates evaluated handles before temporary host retirement (lines 449 and 466). Export publishes preview handle claims to the root before retiring the host's port (lines 530–531). That read/retire order preserves geometry while actual readers use it. Registry retirement runs before the root Session retirement so registered operators relinquish their Session clones; terminal-empty includes both. No stale ambient free-function geometry imports remain in the inspected module header.

## Repeated Rendering Currently Accumulates Geometry

`handle_import_solid` at lines 560–565 imports once to validate, then persists only format/data/tolerance while leaving the imported geometry's root claims live. `imported_geometry_handles` at lines 572–582 re-imports on every render and export. No owner-local source-keyed cache currently exists, so each successful replay can allocate new topology and claim new handles.

Preview lines 428–492 never replace the root Session's retained claim set with the current result union. Operators claim newly produced/intermediate handles on the root, and root tessellation adds selected handles. Closing the temporary host only releases its sibling port claims, preserving the root's ever-growing set. Repeated renders therefore retain prior geometry, imported geometry, and matching cached meshes rather than just the current preview/import union. Export prunes to preview handles before a new import, but that fresh import adds another claimed result.

Repair should reuse imported geometry by exact persisted source within the owner and replace root claims with the complete current preview/import union after all outputs are admitted. Changes/undo must invalidate source cache entries and release their claims. Retention should not prune current imported handles before using a cached result. Sent concrete producer/consumer coordinates to the root.

## Reopened Handle Equality Is Not A Geometry Oracle

New lifecycle test `🧪️tests/🔬️unit/🦀️.rs:341` asserts replayed handle strings equal first-session handles. Native Brep::mint in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🦀️.rs:601–606` hashes kind and Body PersistentLabel. Different prior operations advance those labels differently. First validation import followed by replay already differs from an untouched second Session; later export also evaluates a fixture in the second Session. Equal interchangeable geometry does not require equal derived handle strings across those operation histories.

Keep source-preservation and pre-import cross-session rejection assertions, and compare replayed geometry with an independent shape/export oracle rather than label equality. Presence-only old import tests remain narrow, while this new law intends the actual reopen behavior. This source mismatch was sent to root before native execution.

## Retained Command Scope Is Still One Synchronous Geometry Unit

ModuleGeometryCommandWork::step at lines 761–771 marks consumed, acquires the instance owner, evaluates/export/imports the whole model synchronously, and returns Complete. Its extent remains one work item. This establishes typed per-instance authority routing and prevents repeated execution, but does not add retained geometric progress or cancellation inside the media computation. Native tessellation/export/import costs and parameter payload cloning remain within that one callback. The current source must not be described as newly sliced media computation merely because the framework wraps the callback in a retained job. Sent this concrete scope limitation to the root; no runtime failure or test-green claim is made.
