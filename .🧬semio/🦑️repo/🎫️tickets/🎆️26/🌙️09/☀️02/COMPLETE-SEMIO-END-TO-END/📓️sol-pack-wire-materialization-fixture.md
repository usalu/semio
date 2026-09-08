# Pack Value Wire Materialization Fixture

The language-neutral fixture and Draft-07 schema are:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🧪️fixtures/🧮️wire-materialization/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🧪️fixtures/🧮️wire-materialization/🧬️.schema.json`

The contract is `semio.pack.value-record-materialization.v1`: exact `SPK/1.0-record-body`, field id `1`, root tag `0x11`, and 21 raw hexadecimal vectors. Every row carries all six `PackLimits` fields. Accepted rows carry their decoded JSON value; rejection rows distinguish only `limit` from `malformed` so implementations do not couple to Rust diagnostic text.

The corpus covers zero-allocation null, a repeated 32-byte interned string at logical byte budget 352 and refusal at 351, inline UTF-8, array and map slot charges, nested depth, `u64::MAX` declared list/map counts with no body, non-single and duplicate fields, foreign field id, non-Value root, trailing bytes, symbol-count refusal, and symbol table slot plus UTF-8 materialization refusal.

Validation on 2026-09-08:

- Bun parsed every hex vector, checked unique ids, and confirmed each raw length against its `maxFileLen`.
- The permanent `🌱️value/📜️script.ts` source gate used AJV, BigInt accounting, `fast-deep-equal`, the first-party TypeScript Pack decoder, and a JSON roundtrip oracle. It replayed all 21 vectors and matched all 6 accepted JSON values and 15 rejection classes.
- No Rust/native claim was made; the native OS-kernel consumer law is owned by the root task.
