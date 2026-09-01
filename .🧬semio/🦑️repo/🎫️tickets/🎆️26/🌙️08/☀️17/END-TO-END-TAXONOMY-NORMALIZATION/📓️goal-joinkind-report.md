# `rust-path-join-unproven` — `Vec<String>::join` false positives — Report

Baseline: `bb06c41f73f0122fbed315b7487428b976f99921`.

## (1) `Vec<String>::join` misread as `Path::join`

**The brief's proposed fix (receiver-rooting to the `CARGO_MANIFEST_DIR` chain) is unsound — not implemented.**
`inspectRustJoinArgumentSpans` is the catch-all for `.join(literal)` calls whose receiver
`inspectRustManifestPathReferences`'s strict walker did *not* prove rooted at
`PathBuf::from(env!("CARGO_MANIFEST_DIR"))`. That population legitimately includes real,
unrooted `Path`/`PathBuf` joins (a path passed as a fn parameter, `std::env::current_dir()`, a
struct field, …) that the strict walker is deliberately conservative about proving. Excluding
everything *not* rooted there would silently stop detecting those — the forbidden
"row disappeared because detection stopped" failure mode, and with a huge, unaudited blast
radius (the function isn't even gated on `CARGO_MANIFEST_DIR` — it scans every `.join(` in every
Rust file repo-wide).

**Correct structural discriminator (already partially implemented, extended here): receiver
TYPE, not receiver rooting.** `rustStringCollectionJoinArguments` (🔍️discovery/🟦️component.ts)
already proves a receiver is `Vec<String>` (mutually exclusive with `Path`/`PathBuf` in Rust's
type system) via constructor+push tracing. Two real, narrow gaps in that prover — both verified
against the actual blocking files, both fixed, neither weakens any existing check:

- **Gap A — explicit type annotation not trusted for non-`Vec::new()` initializers.** The `let`
  handler only recognized `let x: Vec<String> = Vec::new();`, not
  `let tokens: Vec<String> = expr.collect();` (the real `math/sampling` shape at
  `🦀️component.rs:6391`). An explicit `: Vec<String>` annotation is a *stronger* proof than
  tracing the initializer (Rust's compiler enforces it regardless of RHS shape), so it's now
  trusted standalone. Mirrored in the independent `syn`-based Rust oracle
  (`🧪️tests/🧪️rust-physical-reference-context/🦀️.rs`) — 33/33 golden cases match exactly,
  0 mismatches, verified via direct `cargo run`.
- **Gap B — `impl Trait for Type` misparsed as a `for`-loop, aborting the ENTIRE remaining file
  scan.** `visit()`'s `for`-loop branch matches the bare token `for` and `return`s (fail-closed,
  aborting all further traversal in that scope AND every enclosing scope) when it can't find
  `in`. `impl core::fmt::Display for TokenId { … }` (before line 6391 in the real file) hit this
  exactly — confirmed by instrumenting every `return` site: `bail for/if-let/while-let at
  token#86 text=for split=-1`. This is why gap A's fix alone didn't clear the real file even
  though it cleared an isolated fixture snippet. Fixed by giving `impl` its own handling (skip
  the header, jump straight to the body's `{`, same pattern already used for `fn`) so `for` is
  never reached in trait-impl position. Likely affects other files repo-wide (any file with
  `impl Trait for Type` before a genuine `Vec<String>` collection) — not quantified here, out of
  this slice's scope; worth a dedicated repo-wide replan to size.

**graph/🗣️dsl/🦀️component.rs:501 (`lines.join("\n")`) is NOT cleared — diagnosed, not fixed, on
purpose.** `lines` has no type annotation and is pushed via `render_wire_line(&value)` (an opaque
function call). The golden fixture's own `unknown-pushed-type` case
(`values.push(other()); values.join(...)` → still flagged) is an existing, deliberate,
already-passing test proving the design intentionally does NOT trust non-literal/non-`format!`
push values as proof of `String`-ness. Clearing this row would require either violating that
locked test or a new, unverified "trust this specific function's return type" mechanism
(analogous to the `RUST_MODULE_STRUCTURE_TRANSPARENT_*` macro allowlists but for a different
domain) — a judgment call left to the coordinator, not made unilaterally here.

## (2) dwg test — already resolved before this slice touched anything

`machine`, `mesh`, `action-argument-resolution` were all **already at `unresolved=0`** on first
measurement, before any change in this session — the `impl_serde_op_codec!`/`format!`/`unreachable!`
trust and `async_test` attribute-path fixes a peer session landed (visible already in
`🧹️normalization/🟦️.ts`) had already cleared the stdio `glue.rs` blocker described in the brief.
Stale by the time this slice started; confirmed live, not assumed.

## Verification (real, pasted output; `--plan` always under `🗑️temp/`)

```
🕸️graph   (before) moves=6 edits=9  unresolved=1  ← rust-path-join-unproven:501  (UNCHANGED, diagnosed not fixed)
🕸️graph   (after)  moves=6 edits=9  unresolved=1  ← same row, same digest family
🧮️math    (before) moves=2 edits=5  unresolved=1  ← rust-path-join-unproven:6391
🧮️math    (after)  moves=2 edits=5  unresolved=0  ← APPLY-READY
🔄️machine (measured) moves=5 edits=8  unresolved=0  ← APPLY-READY (already 0)
🔺️mesh    (measured) moves=1 edits=7  unresolved=0  ← APPLY-READY (already 0)
🧮️action-argument-resolution (measured) moves=1 edits=4 unresolved=0  ← APPLY-READY (already 0)
```

`🧮️math`'s `edits` array is byte-identical before/after (same 5 real `oldValue`→`newValue` pairs,
same `moves`) — only the blocking decision cleared, proving this is a genuine
proven-non-referential resolution, not a detection regression.

## Tests (fail-before / pass-after, both directions verified)

- `🧫️fixtures/🧪️rust-physical-reference-context/🔣️.json` → `joinArguments.cases`: added
  `typed-collect-initializer` (Gap A) and `trait-impl-for-precedes-collection` (Gap B), both with
  a real `rustc`-compiled `compiler`/`compilerOutput: "delimiter"` oracle. Reverted each fix in
  turn and reran `bun test … -t "string collection joins require exact standard receiver
  provenance"`: fails exactly on the new case, passes with the fix restored, both confirmed.
- Independent third-party oracle: `🧪️tests/🧪️rust-physical-reference-context/🦀️.rs` (real `syn`
  crate) mirrors the same `let`-typing logic; ran via `cargo run` directly (Bun's own
  `Bun.spawnSync`-based compiler-oracle tests in this file fail broadly and pre-existingly in this
  sandbox — confirmed on rows untouched by this change, e.g. `mutable-format-branch`, `rustc
  -Zunpretty=ast-tree` in `📦️index.test.ts:4213` — a subprocess-capture sandbox limitation, not a
  regression). 33/33 cases match the golden fixture exactly, 0 mismatches.
- 76→78 total cases in the fixture; all TS-only (non-subprocess) tests in the file pass.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` — `rustStringCollectionJoinArguments`: typed-annotation trust (Gap A), `impl` handling (Gap B).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️rust-physical-reference-context/🦀️.rs` — mirrored Gap A in the independent `syn` oracle.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️rust-physical-reference-context/🔣️.json` — 2 new golden cases.
- This file.
