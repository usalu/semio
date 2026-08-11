# Phase 2 M2 Report — Protocol Dialect + `walk_protocol` Generalization

Scope: the plan's binding "P2-W0 recon findings + orchestrator scope decisions (binding for
M1/M2)" section, **M2 items 1–6, verbatim**. Sole ownership for this wave:
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs` (the same file M1 already
edited for the grammar side — M1's edits were read from the CURRENT working-tree state, not
re-derived from W0-era line numbers, per the brief). No other file needed touching — every M2 item
lives entirely inside this file's `ProtocolModel`/`Parser`/`Writer`/`ProtocolWalk`/`Tests` regions;
the compiler forced zero changes outside it (unlike M1, which had to touch `🔤️token/🦀️component.rs`
and Jack's lexer bridge — M2 added no new token kinds, only new `Prim`/`Block` AST variants private
to this crate's own protocol pipeline).

`git status --porcelain` on `🗣️dsl`/`🎒️pack` was polled before starting and periodically throughout
the session — clean of any concurrent edits the entire time (only M1's own already-applied,
uncommitted changes were present, matching the p2-m1-report.md file list exactly). No stop-and-report
was triggered.

---

## 1. What was built, per scope item — with the exact new `.protocol.semio` syntax

### Item 1 — "Repeated tag-dispatched block" (`repeat`)

New `Block::Repeat { name, dispatch: RepeatDispatch }`. `RepeatDispatch` reads a discriminator
(+ optional length, in either order), branches into a named arm's fields or skips an unrecognized
discriminator's declared length as opaque bytes, and repeats until EOF or a declared sentinel
discriminator value (`until`). An arm may itself declare exactly one further `nested` dispatch
level (GIF 89a's two-level case), recursively (not artificially capped at two).

```
repeat <name> {
  tag <prim>                      # discriminator read (Fixed(n) ascii tag, u8/u32be numeric tag, marker(0xNN) scan)
  length <prim>?                  # optional length/count read
  order tag-first|length-first?   # default tag-first; PNG/GLB need length-first
  trailer <prim>?                 # optional trailer read after each iteration's body (PNG's crc32be)
  until <tag>?                    # optional sentinel — TEXT ("IEND") or int/hex (0x3B) literal
  arm <tag> { field1 ty1 ... }                       # known field-set
  arm <tag> { nested <name> <prim> { arm <tag> {...} arm <tag> {...} } }   # 2nd-level dispatch
  ...
}
```

Worked PNG-shaped example (length-first, ASCII tag, skip-unknown, sentinel):
```
repeat chunks {
tag fixed 4
length u32be
order length-first
trailer u32be
until "IEND"
arm "IHDR" { width u32be height u32be }
arm "IEND" { }
}
```
Worked GIF89a-shaped example (tag-first, byte tag, two-level nested dispatch, byte sentinel):
```
repeat blocks {
tag u8
until 0x3B
arm 0x2C { left u16 top u16 }
arm 0x21 {
nested label u8 {
arm 0xF9 { flags u8 delay u16 }
arm 0xFE { }
}
}
arm 0x3B { }
}
```
JPG's marker-prefix-scan variant mode uses `tag marker(0xFF)` instead of a fixed-position read (see
item 1c below) — same `repeat`/`arm`/`until` shape otherwise.

**(a) unknown/unrecognized discriminator → skip-as-opaque**: implemented (`walk_repeat`'s `None =>`
arm skips exactly `length` bytes when the discriminator matches no `arm`; errors if no `length` was
declared, since there is then no honest way to know how much to skip).

**(b) two-level nesting**: implemented via `NestedDispatch` (an arm's `nested` field), proven by a
real GIF89a-shaped synthetic test with a corrupted-label negative case.

**(c) marker-prefix scanning**: implemented as a distinct `Prim::MarkerScan(u8)` discriminator kind
(`marker(0xFF)`) rather than folding it into the general discriminator read — it genuinely scans
forward past fill bytes rather than reading a fixed-position value, exactly the "variant mode" the
plan anticipated.

### Item 2 — Big-endian `Prim` variants

Added `U16Be, U32Be, U64Be, I32Be, I64Be, F32Be, F64Be` as **always-big-endian-regardless-of-runtime-mode**
siblings of the existing LE-hardcoded `U16/U32/U64/I32/I64/F32/F64` — syntax is the lowercase name
plus `be`: `field length u32be`, `field checksum u64be`. Deliberately distinct from item 6: these are
a static, author-time choice baked into the `.protocol.semio` text (png/jpg/deflate's trailer/ply's
`binary_big_endian` variant/pdf 1.7's xref-stream rows all use these), never affected by a runtime
endian-marker field. `prim_fixed_width`, `print_prim`, `parse_prim`, `walk_prim`, `walk_fields`, and
the new `read_raw_prim_bytes`/`read_scalar_prim` discriminator/length helpers all gained the 7 new
arms (same widths as their LE counterparts, decode always via `from_be_bytes`).

### Item 3 — Cross-block field-env threading

`walk_fields` no longer creates a fresh per-call-local `HashMap` (the old, block-scoped behavior).
`walk_protocol` now owns ONE `WalkState { env, big_endian }`, created once before the block loop and
threaded by `&mut` through every block-walking call (`Header`/`Segment`/`Record`/`Repeat`/
`BackwardScan`/`JumpTo`, and recursively into `walk_prim`/`walk_fields`/`walk_repeat`/
`walk_nested_dispatch`). No new syntax — `Count::Field(name)` and `Cond{field}` (item 4) now simply
resolve against whatever has been decoded by ANY earlier block, not just the current one. Proven by
a synthetic las-VLR-shaped test: a `header`'s `count` field consumed by a *separate, later*
`segment`'s `Array(u8, Field(count))`, plus a negative case (wrong count → truncation error) proving
the value is genuinely read, not defaulted.

### Item 4 — Conditional field/segment presence (`if`)

New `Cond { field: String, op: CondOp, value: u64 }` (`CondOp` = `Eq|Ne|Lt|Le|Gt|Ge`), attachable to
any `Field` (guards that one field's presence) and to a whole `Block::Segment` (guards the entire
segment). Word-keyword operators (`eq`/`ne`/`lt`/`le`/`gt`/`ge`) rather than symbolic `==`/`<=` —
this file's own local protocol lexer (the `lex()`/`GKind` pair at the top of the file, shared by both
`parse_grammar` and `parse_protocol`) only ever whitelists a small fixed token set and would need a
lexer change to accept `<`/`<=`/`==` as protocol-body tokens; word keywords needed zero lexer changes
and stay entirely inside this file's own parser, so this was the lower-risk choice for a framework
file every plugin depends on.

```
field mask u32 if compression eq 3           # field-level guard (bmp's BITFIELDS masks)
segment palette if bits_per_pixel le 8 { colors u8 }   # whole-segment guard (bmp's palette)
```
A guarded field/segment whose condition evaluates false is skipped entirely — not read, not present
in the walk at all. Proven by a bmp-shaped synthetic test exercising both forms, with present AND
absent buffers, plus a negative case (a too-short buffer for the "absent" shape fails against the
"present" shape's byte demands, proving presence genuinely changes consumption).

### Item 5 — ZIP backward-seek + offset-pointer resolution

**(a) Backward-scan framing** — new `Block::BackwardScan { name, magic: Vec<u8>, fields }`:
```
backward <name> magic 0x<hex> { field1 ty1 ... }
```
`magic`'s hex literal encodes to its big-endian byte pattern trimmed of leading zero bytes (reusing
`framing magic`'s existing convention, generalized to non-fixed-8-byte widths). At walk time,
`find_last_occurrence` scans **backward** from EOF for the rightmost match of those exact bytes,
jumps `pos` directly past the match, then walks `fields` forward from there — genuinely locating a
structure (ZIP's EOCD) whose start is unknowable except by finding its end/magic first, exactly the
W0 recon's own reasoning.

**(b) Offset-pointer jump** — new `Block::JumpTo { name, offset_field: String, fields }`:
```
jump <name> from <field-name> { field1 ty1 ... }
```
Looks up `offset_field` in the walk-wide `WalkState.env` (must have been decoded by an EARLIER
block — this is why item 3 had to land first), sets `pos` to that ABSOLUTE value, then walks
`fields` forward from there. A genuine, deliberate, precisely-documented exception to "position only
increases" (see `walk_protocol`'s own doc comment, quoted verbatim):

> once ANY block has explicitly JUMPED `pos` (`BackwardScan`/`JumpTo`), the walk is no longer a pure
> linear forward accounting of every byte in the buffer — that is the whole point of a
> backward-scan/offset-jump... The final `pos == bytes.len()` law is therefore skipped for any walk
> that performed at least one jump; it holds EXACTLY as before for every protocol that declares
> neither block (the overwhelming majority). The walker still only ever reads FORWARD from a jump's
> landing point — jumps move `pos` directly, they never make the walker itself search or backtrack
> mid-block.

Worked ZIP-shaped example (EOCD located by backward scan, `cd_offset` used to jump to the central
directory):
```
backward eocd magic 0x504B0506 {
cd_offset u32
entry_count u16
}
jump central from cd_offset {
entry_tag u32
entry_value u32
}
```
Proven by a synthetic buffer (16 filler bytes + a central-directory-entry-shaped record + an EOCD
record whose `cd_offset` points back at that entry) plus a negative case (corrupting the EOCD magic
bytes makes the backward scan fail to find it at all), plus a print/reparse round-trip of the new
`backward`/`jump` syntax itself.

### Item 6 — TIFF-style runtime-selected endianness (`endian`)

New `Prim::Endian(Vec<(String, bool)>)`, used as an ordinary field type (no new top-level directive
needed — it composes with the existing `field <name> <ty>` surface):
```
field byte_order endian { "II"=le "MM"=be }
```
Reads `key.len()` bytes (TIFF's `II`/`MM` are both 2), matches against the declared table, and
**mutates `WalkState.big_endian`** for every subsequent PLAIN (non-`Be`-suffixed) `Prim` read for
the remainder of the walk — a walker-state mutation, not a value binding, exactly as the plan
specified ("distinct from item 2's static per-format choice"). Proven with two buffers (one `"II"`
+ LE-encoded fields, one `"MM"` + BE-encoded fields for the SAME declared `field count u16` — no
`Be` suffix — both walk successfully with the SAME protocol text, proving the marker genuinely
flips runtime behavior rather than being cosmetic), a negative case (unrecognized marker bytes →
error), and a print/reparse round trip.

---

## 2. Files touched

Sole-owned, only file touched in this wave:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs` (2367 → 3428 lines):
  - `ProtocolModel` region: `Cond`/`CondOp`, `RepeatDispatch`/`DispatchOrder`/`RepeatArm`/
    `NestedDispatch`, `Block::{Repeat,BackwardScan,JumpTo}`, `Block::Segment` gained `cond`,
    `Field` gained `cond`, `Prim` gained `U16Be/U32Be/U64Be/I32Be/I64Be/F32Be/F64Be/MarkerScan/Endian`.
  - `Parser` region: `parse_cond`, `parse_tag_value`, `trim_be_bytes`, `parse_arm_body`,
    `parse_repeat_dispatch`, new `parse_prim` arms (`u16be`.."f64be", `marker(..)`, `endian{...}`),
    `parse_field_pair` gained trailing `if` guard, `segment` directive gained `if` guard, new
    `repeat`/`backward`/`jump` top-level directives.
  - `Writer` region: `print_cond`, `print_tag_bytes`, `print_repeat_arm`, `print_prim`/`print_field`
    updated, `print_protocol` gained `Repeat`/`BackwardScan`/`JumpTo`/segment-`cond` printing.
  - `ProtocolWalk` region: new `WalkState` (walk-wide `env` + `big_endian`), `decode_u16/u32/u64`,
    `eval_cond`, `read_raw_prim_bytes`, `read_scalar_prim`, `find_last_occurrence`,
    `walk_nested_dispatch`, `walk_repeat`; `walk_prim`/`walk_fields`/`walk_protocol`/
    `prim_fixed_width`/`trailing_reserved` all updated to thread `WalkState` instead of a
    per-call-local `HashMap<String,u64>` and to cover the new `Prim`/`Block` variants.
  - `Tests` region: new `//#region 🔖️P2M2Protocol` block, 8 new tests (below).

