# FEM Emoji Repair

## Scope and Outcome

The bounded scope is `✏️s/🔌️plugins/🏗️fem`. The initial repository-context audit reported 425 findings: 121 duplicate sibling identities, 248 missing emojis, 20 presentation defects and 36 stacked identities. Every physical choice below was reviewed against its owner and moved explicitly without Git mutation or overwrite.

The final scoped read-only audit covers 1,251 files, 1,031 directories and 2,242 governed entries. It reports zero missing, generic, presentation, spacing, duplicate, multiple, reserved-name or Unicode-oracle findings.

## Standard and Subset Identities

Both FEM 2D and FEM 3D standards use the same complete logical-id to physical-directory roster:

| Logical subset | Physical directory |
| --- | --- |
| `*` | `🌐️any` |
| `analysis` | `📈️analysis` |
| `boundary` | `🛡️boundary` |
| `load` | `🏋️load` |
| `material` | `🧱️material` |
| `mesh` | `🕸️mesh` |

The combined subset content digest stayed `a303e424476ca04d414a857b8bb016117dc83ca02393fbe552e34d1253b33f14` across the twelve directory moves. All twelve subset oracle roots are `🔮️oracle`; the six editor/viewer option owners are `☑️options`.

The shared taxonomy now declares these two rosters as exact `🪆️subsets` owner overrides. Logical IDs in manifests and APIs remain unchanged. Root scaffold, inventory, coverage, manifest-bijection and self-dialect policies resolve through the exact owner instead of assuming one repository-wide `✳️` prefix.

## Commands and Mutation Owners

The two editor command owners retain the already fitting `🏋️add-area-load` and `🧱️add-material`. Repeated command identities were replaced with role-specific siblings: `🔗️add-combination`, `📋️add-load-case`, `📏️add-member-udl`, `📍️add-nodal-load`, `⚖️set-self-weight`, `➖️add-bar`, `⚪️add-node`, `📐️add-section`, `🛡️add-support`, plus `🌉️add-beam` and `🗺️add-region` in 2D and `🖼️add-frame` and `🧊️add-solid` in 3D. The combined command content digest stayed `b6aec5ae36c63f68bfd97bb7f463a2dc1257b23959e8f5ccafeb74ee5a06c5a2`.

Each mutation directory now has exactly one operation-specific identity. The common analysis/boundary/load/material owners use `🎛️update-analysis-settings`, `🛡️create-support`, `🔁️replace-support`, `🗑️delete-support`, `⚖️change-load-case-self-weight`, `➕️add-load`, `➖️remove-load`, `📋️create-load-case`, `🔗️create-combination`, `🗑️delete-load-case`, `✂️delete-combination`, `🌱️create-material`, `🔁️replace-material` and `🗑️delete-material`. Mesh operations use `📐️create-section`, `⚪️create-node`, `🧩️create-element`, `📏️replace-section`, `♻️replace-element`, `🕳️delete-node`, `✂️delete-section`, `🗑️delete-element`; 2D additionally uses `🗺️create-region`, `🔄️replace-region`, `🚫️delete-region`, while 3D uses `🧊️create-solid`, `🔄️replace-solid`, `🚫️delete-solid`.

The 2D operation-tree content digest stayed `155fd612...`; the 3D digest stayed `9564148b...`. These abbreviated values are retained from the before/after command output; no full digest is invented where the captured output was abbreviated.

## Schemas and Fixtures

All 50 mutation payload leaves were moved individually from the colliding descriptor identity `🔣️.schema.json` to `🧬️.schema.json`. The 2D aggregate schema digest stayed `29b07b2c...`; the 3D digest stayed `2e4a6164...`. A fresh descriptor check finds exactly 50 descriptors, each declaring `payloadSchema: "🧬️.schema.json"`, each resolving beside its descriptor, and every payload parsing as JSON. The separate retained-command-limits schema also uses `🧬️.schema.json` beside its `🔣️.json` document.

