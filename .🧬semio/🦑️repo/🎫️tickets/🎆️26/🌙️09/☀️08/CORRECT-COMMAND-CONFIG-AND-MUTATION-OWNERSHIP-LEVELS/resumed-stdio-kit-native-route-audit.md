# Resumed Stdio Kit and Native Route Audit

Read-only audit on 2026-09-12. Read the root, `✏️s`, and Stdio `AGENTS.md`
instructions. No Cargo, Bun, or Nx command was run and no production source was
changed. This report uses current files rather than the removed pre-2026-09-10
generated output.

The exact Kit schema root is
`/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema`.
Each abbreviated `…/` path in the findings is relative to that root unless it
explicitly starts with `✉️base`, `validation`, `.cargo`, or `.vscode`.

## Kit Status

The Rust parent, snapshot, and diff own the intended data shape:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🦀️.rs:92-103`
  uses Store `ArtifactChild` and `ArtifactLink` plus base `SemioTransform`.
- `…/🧬️schema/🔺️diff/🦀️.rs:31-68` uses six sparse, typed replacement
  slots; the two child collections are `values` wrappers.
- `…/🧬️schema/🧬️mutations/🦀️.rs:35-52` and `…/🔣️.json:6-187` agree on
  fifteen externally tagged mutation variants.

The TypeScript, JSON Schema, GraphQL, and Proto facets do not yet provide that
shape. The incomplete surfaces are concrete remaining work, not merely an
import cleanup.

| Facet | Current evidence | Required correction |
| --- | --- | --- |
| Parent TypeScript | `…/🧬️schema/🟦️.ts:2-20` locally redefines IO/Store types, makes `ArtifactLinkRef.target` a string, and makes design children `unknown[]`. | Import `ArtifactRef` from framework IO, `ArtifactChild` from Store child, `ArtifactLink`/`LinkPin` from Store link, and Kit domain records from snapshot. Export a strict `parseSemioKitArtifact`. |
| Snapshot TypeScript | `…/📸️snapshot/🟦️.ts:4-29` duplicates child/reference/geometry data, flattens a snapshot pin to `hash/size/mediaType`, and represents its link target as a string. | Retain only Kit catalog/design records locally; import the shared owners and base geometry. Export `parseSemioKitSnapshot`, calling Store parsing then the base Semio child policy. |
| Diff TypeScript | `…/🔺️diff/🟦️.ts:3-10` replaces typed list wrappers with arrays and reduces every child target to a string. | Match Rust's `{ values: … }` wrappers for types/designs/objects/models/representations and its optional child replacement shape. Export `parseSemioKitDiff` and `applySemioKitDiff`. |
| Parent/snapshot/diff JSON Schema | Parent `…/🔣️.json:15-42`, snapshot `…/📸️snapshot/🔣️.json:15-68`, and diff `…/🔺️diff/🔣️.json:7-29` admit broad arrays/objects instead of canonical references and closed nested records. | Reference framework IO, Store child/link/blob, base child/geometry, and typed Kit definitions; close all records and encode the sparse diff wrappers. |
| Parent/snapshot/diff GraphQL and Proto | Parent GraphQL `…/🔗️.graphql:4-9` and Proto `…/🛰️.proto:5-14`; snapshot GraphQL `…/📸️snapshot/🔗️.graphql:3-14` and Proto `…/📸️snapshot/🛰️.proto:5-24`; diff GraphQL `…/🔺️diff/🔗️.graphql:1-8` and Proto `…/🔺️diff/🛰️.proto:3-6` use `*Wire`, string targets, or touched booleans. | Expose the actual typed field values and shared child/link/geometry messages. A boolean-only diff cannot carry the replacement required by Rust. |
| Mutation TypeScript/transport | `…/🧬️mutations/🟦️.ts:9` imports `ArtifactRef` and `ArtifactLinkRef` from the broken snapshot facet. GraphQL `…/🧬️mutations/🔗️.graphql:1-6` and Proto `…/🛰️.proto:1-3` invent opaque envelopes. | Import `ArtifactRef` from IO and link pin from Store; retain Kit domain record imports from the corrected snapshot. Replace the GraphQL envelope with 15 typed alternatives and Proto with typed `oneof` payloads. |

The three aggregate TypeScript parsers and the diff applier named by the
existing contract test do not currently exist. The test dynamically obtains
them at `…/🧪️tests/🪪️document-contract/🟦️.ts:49-51`, then invokes them at
`:56-71`; each of the three inspected TS facets is only an interface file.
Thus a runtime contract pass cannot be inferred from the source as it stands.

The six mutation leaf schemas named in the previous facet audit still duplicate
shared definitions: `🏗️create-object`, `🏛️create-model`,
`🏷️create-properties`, `🪢️bind-representation`,
`📌change-representation-pin`, and `🖊️edit-design`. For example,
`🏗️create-object/🧬️schema/🔣️.json:15-55` repeats a permissive
`ArtifactRef`/`ArtifactDialect`; `🏛️create-model` and `🏷️create-properties`
do the same. Replace those definitions with the canonical shared schema refs
when correcting the aggregate mutation facets, preserving the native
snake-case leaf field names unless Rust, vectors, codecs, and tests are changed
together as one decision.

## Child Identity and Dialect Admission

The base policy is correct and should be consumed rather than copied:

- `✉️base/🧬️schema/🪆️child/🟦️.ts:8-11` first calls Store parsing and then
  requires `childId === artifactId`, `s.stdio.semio`, `v1`, and the supplied
  subset.
- Its Rust twin at `…/🪆️child/🦀️.rs:5-10` also checks canonical URI
  round-tripping.
- The current TypeScript document oracle checks the same identity and dialect
  for parent/snapshot and diff vectors at
  `…/🧪️tests/🪪️document-contract/🟦️.ts:24-30,53-66`.

Native admission is incomplete. `SemioKitValidator` at
`…/🧰️kit/🚪️io/🦀️.rs:61-93` receives only a target and rejects a wrong kind or
subset, but accepts a wrong `standard` and never compares `child_id` to
`target.artifact_id`. The three create mutation diffs then create child handles
without validation at `🏗️create-object/🔺️diff/🦀️.rs:8-14`,
`🏛️create-model/🔺️diff/🦀️.rs:8-14`, and
`🏷️create-properties/🔺️diff/🦀️.rs:8-13`. Consequently a native mutation can
produce a handle that the base policy rejects.

Change the native validator to accept the whole child handle and delegate to
`validate_semio_child_identity` for objects, models, and properties. Apply the
same admission before each create diff produces an `ArtifactChild`. Add native
negative cases for wrong child ID, `standard: "v2"`, wrong kind, and wrong
subset for all three slots. Extend the mutation corpus assertion at
`…/🧪️tests/🪪️document-contract/🟦️.ts:90-93` from ID equality to the full
dialect tuple; the three leaf JSON schemas currently accept arbitrary string
dialect components.

## Native Validation Route

Use this existing ticket target after the Kit changes:

```sh
bun nx run abstraction-ownership-validation:stdio-document-contract-native
```

It is declared at `validation/project.json:11-16` and runs the Kit/Object/
geometry TypeScript contract first, then the three focused Cargo filters at
`validation/📜️script.ts:71-80`. Its output target is the ticket's
`🗑️generated/cargo-trinity` directory (`validation/📜️script.ts:10`), while
the repository Cargo configuration keeps the expensive compiler intermediates
in the shared cache (`.cargo/config.toml:1-18`). That config states that a
private `CARGO_TARGET_DIR` diverts only uplifted deliverables; `build-dir`,
fine-grain locking, and checksum freshness remain shared. Do not add a Cargo
`--target-dir`, replace the ticket target, or move the shared Cargo
configuration merely to run this focused suite.

The root convenience target,
`bun nx run workspace:stdio-document-contract-native`, is also valid and is
the registered launcher route (`.vscode/launch.json:5-6`), but it writes its
uplifted deliverables to the shared target root. Prefer the ticket target above
when the execution receipt must remain under this ticket.

No execution result is claimed by this audit.
