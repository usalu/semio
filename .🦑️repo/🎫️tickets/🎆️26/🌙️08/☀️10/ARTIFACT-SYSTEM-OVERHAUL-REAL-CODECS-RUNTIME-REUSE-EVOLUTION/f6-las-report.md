# F6 — las (standard 1.0) — OpText/OpBinary + DiffCodec

**Artifact**: `☁️las`, standard `1.0`. **Scope note**: this artifact was **missed entirely** by the
F6 recon's §8 classification sweep (`f6-recon-report.md`) — it does not appear in that table at
all. This session did STEP 1 classification for both sides completely from scratch, per the
recon's §9 procedure, with no prior hint to trust or distrust. **Flagging this explicitly for the
closer**: the recon report's gap is now filled; no other agent should re-do this artifact's
classification.

## Result summary

| Side | Path | Reason |
|---|---|---|
| `LasDiff` (Diff) | **HAND-ROLL** | 3b tri-state (`LasPointDiff::gps_time`/`rgb`) + a bare-tuple `DslField` gap (see below) |
| `LasMutation` (Mutation) | **HAND-ROLL** | bare-tuple `DslField` gap, both directly (`SetScaleAndOffset`/`SetBounds`) and transitively via `SetSnapshot` → `LasSnapshot` → `LasPoint::rgb` |

Both sides landed on hand-roll for the SAME root cause family as the recon's documented 3b (a
missing blanket impl in the `dsl` crate), but for a type shape the recon never named: **bare
tuples**, not `Option<Option<T>>`. Full detail in §1 below.

## 1. STEP 1 — real classification (compiler-verified, not guessed)

### 1a. Diff side

Temporarily added `dsl::DslDiff` to `LasDiff` plus `dsl::DslRecord` to every nested diff struct
(`LasVlrDiff`, `LasVlrModified`, `LasVlrAdded`, `LasVlrsDiff`, `LasPointDiff`, `LasPointModified`,
`LasPointAdded`, `LasPointsDiff`), ran `cargo check -p semio-s-plugin-stdio --lib`, read the real
errors, then reverted every probe edit. Two independent, confirmed compile errors on
`LasPointDiff`:

```
error[E0277]: the trait bound `std::option::Option<f64>: DslField` is not satisfied
   --> …/🔺️diff/component.rs:311:26
311 |     pub gps_time: Option<Option<f64>>,
    |                          ^^^^^^^^^^^ the trait `DslField` is not implemented for `std::option::Option<f64>`

error[E0277]: the trait bound `std::option::Option<(u16, u16, u16)>: DslField` is not satisfied
   --> …/🔺️diff/component.rs:313:21
313 |     pub rgb: Option<Option<(u16, u16, u16)>>,
    |                     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `DslField` is not implemented for `std::option::Option<(u16, u16, u16)>`
```

The `gps_time` error is the recon's documented 3b (tri-state) exactly. The `rgb` error is
**doubly** blocked: it's tri-state (3b) AND its inner type is a bare tuple `(u16, u16, u16)` — even
a single-layer `Option<(u16, u16, u16)>` would fail the same way, tri-state or not (see §1c).

Full probe output kept at `f6-las-diff-derive-probe-check.txt` in this folder.

### 1b. Mutation side

Separately (diff-side probe reverted first) added `dsl::DslOps` to `LasMutation`, ran `cargo
check` again. Confirmed real, independent blockers:

```
error[E0277]: the trait bound `(f64, f64, f64): DslField` is not satisfied
```
— 4 occurrences, at `SetScaleAndOffset::scale`, `SetScaleAndOffset::offset`, `SetBounds::max`,
`SetBounds::min` (all `(f64, f64, f64)` tuple-typed fields declared directly on `LasMutation`
variants).

```
error[E0277]: the trait bound `las::…::LasSnapshot: DslField` is not satisfied
error[E0277]: the trait bound `las::…::LasVlr: DslField` is not satisfied
error[E0277]: the trait bound `las::…::LasPoint: DslField` is not satisfied
```
— `SetSnapshot { snapshot: LasSnapshot }` fails transitively even before reaching the tuple issue
(no `DslRecord` on the nested types), and even if those cascading derives were added, `LasPoint`
itself would still fail on `rgb: Option<(u16, u16, u16)>` — the same bare-tuple gap, single-layer
this time (not tri-state, since a mutation's `Option<T>` means "the new value", never a diff
tri-state — confirming the recon's §3 prediction that mutation payloads "rarely" hit 3b, exactly as
predicted, but this artifact hits the sibling tuple gap instead).

