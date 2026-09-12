# Generated Styling Owner Correction

The generated-source report proves anonymous basenames and producer closure, but two reported canonical outputs still retain implementation-specific source ownership:

- Styling Python tokens are below styling/packages/python/styling/tokens/🐍️.py.
- The .NET palette remains below styling/🔷️net/Elements.Styling/Generated/palette/🔷️.cs.

These are source ownership concerns even though the leaves are anonymous. Move the Python token projection to the implementation-neutral styling token owner beside its Rust representation, and move the C# palette projection to the semantic palette owner beside its CSS representation (or the exact shared semantic generated owner if the live contract requires it). Keep actual language package metadata/glue in ordinary packages/python or packages/dotnet boundaries. Do not add a new semantic registration that merely perpetuates a language-first source tree.

Update producers, exact output contracts, native Python/.NET packaging/import/build inclusion, ignore rules, fixtures and all consumers together. Preserve the public generated API and independently verify both native imports/builds as available. Native distribution artifacts may stage source according to the tool’s exact contract, but the persistent repository implementation source must remain at its neutral semantic owner; no duplicate compatibility copy or adapter facade at the obsolete source coordinate.

Inspect the complete former .NET owner while changing these files: if it only holds package metadata after extraction, place that metadata in the ordinary package boundary and remove empty old owner chains. Preserve unrelated authored inputs and existing package metadata contracts. Extend the 29-identity portable fixture and producer/native freshness evidence to the corrected paths, update the generated-source report and exact moved-file attribution, and finish this narrow correction before the Terra UI/generated audit. No new runtime dependencies, modifying Git/AGENTS, migration scripts, ticket/goal lifecycle changes, or broad unrelated styling rewrite.
