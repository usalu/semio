# P2-FG1 Closer Report

Scope: close out wave FG1 (`md`, `xml`, `obj`, `stl`, `dxf`, `step` ap214, `ifc` v4 — 7 standards,
6 fan-out agents, step+ifc4 combined into one). Read all 6 fan-out reports
(`p2-fg1-{md,xml,obj,stl,dxf,step-ifc}-report.md`) and the independent verification
(`p2-fg1-verify-report.md`) in full before acting. This closer is the sole agent in the wave
authorized to touch `📦️glue.rs`, `📜️script.ts`, and the framework's `🧪️fixture-sweep` graduation
list.

## 1. `glue_followup` items — none existed

None of the 6 fan-out reports requested a `glue_followup` item. `📦️glue.rs` appears in only one
report (`p2-fg1-dxf-report.md`), cited read-only as evidence that `store`/`dsl`/`protocol` are
`extern crate self as …` aliases for the same kernel crate (justifying `store::write_varint_u64`/
`store::ByteReader` reuse in dxf's new binary primitives) — never edited. `📜️script.ts` and the
fixture-sweep graduation list were untouched by every fan-out agent (all correctly deferred
graduation to this closer, per the ticket's ownership boundary). Nothing to apply.

## 2. Full crate gate — `cargo test -p semio-s-plugin-stdio --lib`

Fresh run, no filter: **1714 passed, 0 failed, 1 ignored**. Matches the independent verifier's own
fresh count exactly (`p2-fg1-verify-report.md` §9), exceeds the recipe's own "≥1671/0/1-ignored"
baseline. Covers all 6 pilots (json/csv/zip/png/txt/binary) plus this wave's 7 standards (13 total)
with zero failures anywhere. Raw output: `p2-fg1-closer-full-crate-test.txt`.

## 3. Policy gate — `bun run ./📜️script.ts policy`

Ran fresh (raw output: `p2-fg1-closer-policy-run.txt`, 21558 lines, exit code 1 — expected, many
pre-existing unrelated breaches remain repo-wide in policies outside this wave's scope). Checked the
5 PC-seeded rules specifically by `kind` string (confirmed exact strings by reading `📜️script.ts`
directly rather than guessing):

| rule | `kind` string | breaches repo-wide | breaches for md/xml/obj/stl/dxf/step/ifc |
|---|---|---|---|
| grammar parseability | `stdio-artifacts/grammar-parseability` | 0 | 0 |
| protocol parseability | `stdio-artifacts/protocol-parseability` | 0 | 0 |
| fixture honesty | `stdio-artifacts/fixture-honesty` | 0 | 0 |
| language registration | `stdio-artifacts/language-registration` | 0 | 0 |
| JSON-transfer ban | `stdio-artifacts/json-transfer-ban` | 0 | 0 |

All 5 are shrink-only allowlist rules (`POLICY_*_ALLOWLIST` sets in `📜️script.ts`). Independently
grepped all 5 allowlist definitions for `📝️md`, `📰xml`, `🧊️obj`, `🟪️stl`, `🖊️dxf`, `📐️step`,
`🏗️ifc` — zero hits in any of them, confirming every one of the 7 standards was cleanly removed
from (or never needed to be added to) every allowlist by its fan-out agent. **Shrink confirmed for
this wave's 7 standards, and — since the repo-wide breach count for all 5 rules is exactly 0 — no
growth occurred for any other standard either.**

**Separately noticed, not part of this closer's scope**: the unrelated, pre-existing
`handcrafted-grammar/generic-spec` heuristic (NOT one of the 5 PC-seeded rules; it predates Phase 2
entirely) flags dxf's real, genuine `tables-diff-payload` production as a false positive — its
regex `/-(json|blob|base64|payload)\b/` matches the substring "-payload" at the end of that
legitimate production name, not an actual generic/placeholder field. Read the full production
(`🔺️diff/📝️text/📖️component.grammar.semio` under dxf) and confirmed it is genuinely domain-true,
real recursive dxf-diff grammar — not a migration leftover. Left untouched: this policy is outside
the 5 this closer is chartered to verify, and fixing an unrelated legacy heuristic's false-positive
regex is scope creep beyond the closer's numbered task list.

## 4. Graduation — 14 tuples appended to `STDIO_CONFORMANCE_GRADUATED`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`
(`//#region 🔖️StdioTransition`). Confirmed the exact directory-name identifiers for each standard by
listing disk (`find … 🏅️standards -maxdepth 1`) rather than guessing: `md/commonmark`, `xml/1.0`,
`obj/3.0`, `stl/ascii`, `dxf/r12`, `step/ap214`, `ifc/4` (not `ifc/2x3`, which stays untouched and
out of scope).

Appended, replicating PC's own tuple shape exactly:

```rust
("📝️md", "🔖️commonmark", ConformanceFacet::Grammar),
("📝️md", "🔖️commonmark", ConformanceFacet::ProtocolPack),
("📰xml", "🔖️1.0", ConformanceFacet::Grammar),
("📰xml", "🔖️1.0", ConformanceFacet::ProtocolPack),
("🧊️obj", "🔖️3.0", ConformanceFacet::Grammar),
("🧊️obj", "🔖️3.0", ConformanceFacet::ProtocolPack),
("🟪️stl", "🔖️ascii", ConformanceFacet::Grammar),
("🟪️stl", "🔖️ascii", ConformanceFacet::ProtocolPack),
("🖊️dxf", "🔖️r12", ConformanceFacet::Grammar),
("🖊️dxf", "🔖️r12", ConformanceFacet::ProtocolPack),
("📐️step", "🔖️ap214", ConformanceFacet::Grammar),
("📐️step", "🔖️ap214", ConformanceFacet::ProtocolPack),
("🏗️ifc", "🔖️4", ConformanceFacet::Grammar),
("🏗️ifc", "🔖️4", ConformanceFacet::ProtocolPack),
```

`ProtocolSpr` deliberately **withheld for all 7** — verified by direct disk check
(`find "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts" -iname "*.spr.semio"`): only `csv` and `txt` (both
already graduated by PC) have a real `📡️example.spr.semio` fixture anywhere in the entire repo.
None of the 6 fan-out reports for this wave mention shipping one (independently confirmed: grepped
all 6 reports for `spr.semio`/`example.spr`/`ProtocolSpr` — zero hits). Graduating `ProtocolSpr`
without a fixture to verify against would be graduation theater, exactly the pattern PC's own report
documents avoiding for json/zip/png/binary. Correctly withheld, not an oversight.

## 5. Framework m5 harness — `cargo test -p semio-framework-os-kernel`

**`fixture_sweep` filtered run** (raw: `p2-fg1-closer-framework-fixture-sweep.txt`):

```
[dsl-fixture-sweep] m5 grammar auto-discovery: 59 facet(s) found, 59 checked,
  0 soft-skipped, 40 stdio-exempt soft failure(s), 3 hard failure(s)
test result: FAILED. 5 passed; 2 failed; 0 ignored; 0 measured; 757 filtered out
```

`stdio-exempt soft failure(s)` dropped from PC's own post-graduation baseline of **47** to **40** —
exactly 7 fewer, matching the 7 newly-graduated `Grammar` facets moving from the exempt-and-failing
side to the exempt-and-passing side. Both hard failures (`m5_handcrafted_grammar_conformance`,
`m5_production_coverage`) are identically attributed to the same 3 pre-existing non-stdio pilots
(`🏗️fem::◻2d`, `📕️norm::📘️en1992`, `🕸️dag::🕸️dag`) that were already red before M1 — confirmed by
name in the panic output, unchanged count (3), zero new hard failures. `m5_handcrafted_protocol_
conformance` and the 3 other m5 test groups all pass clean.

One informational (non-failing) DEBUG note observed in `m5_production_coverage`'s output:
`🗄️stdio::📰xml::🔖️1.0: uncovered productions (1) = char-ref` — this is a soft coverage-reporting
log for a now-non-exempt (graduated) grammar, not an assertion failure; xml's own
`grammar_conformance_law`/`fixture_honesty_law` pass independently. Not a regression.

**Full-crate run** (raw: `p2-fg1-closer-framework-full.txt`):

```
test result: FAILED. 762 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Byte-identical to PC's own recorded post-graduation baseline (`762 passed; 2 failed`, same 2 test
names). Confirms the 7 newly-graduated standards pass for real through the framework's own m5
harness — not just "the number didn't change" — and that nothing else regressed.

## 6. `git check-ignore -v` on new paths

The only genuinely new (`??` untracked) paths introduced by this wave are the 7 artifacts'
`🎒️example.pack.semio` fixture files — every other touched file was a rewrite of an already-tracked
file inside an already-existing directory (confirmed via `git status --porcelain` scoped to all 7
artifact trees: every entry besides the 7 new fixtures is `M`). No new directories were created, so
there is nothing for `git check-ignore` to flag as a directory-level ignore surprise; ran it anyway
on the 7 new files directly:

```
git check-ignore -v <7 pack.semio paths>
```

→ zero matches (exit 1, no output) — none of the 7 new fixtures are gitignored; they will be picked
up correctly by any future `git add`/ticket-close file list.

## 7. STATUS.md — ownership ledger updated

Appended (did not remove any existing content) a new top-level `# Phase 2 —  real grammars/
protocols/binary codecs` section to `STATUS.md`, summarizing M1-M3/P1-P3/PC (brief, pointing at
their own reports) and this wave (FG1) in full detail, including the 4 gates run above and the
program tally: **13 of 31 official stdio standards now have `Grammar`+`ProtocolPack` graduated**
(6 from PC's pilot ladder + 7 from this wave), `ProtocolSpr` graduated for 2 of those 13 (csv, txt
only — the only ones with a real `.spr.semio` fixture). 18 standards remain for future FG-waves.

## 8. Known follow-up, explicitly not fixed this closer pass

The independent verification (`p2-fg1-verify-report.md` §3) found a real, well-evidenced shortfall:
4 of 7 standards (`stl`, `obj`'s diff facet, `step`, `ifc`) left `DiffCodec`/`OpBinary` on the F6
`print_diff()/print_op().into_bytes()` text-as-binary shortcut instead of performing the real
binary-frame upgrade the recipe's own checklist explicitly mandates ("expect to do a real upgrade
here for almost every standard, **not just check**") — even though 3 sibling standards in the same
wave (`md`, `xml`, `dxf`) proved the upgrade mechanically achievable for comparably- or
more-recursive types.

This does **not** block graduation: only `Grammar`+`ProtocolPack` facets were graduated for this
wave (matching what json/zip/png/binary — 4 of PC's own 6 pilots — also graduated without a
`ProtocolSpr`/binary-frame requirement), and no conformance-law test asserts binary density (the
round-trip laws pass regardless of whether the "binary" bytes are genuinely dense or just UTF-8 text
reinterpreted as bytes). It is not a `glue_followup` item either — fixing it requires editing each
artifact's own `🔺️diff/🦀️component.rs`/`🧬️mutations/🦀️component.rs`, files inside each artifact's
own ownership boundary, not this closer's `glue.rs`/`script.ts`/fixture-sweep-graduation exclusive
scope. Recorded here and in `STATUS.md` as a recipe-documented, non-blocking follow-up item for a
dedicated future pass on `stl`, `obj` (diff facet only), `step`, and `ifc`.

## 9. Files touched by this closer pass

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs` — 14-tuple
  graduation append to `STDIO_CONFORMANCE_GRADUATED` only, no other edit.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/STATUS.md`
  — appended, existing content untouched.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-fg1-closer-full-crate-test.txt` (new)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-fg1-closer-policy-run.txt` (new)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-fg1-closer-framework-fixture-sweep.txt` (new)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-fg1-closer-framework-full.txt` (new)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-fg1-closer-report.md` (this report, new)

No `📦️glue.rs`, no artifact-owned file (any of the 7 standards' own component.rs/grammar.semio/
protocol.semio/fixtures) touched. Ticket left open for the orchestrator/next wave (FG2/FG3).