**Compiler-forced outside this file: none.** Confirmed by `cargo check --workspace` — the only
errors present both before and after this wave are the pre-existing/unrelated ones logged in §3
below; nothing newly broke in any dependent crate.

---

## 3. Gate results (real output, pasted, not paraphrased)

### Gate 1 — `cargo check --workspace`

```
$ cargo check --workspace 2>&1 | grep -E "^error" | sort | uniq -c | sort -rn
  17 error[E0433]: cannot find module or crate `dsl` in this scope
   2 error[E0433]: cannot find module or crate `vcs` in this scope
   2 error: cannot find attribute `dsl` in this scope
   1 error[E0599]: no method named `contributes` found for struct `component::app::Plugin` in the current scope
   1 error[E0432]: unresolved import `vcs`
   1 error[E0432]: unresolved import `semio_framework::Contribution`
   1 error: couldn't read `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/../../📄️document/🦀️component.rs`: No such file or directory (os error 2)
   1 error: could not compile `semio-framework-plugin` (lib) due to 2 previous errors; 36 warnings emitted
   1 error: could not compile `semio-framework-os-kernel-db` (lib) due to 1 previous error
   1 error: could not compile `semio-compose-rs` (lib) due to 22 previous errors; 823 warnings emitted
```

Two of these are **exactly M1's two pre-existing, confirmed-unrelated issues** (`🛢️db`'s missing
`📄️document` module file; `semio-compose-rs`'s bare `dsl`/`vcs` crate-name references — both
untouched by any dsl/grammar/protocol file, unchanged since M1's report).

