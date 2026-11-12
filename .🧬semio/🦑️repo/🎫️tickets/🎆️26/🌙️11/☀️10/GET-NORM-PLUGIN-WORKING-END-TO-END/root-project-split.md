# Root project split (rootsplit)

Goal: move the per-domain targets of the repo-root `📋️project.json` (project `workspace`, 329 targets, all through one `📜️script.ts`) into the projects of their domains, split the root dispatch, update callers. Scripts and logs of this work: `rootsplit-*.py` and `🗑️generated/rootsplit-*` in this ticket folder.

## Result

- Root targets: 329 before, 56 after. 273 targets moved to 49 domain projects or removed because the domain project already owned the same lane.
- New projects (project-only, own `📜️script.ts`, no new bun package): `repo-storybook` (`🏗️build/📖️storybook`), `repo-schema-commands` (`🎛️commands/🧬️schema`). Existing projects extended: see table.
- Root verify dispatch `🎛️commands/✅️verify/🟦️.ts`: 40 domain lanes (about 470 lines) moved into domain-owned `OwnedVerifyScript` modules `<domain>/🧪️tests/🔬️owned-verify/🟦️.ts` (new in framework os, replication, value, pack, job, kernel, ui, infinite-dag, block-3d; added to the existing owned verify modules of wires, rewriting, jack). The root keeps only workspace-wide gates. `verify <lane>` stays routable through the declarative `metadata.semio.workspaceCommand` of the owning project, so lane CLI names did not change. `reset-document-ownership` was split into `wires-reset-document-ownership` and `rewriting-reset-document-ownership` (one lane per owning crate).
- Texts: the six refusal messages of the moved lanes live in domain catalogs (`verify-os`, `verify-ui`) and were deleted from the repo verify catalog.
- Callers rewritten: `workspace:<target>` to `<project>:<target>` in READMEs, `.storybook`, dashboard authority fixture and test, build-surfaces / dashboard-control fixtures, `🏗️build/🧭️invocation` (cpp and storybook mapping), `🏗️build/📥️inputs/🦀️resolution` fixture, verify gate lane (`schema-parsers-check`), cache-contract test; `.storybook` catalog-integrity test now reads the `repo-storybook` project; `dependsOn` entries across all project files. No CI workflow, `.agents` skill or `.devcontainer` command referenced a moved name (checked with word grep). The message sample in `🕸️graph/🧾️collect-errors` was left as is (synthetic text).

## Target to new project

Old root names are gone (no aliases). "how": `duplicate` = the domain project already owned the lane under this name; `variant` = same lane with an argument, now its own target next to the lane; `moved` = definition moved with its `cwd` set to the workspace root and the command pointing at the domain script.

