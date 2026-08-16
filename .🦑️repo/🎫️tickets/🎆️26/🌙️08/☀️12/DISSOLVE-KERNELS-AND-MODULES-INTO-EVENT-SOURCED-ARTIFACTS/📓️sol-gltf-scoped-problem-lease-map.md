# glTF Scoped Taxonomy Problem Lease Map

This is the exact grouped deterministic-census path-filter result for the glTF semantic owner. There are 84 errors. `bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf` currently filters these unregistered collection-path problems out and reports zero errors; the central report-scope defect is tracked separately. The groups below are ownership queues, not suppression rules or path exceptions.

`VERSION_ROOT` denotes `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0`. `ROOT` denotes `VERSION_ROOT/🪆️subsets/✳️any`.

## Schema Collection Assembly Lease — 8 Errors

| Exact path | Problem type | Count |
| --- | --- | ---: |
| `ROOT/🧬️schema/💡️inferences/{🔗️component.graphql,🛰️component.proto,🟦️component.ts,🦀️component.rs}` | `collection-authored-behavior` | 4 |
| `ROOT/🧬️schema/🧬️mutations/{🔗️component.graphql,🛰️component.proto,🟦️component.ts,🦀️component.rs}` | `collection-authored-behavior` | 4 |

The owner must move independently authored contracts/behavior into specific members or prove the leaves are generated mechanical assembly; this cannot be solved by an exception.

## Standards and Subset Manifest Lease — 5 Errors

| Exact path | Problem type | Count |
| --- | --- | ---: |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards` | `collection-manifest-missing` | 1 |
| `VERSION_ROOT` | `manifest-child-missing`, `member-component-leaf-missing` | 2 |
| `ROOT` | `manifest-child-missing`, `member-component-leaf-missing` | 2 |

## Artifact I/O Manifest Lease — 12 Errors

| Exact path | Problem type | Count |
| --- | --- | ---: |
| `ROOT/🚪️io/📤️export/🧵️serializers` | `collection-manifest-missing` | 1 |
| `ROOT/🚪️io/📤️export/🧵️serializers/🗿️artifacts` | `collection-manifest-missing`, `manifest-child-missing`, `member-component-leaf-missing` | 3 |
| `ROOT/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json` | `manifest-child-missing`, `member-component-leaf-missing` | 2 |
| `ROOT/🚪️io/📥️import/🧩️deserializers` | `collection-manifest-missing` | 1 |
| `ROOT/🚪️io/📥️import/🧩️deserializers/🗿️artifacts` | `collection-manifest-missing`, `manifest-child-missing`, `member-component-leaf-missing` | 3 |
| `ROOT/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json` | `manifest-child-missing`, `member-component-leaf-missing` | 2 |

## Mutation Collection Lease — 59 Errors

`MUTATION_ROOT` denotes `ROOT/🧬️schema/🧬️mutations`.

| Exact path | Problem type | Count |
| --- | --- | ---: |
| `MUTATION_ROOT` language leaves `{🔗️component.graphql,🛰️component.proto,🟦️component.ts,🦀️component.rs}` | `collection-authored-behavior` | 4 |
| `MUTATION_ROOT/{✏️set-accessor,✏️set-animation,✏️set-buffer,✏️set-material,✏️set-mesh,✏️set-node,✏️set-scene,➕️insert-accessor,➕️insert-animation,➕️insert-buffer,➕️insert-material,➕️insert-mesh,➕️insert-node,➕️insert-scene,➖️remove-accessor,➖️remove-animation,➖️remove-buffer,➖️remove-material,➖️remove-mesh,➖️remove-node,➖️remove-scene,🌳️reparent-node,🏷️set-asset,💾️binary,📄set-snapshot,📝️text,🔄️transform-node,🔗️bind-node-mesh,🔗️bind-primitive-material,🚫️no-mutation,🧭️planning}` | `manifest-child-missing` | 31 |
| `MUTATION_ROOT/{✏️set-accessor,✏️set-animation,✏️set-buffer,✏️set-material,✏️set-mesh,✏️set-node,✏️set-scene,➕️insert-accessor,➕️insert-animation,➕️insert-buffer,➕️insert-material,➕️insert-mesh,➕️insert-node,➕️insert-scene,➖️remove-accessor,➖️remove-animation,➖️remove-buffer,➖️remove-material,➖️remove-mesh,➖️remove-node,➖️remove-scene,🌳️reparent-node,🏷️set-asset,📄set-snapshot,🔄️transform-node,🔗️bind-node-mesh,🔗️bind-primitive-material,🚫️no-mutation}` | `member-component-leaf-missing` | 28 |

`💾️binary`, `📝️text`, and `🧭️planning` participate only in the 31 missing-child findings; they are not among the 28 missing immediate component-leaf findings.

## Lease Boundaries

- The schema collection assembly lease may not overlap the mutation collection lease's per-member implementations.
- The standards/subset manifest lease owns only ancestors and their local canonical manifests, not the metric, module, or I/O leaf implementations.
- The artifact I/O manifest lease owns its serializer/deserializer registry hierarchy and direct child contracts, not the independently owned `🚪️io/💡️inferences` codecs.
- The shared `🧾measure` contract promotion remains a separate semantic-module lease; it is a release blocker discovered from census semantics, not one of these mechanically reported 84 findings.

The totals reconcile exactly: 8 schema-assembly + 5 standards/subset + 12 artifact-I/O + 59 mutation = 84.