**A third, new-since-M1 issue was found: `semio-framework-plugin` fails with
`E0432 unresolved import semio_framework::Contribution` + `E0599 no method named contributes`.**
This is **not caused by this wave** — confirmed by `git status --porcelain`, which shows it as a
**live, currently-uncommitted (`MM`) concurrent session's in-progress edit**:
```
$ git status --porcelain -- "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
MM 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs
MM 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs
```
`🔌️plugin` is a completely different module from `🗣️dsl`/`🎒️pack` (this wave's sole ownership) and
was never touched by this session. The error is a `Contribution` type/`.contributes()` method being
introduced mid-refactor by another live session and not yet fully wired — matches the repo's
documented "Concurrent Cargo Workspace Churn" pattern exactly. Spot-checked `semio-framework-math`
(clean, `cargo check -p semio-framework-math` → `Finished`) as an independent large consumer
unaffected by the plugin churn.

### Gate 2 — `cargo test -p semio-framework-os-kernel`

```
$ cargo test -p semio-framework-os-kernel 2>&1 | tail -12
failures:
    os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::dag_dsl_grammar_recognizes_shipped_fixture_tokens
    os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::en1992_dsl_grammar_recognizes_shipped_fixture_tokens
    os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::fem2d_dsl_grammar_recognizes_shipped_fixture_tokens
    os_dsl::fixture_sweep::m5_production_coverage::dag_reports_uncovered_productions_for_shipped_fixture
    os_dsl::fixture_sweep::m5_production_coverage::en1992_reports_uncovered_productions_for_shipped_fixture

test result: FAILED. 768 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
```
**Exactly the same 5 pre-existing test names** as M1's exit state and the original W0 baseline — not
more, not different. `768 = 760 (M1's exit count) + 8` — the 8 new tests this wave added, confirmed
individually:
```
$ cargo test -p semio-framework-os-kernel --lib grammar:: 2>&1 | tail -3
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 732 filtered out; finished in 0.04s
```
(33 pre-existing `grammar::` tests from M1's baseline + 8 new P2-M2 tests = 41.)

Pilot-specific confirmation (`lowpoly`/`cad`/`note` — the plan's real clean regression gate — still
green; `dag`/`en1992`/`fem2d` still red, exactly the documented pre-existing baseline, unchanged by
this wave):
```
$ cargo test -p semio-framework-os-kernel --lib m5_handcrafted_grammar_conformance 2>&1 | tail -3
test result: FAILED. 3 passed; 3 failed; 0 ignored; 0 measured; 767 filtered out; finished in 0.04s
```
Protocol-conformance harness (unaffected by this wave — no fixture/harness files touched):
```
$ cargo test -p semio-framework-os-kernel --lib m5_handcrafted_protocol_conformance 2>&1 | tail -3
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 766 filtered out; finished in 0.00s
```

### Gate 3 — `cargo test -p semio-s-plugin-stdio --lib`

**Blocked by the same live, unrelated `🔌️plugin` concurrent churn identified in Gate 1** — stdio
depends on `semio-framework-plugin`, which currently fails to compile for reasons entirely outside
this wave's ownership (`🔌️plugin`, not `🗣️dsl`/`🎒️pack`). Retried repeatedly across the session
(including a background retry after several minutes of other work) with the identical error every
time, confirming this is a real, currently-still-in-progress external session, not transient
build-cache flakiness:
```
$ cargo test -p semio-s-plugin-stdio --lib 2>&1 | tail -6
error[E0432]: unresolved import `semio_framework::Contribution`
error[E0599]: no method named `contributes` found for struct `component::app::Plugin` in the current scope
error: could not compile `semio-framework-plugin` (lib) due to 2 previous errors; 35 warnings emitted
```
**Cannot honestly report a passing 1075/0 (or higher) number for this gate right now** — that would
require claiming a test run that never completed. What CAN be reported, as indirect-but-strong
evidence of no regression:
1. `🗣️dsl`/`🎒️pack` stayed `git status --porcelain` clean of anyone else's edits the entire session
   (only this wave's own changes present) — the blocker is provably in a different module.
2. This wave changed zero PUBLIC function signatures on the dsl facade (`parse_grammar`,
   `parse_protocol`, `print_grammar`, `print_protocol`, `walk_protocol`, `Recognizer`,
   `verify_protocol_source`, `verify_protocol_bytes` all keep their exact pre-M2 signatures) — only
   internal AST variants and private walker helpers changed.
3. Per M1's report (confirmed unchanged by this wave): "nothing in stdio uses these new mechanisms
   yet" — stdio's 31 standards don't call `walk_protocol`/`parse_protocol` with any of the new
   `Block`/`Prim` variants at all, so even a hypothetical bug in the new code paths could not
   manifest in stdio's existing test suite.
4. `semio-framework-os-kernel`'s full suite (which DOES exercise every changed code path directly,
   Gate 2 above) is clean at 768/5, the exact expected count.

A later retry (after the `🔌️plugin` compile error above cleared on its own) got further but hit a
**different, much larger** unrelated wall — 68 errors, all `E0425 cannot find type/function` for
dozens of unrelated artifact types (`WavRawChunk`, `SemioMesh`, `Slide`, `WorkflowNode`,
`Mp4RawBox`, `Id3v2Header`, ...) with zero connection to grammar/protocol/dsl. `git status
--porcelain` (full repo, not scoped) shows **262 modified/added files spanning dozens of unrelated
plugins** (writer, mathematical, procedural3d, flow + 7 extensions, gis, vcs, animate, shooting, ...)
— a large, still-in-progress, repo-wide concurrent refactor on a different ticket, exactly the
documented "Concurrent Cargo Workspace Churn" pattern, just a bigger instance of it than the single
`🔌️plugin` file seen earlier in the session. This gate should be re-run by whoever picks this ticket
up next once that churn settles; flagging it explicitly with full real evidence rather than
fabricating a result, per the repo's own "never say a test passed without running it" rule.

### Gate 4 — new unit tests per scope item

8 new tests, all passing, in `🗣️dsl/📖️grammar/🦀️component.rs`'s own `#[cfg(test)]` module, new
`//#region 🔖️P2M2Protocol`:

| item | test |
|---|---|
| 1 (repeat, skip-unknown, sentinel) | `repeat_block_dispatches_png_shaped_chunks_and_skips_unknown_type` |
| 1b (two-level nested dispatch) | `repeat_block_two_level_nested_dispatch_gif89a_shaped` |
| 1c (marker-prefix scan) | `marker_scan_prim_finds_next_marker_byte_over_fill_bytes_jpg_style` |
| 2 (BE prims) | `be_prim_variants_round_trip_and_decode_big_endian_for_real` |
| 3 (cross-block field env) | `cross_block_field_env_threads_header_field_into_a_later_segment_las_vlr_style` |
| 4 (conditional presence) | `conditional_field_and_segment_presence_gate_on_an_earlier_field_bmp_style` |
| 5 (backward-scan + jump) | `backward_scan_and_jump_to_resolve_zip_eocd_and_central_directory_offset` |
| 6 (runtime endian marker) | `endian_marker_field_switches_runtime_byte_order_for_the_rest_of_the_walk_tiff_style` |

Each test builds a real synthetic byte buffer (not a mock), asserts `walk_protocol` succeeds with
the exact expected `consumed` position, AND includes at least one negative case proving the
mechanism is doing genuine work (truncation errors, corrupted magic/label bytes, wrong-endianness
byte counts) rather than vacuously succeeding. Items 2, 5, and 6 additionally assert a print/reparse
round trip of the new syntax itself.

```
$ cargo test -p semio-framework-os-kernel --lib grammar:: 2>&1 | grep -E "P2M2|test result"
test os_dsl::grammar::tests::backward_scan_and_jump_to_resolve_zip_eocd_and_central_directory_offset ... ok
test os_dsl::grammar::tests::be_prim_variants_round_trip_and_decode_big_endian_for_real ... ok
test os_dsl::grammar::tests::conditional_field_and_segment_presence_gate_on_an_earlier_field_bmp_style ... ok
test os_dsl::grammar::tests::cross_block_field_env_threads_header_field_into_a_later_segment_las_vlr_style ... ok
test os_dsl::grammar::tests::endian_marker_field_switches_runtime_byte_order_for_the_rest_of_the_walk_tiff_style ... ok
test os_dsl::grammar::tests::marker_scan_prim_finds_next_marker_byte_over_fill_bytes_jpg_style ... ok
test os_dsl::grammar::tests::repeat_block_dispatches_png_shaped_chunks_and_skips_unknown_type ... ok
test os_dsl::grammar::tests::repeat_block_two_level_nested_dispatch_gif89a_shaped ... ok
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 732 filtered out; finished in 0.04s
```

---

## 4. Deviations from the plan's exact scope, and why

1. **ZIP per-entry local-header jump (item 5's "second jump") scoped DOWN, per the plan's own
   explicit latitude** ("you may scope this second jump as 'recommended but not blocking'... document
   your choice either way"). What was built: ONE general, reusable `Block::JumpTo` primitive (not
   hardcoded to EOCD→central-directory specifically — any protocol file can declare a `jump <name>
   from <field> {...}` block against any earlier-decoded field name), demonstrated end-to-end for
   the EOCD→central-directory-start jump. What was NOT built: a *repeated, per-central-directory-entry*
   jump construct — real ZIP has potentially many central-directory entries, each with its own
   `local_off` pointer requiring a jump PER ENTRY, and my `RepeatArm`'s field list (item 1's
   construct) does not currently allow a `jump` sub-directive inside an arm body — only plain
   `Field`s and one `nested` dispatch. Composing "repeat over central-directory entries" with "jump
   per entry to that entry's local header" would need arm bodies to support an embedded `JumpTo` or
   an analogous per-iteration jump, which this wave did not add. This matches the plan's own
   assessment ("central-directory entries alone already carry the real per-entry metadata" — the
   single EOCD→central-dir jump plus a `repeat` over entries, without the second jump, already gets
   FG2's zip agent everything needed for real per-entry fields). Flagged here explicitly as an open
   extension point for FG2 or a future M-fix, not silently dropped.
2. **`if`-guard operators are word-keywords (`eq`/`ne`/`lt`/`le`/`gt`/`ge`), not symbolic
   (`==`/`<=`/etc.)** — not specified verbatim by the plan (which only named the mechanism:
   "`if <field-name> == <value>`"). Chosen because this file's own local protocol/grammar lexer (the
   `lex()`/`GKind` pair, shared by BOTH `parse_grammar` and `parse_protocol`, pre-dating M1/M2) only
   ever whitelists a small fixed token set and returns a hard `Err` on anything outside it — using
   symbolic operators would have required extending that lexer's alphabet (a framework-wide,
   whole-repo-blast-radius change touching a file every plugin's grammar/protocol source passes
   through) purely for cosmetic syntax preference. Word keywords needed zero lexer changes, stay
   entirely inside this file's own recursive-descent parser, and are exactly as expressive.
3. **`RepeatDispatch`'s nesting is genuinely unlimited depth** (`RepeatArm.nested: Option<NestedDispatch>`,
   `NestedDispatch.arms: Vec<RepeatArm>` recursively), not capped at exactly the plan's stated
   "two levels" — this fell out of the natural Rust type shape (`Vec` already provides the
   indirection recursion needs, no `Box` required) at zero extra implementation cost, so it was kept
   general rather than artificially restricted. GIF 89a's real 2-level need is fully covered; a
   hypothetical 3rd level (not needed by any of the 32 standards per W0's census) would also work
   without further changes.
4. **`marker(0xNN)` (item 1c) implemented as a genuinely separate `Prim::MarkerScan` variant**, not
   folded into the general discriminator-read path — the plan itself flagged this as "a variant mode
   if the core construct doesn't naturally cover it," and a fixed-position read (`Fixed(n)`/`u8`/etc.)
   structurally cannot express "scan forward past an unbounded run of fill bytes," so a distinct
   `Prim` was the honest choice rather than overloading an existing one.

No scope item was skipped or narrowed beyond what's documented above. The explicitly-out-of-scope
items (DWG ac1024's decrypt/decompress pipeline, PDF/1.7's full indirect-object graph, cross-dialect
grammar→protocol field-width parameterization) were not touched.

---

## 5. What this changes about the M3 / FG-wave picture

- **`Prim::Ref` resolution against local `Struct`/`Enum` blocks remains unresolved** (still a hard
  `walk_prim` error) — this was in the W0 census's candidate list but explicitly NOT in M2's final
  "DECIDED" scope, so it was correctly left untouched. Any FG-wave standard whose real layout needs
  a named-struct reference walked (rather than just parsed into the AST and ignored, as today) will
  hit this — worth flagging for M3's own scope review or a future M-fix if ≥2 FG-wave standards need
  it (per the plan's own "M-fix iteration" trigger threshold).
- **Composing `repeat` with `jump`/`backward` is not yet possible** (see §4 item 1) — an arm body
  can contain fields and one nested dispatch, but not an embedded absolute-offset jump. ZIP's own
  central-directory-ENTRIES (plural, needing `repeat`) combined with each entry's OWN `local_off`
  jump is the concrete case that would need this; flagged for FG2's zip agent to hit this boundary
  concretely and decide whether it needs an M-fix or can stay honestly scoped down (per §4 item 1's
  reasoning, the single EOCD jump plus a plain `repeat` over central-directory entries — without
  per-entry local-header cross-validation — already gets real per-entry metadata, which may be
  sufficient).
- **Cross-artifact `use` is still non-functional on the protocol side** (confirmed unchanged — W0's
  §0 finding that `ProtocolFile.uses` is parsed/round-tripped but never consulted by `walk_protocol`
  still holds; this wave did not touch `uses` handling at all, correctly out of M2's item list). Any
  FG4 OPC-tail standard wanting to delegate to zip's protocol productions still needs this built
  first (M3 or a dedicated M-fix), independent of everything M2 added.
- **The `Cond`/`if`-guard mechanism (item 4) and the cross-block `WalkState.env` (item 3) compose
  cleanly with `repeat`/`backward`/`jump`** (all four new block kinds share the same `walk_fields`
  entry point, so a field inside a `repeat` arm, a `backward`-scanned block, or a `jump`-target block
  can ALL carry `if` guards and read/write the same walk-wide env) — this wasn't explicitly required
  by any single item but falls out for free from the unified `WalkState` design, and should make
  FG-wave authoring more uniform than the per-item scope list might suggest (a field is a field,
  regardless of which block kind hosts it).
- **No new token kinds were added to the shared lexer** in this wave (unlike M1) — M2's entire
  surface lives in this file's own local `GKind`/parser, meaning M2 carries ZERO risk to the
  non-stdio pilot grammars or any other consumer of the shared `crate::os_dsl::lex`/`lex_with`
  machinery M1 built. This is confirmed by Gate 1's finding that the ONLY new-since-M1 workspace
  error is the unrelated `🔌️plugin` churn — nothing attributable to this wave appeared anywhere
  outside `🗣️dsl/📖️grammar/🦀️component.rs`'s own test module.
- **Baseline reproduction, per the brief's instruction**: reproduced before starting real edits —
  `cargo test -p semio-framework-os-kernel` → 760/5 (exact match to M1's exit state);
  `git status --porcelain -- 🗣️dsl 🎒️pack` → only M1's own already-committed-to-working-tree changes
  present, clean of any other session, both at start and repeatedly throughout.