Action fixture directories mirror their semantic operations. Every action pair is `⏮️before.json` and `⏭️after.json`. Geometry fixture leaves are `📊️expected.metrics.json`, `🗿️expected.obj` and `🧊️expected.stl`.

The reviewed 2D geometry identities are `⬜️rect-unit-square`, `🏢️rect-floor-slab`, `🍽️rect-thin-plate`, `🪜️polygon-l-shape`, `🔺️polygon-triangle`, `🍩️region-one-hole`, `🕶️region-two-holes`, `🔬️scale-one-hole-1e-3`, `🔭️scale-one-hole-1e3`, `🌌️scale-one-hole-1e6`, `🧵️degenerate-hairline-thickness` and `🪶️degenerate-sliver-outline`. The reviewed 3D identities are `🧊️rect-unit-cube`, `🏠️rect-roof-slab`, `🍽️rect-thin-plate`, `🪜️polygon-l-shape`, `🔺️polygon-triangle`, `🍩️solid-one-hole`, `🕶️solid-two-holes`, `⬇️solid-one-hole-elevated-low`, `🎈️solid-one-hole-elevated-high`, `➖️solid-one-hole-elevated-negative`, the same three scale identities, `🧵️degenerate-hairline-height` and `🪶️degenerate-sliver-outline`.

The recorded fixture digests remained unchanged: common action fixtures `8db9e24f...` (2D) and `638f4d95...` (3D), 2D mesh fixtures `8f565f5f...`, and 3D mesh fixtures `dec142d7...`. The abbreviated values reflect the retained command output.

The generic `📄️rect-thin-plate` siblings identified by the final audit were hand-corrected to the plate-specific `🍽️rect-thin-plate` identity in both dimensions. The round-trip test owner is `🔄️round-trips-the-committed-document` in both dimensions.

## Regeneration Safety

Both geometry generators now keep logical recipe IDs separate from an explicit, complete semantic directory roster. They write only the three semantic geometry filenames and default to the owning `🕸️mesh/🧫️fixtures` directory. Their manifest coordinates point to that exact owner.

Both JSON carrier generators use an explicit 22-entry logical mutation to subset/fixture path table. The Rust sources are `🏭️generate.rs`, `📖️reader.rs` and `📚️lib.rs`, with matching exact Cargo paths. The TypeScript carrier command passes the standard's subset root, and both Rust generators refuse a kind without a registered physical directory.

Nx verification built both carrier projects in ticket-local Cargo target directories. Each `carrier-manifests` target succeeded and read all 22 fixture pairs. Nx geometry smoke generation succeeded for `rect-unit-square` and `rect-unit-cube` into ticket-local output. The only generated directories were `⬜️rect-unit-square` and `🧊️rect-unit-cube`; their leaves were exactly `📊️expected.metrics.json`, `🗿️expected.obj` and `🧊️expected.stl`, plus the one semantic JSON index at the output root.

## Corruption Cleanup

An unrelated malformed empty root `✏️s/🔌️plugins/🏗️fem/🗟️artifacts` was found during the final audit. It contained eight directories, zero files and zero symlinks, was absent from both tracked and untracked Git inventories, and had no active producer. Those eight empty directories were removed individually with `rmdir`; no content was deleted and there is nothing to recover.

## Verification Limits

This report establishes the FEM naming scope only. It does not claim the whole repository is clean. Shared path-statute tests are tracked separately because active Hub and OS producers changed two fixtures during the first focused run.

## Independent Semantic Follow-up

The structural-zero pass had left unrelated animals, flowers, colors, and other arbitrary objects in test identities. A second case-by-case pass reviewed all 50 scenarios and all 22 suite owners (including the two meaningful round-trip suites).

