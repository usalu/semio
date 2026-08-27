# TXT Facade Independent Review

## Scope and Method

This review read the current TXT mutation source and created only independent ticket
fixtures and a Bun/Nx probe. It did not change a TXT production file, did not read or
materialize `compose`, and did not run Cargo.

The neutral schema and 21 vectors are retained in
`🧪️txt-facade-independent-review/🛂️review.schema.json` and
`🧪️txt-facade-independent-review/🧫️fixtures/🔣️vectors.json`. The probe imports the
actual leaf TypeScript decoders, parses actual GraphQL and protobuf files through the
ticket-local GraphQL 16.11.0 and protobufjs 7.5.4 oracles, and validates actual payload
schemas with Ajv.

## Surface Observations

The scoped command below passed the independent TypeScript, schema, and parser boundary
checks. Its retained output is `🧪️txt-facade-independent-review/🧪️verify-green.log`.

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️txt-facade-independent-review/📜️script.ts
```

The check established these actual-source facts:

- All five payload schemas admit the planned valid domains; insert/set reject indexes
  above `u32::MAX` and lone UTF-16 surrogates, while admitting U+FEFF and a valid astral
  pair.
- The actual GraphQL document builds and validates exactly one root `@oneOf` branch;
  zero and two branches are rejected. The parsed protobuf aggregate has the exact five
  `oneof` arms, and its line-ending enum is exactly `LF = 0`, `CR_LF = 1`. The owned scalar admits the maximum `u32` and
  rejects negative, fractional, non-finite, overflow, non-`IntValue`, leading-zero, and
  negative literals.
- The actual raw-byte decoders preserve a leading BOM, accept `LF = 0` presence, accept
  `u32::MAX`, and reject missing, unknown-enum, duplicate, unknown-field, nonminimal
  varint, malformed UTF-8, missing field, wrong enum spelling, and lone-surrogate cases.
- protobufjs is not the admission authority: its actual `verify` returned `null` for
  negative index, `4294967296`, and an unknown object property. The owned byte decoder
  tests above are therefore necessary and are separately passing.

The aggregate TypeScript module has no runtime exports, and its text/binary TypeScript
modules are type aliases only. This is a current representation boundary, not evidence
that a TypeScript text/binary codec exists. The frozen proposal assigns text/binary
runtime decoding to Rust, so a text/binary TypeScript round-trip was not claimed.

## Rust Actual-Source Result

The actual-source `rustc --test` gate uses the retained real TXT aggregate harness,
current mounted snapshot/diff/support/mutation sources, and coherent paired pre-cutover
kernel, schema, serde, and serde-json artifacts. Every source and artifact fingerprint
was unchanged before and after.

The failure is current production source, not an artifact mismatch:

```text
E0308 at .../🧬️mutation-support/🦀️component.rs:42
expected Vec<(&str, &DslValue)>
found    Vec<(&&str, &DslValue)>
```

`txt_required_object` originally produced E0308 by pushing `key`, then E0621 after the
first repair because the returned key lifetime was not tied to `keys`. The designated TXT
owner repaired both defects (`*key` and `keys: &[&'a str]`). The retained first and
post-first-repair diagnostics, invocations, paired artifact hashes, and stable
fingerprints remain in
`🧪️txt-facade-independent-review/🧫️actual-source-run-dILdwu/` and
`🧪️txt-facade-independent-review/🧫️actual-source-run-rIIQzy/`.

After the lifetime repair, the same scoped Nx command with `actual-source` appended
compiled and ran a 34-test current TXT source roster: all passed, with `status: 0`, no
signal, and no process error for compile, roster listing, and runtime. The exact roster,
all command arguments, compiler/runtime logs, binary hash, paired artifact hashes, and
before/after fingerprints are retained in
`🧪️txt-facade-independent-review/🧫️actual-source-run-katEVb/`. It includes both generic
text and binary framing tests, the five leaf inverse/root-codec tests, all five metadata
tests, mutation-support's snapshot codec matrix, aggregate roster verification, and all
diff laws. This is a retained direct `rustc` actual-source gate, not a Cargo registered
test claim. No fallback artifact pair, substitute mutation type, partial aggregate, or
fake codec was used.
