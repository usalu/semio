# Norm History Audit — Cross-Cutting (Wave A)

## Executive summary

Norm work spans **four architectural eras** in ~10 weeks: (1) headless `s/plugin/norm/*/rs/lib.rs` family crates with inline `pub mod part_*` physics (July, peak `514809c9b4`), (2) constitutional 7-crate apps + `norm-plugin` WASM (July 30 `3076aaaa02`), (3) single-crate artifact plugin `✏️s/🔌️plugins/📕️norm` with schema facets and deleted engines (Aug), (4) 15 leaf artifact packages + compliance-assessment ticket (Sep). **July tickets claimed feature-complete** (`218/218` tests, per-part checklist all `[x]`) but the **current ticket's baseline** already flags scalar subjects, hardcoded β, no remediation, and thin reports. Git comparison shows **Eurocode/DIN part math was largely ported** into `🧬️schema` + `💡️inferences/evaluate()` — not bulk-deleted — but **evaluate wiring, subject richness, report model, and catalogue families** regressed or stalled. Best recovery reference: **`git show 514809c9b4:s/plugin/norm/<family>/rs/lib.rs`** (monolithic, pre-facet). Highest-risk open collision: this ticket's Wave B–D vs seven still-open July norm tickets and `STUBS-AND-PLACEHOLDERS-COMPLETION`.

---

## 1. Timeline

| When | Ticket / commit | What happened | Claimed outcome |
|------|-----------------|---------------|-----------------|
| **Jun–Jul 18** | Early `s/plugin/norm` commits | `norm_core` + 15 family crates (`din/4108`, `din/en/16798`, `din/v/18599`, `en/1990`–`1999`, `iso/16757`, `vdi/3805`) | Shared `CheckResult`/`NormHost` kernel |
| **Jul 18–19** | `NORM-TECHNOLOGY-ABSOLUTELY-FEATURE-COMPLETE` (closed) | Round-2 remediation; `part-checklist.md` all parts `[x]`; AnnexParams DE/EN divergence | **218/218** cargo tests; every part wired in `evaluate()` |
| **Jul 19** | `NORM-PLUGIN-AND-PER-FAMILY-APPS` (closed) | 13 WASM `DocumentApp`s, `NormHost` session layer | Headless + plugin parity |
| **Jul 19** | `NORM-EN-1990-1991-FEATURE-COMPLETENESS` (closed) | Persistent/accidental/seismic combos, `check_full_actions` | 16 unit tests pass |
| **Jul 19** | `NORM-EN-1993-AND-EN-1998`, `NORM-EN-1994-1999`, `NORM-ENERGY-FAMILY-COMPLETENESS-GAPS` | Per-family gap tickets | **Still open** — never closed |
| **Jul 26** | `CONVERT-EN-1997-NORM-CRATE-TO-DECLARATIVE-DSL-ENGINE` | DSL-engine conversion planned | **Still open** |
| **Jul 29** | `514809c9b4` | Peak headless crate state — richest `lib.rs` per family | See recovery table §4 |
| **Jul 30** | `3076aaaa02` | **Deletes** flat `s/plugin/norm/*/rs/lib.rs`; creates constitutional `s/plugin/norm/app/<family>/{rs,engine,dsl,op,pack,protocol,ui}` | Migration to app tree |
| **Aug 5** | `NORM-PLUGIN-SHAPE-V2-TREE-PURITY-RETROFIT` (closed) | Consolidates 107 crates → `semio-s-plugin-norm`; path-string retrofit | **894/894** tests, zero drift claimed |
| **Aug 8–12** | `ARTIFACT-SCHEMA-FACETS` | Wave 5 norm glue: 15 artifacts, `📸️snapshot`/`🔺️diff`/`🧬️mutations` facets | `🧪wave5-norm-*` generators |
| **Aug 12** | `ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` (closed) | Dissolves artifact `⚙️engine` dirs; relocates `evaluate` → `💡️inferences` | Norm regressions fixed in-session |
| **Aug 12** | `SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL` | Wave 2 + Wave M: 392 semantic mutation triads; bans `SetSnapshot` in mutation vocabulary | `📓️waveM-reports/norm-*` |
| **Aug 20** | `COMPOSE-TO-PUZZLE5D-MIGRATION` | `🔧️norm-fixture-authoring` workstream | **Open** |
| **Aug 23** | `END-TO-END-TESTING-REFACTOR` | `w12-norm`, `w15-audit/report-norm` oracle sweeps | Closed parent; norm evidence retained |
| **Aug 29** | `STUBS-AND-PLACEHOLDERS-COMPLETION` | Repo-wide stub census | **Open** — includes norm |
| **Sep 8** | `COMPOSABLE-STDIO-ARTIFACT-PACKAGES` | 15 leaf crates `semio-s-artifact-norm-<id>`; contract package | `📓️norm-artifact-package-extraction.md` |
| **Sep 8** | `CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS` | Norm document contract, results-window ownership, `q_k` composed-child identity | `norm-*-ownership.md` audits |
| **Sep 23** | `END-TO-END-OS-HUB-COLLABORATION-MCP` | Norm descriptor links, app-surface gates (`wp-p3`) | **Open** (hub E2E) |
| **Sep 26** | `NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS` | Wave A audits + planned B–D rebuild | **Open** (this fleet) |

