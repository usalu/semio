# TXT Mutation Binary Protocol

## Defect and Schema

The committed TXT mutation protocol declared `chain tag u8`. The protocol parser treats `chain` as a terminal byte-tail directive, so it interpreted `u8` as an unknown directive. Root's registered TXT schema selection therefore ran 50 tests with 48 passes and two protocol failures: `committed_grammar_and_protocol_files_parse` and `protocol_walk_law`.

The corrected, unchanged framing is one byte of tag followed by the owning leaf payload:

```text
framing record
header fixed 1
field tag u8
chain payload bytes
```

The tag stays visible as a typed `field`; only the variable leaf payload is the terminal byte chain. No opaque field hides the tag or any required frame boundary.

## Schema-First Evidence

The neutral five-frame fixture fixes tags `1..5` for SetTrailingNewline, SetLineEnding, InsertLine, RemoveLine, and SetLine. It also fixes seven malformed frames: empty, unknown tag, and each known tag without its required payload.

Ajv validates the fixture, while the ticket reference parser independently accepts only the exact one-byte-tag/payload-tail directive sequence. Before the patch, the Nx-wrapped Bun preflight rejected the old `chain tag u8` source. Retained: `🧪️txt-protocol/🧪️schema-red.log`. After the patch, all seven structural/reference assertions passed: `🧪️txt-protocol/🧪️schema-green-7.log`.

## Actual-Source Runtime

The production root binary test `generic_framing_tags_payloads_parse_and_walk_all_leaf_frames` encodes all five actual `TxtMutation` values, asserts their exact first-byte tags, requires a payload tail, parses the committed protocol, walks every exact encoded frame to `consumed == frame.len()`, decodes each frame back to its mutation, and rejects all seven malformed frames.

The dedicated ticket runtime probe validates five canonical leaf mounts in the reused actual-source fixture, compiles it with supplied fresh paired kernel/schema/serde artifacts, lists the exact test name, and executes that exact test once. The fresh runtime passed:

```text
1 passed; 0 failed; 34 filtered out
```

Compiler and runtime statuses were `0`, with no signal or spawn error. Retained inputs, artifact SHA-256 values, compiler output, listing, and runtime output: `🧪️txt-protocol/🧫️run-7RPZm8`. This is actual-source compiler/runtime evidence, not the root-owned registered 50-test Cargo selection.

## Coordinator Review

Root reviewed the actual protocol and production test, then independently executed the exact retained actual-source test through scoped Bun/Nx. It ran one test, passed, with 34 filtered and exit 0: `🧪️txt-protocol/🧪️root-exact-replay.log`. The bounded protocol-syntax correction is accepted. Full registered TXT runtime remains open until the following canonical source/mount cutover has completed and a fresh binary is built. The completed protocol source and generic framing test are released to TXT-CANONICAL-ROOT-23 for physical filename changes, preserving their verified semantics.
