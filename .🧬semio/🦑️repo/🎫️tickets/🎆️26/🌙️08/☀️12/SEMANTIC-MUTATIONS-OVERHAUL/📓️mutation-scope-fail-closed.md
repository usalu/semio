# Mutation Scope Fail-Closed Verification

## Finding

An explicitly selected nonexistent mutation root previously returned an empty breach list because safe directory/file reads treated absence as empty content. That could turn a misspelled path into a false acceptance result.

## Correction

The shared mutation policy now normalizes Windows and POSIX relative paths, rejects absolute/traversing/opaque selections before source access, and checks every selected directory without following symlinks. Invalid scope selection fails explicitly instead of producing a clean policy report. An existing scope with an absent, unreadable, symlinked, or non-file aggregate receives a high-severity reachability finding; the inventory continues so every unconverted IO root can still be reported.

The aggregate gate also rejects empty source and any mutation variant that is not a single-field tuple wrapping a leaf. Folder/variant comparison now reports orphan folders even when the aggregate has zero variants.

## Test-First Evidence

- Added a permanent language-neutral scope fixture under the repository-library mutation-scope tests.
- The test-only fast-glob oracle independently enumerates the exact existing aggregate root without following symlinks.
- Red command: `bun nx run @semio-tech/repo-lib:test-quick -- -t 'fails closed for missing, opaque, escaped, and symlinked explicit scopes'`.
- Red result: one failing test; the missing-root call returned `[]` instead of rejecting the scope.
- Green command: `bun nx run @semio-tech/repo-lib:test-quick -- -t 'direct mutation ownership|direct mutation taxonomy'`.
- Green result: 11 passed, 261 filtered, zero failed, 87 expectations. Includes the existing Ajv and nightly Rust-parser oracle tests.
- Scoped `git diff --check` passed.

## Aggregate Regression Follow-Up

- A second language-neutral fixture supplies empty, unit, inline-struct, multi-field tuple, and orphan-folder cases.
- The red aggregate test reproduced an empty aggregate passing its structural gate.
- After correction, the same focused Nx selection passed 12 tests, 261 filtered, zero failures, and 107 expectations.
- All five aggregate vectors were parsed by the pinned nightly Rust parser; its variant counts agreed with the repository inspector.
- The first full inventory attempt exposed the Writer IO scope's missing aggregate. This is now a high-severity inventory finding rather than a fatal scope-selection exception. That failed attempt is preserved in `🧪️mutation-inventory-third-checkpoint.log` and is not a passing census.

## Exact Codec Identity Follow-Up

- The PNG gate exposed a false positive: a registry such as `change_header::text::CODEC` visibly identifies its direct owner even without a redundant PascalCase or kebab-case spelling.
- Rust codec identity checks now use exact tokens from the existing Rust lexer, accepting leaf module names and explicit semantic string identities while excluding comment text and identifier-prefix lookalikes. Root token sets are computed once per surface.
- Four language-neutral codec cases cover a valid module reference, a longer unrelated identifier, a comment-only claim, and an explicit opcode. The pinned nightly Rust parser independently agrees with all four identity outcomes.
- The red test rejected a valid module registry. The final focused Nx run passed 13 tests, 261 filtered, zero failures, and 119 expectations.
- The registered full inventory rerun succeeded with 157 roots, 2314 records, and 2070 live findings; it remains a mid-cutover census, not final verification.

## Descriptor-Backed Binary and Payload Identity

- Exact matching also recognizes types declared by the authoritative direct leaf, such as `SetMemberPayload`, instead of assuming every codec payload type equals the aggregate variant name.
- A direct binary contribution may expose only its numeric `BINARY_TAG` when payload codecs are mechanically derived. That identity is accepted only when its literal value exactly matches the descriptor; mismatches produce a high-severity wire-identity finding. Comment-only identities still fail.
- The added payload and binary tests failed before the correction. The final focused run passed 14 tests, 264 filtered, zero failures, and 131 expectations, including nightly parser agreement.

The virtual opaque path cases never create or inspect the actual repository `compose` tree. No runtime dependency or new executable command was introduced.

## Local Binary Identity Aliases

- The all-direct-root rerun exposed 81 valid PDF tags spelled `BINARY_TAG = TAG`; the literal-only check incorrectly produced 162 findings. This failed checkpoint is retained in `🧪️direct-roots-descriptor-identity-checkpoint.log`.
- The tag resolver now follows unambiguous local constant chains, rejecting cycles, missing aliases and mismatched numeric identities without executing Rust expressions.
- Five added neutral vectors reproduced the rejection before the fix. The focused Nx suite then passed 14 tests, zero failures and 146 expectations; the pinned Rust parser independently checked all eight binary vectors.
- The subsequent 29-root rerun has zero findings under the implemented structural checks (`🧪️direct-roots-binary-alias-checkpoint.log`). This is not semantic acceptance: the independent textual and glTF audits prove carrier, codec and schema gaps that require new enforcement and source repairs.

## Shared Runtime Checkpoint

The serialized Demonstrator retry reached STDIO compilation and failed with six E0624 visibility errors (`between` and `is_empty` three times each). No test assertions executed. The raster lane traced these to PNG text-chunk and JPG quantization/Huffman diff helpers; their current live sources already contain schema-scoped visibility changes and were preserved. The same lane removed nine stale no-mutation/set-snapshot grammar lines in JPG/BMP/TIFF and reran its static closure checks successfully. A fresh shared runtime pass remains required.

## Transcripts

- `🧪️mutation-scope-regression-red.log`
- `🧪️mutation-scope-regression-green.log`
- `🧪️small-schema-independent-policy-recheck.log`
- `🧪️mutation-aggregate-regression-red.log`
- `🧪️mutation-aggregate-regression-green.log`
- `🧪️mutation-codec-identity-red.log`
- `🧪️mutation-codec-identity-green.log`
- `🧪️mutation-codec-descriptor-identity-red.log`
- `🧪️mutation-codec-descriptor-identity-green.log`
