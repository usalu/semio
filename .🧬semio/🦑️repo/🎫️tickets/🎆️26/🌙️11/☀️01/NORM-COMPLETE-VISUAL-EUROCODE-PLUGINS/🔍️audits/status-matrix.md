# 📋 Status Matrix

Read-only audit of the norm effort (EN 1990 to EN 1999 plus the shared crates) against `📐️specs/visual-editor-architecture.md` section 10 and norm completeness. Nothing was built, and no source file was modified.

Sources: `🔍️audits/*-completion.md`, `vis-*.md`, `wp*.md`, `📌️important/{requests.md,mounts.md,build-loop.md}`, `🗑️generated/**` logs and `build-en*/latest.md`, and grep/find checks in `✏️s/🔌️plugins/📕️norm`.

Method notes:
- `integration-surface.md` does not exist in `🔍️audits/`. It was not used.
- No completion report states a percentage of the norm catalogue implemented. The column shows the counts the reports give, or `n/r` (not reported).
- "SetFieldRule mentions" are grep counts in each artifact's `🧬️mutations/🧭️edit-rules` and schema trees. They are approximate, not leaf counts.
- `#[test]` counts are grep counts in the artifact trees. None of those tests has been run.

## 1. Build and test evidence (whole `🗑️generated` tree)

- Log files with an `exit 0` line: 0. Every log that has an exit line ends in `exit 1` or `exit 130`.
- Log lines matching `error[`: 0. No rustc diagnostic has ever been produced, so no code has reached the compiler.
- Failure causes seen in the logs:
  - `Unknown project in the closure: @semio-s/plugin-norm-structural-scene` (wp01, integrate/scene1, wp08, wp11). The same message names `@semio-s/plugin-norm-app-surface` (wp02/surface-check) and `@semio-tech/norm-compliance-rs` (wp05/test2).
  - `build:graph-plan was killed by SIGKILL` (en1996/check1, en1998/check, en1999/check1).
  - `Cargo dependency lacks authored member authority: ✏️s/🧪️tests/🧪️mutation-vectors/📦️packages/🦀️rust/Cargo.toml` (en1997/check0).
  - `Unsupported build option: --language` (en1995/test2). This is a wrong flag, not a code error.
  - `Bun installation owner is a symlink: CLAUDE.md` (en1990/check0). This is the infrastructure issue listed in `build-loop.md`, and the note says a re-run fixes it.
  - Most other runs sat in the capacity queue ("waiting … memory pressure"). Several were cancelled by SIGTERM.
- Shared-crate projects: the Cargo manifests of `🎬️structural-scene/📦️packages/🦀️rust`, `⚖️compliance/📦️packages/🦀️rust` and the app-surface crate exist, but no `📋️project.json` naming them exists in the tree. This matches the "Unknown project in the closure" errors.

## 2. Status matrix

Key: `Y` yes, `P` partial, `N` no, `?` unverified (written, never compiled), `n/r` not reported.

