# F6a Closer Report — ply / ifc4 / txt / pdf1.4 / csv / step / xlsx

**Role**: C6a closer for the op-codec fan-out sub-wave covering `☁️ply` 1.0, `🏗️ifc` 4, `📄txt` utf-8,
`📄️pdf` 1.4, `📊️csv` rfc4180, `📐️step` ap214, `📕️xlsx` ecma-376. Read all 7 fan-out reports plus the
independent verify report, applied any glue followups (none needed), ran the full crate gate and the
policy check myself, updated `STATUS.md`'s ownership ledger, and wrote this report. Did not call
`ticket_open`/`ticket_close`/`ticket_reopen`. Did not touch `📦️glue.rs` or `📜️script.ts`.

## 1. Reports read

- `f6-ply-report.md`, `f6-ifc-4-report.md`, `f6-txt-report.md`, `f6-pdf-1.4-report.md`,
  `f6-csv-report.md`, `f6-step-report.md`, `f6-xlsx-report.md` — the 7 per-artifact fan-out reports.
- `f6a-verify-report.md` — an independent verification agent's re-derivation of every one of the 7
  artifacts' classification, test counts, and a fresh whole-crate run, done from disk and real
  `cargo test` output, not from trusting the 7 agents' own self-reports.
- `f6-recon-report.md` — the authoritative spec every fan-out agent (and the pilot before them) was
  told to follow literally; read in full at session start for context (derive machinery limits §1-3,
  the binary/gif89a/svg worked examples §4-5, the schema-id convention §6 — needed no action, the
  §8 classification table — a heuristic sweep every fan-out agent was told to verify for real rather
  than trust, and the §9 step-by-step procedure).

## 2. Classification summary (all independently re-verified via real `cargo check` errors by each
   fan-out agent — never trusted from the recon's own §8 heuristic table, several rows of which were
   wrong)

| Artifact | Standard | Diff path | Mutation path | Note |
|---|---|---|---|---|
| ☁️ply | 1.0 | hand-roll | hand-roll | `PlyProperty`/`PlyValue` enums in the tree — recon guessed DERIVE, wrong |
| 🏗️ifc | 4 | hand-roll | hand-roll | `IfcValue` enum, direct + transitive — recon guessed DERIVE, wrong |
| 📄txt | utf-8 | derive | derive | matched recon's own guess exactly |
| 📄️pdf | 1.4 | derive | derive | matched recon's own guess exactly (1.4 has no `PdfValue`, unlike 1.7) |
| 📊️csv | rfc4180 | hand-roll | hand-roll | Diff: `Vec<Option<CsvFieldDiff>>` tri-state-adjacent. Mutation: a NEW `dsl_derive` hygiene bug (any field literally named `record` shadows the codegen's own accumulator) |
| 📐️step | ap214 | hand-roll | hand-roll | `StepValue` enum, direct + transitive |
| 📕️xlsx | ecma-376 | hand-roll | hand-roll | `XlsxCellValue` enum + `NamedTripleDiff<K,D,T>` (no `DslField` for the generic collection type) |

Every hand-roll followed `f6-recon-report.md` §5's shared grammar template (own local
`hex_encode`/`hex_decode`/`split_top_level`/`strip_brackets`/`encode_option`/`decode_option`
primitives, `pub(crate)` from the diff file for the mutations file to reuse, single-uppercase-letter
enum tags, `[removed];[modified];[added]` collection triples, `encode_*` = printed text bytes
verbatim). Every derive used cascading `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslScalar)]` on
nested types, then `dsl::DslDiff`/`dsl::DslOps` on the top-level type, then the standard `OpText`/
`OpBinary` boilerplate wrapper (per P6, `DslOps` never emits those itself — every mutation side ends
with a handwritten `OpText`/`OpBinary` impl regardless of whether `DslOps` derived clean).

`csv`'s mutation-side finding is worth flagging beyond this ticket: `dsl_derive`'s
`dsl_variants_codegen` unconditionally names its per-variant `RecordValue` accumulator `record` in
generated match-arm bodies, which silently shadows any variant field ALSO named `record` (confirmed
by experiment — renaming the field to `csvrec` made the same derive attempt compile). No other
`🗄️stdio` artifact currently has a `record:`-named mutation field (grepped by the csv fan-out agent),
so nothing else is affected today, but this is a real, reproducible bug in shared framework code
(`dsl_derive`'s own `🦀️component.rs`, out of every F6 agent's ownership boundary) that a future
session touching that macro should know about.

## 3. Real bug caught and fixed mid-wave (xlsx, self-corrected before this closer ran)