### Architectural path (ASCII)

```
s/plugin/norm/{core,din,en,iso,vdi}/*/rs/lib.rs     [Jul peak 514809c9b4]
        │ delete 3076aaaa02
        ▼
s/plugin/norm/app/<family>/{7 constitutional crates}  [Jul 30]
        │ consolidate Aug 5
        ▼
✏️s/🔌️plugins/📕️norm/  (semio-s-plugin-norm, 15 artifacts + 15 apps)
        │ engine dissolve Aug 12; evaluate → 💡️inferences
        │ semantic mutations Aug 12; 392 triads
        │ package extract Sep 8 → semio-s-artifact-norm-*
        ▼
Current: artifact = 🧬️schema + 💡️inferences/evaluate + ✏️editor + 👁️viewer
```

---

## 2. Claimed done vs what exists now

### 2.1 July "feature complete" gate (`part-checklist.md`, ticket closed)

Gate required per part: real formulas, clause tables, DE-NA divergence, numeric worked-example test, `evaluate()` coverage. Checklist marks **all 15 families `[x]`** including EN 1993 (16 parts), DIN 4108 (parts 1–10 + Beiblatt 2), DIN V 18599 (parts 1–12 in `balance_annual`).

**Verified against current tree:**

| Area | July claim | Current reality (Sep 26) |
|------|-----------|------------------------|
| Part modules (`pub mod part_*`) | All wired in `evaluate`/`check_full_*` | **Present** in `🧬️schema/🦀️.rs` for most families (e.g. EN 1993: 16 mods, 28 `check_*`; DIN 4108: 9 mods) |
| `evaluate()` | Full part coverage | **Thin dispatcher** in `💡️inferences/🦀️.rs` (108–247 lines/family) delegating to `check_full_*` — wiring exists for structural families |
| EN 1990 combinations | Persistent + accidental + seismic | `evaluate()` calls persistent + accidental + seismic check + **hardcoded** `check_reliability_index(3.9, …)` — **omits `DesignSituation::Transient`** despite helpers supporting it (`🧬️schema/🦀️.rs:411,621-623`) |
| Eq. 6.10 | Real max(6.10a, 6.10b) | Comment admits **"surrogate as 6.10a"** (`🧬️schema/🦀️.rs:424`) — same in old `lib.rs` |
| Subject model | `Document` with domain fields | **Scalar-heavy snapshots** — EN 1990: 6 scalars + composed `q_k` table; not a full building/structure subject |
| `CheckResult` | Clause-level compliance | `⚖️compliance/🦀️.rs`: `{clause, status, computed, limit, utilization, message: String, annex}` — **no remediation, no localization, no subject-element ref** |
| FEM (EN 1992/1993) | Wired in evaluate path | `check_rc_beam_from_fem` behind **`#[cfg(feature = "cross-fem")]`**; optional leaf dep on `semio-s-artifact-fem-2d` |
| Tests | 218/218 numeric | Plugin reports **894/894** (Aug 5); per-family compliance tests exist but many assert presence/sections not full numeric derivation |
| `AGENTS.md` | Documents norm layout | **Stale** — still links `norm/core/rs/lib.rs`, `norm/din/`, dead `s/plugin/norm` paths |

### 2.2 Line-count signal (schema+inferences vs old `lib.rs` at `514809c9b4`)

Eurocode/DIN families: **net growth** in schema+inferences (math ported + facet overhead). Catalogue families looked smaller in schema+inferences alone but **total `.rs` per artifact grew 6–9×** from mutations/editor/viewer facets.

| Family | Old `lib.rs` | Current schema+infer | Δ |
|--------|-------------|---------------------|---|
| EN 1993 | 1299 | 1445 | +146 |
| DIN 16798 | 1313 | 1411 | +98 |
| EN 1998 | 1043 | 1243 | +200 |
| EN 1990 | 695 | 756 | +61 |
| ISO 16757 | 2493 | 914 (schema+infer only) | −1579 * |
| VDI 3805 | 3211 | 886 (schema+infer only) | −2325 * |