| old root target | new project | new target |
|---|---|---|
| presentation-document-contract | @semio-tech/animate-presentation-rs | verify-presentation-document-contract |
| presentation-document-contract-native | @semio-tech/animate-presentation-rs | verify-presentation-document-contract-native |
| architect-window-ownership-native | @semio-tech/architect-program-rs | verify-architect-window-ownership-native |
| architect-window-ownership-oracle | @semio-tech/architect-program-rs | verify-architect-window-ownership-oracle |
| program-document-contract | @semio-tech/architect-program-rs | verify-program-document-contract |
| program-document-contract-native | @semio-tech/architect-program-rs | verify-program-document-contract-native |
| block3d-window-transient-native | @semio-tech/block-3d-rs | verify-block3d-window-transient-native |
| cad-document-contract | @semio-tech/cad-cad-rs | verify-cad-document-contract |
| cad-document-contract-native | @semio-tech/cad-cad-rs | verify-cad-document-contract-native |
| dag-document-contract-native | @semio-tech/dag-dag-rs | verify-dag-document-contract-native |
| dag-document-laws-native | @semio-tech/dag-dag-rs | verify-dag-document-contract-native-all |
| test-dag-document-contract | @semio-tech/dag-dag-rs | verify-dag-document-contract |
| drawing-canvas-window-ownership-native | @semio-tech/draw-drawing-rs | verify-drawing-canvas-window-ownership-native |
| drawing-canvas-window-ownership-oracle | @semio-tech/draw-drawing-rs | verify-drawing-canvas-window-ownership-oracle |
| fem3d-numerical-child-native | @semio-tech/fem-3d-rs | verify-fem3d-numerical-child-native |
| fem3d-numerical-child-source | @semio-tech/fem-3d-rs | verify-fem3d-numerical-child-source |
| flow-window-ownership-check | @semio-tech/flow-flow-rs | verify-flow-window-ownership-check |
| flow-window-ownership-native | @semio-tech/flow-flow-rs | verify-flow-window-ownership-native |
| flow-window-ownership-oracle | @semio-tech/flow-flow-rs | verify-flow-window-ownership-oracle |
| forms-document-contract | @semio-tech/forms-forms-rs | verify-forms-document-contract |
| forms-document-contract-native | @semio-tech/forms-forms-rs | verify-forms-document-contract-native |
| forms-document-laws-native | @semio-tech/forms-forms-rs | verify-forms-document-contract-native-all |
| framework-flow-physical-retirement | @semio-tech/framework-flow-flow-rs | verify-framework-flow-physical-retirement |
| framework-flow-physical-retirement-native | @semio-tech/framework-flow-flow-rs | verify-framework-flow-physical-retirement-native |
| dag-demo-ownership | @semio-tech/framework-infinite-dag-rs | verify-dag-demo-ownership |
| framework-job-physical-close | @semio-tech/framework-job-rs | verify-framework-job-physical-close |
| file-open-import | @semio-tech/framework-kernel | verify-file-open-import |
| file-open-import-native | @semio-tech/framework-kernel | verify-file-open-import-native |
| media-export-encoding | @semio-tech/framework-kernel | verify-media-export-encoding |
| media-export-encoding-native | @semio-tech/framework-kernel | verify-media-export-encoding-native |
| spawned-job-drive | @semio-tech/framework-kernel | verify-spawned-job-drive |
| spawned-job-drive-native | @semio-tech/framework-kernel | verify-spawned-job-drive-native |
| ui-dirty-scope | @semio-tech/framework-kernel | verify-ui-dirty-scope |
| ui-dirty-scope-native | @semio-tech/framework-kernel | verify-ui-dirty-scope-native |
| ephemeral-transfer-preparation-native | @semio-tech/framework-os | verify-ephemeral-transfer-preparation-native |
| framework-empty-state-contract | @semio-tech/framework-os | verify-framework-empty-state-contract |
| framework-empty-state-contract-native | @semio-tech/framework-os | verify-framework-empty-state-contract-native |
| host-ownership-native-test | @semio-tech/framework-os | verify-host-ownership-native |
| owned-document-closure | @semio-tech/framework-os | verify-owned-document-closure |
| owned-document-closure-native | @semio-tech/framework-os | verify-owned-document-closure-native |
| semio-envelope-identity | @semio-tech/framework-os | verify-semio-envelope-identity |
| semio-envelope-identity-native | @semio-tech/framework-os | verify-semio-envelope-identity-native |
| shared-value-canonical-json | @semio-tech/framework-os | verify-shared-value-canonical-json |
| snapshot-read-retirement | @semio-tech/framework-os | verify-snapshot-read-retirement |
| snapshot-read-retirement-native | @semio-tech/framework-os | verify-snapshot-read-retirement-native |
| store-reload-publication-native | @semio-tech/framework-os | verify-store-reload-publication-native |
| test-browser-actor-host-context | @semio-tech/framework-os | verify-browser-actor-host-context |
| test-browser-actor-host-context-worker | @semio-tech/framework-os | verify-browser-actor-host-context-worker |
| test-context-menu-view-context | @semio-tech/framework-os | verify-surface-view-context-menu |
| test-publication-retirement-authority | @semio-tech/framework-os | verify-publication-retirement-authority |
| test-publication-retirement-authority-oracle | @semio-tech/framework-os | verify-publication-retirement-authority-oracle |
| test-resolved-host-context | @semio-tech/framework-os | verify-resolved-host-context |
| test-retained-window-input | @semio-tech/framework-os | verify-retained-window-input |
| test-retained-window-input-oracle | @semio-tech/framework-os | verify-retained-window-input-oracle |
| test-shared-artifact-addressing | @semio-tech/framework-os | verify-shared-artifact-addressing |
| test-shared-artifact-addressing-oracle | @semio-tech/framework-os | verify-shared-artifact-addressing-oracle |
| test-surface-view-context | @semio-tech/framework-os | verify-surface-view-context |
| test-view-context-integer-carriers | @semio-tech/framework-os | verify-view-context-integer-carriers |
| test-view-context-integer-carriers-native | @semio-tech/framework-os | verify-view-context-integer-carriers-native |
| test-window-action-context | @semio-tech/framework-os | verify-window-action-context |
| test-window-action-context-oracle | @semio-tech/framework-os | verify-window-action-context-oracle |
| test-window-view-context | @semio-tech/framework-os | verify-window-view-context |
| test-window-view-context-native | @semio-tech/framework-os | verify-window-view-context-native |
| window-config-pack-identity | @semio-tech/framework-os | verify-window-config-pack-identity |
| window-config-pack-identity-native | @semio-tech/framework-os | verify-window-config-pack-identity-native |
| window-config-provisioning-lifecycle-oracle | @semio-tech/framework-os | verify-window-config-provisioning-lifecycle |
| window-config-retained-pack-load-native | @semio-tech/framework-os | verify-window-config-retained-pack-load-native |
| scale-fixture-check | @semio-tech/framework-os-dev | scale-fixture-check |
| framework-retained-pack-catalog-native | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership-catalog-native |
| framework-retained-pack-diagnostic | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership-diagnostic |
| framework-retained-pack-diagnostic-native | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership-diagnostic-native |
| framework-retained-pack-inflater | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership-inflater |
| framework-retained-pack-inflater-native | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership-inflater-native |
| framework-retained-pack-outer-native | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership-outer-native |
| framework-retained-pack-ownership | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership |
| framework-retained-pack-ownership-native | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership-native |
| framework-retained-pack-value | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership-value |
| framework-retained-pack-value-native | @semio-tech/framework-pack-rs | verify-framework-retained-pack-ownership-value-native |
| framework-paged-list-ownership | @semio-tech/framework-replication | verify-framework-paged-list-ownership |
| shared-map-delta | @semio-tech/framework-replication | verify-shared-map-delta |
| shared-map-delta-oracle | @semio-tech/framework-replication | verify-shared-map-delta-oracle |
| shared-value-clone | @semio-tech/framework-replication | verify-shared-value-clone |
| gis-map-window-ownership-native | @semio-tech/gis-gismap-rs | verify-gis-map-window-ownership-native |
| gis-map-window-ownership-oracle | @semio-tech/gis-gismap-rs | verify-gis-map-window-ownership-oracle |
| map-document-contract | @semio-tech/gis-gismap-rs | verify-map-document-contract |
| map-document-contract-native | @semio-tech/gis-gismap-rs | verify-map-document-contract-native |
| gis-terrain-window-config | @semio-tech/gis-gisterrain-rs | verify-gis-terrain-window-config |
| gis-terrain-window-config-native | @semio-tech/gis-gisterrain-rs | verify-gis-terrain-window-config-native |
| terrain-document-contract | @semio-tech/gis-gisterrain-rs | verify-terrain-document-contract |
| terrain-document-contract-native | @semio-tech/gis-gisterrain-rs | verify-terrain-document-contract-native |
| procedure-document-contract | @semio-tech/imperative-procedure-rs | verify-procedure-document-contract |
| procedure-document-contract-native | @semio-tech/imperative-procedure-rs | verify-procedure-document-contract-native |
| layout-document-contract | @semio-tech/layout-layout-rs | verify-layout-document-contract |
| layout-document-contract-native | @semio-tech/layout-layout-rs | verify-layout-document-contract-native |
| layout-window-ownership-native | @semio-tech/layout-layout-rs | verify-layout-window-ownership-native |
| layout-window-ownership-oracle | @semio-tech/layout-layout-rs | verify-layout-window-ownership-oracle |
| note-empty-config-ownership-native | @semio-tech/note-note-rs | verify-note-empty-config-ownership-native |
| note-empty-config-ownership-oracle | @semio-tech/note-note-rs | verify-note-empty-config-ownership-oracle |
| playbook-document-contract | @semio-tech/playbook-playbook-rs | verify-playbook-document-contract |
| generation2d-window-camera-ownership-native | @semio-tech/procedural-generation2d-rs | verify-generation2d-window-camera-ownership-native |
| generation2d-window-camera-ownership-oracle | @semio-tech/procedural-generation2d-rs | verify-generation2d-window-camera-ownership-oracle |
| generation3d-document-io | @semio-tech/procedural-generation3d-rs | verify-generation3d-document-io |
| generation3d-document-io-native | @semio-tech/procedural-generation3d-rs | verify-generation3d-document-io-native |
| generation3d-preview-window-transient | @semio-tech/procedural-generation3d-rs | verify-generation3d-preview-window-transient |
| generation3d-preview-window-transient-native | @semio-tech/procedural-generation3d-rs | verify-generation3d-preview-window-transient-native |
| reset-document-ownership | @semio-tech/reasoning-wires-rs | verify-wires-reset-document-ownership |
| test-wires-document-contract | @semio-tech/reasoning-wires-rs | verify-wires-document-contract |
| test-wires-document-contract-oracle | @semio-tech/reasoning-wires-rs | verify-wires-document-contract-oracle |
| test-wires-window-transient | @semio-tech/reasoning-wires-rs | verify-wires-window-transient |
| wires-pointer-move | @semio-tech/reasoning-wires-rs | verify-wires-pointer-move |
| remodel-window-ownership-native | @semio-tech/remodel-remodeling-rs | verify-remodel-window-ownership-native |
| remodel-window-ownership-oracle | @semio-tech/remodel-remodeling-rs | verify-remodel-window-ownership-oracle |
| build-canvas-font-assets | @semio-tech/repo-lib | build-canvas-font-assets |
| cpp | @semio-tech/repo-lib | cpp |
| cpp-build | @semio-tech/repo-lib | cpp-build |
| cpp-configure | @semio-tech/repo-lib | cpp-configure |
| cpp-custody | @semio-tech/repo-lib | cpp-custody |
| cpp-setup | @semio-tech/repo-lib | cpp-setup |
| cpp-test | @semio-tech/repo-lib | cpp-test |
| cpp-test-toolchain | @semio-tech/repo-lib | cpp-test-toolchain |
| test-build-tool-bazel | @semio-tech/repo-lib | test-build-tool-bazel |
| test-build-tool-bazel-bootstrap | @semio-tech/repo-lib | test-build-tool-bazel-bootstrap |
| test-dependency-readmission | @semio-tech/repo-lib | test-dependency-readmission |
| test-graph-memory-reservation | @semio-tech/repo-lib | test-graph-memory-reservation |
| test-runtime-build-producer | @semio-tech/repo-lib | test-runtime-build-producer |
| test-workspace-build | @semio-tech/repo-lib | test-workspace-build |
| test-contract | @semio-tech/repo-test-domain | test-contract |
| test-discover | @semio-tech/repo-test-domain | test-discover |
| test-doctor | @semio-tech/repo-test-domain | test-doctor |
| test-oracle | @semio-tech/repo-test-domain | test-oracle |
| test-parity | @semio-tech/repo-test-domain | test-parity |
| test-report | @semio-tech/repo-test-domain | test-report |
| test-subject | @semio-tech/repo-test-domain | test-subject |
| fem-assembly-physical-owners | @semio-tech/s-fem-composition-tests | verify-fem-assembly-physical-owners |
| fem-assembly-physical-owners-native | @semio-tech/s-fem-composition-tests | verify-fem-assembly-physical-owners-native |
| fem-mesh-preparation-owners | @semio-tech/s-fem-composition-tests | verify-fem-mesh-preparation-owners |
| fem-mesh-preparation-owners-native | @semio-tech/s-fem-composition-tests | verify-fem-mesh-preparation-owners-native |
| fem-numerical-page-owners | @semio-tech/s-fem-composition-tests | verify-fem-numerical-page-owners |
| fem-numerical-page-owners-native | @semio-tech/s-fem-composition-tests | verify-fem-numerical-page-owners-native |
| fem-pcg-publication-owners | @semio-tech/s-fem-composition-tests | verify-fem-pcg-publication-owners |
| fem-pcg-publication-owners-native | @semio-tech/s-fem-composition-tests | verify-fem-pcg-publication-owners-native |
| fem-scalar-owners-native | @semio-tech/s-fem-composition-tests | verify-fem-scalar-owners-native |
| fem2d-window-config-contract | @semio-tech/s-fem-composition-tests | verify-fem2d-window-config-contract |
| fem2d-window-config-contract-native | @semio-tech/s-fem-composition-tests | verify-fem2d-window-config-contract-native |
| fem3d-window-config-contract | @semio-tech/s-fem-composition-tests | verify-fem3d-window-config-contract |
| fem3d-window-config-contract-native | @semio-tech/s-fem-composition-tests | verify-fem3d-window-config-contract-native |
| forms-try-window-ownership-native | @semio-tech/s-forms-composition-tests | verify-forms-try-window-ownership-native |
| forms-try-window-ownership-oracle | @semio-tech/s-forms-composition-tests | verify-forms-try-window-ownership-oracle |
| norm-document-contract | @semio-tech/s-norm-composition-tests | verify-norm-document-contract |
| norm-results-window-ownership-native | @semio-tech/s-norm-composition-tests | verify-norm-results-window-ownership-native |
| norm-results-window-ownership-oracle | @semio-tech/s-norm-composition-tests | verify-norm-results-window-ownership-oracle |
| note-document-contract | @semio-tech/s-note-composition-tests | verify-note-document-contract |
| note-document-contract-native | @semio-tech/s-note-composition-tests | verify-note-document-contract-native |
| framework-ui-protocol-ownership | @semio-tech/s-space-composition-tests | verify-framework-ui-protocol-ownership |
| sequence-window-ownership-native | @semio-tech/sequence-sequence-rs | verify-sequence-window-ownership-native |
| sequence-window-ownership-oracle | @semio-tech/sequence-sequence-rs | verify-sequence-window-ownership-oracle |
| curation-document-contract | @semio-tech/sourcing-curation-rs | verify-curation-document-contract |
| curation-document-contract-native | @semio-tech/sourcing-curation-rs | verify-curation-document-contract-native |
| home-host-panel-owner-native | @semio-tech/space-home-rs | verify-home-host-panel-owner-native |
| home-host-panel-owner-oracle | @semio-tech/space-home-rs | verify-home-host-panel-owner-oracle |
| stdio-codec | @semio-tech/stdio-js | stdio-codec |
| stdio-cross-platform | @semio-tech/stdio-js | stdio-cross-platform |
| stdio-exhaustive | @semio-tech/stdio-js | stdio-exhaustive |
| stdio-fuzz | @semio-tech/stdio-js | stdio-fuzz |
| stdio-inference | @semio-tech/stdio-js | stdio-inference |
| stdio-long | @semio-tech/stdio-js | stdio-long |
| stdio-mutation-law | @semio-tech/stdio-js | stdio-mutation-law |
| stdio-quick | @semio-tech/stdio-js | stdio-quick |
| stdio-runtime | @semio-tech/stdio-js | stdio-runtime |
| stdio-schema-parity | @semio-tech/stdio-js | stdio-schema-parity |
| stdio-standards-coverage | @semio-tech/stdio-js | stdio-standards-coverage |
| stdio-document-contract | @semio-tech/stdio-semio-rs | verify-stdio-document-contract |
| stdio-document-contract-native | @semio-tech/stdio-semio-rs | verify-stdio-document-contract-native |
| jack-results-window-transient | @semio-tech/trinity-jack-rs | verify-jack-results-window-transient |
| test-jack-document-contract | @semio-tech/trinity-jack-rs | verify-jack-document-contract |
| test-jack-query-oracle | @semio-tech/trinity-jack-rs | verify-jack-query-ownership-oracle |
| test-jack-query-ownership | @semio-tech/trinity-jack-rs | verify-jack-query-ownership |
| rewriting-document-contract | @semio-tech/trinity-rewriting-rs | verify-rewriting-document-contract |
| rewriting-map-ownership | @semio-tech/trinity-rewriting-rs | verify-rewriting-map-ownership |
| rewriting-map-ownership-oracle | @semio-tech/trinity-rewriting-rs | verify-rewriting-map-ownership-oracle |
| test-rewriting-window-config | @semio-tech/trinity-rewriting-rs | verify-rewriting-window-config |
| test-rewriting-window-config-oracle | @semio-tech/trinity-rewriting-rs | verify-rewriting-window-config-oracle |
| framework-viewport-ownership | @semio-tech/ui-rs | verify-framework-viewport-ownership |
| framework-viewport-ownership-native | @semio-tech/ui-rs | verify-framework-viewport-ownership-native |
| framework-viewport-projection | @semio-tech/ui-rs | verify-framework-viewport-projection |
| framework-viewport-projection-native | @semio-tech/ui-rs | verify-framework-viewport-projection-native |
| node-graph-viewport-ownership | @semio-tech/ui-rs | verify-node-graph-viewport-ownership |
| node-graph-viewport-ownership-consumers-native | @semio-tech/ui-rs | verify-node-graph-viewport-ownership-native-consumers |
| node-graph-viewport-ownership-native | @semio-tech/ui-rs | verify-node-graph-viewport-ownership-native |
| node-graph-viewport-ownership-renderer-host-native | @semio-tech/ui-rs | verify-node-graph-viewport-ownership-native-renderer-host |
| framework-variant-field-casing | @semio-tech/value-ts | verify-framework-variant-field-casing |
| shared-dynamic-value | @semio-tech/value-ts | verify-shared-dynamic-value |
| writer-window-state-native | @semio-tech/writer-writer-rs | verify-writer-window-state-native |
| writer-window-state-oracle | @semio-tech/writer-writer-rs | verify-writer-window-state-oracle |
| runtime-fixture-graph | os-hub | runtime-fixture-graph |
| dev-mcp | repo-mcp | dev-mcp |
| dev-mcp-repo | repo-mcp | dev-mcp-repo |
| artifact-field-parity-enforce | repo-policy | artifact-field-parity-enforce |
| artifact-field-parity-report | repo-policy | artifact-field-parity-report |
| artifact-field-parity-test | repo-policy | artifact-field-parity-test |
| mutation-outcome-law | repo-policy | mutation-outcome-law |
| semantic-vocabulary | repo-policy | semantic-vocabulary |
| test-abstraction-ownership | repo-policy | test-abstraction-ownership |
| test-artifact-contract-ownership | repo-policy | test-artifact-contract-ownership |
| verify | repo-policy | verify |
| verify-abstraction-ownership | repo-policy | verify-abstraction-ownership |
| verify-artifact-contract-ownership | repo-policy | verify-artifact-contract-ownership |
| verify-build-owners | repo-policy | verify-build-owners |
| verify-build-owners-sync | repo-policy | verify-build-owners-sync |
| verify-build-parity | repo-policy | verify-build-parity |
| verify-canonical-architecture | repo-policy | verify-canonical-architecture |
| verify-dependencies-freeze | repo-policy | verify-dependencies-freeze |
| verify-dependencies-freeze-write-baseline | repo-policy | verify-dependencies-freeze-write-baseline |
| verify-dependencies-literal-external | repo-policy | verify-dependencies-literal-external |
| verify-dependencies-locks | repo-policy | verify-dependencies-locks |
| verify-gate | repo-policy | verify-gate |
| verify-generated-corruption | repo-policy | verify-generated-corruption |
| verify-interactivity | repo-policy | verify-interactivity |
| verify-layering | repo-policy | verify-layering |
| verify-package-purity | repo-policy | verify-package-purity |
| verify-policy-breach-artifact-analyzer | repo-policy | verify-policy-breach-artifact-analyzer |
| verify-policy-breach-artifact-builder | repo-policy | verify-policy-breach-artifact-builder |
| verify-policy-breach-artifact-builder-migrated | repo-policy | verify-policy-breach-artifact-builder-migrated |
| verify-policy-breach-artifact-composer | repo-policy | verify-policy-breach-artifact-composer |
| verify-policy-breach-artifact-decomposer | repo-policy | verify-policy-breach-artifact-decomposer |
| verify-policy-breach-codec-fidelity | repo-policy | verify-policy-breach-codec-fidelity |
| verify-policy-breach-contribution-target | repo-policy | verify-policy-breach-contribution-target |
| verify-policy-breach-io-serializer-matrix | repo-policy | verify-policy-breach-io-serializer-matrix |
| verify-policy-breach-io-terminality | repo-policy | verify-policy-breach-io-terminality |
| verify-policy-breach-plugin-dependency-parity | repo-policy | verify-policy-breach-plugin-dependency-parity |
| verify-policy-breach-schema-representation | repo-policy | verify-policy-breach-schema-representation |
| verify-policy-breach-standards-coverage | repo-policy | verify-policy-breach-standards-coverage |
| verify-rust-warnings | repo-policy | verify-rust-warnings |
| verify-taxonomy-enforce | repo-policy | verify-taxonomy-enforce |
| verify-taxonomy-implementation-enforce | repo-policy | verify-taxonomy-implementation-enforce |
| verify-taxonomy-implementation-report | repo-policy | verify-taxonomy-implementation-report |
| verify-taxonomy-report | repo-policy | verify-taxonomy-report |
| verify-typescript | repo-policy | verify-typescript |
| verify-typescript-project | repo-policy | verify-typescript-project |
| verify-zero-touch | repo-policy | verify-zero-touch |
| schema-audit | repo-schema-commands | schema-audit |
| schema-check | repo-schema-commands | schema-check |
| schema-compile | repo-schema-commands | schema-compile |
| schema-docs | repo-schema-commands | schema-docs |
| schema-entries | repo-schema-commands | schema-entries |
| schema-fixture-boundary | repo-schema-commands | schema-fixture-boundary |
| schema-generate | repo-schema-commands | schema-generate |
| schema-oracle | repo-schema-commands | schema-oracle |
| schema-parsers-apply | repo-schema-commands | schema-parsers-apply |
| schema-parsers-check | repo-schema-commands | schema-parsers-check |
| schema-parsers-report | repo-schema-commands | schema-parsers-report |
| schema-parsers-restate | repo-schema-commands | schema-parsers-restate |
| schema-test | repo-schema-commands | schema-test |
| schema-verify | repo-schema-commands | schema-verify |
| build-storybook | repo-storybook | build-storybook |
| dev-storybook | repo-storybook | dev-storybook |
| dev-storybook-animate | repo-storybook | dev-storybook-animate |
| dev-storybook-cad | repo-storybook | dev-storybook-cad |
| dev-storybook-framework | repo-storybook | dev-storybook-framework |
| dev-storybook-framework-hosts | repo-storybook | dev-storybook-framework-hosts |
| dev-storybook-framework-os | repo-storybook | dev-storybook-framework-os |
| dev-storybook-infinite | repo-storybook | dev-storybook-infinite |
| dev-storybook-puzzle | repo-storybook | dev-storybook-puzzle |
| dev-storybook-puzzle-2d | repo-storybook | dev-storybook-puzzle-2d |
| dev-storybook-puzzle-3d | repo-storybook | dev-storybook-puzzle-3d |
| dev-storybook-puzzle-5d | repo-storybook | dev-storybook-puzzle-5d |
| dev-storybook-styling | repo-storybook | dev-storybook-styling |
| dev-storybook-ui | repo-storybook | dev-storybook-ui |
| prepare-storybook | repo-storybook | prepare-storybook |
| setup-storybook | repo-storybook | setup-storybook |
| test-storybook | repo-storybook | test-storybook |
| test-storybook-catalog | repo-storybook | test-storybook-catalog |
| test-storybook-subjects | repo-storybook | test-storybook-subjects |

