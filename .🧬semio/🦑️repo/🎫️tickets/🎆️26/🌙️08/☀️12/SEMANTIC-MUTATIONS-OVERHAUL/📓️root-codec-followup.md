# Root Codec Follow-Up

## Bounded Read-Only Census

The coordinator inspected only text/binary Rust codec files directly under the same 29 known mutation roots. The source-level follow-up found eighteen whole-enum `serde_json::to_vec(self)` paths: two each for PDF 1.7 Any/E/H/UA/VT/X and JSON/XML/SVG Any. TXT was already being repaired and no longer had that whole-enum call, but still had an intermediate per-kind root match table when reviewed. glTF codecs outside these exact roots were not part of this bounded census; their separate audit remains authoritative.

These are follow-up findings, not a new global inventory. The source matcher is a bounded triage query, not a substitute for the AST verifier. Exact files and lines are retained in `🧪️root-codec-bypass-followup.log`.

## Runtime-Confirmed Hex Decoder Defect

Seven inspected text roots contained the eager expression `(b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)` after a decimal-digit early return. `then_some` evaluates its argument even when the range predicate is false. The retained isolated Rust probe executes that exact expression against the standard-library digit oracle: byte33 (`!`) and byte65 (`A`) panic with overflow checks enabled, where the lowercase-hex grammar requires rejection; byte48 (`0`) and byte102 (`f`) produce the correct 0/15 controls.

The probe compiled and ran with exit0 because its assertions confirm the defect. This is not a passing production codec test. Source and `[DEBUG]` observations are retained in `🧪️root-codec-underflow/`.

## Repair Contract

Each direct leaf must own its aggregate wrap/unwrap callbacks, parser/printer and encoder/decoder. Root registries may list those callbacks and dispatch generically; neither whole-enum serde nor hand-written per-kind root match tables satisfy the ownership contract. Malformed text/binary frames must produce typed errors without panic, including punctuation, odd lengths, invalid ASCII, invalid UTF-8 and unsupported identities.

TXT's active lane has been notified of both issues. The six PDF 1.7 roots and other textual roots need their explicitly assigned codec repair waves. No out-of-scope production file was modified by this census or probe.

## Shared Runtime Route

The inspected STDIO package router forwards remaining arguments through the existing budgeted Cargo test helper. That helper separates Cargo build flags from assertion filters at `--`. The last full STDIO test-build transcript in this ticket reported 116 library errors and 258 test-library errors; the subsequent successful shared `cargo check --lib` closed the library errors only, not test-library compilation. Demonstrator runtime checks likewise do not execute STDIO's own test library. The next coherent source checkpoint therefore needs the registered STDIO test target, not an inference from a dependent library build. Existing failing transcripts are preserved in `🧪️textual-base-direct/`.
