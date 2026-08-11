# STATUS

Append-only real-state log for ticket `26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT`.

## 2026-08-11 — W2b closer

**Outcome: W2b CLOSED.** Fixed all verifier-flagged compile bugs across the 7 W2b subsets
(document, image, video, audio, animation, presentation, workflow), discovered and fixed 2
additional test-fixture bugs (workflow, animation) found only once the crate finally compiled,
built the `✳️any` envelope subset end-to-end (real `SemioSnapshot`/`SemioDiff`/`SemioMutation`
tagged unions over all 13 domain subsets, real `ArtifactComposer` + `SubsetValidator`, all 8
laws), and burned down 8 `POLICY_DIFF_COMPLETENESS_ALLOWLIST` entries in `📜️script.ts`.

- `cargo test -p semio-s-plugin-stdio --lib`: **1483 passed, 14 failed** (crate now compiles —
  it did not before this closer's session). All 14 remaining failures are OUTSIDE W2b's scope:
  5 in `brep`/`mesh`/`model` (W2a subsets — real bugs, discovered only now that the crate
  compiles, left untouched per this ticket's write-scope rules) and 9 in `csv`/`json` (unrelated
  format artifacts, foreign to this ticket entirely — pre-existing). Zero failures remain in
  `document`/`image`/`video`/`audio`/`animation`/`presentation`/`workflow`/`✳️any`.
- `bun ./📜️script.ts policy`: **21524 high-priority breaches across 25 rules** (verifier's own
  snapshot immediately prior was 21523 — net +1, ordinary concurrent-wave churn, not from this
  session's edits: this session's own 7 subsets + `✳️any` each carry exactly 2 breaches, the same
  pre-existing sanctioned patterns every other real subset in this program carries, with `image`
  at 13 — a known structural outlier flagged by the verifier, documented as a follow-up below, not
  fixed this session).
- W2a status: **no `w2a-close-report.md` exists yet** — W2a has not formally closed. Direct code
  inspection (not just report-reading) confirmed all 6 W2a subset snapshot types (brep, mesh,
  model, object, cad, drawing) are real, substantial, non-scaffold implementations (5 have their
  own `w2a-<subset>-report.md`; `cad` has none but its code is fully real, matching every other
  complete subset's shape) — the `✳️any` envelope was built against these real types, NOT faked.
  This should be independently reconfirmed once `w2a-close-report.md` lands.

See `w2b-close-report.md` for full detail.

## 2026-08-11 — W3 closer

**Outcome: W3 CLOSED.** Fixed all 4 verifier-flagged compile bugs across the 4 W3 sub-agents'
scopes (mp4+avi, mp3+wav, epw+tsv, html — 7 new format artifacts), unblocking crate-wide compile
for the first time in this ticket's W3 phase. Once tests could actually run, discovered and fixed
9 more real bugs (mp4 sample-entry encoder byte-count, 2 mp4/avi/mp3 test-fixture bugs, wav's
internally-tagged-enum serde limitation, wav's fixture-rounding mismatch, epw/tsv's
impossible-simultaneous-removed-and-added test assertions) — none design-judgment, all mechanical
and self-contained within their own artifact's tree. Removed 10 now-satisfied
W1b-seeded shrink-only allowlist entries from `📜️script.ts` (5×`POLICY_DIFF_COMPLETENESS_ALLOWLIST`,
5×`POLICY_ROUND_TRIP_TEST_ALLOWLIST`), verifying each before removal and confirming zero regression
via a before/after policy diff. Spot-checked `catalog.json`'s `depends` arrays for all 7
artifacts — accurate, no fix needed (mp4/avi/mp3/wav → `binary`; epw/tsv/html → `txt`; confirmed via
grep for cross-artifact imports and `DEP_` composer dialects, none found beyond the primitives).

- `cargo test -p semio-s-plugin-stdio --lib`: **1484 passed, 13 failed**. All 13 failures are
  OUTSIDE W3's scope (4 `csv`, 5 `json`, 4 `semio` `mesh`/`model` — confirmed foreign via
  `git status --porcelain`, stable across two consecutive runs). **Zero failures anywhere in
  mp4/avi/mp3/wav/epw/tsv/html** — each artifact's own scoped test suite (129 tests total across
  the 7) passes 100% clean (25/19/17/16/13/14/25).
