# Selected Resource and Dependency Frontier

## Captured Source

Read-only examination while Root's native10 capture remained active. No duplicate selected closure, initializer or frontend was launched. No implementation or owning project document was changed.

| Input | SHA-256 |
| --- | --- |
| Build graph inference | `a0a2cefdadc4fed3b216db03611b59fb316295122f96e469422cd2d64c378a7a` |
| Closed owner admission | `8ebf1192aa019a6f7105d1616fe8c684d28278053e725b5823811b30c2522d26` |
| Build project input inference | `207991f3747d3fc179574ca908cc9f779f506f661526fb3e2dda3ec85f3896b4` |
| Canonical owner registry | `30bc07fd79d85284727902b0d0447969aed128458a991be0e9dd957d06776491` |

## Reachable Overreach

`discoverBuildOwnerAdmissionFactsV1` defines `addFile` to add every physical file owner to `dependencies`. This conflates observed ownership with an actual project prerequisite. Native command inputs include globally authored compiler/toolchain resources, including the workspace `Cargo.toml`. That resource's canonical owner is `workspace`, root `.`.

The portable fixed point correctly admits the owner of every observed file. It also propagates each caret named input to every reported `dependencies` entry. A native target's normalized inputs include `^nativeSources` or `^nativeTestSources`. Therefore a compiler resource observation can propagate a native source group to the workspace owner without an actual native dependency declaration.

`cargoSourceInputs` returns undefined for a workspace manifest with no package/source entry. `inferProjectInputs` recognizes the presence of Cargo.toml as Cargo tooling, and its undefined native-source fallback is `{projectRoot}/**/*`. For workspace root `.` that is the whole workspace. This is a concrete reachable global admission path. It can also activate declared catalog providers after unrelated consumer owners are admitted.

This Source path is not proof that it dominated native10. Its existing progress log omits current owner, file count and cache hits. Root's Product600 capture ended at its original deadline, and native10 remained active. CPU and repeated open/read observations do not identify an exact owner or a Store wait.

## Closed Ownership Choice

Keep physical resources in `files`; their canonical owners are admitted and observed. Report only actual static compile imports, declared package dependencies, Cargo path dependencies, and catalog producer prerequisites in `dependencies`. Caret inputs and caret scheduled targets expand on those prerequisite edges. Do not filter by names, hide workspace resources, suppress actual IO/kernel prerequisites, drop type imports, or skip selected source facts.

The production import traversal already has exact parsed static/type/dynamic import identities and physical resolution. It is the appropriate place to add canonical dependency owners separately from generic resource admission. Package, native and catalog-producer edges are already explicitly authored by their respective discovery branches. Command imports and source input entries require their actual imported module edges; arbitrary glob members, manifests and compiler resource files must not manufacture edges merely through ownership.

The existing `BuildOwnerAdmissionFactsV1` separates `files` and `dependencies`, so resource/dependency separation needs no alias or compatibility interface. Its semantics and neutral law must state that separation explicitly. The independent Nx comparison must use explicit dependency edges rather than fabricate edges from resource paths.

One additional complete-closure defect is directly visible: `withNativePreparation` replaces native caret inputs with `{input:"nativeSources",projects:[actualCargoOwnerNames]}`. The admission expander silently ignores all non-string inputs, including these explicit canonical named-input references. Preserve those exact native owners and groups. The clean fact representation changes `dependencyInputs` from an undirected string list to a canonical owner-to-group-list map. Caret strings expand against actual prerequisite edges; explicit project references expand against their own canonical project list. Core admission consumes that exact map. Add its schema/law before implementation; no old list alias remains.

## Prepared Neutral Law

An isolated workspace fixture has selected native owner `native`, true native/compiler prerequisite `kernel`, compiler resource owner `workspace`, and an unrelated `specific` owner with a catalog provider. The selected command reads workspace Cargo.toml and declares `^nativeTestSources`. Expected closure admits native, kernel, and workspace; nativeTestSources expands only for kernel; unrelated Specific project and provider are untouched, even if absent or malformed. A static type import to another canonical owner remains a genuine edge and receives the requested source group. A `?raw` resource is physically admitted but does not create a compile prerequisite.

Use the existing mount fixture, schema validator plus independent Ajv, and independent Nx dependency/task graph boundary. A pure port-only test alone would validate the core but miss the production `addFile` defect, so the isolated fixture must exercise the production admission port as well. No law or test has been run for this prepared repair.

## Repeated Work Boundary

Each owner observation constructs fresh command source/native caches. Base inspection and selected/input inspection are separate. Fixed-point input revisions can repeat normalization and shared facade import traversal, and the final selected inventory reconstructs the full fixed point to verify original Source. The latter recheck is required. Reusing derived analyses within the same original observed construction can reduce repeated work only if every observation is replayed and original Source authority is checked; a global module result or an unobserved old graph cannot substitute for the recheck.

Address semantic overreach first. No timeout, quota, caller, cancellation, Store ownership or original progress law changes belong to this repair. Any later cache or performance change requires measured current owner/visit counters in a newly coordinated invocation.

A bounded local improvement needs no analysis reuse: `buildSourceOwnerV1` reconstructs and sorts the complete registry for every admitted file. The fixed-point construction can compile the captured immutable registry's deepest-root index once and use it for each file/provider lookup. This preserves the exact ownership decision and all original observations; it is local to one closure construction and cannot substitute cached authority for final Source rechecks.

## Owning Asset Batch

The already prepared IO control and SQLite model executor fixes are limited to four wrong authored `nx:run-commands` targets. Handcraft their executor as `repo:script` and exact execution metadata, preserving original command/cwd/cache/configuration. No linked generator declaration was found; their project JSON is the current direct authoring declaration. The separate cache-verification fixture constructor is not their generator.

Root retains the Source hold for active native10. Implementation and owning JSON repairs remain deferred until Root coordinates a terminal capture or explicit batch boundary.