## Kept at the root (56), workspace-wide by nature

- `agents` (2): agents-check, agents-write
- `bench` (1): bench-plugins
- `clean` (7): clean-test, clean-coverage, clean-marker-only-folders, clean-taxonomy-inventory, clean-taxonomy-plan, clean-taxonomy-apply, clean-taxonomy-verify
- `components` (1): components
- `examples` (1): examples
- `format` (1): format
- `generate` (4): generate-config, generate-config-check, generate-taxonomy-census, generate-taxonomy-duplicates
- `lint` (2): lint, lint-repo
- `new` (1): new-taxonomy-mutation
- `os` (1): os
- `own script / aggregate` (19): build, deps-javascript, deps-python, deps-cargo, deps-go, deps-dotnet, deps-cpp, deps-browsers, deps-wasm, deps-trunk, deps-wasm-opt, deps-tools, ...
- `publish` (1): publish
- `release` (1): release
- `repo` (1): deps-build-tools
- `semio` (1): semio
- `setup` (5): setup, setup-git, setup-native, setup-devcontainer-start, setup-devcontainer-attach
- `test` (7): test, test-metrics, test-metrics-enforce, test-outcome-law-gate, test-canvas-native, test-canvas-react, test-command-entrypoint

## Verification status (honest)

Done and clean:
- All edited and new project files parse as JSON; every `project:target` entry in `dependsOn` that names a moved or new project resolves to an existing target (script check over all 1559 project files; the only unresolved entries are pre-existing generated `framework-os-dev:prepare-*/activate-*` names of other work).
- `tsc --strict` over every new `owned-verify` module, every edited domain `📜️script.ts`, the edited wires/rewriting/jack/block-3d/infinite-dag modules and the root verify module reports no error in any of those files (logs `🗑️generated/rootsplit-tsc-*.log`; the remaining errors printed there belong to unrelated modules in their closure).
- Word and pattern greps (`workspace:<old>`, `repo run <old>`, `--targets`, CI workflows, `.agents`, `.devcontainer`, READMEs) show no remaining reference to an old root target name.
- Relative-import closure of the root script (static count of reachable modules, `rootsplit-closure.py`): 2069 modules before; the commands still used by root targets reach 1990 after the split. The `verify` command alone still reaches 1015 modules because `🎛️commands/✅️verify/🌳️root/🟦️.ts` statically imports 35 self-test lanes.

