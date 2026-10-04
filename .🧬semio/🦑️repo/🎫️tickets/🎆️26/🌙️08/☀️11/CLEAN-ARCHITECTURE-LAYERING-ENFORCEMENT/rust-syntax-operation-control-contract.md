# Rust Syntax Operation Control Contract Draft

Fresh read-only source inspection. Contract proposal only: no runtime API, production edits, native/Cargo or test runs. Full current sources and SHA256 are captured below.

## Schema-first closed operation request

Declare a language-neutral request schema before implementation: `{ source: { id, raw }, limits: { sourceBytes, tokens, nesting, expansions, matcherWork, comparisonWork, references, outputBytes, chunk }, control }`. Every limit is a mandatory positive safe integer; reject missing/unknown/noninteger/nonfinite/nonpositive values. `sourceBytes` counts exact UTF-8 bytes; raw source retains identity. Reject lone UTF-16 surrogates before UTF-8 encoding rather than replacing them. Token spans retain their established character coordinates; publish coordinate unit explicitly, and add byte coordinates only with an independent admitted mapping. Do not silently change original span semantics.

Owned control carries cancellation predicate, progress callback and async yield capability, never external framework types. Progress snapshot has phase `admission|lex|pairs|templates|match|references|complete` and monotonic cumulative sourceBytes/tokens/work/references. Budgets count actual work, not just successful outputs: each token inspected/matcher step consumes matcherWork; each template-name/scope/delimiter/argument comparison consumes comparisonWork; each candidate bound invocation consumes expansions before allocating bindings, even if later refused. Count output bytes before accumulating paths/provenance. Check cancellation at entry, before every callback, after every await and within at most chunk charged steps. Reject/cancel returns an owned deterministic failure with source position and counters, never partial success references. A callback exception is explicit operation failure, not swallowed or treated as successful scan.

An async canonical scan should perform bounded admission and chunked iteration; a cancellation flag cannot change while a synchronous CPU loop monopolizes the event loop. Do not expose a synchronous wrapper as claiming interactive cancellation. Preserve the original no-controls parser law corpus as fixed semantic expectations while exercising the new controls separately; no legacy default/unlimited operation mode.

## Current concrete work hotspots

Current neutral owner lines70–165 tokenize nested comments, raw/normal/byte/character strings and identifiers; huge single comments/literals can bypass token-count controls, so charge scanned characters/bytes and bound comment nesting. String decode46–59 scans bodies and continuation whitespace; charge work before append and replace large indexOf/slice regions with bounded scans when needed. Pairing180 charges every token and stack depth. `lineAt`227 repeatedly slices/splits the full source prefix; precompute line-start coordinates during admitted scan and use bounded lookup. Do not defer admission until after these allocations.

Template construction373–389 scans token ranges and materializes pair arrays/filter/sort; template-name duplicate search394 is quadratic; each supported template scans the entire scope395; template/pair membership comparisons404 repeat across invocations. Count these comparisons before performing them; index template names and enclosing scopes once where semantics permit. Expression/matcher recursion259–363 and reference traversal428 onward need an explicit depth budget or bounded stack. Attribute metadata recursion566/637 must share nesting/work counters; no uncharged helper recursion. Array `.some/.filter/.sort` operations need bounded explicit work or preflight upper-bound charging plus cancellation/yield before execution; charging only after a large operation defeats responsiveness.

## Language-agnostic control cases

1. Cancellation at admission: empty result, no token/source normalization publication.
2. One huge block comment or raw string with few tokens: source/work budget and mid-lex cancellation still apply.
3. Deep nested comments/delimiters/concat/attributes: deterministic nesting refusal before JS stack overflow.
4. Many macro templates and candidate invocations: comparison/matcher budget refusal even when no reference is eventually emitted.
5. Repeated reference emissions: expansions/references/output budgets charge duplicates before downstream deduplication.
6. Cancel immediately after yield or progress: no subsequent emitted result or complete callback.
7. Exact limit boundary versus boundary+1: repeatable counters/refusal code; original successful scan output matches original parser oracle.
8. Malformed macro input under tight budget: deterministic precedence documented; cancellation observed first at a checkpoint, otherwise earliest charged budget/semantic refusal. Never convert unsupported expression to absent reference.

Independent original-body/system-library probes validate unchanged successful semantics; timing/control laws require actual async checkpoints and deterministic instrumentation, not wall-clock-only assertions. Native compiler expansion/dep-info remains independent emitted-edge proof.

## DSL Sharing and Source Probe Preservation

Neutral DSL lexer `🗣️dsl/🔍️lexer/🦀️.rs` has configured quoted-string modes Raw/Backslash/Doubled and JSON-style four-digit unicode decoding. Rust syntax has hash-delimited raw strings, byte strings, Rust scalar `\u{...}` escapes, nested comments, lifetime/raw identifiers and Rust punctuation. These grammars are distinct. Share owned coordinate/control/work primitives only if their semantics actually match; do not reexport DSL tokens or blindly route Rust through DSL decoding. No current TS DSL lexer implementation was found in the inspected DSL subtree.

Current physical-reference-context test now loads canonical `rustSyntax` at18–19; narrowing probe takes RustTokenKind/RustToken from that source at109 while retaining Repo helper bodies. Finite candidate probe combines syntax+Repo declarations and keeps the four-declaration assertion, using each node's actual source file for text. This preserves the inspected original extraction assertions. Tests were not run here.

Finite-target-consumption still includes `rustTokens` in its normalization function-name set at24. Correct: normalization has its own `function rustTokens(path, content, index)` at1788 producing ReferenceToken[], distinct from canonical lexical `rustTokens(source)` imported as rustSyntaxTokens. Removing the normalization name would break the original extraction law. Keep both actual identities and preserve original assertions.

## Captured Sources

| Source | SHA256 |
|---|---|
| `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts` | `4328b2d48a82da3165a94b4d52f0b05a588a9099aecbd58ae0d8704d690868da` |
| `🧰️framework/🔨️modules/🗣️dsl/🔍️lexer/🦀️.rs` | `c018119d3d5c6ce3c7feb496567813101cb1f27f81e3b9640c8f87d47b9dd48d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts` | `15b85cdbcd4c8a67bde2168f1e8a937e44622847320f8d5c9ffe5889d9686f12` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts` | `4ede4cfac606af18a857a893eaa4f6fd4112a8178314430e27a78c38348aa909` |
