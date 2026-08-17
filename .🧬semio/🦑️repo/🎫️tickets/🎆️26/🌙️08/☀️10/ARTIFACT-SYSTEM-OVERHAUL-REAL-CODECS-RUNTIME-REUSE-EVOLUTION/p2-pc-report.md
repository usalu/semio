# Phase 2 PC Report — Pilot Closer (grammar recipe, policy seeding, graduation)

Scope: the plan's "PC — Pilot closer" wave, after M1 (grammar/lexer), M2 (protocol/walker), M3
(harness/registry/envelope), and the P1-P3 pilot ladder (json, csv, zip, png, txt, binary) all
landed and were independently verified green (whole-crate 1671/0/1-ignored). Three deliverables:
`📖️grammar-recipe.md`, four (in practice five, see §2) new shrink-only `📜️script.ts` policy rules,
and graduating the 6 piloted standards out of the framework's `STDIO_CONFORMANCE_GRADUATED`
exempt list. Sole authorization for this wave to touch `📜️script.ts` AND the framework's
`🧪️fixture-sweep/🦀️component.rs` graduation mechanism, per M3's design.

Read in full before starting: the entire "PHASE 2 PROGRAM" section of
`~/.claude/plans/the-current-schemas-are-scalable-journal.md` (incl. "Phase 2 execution log"),
and all 11 ticket-folder reports (`p2-w0-recon-report.md`, `p2-m1-report.md`, `p2-m2-report.md`,
`p2-m3-report.md`, `p2-p1-json-report.md`, `p2-p1-csv-report.md`, `p2-p1-fix-report.md`,
`p2-p2-zip-report.md`, `p2-p2-png-report.md`, `p2-p3-txt-report.md`, `p2-p3-binary-report.md`).

---

## 1. `📖️grammar-recipe.md` — complete

`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/📖️grammar-recipe.md`

Contains, each with real verbatim excerpts and exact file citations (grammar/protocol dialect
syntax pulled directly from the 6 pilots' own committed `.grammar.semio`/`.protocol.semio` files
on disk, not paraphrased from the reports):

1. **Grammar-file syntax**: header (`dialect grammar`/`grammar <id>`/`extension`/`start`),
   productions/alternation/grouping/terminals, all 6 M1 capabilities (string escape modes,
   `LINE`/`REST` raw-span, promoted tokens, per-grammar comment dialect, trailing-dot
   float/leading-dot enum, the `hex` macro) — with verbatim excerpts from json, csv, txt (pilots
   that exercise these) and, where no pilot exercises a capability (promoted tokens `<>&$;`,
   trailing-dot float/`DOTENUM`, `comment block`), an explicitly-labeled citation to M1's own
   worked unit tests instead (transparently marked as "not yet exercised by a pilot").
2. **Protocol-file syntax**: header/framing, `Prim` types incl. all 7 BE variants, all 6 M2
   capabilities (`repeat`/`arm`/`until`/`nested`, `marker()`, cross-block field-env threading,
   `Cond`, ZIP's `backward`/`jump`, TIFF's `endian{}`) — with full worked verbatim excerpts from
   zip's snapshot protocol (the real 3-block `repeat`/`backward`/`jump` ZIP layout) and png's
   snapshot protocol (the real `repeat`/BE-fields chunk loop), plus a dedicated worked section
   (§2.4) reproducing binary's real `.spk`-container diff protocol verbatim — flagged as the
   copy-paste answer for EVERY future `#[derive(dsl::DslDiff)]`-derived standard, not just binary.
3. **The 5 documented authoring pitfalls** from `p2-p1-fix-report.md` + the P2-P2 png finding:
   grouping is always `{...}`; the `hex` macro (not hand-rolled `{INT|IDENT}*`); the 5 reserved
   header keywords can't be production names; one physical line per production; `Prim::Ref` still
   can't recurse — each with the real worked example of the pitfall (json's `string` production
   rename, png's 2 multi-line productions that had to be collapsed).
