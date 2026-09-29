# WP-FH4 — Fault Localization Helper 4 (pass 2: P2-X, P2-Y)

Slice: FH4 (session 15, helper of S20; spec `📓️fault-localization-api.md` §6). Overlay only:
`.🧬semio/🌐hub/s14-s20-overlay-faults-p2/`. Work dir `wp-fh4/` (site list `my-sites-0.json` = my families' rows of
`.🧬semio/🌐hub/s14-s20-sets/p2/drift-sites.json` at 20:30). Source-only (no cargo), no live-tree edits, no git.

## Session 15

| Item | Families | Drift sites | State | Evidence |
|---|---|---|---|---|
| P2-X | wfc, norm, remodel, draw, cad, raster, fem, forms, space, gis, layout | 130 (+ 0 apply after S20's codemod) | **done 21:2x: ledger 0, census 0** | `python3 wp-fh4/ledger.py` → `drift sites left: 0`; census `.🧬semio/🌐hub/s14-fh4-logs/census-2.txt` (violations=0, oracle agrees) |
| P2-Y | stdio (+ laws asserting report text) | 50 + 25 apply | **done 21:2x: ledger 0, census 0** | same captures; laws converted: step cc2–cc6 (5), xml valid (4 asserts), json i-json (3), 15 apply-code laws |
| — | files touched on the p2 overlay | 455 (stdio 217, fem 111, wfc 37, remodel 34, norm 25, raster 9, cad 6, draw 5, forms 4, space 4, gis 2, layout 1) | pre-edit copies | `.🧬semio/🌐hub/s14-fh4-backup/` (same relative paths) |
| — | compile proof | — | **not run** (source-only brief; framework API P2-F1 present on the overlay since ~21:1x) | offered to S20 via main |

### Session 15 log

- 20:31 read preambles 15/14, `📓️fault-localization-api.md` (§6), wp-s20 session 15, drift ledger (186 rows; mine: X 130, Y 50,
  126 files). p2 overlay: codemod applied (frozen-code sites → `MutationCode::*`), framework API (P2-F1) NOT yet changed
  (`MutationMessageCode` still in `🎮️mutation/🦀️.rs`; census has no pass-2 rule yet) → my sites are written against the §6 API.
- 20:4x tool `wp-fh4/fh4-p2.py` (sites from the ledger, per-site overrides, exact edits, transforms, code renames in
  tests/fixtures/Python references/TS twins/feature prose, doc-comment renames; backups of every touched file under
  `.🧬semio/🌐hub/s14-fh4-backup/`; idempotent — a re-run reports `applied`), decisions in `wp-fh4/edits/<family>.py`,
  helpers `jsargs.py` (TS call args), `rsargs.py` (Rust call args, dead text-parameter scan), `ledger.py` (S20's own
  `p2-codemod.convert` over my families, nothing written), `live-drift.py` (live vs the p2 base, sha1).
- 20:4x wfc 53 → 0 (bitmap/3d/grid3d; element-naming targets; builder no-ops/cascades as targeted `absorb_messages`;
  connect-slots no-op names the existing edge; tests, fixtures, Python references, feature prose → frozen codes).
  norm 33 → 0 (en1992 empty targets now name member/layer/action/anchor/grade; iso16757 kept). Live drift of all 126 files
  vs the p2 base: 0 (norm included; the Cursor peer's en1994/din4108 are not among my sites).
- 20:4x remodel 17 → 0 (all `Invariant`, levels kept); the TS twin drops the report text (`{severity, code, target}`,
  79 `noted`/`refuse` calls) + suite, 11 fixtures, oracle notes, Python reference, feature tables (column padding kept).
- 20:5x RELAY S20 (MutationApplyError scope) → answered: S20's codemod converted 656 apply sites; ledger re-run
  (`ledger.py 1`) left 102 in my families (X 27, stdio 75 incl. 25 `apply`).
- 20:5x raster 5 → 0: validators return raster-owned refusals (`raster-layer-not-found`, `raster.layer.*`) instead of
  `mutation.*` literals; `edit_fault` + editor declarations renamed (9 `mutation.*` app faults were an `app-mutation-code`
  breach), new `edit_report(code) -> MutationCode` for the diff. fem 4 → 0 + codemod leftovers: guard helpers lose their
  text parameters (fem2d `invariant/missing/identity_matches/referenced`, fem3d `id_mismatch/target_referenced/invariant`),
  fem3d `*_breach` → `bool`, callers updated, unused bindings removed; 30 fixtures + 11 Python references + tests renamed.
- 21:0x stdio (75 → 0): zip/svg/xml/ifc/step/pdf/png/wav/json/xlsx/docx/semio sites by meaning (profile/class/view/serialization
  rules → Invariant, absent node → TargetMissing, taken name → DuplicateId), targets name the element (svg node paths, xml
  `root`/`doctype`, ifc `instances/<id>`, step `ladder::class_edit_target` = `header/fileSchema` · `entities[/<id>]`);
  `CODE_REJECTED` consts and message-only helpers removed (`rejected`, `target_error` inlined as
  `MutationApplyError::new(<code>).at(…)`, i-json `Refusal` = the fatal outcome); one shared stdio contract report for refused
  snapshot patches (`editing::patch_refusal`, TargetMissing for `snapshot-edit.path-invalid`, else Invariant, addressed at the
  edit path the fault names); readers of the removed text (ifc test diagnostics, semio refusals, dwg/step checked-apply,
  gltf bridge) show code + target; gltf `rejection_outcome` loses its dead `detail` parameter (121 callers).
- 21:1x leftovers the ledger cannot see (compile breakers under the new API): forms + semio object `MutationApplyError {…}`
  struct literals naming `mutation.child-identity` + a message → `MutationApplyError::new(Invariant)`; semio `Rejected` diff
  codec serialized the removed message → hex of the rejection's own JSON wire form; 9 apply closures with dead params → `|_|`;
  15 laws comparing `error.code` to the old strings → `protocol::MutationCode::<variant>`; TS twins/oracle notes/generators/doc
  comments naming `mutation.apply.*` (zip TS twin + test, avi/docx oracles + generators, raster/semio/html/avi docs) → frozen codes.
- 21:2x checks: ledger re-run 0 (all 12 families); `verify faults` on the p2 overlay for my 12 owners: violations=0, oracle
  agrees (`.🧬semio/🌐hub/s14-fh4-logs/census-{1,2}.txt`); live drift of the 126 original site files vs the p2 base: 0.
- Notes for S20 (not changed by FH4): (1) the census has no `app-mutation-code` rule yet — my families carry no app raise of a
  `mutation.*` code any more (raster's 9 renamed); (2) every artifact builder still pushes `dsl::Diagnostic::error("mutation.apply",
  …, error.to_string())` (≈25 files in my families, framework-wide pattern); (3) gltf's typed rejection record still carries a
  `detail` sentence in its schema-first JSON (`GltfTopLevelMutationRejection`), now unused by reports; (4) `sourcing` (2 apply)
  and `reasoning` (1 apply) drift sites in S20's ledger are outside P2-X/P2-Y.

