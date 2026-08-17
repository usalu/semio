# F4 Closer Report (C4) — gltf / pdf / step / ifc / docx

Role: C4 closer for the F4 fan-out wave (gltf 2.0, pdf 1.4+1.7, step ap214, ifc 4, docx
ecma-376 — 5 artifacts, 6 standards). Only agent this wave allowed to touch `📦️glue.rs` and
`📜️script.ts`.

## 1. Inputs read

`f4-gltf-report.md`, `f4-pdf-report.md`, `f4-step-report.md`, `f4-ifc-report.md`,
`f4-docx-report.md` (the 5 fan-out reports) and `f4-verify-report.md` (independent verifier —
re-derived every claim from disk/`cargo test`, not from the self-reports). Cross-checked against
`s2-spine-report.md`'s ownership-boundary resolution (fan-out agents need zero `glue.rs` edits;
real work lives inside already-mounted `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}` +
`⚙️engine`/`🏗️builder`/`🧐️analyzer` leaves) and `w0-recon-report.md` §7 for the defects originally
flagged against each of these 5 artifacts.

## 2. `glue_followup` — none required

All 5 fan-out reports and the verify report independently state no `glue.rs`/`script.ts` edits
were needed: every rewrite landed inside files already mounted by prior waves (S1/S2). Confirmed
directly — `git diff --stat` on
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` shows a pre-existing 621-insertion diff from
earlier waves (S1/S2/F1/F2/F3), untouched further by this wave. **`glue_edits: []`.**

docx's report flags one deferred (not urgent) design note: `DocxOpcDiff` and its 4 sibling diff
types currently live inside docx's own `🔺️diff/🦀️component.rs` rather than
`zip/📦️opc/🦀️component.rs`, because docx's own ownership boundary didn't extend to the zip
plugin's file. This is future consolidation work for whenever xlsx/pptx/bcf need the same OPC
diff shape — not a defect, not actioned this wave (the types are written generically enough to
lift verbatim later).

## 3. Full-crate gate

`cargo test -p semio-s-plugin-stdio --lib` (no filter, run fresh by this closer):

```
test result: ok. 965 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.61s
```

Per-artifact filters (also re-run fresh, not trusted from the reports):

| artifact | filter | result |
|---|---|---|
| gltf | `artifacts::gltf` | 35 passed, 0 failed |
| pdf (1.4+1.7) | `artifacts::pdf` | 131 passed, 0 failed |
| step | `artifacts::step` | 91 passed, 0 failed |
| ifc | `artifacts::ifc` | 62 passed, 0 failed |
| docx | `artifacts::docx` | 45 passed, 0 failed |

Sum (364) plus the pre-F4 baseline accounts for the 965 whole-crate total; matches the verify
report's own number exactly. `**full_crate_passed: 965, full_crate_failed: 0**`. (Both gltf's and
ifc's own self-reports had caught transient failures mid-session from concurrent sibling F4
agents' in-flight work on pdf/step/docx — none of that persisted; this closer's own run, after
all 5 fan-out agents finished, is clean.)

## 4. Policy shrink — `bun ./📜️script.ts policy`, scoped to the 4 S-8 rules for these 5 artifacts

Ran `bun run ./📜️script.ts policy` (full repo run; regenerates
`.🦑️repo/⚡️cache/breaches/compose.json`) and queried the regenerated cache **directly** (not the
CLI's low-priority-filtered stdout, which prints none of these low-priority rules by default —
same discipline F1/F3b's closers used).

**Before pruning**, filtered to the 4 S-8 rules
(`stdio-artifacts/diff-algebra`, `stdio-artifacts/field-sweep-presence`,
`stdio-artifacts/grammar-honesty`, `stdio-artifacts/facet-mirror-drift`) and these 5 artifacts:
**60 breaches, every one `-stale-` (satisfied but still allowlisted), zero real (missing)**:

- `POLICY_DIFF_ALGEBRA_ALLOWLIST`: 6 stale (gltf, ifc/4, pdf/1.4, pdf/1.7, step, docx — every
  standard's diff type now really implements `DiffAlgebra`).
- `POLICY_FIELD_SWEEP_ALLOWLIST`: 6 stale (same 6 — every standard now has a real
  `field_sweep`-named passing test).
- `POLICY_GRAMMAR_HONESTY_ALLOWLIST`: 48 stale — ifc/4 (6, snapshot facet only — diff/mutations
  grammar leaves are a documented, deliberately-deferred gap per ifc's own report, correctly
  **not** stale, still real placeholders), step/ap214 (6, one `.protocol.semio` +
  `.grammar.semio` pair per facet × 3 facets — the rest of each facet's 5 remaining leaf kinds are
  a documented, deliberately-deferred gap matching zip's own established precedent, correctly
  left allowlisted), pdf/1.7 (17 — pdf/1.4's grammar leaves were explicitly left as placeholder
  per the brief's "main target" triage, correctly **zero** stale entries for 1.4), gltf/2.0 (19 —
  every facet × every leaf kind, matching gltf's own "all grammar leaves real" claim). docx: 0
  stale — its own report explicitly left grammar leaves untouched this wave (deviation,
  documented, tracked by the existing allowlist — correctly still allowlisted, not stale).
- `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST`: **0 stale, 0 missing, for all 5 artifacts.** The
  heuristic (every camelCased Rust field name must appear as a substring in each sibling
  `.ts`/`.graphql`/`.json`/`.proto` leaf) did not confirm zero-drift for gltf/pdf/step/ifc despite
  their own reports' claims of complete field-for-field mirrors — left **fully untouched**,
  same "don't trust the self-report over the live checker" discipline F3b's closer used on tiff's
  grammar-honesty claim. docx's own report explicitly did not touch facet mirrors this wave, so
  correctly not stale either.

**Pruned exactly the 60 confirmed-stale entries** (6 diff-algebra + 6 field-sweep + 48
grammar-honesty; 0 facet-mirror-drift) from the 3 allowlists in `📜️script.ts`, verifying via a
`count()==1`-guarded exact-line match before each removal (one accidental double-match caught and
handled: the diff-algebra and facet-mirror-drift allowlists share identical literal key strings
for gltf/pdf/step/ifc/docx, e.g. `"stdio/ifc/standards#4-subsets-any-schema-diff-component"`
appears verbatim in both arrays — scoped the removal to each array's own `[`...`]);` span so only
the diff-algebra copy was removed, the facet-mirror-drift copy correctly left in place).

**After pruning**, re-ran `bun run ./📜️script.ts policy` and re-checked the freshly regenerated
breach cache: **0 breaches, real or stale, for all 4 S-8 rules across gltf/pdf/step/ifc/docx**
(excluding ifc's untouched `2x3` sibling standard — see §6). `cargo test -p semio-s-plugin-stdio
--lib` re-run clean after the `📜️script.ts` edits: **965 passed, 0 failed** (unchanged — expected,
TypeScript-tooling-only allowlist edits with zero Rust surface). **`policy_shrink_confirmed:
true`.**

### Leftover stale entries from prior waves (per the brief's explicit sweep instruction)

Grepped every prior wave's own fan-out `f*-report.md` (excluding closers/verifiers) for
"should be pruned"/"stale" — one hit: `f3b-tiff-report.md` flagged its
`POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` tiff entries as stale. Checked `f3b-closer-report.md`:
already investigated and **deliberately left in place** (verified independently that real drift
remained despite tiff's own agent's claim) — not an oversight, no action needed. Cross-checked
this by querying the now-regenerated breach cache for `-stale-` entries across all 4 S-8 rules
**repo-wide** (not just F4's 5 artifacts): **zero**, confirming every prior wave's closer (F1,
F2, F3, F3b) left its own allowlists fully shrunk, and this wave's pruning above is likewise
complete.

## 5. `git check-ignore -v` on new directories

None of the 5 fan-out reports created new directories (each explicitly confirms this; the verify
report corroborates via `git status`). The only untracked paths under
`🗿️artifacts/{🧊️gltf,📄️pdf,📐️step,🏗️ifc,📜️docx}/**` are pre-existing deliverables from the
separate, now-closed sibling ticket `26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`
(pdf's `✳️a`/`✳️e`/`✳️h`/`✳️ua`/`✳️vt`/`✳️x`, step's `✳️cc1`–`✳️cc6` + `⚙️engine/🪜️ladder`, docx's
`✳️strict`/`✳️transitional`, ifc's `2x3` standard, and one `🔣️component.json` subset-registry file
per artifact) — none created or modified by any F4 agent. Ran `git check-ignore -v` on all of
them: every one matches only the `.gitignore` line-179 **negation** rule (`!**/🔖️*/**`), i.e.
explicitly un-ignored/trackable, not actually gitignored (`git status --porcelain` independently
confirms all show as plain `??` untracked, not silently absent). No `.gitignore` action needed.

## 6. Residual defect flagged, not this wave's scope

The verify report's own finding, corroborated directly: **ifc's `2x3` standard still has the
original W0-flagged defect** — its `IfcSnapshot`/diff/mutation structs directly
store/import `step::engine::part21::Part21Document`/`Part21Header`/`Part21Instance`/
`Part21Value` as their own persisted types (`🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/
📸️snapshot/🦀️component.rs:13` etc.), the exact copy-paste-type risk pattern that ifc's `4`
standard fixed this wave. F4's mandate (per every fan-out brief) scoped ifc to standard `4` only;
`2x3` was never assigned to any F4 agent. Confirmed via the regenerated breach cache: `2x3`'s
grammar-honesty entries fire as genuine (non-stale) breaches, not allowlisted, at "low" priority
— pre-existing state, untouched by F4, not a regression. Flagging for the orchestrator to decide
whether `2x3` needs its own future wave.

## 7. STATUS.md

Appended a new `## F4 (fan-out wave, gltf/pdf/step/ifc/docx) — closed` ownership-ledger section
(below), matching the style of every prior wave's closer entry.

## Structured output

```
full_crate_passed: 965
full_crate_failed: 0
glue_edits: []
policy_shrink_confirmed: true
report_path: .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f4-closer-report.md
```
