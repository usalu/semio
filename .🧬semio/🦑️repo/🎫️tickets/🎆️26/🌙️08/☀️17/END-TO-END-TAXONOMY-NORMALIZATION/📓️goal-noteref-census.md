# reference-syntax-unsupported — 35-row census (my slice)

Source: `$T/🗑️temp/🔣️note1.json` `.unresolved`, filtered `code == "reference-syntax-unsupported"`.
Grouped by `message` prefix (the scanner family that produced the row), one row per file listed once.

## `rust-path-join-unproven` (23 rows) — `.join(literal)` with no proven `CARGO_MANIFEST_DIR` base
- `.🧬semio/…/w18-mutation-fixture-completeness/🏗️vector-converter/src/main.rs` (14) — **correct refusal**.
  `repo` is `env::args().nth(1)` (CLI arg), `target`/`tests` come from `fs::read_dir` walks. No
  `CARGO_MANIFEST_DIR` in the file at all. Genuinely unprovable at scan time.
- `✏️s/…/🎨️svg/…/quick-xml-svg-codec/src/main.rs` (2) — **correct refusal**, same shape:
  `Path::new(out_dir).join(id)`, `out_dir` from `fn main` CLI args.
- `✏️s/…/🖊️dxf/…/engine/src/main.rs` (2) — **correct refusal**, same shape, confirmed no
  `CARGO_MANIFEST_DIR` in file.
- `🧰️framework/…/💻️os/…/🗣️dsl/✨️derive/🦀️component.rs` (5) — **correct refusal**. All 5 are
  `fn materialize(...)` test-fixture helpers; base is `fixture_workspace()` →
  `env::var_os("SEMIO_TEST_ARTIFACT_DIR") ?? std::env::temp_dir()` — a runtime ephemeral tempdir, not
  a repo path.

## `rust-path-join` (8 rows) — proven `CARGO_MANIFEST_DIR` base, ownership proof never completed
- `✏️s/…/📄️pdf/…/✳️a/🧬️schema/🧬️mutations/🦀️.rs` (4) and `…/✳️x/…` (4) — **fixed, newly REWRITABLE**
  (was: detected + unresolved → now: detected + resolved with `rewriteKind: "rust-path-join"` and one
  proven `physicalTargets` entry each, verified directly against the real repo tree). Root cause:
  the ancestor `🧬️schema/🦀️component.rs` file tripped `rustFiniteManifestTargets`'s `/[#!]/` ancestor
  trust scan on two DIFFERENT `!` shapes the guard's textual proxy couldn't tell apart from a macro
  invocation bang: `vec![...]` (✳️a) and prefix negation `if !(page.width > 0.0 && …)` /
  `!=` (✳️x). Neither can expand to or hide a `mod`.

## `rust unsupported-path-syntax` (2 rows) — whole-token candidates from the generic fallback scanner
- `✏️s/…/🗒️note/…/✏️editor/🦀️component.rs:198` — self-reference `owner_file: "<own path>"` inside
  `bounded_first_step_tool_proofs!` (36 plugin editor files use this exact field). **Fixed, newly
  DETECTED+REWRITABLE**: no existing rust detector saw a bare literal outside `.join()`/`#[path]`.
- `✏️s/…/🗒️note/…/🧪️oracle/🦀️component.rs:31` — `@see ../🧪️oracle/🔣️.json` (bare, no backticks) vs
  the SAME file's own lines 3/21 using backtick-quoted `` `../🧪️oracle/🔣️.json` `` for the identical
  target. **Fixed via content edit** (added backticks) — no scanner change; already-supported
  `rust-comment-path` form now matches.

## `typescript unsupported-path-syntax` (2 rows) — same fallback scanner, TS adapter
- `✏️s/…/📸️snapshot/📝️text/🟦️component.ts:6` and `…/🧬️mutations/📝️text/🟦️component.ts:3` — same
  doc-comment sentence in both files, path split across a JSDoc line-wrap AND not backtick-quoted.
  **Fixed via content edit** (rewrapped onto one line, added backticks) — matches the existing,
  already-supported `typescript-comment-block-path` form (single-line, backtick-quoted) used
  elsewhere in both files; no scanner change.

## Total: 23 correct refusals (untouched) + 12 fixed (8 scanner-family, 4 content-only) = 35.
