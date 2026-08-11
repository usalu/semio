# F5 — dwg (ac1018 + ac1024) — Report

Plan: `~/.claude/plans/the-current-schemas-are-scalable-journal.md`. Recipe: `🧬️schema-design.md`. W0 recon: `w0-recon-report.md` §7/§8. S2 spine (glue-mounting policy resolution, load-bearing for this report): `s2-spine-report.md`.

## 1. Scope confirmation (both standards, per the brief's explicit boundary)

- **ac1018**: confirmed a deliberately frozen legacy shim (Decision #5, documented in its own `DwgArtifact::to_snapshot`) — never brought toward ac1024 decode parity. Its own `decode_dwg` only recognizes the 6-byte `AC10xx` version sentinel and detects section NAMES (substring scan over `KNOWN_SECTIONS`, or a fixed-offset label-table fallback) — no byte ranges, no per-section content. Kept exactly that scope; only added the two new header fields (see §2) and gave it a real diff/mutations layer.
- **ac1024**: confirmed real D1 (file-header decrypt + section/page location) and D2 (LZ77-variant decompression) — all 13 real sections on the 145KB `architectural.dwg` fixture locate and decompress cleanly (re-verified: `real_fixture_d1_locates_every_named_section`, `real_fixture_d2_decompresses_every_section`, `real_fixture_page_directory_matches_header_cross_check` all green). D3-D5 untouched, as instructed — no new codec depth added, only snapshot/diff/mutation enrichment within the existing honest boundary.

## 2. New real header fields: `maintenance_version: u8`, `codepage: u16`

Externally verified (not fabricated) via LibreDWG's own `header.spec` (fetched live from `github.com/LibreDWG/libredwg`, cross-checked against the real `architectural.dwg` fixture's raw bytes):

```
FIELD_RC (zero_one_or_three, 0);        // @0x0B — NOT what we want (a different flag byte)
FIELD_RL (thumbnail_address, 0); //@0x0d
FIELD_RC (dwg_version, 0);              // @0x11
FIELD_RC (maint_version, 0);            // @0x12  <- maintenance_version
FIELD_RS (codepage, 0); //@0x13: 29/30 for ANSI_1252, since r2007 UCS-16   // @0x13, LE u16
```

Cross-check against the real fixture: byte 0x12 = `0x02`, bytes 0x13-0x14 = `1e 00` → codepage `30`, an EXACT match for the spec's own documented "29/30 = ANSI_1252" note (the single most common real-world codepage) — strong evidence the offsets are correct, not guessed. Both offsets sit in the PLAIN (unencrypted) preamble shared by every AC1015+ file, before the encrypted R2004+ system section at 0x80 that D1 already decrypts — reading them required zero new codec depth (same complexity class as the existing 6-byte version-sentinel read), so this does not violate "no new codec depth for ac1024" or "do not expand what ac1018 decodes."

`decode_dwg` (both standards) now populates both fields via a shared-shape `parse_version_header_fields(bytes) -> (u8, u16)` helper (independently duplicated per standard, since each standard's snapshot module is self-contained — see §6 for why cross-standard sharing isn't structurally available here). Graceful zero-default when `bytes` is too short to reach the offsets (never a hard error).

## 3. Snapshot — completeness per the brief's target shape

