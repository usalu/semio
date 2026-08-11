# F6 — 🖊️dwg / ac1024 — OpText/OpBinary + DiffCodec

**Artifact**: 🖊️dwg, standard `ac1024` ONLY. `ac1018` (sibling standard) untouched — a different
agent owns it, per the sub-wave split.

## Summary

Both sides — `DwgDiff` (`DiffCodec`) and `DwgMutation` (`OpText`/`OpBinary`) — landed on the
**DERIVE** path, verified for real by actually compiling and running the tests, not assumed from
the recon report's "probable" classification. This matches the recon table's guess (row 13:
"Same family as ac1018 ... DERIVE (probable)"), now confirmed independently rather than trusted.

## STEP 1 — classification (verified for real)

Walked `DwgDiff`'s field tree recursively (`DwgDiff` → `DwgSectionsDiff` → `DwgSectionModified`/
`DwgSectionAdded` → `DwgSectionDiff`/`DwgSection` → `DwgSectionPage`) per f6-recon-report.md §3's
decision rule:

- **Tri-state (`Option<Option<_>>`) check**: zero occurrences anywhere in the tree. Every nullable
  field in `DwgDiff` (`version`/`maintenance_version`/`codepage`/`bytes`/`sections`) and in
  `DwgSectionDiff` (`compressed`/`declared_size`/`pages`) is a single-layer `Option<T>` meaning
  "the new value if changed" — none of them encode tri-state removal semantics, because none of
  the underlying `DwgSection`/`DwgSnapshot` fields are themselves nullable in the first place.
  `DwgSectionPage.error: Option<String>` is the one genuinely-nullable leaf field anywhere in the
  tree, and it's a plain single-layer `Option<String>` (the `OptionScalar` derive arm), not
  tri-state.
- **Data-carrying-enum check**: zero. `DwgDecodeStatus` is the only enum reachable from either
  tree (via `DwgSnapshot` on the Mutation side's `SetSnapshot`; it never appears in `DwgDiff` at
  all — see the diff module's own doc comment, it's a DERIVED field, deliberately excluded from
  the diff). It is unit-variant-only (`SentinelOnly`/`SectionsLocated`/`SectionsDecompressed`, no
  payload on any variant) — exactly the shape `#[derive(dsl::DslScalar)]` supports.

Confirmed via real `cargo check -p semio-s-plugin-stdio --lib`: **zero compile errors** after
adding the derives (§ below) — no `DslField is not implemented for X` errors anywhere in the dwg
ac1024 files, on either the Diff or the Mutation side.

## STEP 2a — DERIVE path applied

### Snapshot side (cascading requirement — every nested struct needs `DslRecord` too)
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
- `dsl::DslRecord` added to `DwgSectionPage` (+ `#[dsl(base64)]` on `decoded: Vec<u8>`),
  `DwgSection`, `DwgSnapshot` itself (so `DwgMutation::SetSnapshot{snapshot: DwgSnapshot}`
  compiles under `DslOps`).
- `#[dsl(base64)]` added to `DwgSnapshot::bytes: Vec<u8>` (bare `Vec<u8>`, gets the compact
  grammar, unlike the `Option<Vec<u8>>` case below).
- `dsl::DslScalar` added to `DwgDecodeStatus` (unit-variant enum).

### Diff side
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`:
- `dsl::DslRecord` added to `DwgSectionDiff`, `DwgSectionModified`, `DwgSectionAdded`,
  `DwgSectionsDiff`.
- `dsl::DslDiff` added to `DwgDiff` — **fully generates `protocol::DiffCodec`** (`print_diff`/
  `parse_diff`/`encode_diff`/`decode_diff`). No hand-written `impl DiffCodec` needed.
- **Deviation from the recon report's §5 base64 note**: `DwgDiff::bytes: Option<Vec<u8>>` hits
  the documented derive quirk (`#[dsl(base64)]` is silently a no-op through one `Option` layer,
  falls back to `Shape::List(UInt)` — a bracketed list of decimal byte values). Left `#[dsl(...)]`
  off this field entirely (adding it would be misleading — it does nothing) and documented the
  quirk in a doc comment on `DwgDiff` itself. Harmless for this ticket's small test fixtures; the
  real 145KB `architectural.dwg` fixture is never round-tripped through `print_diff`/`encode_diff`
  in any test (only through `DwgSnapshot`'s own separate hand-rolled `ArtifactDsl`/`ArtifactPack`
  hex/binary envelope, which is untouched — `codec_retention_law` still exercises that path and
  still passes).