Not completed (host capacity and graph rebuild, not a defect found in the change):
- `repo run <project>:<target>` for the moved lanes needs a rebuilt project graph, because about 50 project files changed. Run 1 (`@semio-tech/value-ts:verify-shared-dynamic-value`, log `rootsplit-run-_semio-tech_value-ts_verify-shared-dynamic-value.log`) waited about 27 min for capacity, then the resident service failed in the slice worker (`Slice worker exited with code 124` after "no progress for 11 min" at owners 25 of 25; service log reference 255ff541). Run 2 (`@semio-tech/framework-os:verify-resolved-host-context`) is still in a cold 628-owner graph read (760 s, 0 of 628) while this report is written. `rootsplit-validate.sh` keeps running in the background and then runs `@semio-tech/repo-lib:workspaces-check`, `repo:graph-check` and `@semio-tech/framework-process:test-command-owner` one after another, each into `🗑️generated/rootsplit-run-*.log`; the file `🗑️generated/rootsplit-validate.done` appears when it is finished. `repo show projects` reads the sealed (pre-change) graph and so cannot show the moved targets yet.
- Direct execution of a domain script outside `repo run` is refused ("Original parent Script process invocation required"), so the lanes cannot be exercised without the graph.

## Findings and what is left (needs a decision, not done)

