# UI Protocol and Retained-Command Ownership Audit

## Scope and method

This is a read-only audit of the two ownership boundaries in this ticket. Paths and line numbers below describe the current working tree. Searches used `rg`; no source, configuration, Git, worktree, or Cargo operation was performed.

The two findings are independent in implementation, but have the same cause: a renderer-facing or plugin-runtime protocol was placed under an unrelated framework owner. The repairs should be atomic moves with direct consumer updates; do not leave re-export or schema-reference compatibility shims.

## 1. TypeScript UI-scene protocol is incorrectly owned by `mesh`

### Finding

`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts` is a 896-line UI scene/host protocol. It has no mesh domain types, imports, or behaviour. Its public types describe all renderer surfaces, host dispatch, menus, and World3D scene-lane reassembly. The native UI equivalent already belongs to `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` (2,162 lines).

Move the *entire* TypeScript module to a new sibling source file:

`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`

This is the clean first boundary. Splitting the file during this move would manufacture module edges between the React target, menu policy, World3D lane decoding, and individual scene shapes. There is no TypeScript source presently under `🖱️ui/🎬️scene`, so this is a source relocation, not a collision with an existing barrel.

| Current range | Relocated UI ownership | Reason |
| --- | --- | --- |
| `🟦️.ts:10-20` | Canvas2D scene | Renderer scene payload. |
| `:21-265` | `UiMenuRef`, `ContextMenuItemSpec`, category constants, `organizeContextMenu` | UI menu protocol and renderer-neutral menu organization. |
| `:268-495` | World3D scene, compute-status decoding, lane table, lane reassembly | UI scene transport and host decoding. |
| `:496-691` | Node-graph and text-editor scenes plus action maps | Renderer scene payloads and dispatch identifiers. |
| `:702-840` | Table, Paint2D, icon, VFS, tiled-map, board, ink, graph-timeline, block-list, diff, and event-feed scenes | Renderer scene payloads. |
| `:841-896` | external-slot node, `ComponentKind`, `UiComponentSceneNode`, host props | The scene host’s discriminated UI contract. |

The menu portion is still UI-owned. The native contract supersedes the old `UiMenuRef` name with `MenuRef` at `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:1550-1559`, and the native WGPU target has an intentional organizer twin at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊wgpu/🧩component/🦀️.rs:282-365`. Neither fact makes React depend on the WGPU target. A later protocol unification may put the menu reference under `🖱️ui/🧬️contract/🎬️action`, but it must not be folded into this ownership move.

### Imports, export, and cycle assessment

The module has exactly three imports, all type-only:

* `@semio-tech/assets` (`IconName`) at `🔺️mesh/🟦️.ts:4`.
* `🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts` (`LocalizedLabel`) at `:5`.
* `🛂️manifest/🟦️.ts` (`ActionDescriptor`, `PluginContextMenuRequest`) at `:6`.

After relocation their paths become `../../🛂️manifest/...`. They are erased from emitted JavaScript; there is no runtime import or dependency cycle. The runtime functions in the file only use local data and JavaScript standard APIs.

There is one public source export to update:

* Replace `export * from "../../🔨️modules/🔺️mesh/🟦️.ts";` at `🧰️framework/📦️packages/🟦️typescript/🟦️.ts:17` with a direct export from `../../🔨️modules/🖱️ui/🎬️scene/🟦️.ts`.

No TypeScript source imports the old `🔺️mesh/🟦️.ts` path directly. First-party consumers import `@semio-tech/framework`, so their source import strings remain correct after the package-barrel change. Delete the old file when the barrel points at the UI source; do not keep a `mesh` re-export.

The following counts are direct symbol occurrences in TypeScript/TSX consumers, excluding the defining file. They identify the code that needs compilation and focused behavioural verification, not import-path edits.

| Protocol surface | Consumers | Principal runtime consumers |
| --- | ---: | --- |
| `UiComponentSceneNode` | 18 | React target, scene stories, interpreter, three plugin window kits |
| `ComponentSceneHostProps` | 19 | React target and all scene hosts |
| `ContextMenuItemSpec` | 11 | React/World3D/Shell hosts |
| `organizeContextMenu` | 5 | React target, ShellHost, ShellHelpers, I18n, framework dock-layout test |
| `World3dScene` | 7 | World3D host and stories |
| `WORLD3D_SCENE_LANES` | 3 | OS renderer Interpreter and lane test |
| `world3dSceneFromLanes` | 2 | Interpreter and `surface-scene-lanes` test |
| `world3dComputeStatusV1` | 1 | World3D host |
| `NodeGraphScene` / `TextEditorScene` / `TableScene` | 4 / 5 / 6 | React target and their hosts/stories |
| `nodeGraphActions` / `textEditorActions` / `inkCanvasActions` | 2 / 2 / 3 | React target and component hosts |

The remaining scene payloads have one to three consumers each: Canvas2D (3), Paint2D (3), VFS (1), tiled map (2), board (2), icon render (1), ink canvas (2), graph timeline (3), block list (2), diff (1), event feed (1), and external slot (1). This confirms the module is a shared UI protocol rather than a mesh implementation.

### Test boundary and bounded repair

Do not relocate consumer-owned tests simply because the source moves. Retain and run these from their current owners:

* `🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts` exercises the public package export of `organizeContextMenu`.
* `🧰️framework/🛍️products/💻️os/🎯️targets/🖥️renderer/🔨️modules/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx` exercises lane table and reassembly through the public package.
* `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs` is native target parity coverage and remains under UI.

The implementation change is bounded to the moved TypeScript file, the package barrel, and import-relative-path rewrites in that file. Validation should typecheck the framework package and run the named Bun/Nx tests; no Rust/Cargo validation is required for this TypeScript move.

## 2. Retained-command schema is UI-misowned and inconsistent with its runtime owner

### Finding and first-party owner

`🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json` owns a generic retained-command vocabulary even though it explicitly names OS plugin runtime concepts: `ArtifactToolPublicationLane` and `InteractiveJobClassification` (`:7-35`). The first-party owner is:

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🔣️.json`

