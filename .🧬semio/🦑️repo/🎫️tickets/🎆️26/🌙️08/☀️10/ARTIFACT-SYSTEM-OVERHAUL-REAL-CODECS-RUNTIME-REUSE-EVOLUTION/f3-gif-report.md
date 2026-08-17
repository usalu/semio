# F3 mop-up — gif (87a + 89a) — agent report

Wave: F3 mop-up (this ticket's earlier F3 fan-out never landed real gif work — see
`f3-closer-report.md`'s "only `f3-md-report.md` and `f3-png-report.md` exist on disk" finding;
this is the resumed/full pass for gif, both standards). Ownership: exactly
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/**` + this report. `📦️glue.rs`, `📜️script.ts`, the SDK
traits, the schema module, the io module, and `🏪️store` were never touched, per this session's
explicit boundary.

## 1. Starting state (confirmed by reading, not assumed)

Both standards were "partial" per `w0-recon-report.md` and `STATUS.md`'s F3-closer section:

- **87a**: single static `RasterImage{width,height,rgba}` (decoded RGBA, no palette/LCT/GCT
  retention), diff was the pre-overhaul `{snapshot: Option<GifSnapshot>}` full-replace-only stub,
  mutations only `{NoMutation, SetSnapshot}`.
- **89a**: furthest-along of all 31 standards but still short: `GifSnapshot` stored decoded
  per-frame `rgba: Vec<u8>` (no GCT, background_color_index, pixel_aspect_ratio, comments,
  app_extensions); `GifDiff` was the known-buggy op-slot shape (`snapshot: Option<GifSnapshot>`
  PLUS one `Option<T>` field per mutation kind) with a documented LWW-loses-coalesced-inserts
  absorb bug; mutation enum had only 6 of the target ~20 variants; `apply_gif_mutation` returned
  `()`, not a diff (this was fixed by S1 mechanically, but the diff shape itself was still the
  op-slot stub going into this wave).

## 2. What was built (real rewrite, both standards, per `🧬️schema-design.md`)

### Snapshot
- **87a**: `GifSnapshot{schema, width, height, gct: Option<GifColorTable>,
  background_color_index: u8, pixel_aspect_ratio: u8, images: Vec<GifImage>}`.
  `GifImage{left, top, width, height, interlace: bool, lct: Option<GifColorTable>,
  indices: Vec<u8>}` — palette indices, never decoded RGBA (the lossless-payload exception).
  `rgba()` is now a derived accessor method (`GifImage::rgba(&self, gct: Option<&GifColorTable>)`),
  not a stored field. 87a genuinely supports MULTIPLE images per file (GIF87a §20 — no GCE
  needed for that), a real spec-fidelity gain over the prior single-`RasterImage` model.
- **89a**: `GifSnapshot{schema, width, height, gct, background_color_index, pixel_aspect_ratio,
  loop_count, frames: Vec<GifFrame>, comments: Vec<String>, app_extensions: Vec<GifAppExtension>}`.
  `GifFrame{left, top, width, height, interlace, lct, indices, delay_cs, disposal,
  transparent_index: Option<u8>, user_input, plain_text: Option<GifPlainText>}` — same
  lossless-indices treatment, plus real GCE/loop/comment/plain-text/app-extension modeling that
  the prior stub dropped entirely (comments and unrecognized application extensions were
  previously "read and discarded, never causing a decode failure" per the old doc comment — now
  fully retained). `GifColorTable{sorted: bool, colors: Vec<GifRgb>}` — stored exactly as read
  from disk (including power-of-two padding entries), matching real on-disk bytes.
- Both standards declare their OWN `GifColorTable`/`GifRgb` types (no cross-standard shared
  type) — the one legitimate cross-standard bridge is the migration module (§6).

### Diff — DELETED the old shapes wholesale, replaced with the sparse recipe
- **87a**: `GifDiff{width, height, gct: Option<Option<GifColorTable>>, background_color_index,
  pixel_aspect_ratio, images: Option<GifImagesDiff>}`. `GifImagesDiff{removed: Vec<usize>,
  modified: Vec<GifImageModified{index, diff: GifImageDiff}>, added: Vec<GifImageAdded{index,
  image}>}`. `GifImageDiff` covers every `GifImage` field as `Option<T>` (tri-state `lct`).
- **89a**: `GifDiff{width, height, gct, background_color_index, pixel_aspect_ratio, loop_count:
  Option<Option<u16>>, frames: Option<GifFramesDiff>, comments: Option<GifCommentsDiff>,
  app_extensions: Option<GifAppExtensionsDiff>}`. `GifFramesDiff` mirrors the images shape;
  `GifFrameDiff` covers every field (tri-state `lct`, `transparent_index`, `plain_text`).
  `comments`/`app_extensions` are weak/value collections per the recipe's strong/weak split — the
  collection "diff" for a modified entry IS the whole new `String`/`GifAppExtension`, no further
  sub-diffing.
- **Zero `snapshot: Option<XSnapshot>` full-replace slots anywhere** — grep-confirmed (only doc
  comments mention the phrase, describing what was deleted).
- Both diff types `impl protocol::MutationDiff<XSnapshot>` (`apply`, `absorb`) AND
  `impl protocol::os_spr::command::DiffAlgebra<XSnapshot>` (`inverse`, `between`, `is_empty`).

### Absorb — the hard part, implemented as a shared generic algorithm (duplicated once per
standard, since each standard owns its own diff module)
A closed-form, base-free index-transport algorithm (`rank_excluding`/`unrank_excluding`/
`transport_forward` + a generic `absorb_indexed_collection<T, D>` / `inverse_indexed_collection<T,
D>` pair) reused for all three collection kinds per standard (images/frames, comments,
app_extensions). Derivation and correctness reasoning are in this session's own working notes;
verified against the plan's exact 3 canonical cases as unit tests in both standards' diff files:

- `Insert(2,f)` + `Remove(0)` → `{removed:[0], added:[(1,f)]}` — **PASSES** (both standards).
- `Insert(2,f)` + `Insert(2,g)` → **both survive** as `added:[(2,g),(3,f)]` — **PASSES** (the
  exact bug this recipe replaces; the OLD op-slot absorb would have LWW'd one insert away).
- `Insert(1,f)` + `SetField(1,v)` → patches INTO the added payload, `added:[(1,f_with_v)]`, no
  separate `modified` entry — **PASSES**.
- Plus a broader `absorb_law_holds_over_curated_ops` test per standard combining insert+remove+
  modify+comment/app-extension-add in one sequence, asserting `absorb(d1,d2).apply(base) ==
  d2.apply(d1.apply(base))`.

### Mutations
- **87a** (~11 variants + NoMutation, matching what GIF87a's actual spec surface supports):
  `SetSnapshot, SetScreenSize, SetGlobalColorTable, SetBackgroundColorIndex,
  SetPixelAspectRatio, InsertImage, RemoveImage, MoveImage, SetImageGeometry, SetImagePixels,
  SetImageInterlace`. No GCE-shaped variants — 87a genuinely has none.
- **89a** (20 variants + NoMutation, the plan's full worked-design list): `SetSnapshot,
  SetScreenSize, SetGlobalColorTable, SetBackgroundColorIndex, SetPixelAspectRatio,
  SetLoopCount, InsertFrame, RemoveFrame, MoveFrame, SetFrameGeometry, SetFramePixels,
  SetFrameInterlace, SetFrameDelay, SetFrameDisposal, SetFrameTransparency, SetFrameUserInput,
  InsertComment, RemoveComment, AddAppExtension, RemoveAppExtension`.
- Every variant's `diff()` is handcrafted directly against the sparse `GifDiff` shape (no
  apply-and-capture anywhere). `inverse()` is handcrafted per variant, key/index-aware, verified
  by `mutation_apply_inverse_round_trips_every_variant` covering every listed variant in both
  standards.

### Engine (real byte-level codec rewrite, both standards)
- Encode/decode rewritten end-to-end: GCT/LCT read/written exactly as on-disk bytes (including
  power-of-two padding — `write_color_table` pads a non-power-of-two snapshot table with black
  filler entries, a documented, honest, one-way normalization matching real encoders; only
  lengths `>256` are a typed error), `background_color_index`/`pixel_aspect_ratio` are real
  fields (previously hardcoded `0`), `interlace` is now genuinely round-trippable (added
  `interlace_rows`, the missing inverse of the pre-existing `deinterlace_rows`, so encode
  actually reorders rows into the on-disk interlaced pass order instead of only decode
  understanding it).
- **89a additionally**: comments (Comment Extension `0x21 0xFE`) and every non-NETSCAPE
  application extension (`0x21 0xFF`, `GifAppExtension{identifier,auth_code,data}`) are now
  real, round-trippable, typed content — not "read and discarded" as before. Plain Text Extension
  (`0x21 0x01`) is modeled via `GifFrame.plain_text: Option<GifPlainText>`; a frame with
  `plain_text: Some` and empty image data (`width==0, height==0, indices.is_empty()`) encodes as
  a real Plain-Text-only graphic-rendering block (no Image Descriptor). A frame combining BOTH
  real image data and `plain_text` is a documented, typed encode error (deliberately unsupported
  combo — Plain Text Extension is essentially unused in real-world GIFs, and conflating it with
  an Image Descriptor isn't spec-representable as one block anyway).
- Documented normal form: comments are written right after the screen descriptor/GCT, then the
  loop extension, then every other app extension, then the frames — content-losslessness (not
  exact original byte position) is the contract, matching the recipe's "byte-preserving up to
  documented normalizations."

### A real, previously-latent bug found and fixed (LZW core, shared by both standards)
`lzw_encode`/`lzw_decode` (87a engine, reused by 89a via `pub` functions) had a genuine
encoder/decoder desync bug at the TAIL of the stream: `lzw_decode` performs an insert-then-maybe-
grow step for every code it reads with a preceding code — including the very last data code
before the end code — but `lzw_encode`'s loop only performs its matching insert+growth-check for
in-loop dictionary misses, never for the trailing flush of the final outstanding symbol (there's
no further symbol to trigger one). When that final flush happened to land exactly on a code-size
growth boundary, the encoder would write the END code at the OLD (narrower) bit width while the
decoder, having just grown from its own insert on the final data code, expected the NEW (wider)
bit width — a real `"unexpected end of lzw stream"` decode failure. Found via this ticket's own
field-sweep-style test data (a plain period-2 alternating sequence, `0,1,0,1,...`) — the
pre-existing pseudo-random/solid-run test suite (20,000+ samples across `min_code_size` 2..8)
never happened to land a final symbol exactly on that boundary. Fixed by mirroring the decoder's
one extra insert+growth-check after the final flush (only when a prior in-loop write happened
since the last clear code, matching the decoder's own `prev.is_some()` guard) — the well-tested
asymmetric `>`/`>=` mid-stream growth thresholds (already verified against the real
`dancing.gif` fixture) were untouched. New regression test:
`lzw_round_trip_period_two_alternating_hits_growth_boundary_at_tail` (87a engine, swept across
`min_code_size` 2..6 and lengths 2..80). All pre-existing LZW tests still pass unchanged.

### GifBuilder (89a) gained typed setters
`set_global_color_table`, `set_background_color_index`, `set_pixel_aspect_ratio`, `add_comment`,
`add_app_extension` — needed once the snapshot grew real fields the prior builder's `new()`/
`add_frame()`/`set_loop_count()` alone couldn't populate; the `💃️dancing` fixture's
analyzer→builder round-trip test now exercises all of them (previously would have silently
dropped background_color_index/pixel_aspect_ratio/gct/comments/app_extensions on rebuild).

### Migration (89a's `🧬️migrations`)
`migrate_87a_to_89a` rewritten: since BOTH standards now store lossless palette indices (not
RGBA), the migration is a direct field carry-over per image→frame (no pixel re-quantization at
all, unlike the pre-rewrite version which round-tripped through already-RGBA-expanded bytes) —
`left/top/width/height/interlace/lct/indices` carry straight over; 89a-only fields default
honestly (`delay_cs: 0, disposal: Unspecified, transparent_index: None, user_input: false,
plain_text: None`); screen-level fields (`width/height/gct/background_color_index/
pixel_aspect_ratio`) carry straight over too; `loop_count: None` (87a has no loop concept — the
existing comment's reasoning still applies, unchanged). All 3 migration tests updated and
passing.

### `💃️dancing` fixture
Kept passing per the ticket's explicit requirement — `decode_gif`/`encode_gif`/analyzer/builder
round trips all still work against the real 54-frame, 800×800, per-frame-LCT, NETSCAPE-loop
fixture; the analyzer→builder test now round-trips the full field set (see builder note above).

## 3. Verification

`cargo test -p semio-s-plugin-stdio --lib "artifacts::gif"` → **55 passed, 0 failed**.
`cargo test -p semio-s-plugin-stdio --lib` (whole crate, no filter) → **853 passed, 0 failed** —
confirms zero collateral breakage anywhere else in the crate from this wave's work.

Grep gates:
- `snapshot: Option<` inside either diff file: **0 struct-field hits** (only doc-comment
  mentions describing what was deleted).
- `impl DiffAlgebra` present in both `🔺️diff/🦀️component.rs` files: **confirmed**.
- `field_sweep` (name containing it) present in both diff files: **confirmed**
  (`field_sweep_covers_every_mutable_field`).
- All 6 law suites present per standard (`mutation_diff_law`, `inverse_law` [+ mutation-level
  `mutation_apply_inverse_round_trips_every_variant`], `absorb_law` [as
  `absorb_law_holds_over_curated_ops` + 3 named canonical-case tests], `between_roundtrip_law`,
  `codec_retention_law`-equivalent [`encode_decode_round_trip_*`/`encode_decode_encode_decode_is_stable`
  in the engine files], `field_sweep`): **confirmed present in both standards**.

Policy (`bun ./📜️script.ts policy`, read-only — did not edit `📜️script.ts` per this session's
ownership boundary): cross-checked `.🦑️repo/⚡️cache/breaches/compose.json` directly, scoped to
gif. All 4 S-8 rule breaches for gif are **`-stale-`** (2× `stdio-artifacts/diff-algebra`, 2×
`stdio-artifacts/field-sweep-presence`, one per standard each) — confirming both standards now
genuinely implement `DiffAlgebra` and have a real `field_sweep` test, with only the
`POLICY_DIFF_ALGEBRA_ALLOWLIST`/`POLICY_FIELD_SWEEP_ALLOWLIST` entries left to prune (closer's
job — `stdio/gif/standards#87a-subsets-any-schema-diff-component`,
`stdio/gif/standards#89a-subsets-any-schema-diff-component`,
`stdio/gif/standards#87a`, `stdio/gif/standards#89a` respectively). **Zero real (non-stale)
breaches for any of the 4 S-8 rules on gif.** `grammar-honesty` and `facet-mirror-drift` produced
**zero breach entries of either kind** for gif — both remain correctly, silently allowlisted as
genuinely not-yet-fixed (see deviations below), matching F3-closer's earlier documented
precedent for gif/dxf.

## 4. Deviations / not done (honest gaps, flagged rather than silently skipped)

1. **Facet mirrors (`.ts`/`.graphql`/`.json`/`.proto` for snapshot/diff/mutations) were NOT
   rewritten** to match the new field shapes — they remain the pre-existing (stale/placeholder)
   content. This is real, documented drift; `POLICY_FACET_MIRROR_DRIFT` produced zero breaches
   either way (gif was never seeded in that allowlist, and the checker has known false-positive
   sources per F1's own investigation), so this doesn't block policy, but it's a genuine
   remaining gap. Recommend a dedicated facet-mirror wave (same deferral png/md already used via
   their own `glue_followup` to F6).
2. **Grammar leaves** (`.g4`/`.ebnf`/`.grammar.semio`/`.ksy`/`.spicy`/`.abnf`/`.protocol.semio`)
   were NOT handcrafted this wave — still the pre-existing placeholder content, still correctly
   allowlisted in `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (6 entries, unchanged). Per the plan's user
   decision ("handcraft ALL formats honestly") this is real remaining scope, consistent with
   every other F-wave's own grammar-leaf debt (deferred to F6 per precedent, not skipped
   silently).
3. **Plain Text Extension + real image data on the same frame is an unsupported, typed-error
   combo** (documented above) — a deliberate scope decision, not an oversight; real-world GIFs
   essentially never combine these.
4. Composer/analyzer/io-serializer leaves were read and confirmed to only reference `GifSnapshot`/
   `GifFrame`/`GifImage` opaquely (never touching individual fields) — genuinely zero-touch,
   confirmed by successful compilation, not just left alone by assumption.

## 5. glue_followup

None. No new top-level directory was needed; every field/type/absorb/inverse/between addition
fit inside the already-mounted `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/🦀️component.rs`,
`⚙️engine/🦀️component.rs`, `🏗️builder/🦀️component.rs`, and `🧬️migrations/🦀️component.rs` files for
both standards, per S2's confirmed resolution that triad directories are optional scaffolding.

## 6. Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — full rewrite: `GifImage`/`GifColorTable`/`GifRgb`, `images: Vec<GifImage>` snapshot.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — full rewrite: sparse `GifDiff`/`GifImagesDiff`/`GifImageDiff`, generic index-transport absorb/inverse, `DiffAlgebra` impl, tests.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — full rewrite: ~11-variant `GifMutation`, handcrafted diff/inverse per variant, tests.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` — signature fix (`diff(base, snapshot)`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — `GifArtifact` grown to mirror the new snapshot fields.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/⚙️engine/🦀️component.rs` — real encode/decode rewrite (GCT/LCT/multi-image/interlace/screen fields); **the LZW encoder tail-desync bug fix**; color-table byte-conversion helpers; test suite rewrite + new regression test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — full rewrite: `GifFrame`/`GifColorTable`/`GifRgb`/`GifAppExtension`/`GifPlainText`, complete 89a snapshot.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — full rewrite: sparse `GifDiff` (frames/comments/app_extensions triples), generic absorb/inverse, `DiffAlgebra` impl, tests (incl. all 3 canonical absorb cases + field_sweep).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — full rewrite: 20-variant `GifMutation`, handcrafted diff/inverse per variant, tests.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` — signature fix.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — `GifArtifact` grown to mirror the new snapshot fields.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/⚙️engine/🦀️component.rs` — real encode/decode rewrite (GCT/LCT/comments/app-extensions/plain-text/interlace/screen fields); test suite rewrite + new tests.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🏗️builder/🦀️component.rs` — new typed setters (gct/background/aspect/comment/app-extension).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🧬️migrations/🦀️component.rs` — `migrate_87a_to_89a` rewritten for the new lossless-indices shape; tests updated.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📚️examples/💃️dancing/🧪️tests/🦀️test.rs` — updated for `indices`/`rgba()` accessor; analyzer→builder round trip now exercises the full field set.
- This report.

No temp/scratch files were left outside this ticket folder; no ad-hoc scratch files were created
in the ticket folder either (debug `eprintln!`s used during investigation were added and removed
from source files in the same session, never committed to a persisted state).