**ac1018** (`DwgSnapshot`): `schema, version, maintenance_version: u8, codepage: u16, bytes: Vec<u8>, section_names: Vec<String>` — unchanged shape otherwise (per Decision #5, no `sections`/`decode_status`, honestly opaque).

**ac1024** (`DwgSnapshot`): same header fields added, plus the existing real D1/D2 structural model kept as-is: `sections: Vec<DwgSection{name, compressed, declared_size, pages: Vec<DwgSectionPage>}>`, `decode_status: DwgDecodeStatus`. `section_names`/`decode_status` are DERIVED from `sections` (new `derive_section_names`/`derive_decode_status` helpers, extracted from the pre-existing inline logic in `decode_sections`, now reused by `decode_dwg`, `DwgDiff::apply`, and every section-mutating `apply_dwg_mutation` arm) — deliberately excluded from the sparse diff struct so they can never drift out of sync with `sections`.

## 4. Diff — real, handcrafted, no full-replace slot (both standards)

**ac1024**: `DwgDiff{version: Option<String>, maintenance_version: Option<u8>, codepage: Option<u16>, bytes: Option<Vec<u8>>, sections: Option<DwgSectionsDiff{removed: Vec<String>, modified: Vec<DwgSectionModified{name, diff: DwgSectionDiff}>, added: Vec<DwgSectionAdded{index, section}>}>}` — a real name-keyed triple mirroring zip's `ZipEntriesDiff` shape closely, SIMPLIFIED relative to zip's own `absorb` because section names are immutable (no rename concept exists for DWG sections), so no rename-transport map is needed in `absorb_sections`. `DwgSectionDiff{compressed, declared_size, pages}` — `pages` is whole-value replaced (weak entity, no splice mechanism), per the brief's explicit instruction.

**ac1018**: `DwgDiff{version, maintenance_version, codepage, bytes, section_names: Option<Vec<String>>}`. **Deviation from the brief's literal `sections: Vec<DwgSection{name,data}>` shape** (see §7) — `section_names` is a WHOLE-VALUE weak-entity replace, not a keyed triple. This was NOT the first design tried: an earlier revision modeled it as an add/remove name-multiset (mirroring the recipe's general "name-keyed" collection guidance) and hit a REAL, reproducible `between_roundtrip_law` failure — reconstructing "survivors keep prior relative order, new names appended at the end" does not, in general, reproduce an arbitrary target order (verified with a concrete counterexample: `between(b,a).apply(b) != a` when `a`'s first element ends up as `b`'s trailing survivor). Since `section_names` entries carry no sub-fields, no rename concept, and (per Decision #5) no position semantics any real mutation needs to preserve independent of the whole list, this is a textbook "weak entity" per the recipe's own rule ("value structs … whole-value replaced in diffs, never sub-diffed") — not a workaround, a correct application of the recipe to a field that isn't actually a keyed collection.

Both: `impl MutationDiff<DwgSnapshot> for DwgDiff { apply, absorb }` + `impl DiffAlgebra<DwgSnapshot> for DwgDiff { inverse, between, is_empty }`. `inverse` is derived generically (`Self::between(&self.apply(base), base)`, correct by construction, mirrors zip's own precedent) rather than hand-duplicating per-field undo logic. `between` is real field-by-field/name-keyed comparison. Zero `snapshot: Option<DwgSnapshot>` full-replace slots (grep-verified — only doc-comment prose describing what was deleted).

## 5. Mutations — real vocabulary (both standards)

**ac1024**: `NoMutation, SetSnapshot, SetVersionInfo{version,maintenance_version,codepage}, InsertSection{index,section}, RemoveSection{name}, SetSectionData{name,compressed,declared_size,pages}` — matches the brief's target vocabulary exactly.

**ac1018**: `NoMutation, SetSnapshot, SetVersionInfo{version,maintenance_version,codepage}, InsertSectionName{index,name}, RemoveSectionName{name}`. **Deviation**: no `SetSectionData` (nothing to set — ac1018 never has section content), and `InsertSection`/`RemoveSection` renamed to `InsertSectionName`/`RemoveSectionName` to make explicit these operate on the flat opaque name list, not a `DwgSection` struct with content — an honest reflection of ac1018's real scope, not a shortcut. `InsertSectionName` is positional (`index`, clamped to `len`) rather than append-only, precisely so `RemoveSectionName`'s mutation-level `inverse()` can restore the exact original position (found via a real `inverse_law` test failure during verification — append-only insert put the restored name at the end instead of its original middle position; fixed by adding the index parameter, mirroring ac1024's own `InsertSection{index,...}` convention).

`SetVersionInfo` (both standards) patches BOTH the typed scalar mirrors AND the underlying `bytes` at the real plain-preamble offsets (`patch_version_info_bytes`, shared by the diff-builder and the imperative `apply_dwg_mutation` arm) — keeps the existing `encode_dwg` version-consistency invariant intact rather than letting the scalars drift from the byte ground truth.

Every variant's `diff()` is handcrafted (constructs the diff directly via `schema::diff` builders) — apply-and-capture is never used. Mutation-level `inverse()` is handcrafted per variant, key/index-aware, gracefully degrading to `NoMutation` for stale/absent names (verified: `out_of_range_section_mutation_is_noop_not_panic` / `out_of_range_section_name_mutation_is_noop_not_panic`).

## 6. Real defect found and fixed: ac1018's own module tree was importing the WRONG (ac1024) types

Pre-existing (not introduced this wave), found while making the new diff/mutations code for ac1018 compile: `crate::artifacts::dwg::{DwgSnapshot, DwgDiff, DwgMutation, ...}` (the top-level re-export) is aliased to the CANONICAL richer standard per S-6 (`crate::artifacts::dwg::schema` = `standards::v_ac1024::subsets::any::schema::*`, exactly like gif 89a/pdf 1.7's own S-6 flip). ac1018's ORIGINAL (pre-this-wave) `diff.rs`/`mutations.rs` — and their `📄set-snapshot` triad leaves — used this exact top-level import for their own `DwgSnapshot` type, meaning they were silently operating on ac1024's type all along. This was harmless while both standards shared the byte-identical `DwgDiff{snapshot: Option<DwgSnapshot>}` generic template (any `DwgSnapshot` type fit the same shape), but became a hard compile error the moment ac1018 got its own genuinely different vocabulary. **Fixed**: `schema/{snapshot,diff,mutations}` and the `📄set-snapshot` triad's three leaves now import via the fully-qualified `crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::...` path, mirroring gif 87a's own established precedent for a non-canonical secondary standard.

**NOT fixed (flagged, out of scope for this wave)**: ac1018's `⚙️engine`, `🧐️analyzer`, `🏗️builder`, `🎹️composer`, and `🚪️io` files ALL still import the canonical (ac1024) types via the same top-level re-export — meaning `standards::v_ac1018::subsets::any::builder::DwgBuilder` and friends currently build/mutate/analyze AC1024-shaped snapshots, not ac1018's own. This is a genuine, deeper architectural inconsistency, but: (a) it's fully pre-existing and self-consistent (nothing in that subtree references my fixed `schema::{diff,mutations}` files, so there is zero compile interaction — confirmed via `cargo check`/`cargo test` both green), (b) fixing it would mean rewiring composer/builder/analyzer/io registration and possibly touching how ac1018 is registered as a distinct standard at all, well beyond "give ac1018 a real diff/mutations layer for its existing fields," and (c) it may in fact be intentional given ac1018's "deliberately frozen legacy shim" status (delegate operational plumbing wholesale to ac1024, keep only the schema-description layer honestly distinct). Recorded in `glue_followup` for the wave closer / a future targeted pass to decide.

## 7. Deviations from the literal brief text (both explained above, summarized here)

1. ac1018's `sections`/collection field stays named `section_names: Vec<String>` (not renamed to a `sections: Vec<DwgSection{name,data}>` shape) — no honest `data: Vec<u8>` payload exists to populate; renaming would either fabricate content or require an always-empty `data` field that misrepresents "never decoded" as "decoded to empty." Diffed as a whole-value weak replace, not a keyed triple (see §4's design-rationale).
2. ac1018's mutation vocabulary is `InsertSectionName`/`RemoveSectionName` (no `SetSectionData`) rather than the brief's literal `InsertSection`/`RemoveSection`/`SetSectionData` — reflects ac1018's real scope.
3. A trivial, mechanical, out-of-scope fix: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `fn cells_diff` was missing a lifetime parameter (`E0106`), blocking the ENTIRE crate's test binary from building for ~20 minutes (confirmed via repeated polling and via `ps aux` showing sibling F5 sessions — xlsx's and pptx's and bcf's own agents — independently stuck on the exact same blocker). Applied rustc's own suggested one-line fix (`fn cells_diff<'a>(diff: &'a XlsxDiff, sheet_name: &str) -> &'a ...`) after confirming the file hadn't been touched in 17 minutes (no active edit in progress) and that this was purely mechanical with zero semantic ambiguity — same "trivial, obviously-correct, blocking repo-wide compilation" precedent already documented in this ticket's own `STATUS.md`. Did not touch anything else in that file. By the time of the final full-crate run, xlsx's own agent had independently landed a much larger rewrite of that same file (confirmed via a mid-session file-change notification) superseding my one-line fix — no conflict, no action needed on my part.

## 8. Verification

- `cargo check -p semio-s-plugin-stdio` → clean (0 errors) after the import-path fix in §6.
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::dwg"` → **31 passed, 0 failed** (both standards' 6 law suites + engine/fixture tests). Two real bugs found and fixed DURING verification (not pre-existing, introduced-then-caught in this same session): the `between_roundtrip_law` ordering bug (§4) and the `InsertSectionName` positional-inverse bug (§5) — both are exactly the kind of thing the law suite exists to catch, and both are now fixed with the corrected design, not weakened tests.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1009 passed, 3 failed** — all 3 failures are in `pptx`/`xlsx` (2 pptx + 1 xlsx), zero in `dwg`; confirmed these are other F5 sibling agents' own in-progress work (`ps aux` showed their sessions' own polling loops running concurrently), classified as external churn per this ticket's own convention, not chased further.
- Grep gates: `snapshot: Option<` in both diff files → zero real hits (only doc-comment prose). `impl DiffAlgebra` → present in both. `field_sweep` → present in both (grep count 2/2).
- Real fixture regression (`architectural.dwg`, 149KB, real AC1024): `real_decode_reaches_d2_with_every_named_section`, `real_decode_stays_lossless_on_reencode`, plus all of D1/D2's own engine tests (`real_fixture_d1_locates_every_named_section`, `real_fixture_d2_decompresses_every_section`, `real_fixture_page_directory_matches_header_cross_check`) — all still green, unweakened.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (trivial out-of-scope lifetime fix, §7.3 — later superseded by xlsx's own agent's larger rewrite)

## Facet mirrors (deviation, matching the wave-wide precedent)

Not updated — TS/GraphQL/JSON-schema/proto mirrors for snapshot/diff/mutations remain the pre-existing stale placeholders (same defect docx's F4 report documents repo-wide, tracked by the existing shrink-only `POLICY_FACET_MIRROR_DRIFT`/`POLICY_GRAMMAR_HONESTY` allowlists). Deprioritized in favor of the Rust snapshot/diff/mutations correctness + 6 law suites (this wave's actual acceptance criterion) within the time budget, consistent with every other F4/F5 agent's documented choice.

## glue_followup

- ac1018's `⚙️engine`/`🧐️analyzer`/`🏗️builder`/`🎹️composer`/`🚪️io` subtree still imports the CANONICAL (ac1024) types via the top-level `crate::artifacts::dwg::{...}` re-export rather than its own `standards::v_ac1018::subsets::any::*` types (§6) — needs an explicit decision (fix to be genuinely ac1018-typed, or document as intentional delegation) from the wave closer or a future targeted pass; not touched this wave (self-consistent, zero compile interaction with the fixed schema files, well beyond this brief's scope).