Its existing `$id` is `https://semio.tech/schema/os/plugin/retained-command/component.json` (`:3`), it already owns retained-command checkpoints and `ScalarConfigCohortV1` (`:528-639`), and its tests perform real schema and owner-source checks. It is also where the plugin runtime declares `ArtifactToolPublicationLane` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12785-12802`.

Move the full retained-command group, not only the three externally referenced names:

* 19 internal definitions at `🖱️ui/🧬️schema/🔣️.json:6-685`: lane, disposition, execution, byte/step/class budgets, publication contract, boundary case, oracle, three route forms and union, corpus/declared limits, cohort factory/route/app/census.
* Five public definitions at `:686-786`: `RetainedCommandLimits`, `RetainedCommandRoute`, `RetainedCommandRoutes`, `RetainedCommandCohort`, and `RetainedCommandRoutesDocument`.

Place these in the OS-plugin schema’s `$defs`, rewrite their internal references there, and remove the group from the UI schema. Change every consumer reference directly to `https://semio.tech/schema/os/plugin/retained-command/component.json#/$defs/...`; do not retain UI-schema aliases.

### Reachable schema references

There are seven first-party JSON schema files with a live retained-command reference. The twelve total references to the UI schema include five unrelated UI definitions; they are out of scope.

| Consumer schema | Current line(s) | Referenced public definition |
| --- | --- | --- |
| `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` | 49 | `RetainedCommandRoutesDocument` |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` | 498 | `RetainedCommandRoutes` |
| `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️schema/🔣️.json` | 30 | `RetainedCommandRoutes` |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` | 1569 | `RetainedCommandRoutes` |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` | 27, 102 | `RetainedCommandLimits`, `RetainedCommandRoutes` |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` | 103, 432 | `RetainedCommandLimits`, `RetainedCommandRoutes` |
| `✏️s/🔌️plugins/🪐️space/🧬️schema/🔣️.json` | 72, 428 | `RetainedCommandLimits`, `RetainedCommandRoutes` |

The Space Rust package script is a live downstream compiler: `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts:136,489-499` compiles the three retained-limit contracts. Include it in post-move Bun/Nx validation.

`RetainedCommandCohort` and standalone `RetainedCommandRoute` have no external first-party `$ref` today. They are schema helpers, not currently reachable application contracts; move them with their group for coherent ownership, but do not claim an existing runtime consumer. The VCS, Wires, and imperative Rust unit tests load route fixtures with `include_str`; those fixtures are test data until their schemas are compiled by a test.

### Concrete stale lane, case, and table assumptions

This is more than placement:

