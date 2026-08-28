# Store Owned Decoder Grammar Test Preparation 48

## Scope

This packet adds only schema-first vectors, a ticket controller, and native tests in Store's existing `#[cfg(test)] mod tests` mount. It does not change `OwnedSchemaRecordCursor`, `OwnedSchemaNestedRecordCursor`, catalog ownership, cursor/envelope decoder production behavior, DSL/derive, commands, interactions, lifecycle, or output. No `compose` path was accessed.

## Frozen pre-correction observation

The current outer and nested cursors both transition `Separator + Comma` to `Key`, then admit `Key + ObjectEnd`; a complete record can therefore accept a trailing comma. Both key lookups compare raw quoted bytes and require `expected.len() + 2`, so a valid JSON key escaped to the same ASCII string is rejected before schema identity/duplicate logic.

The source-only packet's current stable baseline is recorded with the final reference result below. It deliberately leaves the cursor's current grammar unchanged; the test mount and test-only schema fixtures are the only Store changes.

## Final direct test owner

- Domain vector: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️owned-schema-record/🔣️vectors.json`
- Domain vector schema: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️owned-schema-record/🧬️schema/🔣️.json`
- Domain-native test module: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️owned-schema-record/🦀️.rs`
- Ticket controller: `🧪️store-owned-decoder-grammar-48/📜️script.ts`
- Existing actual cursor test mount: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` mounts that direct test module once at Store source scope, beside the existing fixture mount and before its inline `tests` module.

The raw vectors cover literal and escaped ASCII schema keys, literal and escaped-semantic duplicates, missing required and unknown fields, leading/trailing commas, EOF, trailing tokens, mismatched delimiters, escaped quote/backslash/control/`u00xx` keys and values, whitespace around the full JSON text, nested records, split page boundaries, and cancellation.

The controller validates the fixture against its actual Ajv 2020 schema, then independently uses standard `JSON.parse` and `jsonc-parser` with comments/trailing commas disabled. It deliberately walks `jsonc-parser` property nodes so escaped/literal duplicate keys are checked by decoded semantic key rather than the object value that `JSON.parse` collapses. It accepts JSON-leading/trailing whitespace. `referenceCode` is the standard-parser/reference classification; `nativeCode` is a separately declared required cursor diagnostic. It derives the workspace root from `import.meta.url`, checks workspace/ancestor/final paths without following symlinks, first-hashes the controller, fixture, fixture schema, Store component, and direct test module, and rereads every input before success.

The older retained results under `🧪️store-owned-decoder-grammar-48/🧫️runs/1787859075293` and `1787859166472` describe the pre-move test owner and remain evidence only. The old vector was intentionally moved as the task's exact owned-byte relocation; it is not recreated.

## Authored native test roster

- `owned_schema_record_accepts_every_valid_semantic_key_and_small_page_fixture`
- `owned_schema_record_rejects_every_invalid_fixture_with_its_native_diagnostic_and_retires_pages`
- `owned_schema_record_cancellation_fixture_retires_every_owned_page`
- `owned_schema_nested_record_accepts_and_rejects_every_fixture_with_the_same_semantic_key_rules`

These instantiate actual `OwnedSchemaRecordCursor` and `OwnedSchemaNestedRecordCursor` with the existing `OwnedSchemaDecodePages`, `StepContext`, and `close_step`/`terminal_is_empty` protocol. They do not model or copy the cursor. The current production grammar is expected to make the strict trailing-comma and escaped-key native tests red until the separate authorized correction lands. Native tests have not been run: no Cargo, rustc, or native slot was used.