4. **The full per-standard deliverable checklist** — grammar/protocol pair per facet, the 6
   conformance-law test names, real fixtures (with the "generate via a temp test, delete before
   finishing" convention every pilot used), 5-role `LanguageSpec` registration (with the
   `register_schema_spec` caveat from binary's own fix — register whichever specs are genuinely
   derivable, don't skip the whole call just because the mutations facet has multiple per-variant
   specs), JSON-transfer elimination, and an explicit "you do NOT touch `STDIO_CONFORMANCE_GRADUATED`"
   line.
5. **The M2 exclusions carve-out** (DWG ac1024's decrypt/decompress, PDF/1.7's full object graph,
   cross-dialect field-width parameterization) restated for FG2/FG3.
6. **A consolidated "known mechanism gaps" table** — every `mechanism_gaps` entry from all 6 pilot
   reports, deduplicated where the same root cause was independently rediscovered
   (`protocol-prim-ref-recursion` hit by json/csv/png/zip/binary independently;
   `protocol-array-of-records` is the general form zip/csv both named separately) — each row has
   which artifacts hit it and the exact honest-workaround pattern, so later waves recognize these
   immediately instead of rediscovering them from scratch.

---

## 2. Four (in practice five) new policy rules — seeded from a real, fresh census

The mission's own deliverable list names 5 distinct rules (`POLICY_GRAMMAR_PARSEABILITY`,
`POLICY_PROTOCOL_PARSEABILITY`, `POLICY_FIXTURE_HONESTY`, `POLICY_LANGUAGE_REGISTRATION`,
`POLICY_STDIO_JSON_TRANSFER_BAN`); the hard-exit-gate text says "4 new rules" — built all 5 named
in the deliverable list (grammar and protocol parseability are genuinely separate consts/allowlists,
matching how every other file/facet-scoped policy in this file is split). All follow the exact
Phase-1 S-8 house style (`policyGrammarHonestyBreaches` et al. — a `POLICY_<NAME>_ALLOWLIST` Set,
a `policy<Name>Breaches(repoRoot)` function, "new breach if bad-and-not-allowlisted, stale breach
if allowlisted-but-already-fixed"), added as a new `//#region 🔧️PolicyRuleSchemaOverhaulPC` in
`📜️script.ts` (right after S2's own region), aggregated by `policySchemaOverhaulPCBreaches` and
wired into the main `policy` export.

**Every seed number below was computed by actually running the detection logic against the real
tree** (first as a standalone Python replica of each TS check for fast iteration, then confirmed
via direct `bun run` import of the real `policySchemaOverhaulPCBreaches` function against the live
repo — see §4 for the sanity-check methodology that proves the rules genuinely execute, not just
compile).

### `POLICY_GRAMMAR_PARSEABILITY` — 138 seeded (of 159 discovered `.grammar.semio` files)

Textual heuristic: real header shape (`dialect grammar` own-line / `grammar <id>` / `start <prod>`)
AND no leftover ABNF tell (`;`-prefixed comment line, `%xHH` char-class, `*hexdig`/`*OCTET` prefix
repetition — checked only outside `#`-comment lines, so a real grammar's own doc-comment prose
quoting the old ABNF spec, e.g. csv's own RFC 4180 §2 citation, never false-positives).

Seed: **21 of 159 already look real** — the 6 pilots' 3 facets each (json/csv/zip/png/txt/binary
× snapshot/diff/mutations = 18) **plus** `stdio/semio#v1`'s `✳️object` subset (3 more) — a
currently-live, unrelated concurrent ticket's WIP artifact that happens to already satisfy both
checks by accident (real `#`-comment style, no ABNF tells) even though it was never part of this
program's roster. Included honestly in the "already real" set (excluded from the 138-entry seed)
rather than force-included to match a hand-picked "exactly 6" expectation — this is what "actually
run the check" means. `pdf/1.7` (which W0 flagged as having a contract-correct header but an
unconverted ABNF body) is correctly caught as still-fossil by the ABNF-tell check and IS in the
138-entry seed.

### `POLICY_PROTOCOL_PARSEABILITY` — 138 seeded (of 159 discovered `.protocol.semio` files)

Same heuristic/header shape (`dialect protocol`/`protocol <id>`/`start <block>`), same 21-of-159
already-real set (same 6 pilots + `semio#v1/object`).

### `POLICY_FIXTURE_HONESTY` — 31 seeded (of 37 stdio artifact dirs)

Per-ARTIFACT (not per-standard — the demo fixture pair lives once per artifact dir, shared across
a multi-standard artifact like gif 87a/89a): `🗣️example.dsl.semio`'s first line must start with
`semio stdio.<artifact>` AND a sibling `🎒️example.pack.semio` must exist. Seed: exactly the 6
pilots (binary/csv/json/png/txt/zip) are clean; all 31 other stdio artifact dirs are still fake —
including `stdio/schema`, a stray content-less `🧬️schema` directory sitting directly under
`🗿️artifacts/` (debris, not a real artifact — harmless to seed, will surface as a stale breach if
that directory is ever cleaned up).

### `POLICY_LANGUAGE_REGISTRATION` — 34 seeded (of 40 discovered stdio standards)

`⚙️engine/🦀️component.rs`'s own `register_language` call count, per (artifact, standard) — must
be ≥5 (the full 5-role set every P1-P3 pilot landed). Seed: exactly the 6 pilots have 5 calls each;
all 34 other standards have 0-1 (pre-Phase-2 single-role registration or none).

### `POLICY_STDIO_JSON_TRANSFER_BAN` — 25 seeded (real current census, wider than W0's original 4)

Brace-matched scan of each `impl (...::)?(ArtifactPack|OpBinary|DiffCodec) for ... { ... }` block
for a literal `serde_json::to_vec(`/`from_slice(` call inside the block body (not a whole-file
grep — an artifact's legitimate NATIVE json parsing elsewhere in the same file, e.g. gltf's own
`⚙️engine`, is never a false positive), plus a second, narrower check for the one real
cross-artifact bridge W0's census found that isn't literally one of those 3 impls (any `.rs` file
under a `🚪️io/` bridge dir using `serde_json::{to_vec,from_slice,to_string,from_str}(`).

**Re-confirmed by direct grep, not assumed**: W0's originally-named 4 (ifc/2x3's mutations
`OpBinary`, svg's and xml's snapshot `ArtifactPack`, gltf's io bridge) are all **still real
violations**, unfixed by the pilot ladder (none of those 4 were in the P1-P3 roster). The
detection logic, run for real, additionally found **21 more** from a separate, currently-live
concurrent ticket scaffolding new stdio artifact types: avi/mp3/mp4/wav's mutations `OpBinary`,
and 17 of 🧿️semio v1's many subsets' snapshot `ArtifactPack`/mutations `OpBinary`. Included
honestly (25 total, not artificially capped at 4) per the mission's own "confirm by checking
current state, don't assume" instruction.

### Wiring / verification

```
📜️script.ts new region: //#region 🔧️PolicyRuleSchemaOverhaulPC  (after //#endregion 🔧️PolicyRuleSchemaOverhaulS2)
  policyLooksLikeRealGrammarOrProtocolDialect()   — shared heuristic
  policyGrammarParseabilityBreaches()             — POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST (138)
  policyProtocolParseabilityBreaches()            — POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST (138)
  policyFixtureHonestyBreaches()                  — POLICY_FIXTURE_HONESTY_ALLOWLIST (31)
  policyLanguageRegistrationBreaches()            — POLICY_LANGUAGE_REGISTRATION_ALLOWLIST (34)
  policyStdioJsonTransferBanBreaches()            — POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST (25)
  policySchemaOverhaulPCBreaches()  aggregates all 5, wired into the main `policy` export
```

`bun run ./📜️script.ts policy` (full run, real output):
```
21535 high-priority breach(es) across 25 rule(s):   [unchanged from pre-PC baseline — no regression]
```
Direct import of `policySchemaOverhaulPCBreaches(repoRoot)` (bypassing the priority filter that
hides medium/low breaches from the printed summary): **0 breaches** — confirmed via the process's
own full JSON breach cache (`.🦑️repo/⚡️cache/breaches/compose.json`, 22115 total repo-wide,
`stdio-artifacts/grammar-parseability` / `protocol-parseability` / `fixture-honesty` /
`language-registration` / `json-transfer-ban` all present at count 0) — **zero drift between the
seed and a fresh census, in both directions** (no new breach, no stale allowlist entry).

**Sanity check that the rules genuinely execute** (not merely "coincidentally zero because
unreachable"): temporarily removed `"stdio/las"` from `POLICY_FIXTURE_HONESTY_ALLOWLIST`, re-ran
the direct import → **1 breach reported** (`stdio-artifacts/fixture-honesty` for `las`), exactly
as expected; restored the entry, re-ran → back to 0. Confirms the detection logic is live and
correctly discriminates, not a silently-broken no-op.

---

## 3. Graduation — 14 tuples appended to `STDIO_CONFORMANCE_GRADUATED`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`
(`//#region 🔖️StdioTransition`). Per M3's design (read from `p2-m3-report.md`): the exempt set is
"all of stdio, minus whichever `(artifact, standard, facet)` tuples have graduated" — append-only,
never edit/remove another standard's entry.

Graduated, all 6 pilots' `Grammar` + `ProtocolPack` facets (real snapshot grammar+`.dsl.semio`,
real snapshot protocol+`.pack.semio`, both genuinely passing):

```rust
("🔣️json", "🔖️rfc8259", ConformanceFacet::Grammar),
("🔣️json", "🔖️rfc8259", ConformanceFacet::ProtocolPack),
("📊️csv", "🔖️rfc4180", ConformanceFacet::Grammar),
("📊️csv", "🔖️rfc4180", ConformanceFacet::ProtocolPack),
("📊️csv", "🔖️rfc4180", ConformanceFacet::ProtocolSpr),
("🎒️zip", "🔖️2.0", ConformanceFacet::Grammar),
("🎒️zip", "🔖️2.0", ConformanceFacet::ProtocolPack),
("📷️png", "🔖️1.2", ConformanceFacet::Grammar),
("📷️png", "🔖️1.2", ConformanceFacet::ProtocolPack),
("📄txt", "🔖️utf-8", ConformanceFacet::Grammar),
("📄txt", "🔖️utf-8", ConformanceFacet::ProtocolPack),
("📄txt", "🔖️utf-8", ConformanceFacet::ProtocolSpr),
("💾️binary", "🔖️raw", ConformanceFacet::Grammar),
("💾️binary", "🔖️raw", ConformanceFacet::ProtocolPack),
```

**`ProtocolSpr` (mutations protocol + `.spr.semio` fixture) graduated ONLY for csv and txt** —
verified by direct disk check: only these 2 pilots shipped a real `📡️example.spr.semio` fixture
(json/zip/png/binary's mutations protocol facets ARE real dialect per their own reports, but have
no `.spr.semio` fixture on disk to check against). Graduating a facet with nothing to verify would
be graduation theater, not a real gate — deliberately withheld for those 4 rather than
force-graduated. (Confirmed safe either way: `m5_handcrafted_protocol_conformance`'s own missing-
fixture soft-skip fires BEFORE the exemption check, so even a mistaken graduation of a
fixture-less facet could not have hard-failed — verified by reading the harness source directly,
not assumed.)

### Verification (real output, pasted)

`cargo test -p semio-framework-os-kernel --lib fixture_sweep`:
```
test os_dsl::fixture_sweep::m5_semio_envelope_protocol::semio_envelope_protocol_parses_under_the_real_dialect ... ok
test os_dsl::fixture_sweep::m5_semio_envelope_protocol::semio_envelope_protocol_walks_a_different_token_length_and_an_empty_payload ... ok
test os_dsl::fixture_sweep::m5_semio_envelope_protocol::semio_envelope_protocol_walks_a_real_wrap_binary_payload ... ok
test os_dsl::fixture_sweep::m5_handcrafted_protocol_conformance::all_discovered_snapshot_protocols_walk_their_shipped_fixtures ... ok
test os_dsl::fixture_sweep::m5_cross_artifact_rejection::all_non_stdio_grammars_reject_each_others_shipped_fixtures ... ok
test os_dsl::fixture_sweep::m5_production_coverage::all_discovered_grammars_report_uncovered_productions_for_their_shipped_fixture ... FAILED
test os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::all_discovered_snapshot_grammars_recognize_their_shipped_fixtures ... FAILED
[dsl-fixture-sweep] m5 grammar auto-discovery: 59 facet(s) found, 59 checked, 0 soft-skipped, 47 stdio-exempt soft failure(s), 3 hard failure(s)
test result: FAILED. 5 passed; 2 failed; 0 ignored; 0 measured; 757 filtered out; finished in 0.29s
```

**The 6 graduated standards genuinely pass through the framework's own m5 harness — real, verified,
not assumed**: `stdio-exempt soft failure(s)` dropped from **53 (M3's own baseline) to 47** —
exactly 6 fewer, matching the 6 grammar facets moved off the exempt-and-failing side onto the
exempt-and-passing (i.e. no-longer-counted-as-a-failure-at-all) side. Both hard-failure counts
(**3, in both the grammar-conformance and production-coverage tests**) are IDENTICAL to the
pre-graduation baseline — the same 3 non-stdio pilots (`dag`, `en1992`, `fem2d`) that were already
red before M1 even started (per W0's own recorded baseline) — confirming **none of the 6 graduated
standards regressed to a hard failure**; the harness's own protocol-conformance test (`Pack`+`Spr`
kinds, both graduated and un-graduated facets together) passed clean on the FIRST run, 0 failures,
no iteration needed.

`cargo test -p semio-framework-os-kernel` (full crate, post-graduation):
```
test result: FAILED. 762 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s
```
Exactly the pre-existing baseline (`dag`/`en1992`/`fem2d`, unchanged) — matches the hard-exit gate's
own wording ("the same pre-existing pilot-failure baseline as before, not worse") exactly.

---

## 4. Hard exit gates — all 4, real pasted output

### Gate 1 — `cargo test -p semio-s-plugin-stdio --lib`

Baseline (before any PC-wave edit, confirmed first per the mission's own instruction):
```
test result: ok. 1671 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 7.77s
```
Final (after script.ts + fixture-sweep graduation edits — expected byte-identical, since neither
edit touches any stdio artifact source):
```
test result: ok. 1671 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 7.77s
```
**1671/0/1-ignored, exact match, both before and after.**

### Gate 2 — `cargo test -p semio-framework-os-kernel`

Baseline (before graduation, confirmed first): `762 passed; 2 failed` (dag/en1992/fem2d, aggregated
into 2 test names per M3's own refactor). Final (after graduation): `762 passed; 2 failed`, same 2
test names, same underlying 3-pilot verdict — see §3 for the full breakdown proving the 6
graduated standards pass for real through the m5 harness specifically (not just "the number didn't
change").

### Gate 3 — `bun run ./📜️script.ts policy`

`21535` high-priority breaches, unchanged from the pre-PC-wave baseline (no regression on any
existing rule). The 5 new rules: seeded accurately, **zero drift** between the seed and a fresh
census (§2), confirmed twice (once immediately after seeding, once again after the graduation edit
— the two changes are independent, script.ts doesn't scan `.rs` test-module content the graduation
edit touches in any way that would move an existing rule's count).

### Gate 4 — `📖️grammar-recipe.md`

Written, comprehensive, real verbatim excerpts with exact citations throughout (§1). Located at
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/📖️grammar-recipe.md`.

---

## 5. Files touched

- `📜️script.ts` — new `//#region 🔧️PolicyRuleSchemaOverhaulPC` (5 policy rules + shared heuristic +
  aggregator), one new line wiring `policySchemaOverhaulPCBreaches` into the main `policy` export.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs` — 14 tuples appended
  to `STDIO_CONFORMANCE_GRADUATED`, doc comment extended explaining the graduation + the
  deliberate `ProtocolSpr` withholding for the 4 fixture-less pilots.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/📖️grammar-recipe.md`
  — new file, this wave's main content deliverable.
- This report.

No stdio artifact source file was touched (confirmed — this wave's own mandate was script.ts +
the framework graduation list + a ticket-folder doc, never artifact code; Gate 1's byte-identical
before/after result independently confirms this).

---

## 6. Ready for FG1 — explicit statement

**FG1 (text-native fan-out: md, xml, obj, stl, dxf, step+ifc4, csv-family leftovers) has
everything it needs to start:**

1. **The mechanism spine is complete and stable** — M1 (grammar/lexer), M2 (protocol/walker), M3
   (harness/registry/envelope) all landed, zero regressions, confirmed again by this wave's own
   fresh gate runs (§4).
2. **`📖️grammar-recipe.md` is the copy-pasteable reference** every FG1 agent's brief should point
   at instead of re-reading all 11 raw pilot reports — real syntax, real pitfalls, real mechanism
   gaps, the full per-standard checklist, all in one place with exact citations back to the source
   files for anyone who wants to verify further.
3. **The enrollment mechanism needs zero framework edits from FG1** — land a real grammar/protocol
   pair + fixtures under your own artifact's tree and `m5_handcrafted_grammar_conformance`/
   `m5_handcrafted_protocol_conformance` discover and soft-check it automatically on the very next
   `cargo test -p semio-framework-os-kernel` (M3's own "ownership keystone," re-confirmed working
   by this wave's own graduation — the discovery walk found all 6 pilots' real files with zero
   framework-side enrollment edits needed).
4. **FG1 agents must NOT touch `📜️script.ts` or `🧪️fixture-sweep/🦀️component.rs`** — per this
   wave's own mandate and every prior pilot's explicit deviation note, graduation
   (`STDIO_CONFORMANCE_GRADUATED`) and policy-allowlist shrinkage are a framework-owner/closer-only
   action. FG1 agents land real, passing, in-artifact conformance tests (the 6 laws, §1 item 4)
   as their own early-warning; a future closer/graduation pass shrinks the 5 PC-wave policy
   allowlists and appends `STDIO_CONFORMANCE_GRADUATED` tuples once it re-confirms each standard's
   census for real (exactly how this PC wave itself was run — see §2's methodology).
5. **The 4 known mechanism-gap categories most likely to recur in FG1's own roster** (md, xml,
   obj, stl, dxf, step, ifc): `protocol-prim-ref-recursion`/`protocol-array-of-records` (any
   nested/recursive value type — xml/md's node trees, step/ifc's entity graphs will all hit this
   immediately), `csv-newline-trivia`'s general form (any line-oriented text format needs the same
   "recover the boundary structurally, no NEWLINE terminal exists" treatment — obj/stl/dxf all
   qualify), the M1-documented "markdown's whitespace-count nesting is architecturally impossible
   for this token model" exclusion (md's own FG1 agent must model what's expressible and document
   the nesting gap explicitly, per the plan's own decided scope — not attempt a workaround), and
   STEP/IFC's own real use case for M1's `comment block "/*" "*/"` + `string single doubled` +
   `DOTENUM` capabilities (built and unit-tested by M1, never yet exercised by a real pilot file —
   FG1's step/ifc agent will be the FIRST real user of these, worth extra care verifying against
   the real Part-21 lexer's exact escape/comment semantics before trusting the recipe's own
   M1-report-sourced excerpt blindly).