### Mutation side
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`:
- `dsl::DslOps` added to `DwgMutation` — emits `dsl::DslVariants` only (P6 — `OpText`/`OpBinary`
  always handcrafted regardless of derive success, per the recon report §1/§2).
- `#[dsl(block)]` added to the two struct-valued variant fields (`SetSnapshot.snapshot`,
  `InsertSection.section`) for readability, matching the `BinaryMutation`/`GifMutation` precedent.
- The old `serde_json`-based `OpText`/`OpBinary` stub impls **replaced** with the §2 handcrafted
  wrapper (copied verbatim from the recon report's template / `BinaryMutation`'s real
  implementation): `parse_op`/`print_op` via `DslVariants::variants()`/`dsl::parse`/`dsl::print`,
  `encode_op`/`decode_op` as a pure forward to `dsl::variants_binary::encode_op`/`decode_op`.

No mirror-enum (`FlowMutationDsl`-style) was needed — `DwgMutation` derives `DslOps` directly,
same as `BinaryMutation`/`GifMutation`, since nothing in its variant closure carries a
data-carrying enum.

## STEP 3 — tests added

Both are **new tests, added to existing test modules** (no new test files, per CLAUDE.md):

- `diff_codec_text_binary_roundtrip_law` — added to a brand-new `#[cfg(test)] mod tests` block at
  the end of the diff file (none existed there before; all prior tests for this artifact lived in
  the mutations file). 3 cases: `DwgDiff::default()`, a diff exercising every scalar field plus
  all three arms of the `sections` triple at once (`removed`+`modified`+`added`, with the
  `modified` entry's `pages` including a page with `error: Some(...)` to cover the one genuinely-
  nullable leaf field), and a diff built via the existing `diff_set_version_info` builder.
- `op_text_binary_roundtrip_law` — added to the mutations file's existing `tests` module. Covers
  all 6 `DwgMutation` variants: `NoMutation`, `SetSnapshot` (whole nested `DwgSnapshot`, reusing
  `base_snapshot()`), `SetVersionInfo`, `InsertSection` (nested `DwgSection` with a page carrying
  `error: Some(...)`), `RemoveSection`, `SetSectionData`.

Both tests assert `!printed.contains('\n')`, `parse(print(x)) == x`, `decode(encode(x)) == x`,
exactly per the recon report's STEP 3 spec.

Real captured output (via a temporary `eprintln!` added then removed before the final test run —
not left in the committed test code):

```
print_diff => version=AC1032 maintenance-version=9 codepage=65001 bytes=[ 170 187 204 ] sections=removed=[ gone ] modified=[ name=stay diff=compressed=false declared-size=999 pages=[ page-number=0 file-address=2304 compressed-size=50 decoded="YWZ0ZXI=" page-number=1 file-address=2305 compressed-size=10 decoded="" error="truncated page" ] ] added=[ index=2 section=name=new compressed=true declared-size=5 pages=[ page-number=2 file-address=48 compressed-size=5 decoded="YnJhbmQgbmV3" ] ]

print_op => set-snapshot snapshot { schema=stdio.dwg version=AC1024 maintenance-version=2 codepage=30 bytes="QUMxMDI0AAAAAAAAAAAAAAAAAh4AAA==" decode-status=sections-decompressed section-names=[ "AcDb:Header" "AcDb:Classes" "AcDb:Handles" ] sections=[ name="AcDb:Header" compressed=true declared-size=100 pages=[ page-number=0 file-address=512 compressed-size=50 decoded="aGVhZGVyLWJ5dGVz" ] name="AcDb:Classes" compressed=true declared-size=200 pages=[ page-number=1 file-address=768 compressed-size=80 decoded="Y2xhc3Nlcy1ieXRlcw==" ] name="AcDb:Handles" compressed=false declared-size=40 pages=[ page-number=2 file-address=1024 compressed-size=40 decoded="aGFuZGxlcy1ieXRlcw==" ] ] }

print_op => insert-section index=1 section { name="AcDb:Template" compressed=true declared-size=10 pages=[ page-number=9 file-address=2304 compressed-size=10 decoded="bmV3" page-number=10 file-address=2305 compressed-size=3 decoded="" error="bad page" ] }
```

