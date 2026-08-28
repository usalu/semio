# Plugin Declaration Negative-zero Correction 48

## Frozen correction scope

Only the declaration fixture controller, duplicated ticket/actual raw-wire vectors, and this report changed. No Plugin main source, production codec, descriptor, schema contract, compatibility layer, launch entry, Cargo target, or Store/common compiled input was changed.

## Genuine reference RED

The controller first adopted the pinned-serde lexical rule while the raw vector still claimed that `-0` was accepted. The retained scoped Bun/Nx RED is [run-bMYYze](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-declaration-fixtures-43/🧫️run-bMYYze/📓️result.md): `567/570`; the three failures are exactly one `i32 negative zero` model mismatch for each of Std1Any, Std1Strict, and Std2Any.

This is a source/reference RED, not a Rust runtime run.

## Correction

The two parity fixtures now declare `{"SetValue":{"value":-0}}` as `accept: false` with no `decodedValue`. The controller rejects raw `-0` before its BigInt range model. The shared mounted native `assert_codecs` helper already consumes every raw vector through both `parse_op` and `decode_op`; removing `decodedValue` correctly prevents any invented successful decode assertion.

The retained Bun/Nx GREEN is [run-746vMw](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-declaration-fixtures-43/🧫️run-746vMw/📓️result.md): `570/570`, with canonical-nofollow first/final input stability. Its changed parity fixture hash is `9402b49f7396787e62168b293492dee128de1269ad8b15c3f05701ab1f1a7134`; controller hash is `42af7b8addcb1b3c742bdee54e548652b5d66cdde05e47977c8adf4cfe1f3f5e`.

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-declaration-fixtures-43/📜️script.ts
```

## Evidence status

The earlier `run-ff9nZl` 570/570 receipt is preserved but flawed on negative zero and must not be used as codec acceptance evidence. The corrected green is still controller/Ajv/raw-parser evidence. Native declaration tests have not run; root's next Plugin runtime inventory must establish actual serde text and UTF-8-binary rejection.
