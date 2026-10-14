# Current Repo Cargo Execution Endpoint

The assigned Repo endpoint batch is frozen at the hashes below. It uses the current schema-first General Cargo execution request/backend/declaration contract and its required `workspaceRoot` operation selector and explicit continuation contract. HighScope owns the General contract, Actor declarations/bindings, native capability/build caller receiving, and existing Repo test command roster. This batch changes only the four listed files.

## Implemented behavior

- `issueRepositoryCargoExecutionBackendV1(root, command, transport)` synchronously reads the selected Cargo package's explicit `package.metadata.semio['cargo-execution']` reference relative to that manifest. It admits one unique declaration selection, requires the declared request manifest to be the original command manifest, and resolves all request/source paths within the explicit root. Missing metadata or an unmatched command returns no backend; malformed or ambiguous declarations refuse.
- The backend includes required `continuation`: `null` for direct commands, or the exact declared nested child program/cwd/full argv with its complete selected argument tail. Repo maps only the declared `bun` program to the actual `process.execPath`, resolves child cwd within the explicit root and preserves all complete continuation arguments. It uses `selectCargoExecutionOperationV1`; the old request-only selector is removed. Contract-reader return types come from the closed defining map, with no caller-selected arbitrary result type.
- The backend is the current Bun executable, the exact private `📜️script.ts execute` argv and canonical root cwd. Its two Source snapshots are the actual Cargo manifest and declaration. Each snapshot records the full real directory ancestry, regular file device/inode as exact decimal bigint strings, actual byte count and SHA-256. Bounded reads check open-file/path identities and nanosecond stamp consistency, then recheck every ancestor. Source bytes share the original transport byte grant; no source copies or guessed quota are introduced.
- The receiver validates the closed original native capabilities, reissues the expected backend from current physical Source and the exact original command, and compares the entire descriptor and request. It does this before and after awaited input, imports and execution. It additionally retains exact policy/control/capability references and actual signal/time/progress/continuation ports, policy/capability bytes, request bytes and inherited environment through awaited work.
- Bounded stdin enforces the original byte/line transport, uses fixed-size slabs to avoid one allocation per tiny chunk, decodes strict UTF-8 JSON and admits the operation-specific request. Original signal/deadline interrupts an incomplete read. The private executable receives the existing Script envelope; it does not create a policy, envelope, factory, storage object or fallback authority. The parent General port uses owned process supervision; this receiver keeps that original serialized policy/capability/deadline lineage.
- Standard tests retain the explicit selected test level and original inherited Cargo policy, checking its manifest/mode/target and the original native mode/offline/artifact environment before calling `runRepositoryCargoTestsV1`. Exact laws call `runRepositoryExactCargoLaws` with all three authored phase ceilings clipped to the same finite original remaining time. The existing wrappers retain mode/profile/network, Store recovery, dependencies, selected Source admission, compiler/capacity leases and paid image close within the endpoint's process. No reservation is serialized or reconstructed from request JSON.

## Executed isolated evidence

Retained input recorder: `cargo-endpoint-inputs/📜️script.ts`.

- Source-only Bun unit probe: **61 pass, 0 fail, 141 expectations**. It uses actual production issuer/receiver APIs with injected execution ports; it does not invoke the Repo Cargo wrappers or Cargo.
- Strict TypeScript plus `noUncheckedIndexedAccess`: **0 diagnostics**, three entry files and **390 loaded sources**. Compiler output is `🗑️generated/cargo-endpoint/semantic.json`.
- The portable corpus includes 23 field/selection/source denials, eight continuation denials, nine original invocation mutation names, three native environment fields, physical Source mutations, declaration cases and stream cases. It was authored against the existing defining General schema before endpoint implementation. The laws cover wrong manifest/action/cwd/program/argv/package/law/hash/identity, extra policy/environment/storage/target authority, actual Source byte changes and same-byte inode replacement before effects, Source/control changes across awaited work, exact original phase clipping and explicit test-level receiving, symlink ancestry, ambiguous/ref-escape declarations, fragmented JSON, byte/line limits and original cancellation before EOF. New continuation laws prove a genuine nested descriptor resolves to the complete original child, then refuse a foreign program/cwd, same-suffix foreign argument prefix, altered/truncated argv, null nested continuation, unsolicited direct continuation and missing required continuation. Actual Ajv and production admission both refuse missing declaration/backend continuation; no legacy shape is admitted.
- Independent existing oracles: strict Ajv validates the actual canonical General schema, `@iarna/toml` independently reads the manifest's declaration reference, and Node Buffer/crypto independently compare actual snapshot file bytes/counts/digests. These validate schema/source representation and binding observations; they do not prove native Store behavior.
- `🗑️generated/cargo-endpoint/inputs.json` records matching before/after hashes for all four owned files during the final unit/compiler probe. `unit.log` retains console outcomes. No generated output was pruned; the whole ticket remains active.

## Registration and limits

HighScope is mounting the endpoint law file in the existing Specific Repo caching `test repository-exact` roster, whose registered target is `test-repository-exact-cargo`. The General Process test suite does not gain a runtime import of this Specific Repo test. No new GUI target or executable fallback is added here.

**No registered frontend, private executable process, actual Cargo/native test, genuine compiler Store/Source admission, fake-reservation owning-route law, initializer or paid-close runtime capture was run by this batch.** Existing underlying General/Repo storage laws remain required for qualification. The injected-port unit successes are not native acceptance or a fabricated receipt. Root coordinates the one genuine registered qualification after all concurrent batches settle; Process GUI2 remains prepared and held.

## Owned current files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/🟦️.ts` — `6ee2ef94ea9d531c4fed170d2a8dfaa02d0d5b9f3f6c815f305f52c84b7ea9fb` (14295 bytes; 113 absolute path characters).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/📜️script.ts` — `ec4d91529e26a00791e41947437648b51cadcc393607d9dad9d45ca9e1d7f50f` (958 bytes; 119 absolute path characters).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/🧪️tests/🟦️.ts` — `05b8c0c72657bdffd6e857a41a61184ad9cae480885158c2296f956a6a440627` (16445 bytes; 121 absolute path characters).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json` — `bc485800c81fa766d1d1ab8308c493606fbbacb04dcbe4c78390d8dc63249613` (6781 bytes; 134 absolute path characters).

## Current defining handshake byte observations

These are current read-only dependency hashes, not an independent original Source receipt or an ownership claim.

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🟦️.ts` — `672eb83e1565fb48d1f08344375ba5ff0ad4fa2a38f4755a53b3734a6cc53af2`.
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🧬️schema/🔣️.json` — `62a23334db82ddce703c572e2b1b4c64db3834ede50d4ce8c545de6c7c5f9746`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts` — `c0928ced5c0303377f0690b8dd29c5e81ac8966f1d0058fd4af162426680d993`.

The earlier51-law frontier is historical. The current61-law unit/strict run followed the required continuation/closed-reader handshake. No registered/native run was added, and all cx1/ce1/an1/Process2/Native11 recorders remain held.

## Final public facade repair

The final61-law/141-expectation run and strict/noUnchecked390-source0-diagnostic probe were repeated after replacing only this batch’s General imports with authored public entries: `@semio-tech/process/cargo-execution`, `routing`, `cargo-test`, `test-budget`, `cargo-laws`, and `@semio-tech/schema-validator-ts`. The private executable receives from the public routing entry; tests use the public named `cargoExecutionSchemaV1`. The four owned file hashes matched before and after this repeat. The earlier387-source compiler observation is historical. No actual executable/Store/native/frontend was run. Scope owns the defining Process export and Native reader edits; the new package export is real install material and no generation fallback is introduced.
