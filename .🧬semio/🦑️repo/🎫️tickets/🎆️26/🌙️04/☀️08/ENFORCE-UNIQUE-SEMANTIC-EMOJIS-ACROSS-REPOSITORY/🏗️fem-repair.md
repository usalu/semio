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
