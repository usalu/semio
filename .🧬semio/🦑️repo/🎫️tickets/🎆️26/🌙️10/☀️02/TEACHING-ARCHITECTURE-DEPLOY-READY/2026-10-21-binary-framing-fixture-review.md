# Binary Framing Fixture Review

Current six-file candidate statically clear. The entire serialized ReadSessionFixture validates with strict Ajv after normalizing schema and DATA into the validator realm; the earlier const mismatch is not a corpus defect. No Source/helper execution or Native test occurred.

Ten finite rows cover empty,64KiB Text threshold/+1,4MiB decoded Binary threshold/+1,16MiB aggregate threshold/+1, valid four-byte UTF8 straddling Binary boundary, non-ASCII newline and lone UTF16 surrogate refusal. The string helper validates lossless aggregate roundtrip before chunking. Chunks are canonical base64 bounded by existing Binary authority; independent base64-js encode/decode plus SQLite BLOB lengths and ceil chunk counts verify bytes and frame counts. Aggregate reconstruction uses fatal UTF8 decoding, not per-frame text decoding.

The16MiB exact payload is encodable but cannot admit another request plus reserved final frame; this distinction is explicitly asserted. Existing cohort law covers256-frame exact reserve and byte boundary/+1. Actual session state initializes bootstrap frame count/bytes, charges one credit plus actual request-frame count, retains5-byte final null-newline reserve, and awaits readiness before traffic. Current request maximum fits one Binary frame, so existing request-plus-credit-plus-final three-frame reserve is sound. No generic writer or canonical own-data guard weakening is involved.

New literal law is closure input frames preserve exact bytes within existing own-data authority, explicit60000. It tests actual helper/DTO guard and structural production wiring; it does not itself prove a real channel/loader bootstrap succeeded. Existing actual worker laws and future Native startup remain the physical integration evidence. String-source assertions are limited structural evidence, not loaded-byte attestation.

Current pins:

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts — 10051/c2a65ecae1f2de61932694d23de44c9b98c307c853c39b2c2b35d11d8c4e7203
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟨️.mjs — 6792/bfe8527969c20e488575626c151d4f6f48016813e0c816ec49b1d922a65feaff
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟨️.d.mts — 1658/404ac6c24590782cb437b4b772c003aca3e5bf6ca986cfbae08fa3395e23ee84
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧬️schema/🔣️.json — 39693/be4522150fd41cc7a53019326e8374e101b3658def882b76d5a1a11fecaa664d
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🟦️.ts — 13174/1aaaf94183731fc26e9b930537092a3ff515a6a4a9c9c2ed065e4bf6e6468e68
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🔣️.json — 12246/bc9236a4f6d59df43528378ec6e2c1cbb4d9718f9338e6226fbee2d86b3e2c71

Validator realm detail: createRequire(root+"/package.json") loads existing Ajv; JSON.stringify then JSON.parse in node:vm.runInThisContext normalizes both plain schema wrapper and fixture DATA into the same tool realm before Ajv({strict:true}).validate. Wrapper is {definitions:schema.definitions,$ref:"#/definitions/ReadSessionFixture"}; full validation returned true with errors null. This parses DATA only and never evaluates repository Source.