1. The root owner's weight is not only the script. The root `namedInputs.default` is `{projectRoot}/**/*`, which for the root project is the whole repository, and 94 of the remaining 99 root targets have no `inputs` (so use it); 39 of them are cached policy gates. The graph owner for `workspace` will keep the whole-repository file rows until these gates declare narrow inputs or move to a project without the root `Cargo.toml` (kind `language:cargo`). Recommended next step: a project-only `repo-policy` project for the 43 `verify-*` gates with explicit per-gate inputs, then narrow `default` at the root to the root files.
2. The root script is still the human CLI router for all commands (`schema`, `stdio`, `dev`, `scale-fixture`, `cpp`, ...). Their root targets are gone and their domains own the targets, but removing the loaders from the root registry touches the registry schema, help snapshots in en and de, dashboard and launcher tests; this was not attempted without a test run.
3. `schema-tooling` and `ui-storybook` first lived under `🧰️framework/🔨️modules` and were moved into `🛍️products/🦑️repo` because the General/Specific boundary law (`🏃️process/🧭️routing/🎛️command/🧪️tests/🚧️boundary`) forbids a General project from naming product paths in commands or imports. The same law is the reason `video-render-export` moved from the kernel to the os domain.
4. While applying the first batch a helper truncated `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json` (open for write before read). It was restored from the git index (`git show :path`, identical to HEAD) and then received its new targets; any uncommitted edit by another agent to that one file before this change would have been lost. Nothing else was affected.
5. Moved targets that used the root named inputs `default` or `production` no longer hash the whole repository: they got narrow explicit inputs (schema sources, `.storybook`, ui tree, story files, library build tree). `cmakeSources` and `cmakeToolchain` moved with the cpp targets into `@semio-tech/repo-lib`.