\* ISO/VDI **total artifact `.rs`** is now ~10k lines each (mutations dominate); evaluate logic for ISO 16757 is ~199 infer lines with `part_1`/`part_2` calls — less than old monolith but not zero.

### 2.3 August restructuring — what actually moved

| Deleted (git) | Relocated to | Notes |
|---------------|--------------|-------|
| `s/plugin/norm/*/rs/lib.rs` (3076aaaa02) | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/*` | Physics → `🧬️schema`; evaluate → `💡️inferences` per ENGINELESS ticket |
| Artifact `⚙️engine/` dirs (Aug 12) | `💡️inferences/🦀️.rs` | Comment: "relocated verbatim from deleted `⚙️engine`" |
| `SetSnapshot` mutation variants | Per-field `change-*` + `from_snapshot()` | Wave M: 392 triads; `evaluate` app command → `Emit::default()` |
| `norm_core` crate dep | `⚖️compliance/🦀️.rs` in plugin | Types inlined into plugin shared module |
| Plugin config/presence top-level schemas | Removed (`8add1df147`) | Families own schemas under artifact facets |

---

## 3. Lost / degraded functionality & recovery pointers

### 3.1 Not lost (do not re-implement from git)

- Inline `pub mod part_*` check functions for Eurocodes, DIN 4108/16798/18599 — **already in current `🧬️schema`**
- `check_full_steel_member`, `check_full_envelope`, `balance_annual` orchestrators — **in `💡️inferences`**
- AnnexParams DE/EN branching pattern — **preserved** in schema helpers
- Semantic mutation vocabulary — **392 triads** (Wave M); do not reintroduce `SetSnapshot` in `🧬️mutations/`

### 3.2 Genuinely degraded or never finished

| Gap | Evidence | Impact |
|-----|----------|--------|
| Complete subject (building/element/structure) | EN 1990: 6 scalars; coordination baseline | Artifact is not "the complete subject under evaluation" |
| Remediation in reports | `CheckResult` has `message: String` only | No "increase layer X to ≥124 mm" |
| Localization en+de | Messages are English strings | DE ξ table exists but report not localized |
| EN 1990 Transient situation | Helper supports it; `evaluate()` skips it | Missing design-situation coverage |
| β from consequence class | `check_reliability_index(3.9, …)` hardcoded | Check ignores subject |
| Eq. 6.10 true max | Surrogate comment at `🦀️.rs:424` | Systematic under/over-check |
| FEM integration | `cross-fem` feature gate | Default build skips FEM path |
| ISO/VDI catalogue depth | Old monoliths 2493/3211 lines; evaluate thinner | Product-data validation simplified |
| Representation parity | Sep 8 audit: JSON/GraphQL declare fields absent from native Rust | Wire-format drift |

### 3.3 Best recovery candidates (`git show <rev>:<path>`)

Peak headless reference commit: **`514809c9b4`** (2026-07-29). Constitutional-app snapshot: **`3076aaaa02`** (2026-07-30, same logic in `app/<family>/engine/rs/lib.rs` paths). No separate `part_*.rs` files ever existed — parts are **inline modules in `lib.rs`**.

| Family | Rev | Path | Lines | Recover for |
|--------|-----|------|-------|-------------|
| **VDI 3805** | `514809c9b4` | `s/plugin/norm/vdi/3805/rs/lib.rs` | 3211 | Sheet macro catalogue, `evaluate_bbox`, full Part 1 + sheets 2–100 checks, `parse_native_text` |
| **ISO 16757** | `514809c9b4` | `s/plugin/norm/iso/16757/rs/lib.rs` | 2493 | `part_1` selection/BIM embedding, `part_2` bbox/STEP, `part_4`/`part_5`, `io` module |
| **EN 1993** | `514809c9b4` | `s/plugin/norm/en/1993/rs/lib.rs` | 1299 | 16 `part_*` modules, `check_full_steel_member` wiring, numeric tests at file bottom |
| **EN 1998** | `514809c9b4` | `s/plugin/norm/en/1998/rs/lib.rs` | 1043 | DE zone vs EN Type 1/2 spectrum divergence, parts 3–6 reshuffle |
| **DIN 16798** | `514809c9b4` | `s/plugin/norm/din/en/16798/rs/lib.rs` | 1313 | 9 normative parts, PMV/PPD, ventilation classes |
| **DIN 4108** | `514809c9b4` | `s/plugin/norm/din/4108/rs/lib.rs` | 1044 | Glaser, summer heat, airtightness, `LayerDocument` model |
| **EN 1992 + FEM** | `514809c9b4` | `s/plugin/norm/en/1992/rs/lib.rs` | 836 | `check_rc_beam_from_fem`, `fem_core` integration (~lines 566–664) |
| **DIN 18599** | `514809c9b4` | `s/plugin/norm/din/v/18599/rs/lib.rs` | 742 | `balance_annual` calling parts 1–12, cross-crate DIN 4108/16798 wiring |
| **norm_core** | `514809c9b4` | `s/plugin/norm/core/rs/lib.rs` | 772 | `NormFamily`/`NormHost`/`Quantity` kernel before plugin inlining |
| **norm_plugin** | `514809c9b4` | `s/plugin/norm/plugin/rs/lib.rs` | 251 | 13-app registration, pre-artifact `define_norm_family_app!` |

**Recovery command template:**

```bash
git show 514809c9b4:s/plugin/norm/en/1993/rs/lib.rs | less
git show 514809c9b4:s/plugin/norm/vdi/3805/rs/lib.rs > /tmp/vdi3805-reference.rs
```

Also useful: `.cursor/plans/norm_absolute_completeness_da7095c2.plan.md`, `.cursor/plans/norm_plugin_apps_88de84b8.plan.md` (completeness gate definition).

---

## 4. Conventions & decisions implementation must respect

### 4.1 From completed tickets (do not violate)

| Convention | Source | Rule |
|------------|--------|------|
| **No artifact engines** | ENGINELESS Aug 12 | `evaluate()` lives in `💡️inferences/🦀️.rs`; artifacts are schema+io only |
| **No `SetSnapshot` in mutation vocabulary** | Wave M | Use per-field `change-*`/`insert-*`/`remove-*`/`reorder-*`; whole-doc replace via `from_snapshot(base, target) -> Vec<Mutation>` |
| **`evaluate` app command emits zero mutations** | Wave M §4 | `Ok(Emit::default())` — report is derived on read |
| **Composed child tables** | Wave 2 EN 1990, Sep 8 audit | `q_k`, `layers`, etc. are `s.stdio.semio`/`table` child slots — not inline-only vecs in wire projections |
| **Emoji-unique within facet** | Wave M §6 | Each mutation triad dir has distinct emoji; cross-facet reuse acceptable |
| **`#[path]` glue mounting** | Shape V2, Wave 5 | Grouping modules use `#[path = "."]`; leaves prefixed `../../` from `📦️glue.rs` |
| **No serde at runtime** | Repo convention | `serde` only under `cfg(test)` or `feature = "compliance-testing"` on norm types |
| **AnnexParams pattern** | July checklist | Eurocodes: `AnnexChoice::De` must change numeric results where NDP differs |
| **DIN scope decisions** | July checklist | DIN 4108/16798/18599: no EN/DE annex split; EN 1997 classic generation only; EN 1998 excludes dams |
| **Leaf package boundaries** | Sep 8 extraction | 15 crates `semio-s-artifact-norm-<id>`; plugin is composition facade; dependency chain documented |
| **FEM optional** | Sep 8 | `cross-fem` feature → `semio-s-artifact-fem-2d`; EN 1992/1993 only |

### 4.2 From current ticket objective (Wave B–D must add)

- `CheckResult` → remediation + localization + subject-element reference + grouping
- Subject snapshot must model **complete assessment subject** (not scalar demo)
- Report: per-clause comply / fail / how-to-comply in **en + de**
- End-to-end: open artifact → edit all subject fields → localized report in viewer

### 4.3 Stale documentation to fix (not blocking Wave A)

- `✏️s/🔌️plugins/📕️norm/AGENTS.md` references dead `norm/core`, `norm/din`, `iso/16757/rs/lib.rs` paths

---

## 5. Open colliding tickets (avoid duplicate work)

### 5.1 Direct norm scope — **do not reopen/close; coordinate**

| Ticket ID | Status | Scope | Collision risk |
|-----------|--------|-------|----------------|
| `26/09/26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS` | **open** | This fleet: audits → core model → per-family rebuild → verify | **Owner** |
| `26/07/19/NORM-ENERGY-FAMILY-COMPLETENESS-GAPS` | open | DIN 4108/16798/18599 gaps | Overlaps Wave C energy families |
| `26/07/19/NORM-EN-1993-AND-EN-1998-FEATURE-COMPLETENESS` | open | Steel + seismic completeness | Overlaps Wave C |
| `26/07/19/NORM-EN-1994-1999-FEATURE-COMPLETENESS` | open | EN 1994–1999 gaps | Overlaps Wave C |
| `26/07/19/RUST-CLEAN-REFACTOR-WAVE-12-NORM-CRATES-FEM-2D-FEM-3D` | open | Norm crates + FEM 2D/3D | Overlaps FEM/`cross-fem` wiring |
| `26/07/26/CONVERT-EN-1997-NORM-CRATE-TO-DECLARATIVE-DSL-ENGINE` | open | EN 1997 DSL engine | Conflicts if Wave C rewrites EN 1997 imperatively |
| `26/07/18/EXPAND-EN-1998-AND-EN-1999-NORM-CRATES` | open | EN 1998/1999 expansion | Overlaps Wave C |

### 5.2 Cross-cutting — may touch `📕️norm`

| Ticket ID | Status | Norm touchpoint |
|-----------|--------|-----------------|
| `26/08/29/STUBS-AND-PLACEHOLDERS-COMPLETION` | open | Repo-wide `todo!`/`unimplemented!` census includes norm |
| `26/08/11/SEMIO-ARTIFACT-LOSSLESS-WELL-KNOWN-FORMAT-ROUNDTRIPS` | open | Norm plugin listed among 15 rewired plugins |
| `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS` | open | May exercise norm apps in demo studio |
| `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP` | open (active) | `wp-p3` norm app-surface gates; descriptor links |
| `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION` | open | `norm-fixture-authoring` |
| `26/08/12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL` | open* | Wave M **done** for norm; ticket still open for other plugins |

\*Closed work landed; ticket status may lag.

### 5.3 Closed but authoritative for conventions

`NORM-TECHNOLOGY-ABSOLUTELY-FEATURE-COMPLETE`, `NORM-PLUGIN-AND-PER-FAMILY-APPS`, `NORM-PLUGIN-SHAPE-V2-TREE-PURITY-RETROFIT`, `ARTIFACT-SCHEMA-FACETS`, `ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES`, `COMPOSABLE-STDIO-ARTIFACT-PACKAGES` (norm extraction packet).

---

## 6. Key ticket artifact index

| Path | Contents |
|------|----------|
| `🌙️07/☀️18/NORM-TECHNOLOGY-ABSOLUTELY-FEATURE-COMPLETE/part-checklist.md` | Per-part gate checklist (claimed all done) |
| `🌙️07/☀️18/NORM-TECHNOLOGY-ABSOLUTELY-FEATURE-COMPLETE/norm_feature_complete_73be55cf.plan.md` | Original completeness gate definition |
| `.cursor/plans/norm_absolute_completeness_da7095c2.plan.md` | Round-2 remediation plan |
| `.cursor/plans/norm_plugin_apps_88de84b8.plan.md` | NormHost + 13 apps architecture |
| `🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️waveM-reports/norm-lane-summary.md` | 392 triads, `from_snapshot`, evaluate behavior |
| `🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️wave2-reports/norm-*.md` | Per-family wave 2 mutation reports |
| `🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS/🧪wave5-norm-*` | Schema facet generators + glue integration |
| `🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/📓️norm-artifact-package-extraction.md` | 15 leaf crate dependency graph |
| `🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/norm-*.md` | Representation ownership audits |

---

## 7. Risks for coordinator

1. **July closed tickets overclaim** — checklist `[x]` does not match Sep 26 compliance objective; reopening July tickets would collide with this fleet — prefer supersede via current ticket Waves B–D.
2. **Seven open July norm tickets** — stale scope; should be closed or explicitly deferred before Wave C to avoid two agents rewriting EN 1993.
3. **Recovery ≠ copy-paste** — `514809c9b4` code uses `norm_core::`, `Document`, `SetDocumentOperation`; must be adapted to `En1993Snapshot`, `⚖️compliance`, facet paths, no runtime serde.
4. **Test illusion** — many tests assert `6.10a`/`6.10b` presence or `!is_empty()`; Wave D must require numeric worked examples with derivation.
5. **Hub MCP ticket active** — `wp-p3` norm app-surface tests may race Wave C editor/viewer changes.
6. **`AGENTS.md` stale** — misleads agents about crate locations.

---

## 8. Git commands used (read-only)

```bash
git log --oneline -- '✏️s/🔌️plugins/📕️norm' | head -80
git log --diff-filter=D --name-only --oneline -- '*norm*' | head -200
git show 514809c9b4:s/plugin/norm/<family>/rs/lib.rs
```

Deletion of headless crates: `3076aaaa02` removes all `s/plugin/norm/{din,en,iso,vdi,core,plugin}/**/lib.rs`.
