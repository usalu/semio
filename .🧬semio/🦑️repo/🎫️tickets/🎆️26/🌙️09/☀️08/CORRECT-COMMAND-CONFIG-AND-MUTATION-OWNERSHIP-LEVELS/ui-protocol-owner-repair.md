# UI Protocol and Retained-Command Ownership Repair

## Outcome

The two audited ownership defects are repaired.

- The complete TypeScript UI scene protocol now lives at `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`. The framework TypeScript package exports that source directly, the manifest type import follows it, and the old `🔺️mesh/🟦️.ts` source is deleted. There is no old-path shim.
- The 24 generic retained-command definitions formerly owned by `framework.ui` now belong to the existing `os.plugin.retained-command` schema. All definition IDs are canonical PascalCase, all internal references are local `$defs` references, and all seven consumer schemas point directly at the OS/plugin owner.
- The canonical lane vocabulary is exactly `HostOnly`, `Artifact`, `Config`, `Draft`, `Presence`, `Transient`, `WindowConfig`, `WindowTransient`, `Child`, and `Interaction`.
- The canonical classification vocabulary is exactly `Unclassified`, `Migrated`, `BatchOnlyPendingRewrite`, `ForbiddenFromUi`, and `Deleted`. `FailClosed` is modeled separately by `RetainedCommandAdmissionOutcome`, and `RetainedCommandAdmission` is the union used by admission fields.
- The UI schema now exports only `UIDialogModalFixture`; it contains no retained-command definitions. The regenerated catalog resolves every relocated owner export and contains no retained-command export or old owner reference under `framework.ui`.

The manifest's distinct `interactiveJob` wire serialization remains camelCase (`migrated`). Retained-command route fields, fixtures, and native expectations use the canonical PascalCase proof vocabulary.

## Source and schema changes

The scene move preserved the public `@semio-tech/framework` API while correcting source ownership. Only the package barrel and the manifest's relative type import changed; first-party package consumers continue to import the same package API. Two DAG source-location comments and the World3D lane fixture source comment now identify the new owner.

The owner schema consolidates the relocated grammar with its existing scalar-config retained-command grammar. `ScalarConfigRoute` uses the shared lane and classification definitions, with its local classification restriction retained. The owner exports the 24 relocated identities plus `RetainedCommandAdmissionOutcome` and `RetainedCommandAdmission`.

The seven direct consumer schemas are VCS, Wires, Shooting, Remodel, Space Home, Space Index, and Space root. Every relevant duplicate `$ref` in those files now targets `https://semio.tech/schema/os/plugin/retained-command/component.json#/$defs/...`. VCS and Wires fixtures and Rust expectation maps were canonicalized with their schemas. The Imperative route fixture and its Rust oracle were canonicalized as a direct owner-schema consumer even though it has no document-schema `$ref`. Shooting, Remodel, and the three Space proof surfaces received the same fixture/schema/native expectation update.

Strict validation found three stale cardinality assumptions and they were reconciled against concrete owners:

| Surface | Concrete census | Repaired contract |
| --- | ---: | --- |
| VCS | 9 route rows | exact/minimum 9 |
| Wires | 10 route rows | exact 10 |
| Shooting | 38 declared route rows | routes 38, bounded 2, resumable/fail-closed 36; `every_command` expects its actual 36 rows before the framework-injected command |
| Remodel | 40 enum, retained-ID, publication-contract, and fixture rows | schema and native oracle require 40 |

The Remodel count was coordinated with the concurrent Remodel owner: its enum, `REMODELING_RETAINED_TOOL_IDS`, `REMODELING_PUBLICATION_CONTRACTS`, and fixture independently agree on 40. The retained `$ref`, PascalCase tokens, and count edits were kept separate from its concurrent window/config work.

## Durable validation

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🧬️schema-owner/🟦️.ts` is the permanent, language-neutral ownership oracle. It:

- compiles the OS/plugin owner with strict Ajv and independently validates the same documents with `jsonschema`;
- validates representative instances for all seven referenced consumer definitions plus the direct Imperative route fixture;
- rejects a lowercase hostile lane with both validators;
- proves the exact 10-lane and 5-classification sets and the separate `FailClosed` admission outcome;
- proves all 24 moved definitions resolve from the OS/plugin owner, none remain under UI, and the catalog has no old UI retained-command exports or references;
- proves the scene source exists only at its UI owner, the package barrel points there, and the manifest's relative type import resolves.

The permanent root route `workspace:framework-ui-protocol-ownership` invokes that durable module directly, typechecks it, runs the focused public-package `organizeContextMenu` tests, runs the renderer's long `world-3d paged scene carrier` tests, and invokes Space's existing `interactive-job-catalog-check`. It does not depend on this ticket directory. The ticket facade remains available only for the workspace's intermittently broken root Nx metadata. Both route registrations use `📜️script.ts`; launch order is `311.211`.

## Evidence

- `bun nx run workspace:framework-ui-protocol-ownership` passed end to end before the final identifier-only PascalCase catalog refinement.
- The ticket facade `bun ./📜️script.ts framework-ui-protocol-ownership` passed after that refinement: the schema oracle validated 8 fixtures with Ajv and `jsonschema`; the framework package reported 1 passed file and 2 passed focused tests; the long renderer suite reported 1 passed file and 12 passed focused tests; Space reported 23 catalog checks with zero stale descriptor rows. Space emitted only its existing harmless `int64` format warnings.
- The durable oracle was rerun after the final catalog regeneration and reported `[DEBUG] retained-command owner validated 8 fixtures with Ajv and jsonschema; lanes=10 classifications=5`.
- `bun nx run workspace:schema-generate --skip-nx-cache` regenerated the catalog with 3,086 scopes and 6,266 diagnostics. `bun ./📜️script.ts schema generate --check` then confirmed the generated catalog was current. A focused catalog inspection found `framework.ui` exporting only `UIDialogModalFixture`, all 24 relocated definitions plus both admission definitions resolved from `os.plugin.retained-command`, zero missing moved exports, and zero old UI retained-command references.
- The full repository command `bun ./📜️script.ts schema check --report <ticket>/🗑️generated/ui-protocol-schema-global-check.md` exited nonzero with 6,269 existing global findings: 1 cross-scope uncataloged, 22 non-draft-07 dialects, 13 unaddressable document IDs, 153 duplicate exports, 84 invalid exports, 5,580 incomplete exports, 65 fixture-defines-schema, 7 inconsistent module IDs, 35 missing schemas, 5 aggregate and 16 leaf mutation findings, 55 ineligible owners, 3 forbidden filenames, 226 unresolved references, and 4 ambiguous scopes. The affected UI/retained-command owners and seven named retained exports have no focused unresolved-reference, invalid-export, cross-scope, or owner-ineligible finding. The generated report is at `🗑️generated/ui-protocol-schema-global-check.md`.

Cargo was intentionally not run. The audit did not require Cargo for this JSON/TypeScript repair, and Cargo execution was serialized under the root agent during this work.

## Changed paths

Scene ownership:

- `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts` (deleted)
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` (added)
- `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️main/🟦️.ts`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🕸️main/🟦️.ts`

Schema owner, oracle, and generated catalog:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🧬️schema-owner/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`

Consumer schemas and fixtures:

- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🚧️retained-command-limits/🔣️.json`
- `✏️s/🔌️plugins/🪐️space/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧫️fixtures/🧫️retained-command-limits/🔣️.json`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json`

Native expectation and package validation surfaces:

- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🧪️tests/🔬️interactive-job-catalog/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`

Executable registration and ticket evidence:

- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/ui-protocol-schema-global-check.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/ui-protocol-owner-repair.md`