## Part 2: policy-gates project and narrow root inputs

Source-first, static-verified only (JSON parse, `dependsOn` resolution, import paths of the new script exist, no root target without `inputs`). The `tsc` of the new script timed out under host load; its two imports resolve and it is a copy of the already type-checked pattern of `repo-storybook`. Runtime validation waits for the graph rebuild (see the status above).

- New project-only project `repo-policy` at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws` with its own `📜️script.ts` (registers only `verify` to `RootVerifyScript`, run against the workspace root). It has no `Cargo.toml`, so it no longer carries kind `language:cargo`.
- It owns the 43 workspace-wide gates, now with explicit per-gate inputs from five named inputs defined in the project: `policyEngine` (gate and policy library trees, taxonomy), `tsSources` (`**/*.{ts,tsx}`, package manifests, bun.lock), `rsSources` (`**/*.rs`, Cargo manifests and lock), `projectFiles` (project files, build owners, package manifests), `dependencyFiles` (lock and manifest files only) and `schemaFiles` (schema and standards trees). Mapping: taxonomy = engine+ts+rs; package-purity, typescript, canonical-architecture, layering, interactivity = engine+ts; rust-warnings = engine+rs; dependencies = engine+dependencyFiles; build-owners, zero-touch = engine+projectFiles; generated-corruption, semantic-vocabulary = engine+ts+rs; abstraction/artifact-contract/field-parity/policy-breach/mutation-outcome = engine+schemaFiles; aggregate, gate, build-parity = engine only. These are file-kind globs, not everything: a gate that really reads every source file of a kind still matches those files, but no longer binaries, assets, fixtures, generated stores or lock-free data.
- Root `namedInputs.default` was `{projectRoot}/**/*` (whole repository); it is now only `📋️project.json`, `📜️script.ts`, `package.json`, `nx.json`, `sharedGlobals` and the two test configs. New root named inputs: `rootFiles` and `rootCommands` (root files plus the library `🎛️commands` tree).
- All 56 remaining root targets now declare explicit `inputs` (none relies on `default`): `rootFiles` for everything that mutates or only orchestrates; `rootCommands` plus a narrow tree for the cached ones (taxonomy census and duplicates: project files and taxonomy; metrics: test sources; canvas: the canvas module; examples: the examples trees; outcome-law gate: schema trees). The human-CLI registrations in the root registry are unchanged.
- Callers: `workspace:<gate>` rewritten to `repo-policy:<gate>` in 14 files (READMEs, dashboard registry tests and fixtures, build-owner registry law, interactivity all-app lane and its Python oracle fixture, source-roster and source-census fixtures). No test reads these targets from the root project file by name.
- Root now has 56 targets; `repo-policy` has 43. Table of the 43 gates (old root name = new name, project `repo-policy`): mutation-outcome-law, semantic-vocabulary, verify, verify-gate, verify-build-parity, verify-rust-warnings, verify-taxonomy-report, verify-taxonomy-enforce, verify-taxonomy-implementation-report, verify-taxonomy-implementation-enforce, verify-package-purity, verify-generated-corruption, verify-build-owners, verify-zero-touch, verify-build-owners-sync, verify-typescript-project, verify-typescript, verify-interactivity, verify-dependencies-freeze, verify-dependencies-literal-external, verify-dependencies-locks, verify-dependencies-freeze-write-baseline, verify-abstraction-ownership, verify-policy-breach-artifact-builder, verify-policy-breach-artifact-decomposer, verify-policy-breach-schema-representation, verify-policy-breach-io-serializer-matrix, verify-policy-breach-io-terminality, verify-policy-breach-codec-fidelity, verify-policy-breach-standards-coverage, verify-policy-breach-artifact-analyzer, verify-policy-breach-artifact-composer, verify-policy-breach-artifact-builder-migrated, verify-policy-breach-plugin-dependency-parity, verify-policy-breach-contribution-target, test-abstraction-ownership, verify-canonical-architecture, verify-layering, verify-artifact-contract-ownership, test-artifact-contract-ownership, artifact-field-parity-test, artifact-field-parity-report, artifact-field-parity-enforce.


## Part 3: classification of the five validation runs (read-only, builds paused)

None of the runs reached target execution; no failure is caused by the move.
- `value-ts:verify-shared-dynamic-value`, rc=1: resident service failure, reference 255ff541, `Slice worker exited with code 124` after no progress for 11 min while reading the graph slice. Graph build.
- `framework-os:verify-resolved-host-context`, rc=1: "the service holds 3579 MB of its 3579 MB memory limit ... cancelled" during planning. Graph memory.
- `repo-lib:workspaces-check`, rc=1: same message at 3614 MB. Graph memory.
- `repo:graph-check`, rc=124 and `framework-process:test-command-owner`, rc=124: my own `timeout 2400` sent SIGTERM while the plan was still loading the graph. Time budget exhausted by graph load, not a target error.
No wrong cwd, missing input, broken `dependsOn` or script import was reported by any log (all stopped at "Planning 1 targets"). Source audit found nothing further to fix. Re-validate after the pause: first `repo-lib:workspaces-check`, then one moved lane per domain (value-ts, framework-os, repo-policy `verify-zero-touch`), with a timeout longer than the graph load (about 1 h).

## Part 4: re-validation after RESUME (in progress)

`rootsplit-validate2.sh` (detached, one `repo run` at a time, 3700 s timeout each) runs `repo-lib:workspaces-check`, `value-ts:verify-shared-dynamic-value`, `framework-os:verify-resolved-host-context`, `repo-policy:verify-zero-touch`. Logs: `🗑️generated/rootsplit-run2-*.log`; `rootsplit-validate2.done` marks the end. At the time of writing the first run was still queued for host capacity (about 9 min, memory-bound by other agents). Classify each log as in Part 3 when it ends: "service holds N MB of its limit", "Slice worker exited", or a timeout at "Planning" is graph build; a failure after the target starts is a real target failure.

