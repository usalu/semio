# F2 — las (standard 1.0) — Fan-out Agent Report

Wave: F2 (stl, obj, ply, las, bmp, tiff). Artifact: `☁️las` / standard `1.0`. Ownership boundary:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/**` (never touched `📦️glue.rs`, `📜️script.ts`, the SDK
traits, the schema module, the io module, or `🏪️store`).

## 1. Starting state ("Weak" tier)

`LasSnapshot` was `{schema, points: Vec<LasPoint>}` only — no header, no VLRs. `LasDiff` was the
byte-identical 34-line replace-template (`{snapshot: Option<LasSnapshot>}`). `LasMutation` was the
universal `{NoMutation, SetSnapshot}` stub. `apply_las_mutation` already returned a diff (S1's
mechanical sweep), but the diff itself carried no real semantics. `engine::{encode_las, decode_las}`
already had a real, well-tested point-record codec for formats 0-3 (7 tests) but hardcoded a
227-byte header with zero real header fields and no VLR support.

## 2. What changed — Snapshot (full LAS 1.0 completeness per the ticket's contract)

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:

- **`LasHeader`** (new, 25 fields): `version_major/minor`, `system_identifier`,
  `generating_software`, `creation_day_of_year/year`, `header_size`, `offset_to_point_data`,
  `number_of_vlrs`, `point_data_format_id`, `point_data_record_length`,
  `number_of_point_records`, `points_by_return: [u32;5]`, `x/y/z_scale`, `x/y/z_offset`,
  `max_x/min_x/max_y/min_y/max_z/min_z`. Six fields (`header_size`, `offset_to_point_data`,
  `number_of_vlrs`, `point_data_format_id`, `point_data_record_length`,
  `number_of_point_records`) are documented as STRUCTURAL — `engine::encode_las` always
  recomputes them from the real `vlrs`/`points` content (matching the pre-existing
  `header_size=227` precedent), so a stale stored value can never corrupt a re-encode. The
  remaining 19 are genuinely round-trip-preserved. `file_source_id`/`global_encoding`/
  `project_id_guid` (spec-real, 20 bytes) are explicitly out of the ticket's contracted field
  list — skipped on decode, zeroed on encode; noted as a deviation below.
- **`LasVlr`** (new, index-keyed collection element): `{user_id, record_id, description,
  data: Vec<u8>}` — `data` retained byte-verbatim (typed raw-retention exception, same shape as
  `PngChunk`/`GifAppExtension`).
- **`LasPoint`**: unchanged (kept `gps_time`/`rgb` for formats 1/2/3 even though the ticket's
  contracted scope is 0/1 — already-working, already-tested content the recipe's "nothing real
  on disk silently dropped" rule forbids regressing).
- **`LasSnapshot`**: `{schema, header: LasHeader, vlrs: Vec<LasVlr>, points: Vec<LasPoint>}`.

## 3. What changed — Diff (handcrafted sparse, `🔺️diff/🦀️component.rs`)

- **`LasDiff`**: every one of the 25 header fields is a top-level `Option<T>` scalar (per the
  ticket's explicit instruction — flat, not nested under a `header: Option<LasHeaderDiff>`
  wrapper), plus `vlrs: Option<LasVlrsDiff>` and `points: Option<LasPointsDiff>`. Zero
  `snapshot: Option<LasSnapshot>` full-replace slot anywhere, including `SetSnapshot` (its diff
  is `LasDiff::between(base, next)`, same machinery every other mutation's diff composes from).
- **`LasVlrDiff`/`LasVlrsDiff`** and **`LasPointDiff`/`LasPointsDiff`**: index-keyed
  `{removed: Vec<usize>, modified: Vec<{index, diff}>, added: Vec<{index, item}>}` triples,
  per-entity sparse field diffs (`LasPointDiff`'s `gps_time`/`rgb` are tri-state
  `Option<Option<T>>`).
- **`impl MutationDiff<LasSnapshot> for LasDiff`**: `apply` (header scalars in-place, then
  per-collection triple apply) + `absorb` (LWW scalars + a generic
  `absorb_indexed_triple<Item, D>` helper — one index-transport/label-simulation algorithm
  written ONCE and reused for both `vlrs` and `points`, since they're the same collection shape
  within this one artifact; not shared across artifacts, so doesn't reintroduce the
  cross-artifact generic-type problem the ticket targets for elimination). Mirrors `txt`'s
  `Lbl`/`simulate_labels`/`absorb_pair` pattern (own copy, generalized once via closures for
  `absorb_field`/`patch_item`).
- **`impl DiffAlgebra<LasSnapshot> for LasDiff`**: `inverse` (derived via `apply`+`between`,
  correct-by-construction), `between` (pairwise `0..min(len)` for `vlrs`/`points`, base-tail
  removed, other-tail added — the recipe's normative index-keyed rule), `is_empty`.
- Diff builder functions (`diff_set_version`, `diff_set_system_identifier`,
  `diff_set_software_info`, `diff_set_creation_date`, `diff_set_scale_and_offset`,
  `diff_set_bounds`, `diff_set_points_by_return`, `diff_insert_vlr`, `diff_remove_vlr`,
  `diff_set_vlr_data`, `diff_insert_point`, `diff_remove_point`, `diff_set_point`) all derive
  `number_of_vlrs`/`number_of_point_records` from the REAL collection length
  (`base.vlrs.len()`/`base.points.len()`), never from the (possibly-desynced)
  `base.header.number_of_vlrs`/`number_of_point_records` fields — required for
  `mutation_diff_law` to hold unconditionally, not just on already-synced fixtures.

## 4. What changed — Mutations (`🧬️mutations/🦀️component.rs`)

13 real variants beyond `{NoMutation, SetSnapshot}`: `SetVersion`, `SetSystemIdentifier`,
`SetSoftwareInfo`, `SetCreationDate`, `SetScaleAndOffset`, `SetBounds`, `SetPointsByReturn`,
`InsertVlr`, `RemoveVlr`, `SetVlrData`, `InsertPoint`, `RemovePoint`, `SetPoint`. Every variant's
`diff()` is handcrafted (constructs `LasDiff` directly via the `schema::diff` builders) —
apply-and-capture never used. `apply_las_mutation`'s imperative body keeps
`header.number_of_vlrs`/`header.number_of_point_records` in sync with the real collection length
after `Insert*`/`Remove*` (a snapshot-level consistency guarantee; `encode_las` independently
recomputes both anyway). Handcrafted, index-aware `inverse()` per variant (out-of-range index →
`NoMutation`, graceful no-op).

Also fixed the S2-pre-mounted triad placeholder
`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` (its `diff(snapshot)` free function called the
old 1-arg `diff_set_snapshot` signature) — same fix pattern F1's zip agent applied to its own
identical triad leaf (`diff(base, snapshot)`).

## 5. Engine — real header + VLR + point codec (`⚙️engine/🦀️component.rs`)

`decode_las`: reads all 25 header fields at their real byte offsets (227-byte public header
block), trusts `offset_to_point_data`/`header_size` as ground truth (no hardcoded 227 clamp,
preserving the pre-existing test's intent), falls back to the LAS 1.4 extended point count at
offset 247 when the legacy count is zero (pre-existing behavior, preserved), then walks
`number_of_vlrs` VLRs starting at `header_size` (54-byte VLR header + verbatim data, bounded by
`offset_to_point_data`/`bytes.len()` — graceful truncation, never panics), then decodes point
records exactly as before (formats 0-3).

`encode_las`: writes all 25 header fields; the 6 structural ones are always recomputed from real
content (see §2); VLRs are written immediately after the header (`54 + data.len()` bytes each);
`offset_to_point_data` is computed as `227 + Σ(vlr bytes)`; point records follow, using the
snapshot's own `x/y/z_scale`/`x/y/z_offset` (previously hardcoded to `0.01`/`0`) to invert
real-world coordinates back to on-disk integers — this is the one genuine behavior change to the
pre-existing point encode/decode math, and it's backward-compatible (default `LasHeader::scale =
0.01, offset = 0.0` reproduces the old hardcoded behavior exactly).

New/extended engine tests: `vlrs_shift_offset_to_point_data` (new), `header_too_short_is_rejected`
(new); all 7 pre-existing point-format/edge-case tests kept, updated to exercise the enriched
header/VLR fields (`snapshot_with`/`sample_vlrs` fixtures replace the old bare-points-only
fixtures) while preserving every original assertion's intent.

## 6. Aggregator updates

`LasArtifact` (`🧬️schema/🦀️component.rs`, the parent aggregator holding the full artifact state)
gained `header`/`vlrs` fields alongside `points`, with `to_snapshot`/`from_snapshot`/`set_snapshot`
updated to keep all three in sync — this file isn't one of the three explicitly-named schema
subdirs but lives inside the artifact's own `🧬️schema/` tree (not glue.rs/script.ts/SDK/store), so
it was in-scope to keep `LasArtifact` a faithful mirror of `LasSnapshot`.

## 7. Facet mirrors (S-8 `POLICY_FACET_MIRROR_DRIFT`/`POLICY_GRAMMAR_HONESTY`)

Rewrote every top-level `.ts`/`.json`/`.graphql`/`.proto` for the `artifact`, `snapshot`, `diff`,
and `mutations` facets to match the new field shapes exactly (all were stale placeholder stubs
before this wave — the `.ts` files literally contained a `PLACEHOLDER_BYTES_COLON` typo token).
Rewrote the snapshot facet's **live-wired** grammar leaves honestly:
`📸️snapshot/💾️binary/{🥋️component.ksy, 🔠️component.abnf, 🌶️component.spicy,
📡️component.protocol.semio}` now describe the REAL 227-byte header + VLR + point-record-format
0-3 byte layout (previously `payload = *OCTET`/`size-eos: true` placeholders); `📸️snapshot/
📝️text/{🅰️component.g4, 🔤️component.ebnf, 📖️component.grammar.semio}` now describe the real
`semio stdio.las vN` preamble + lowercase-hex-pair body grammar (previously a bare
`payload = *OCTET`), matching the `stdio.binary raw` precedent's honest-opacity pattern and
pointing readers at the binary facet for the real structure.

**Deviation** (matches F1's own documented precedent exactly): the `diff`/`mutations` facets'
NESTED `text`/`binary` grammar leaf pairs (8 leaves total, e.g.
`🔺️diff/💾️binary/🥋️component.ksy`) were left as pre-existing placeholders. These are not
"live-wired" (nothing in `register_pilot_languages` references them — only the snapshot facet's
`grammar.semio`/`protocol.semio` are registered), and F1's own closer report documents leaving
the analogous zip leaves unfixed as an accepted, deliberate scope boundary for this wave (real
OpText/OpBinary grammar-writing for diff/mutation wire formats is F6's mandate). Also left the
snapshot facet's `📝️text/{🔣️component.json, 🔗️component.graphql, 🛰️component.proto}` (the
generic `Document{schema,payload}` wrapper shape) untouched — these describe the DSL envelope's
outer JSON/GraphQL/proto shape, not the grammar itself, and match the same generic pattern already
present in `stdio.binary raw`'s equivalent leaves.

## 8. Test laws — all 6 present, all green

`cargo test -p semio-s-plugin-stdio --lib "artifacts::las"` → **21 passed, 0 failed**:
- `mutation_diff_law`, `inverse_law`, `absorb_law` + `absorb_law_associativity`,
  `between_roundtrip_law`, `codec_retention_law` (real `example.las` fixture, 287 bytes),
  `field_sweep_covers_every_mutable_field` — all in
  `🧬️mutations/🦀️component.rs::tests` (zip's file-organization precedent: law tests live
  alongside the mutation enum + diff builders they exercise, not in the engine file).
- `absorb_law` explicitly covers Insert+Remove-before, Insert+Insert-same-index (both survive),
  Add+SetField (patch-into-added), Modify+Remove, and an annihilate-own-insert case — for BOTH
  `vlrs` and `points` independently.
- `field_sweep`: `sweep_a`/`sweep_b` differ in every one of the 25 header scalars, `vlrs` SHRINKS
  a->b (2→1, exercising `removed` forward / `added` backward), `points` GROWS a->b (2→3,
  exercising `added` forward / `removed` backward) — avoiding the exact same-length-collection
  structural trap F1's txt agent hit (documented in `f1-closer-report.md` §4.4); both tri-state
  point fields (`gps_time`, `rgb`) are exercised going both `Some→None` and `None→Some`.
- 7 pre-existing + 2 new engine tests (`⚙️engine/🦀️component.rs::tests`) all pass.

Grep gates: `snapshot: Option<` — zero real hits in the diff file (the one match is a doc-comment
describing the OLD template). `impl DiffAlgebra` — present.

## 9. Full-crate gate

`cargo test -p semio-s-plugin-stdio --lib` (no filter): **794 passed, 1 failed**. The 1 failure is
`artifacts::stl::standards::v_ascii::subsets::any::schema::mutations::component::tests::
field_sweep_covers_every_mutable_field` — confirmed NOT mine: `git status` shows `stl`'s entire
schema/engine/facet tree mid-edit by a concurrent live session (30+ modified files, none of which
reference `las::` anywhere), and the failure message ("vertices must be diffed") is stl's own
mesh-vertex terminology, unrelated to anything las touches or shares. Per the ticket's own
guidance ("if it's stl/obj/ply/las/bmp AND not yours, it might be real cross-artifact fallout from
your own change — fix that, it's your responsibility"): I verified las never touched any stl file
and shares no type with it (the plan's only noted stl/ply shared-type defect is
`MeshVertex`/`MeshTriangle`, which las has no analog of) — this is stl's own sibling F2 agent's
in-progress work, not fallout from las. Left untouched (editing another live session's
in-progress file would race a lost update).

## 10. Policy gate (`bun ./📜️script.ts policy`)

Filtered the regenerated `.🦑️repo/⚡️cache/breaches/compose.json` for the 4 S-8 rule kinds scoped
to `☁️las`:
- `stdio-artifacts/diff-algebra`: 1 hit, `-stale-` (already-fixed, allowlist entry not yet pruned).
- `stdio-artifacts/field-sweep-presence`: 1 hit, `-stale-`.
- `stdio-artifacts/grammar-honesty`: 7 hits, all `-stale-` (the 7 snapshot-facet grammar leaves I
  rewrote honestly this wave).
- `stdio-artifacts/facet-mirror-drift`: **0 hits** (real or stale).

**Zero real (non-stale) S-8 breaches for las.** Per my ownership boundary I did NOT edit
`📜️script.ts`'s allowlists myself (closer-only, matching F1's C1 closer precedent) — the 9 stale
entries above are ready for the F2 closer to prune in one pass.

## 11. Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/{🟦️component.ts,🔣️component.json,🔗️component.graphql,🛰️component.proto}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/{🟦️component.ts,🔣️component.json,🔗️component.graphql,🛰️component.proto}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/{🥋️component.ksy,🔠️component.abnf,🌶️component.spicy,📡️component.protocol.semio}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/{🅰️component.g4,🔤️component.ebnf,📖️component.grammar.semio}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/{🟦️component.ts,🔣️component.json,🔗️component.graphql,🛰️component.proto}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{🟦️component.ts,🔣️component.json,🔗️component.graphql,🛰️component.proto}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`

All 30 paths above verified against `git status --porcelain` (§13) and the correct
`🏅️standards` path segment throughout — see §12 deviation 4 for a transient authoring-tool typo
that was caught and fully corrected before this report was written (no stray files remain).

## 12. Deviations

1. `file_source_id`/`global_encoding`/`project_id_guid` (20 bytes, spec-real) are NOT modeled as
   typed fields — out of the ticket's explicit contracted field list. `decode_las` skips over them
   (never indexed); `encode_las` leaves them zero. Flagged for a future enrichment wave if desired.
2. `LasPoint` keeps formats 2/3 (`rgb`) even though the ticket's contracted scope says "0/1" —
   kept because it was already-working, already-tested content; removing it would violate the
   recipe's "nothing real on disk silently dropped" rule.
3. Diff/mutations facets' nested text/binary grammar leaves (8 files) left as pre-existing
   placeholders — matches F1's own documented, accepted scope boundary (not live-wired; real
   OpText/OpBinary grammar work is F6's mandate).
4. **Transient path typo** (self-corrected, no lasting effect): during this session two
   `Write` calls were accidentally issued with the `🏅️standards` path segment corrupted to the
   Chinese characters `🏅️标准`, creating a stray 2-file directory tree
   (`☁️las/🏅️标准/...`). Caught immediately via `find`/`ls`; `rm -rf`'d the stray tree and
   re-wrote the correct files under the real `🏅️standards` path. Verified via
   `find "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las" -iname "*标准*"` (empty) and the final green
   compile/test run that no trace remains on disk.
5. `number_of_vlrs`/`number_of_point_records`/`header_size`/`offset_to_point_data`/
   `point_data_format_id`/`point_data_record_length` are typed+diffable but always recomputed at
   `encode_las` time (STRUCTURAL, see `LasHeader`'s doc comment) — a `SetSnapshot`/`SetBounds`-style
   diff that only touches these 6 fields is meaningful in-memory (proven by `mutation_diff_law`)
   but does not survive an encode→decode round trip in isolation from the real `vlrs`/`points`
   content that determines them. This mirrors the pre-existing `header_size=227` precedent and is
   documented in the struct's own doc comment.

## 13. Verification commands run

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::las"` → 21 passed, 0 failed.
- `cargo test -p semio-s-plugin-stdio --lib` (full crate) → 794 passed, 1 failed (stl, not mine).
- `bun ./📜️script.ts policy` → regenerated `compose.json`, filtered for the 4 S-8 rules scoped to
  `☁️las` → 0 real breaches (9 stale allowlist entries ready for closer pruning).
- `grep -n "snapshot: Option<" .../🔺️diff/🦀️component.rs` → 1 hit, doc-comment only (not code).
- `grep -n "impl DiffAlgebra" .../🔺️diff/🦀️component.rs` → present.
- `git status --porcelain -- "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs" "📜️script.ts"` →
  both show pre-existing modifications from other sessions; `git diff --stat` on `glue.rs` shows
  621 insertions none of which are mine (confirmed via `grep las::` returning nothing new I added).
