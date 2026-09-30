# Current Artifact Contract Audit

Read-only audit of current sources on 2026-09-30. No tests or runtime checks were run. Root, s, products, OS, and sequence AGENTS instructions were read.

## Confirmed Contract Drift

- Shared Rust `🧰️framework/🔨️modules/🚪️io/🦀️.rs` owns `Serializer`, `Deserializer`, `IoEntry`, deterministic `io_route`, `io_run`, and `io_entries` (around lines 1984–2390). These APIs are renderer independent.
- Shared TS `🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts` only supplies `ArtifactDialect`, `ArtifactRef`, and their coordinate/URI parsers. It has no `IoEntryDescriptor`, fidelity, payload, route, outcome, or diagnostic contract mirror.
- Sequence subset `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️.ts` declares its own `IoEntryDescriptorMirror` using string coordinates. Its txt import/export rows both claim `Lossy`; the corresponding Rust leaves both explicitly declare `IoFidelity::Exact` and call the real `serialize_dsl_txt`/`deserialize_dsl_txt` helpers. The TS root's statement that txt is an unimplemented stub is stale. The txt TS leaf itself is merely `export {}`.
- Sequence binary snapshot TS leaf exports only `type SequenceSnapshotBinary = Uint8Array`; it is a representation marker, not an executable codec. Existing TS leaves must not be presented as equivalent implementations without tests.

## Existing Checks and Gaps

- Root `📜️script.ts` `policyIoDeclarationBreaches` (25668+) checks Rust leaf directory name presence in root source and existence of a TS file. This accepts empty TS twins, stale descriptor fidelity, names appearing only in comments, and descriptor rows without registered implementations.
- `policyIoExclusivityBreaches` (25637+) scans Rust only and emits medium priority; it does not enforce TS import/codec ownership. Sequence editor `✏️editor/🦀️.rs:58` calls `ArtifactPack::encode_pack` outside IO, in an envelope assembly helper. This is a real ownership violation, although it is not evidence of a renderer dependency.
- `policySubsetFacetTotalityBreaches` still describes/requires an engine by fallback (20093+) while `policyArtifactEngineFacetForbiddenBreaches` immediately below forbids artifact engines. Taxonomy defaults must be aligned before promoting gates.
- `policyOsStateAuthorityBreaches` (17520+) checks globals and authority-shaped Rust structs outside OS. It does not prove mutation-only writes inside OS or TS.
- Sequence node-graph command calls `host.replace_snapshot(fixture)`; the editor also imports `sequence_snapshot_mutations`. This alone does not prove a bypass: inspect the host's implementation before replacing it. Store exposes read leases and explicit apply batches; direct writes were not established by this audit.
- A narrow rg scan of `🚪️io` files found no react/bevy/egui dependency match; this is not a comprehensive dependency proof.

## Coherent Implementation Slice

Start with schema-owned descriptor parity rather than a global state rewrite. Extend the existing shared IO JSON schema and Rust/TS schema owners with the same `IoEntryDescriptor` and `IoFidelity` wire contract; replace sequence's local mirror with that contract and correct its txt entries to Exact. Add a language-neutral descriptor fixture containing all eight sequence rows and mismatch cases. Consume it from sequence Rust IO carrier-contract tests and TS tests, comparing Rust registry descriptors and TS declarations. Use an existing JSON/schema validation library as the independent test oracle, behind test code only.

Exact source owners: shared `🚪️io/🧬️schema/{🔣️.json,🦀️.rs,🟦️.ts}`; sequence subset `🚪️io/{🦀️.rs,🟦️.ts}` and existing `🚪️io/🧪️tests/🔬️carrier-contract/🦀️.rs`; root `📜️script.ts` policy plus its current fixture/self-test owner. Extend permanent script commands only in existing `📜️script.ts`; register the Nx target in `📋️project.json` and launcher in `.vscode/launch.json`. New desired command: `bun nx run workspace:artifact-io-descriptor-parity` invoking `bun ./📜️script.ts verify artifact-io-descriptor-parity`; do not claim this target already exists.

Then strengthen the declaration policy to require real descriptor parity and reject empty TS declaration twins. Keep full codec implementation parity distinct from metadata parity: a TypeScript type alias cannot validate encoding behavior. Handle the editor pack assembly separately by moving it under its subset IO owner and testing the same envelope bytes, after reading its lifecycle constraints.