Note `bytes=[ 65 67 49 48 50 52 0 0 ... ]` (the diff-level `Option<Vec<u8>>` verbose-list quirk)
vs. `bytes="QUMxMDI0..."` (the snapshot-level bare `Vec<u8>` compact base64) in the same test run
— real, observed confirmation of the recon report's §3b quirk note.

## STEP 4 — verification (real, this session)

- `cargo check -p semio-s-plugin-stdio --lib` — 0 errors (initial attempt hit 4 pre-existing
  errors in `🎞️gif/🏅️standards/🔖️87a` — `v87a`/`GifColorTable`/`GifImagesDiff` `DslField` bounds
  — from a concurrent sibling session working that artifact; not caused by, or related to, any
  file this session touched; resolved on its own by the next check, confirming it was transient
  concurrent churn, not a bug in this work).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::dwg::standards::v_ac1024"` (module path
  confirmed via the artifact's own `📸️snapshot`/`🔺️diff` file paths and matches the task brief) →
  **18/18 passed, 0 failed** — includes the 2 new law tests, all pre-existing F5 tests
  (`mutation_diff_law`, `inverse_law`, `absorb_law`, `absorb_law_associativity`,
  `between_roundtrip_law`, `field_sweep_covers_every_mutable_field`,
  `out_of_range_section_mutation_is_noop_not_panic`), the `engine` module's own tests, and —
  critically — **`codec_retention_law` still passes**: the real 145KB `architectural.dwg` fixture
  still decodes → re-encodes byte-identical, confirming this session's derive additions didn't
  touch (and can't break) the artifact's actual lossless byte-level codec.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1042 passed, 2 failed**. Both
  failures are in `artifacts::stl::standards::v_ascii::...` (`diff_codec_text_binary_roundtrip_law`,
  `op_text_binary_roundtrip_law`) — a different artifact, owned by a different concurrent sibling
  session (confirmed via `git status`: only `🖼️bmp`/`🟪️stl` files show as modified beyond this
  session's own `🖊️dwg`/`ac1024` changes; this session made zero edits to either). A first
  full-crate run mid-session additionally showed a 3rd failure in `🖼️bmp` that had disappeared by
  the final run — direct evidence of another session's in-progress edits landing between runs,
  not anything caused by dwg's changes. **Zero dwg-related failures at any point.** Pass count
  went from the recon report's stated 1019 baseline up to 1042 (net +23, consistent with several
  other F6 sibling agents' work landing concurrently in the shared tree plus this session's own +2)
  — never down, satisfying STEP 4's "count only goes up" requirement for this session's own
  contribution.

**Addendum (end-of-session re-check)**: a final re-run of `cargo check`/`cargo test` immediately
before closing this report hit a *different* set of transient errors — `StlSnapshot`/
`StlTriangle: DslField` — again 100% inside `artifacts::stl::standards::v_ascii`, again zero
`dwg`/`ac1024` mentions. This is the SAME concurrent `stl`-owning sibling session continuing to
iterate on its own `DslRecord`/`DslDiff` derive attempt (its earlier state, captured in
`f6-dwg-ac1024-fullcrate-test-final.txt`, showed it as 2 RUNTIME test failures; now it's mid-edit
again and doesn't compile at all) — expected live-tree churn per this ticket's own repo rules
("others working simultaneously"), not a regression in this session's `dwg`/`ac1024` work. This
session's own `dwg`/`ac1024` files were re-confirmed unchanged and untouched by this churn (`git
status` shows only the 3 files listed in "Files touched" below as modified by this session, stl's
files modified separately by the other session). Given the "18/18 scoped, 1042/2 whole-crate
(0 dwg failures)" clean snapshot already captured earlier in this same session (STEP 4 above), and
that this task's mandate is strictly `dwg`/`ac1024`'s own `OpText`/`OpBinary`/`DiffCodec` work
(not fixing sibling artifacts mid-edit by other sessions), no further action was taken on `stl`.

## Deviations from f6-recon-report.md §5's conventions

1. `DwgDiff::bytes: Option<Vec<u8>>` does not get the compact base64 grammar (documented derive
   quirk — `#[dsl(base64)]` is a no-op through one `Option` layer). Not applicable to the
   hand-roll grammar at all since this artifact stayed fully on the derive path; noted as a
   doc-comment caveat on `DwgDiff` instead. No `#[dsl(base64)]` attribute was added to that one
   field (adding it would silently do nothing and mislead a future reader).
2. No hand-rolled hex/split_top_level/encode_option primitives were needed anywhere — this
   artifact never left the derive path on either side, so §5's template was not exercised.
3. Added a brand-new `#[cfg(test)] mod tests` block to the diff file (none existed there
   previously) rather than only extending the mutations file's tests module — necessary because
   `DiffCodec`'s round-trip law naturally belongs beside `DwgDiff`'s own type definition, and no
   pre-existing test module existed in that file to extend into. This still satisfies CLAUDE.md's
   "extend existing files, no new test files" rule (same file, new region within it).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  — `dsl::DslRecord` on `DwgSectionPage`/`DwgSection`/`DwgSnapshot`, `dsl::DslScalar` on
  `DwgDecodeStatus`, `#[dsl(base64)]` on `DwgSectionPage::decoded` and `DwgSnapshot::bytes`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — `dsl::DslRecord` on `DwgSectionDiff`/`DwgSectionModified`/`DwgSectionAdded`/`DwgSectionsDiff`,
  `dsl::DslDiff` on `DwgDiff` (fully derived `DiffCodec`, replacing nothing — no hand-rolled impl
  existed before), new `#[cfg(test)] mod tests` block with `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — `dsl::DslOps` on `DwgMutation`, `#[dsl(block)]` on `SetSnapshot.snapshot`/
  `InsertSection.section`, handcrafted `OpText`/`OpBinary` impls replacing the `serde_json` stubs,
  new `op_text_binary_roundtrip_law` test added to the existing `tests` module, `#[cfg(test)] use
  protocol::{OpBinary, OpText};` import added.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-dwg-ac1024-check1.txt` (first `cargo
  check`, showed 4 pre-existing unrelated `gif87a` errors from a concurrent sibling session),
  `f6-dwg-ac1024-check2.txt` (clean `cargo check` after this session's edits), `f6-dwg-ac1024-test1.txt`
  (scoped 18/18 pass), `f6-dwg-ac1024-fullcrate-test1.txt`/`f6-dwg-ac1024-fullcrate-test-final.txt`
  (whole-crate runs, both showing only unrelated `bmp`/`stl` failures from concurrent sessions,
  zero dwg failures).

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework
crates, `🏪️store`, and `ac1018`'s files were all read-only (glue.rs/script.ts/framework read for
reference per the recon report's own citations; ac1018 read zero times — not needed, ac1024's
classification was verified independently rather than by comparison).
`POLICY_DIFF_COMPLETENESS_ALLOWLIST` (📜️script.ts:2304) — not touched, per instructions.
