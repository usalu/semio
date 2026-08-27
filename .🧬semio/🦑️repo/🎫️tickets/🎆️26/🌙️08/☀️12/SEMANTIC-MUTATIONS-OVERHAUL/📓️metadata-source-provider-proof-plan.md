# Metadata Source-Provider Proof Plan

## Frozen Scope

FND-METADATA-SOURCE-PROVIDER-28 will add one discovery proof inspector, `inspectMutationMetadataSource`, and its repository-owned result types. It joins FND21 wrapped-origin evidence, FND12 metadata syntax facts and module context, and FND25 selected Cargo bindings. The wrapped payload remains a consumer declaration; derive and lower-contract providers resolve independently and are compared to the three frozen canonical provider identities.

## Planned Write Region

- The narrow metadata-source proof region in `🔍️discovery/🟦️component.ts`, reusing existing structure, metadata, module-graph, and Cargo-binding inspectors.
- The deliberate discovery facade exports in `📦️packages/🟦️typescript/📦️index.ts`.
- A new neutral schema/vector fixture and focused library tests, including an isolated compiler-backed fixture runner, the existing TypeScript test router, Nx target, and launch entry.

## Proof Shape

The inspector will accept a repository root, FND21 origin triple, caller-owned Rust sources/read source callback, exact consumer manifest context, and cancellation callback. It will return acceptance/diagnostics plus exact consumer context and independently resolved derive/contract canonical identities. It will fail closed for ambiguous, conditional, unsupported, cyclic, shadowed, unsafe, or missing alias evidence.

The implementation will reuse `inspectRustModuleGraph`, `inspectRustModuleGraphFacts`, `inspectRustMutationMetadataFacts`, `inspectRustStructure`, and `resolveCargoProviderBinding`. It will not add a parser, inventory, Cargo invocation, provider allowlist, root-policy activation, or production Rust edit.