Twenty domain suites were renamed explicitly to analysis `📈️`, boundary `🛡️`, load `🏋️`, material `🧱️`, and mesh `🕸️`, both in their owning subsets and the duplicate whole-document suites. Thirty-six individual scenarios now identify the actual subject or change: steel/concrete/aluminium, wind, roof slab, uniform member load, combined load cases, roller support, locking and releasing constraints, section geometry, stiffening, torsion, rotation, bars, node positions, stair openings, and layered slabs. Fourteen already meaningful removal/count scenarios were preserved. All 50 oracle `directoryName` fields now resolve to those exact physical scenarios. Source, feature, and package mount references were reconciled without regenerating a shared barrel.

Fifty operation descriptors were checked against their owners. Thirty-nine retained stacked or mismatched metadata emojis; each was corrected to its existing operation's single identity. This does not change mutation IDs, schemas, or payload semantics.

The geometry corpus's point-node circles, two-hole glasses, scale microscopes/telescopes, hairline threads, and elevated-high balloon were separately reviewed and retained as direct geometry metaphors. Existing generator hand-authored recipe rosters remain the authority; none of the renamed suites or scenarios is emitted by an automatic emoji chooser.

### Immutable Carrier Preservation

The correct fixture selectors are `s.fem.2d` and `s.fem.3d`. Initial verification selected 34 and 37 fixtures and found respectively 12 and 15 metrics digest mismatches. Read-only byte reconstruction proved all 27 mismatches came exclusively from an earlier edit to the provenance string: removing the seven-byte emoji prefix from `measuredFrom: "🧊️expected.stl, re-imported and welded"` reproduced every original pinned SHA256 exactly. Only that historical carrier-citation byte string was restored. No numeric measurement, pinned digest, pinned byte count, mesh, or fixture manifest baseline was changed. The bare name inside the immutable metrics payload records the original measured carrier, not a live filesystem lookup.

### Follow-up Verification

The read-only naming audit reports 1,249 files, 1,013 directories, 2,242 governed entries and all eight counts zero. All 855 literal Rust embedded-file references and 522 Rust path mounts resolve; all 534 JSON files parse; all 50 catalog scenarios and 50 descriptor identities resolve consistently. A scan found no remaining unrelated animal, fruit, flower, or colored-marker names in this FEM scope.

Fixture-verifier reruns pass: `s.fem.2d` has 34 fixtures with zero file problems and `s.fem.3d` has 37 fixtures with zero file problems. The original pinned geometry baselines are preserved.

The first native Nx check stopped before compilation because a shared external STEP generator contract still cited seven moved fixture paths. That independent owner corrected those paths; the native retry has reached `cargo-nextest ... list --list-type binaries-only --message-format json --profile fundamental -p semio-s-plugin-fem` and is still running. This report does not yet claim native test success.
# Production Discovery and Independent Execution Follow-up

The production feature parser exposed 150 old bare scenario URI paths across the subset suites. Exact feature tables now use the already handpicked physical directory names. Production plans resolved 174 fixture URIs across 22 suites and the committed Python host executed all 252 scenarios: 252 passed, zero failed. All test-child names are registered through the central tests member vocabulary; non-test schema, fixture, and command identities received exact reviewed registrations as well. Subset identity validation uses the existing owner-scoped subset overrides.

The second native quick run passed the formerly broken shared STEP contract but exhausted the runner's 1,200,000 ms budget while waiting for the shared Cargo build-directory lock. It produced no native test result and is not reported as a pass.

## Actual Example Execution

The FEM TypeScript router previously printed `fem ts ok` without testing anything; its package test script also incorrectly targeted CAD. The existing FEM test target now invokes the shared Vitest runner and selects the four committed examples with empty selections forbidden. A red run found all four obsolete `🧪️artifact.ts` imports. Their exact imports now match the runner, and the same `bun nx run @semio-tech/fem-js:test` command executes four files and passes all four tests. Each test reads the current committed DSL or command asset. No concurrent oracle registration or native assertion was removed.

The final post-demo naming audit covers 1,250 files, 1,014 directories, and 2,244 governed entries with all eight categories zero. This supersedes the earlier 2,242-entry checkpoint. No FEM verification process remains running; the native timeout limitation above remains unchanged.
