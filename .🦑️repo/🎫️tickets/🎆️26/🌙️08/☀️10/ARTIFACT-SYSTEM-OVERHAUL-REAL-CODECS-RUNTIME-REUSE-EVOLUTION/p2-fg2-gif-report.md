# P2-FG2 — gif (87a + 89a) — Real Codecs / Runtime Reuse / Evolution

Agent scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/**`, both standards `🔖️87a` and `🔖️89a` (one
agent for both, per the ticket's own Phase-1 precedent — 89a is the richer canonical standard,
87a the simpler predecessor sharing most structure).

## Summary

Both standards were found with a **fully real, hand-rolled byte-level codec engine**
(`⚙️engine/🦀️component.rs`) already landed from prior work on this ticket (GIF87a/89a magic,
Logical Screen Descriptor, GCT/LCT, LZW, sub-block chains, GCE/loop/comment/app-extension/
plain-text extensions for 89a — 89a reuses 87a's byte-level primitives via `pub` engine
functions). `OpBinary` was **already real** for both standards' mutations (confirmed by direct
reading, not assumed: both forward straight to `dsl::variants_binary::encode_op`/`decode_op`).

The one genuine F6-era gap, matching the FG1 lesson this ticket explicitly warns against
repeating, was **`DiffCodec::encode_diff`/`decode_diff`**: both standards were still on the
`print_diff().into_bytes()` text-as-binary shortcut. This program upgraded both to real,
field-by-field binary frames (`dsl::ByteWriter`/`dsl::ByteReader`, 2-way/3-way presence flags,
varint-length-prefixed opaque blobs only for the genuinely nested/recursive collection-triple
payloads) — mirroring PNG's own upgraded `PngDiff` binary frame shape exactly.

All 6 grammar/protocol facet files per standard (12 total) were found as **stale pre-Phase-2
placeholders** (a pre-2026 ABNF-shaped stub dialect, not even the real `.grammar.semio`/
`.protocol.semio` syntax) and were rewritten from scratch to the real dialect, verified against
real `print_dsl`/`print_op`/`print_diff`/`encode_pack`/`encode_op`/`encode_diff` output via 12
new conformance-law tests (6 per standard), all passing. 5-role `LanguageSpec` registration and
`register_schema_spec` were added for both standards (10 language roles total). Real
`.dsl.semio`/`.pack.semio` fixtures were generated for both standards — 87a's from a small
synthetic demo snapshot, 89a's from the **real `dancing.gif`** fixture (54 frames, 800×800) per
the ticket's own explicit instruction, for byte-real conformance.

## What was already real (confirmed, not assumed)

- **87a `⚙️engine/🦀️component.rs`** (816 lines): real LZW encode/decode, sub-block pack/unpack,
  color-table I/O, RGBA quantize, interlace de-row, full `encode_gif`/`decode_gif`.
- **89a `⚙️engine/🦀️component.rs`** (577 lines): real GIF89a codec reusing 87a's byte-level
  primitives via `pub` functions (`use crate::artifacts::gif::standards::v87a::engine as
  codec;`) — GCE, NETSCAPE2.0 loop, comment/app extensions, plain-text extension.
- **Both standards' `OpBinary`** (`🧬️mutations/🦀️component.rs`): `encode_op`/`decode_op` are
  pure forwards to `dsl::variants_binary::encode_op`/`decode_op` — a real, framework-generic
  varint-prefixed record-body wire, not `print_op().into_bytes()`. Confirmed by reading both
  bodies directly.
- Both standards' `dsl::DslOps`-derived `GifMutation` enums (12 variants for 87a, 21 for 89a).
- `GifSnapshot` derives `dsl::DslRecord` for real on both standards (used for `#[dsl(block)]`
  embedding inside mutation payloads and now for `register_schema_spec`'s real `__dsl_spec`).

## What was upgraded this program

### 1. `DiffCodec::encode_diff`/`decode_diff` — real binary frames (the critical item)

Both `🔺️diff/🦀️component.rs` files (87a: 911→~1060 lines, 89a: 1265→~1420 lines) gained a new
`RealBinaryPrimitives`/`RealBinaryDiffFrame` region pair (mirroring PNG's own upgraded
`PngDiff` shape field-for-field) and the `DiffCodec::encode_diff`/`decode_diff` impls were
rewritten from `Ok(self.print_diff().into_bytes())` to genuine `dsl::ByteWriter`/
`dsl::ByteReader` field-by-field encoding:

- **87a `GifDiff`**: `width`/`height`/`background_color_index`/`pixel_aspect_ratio` as 2-way
  presence-flagged scalars; `gct` (tri-state `Option<Option<GifColorTable>>`) as a 3-way flag
  wrapping a length-prefixed opaque blob (variable-length payload); `images` (the
  removed/modified/added collection triple, itself nesting `GifImage`/`GifImageDiff`
  recursively) as a 2-way-flagged length-prefixed opaque blob. All varint counts, all recursive
  per-item encoding — genuinely structured, never text-as-bytes.
- **89a `GifDiff`**: same shape plus `loop_count` (tri-state, FIXED-width `u16`, no blob wrap
  needed — same treatment PNG's own fixed-width tri-state fields get) and three collection
  triples (`frames`, `comments`, `app_extensions`), `GifFrameDiff`'s own three nested tri-states
  (`lct`/`transparent_index`/`plain_text`) all genuinely encoded.

Both verified via the pre-existing `diff_codec_text_binary_roundtrip_law` test (already present,
now exercising the real binary path) — **0 failures**, plus the new `protocol_walk_law`
conformance test walking the SAME real bytes through the new protocol description.

### 2. Grammar files (6, real dialect)

All were the pre-Phase-2 ABNF-shaped placeholder (`dialect grammar stdio.gif.snapshot\nroot =
document\n...`, not even the real header syntax) — rewritten to the real dialect:

- **Snapshot** (both standards): the hex-dump grammar per PNG's own precedent (`GifSnapshot`'s
  `ArtifactDsl` is hand-rolled hex-encode/decode of the real binary bytes, confirmed by reading
  — `HandcraftedArtifactCodecs` region, same as PNG, NOT the `dsl::DslRecord`-derive path even
  though `GifSnapshot` derives `DslRecord` for the mutation-embedding use case).
- **Diff** (both standards): the real one-line `name=value`/collection-triple form
  `print_gif_diff`/`parse_gif_diff` emit — matches this artifact's own hand-rolled `DiffCodec`
  text grammar exactly.
- **Mutations** (both standards): the real `dsl::print`/`dsl::parse`-over-`DslVariants` kebab-
  case `keyword field=value ...` form — captured verbatim via a temporary `[DEBUG]`-prefixed
  probe test (`mutation.print_op()` over every real variant), never guessed. Key empirically-
  confirmed (not assumed) shape rules, cross-checked by directly reading
  `🔍️lexer/🦀️component.rs`:
  - Any `Option<T>` field (block-struct OR plain scalar) is OMITTED ENTIRELY when `None` — no
    tri-state bracket tag at all in the mutations text form (that convention belongs solely to
    the separate hand-rolled DIFF codec grammar).
  - Non-block scalar fields print first (declaration order), block/`Vec<Struct>` fields last
    (declaration order among those) — NOT strict declaration order (`gct`/`images`/`frames`/
    `comments`/`app_extensions` all migrate to the end).
  - A plain `String` field (not `#[dsl(base64)]`) prints as a bare, UNQUOTED `Ident` token — the
    shared lexer's `is_ident_continue` allows `.`/`-`/`_`/`/`, so `schema=stdio.gif.89a` lexes
    as one `Ident`, never `Text` (confirmed by reading the lexer source directly after an
    initial wrong guess using `TEXT` — see Deviations).
  - A `Vec<Struct>` field prints as `field=[ item-fields item-fields ... ]`, items back-to-back
    with NO separator (parser relies on each item's own fixed field count).
  - A fixed `[u8; N]` field (`GifAppExtension::identifier`/`auth_code`) prints as
    `field=v1,v2,...,vN` — bare comma-separated ints, no enclosing brackets at all.
  - A plain unit-variant enum (`GifDisposal`) prints as its own bare kebab-case keyword.

### 3. Protocol files (6, real dialect)

- **Snapshot** (both standards): real magic + Logical Screen Descriptor fields (`width`/
  `height`/`packed`/`bg_color_index`/`pixel_aspect_ratio`), opaque `chain rest bytes` past that
  — see Mechanism Gaps for the two genuine reasons the block sequence can't be modeled further.
- **Diff** (both standards): real 2-way/3-way presence-flag-per-field layout, matching the new
  Rust binary frame exactly, verified live via `protocol_walk_law`.
- **Mutations** (both standards): `format u8` + `ordinal varint` genuinely walked, remaining
  record-body bytes one opaque `chain bytes` — copied verbatim from `📄txt`'s own
  `txt-opbinary-record-body-wire-is-framework-generic` precedent (a framework-generic wire,
  not artifact-specific).

### 4. Registration

Both standards' `⚙️engine/🦀️component.rs::register()` now call `register_pilot_languages()`
(5-role `LanguageSpec`: Document/Ops/Diff/Pack/Spr, all `dsl::passthrough_hooks`) and
`register_schema_specs()` (`dsl::registry::register_schema_spec("stdio.gif"[.89a],
GifSnapshot::__dsl_spec)` — real, since `GifSnapshot` derives `DslRecord`; `GifDiff`'s own
`#diff` id is deliberately NOT registered, hand-rolled with no derivable `RecordSpec`, filed
as `mechanism_gaps`).

### 5. Fixtures + conformance tests

- **87a**: new `🏅️standards/🔖️87a/📚️examples/🎬️demo/🖼️assets/` with a genuine `print_dsl`
  (139 bytes) and `encode_pack` (87 bytes) of a small hand-built `demo_gif_snapshot()`
  (2 images, one with its own LCT, real GCT).
- **89a**: new `🏅️standards/🔖️89a/📚️examples/🎬️demo/🖼️assets/` with a genuine `print_dsl`
  (8,867,085 bytes) and `encode_pack` (4,433,562 bytes) of `demo_gif_snapshot()` = the REAL
  `dancing.gif` fixture (54 frames, 800×800, per-frame LCTs, NETSCAPE2.0 loop) decoded via the
  real 89a codec — per the ticket's own explicit instruction, for byte-real conformance.
- Both engines' `⚙️engine/🦀️component.rs` gained a `conformance_laws` test module (6 tests
  each, 12 total): `committed_facet_files_parse`, `grammar_conformance_law`,
  `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
  `fixture_honesty_law`. All 12 pass. `diff::demo_diff_cases()`/`mutations::demo_mutation_cases()`
  helper functions were added to each standard's `🔺️diff`/`🧬️mutations` modules.

## Files touched (created or modified)

**87a:**
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — real binary `DiffCodec` frame + `demo_diff_cases()`
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` — rewritten
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` — rewritten
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `demo_mutation_cases()` added
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` — rewritten
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` — rewritten
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` — rewritten
- `⚙️engine/🦀️component.rs` — `demo_gif_snapshot()`, `register_pilot_languages()`,
  `register_schema_specs()`, `conformance_laws` test module
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (new), `🎒️example.pack.semio` (new)

**89a:** identical file set under `🏅️standards/🔖️89a/`, plus `📚️examples/🎬️demo/🖼️assets/`
sourced from the real `dancing.gif`.

## Mechanism gaps

| gap id | engine area | symptom | blocking | honest workaround |
|---|---|---|---|---|
| `protocol-framing-magic-fixed-8-bytes` (NEW, not yet in recipe §5) | `walk_protocol`'s `Framing::Magic` arm | Unconditionally reads/compares EXACTLY 8 bytes for a `framing magic` byte-check, regardless of how many hex digits the declared literal has — confirmed live: `framing magic 0x474946383761` (GIF's real 6-byte "GIF87a" magic) got read/compared as an 8-byte, left-zero-padded value against the real 6-byte on-disk magic + 2 real trailing bytes, a genuine mismatch caught by `protocol_walk_law`. Every prior real `framing magic` precedent in this repo (PNG, the SEMIO envelope, the `.spk` container) happens to be exactly 8 bytes. | No (worked around) | `framing record` + a real, individually-read `field magic fixed 6` — genuinely walked, not byte-validated at the protocol layer (Rust `decode_gif` already validates it byte-for-byte). |
| `protocol-bitfield-bytelen-computation` (NEW, not yet in recipe §5) | `Prim`/`Cond`, `Block::Repeat` | The Logical Screen Descriptor's `packed` byte bundles GCT-presence (bit 7, testable via `Cond ge/eq`) with the GCT's own SIZE (low 3 bits) — this dialect has no bitfield/sub-byte field decomposition primitive AND no arithmetic-expression `Array` count source (`3 * 2^(size+1)`), so the GCT's own byte length can never be computed declaratively, which in turn means the repeated image/extension/trailer block sequence's own START position is unknowable — blocking the ENTIRE remainder past the fixed header, not one arm. | Yes, for everything past the 13-byte fixed header | `chain rest bytes` — one honest opaque tail past the real, individually-walked magic+screen-descriptor fields. The Rust `encode_gif`/`decode_gif` side is genuinely, fully structured (real bit-shift GCT-size decode, real [nested, for 89a] tag-dispatched block loop, real LZW sub-block chain, real cross-block GCE state carry for 89a) — this is purely a protocol-DESCRIPTION depth limit. |
| `register-schema-spec-needs-recordspec` (existing recipe §5 entry, hit here) | `dsl::registry::register_schema_spec` | `GifDiff` (both standards) is hand-rolled with no `#[derive(dsl::DslDiff)]` (tri-state fields block it) — no `__dsl_diff_spec` exists. | No | `GifSnapshot`'s own real `__dsl_spec` IS registered (`stdio.gif`/`stdio.gif.89a`); the `#diff` id is simply not registered, per the recipe's own explicit "do NOT fabricate a spec" instruction. |
| `protocol-prim-ref-recursion` (existing, hit here) | `walk_protocol`'s `Prim::Ref` arm | `images`/`frames`/`comments`/`app_extensions` collection triples and the `gct`/`lct` color tables all nest structurally past what `Array`/`Ref` can describe. | Only for those nested payloads | Length-prefixed opaque blobs (`Array(u8, Field(<name>_len))`), same pattern every other pilot's own nested diff payload uses. |
| cross-block GCE state carry (P2-W0 §1b's own `gif/89a` row, restated) | 89a's real `decode_gif` | A GCE's decoded fields (`pending_gce`) are consumed by the NEXT, different block (Image Descriptor or Plain Text Extension) — beyond any per-block-local field env `walk_fields` provides. | Subsumed by `protocol-bitfield-bytelen-computation` above (already opaque past the header) | Same `chain rest bytes` treatment — the real Rust decoder's own `pending_gce` carry is unaffected. |

## Test results

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::gif::"` — **75 passed, 0 failed**
  (includes all pre-existing engine/diff/mutation/example tests plus the 12 new
  conformance-law tests, 6 per standard).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) — **1772 passed, 0 failed, 1 ignored**
  (baseline expectation was ≥1714/0/1-ignored; several other FG-wave agents' work landed
  concurrently during this session, raising the total).
- `bun run ./📜️script.ts policy` — grepped the full breach report for `gif` under each of the
  five policies this wave is responsible for (`POLICY_GRAMMAR_PARSEABILITY`,
  `POLICY_PROTOCOL_PARSEABILITY`, `POLICY_FIXTURE_HONESTY`, `POLICY_LANGUAGE_REGISTRATION`,
  `POLICY_STDIO_JSON_TRANSFER_BAN`, and the dominant `handcrafted-grammar/spec-distinctness`
  rule) — **zero gif hits on all of them**. The policy run's overall repo-wide exit code is
  non-zero (21,514 breaches across 25 rules, dominated by `handcrafted-grammar/spec-
  distinctness` at 19,340 — pre-existing, repo-wide debt spanning dozens of unrelated plugins,
  not introduced by this program). Grep for `serde_json::to_vec`/`from_slice`/`to_string`/
  `from_str` inside `gif`'s `ArtifactPack`/`OpBinary`/`DiffCodec` impl blocks: clean.

## Deviations from the brief

1. **`framing magic` mechanism gap discovered live.** My first draft of both snapshot protocol
   files used `framing magic 0x474946383761`/`0x...3961` (GIF's real 6-byte magics). A real
   `protocol_walk_law` run caught that `Framing::Magic` unconditionally reads exactly 8 bytes
   regardless of the literal's own digit count — reverted to `framing record` + a real `field
   magic fixed 6`, documented as a new mechanism gap (see table above).
2. **Mutations grammar's `schema`/string fields: `TEXT` → `IDENT`.** My first draft used
   `TEXT` for `GifSnapshot.schema` (a plain, unquoted string field in `dsl::print`'s real
   output). `ops_grammar_conformance_law` failed to recognize the real printed text; reading
   `🔍️lexer/🦀️component.rs` directly showed `is_ident_continue` allows `.`/`-`/`_`/`/`, so
   unquoted dotted/hyphenated content lexes as `Ident`, not `Text` (`Text` is specifically the
   QUOTED form). Fixed by switching every plain-`String`-field production to `IDENT`.
3. **89a's `demo_mutation_cases()` uses a small, hand-built `SetSnapshot` payload, NOT the full
   `dancing.gif`-derived `demo_gif_snapshot()`.** Embedding the real 800×800/54-frame fixture
   inside a mutation op-text payload is unnecessary for exercising the mutations grammar's own
   shape (which the compact snapshot already covers field-for-field) and was needlessly large;
   the SNAPSHOT-facet conformance laws (`grammar_conformance_law`, `protocol_walk_law`,
   `fixture_honesty_law`) do use the real `dancing.gif`-derived snapshot, per the ticket's own
   instruction.
4. **`OpBinary` was already real for both standards** (forwards to `dsl::variants_binary`) —
   confirmed by direct reading before touching anything, per the ticket's own explicit
   instruction not to assume; no changes needed there, reported as `opbinary_binary_upgraded:
   true` reflecting the current (already-correct) state, not a change made this program.
5. **Left the pre-existing, artifact-level `🎞️gif/📚️examples/🎬️demo/` example untouched.**
   This is a SEPARATE, artifact-root-level (not per-standard) example asset containing a stale
   pre-F6 stub (`68656c6c6f` hex = "hello", flagged by the policy run's
   `handcrafted-grammar/empty-example` rule, pre-existing and not introduced by this program).
   It is distinct from the per-standard `🏅️standards/🔖️87a|89a/📚️examples/🎬️demo/` fixtures
   this program created, and its own `🦀️component.rs` doesn't reference either standard's real
   codec at all (unlike `💃️dancing`, which does). Fixing it was out of this wave's explicit
   per-standard checklist scope; flagged here rather than silently left for a future wave to
   rediscover.
6. **6 conformance-law tests added per standard (12 total), plus `demo_diff_cases()`/
   `demo_mutation_cases()` helper functions** — not explicitly named as a hard requirement for
   every FG-wave in the top-level brief text, but required by the recipe's own §4 per-standard
   checklist ("The 6 conformance-law tests") and the only way to actually VALIDATE (not assume)
   that the newly-authored grammar/protocol files are correct, per CLAUDE.md's own
   "must validate assumptions" rule.

## Verification commands run

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::gif::"          # 75 passed, 0 failed
cargo test -p semio-s-plugin-stdio --lib                              # 1772 passed, 0 failed, 1 ignored
bun run ./📜️script.ts policy                                          # 0 gif hits on the 5 named policies + spec-distinctness
```