| Row | Norm catalogue implemented (from report) | Defects fixed (listed in report) | Side-catalogue vs snapshot-integrated | Edit coverage (SetFieldRule mentions) | Scene 3D written | Diagrams written | Windows replaced (editor / viewer) | Live re-eval wired | NDP registry adopted | Oracle third-party / independent | Last build result | Tests green | Open requests (keyword match) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EN 1990 | 30 of 30 registry ids E90.01–E90.30 (`evaluate_case`); no % | ~27 listed | Mixed: snapshot checks plus side registry (not in a window) | 14 | ? (`🎬️scene`, 9 .rs) | ? (matrix, psi, reliability) | Y / Y | N | P (10 kernel ids, 18 calls, all in registry) | Y (scipy, `-I`) | check0 exit 1 (Bun symlink, pre-compile); check1 queued | N (0 runs) | 12 |
| EN 1991 | 96 registry entries; no % | 16 rows | Mixed (document checks share the `part_*` code with catalogue) | 71 | ? (14 .rs) | ? | Y / Y | N | N (2 private UNVERIFIED lists) | Y (scipy) | check1 queued, no exit | N | 6 |
| EN 1992 | n/r: no completion report, only `en1992-inventory.md` and `vis-en1992.md` | n/r (inventory only) | Snapshot only (no catalogue report) | 24 | ? (11 .rs) | ? | Y / Y | N (full re-project per edit) | P (63 `.ndp()` calls with local keys, none in kernel: all would flip to UnverifiedBasis) | N (stdlib only; no completion audit) | check0 queued, no exit | N | 5 |
| EN 1993 | n/r; "not norm complete" (self-stated); clause library with 17 tests, engine wiring partial | 22 numbered | Mixed: E-marked clauses wired, many groups functions only | 2 | ? (10 .rs) | ? | Y / Y | N | N | Y (numpy, scipy; sectionproperties absent) | check.txt still queued/running at last record, no exit | N | 6 |
| EN 1994 | 52 of 54 C-items mapped (C-48, C-54 open) | ~19 numbered plus presets | Snapshot-integrated (checks), formula module delegated | 20 (report: 19 leaves) | ? (10 .rs), golden absent | ? | Y / Y | N | N | P (check oracle stdlib; scene oracle numpy/shapely) | check0 queued, no exit | N | 6 |
| EN 1995 | n/r; registry T-01..T-124 via `evaluate_case`; snapshot has no catalogue entities | 14 numbered | Side-catalogue (not in snapshot or windows) | 59 | ? (9 .rs) | ? | Y / Y; legacy `📥️inputs`, `📊️results`, `📊️report` dirs still on disk (unmounted) | N | N | N (stdlib only) | check.txt queued, no exit; test2 exit 1 (`--language` misuse) | N | 7 |
| EN 1996 | n/r; M-01..M-95 registry; 11 ids also built from the wall model (document cases) | ~10 groups | Mixed: 11 ids integrated in document, rest side | 51 | ? (11 .rs) | ? | Y / Y | N | P (2 private `UNVERIFIED_NDP` uses; kernel not used) | Y (scipy, shapely) | check1 exit 1 (SIGKILL); check2 queued | N | 10 |
| EN 1997 | n/r; 62 registry entries; snapshot has no anchors, walls-embedded, pile groups, lab tests | 16 numbered | Side-catalogue for new kinds; snapshot-integrated for footings, walls, slopes | 15 (report: 14 of ~69 leaves) | ? (14 .rs) | ? | Y / Y (phase 2 mounted; `vis-en1997.md` still says not done) | N (only an incremental test name) | N (kernel cited none) | P (main oracle stdlib; scene oracle numpy/scipy/shapely) | check0 exit 1 (manifest authority); test1 SIGTERM | N | 8 |
| EN 1998 | n/r; PARTIAL (self-stated); pure formulas plus snapshot checks | ~12 listed | Mixed (pure functions, several not yet catalogue entries) | 94 (report: 81 new leaves + 12, all non-id scalar leaves) | ? (10 .rs) | ? | Y / Y | N | P (private `ndp::REGISTRY`, kernel used once) | P (main oracle stdlib; scene oracle numpy) | check exit 1 (SIGKILL); check2 queued | N | 6 |
| EN 1999 | A-01..A-81 spec; ~12 ids with stated schema/table gaps | ~40 numbered | Snapshot-integrated (checks) plus pure functions | 17 | ? (11 .rs) | ? | Y / Y | N | P (private `na_de::NDP_REGISTRY`; 3 kernel-form refs in tests only) | N (stdlib only) | check1 exit 1 (SIGKILL); check2 queued | N | 6 |
| structural-scene (shared) | n/r; wp01, wp03/04, wp07, wp08 written | n/a | n/a | n/a | Y (crate written) | Y (diagram-kit, written) | n/a | n/a | n/a | n/a | Unknown project in closure (wp01, integrate/scene1, wp08, wp11); no `📋️project.json` | N | 13 (keyword match on A1/A3/A4/A8) |
| app-surface (shared) | n/a | `render_document_editor` removed (0 refs) | n/a | n/a | n/a | n/a | Y (checks, properties, viewport, diagrams windows) | P (`WP-05` not reached) | n/a | n/a | Unknown project in closure (wp02/surface-check, 4.8 MB log) | N | 19 (keyword match on A2/A6/A9/A10/A12) |
| compliance (shared) | n/a | NDP registry added; global report cache and `document_revision_key` still present | n/a | n/a | n/a | n/a | n/a | P (`IncrementalNormFamily` written, not wired in artifacts) | Kernel: 520 entries, all `unverified` | n/a | Unknown project in closure (wp05/test2) | N | 3 |

Notes on the table:
- Open requests are keyword counts from `requests.md` (82 `[open]`, 6 `[done]` overall). Some lines count for two artifacts, and 15 open lines name no artifact.
- The `?` in the scene and diagrams columns means the crate or module exists in the tree but has never compiled. The EN 1994 golden file is absent, the EN 1990, 1992, 1993, 1996 and 1999 golden files are empty (0 bytes).
- The "Y" for windows means the editor and viewer mode layouts mount `🧊️viewport`, `📈️diagrams`, `✅️checks` (and `🧾️properties` in the editor). Legacy inputs/results/report are unmounted in all ten. The EN 1995 directories remain on disk.

## 3. Definition of done (section 10 of the architecture spec)

