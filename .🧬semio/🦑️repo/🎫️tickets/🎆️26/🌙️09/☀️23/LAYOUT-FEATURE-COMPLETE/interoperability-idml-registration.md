# IDML Native Owner Registration

The canonical IDML declaration and direct native codec factory now select `issuer::IdmlCodecOwners`. That policy delegates to the actual `owners_demand` and `admit_owners` functions; it does not substitute framework semantic preparation after refusal. The resulting catalog shares the concrete `PagedOneItemPreparationFactory<IdmlSnapshot,IdmlMutation,MutationApplyEdit<...>>` between publication and replay, with their original retirement authorities.

## Actual Caller Chain

- IDML `🧾️artifact-declaration/🦀️.rs`: both `native_codec` and `declaration` select the same source type.
- Plugin `DocumentCodecSpec::bare_with_owners` stores a noncapturing typed codec thunk; `ArtifactDeclarationBuilder::document_codec_bare_with_owners` records the ordinary codec capability claims through the same helper as the standard bare codec.
- Store codec factory `bare_artifact_codec_with_owners` monomorphizes both apply and replay thunks with that source. Existing standard codecs select `BoundedArtifactCodecOwners` explicitly.
- Apply admits its original owners before store birth and calls `codec_document_owners<P,M,Source>` before installing owners. Source refusal retires the unadopted parsed envelope without creating an ownerless store. Store-birth failure closes the already-admitted catalog. Replay supplies that same concrete constructor to `replay_envelopes_onto_pair`, which installs before ingest.
- The constructor quotes source demand, grants all four original copy/capacity/release/depth axes plus one item, admits the source, and completes each original factory retirement ticket using a fresh exact demand. Every failure with retained owners calls `close_unscheduled_catalog`. No snapshot or source payload is cold-cloned by this metadata constructor. Semantic payload preparation continues to require the publication/replay caller grants.

## Validation

Language-neutral owner-source fixture and schema were written before the implementation. Native laws assert full grant mapping and six admission verdicts; IDML laws assert refusal for missing items/copy/capacity/depth, no admitted owners on refusal, completed retirement tickets and actual shared semantic issuer identity. These native laws are authored but unrun.

`idml-registration-check.mjs` executed against actual sources with web-tree-sitter Rust and Ajv: 9 Rust files parsed without errors, both concrete registration routes and both store installation paths inspected, all 4 quoted grant axes checked, and 6 independent JSON Schema grant verdicts passed. This is structural/schema evidence, not native type checking, borrow checking or execution. No runtime external dependency was added; serde_json is test-only in the existing codec factory package.

## Remaining Native Execution Boundary

The codec's existing `apply_ops_binary` still invokes `ArtifactStore::dispatch_apply_exact`; registering retained owners does not turn that cold reduction call into the resumable publication operation. The actual retained semantic factory is now installed for the typed publication/replay ports.

There is a pre-existing generic declaration inconsistency that remains unverified and unresolved in this change: Store `⚙️engine/📥️lifted/🏪️store/🚦️cursor-admission-dispatch-and-apply-batch/🦀️.rs` defines `dispatch_apply_exact` inside an impl requiring `P: Clone + ToValue + FromValue`, `Mutation: Clone + Mutation<P>`, and the method additionally requires `Mutation: ColdMutation<P>`. The codec apply thunk's generic bounds omit P Clone, Mutation Clone and ColdMutation; its outer bare-codec bounds omit Mutation Clone/ColdMutation too. Baseline source construction now avoids the old convenience constructor's unnecessary Clone bounds, but that does not repair this separate existing dispatch contract. A native compiler is unavailable through the repository runner (see native-runner.md); no compilation or end-to-end native execution is claimed. Full Layout interchange remains incomplete.