## Executed reference gate

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-owned-decoder-grammar-48/📜️script.ts
```

The final independent reference/controller passed 192 assertions. Retained result: `🧪️store-owned-decoder-grammar-48/🧫️runs/1787859858999/🔣️result.json`. Its final reread hashes are Store component `d9c8ce77be44113b217687d5bba4f3da6c55b7feb0d99bbe3fa2c002fe269beb`, direct native module `ca91e79605d14d2fd38e68fbceef766a81c5ee5f154495133b293c6630c1f9a7`, vectors `11f6fad0e8ae2e4ca3fe1cf760269070599c9ef2ec82a23a239fcbaeeebe865f`, fixture schema `83337d3c9d9353ae92fb712eeea303877418e1440dc1ea0b78139d269ff133ef`, and controller `438c127e5744c14d9ab3a3bce406d1a5cee6f875ef936d5b206bf5a2593d7f66`.

## Requested correction boundary

When authorized, change only the outer/nested record grammar transitions and semantic key decoding needed to satisfy the native tests. Do not alter catalog ownership or the Store cursor/envelope close contract.

## Root Pre-Native Review

The native module is mounted at Store source scope, not inside the inline `tests` module. Root corrected the leading-comma diagnostic expectation to the actual `schema-json.expected-field` contract before execution. The test methods now close each cursor and collect all row failures before asserting, so one escaped-key failure cannot hide the trailing-comma or later page cases. Cancellation also closes before checking its result. Production cursor behavior remains unchanged.

The reference controller no longer requires the known-bad production source spelling; it checks actual cursor mounts and retirement assertions. Independent replay passed 192 assertions at `🧪️store-owned-decoder-grammar-48/🧫️runs/1787860140475/🔣️result.json`. Controller SHA256 `4977958ada3f69891e86fabe321926cd5359556da0485748339fe6632aa89b12`; native module `8e8ccaffa0c23179b33ea55d87ac19df0b5c631362318b0d41a2b6326962cda9`. Store component remains `d9c8ce77be44113b217687d5bba4f3da6c55b7feb0d99bbe3fa2c002fe269beb`.

The existing OS native controller now accepts `owned-schema-grammar`, selecting exactly the four declared tests, recording every failed test, and stopping on timeout or source drift. Its default fixture and checked-integer commands are unchanged. Native controller SHA256 `ff97c787cd30a402a8f7b3e2a89505fc75801acbb800832693f7b6455737ee6d`. The bounded native RED attempt was announced to Demonstrator and Runtime after both exact source exclusions and target availability were released. This paragraph does not claim its eventual result.

## Actual Native RED And Repair

The first native attempt is retained at `🧪️os-kernel-fixtures-41/🧫️run-zqr2UU`. OS-kernel compiled successfully in 107.863 seconds with the existing default test profile and two build jobs. All four selected tests executed; zero passed and four failed. Captured inputs were stable, and the target was explicitly released afterward.

Two failures are actual grammar defects: the outer-invalid and nested tests observed escaped-equivalent keys rejected as unknown, escaped duplicates rejected with the wrong diagnostic, and trailing commas accepted to completion. The valid-page and cancellation tests instead failed in their test setup: a short nonterminal page violates the existing fixed-4096-byte admission contract. Those two failures do not establish a production cursor defect.

The corrected language-neutral page fixture declares the actual 4096-byte capacity and prefixes whitespace to fill the first page while preserving the exact split escape. Both reference and native consumers now construct that same layout. The page admission implementation was not relaxed.

The production repair is limited to semantic ASCII key comparison in the existing token cursor and separate after-comma states in the existing outer/nested record cursors. Comparison accepts JSON short escapes and `\u00xx` equivalents, rejects non-ASCII truncation, reads only the bounded token span, and allocates no payload. The existing field-spec maximum bounds the work. Both record cursors reject an object-end token after a comma with `schema-json.trailing-comma`.

The expanded reference gate passed 212 assertions at `🧪️store-owned-decoder-grammar-48/🧫️runs/1787860628876/🔣️result.json`. Current Store SHA256 is `0ed0d7a78c833c1081825c598de3a5dde36ecc858a2e1448c5695899358efd0d`; native module `c9e5ab882e662e74470a5604de972e3828e5037f2a3a5177ba79d0825f380047`; reference controller `06648d151ebabada8e24b93cf1b6e2e9966a905c8f784f3d19678665a842c189`. The unchanged native controller was granted the next same-target four-test attempt. Its result is not implied by the source/reference gate.

Taxonomy registered the exact source/native commands at 410.494 and 410.495, preserving default 410.38 and checked-integer 410.41. No registration command executed the tests.

## Verified Native GREEN

The repaired native attempt at `🧪️os-kernel-fixtures-41/🧫️run-QraYC8` completed successfully: OS-kernel compiled in 63.768 seconds, then all four exact grammar tests passed. Captured inputs remained stable; no timeout or skipped selected test occurred. Store/source/test/controller hashes are the ones recorded above. This verifies outer/nested semantic-key matching, strict record delimiters, valid fixed-page splits and cancellation retirement for the actual fixture cases.

The compiler slot was explicitly released and a fresh exact-name process check found no Cargo/rustc. The earlier RED and fixture-setup failures remain retained. This does not verify FreshField/FreshVcs retained typed-field cleanup, all Store tests, Plugin compilation, or the whole monorepo.
