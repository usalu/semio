# Framework Artifact I/O Exploration

## Architecture

Keep semantic definitions under `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}`. Put native representation codecs under `🚪️io/{📝️text,💾️binary}/<semantic-facet>/[member]`. Binary and text are representations, never mutation members. Foreign format import/export trees remain directional; native codecs remain bidirectional.

## Existing Abstractions

`🧰️framework/🔨️modules/🚪️io/🦀️.rs` already provides `PayloadCodec` (line 724), `ArtifactCodec` (738), payload sources/sinks, resource resolution, bounded streaming, diagnostics, budgets and cancellation. Reuse these contracts instead of introducing competing transport abstractions. Its language-agnostic fixture directories and Rust mechanism/fidelity tests provide existing validation locations.

Semantic-layer ownership cleanup remains a follow-up concern: `OpText` and `OpBinary` are defined in `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` around 1387–1412; `ArtifactDsl` and `ArtifactPack` live in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` around 6118 and 11048. The latter owns OS-specific native registration and SQLite capability hooks, so wholesale extraction needs deliberate separation of framework codec contracts from OS registry policy.

## Coordinated Infrastructure Changes

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: remove the old shape and migration allowance from `_cleanMechanismComment`; revise `ioSemanticCollectionDirNames`, `semanticCollections`, and representation-parent vocabulary. The current comment explicitly authorizes `io/<facet>/<representation>` and old `schema/<facet>/<representation>` trees.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`: revise recognized child shapes around 3840–3862, mutation-member exclusion around 3891, semantic collection inference around 12290, and related taxonomy interface documentation around 1007. Representation folders must never appear as mutation members.
- Root `📜️script.ts`: `policyTaxonomyDirsBreaches` around 17580 currently admits representations under schema children and allows any mutation child. `policySchemaRepresentationBreaches` around 22269 explicitly requires representation trees inside every schema child. Move those completeness requirements to the I/O representation-first tree and reject misplaced codec directories. Update old explicit codec paths around 6488–6517. The broader ArtifactIo policy region spans 21508–23069.
- Existing schema facet loader `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/📚️facet-leaves/🟦️.ts` loads structural schema leaves; keep these semantic contracts separate from grammar/protocol discovery.
- Authoring templates, generated owner module mounts, artifact inventories, Rust module references, TypeScript imports and registered tests must follow the same move. Do not retain re-export compatibility aliases in semantic mutation modules.

## Verification Recommendations

Add language-agnostic path vectors for accepted representation-first snapshot/diff/mutation/inference codecs and rejected schema-contained or mutation-contained representations. Validate paths using an existing third-party filesystem glob library alongside repo discovery. Test mutation discovery explicitly excludes binary/text and preserves real mutation members. Exercise the existing I/O codec round-trip fixtures after moving source files. Use Bun and Nx targets and register any new executable command in launch.json.

No runtime tests were run during this read-only exploration; these findings come from source inspection.