xlsx's own `diff_codec_text_binary_roundtrip_law` first run silently dropped a legitimate
empty-string OPC relationship-owner key (`""`, a real `zip::opc::OpcPackage` key shape) because every
`dec_*` list-splitter in both xlsx files chained a defensive `.filter(|s| !s.is_empty())` after
`split_top_level` — copied from the gif/svg pilot's idiom, where it was harmless. It's wrong here: it
also drops a genuinely-empty-string-encoded single item sitting alongside other non-empty items in
the same list (`split_top_level`'s own empty-input short-circuit already handles the "0 items" case
correctly). Fixed by removing all 12 occurrences across both xlsx files; re-ran clean. This is why
6 of the 7 fan-out reports (ply/ifc4/txt/pdf1.4/csv/step, all of which ran their own whole-crate
check at some point during this session) show a transient "1032 passed, 1 failed" — that failure was
this exact xlsx bug, caught mid-flight by whichever check happened to run before xlsx's own fix
landed, not a regression any of those 6 artifacts caused. Confirmed by this closer's own fresh
whole-crate run below, taken after the fix, showing 0 failures.

## 4. Full crate gate (this closer's own run, not delegated)

```
cargo test -p semio-s-plugin-stdio --lib
test result: ok. 1033 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.46s
```

Matches the independent `f6a-verify-report.md`'s own number exactly. Full raw output saved:
`f6a-closer-full-crate-test.txt` in this folder (its final ~80 lines are the crate's own summary; the
full test list is included for auditability).

Per-artifact filtered counts (from the independent verify report, scoped precisely to exclude
sibling standards under the same artifact directory that are out of this wave's scope — e.g.
`artifacts::ifc::standards::v4::`, not the unscoped `artifacts::ifc::` which also pulls in `2x3`):

| Artifact | Filter | Passed | Failed |
|---|---|---|---|
| ☁️ply | `artifacts::ply::` | 25 | 0 |
| 🏗️ifc | `artifacts::ifc::standards::v4::` | 19 | 0 |
| 📄txt | `artifacts::txt::` | 21 | 0 |
| 📄️pdf | `artifacts::pdf::standards::v1_4::` | 23 | 0 |
| 📊️csv | `artifacts::csv::` | 19 | 0 |
| 📐️step | `artifacts::step::` | 93 | 0 |
| 📕️xlsx | `artifacts::xlsx::` | 43 | 0 |
| **sum** | | **243** | **0** |

Every one of the 7 includes both mandatory law tests (`diff_codec_text_binary_roundtrip_law`,
`op_text_binary_roundtrip_law`) passing.

## 5. Policy check (this closer's own run, not delegated)

```
bun ./📜️script.ts policy   (full output: f6a-closer-policy-full.txt in this folder, 21612 lines)
```

`dsl-migration/diff-completeness` rule, stdio-scoped: **22 breaches remain**, down from the recon's
§7-recorded baseline of **28 remaining before this sub-wave**. Verified precisely — grepped the full
breach listing for every one of this wave's 7 artifact/standard paths specifically (not just the
summary count):

- `☁️ply/🏅️standards/🔖️1.0` — **absent** ✓
- `🏗️ifc/🏅️standards/🔖️4` — **absent** ✓ (only `🔖️2x3`, a different standard, remains)
- `📄txt/🏅️standards/🔖️utf-8` — **absent** ✓
- `📄️pdf/🏅️standards/🔖️1.4` — **absent** ✓ (only `🔖️1.7`, a different standard, remains)
- `📊️csv/🏅️standards/🔖️rfc4180` — **absent** ✓
- `📐️step/🏅️standards/🔖️ap214` — **absent** ✓
- `📕️xlsx/🏅️standards/🔖️ecma-376` — **absent** ✓

All 7 confirmed real — each one's new `DiffCodec` impl satisfies the check's own literal-text grep
(`content.includes("dsl::DslDiff") || content.includes("DiffCodec for")`, `📜️script.ts:3185-3205`).

**Reconciling 22 vs. the naively-expected 28−7=21**: the extra 1 is `🏗️ifc/2x3` — a *32nd* standard,
added by the separate, unrelated sibling ticket `ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES` after
this program's original 31-standard scope was fixed (per F5's closer's own note in `STATUS.md`),
explicitly out of scope for every wave to date including this one. It was already present in the
breach list before this sub-wave started and remains present — not a regression, not part of this
wave's 7, not this closer's or any F6 agent's responsibility. **21 of the 22 remaining breaches are
in the official 31-standard scope** (matching `28 − 7 = 21` exactly); the 22nd is `ifc/2x3`.

`POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) confirmed untouched by any of the 7
fan-out agents or this closer — still zero stdio entries. Every one of the 7 breaches this wave
targeted disappeared on its own real merits, never via allowlisting, matching the mission's stated
goal ("zero stdio entries, for real, not allowlisted around").

Remaining 22 stdio `dsl-migration/diff-completeness` breaches, for whichever wave picks up next:
`☁️las` (1.0), `🎒️zip` (2.0), `🎞️gif` (87a), `🎞️pptx` (ecma-376), `🏗️ifc` (2x3, out-of-scope 32nd),
`💬️bcf` (2.1), `📄️pdf` (1.7), `📜️docx` (ecma-376), `📝️md` (commonmark), `📰xml` (1.0), `📷️jpg`
(jfif-1.01), `📷️png` (1.2), `🔣️json` (rfc8259), `🖊️dwg` (ac1018), `🖊️dwg` (ac1024), `🖊️dxf` (r12),
`🖼️bmp` (v3), `🖼️tiff` (6.0), `🗜️deflate` (rfc1950), `🟪️stl` (ascii), `🧊️gltf` (2.0), `🧊️obj` (3.0).

## 6. glue_followup — none applied (none needed)

None of the 7 fan-out reports flagged a need for a new `glue.rs` mount. All op-codec work landed
inside `🧬️schema/{🔺️diff,🧬️mutations}/🦀️component.rs` files that were already `#[path=...]`-mounted
by the earlier F1-F5 schema wave — no new directories, no new leaves, nothing to wire in.

`📦️glue.rs` and root `📜️script.ts` were confirmed read-only for every one of the 7 fan-out agents
(each report's own "no shared files touched" section) and were not edited by this closer. `git
status` at session start showed both files with large pending diffs — investigated and confirmed
these belong entirely to the separate, concurrently-active `ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`
sibling ticket: `git diff` on `glue.rs` shows only `📐️step`'s `✳️cc1`-`✳️cc6` subset `#[path=...]`
mounts (a different, unrelated wave's work), none of this wave's 7 artifacts' op-codec files (which
correctly need no new mounts). Not touched, not this closer's concern.

## 7. `.gitignore` / untracked-directory check

No new directories were created by any of the 7 fan-out agents. The untracked paths visible under
this wave's own artifact directories (`✳️a`/`✳️x` under `pdf/1.4`, `✳️cc1`-`✳️cc6` under `step/ap214`,
`✳️strict`/`✳️transitional` under `xlsx/ecma-376`, plus one stray `🔣️component.json` scaffold file
per standard) are the same pre-existing sibling-ticket scaffold pattern every closer since F2 has
found and correctly left alone (per `STATUS.md`'s F4/F5 closer sections) — confirmed, not touched.

## 8. STATUS.md

Appended a new `## F6a (op-codec fan-out sub-wave, ply/ifc4/txt/pdf1.4/csv/step/xlsx) — closed
2026-08-11` section at the end of `STATUS.md`, following the same structure/level of detail as the
F4/F5 closer sections above it (roster, classification table, the real bug found, full-crate gate,
policy shrink with precise per-artifact verification, glue_followup, git-ignore check, ownership-
ledger update, report pointers). This is the first `## F6*` section in the file — F6 (op-codec) had
no prior STATUS.md entry (only referenced prospectively at the end of F5's own section, and the
recon/pilot work that preceded this sub-wave, tracked only in this ticket folder's own `.md` reports,
not yet folded into `STATUS.md`).

## 9. Files touched by this closer

- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/STATUS.md`
  — appended the F6a section (§8 above). No other edits to this file.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6a-closer-report.md`
  — this report.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6a-closer-full-crate-test.txt`
  — full raw output of this closer's own `cargo test -p semio-s-plugin-stdio --lib` run (1033/0).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6a-closer-policy-full.txt`
  — full raw output of this closer's own `bun ./📜️script.ts policy` run (21612 lines).

**No shared files touched**: `📦️glue.rs`, root `📜️script.ts` (incl. `POLICY_DIFF_COMPLETENESS_ALLOWLIST`),
the `dsl`/`protocol`/`schema` framework crates, and every artifact outside this wave's 7 were all
read-only for this closer session. No `ticket_open`/`ticket_close`/`ticket_reopen` calls made.

## 10. Summary (report JSON fields)

- `full_crate_passed`: 1033
- `full_crate_failed`: 0
- `diff_completeness_remaining`: 22 (stdio-scoped `dsl-migration/diff-completeness` breaches; 21 of
  these are in the official 31-standard scope — exactly `28 − 7`, matching the recon's own baseline
  math — the 22nd is `🏗️ifc/2x3`, a pre-existing, permanently-out-of-scope 32nd standard unrelated to
  this wave)
- `report_path`: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6a-closer-report.md`