| DoD item | Status | Evidence |
|---|---|---|
| 1. Opening an artifact shows outliner, 3D, typed properties, checks with bars and clause links, diagrams; no JSON text | ? | Windows are mounted in all ten. `render_document_json` has 0 references. Checks-bar and clause-link rendering not verified (no build). |
| 2. Live drag preview <12 ms p95; commit as one undoable event; checks within 400 ms p95 | N | No latency measurement exists. Incremental evaluation is not wired (0 of 10 artifacts). The editor rebuilds the host on every render (`vis-en1992.md`). |
| 3. Every fixture projects to a golden scene validated by independent oracles; incremental equals full | N | Goldens absent or empty in all ten. Oracles are partial (section 4). Incremental equivalence has no test that has run. |
| 4. EN and DE complete; keyboard and screen-reader paths; CVD-safe palettes | ? | `wp11-a11y-themes.md` written, never compiled or run. EN and DE texts exist in field-spec rows. Not verified. |
| 5. Desktop, mobile and tablet layouts from the same windows | ? | `DeviceClass::profile()` written in wp07. Layouts are fixed fractions (22/50/28). Not verified. |
| 6. Viewer shows the same scene read-only; no editor import in the viewer tree | Y (source) | Grep finds no `use` of editor modules in any `👁️viewer` tree. Only comments mention `✏️editor`. "SDK GAP" comments remain in all ten viewers and editors. |
| 7. Obsolete code removed (`render_document_editor`, `NormFieldMeta`, global report cache, `document_revision_key`) | P | `render_document_editor`: 0 references (Y). `NormFieldMeta`: stale test still uses it in `app-surface/🪟️visual-windows/🧾️properties/🧪️tests/🔬️unit/🦀️.rs` (lines 19 and 21), so requests that say it is gone are wrong; `NormFieldMetaFn` also remains. Global report cache (`report_cache` in `⚖️compliance/🏛️norm-error-and-family`) still present. `document_revision_key` still defined and used in app-surface and retained-commands. EN 1995 legacy window dirs still on disk. |

## 4. Oracle independence (per the third-party rule)

| Artifact | Third-party library in an oracle | Independent formulation only |
|---|---|---|
| EN 1990 | scipy | yes |
| EN 1991 | scipy | yes |
| EN 1992 | none (no completion report; three .py files, all stdlib) | n/a |
| EN 1993 | numpy, scipy (clauses oracle; sectionproperties absent) | yes |
| EN 1994 | numpy, shapely (scene oracle only); compliance oracle stdlib | partial |
| EN 1995 | none (stdlib) | yes, but no third-party |
| EN 1996 | scipy, shapely, numpy | yes |
| EN 1997 | numpy, scipy, shapely (scene oracle); main oracle stdlib, scipy optional | partial |
| EN 1998 | numpy (scene oracle); main oracle stdlib | partial |
| EN 1999 | none (stdlib) | yes, but no third-party |

## 5. Prioritized blockers

1. **No build can complete for any artifact or shared crate.** Three shared crates (structural-scene, compliance, app-surface) have no `📋️project.json`, so the repo closure rejects them ("Unknown project in the closure"). This blocks every artifact that depends on them. Fix the project registration first, then re-run.
2. **Zero compile and test results.** No `error[` and no `exit 0` in the whole tree. All "fixed defects" and all test counts (about 1,000 `#[test]` lines across ten artifacts) are unverified. Most likely failures are at compile of the new scene crates and of the snapshot twins.
3. **Build infrastructure.** `build:graph-plan` was SIGKILLed three times (en1996, en1998, en1999), and the capacity queue held the rest for 1 to 3 hours. Serialise builds through one owner, and fix the wrong `--language` flag (en1995) and the `mutation-vectors` manifest authority error (en1997) before the next run.
4. **Stale and obsolete code (DoD 7).** Remove the global report cache and `document_revision_key` from compliance and app-surface. Remove the stale `NormFieldMeta` test and the `NormFieldMetaFn` name. Delete the EN 1995 legacy inputs, results and report directories. Remove the "SDK GAP" comments.
5. **NDP registry adoption.** The kernel registry holds 520 entries, all unverified, so the gate turns every citing check into `UnverifiedBasis`. Only EN 1990 cites kernel ids. EN 1992 cites 63 local keys that are not in the registry, so its checks will all flip to `UnverifiedBasis` once the gate is live. EN 1991, 1993 to 1997 and 1999 keep private lists or nothing. Each artifact needs its NDP rows registered and cited by registry id.
6. **Placement fields missing in all ten artifacts.** Persisted placements, which the scene layout depends on, are not in the snapshot or twins. The requests for `Placement` (EN 1990, 1993, 1994, 1995, 1996, 1997, 1998, 1999) are still open.
7. **Catalogue reachability.** Several artifacts keep their catalogue outside the snapshot and outside any window (EN 1995, EN 1997, EN 1990, EN 1996 in part). Their results do not appear in the editor, so DoD 1 does not hold for them.
8. **Live re-evaluation (DoD 2).** The WP-05 incremental planner exists in compliance but no artifact uses it. Latency has not been measured.
9. **Golden scenes (DoD 3).** Goldens are absent or empty. They need one green test run each (`SEMIO_UPDATE_GOLDEN=1`).
10. **Oracle rule.** EN 1992, 1994 (compliance oracle), 1995, 1997 (main oracle), 1998 (main oracle) and 1999 do not validate their output with a third-party library. EN 1992 also has no completion report.
11. **Edit coverage.** EN 1997 has about 14 of ~69 editable leaves with a rule (report: 55 missing). EN 1992 lacks handles for prestress, punching and anchors. EN 1993 has 2 rules. EN 1998 is the only artifact at full non-id scalar coverage.
12. **Open request backlog.** 82 open requests against 6 done (keyword counts); `mounts.md` has 7 done and 0 open.
13. **Stale audit.** `vis-en1997.md` says phase 2 is not done, but the windows are mounted. Reconcile before relying on it.