- `bun ./📜️script.ts policy`: **21524 high-priority breaches across 25 rules** — byte-identical to
  the pre-allowlist-edit snapshot, confirming the 10 removed entries introduced zero regressions.
- Two allowlist entries deliberately kept (not stale): mp4/avi's `POLICY_DIFF_COMPLETENESS_ALLOWLIST`
  rows (both artifacts genuinely lack a `DiffCodec` impl for their Diff type — real open gap, see
  follow-up in `w3-close-report.md` §7) and html's `POLICY_ROUND_TRIP_TEST_ALLOWLIST` row (html's
  real round-trip test lives in its snapshot file, not its engine file — a rule/architecture
  mismatch, also documented as a follow-up).

See `w3-close-report.md` for full detail.

## 2026-08-11 — W2a closer

**Outcome: W2a CLOSED.** object/cad/drawing were already clean per `w2a-verify-report.md`. Fixed
all 3 real bugs the verifier found in brep/mesh/model, plus a 4th latent bug (mesh mutation-level
`RemoveX` inverse losing position) exposed only once the diff-level fix landed:

- **brep**: `field_sweep` test-fixture bug (`sweep_b`'s `e1.end_vertex` didn't actually differ from
  `sweep_a`'s) — fixed the fixture.
- **mesh**: `NamedTripleDiff.added` had no positional fidelity (same root cause `object` already
  found and fixed in its own subset) — ported `object`'s local `NamedAdded<T>{index,item}` wrapper
  fix into mesh's diff engine, switched `DiffAlgebra::inverse` to the generic
  `mid=apply(base); between(mid,base)` derivation, and fixed 4 `RemoveMesh`/`RemovePrimitive`/
  `RemoveMaterial`/`RemoveTexture` mutation-level inverses to preserve original position (same
  remove-tail/re-add technique `object`'s own `RemoveMapEntry` inverse uses).
- **model**: `op_text_binary_roundtrip_law` double-`Option` serde bug on `SetElement.spatial_id`
  (verifier-confirmed) and `SetSpatialNode.parent_id` (same shape, fixed proactively) — standard
  `skip_serializing_if` + `deserialize_with` workaround.
- **cad**: backfilled the missing `w2a-cad-report.md` from direct inspection (code itself needed no
  changes — already real and passing).

Removed all 6 of W2a's now-satisfied `POLICY_DIFF_COMPLETENESS_ALLOWLIST` entries from
`📜️script.ts` (brep/cad/drawing/mesh/model/object), each verified to carry a real
`impl protocol::DiffCodec for Semio<X>Diff` before removal, re-run confirmed zero regression.

- `cargo test -p semio-s-plugin-stdio --lib`: **1491 passed, 6 failed** (up from W2b/W3's 1483-1484
  passed / 13-14 failed — the 5 semio failures W2b's closer flagged as "outside scope, W2a's to
  fix" are now gone). All 6 remaining failures are `csv`/`json` standards engines — entirely
  outside this ticket. **Zero failures anywhere in `artifacts::semio::standards::v1::subsets::
  {brep,mesh,model,object,cad,drawing}`** (111 tests across the 6 scoped runs: 17/21/15/32/13/13,
  all green).
- `bun ./📜️script.ts policy`: **21524 high-priority breaches across 25 rules** — byte-identical
  total to the verifier's own snapshot immediately prior; direct inspection of
  `.🦑️repo/⚡️cache/breaches/compose.json` confirms zero new breaches for any of the 6 subsets and
  zero `dsl-migration/diff-completeness` breaches remaining for them (the 6 allowlist removals were
  clean).
- Deferred (real design/policy-authoring judgment, not fixed here): grammar-honesty allowlisting for
  binary leaves, facet-mirror-drift allowlisting for `🔺️diff` facets, and the shared
  `⚙️engine/🧰️triples::NamedTripleDiff<K,D,T>` gaps (spurious `T: Default` bound + no built-in
  positional `added` wrapper) that brep/mesh/model/object each independently worked around locally —
  all documented as follow-ups in `w2a-close-report.md` §5, none touched (editing shared engine
  files or repo-wide allowlist policy is outside a single closer's scoped write access this session).

See `w2a-close-report.md` for full detail.

## 2026-08-11 — W4 closer

**Outcome: W4 CLOSED.** 26 real format-pair bridges (52 leaf files: brep↔step; mesh↔gltf/stl/obj/
ply/las; model↔ifc/bcf; object↔json/xml/csv; cad↔step (+dxf/dwg per G4); drawing↔svg/dxf/pdf;
image↔png/jpg/gif/bmp/tiff; video↔mp4/avi; audio↔mp3/wav; animation↔gltf/mp4/gif;
document↔docx/md/txt/pdf; presentation↔pptx; workflow↔json) delivered across 6 parallel groups, all
confirmed real/honestly-documented/zero-codec-reimplementation by `w4-verify-report.md`'s 12-pair
sample (50% over the required minimum). Real (lossless-in-scope) mappings: object↔json,
workflow↔json (both genuinely lossless), audio↔wav (byte-exact PCM), mesh↔ply (real indexed
round-trip). Documented-lossy (real mapping, honest gaps, never fabricated): brep↔step (ref_direction
rotation, same_sense), model↔ifc (geometry/name fields out of scope), model↔bcf (issue-tracker vs.
spatial — narrow by design), mesh↔gltf/obj/stl/las (index-structure/material/format-cardinality
losses), drawing/document↔pdf (flat structure, page boundaries only), presentation↔pptx (media/table
shapes), video↔mp4/avi, audio↔mp3 (decode-only, no encoder — zero codec reimplementation), animation↔
gltf/mp4/gif (tangent/cardinality losses) — every loss itemized in-file, never silently defaulted.

Fixed the one real bug the verifier found (§2 of `w4-verify-report.md`): `pdf`'s shared
`⚙️engine/🦀️component.rs::extract_text` never emitted a newline for the `T*` (move-to-next-line)
content-stream operator, so multi-line text written by `encode_pdf` (which emits `T*` between every
line's `Tj`) came back joined with no separator on decode — a spec-correct one-arm fix
(`"T*" if in_text => out.push('\n')`, PDF32000-1 §9.4.2), not specific to any one leaf. Fixed the
shared-infra gap flagged since W2 in `⚙️engine/🧰️triples/🦀️component.rs`: added the
`#[serde(bound(...))]` override bcf's own local copy already used (stops a spurious `T: Default`
bound on `IndexedTripleDiff`/`NamedTripleDiff`) and hoisted `object`'s local `NamedAdded<T>
{index,item}` positional-fidelity wrapper (+ generic `enc_named_added`/`dec_named_added` codec
helpers) into the shared file as the new canonical copy — the 5 subsets with local workarounds
(bcf, brep, mesh, model, object) were deliberately left untouched (still correct), only future W4/W5
consumers benefit. Checked `📜️script.ts` for now-satisfied shrink-only allowlist entries tied to
io-leaf coverage/composer-dependency: verified there were none to remove (both rules are fully
computed, not allowlist-gated, and already show zero breaches; no allowlist anywhere names any of
W4's 12 subsets or their format pairs) — no speculative edits made.

- `cargo test -p semio-s-plugin-stdio --lib`: **1657 passed, 0 failed, 1 ignored** — fully green
  (up from the verifier's 1645/10-failed snapshot: the pdf fix cleared the 1 real failure, and the 9
  foreign png/zip conformance-law failures the verifier saw from a different concurrent session's
  in-progress work were already resolved by that session before this closer's own gate run).
- `bun ./📜️script.ts policy`: **21532 high-priority breaches across 25 rules** — byte-identical to
  the verifier's own snapshot, confirming both fixes introduced zero new breaches and zero
  regressions.
- Not fixed, documented as a follow-up for W5: **G4 (drawing/cad/image) never filed its required
  `w4-*-report.md`** (a real CLAUDE.md process gap; its underlying code is real and substantial per
  direct inspection by both the verifier and this closer, so nothing to fix there beyond the missing
  paper trail — not backfilled here, out of a closer's scope to author another group's first-person
  report).

See `w4-close-report.md` for full detail.
