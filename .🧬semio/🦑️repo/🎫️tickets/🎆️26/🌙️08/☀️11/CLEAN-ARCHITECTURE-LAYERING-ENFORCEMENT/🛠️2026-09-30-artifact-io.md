# Artifact IO Contract Slice

## Implemented Contract

The framework IO JSON schema now owns `IoFidelity`, `IoEntryDescriptor`, and `IoRoute`, alongside its existing owned dialect/reference vocabulary. TypeScript exposes the same owned descriptor and route shapes with strict dependency-free parsers. Rust descriptor and route decoders reject unknown fields to match JSON Schema `additionalProperties: false`. GraphQL and protobuf vocabulary declares the same fields.

Sequence IO declarations consume the shared descriptor contract instead of defining a string-coordinate mirror. Both text carrier directions declare `Exact`, matching their executable Rust leaves. All four import descriptors declare `sniffs: true`, matching the deserializer constructor's installed sniff function; this indicates a sniff entry point exists, including a default that can return `Confidence::None`.

The sequence IO owner exposes `snapshot_pack`. Its editor consumes that operation to build `LoadDocument` effects, so direct `ArtifactPack::encode_pack` no longer occurs in the editor's document-reset path. SPR generation remains in its host lifecycle owner and does not construct a live envelope.

## Schema-First Tests

The neutral `📇️descriptor-parity.json` fixture specifies all eight rows, valid routes, structural rejection cases, and structurally valid metadata drift cases. The TS test compares declarations to the fixture, compares strict owned parsers to Ajv, and proves a valid descriptor can still drift from registered metadata. Ajv remains test-only.

The existing Rust carrier-contract module now has two `artifact_io_` tests. They compare actual declaration and live registry descriptors to the neutral fixture via the owned JSON codec and independent `serde_json` projection, reject malformed descriptor/route rows, resolve the registered Exact text route, build actual `LoadDocument` effects with the native bytes, and execute the registry's text export/import round trip over the existing neutral snapshot fixtures. Permanent test progress messages record registry counts and payload sizes; temporary debug prefixes were removed.

## Package Integration

The sequence Rust package owns script routing and these Nx targets:

- `@semio-tech/sequence-sequence-rs:io-descriptor-parity` — TS/Ajv assertions plus native Rust `artifact_io_` tests.
- `@semio-tech/sequence-sequence-rs:io-descriptor-parity-oracle` — TS/Ajv assertions.
- `@semio-tech/sequence-sequence-rs:canonical-architecture` — the complete focused check for the root's generic coordinator.

The existing package `test` target also runs the TS descriptor oracle as a twin. Root command and launch integration belong to the coordinating agent.

## Validation Status

- Red: loading the new TS test before implementation failed with `Export named parseIoRoute not found`. The original Nx test run waited on graph construction and was canceled to avoid launching an unnecessary full suite.
- Green diagnostic: after implementation, the same Bun import ran 45 assertions over eight descriptors and emitted `[DEBUG] artifact IO descriptor parity checks=45, entries=8`.
- `git diff --check` passed for the changed framework IO and sequence paths.
- `NX_DAEMON=false bun nx run @semio-tech/sequence-sequence-rs:io-descriptor-parity --skip-nx-cache` passed after the coordinating dependency agent repaired the Nx discovery graph. The target ran all 45 TS/Ajv assertions and both native Rust tests (210 unrelated tests filtered out) in 2m10s. Real registry/reset/text round-trip progress included a 312-byte native payload and 186-byte SPR. The queued oracle-only invocation was canceled as the complete target includes it.

## Files Changed

- `🧰️framework/🔨️modules/🚪️io/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs,🔗️.graphql,🛰️.proto}`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{🟦️.ts,🦀️.rs}`
- `…/🚪️io/🧫️fixtures/📇️descriptor-parity.json`
- `…/🚪️io/🧪️tests/🔬️descriptor-parity/🟦️.ts`
- `…/🚪️io/🧪️tests/🔬️carrier-contract/🦀️.rs`
- `…/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`

No runtime libraries, migration scripts, compatibility exports, AGENTS edits, git mutations, or worktrees were introduced. Temporary output is confined to the shared ticket's generated folder, which the coordinating agent removes when closing the ticket.

## Exact Owned File List

Repository-relative paths; shared facade/kernel edits replace duplicate definitions with canonical imports.

- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🔗️.graphql`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🛰️.proto`
- `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧫️fixtures/📇️descriptor-parity.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️descriptor-parity/🟦️.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️carrier-contract/🦀️.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/📋️project.json`

Removed files: none in the IO slice.

Research reports: `🛠️2026-09-30-artifact-io.md` and `🔍️2026-09-30-artifact-state-authority.md` in this ticket.
