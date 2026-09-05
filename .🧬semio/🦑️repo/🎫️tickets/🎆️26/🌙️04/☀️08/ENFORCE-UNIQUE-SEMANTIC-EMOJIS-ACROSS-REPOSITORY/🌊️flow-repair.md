# Flow Hand-Reviewed Repair

Status: naming review complete for the Flow tree; native implementation verification remains limited below. Earlier command and panel decisions are recorded in `🩺️repair/🖐️names.json`. No automatic emoji selection, replacement script, or modifying Git command is used in this pass.

## Additional Decisions

| Parent | Original | Handpicked | Reason |
| --- | --- | --- | --- |
| Flow mutation vocabulary | `🔀️🪟️reorder-widgets` | `🔢️reorder-widgets` | Changes the numerical widget order; distinct from reordering synapses. |
| Flow mutation vocabulary | `🔗️connect-widgets` | `🔌️connect-widgets` | Connects widget ports; distinct from the sibling GraphQL leaf. |
| Flow v1 any subset | `🧪️oracle` | `🔮️oracle` | Independent reference implementation, distinct from executable tests. |
| Root action cohort | `🔣️.schema.json` | `🧬️.schema.json` | Schema for the sibling JSON data, not the data itself. |

The two mutation descriptors retain their semantic IDs and payloads; only exact owner paths and their handpicked emoji metadata change. Rust module references and the oracle catalog point to the new names. The oracle name is registered for this exact owner so discovery does not silently drop it.

## Verification

Direct final enumeration found 813 entries, 777 governed entries, and zero naming findings, including authored fixtures and generated entries. The Flow `test-source` Nx route passed after its parameter-fixture schema reference was corrected: it had loaded the command schema twice. Runtime scenario/oracle output confirmed execution.

Before/after graph-generator previews each contain 23 nodes with byte-for-byte identical contents and zero stale removals. Discovery now uses the semantic `manifest.json` suffix and its existing parsed manifest-schema discriminator, rather than requiring every graph manifest to share one emoji.

The earlier Flow Cargo check reached seven API/type errors after path-related failures were repaired. Those failures are not yet attributed to this naming task; concurrent implementation changes must not be overwritten.

## Remaining Individual Decisions Applied

Six window-options folders now use `☑️options`, distinct from their configuration siblings: edit/main, edit/compiled, generate/preview, generate/form, generate/generations, and viewer/view/main. Their module references and canonical window role were updated together.

The ten mutation schemas now use `🧬️.schema.json`, distinct from their JSON fixtures: create-widget, delete-widget, duplicate-widget, move-widgets, reorder-widgets, reorder-synapses, replace-widget, update-synapse-endpoints, connect-widgets, and disconnect-widgets. Each descriptor's exact payload-schema reference was updated.

Editor fixture folders now express their individual roles: `🪪️content-identity`, `🧹️delete-cascade`, and `📡️host-wire`. Each corresponding schema and the grant-frontier schema use `🧬️.schema.json`. Flat fixture schemas are `📝️slider-labels.schema.json`, `🧩️artifact-recipes.schema.json`, and `📐️artifact-canonical.schema.json`; their paired fixture filenames remain distinct. Source includes and the fixture command router were updated exactly.

The artifact's oracle is `🔮️oracle`, with an exact owner-specific discovery registration. Its scene-owner specimens are `⚖️flow-scene-owner-law.json` and `🏠️flow-scene-owner.schema.json`; the schema reference and its filename assertion changed together, without changing the semantic schema URI.

At the plugin root, `🪪️manifest` denotes its identity facet and `🕸️manifest.json` denotes the graph catalog. The existing `🛂️.descriptor.semio` keeps its own distinct identity.

## Normalizer Guard

The normalizer previously preserved stacked filenames while rewriting a handpicked schema filename back to `🔣️.schema.json`, reintroducing a sibling collision. A neutral regression fixture now covers preserving meaningful single-emoji names, rejecting stacks and emoji embedded in the stem, and preserving reserved README names. The focused Nx run passes eight tests through the Bun and TypeScript compilers, with Ajv and emoji-regex checking the independent fixture contract. No normalization write command was run.

## Final Census Corrections

A later repository-wide census exposed two authored directories that the earlier review had not distinguished. The plugin-root test cohort is now `🎬️action-cohort`, leaving `🧪️fixtures` as the fixture collection. The editor's state-store ownership fixture is now `🏪️store-owners`, distinct from sibling `🧹️delete-cascade`. Exact Rust and fixture-runner references were updated without changing the Note plugin's separate cohort path.

The strict Flow audit now reports 484 files, 363 directories, 811 governed entries, and zero findings in every category. The existing fixture runner completed successfully after the correction, including its state-store owner oracle.