* The actual runtime enum has ten lanes: `HostOnly`, `Artifact`, `Config`, `Draft`, `Presence`, `Transient`, `WindowConfig`, `WindowTransient`, `Child`, and `Interaction` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12785-12802`). The UI generic schema claims exhaustiveness but permits only eight lowercase/kebab values, omitting both window lanes (`🖱️ui/🧬️schema/🔣️.json:6-17`).
* The existing retained-command owner has the correct PascalCase contract style but repeats the same eight-lane omission in `ScalarConfigRoute` (`🧵️retained-command/🧬️schema/🔣️.json:650-690`). Moving bytes without consolidating this vocabulary would preserve the defect under a better directory.
* The current FEM3D app-specific retained-limits schema and fixture are the authoritative in-tree concrete precedent: `.../🎮️commands/🧬️schema/🚧️retained-limits/🔣️.json:88-157` and `.../🧫️fixtures/🚧️retained-command-limits/🔣️.json:51-157` use `Migrated`, `Artifact`, `HostOnly`, and `WindowConfig`. Its local Ajv fixture validation succeeds. Keep this app-specific contract under its editor command owner; do not put it back in a document schema.
* The Wires retained-command fixture uses `window-transient` at lines 46, 54, and 63, while its schema extends the generic `RetainedCommandRoutes` at `.../🎮️commands/🧬️schema/🔣️.json:30`. A strict Ajv compilation of the shared definition rejects its route rows 6-8 because `window-transient` is absent from the shared enum. This is a live schema-validity failure, not merely an outdated comment.
* Wires also has an app-local table cardinality conflict: its schema requires exactly nine route rows (`:34-38`), but the fixture contains ten rows. VCS has the same class of independent local issue: `VcsRetainedCommandRoutes` sets a minimum of ten rows while `.../🧫️fixtures/🛣️retained-command-routes.json` has nine. These cardinalities should be corrected from the actual owner/source census when the application fixtures are canonicalized; they are not caused by the move.
* `InteractiveJobClassification` serializes in camelCase (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:735-748`), while retained-command proof fixtures and their owner schema use Rust-variant PascalCase. The UI schema’s comment claims that lower kebab values are one-for-one runtime variants and adds `fail-closed`; both claims are false. `fail-closed` is a decision/veto outcome, not a sixth `InteractiveJobClassification` variant.

### Schema-first repair plan

1. Consolidate the retained-command grammar under the existing OS-plugin schema. Define one canonical retained-command artifact vocabulary in PascalCase, matching the owner’s existing checked proof contract and FEM3D fixture: all ten lane names above; `Unclassified`, `Migrated`, `BatchOnlyPendingRewrite`, `ForbiddenFromUi`, and `Deleted` for classification. Model a fail-closed decision as a separate admission/outcome field rather than adding it to classification.
2. Replace the duplicate eight-lane `ScalarConfigRoute` vocabulary with the new shared owner definition or a local structural restriction of it. This must make `WindowConfig` and `WindowTransient` valid in both forms, while retaining each app’s own `const` and cardinality requirements.
3. Relocate the 24 definitions from the UI schema as one schema-first change, rewrite their internal references, update the seven consumer `$ref`s above, and remove every retained-command definition from the UI schema. Do not add a forwarding `$ref` at the old location.
4. In the same atomic sweep, canonicalize app-specific route documents, fixture tokens, local enums/consts, and their Rust test expectation maps from lowercase/kebab to the PascalCase owner vocabulary. This includes VCS and Wires; their current literals would otherwise fail the repaired owner schema. Reconcile the Wires and VCS row limits with their actual fixture/source inventories instead of guessing a lane-independent number.
5. Extend the existing OS retained-command Bun test cluster with an Ajv test that compiles the owner schema and validates each of the seven referenced document definitions and representative fixtures, including `WindowConfig` and `WindowTransient`. Retain the existing FEM3D window-config contract test. Run the Space package’s schema compiler through its Bun/Nx script. No Cargo step is needed for this JSON/TypeScript ownership repair.

### Validation performed in this audit

* Ajv validated the FEM3D retained-limits fixture against its app-specific schema successfully.
* Strict Ajv, configured with the repository’s two custom keywords and `int64` format, rejected Wires route rows 6-8 against the current shared `RetainedCommandRoutes` definition. Each rejects `window-transient` against the eight-item shared lane enum.
* The same strict compilation exposed the independent VCS minimum-row mismatch. It was recorded as application fixture/schema drift, not attributed to the shared vocabulary issue.