Full probe output kept at `f6-las-mutation-derive-probe-check.txt` in this folder.

### 1c. A third blocker class, beyond the recon's 3a/3b taxonomy

The recon's §3 names exactly two derive-blocking shapes: 3a (data-carrying enum, no `DslField`)
and 3b (`Option<Option<T>>` tri-state, no `impl<T: DslField> DslField for Option<T>`). This
artifact's `rgb: (u16, u16, u16)` / `scale: (f64, f64, f64)` fields expose a **third, structurally
identical** gap: there is no blanket `impl<...> DslField for (A, B, ...)` for tuples anywhere in
the `dsl` crate either (confirmed by the same real compiler errors above — grep of the crate
independently corroborates this: only `Vec<T>`, `BTreeMap<String, T>`, `[T; N]`, and the scalar
primitives have `DslField` impls). Same root cause as 3b (a missing blanket impl), different type
shape (product tuple instead of nested `Option`). Noting this for whichever future agent maintains
`f6-recon-report.md` or the `dsl` crate's `DslField` impl surface — a blanket tuple impl (up to
some reasonable arity) would close both `LasDiff`'s `rgb` field and `LasMutation`'s
`scale`/`offset`/`max`/`min` fields, i.e. this whole artifact's hand-roll requirement, in one
framework-level change. Not attempted here (framework-level change, out of this artifact's
ownership boundary, matches the recon's own restraint around `dsl_registry`/shared-file changes).

## 2. STEP 2b — hand-rolled implementations

### 2a. `LasDiff` — `protocol::DiffCodec`

Added to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`,
region `🔖️HandcraftedDiffCodec`. Follows the recon's §5 template exactly (copied and adapted from
gif 89a's own hand-rolled codec, the ticket's explicitly-flagged sibling reference):

- Primitives: `hex_encode`/`hex_decode`, `parse_u8`/`parse_i8`/`parse_u16`/`parse_u32`/`parse_usize`/
  `parse_f64` (new — gif 89a had no f64 fields), `split_top_level` (bracket-depth-aware, tracks
  `[`/`]`), `strip_brackets`, `encode_option`/`decode_option` (uniform `[0]`=None/`[1,<v>]`=Some tag,
  used both for `LasDiff`'s plain optional-scalar tokens' inner values where needed and for
  `LasPointDiff`'s two real tri-states).
- Value codecs: `enc_rgb`/`dec_rgb` (the `(u16,u16,u16)` tuple, positional `[r,g,b]`),
  `enc_u32x5`/`dec_u32x5` (the `points_by_return: [u32; 5]` array), `enc_vlr`/`dec_vlr` (positional
  `[user_id_hex,record_id,description_hex,data_hex]`), `enc_point`/`dec_point` (positional 14-field
  tuple, `gps_time`/`rgb` wrapped via `encode_option`/`decode_option`).
- Diff-value codecs: `enc_vlr_diff`/`dec_vlr_diff` (single-letter tags `U`/`R`/`N`/`X`),
  `enc_point_diff`/`dec_point_diff` (single-letter tags `X`/`Y`/`Z`/`I`/`R`/`N`/`D`/`E`/`C`/`A`/`U`/
  `P`/`G`/`B` — `G`=`gps_time`, `B`=`rgb`, both tri-state via `encode_option`/`decode_option`),
  `enc_collection_triple`/`dec_collection_triple` (the generic `name{[removed];[modified];[added]}`
  shape, byte-identical to gif 89a's copy — own copy per artifact, per the recon's documented
  "known duplication, not fixed here" note), instantiated for `vlrs`/`points`.
- Top level: `print_las_diff`/`parse_las_diff` — space-separated `field-name=value` tokens (kebab
  field names, e.g. `version-major=`, `points-by-return=`, `x-scale=`), absent token = unchanged.
  Every `LasDiff` top-level field is a **plain** `Option<T>` (not tri-state — no `LasHeader` field
  is itself optional), so no `[0]`/`[1,...]` wrapper is needed at the top level, unlike gif 89a's
  `gct`/`loop_count`. `vlrs{...}`/`points{...}` sections use the collection-triple shape.
- `protocol::DiffCodec for LasDiff`: `encode_diff`/`decode_diff` = `print_diff()`/`parse_diff()`
  bytes verbatim — same simplification `WriterDiff`/gif 89a/svg all use, satisfies every `DiffCodec`
  law (round-trips, deterministic, no `\n`) without inventing a denser wire format.

Real captured `print_diff` output (from the new roundtrip test, exercising both point tri-states +
both collection triples at once via a real `between()` call — see the test for full values):
non-empty diffs correctly print every changed header scalar plus
`vlrs{[...];[...];[...]}`/`points{[...];[...];[...]}` sections.

### 2b. `LasMutation` — `protocol::OpText`/`protocol::OpBinary`

Added to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`,
region `OpCodecs`, replacing the pre-existing `serde_json`-based stub entirely. Reuses the diff
module's primitives/value codecs (now `pub(crate)`) via the already-imported `diff::` alias — same
intra-artifact reuse pattern svg's mutations module uses against its own diff module (per the
recon's §5 note), no new cross-artifact sharing.

- `enc_snapshot`/`dec_snapshot` + `enc_header`/`dec_header` (new — `LasDiff` never needed a
  whole-snapshot or whole-header codec; only `SetSnapshot`'s payload does). Positional:
  `[<25-field header>,[<vlr>,...],[<point>,...]]`.
- `enc_f64x3`/`dec_f64x3` (new — the bare-tuple mutation fields; positional `[a,b,c]`).
- Top level: `print_las_mutation`/`parse_las_mutation` — `keyword arg=value ...`, one token per
  argument (never elided — a mutation's arguments are never "unchanged", unlike a sparse diff).
- `protocol::OpText for LasMutation`: thin wrapper over `print_las_mutation`/`parse_las_mutation`.
- `protocol::OpBinary for LasMutation`: `encode_op`/`decode_op` = the same text bytes verbatim
  (same simplification as `DiffCodec::encode_diff` above — the recon's §5 explicitly sanctions this
  for `OpBinary` too, "satisfies every LAW... without inventing a second wire format").

One implementation bug caught and fixed during development: the first draft of `OpBinary::encode_op`/
`decode_op` called `self.print_op()`/`Self::parse_op()` (the sibling `OpText` trait's methods) from
inside the `OpBinary` impl block without that trait in scope — `cargo check` correctly rejected this
(`no method named 'print_op' found for &LasMutation`). Fixed by calling the underlying free
functions `print_las_mutation`/`parse_las_mutation` directly instead of going through the trait.

## 3. STEP 3 — tests (both mandatory laws added)

- `diff_codec_text_binary_roundtrip_law` (new, in the diff module's test suite — this module had
  **zero** tests before this session, unusual among the artifacts already touched by prior F-waves;
  added a full `#[cfg(test)] mod tests` block). Three cases: `LasDiff::default()`, a real
  `between(a, b)`, and its reverse `between(b, a)`, where `a`/`b` differ in every header scalar, both
  `vlrs`/`points` collection triples (`removed`/`modified`/`added`), and both `LasPointDiff`
  tri-states (`gps_time` going `Some(1000.0) -> None`, `rgb` going `None -> Some((10,20,30))`).
  Asserts `!printed.contains('\n')`, `parse(print(x)) == x`, `decode(encode(x)) == x` for all three.
- `op_text_binary_roundtrip_law` (new, appended to the mutations module's existing test suite).
  Exercises all 15 `LasMutation` variants, including `SetSnapshot` (full header+vlrs+points
  round-trip), both bare-tuple variants (`SetScaleAndOffset`/`SetBounds`), the `[u32; 5]` array
  variant (`SetPointsByReturn`), and a point carrying both `gps_time`/`rgb` set (`InsertPoint`/
  `SetPoint`). Same three assertions per variant.

## 4. STEP 4 — verification (real, this session)

```
cargo check -p semio-s-plugin-stdio --lib
```
Clean (0 errors) after the implementation was complete. Note: mid-session, this command
transiently failed due to **other concurrent F6 sibling sessions'** in-progress edits (`zip` and
then `stl` each briefly had their own unrelated `DslField` compile errors from their own
derive-probe work) — confirmed via `git status` showing those artifacts' files modified by another
session, not touched by this one. Not this artifact's bug; resolved on its own once those sibling
sessions finished. No las-related errors appeared at any point after the implementation was
written.

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::las"
```
**23/23 passed, 0 failed** — includes both new law tests plus every pre-existing las test
(engine round-trips, mutation-diff law, inverse law, absorb law + associativity, field sweep,
codec retention law, between-roundtrip law, out-of-range no-op law). Full output:
`f6-las-scoped-test-run.txt`.

```
cargo test -p semio-s-plugin-stdio --lib
```
**1047/1047 passed, 0 failed** (whole crate — count includes other concurrent sibling sessions'
tests landed in the shared tree since the recon's 1019 baseline; zero failures, count only went up,
consistent with the ticket's requirement). Full output: `f6-las-full-crate-test-run.txt`.

## 5. Deviations from the recon's §5 grammar conventions

- Added `parse_f64`/`enc_f64x3`/`dec_f64x3` — not present in either of the recon's two worked
  examples (binary/gif 89a/svg have no `f64` fields). Relies on Rust's `f64::to_string()`/`FromStr`
  round-trip guarantee (shortest-round-tripping decimal representation), same trust boundary the
  language already gives every other numeric type's `Display`/`FromStr` pair used elsewhere in this
  grammar family.
- Added a bare-tuple codec (`enc_rgb`/`dec_rgb` for `(u16,u16,u16)`, `enc_f64x3`/`dec_f64x3` for
  `(f64,f64,f64)`) — not present in gif 89a/svg (neither artifact has bare-tuple fields). Positional
  `[a,b,...]`, same convention as struct codecs.
- Added a whole-snapshot codec (`enc_snapshot`/`dec_snapshot`/`enc_header`/`dec_header`) in the
  *mutations* file, not the diff file — `LasDiff` never embeds a whole `LasSnapshot`/`LasHeader`
  (unlike gif 89a's `GifDiff`, which also never does; this is consistent, not a deviation — only
  `SetSnapshot` needs it, so it lives where it's used).
- No `dsl(base64)` opportunities apply (hand-roll path never touches `#[dsl(...)]` attributes at
  all — those only matter on the derive path).

## 6. Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — hand-rolled `impl protocol::DiffCodec for LasDiff` (primitives, value codecs, diff-value
  codecs, collection triples, top-level print/parse), `pub(crate)` on every primitive/value codec
  the mutations module reuses, `#[cfg(test)] use protocol::DiffCodec;` import, new
  `#[cfg(test)] mod tests` with `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — hand-rolled `impl protocol::OpText for LasMutation` / `impl protocol::OpBinary for LasMutation`
  (replacing the `serde_json` stub entirely), new `enc_snapshot`/`dec_snapshot`/`enc_header`/
  `dec_header`/`enc_f64x3`/`dec_f64x3` helpers, `LasHeader` added to the top-level import (removed
  a now-redundant duplicate import inside the test module), `use protocol::{OpBinary, OpText};`
  added to the test module, new `op_text_binary_roundtrip_law` test appended to the existing test
  suite.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-las-diff-derive-probe-check.txt`,
  `f6-las-mutation-derive-probe-check.txt`, `f6-las-scoped-test-run.txt`,
  `f6-las-full-crate-test-run.txt`.

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework
crates were all read-only for this session (except the temporary, fully-reverted `dsl::DslDiff`/
`dsl::DslRecord`/`dsl::DslOps` derive-attribute probes on `LasDiff`'s own file and `LasMutation`'s
own file, used only to capture real compiler errors for STEP 1 — confirmed reverted, see §1a/§1b).
`POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) — not touched, per instructions; `las`
was never in it and does not need to be (the goal is for `bun ./📜️script.ts policy`'s
`dsl-migration/diff-completeness` check to stop flagging `LasDiff`'s file by now having a real
`DiffCodec` impl in it — not verified via a live `policy` run this session, but the mechanism is
identical to gif 89a's/svg's, which the recon report already confirmed drops out of that check the
same way).

## 7. Note for the closer

`las` was absent from `f6-recon-report.md`'s §8 classification table entirely (confirmed — grepped
the table, 31 rows, no `las` row). This report is the first real classification of this artifact
for the F6 program. Both sides landed on HAND-ROLL, for a reason (bare tuples) not previously named
by the recon's 3a/3b taxonomy — flagging for whoever eventually reconciles the recon report or
tallies the program's final derive-vs-hand-roll counts.
