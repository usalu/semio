# P2-FG4 Closer Report

Scope: close out wave FG4 — the FINAL fan-out wave (docx/ecma-376, xlsx/ecma-376, pptx/ecma-376,
bcf/2.1, ifc/2x3 — 5 standards, 5 fan-out agents), completing real grammar/protocol work for all 32
official stdio standards. Read all 5 fan-out reports (`p2-fg-docx-report.md`, `p2-fg-xlsx-report.md`,
`p2-fg-pptx-report.md`, `p2-fg-bcf-report.md`, `fg-ifc-2x3-report.md`) and the independent verification
(`p2-fg4-verify-report.md`) in full before acting. This closer is the sole agent in the wave authorized
to touch `📦️glue.rs`, `📜️script.ts`, and the framework's `🧪️fixture-sweep` graduation list (the fan-out
agents' own ownership boundary explicitly excludes these; the closer role's numbered task list
explicitly requires touching two of them).

## 1. `glue_followup` items — none existed

Grepped all 5 fan-out reports plus the verify report for `glue_followup`/`glue.rs`/`mechanism_gap`.
None requested a `glue_followup` item. pptx's own report references F5's own pre-existing
`PptxOpc*Diff`/`DocxOpc*Diff` own-file-copy duplication (already flagged as `glue_followup` in F5's own
report) but explicitly notes this wave adds no new instance — nothing new to apply. `📦️glue.rs` was not
touched by any of the 5 fan-out agents (confirmed by reading each report's own "files touched" list),
nor by this closer.

## 2. Full crate gate — `cargo test -p semio-s-plugin-stdio --lib`

Run 3 times across this closer pass:

- **Run 1** (before any closer edits): `1866 passed, 4 failed, 4 ignored`. All 4 failures confined to
  `artifacts::semio::*` (`brep`/`model`/`object` subsets' `fixture_honesty_law`/`grammar_conformance_law`)
  — a wholly different, unrelated artifact family, not one of stdio's 32 official standards. Confirmed
  via `git status --porcelain -- "✏️s/…/🗿️artifacts/🧿️semio/"` showing **172 modified files** under
  that tree — matches this ticket's own documented "large concurrent session actively adding new
  artifact types under `🧿️semio/**`" ambient-churn warning exactly. None of FG4's own 5 standards'
  tests failed.
- **Run 2** (retried per "classify via file path, don't chase, retry once"): `1868 passed, 1 failed,
  3 ignored` — same `artifacts::semio::object` failure, count shrinking as the concurrent session's own
  edits progressed. Still zero FG4 involvement.
- **Run 3** (final, after all this closer's edits): **clean — `1870 passed, 0 failed, 3 ignored`**.
  All 260 of FG4's own 5 standards' tests pass cleanly across every one of the 3 runs (only the
  pre-flagged ambient `zzz_generate_p2p1_fixtures` test shows `ignored`, exactly as pptx's own
  fan-out report and the verify report both already documented).

Covers all 27 prior standards + this wave's 5 (**32 total, every official stdio standard**) with zero
failures anywhere. Raw captures: `fg4-closer-full-crate-1.txt`, `-2.txt`, `-final.txt`.

## 3. Policy gate — `bun run ./📜️script.ts policy`

Same structurally tautological CLI-level measurement FG2's/FG3's own closers already diagnosed: the 5
PC-seeded stdio rules (`stdio-artifacts/{grammar-parseability,protocol-parseability,fixture-honesty,
language-registration,json-transfer-ban}`) are unconditionally `priority: "low"` at every one of their
`breaches.push({...})` call sites, and `runPolicyExit`'s own CLI path prints ONLY `priority: "high"`
breaches — this command's printed output (**21621 high-priority breaches across 25 rules**) is
structurally incapable of showing shrink/growth for these 5 rules. Raw capture: `fg4-closer-policy-run.txt`.

Verified the real signal via a retargeted copy of FG2's/FG3's own direct-import scratch script
(`.🦑️repo/🎫️tickets/…/generators/policy_pc_breach_check_fg4.ts`, imports the exported
`policySchemaOverhaulPCBreaches` directly, bypassing the CLI's high-priority-only filter; kept per the
ticket's "don't delete scratch" rule). This wave's own copy needed one extra layer of care beyond FG3's
template: 4 of the 5 PC rules (`grammar-parseability`, `protocol-parseability`, `fixture-honesty`,
`language-registration`) report `scope` at the **ARTIFACT level** (no standard tag) for some or all of
their breach kinds — a `b.scope.includes("🔖️2x3")` filter (FG3's own pattern) silently matches nothing
for ifc, since `ifc`'s `scope` is just `"…/🗿️artifacts/🏗️ifc"` with no standard suffix. Fixed by matching
against `id`/`summary` instead (which embed the full per-file relPath, including the standard tag) —
confirmed by an iterative before/after check (`fg4-pc-breach-before.txt` → wrong, 33 hits, missing all
grammar/protocol/language-registration entries for ifc/2x3 → `fg4-pc-breach-before2.txt` → correct, 40
hits, ifc/2x3 properly disambiguated from ifc/4).

- **Before** (`fg4-pc-breach-before2.txt`): **129 real repo-wide low-priority breaches, 40 of them
  "stale allowlist entry, file is now genuinely real" breaches touching exactly this wave's 5
  standards**:
  - `POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST`: 3 each × docx/xlsx/pptx/bcf/ifc-2x3 = 15
  - `POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST`: same shape = 15
  - `POLICY_FIXTURE_HONESTY_ALLOWLIST`: 1 each × docx/xlsx/pptx/bcf = 4 (**`stdio/ifc`'s own
    artifact-level entry deliberately NOT counted as "mine" and left untouched** — this rule checks
    fixtures at the ARTIFACT level via a single shared `📚️examples/🎬️demo/🖼️assets/` slot, which is
    ifc/4's own real fixture, not ifc/2x3's own standard-local one; ifc/2x3's own fan-out work never
    touched this shared slot, so any staleness here predates FG4 and is ambiguously shared with
    already-closed ifc/4 — outside this wave's ownership boundary to touch)
  - `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST`: 1 each × docx/xlsx/pptx/bcf/ifc-2x3 = 5
  - `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST`: 1 (ifc/2x3's mutations facet)
  - Total: 15+15+4+5+1 = 40, matching exactly.
- Investigated the json-transfer-ban entry for a FG3-gltf-style false-positive-masking trap before
  removing it: read `🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  directly — its own doc comment reads `` `OpText`/`OpBinary` for `Ifc2x3Mutation`, replacing the prior
  `serde_json::to_string`/`from_str`/\n/// `to_vec`/`from_slice` `` — the four JSON call names are split
  across separate backtick spans on two lines, never forming the literal contiguous substring
  `serde_json::to_vec(` the rule's naive `content.includes(...)` matcher looks for. Confirmed genuinely
  stale (not a masked live hit like FG3's gltf case) — safe to remove.
- Removed all 40 stale entries from `📜️script.ts`'s 5 `POLICY_*_ALLOWLIST` sets, scoped precisely to
  each artifact's own lines (never a global find/replace).
- **After** (`fg4-pc-breach-after.txt`): **89 breaches repo-wide, 0 touching FG4's 5 standards** —
  shrink confirmed for this wave (129→89, exactly the 40 removed), zero growth for anyone else.
  ifc/4's own 8 pre-existing stale entries (3 grammar-parseability, 3 protocol-parseability, 1
  fixture-honesty [`stdio/ifc`, shared], 1 language-registration — all `standards#4`-scoped or
  artifact-shared) left completely untouched, confirmed both by the scoped script's own dedicated
  "ifc/4 diagnostic, must stay untouched" bucket and by their continued absence from the wave's own
  "mine" list.

## 4. Graduation — 4 `ProtocolPack` tuples appended to `STDIO_CONFORMANCE_GRADUATED`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`
(`//#region 🔖️StdioTransition`). Confirmed exact directory-name identifiers by listing disk rather than
guessing: `📜️docx/🏅️standards/🔖️ecma-376`, `📕️xlsx/🏅️standards/🔖️ecma-376`,
`🎞️pptx/🏅️standards/🔖️ecma-376`, `💬️bcf/🏅️standards/🔖️2.1`, `🏗️ifc/🏅️standards/🔖️2x3` (alongside
`🏗️ifc/🏅️standards/🔖️4`, already graduated, untouched). Verified every grammar/protocol file is
non-trivial and real by size (693B–10.8KB) before graduating anything.

**Appended, `ProtocolPack` ONLY, for 4 of the 5 standards:**

```rust
("📜️docx", "🔖️ecma-376", ConformanceFacet::ProtocolPack),
("📕️xlsx", "🔖️ecma-376", ConformanceFacet::ProtocolPack),
("🎞️pptx", "🔖️ecma-376", ConformanceFacet::ProtocolPack),
("💬️bcf", "🔖️2.1", ConformanceFacet::ProtocolPack),
```

**`Grammar` deliberately withheld for all 5 standards — two distinct, both live-confirmed reasons:**

### 4a. docx/xlsx/pptx/bcf — a NEW mechanism gap, discovered live this closer pass

Initial plan was to graduate `Grammar`+`ProtocolPack` for all 4 (they are each the ONLY standard under
their own artifact dir, so none can hit the `pilot_resolve` shared-slot gap). Staged all 8 tuples
(`Grammar`+`ProtocolPack` × 4) and ran
`cargo test -p semio-framework-os-kernel --lib m5_handcrafted -- --nocapture`
**before** finalizing, per this ticket's own "verify live, not assumed" precedent — got **4 real hard
failures**: `grammar did not recognize shipped fixture DSL body` for all 4 (`fg4-closer-m5-scoped.txt`).

Traced why rather than reverting blind. Read each of the 4 standards' own `grammar_conformance_law`
test (`⚙️engine/🦀️component.rs`) — every one shares the exact same shape: it decodes the REAL zip
container the artifact's own `encode_<artifact>` genuinely produces (via `zip::engine::decode_zip`) and
recognizes each individual PART's text separately against the grammar (docx: `[Content_Types].xml`,
`word/document.xml`, …; xlsx: adds `xl/worksheets/sheetN.xml` per-sheet; pptx: adds
`ppt/slides/slideN.xml` per-slide; bcf: `bcf.version`/`markup.bcf`/`*.bcfv`) — **never the whole fixture
body**. This is explicit, documented artifact design: the snapshot TEXT grammar models the syntax of
the individual XML/text PARTS a real OPC/zip package contains, while the artifact's top-level
`🗣️example.dsl.semio` fixture (and `print_dsl()`) hex-dumps the WHOLE outer binary package — matching
the SNAPSHOT BINARY PROTOCOL facet, a different layer entirely (docx's own grammar file doc comment
says this explicitly: "this artifact's `ArtifactDsl::print_dsl` hex-dumps the WHOLE binary OPC package
... UNLIKE a binary-native pilot's `grammar_conformance_law`").

Meanwhile this framework file's own `m5_handcrafted_grammar_conformance`
(`check_grammar_recognizes`/`M5HandcraftedGrammar` region) feeds the artifact's WHOLE
`.dsl.semio` fixture body directly to the grammar's `Recognizer` — structurally correct for every
text-native artifact graduated so far (gltf/pdf/ply/svg/md/xml/csv/json/…), but categorically cannot
pass for an OPC-container artifact's grammar facet, by the artifact's own honest design. **Not a content
shortfall** — each standard's own `grammar_conformance_law` (56/49/58/27 tests total per standard, 0
failed, confirmed both by this closer's own run and `p2-fg4-verify-report.md`) is the real, trustworthy,
independent proof the grammar is correct. It is a **harness-assumption gap**
(`check_grammar_recognizes` has no container-vs-part awareness) — out of a closer's append-only mandate
to fix. Sanity-checked the theory against `zip/2.0` itself (graduated for `Grammar` since the P2-PC
pilot wave, still passing) — zip does NOT hit this, because zip's own snapshot grammar models zip's OWN
text-recognizable content directly (it has no nested container), not a nested container's parts.

Reverted the 4 `Grammar` tuples, kept the 4 `ProtocolPack` tuples, re-ran
`cargo test -p semio-framework-os-kernel --lib m5_handcrafted -- --nocapture`: **Grammar — 59 facets,
59 checked, 27 stdio-exempt soft, 4 hard failures (see §5, none are FG4's); Protocol — 118 facets, 44
checked, 74 soft-skipped, 10 stdio-exempt-or-known-gap soft, 0 hard failures**
(`fg4-closer-m5-scoped2.txt`). Confirms the 4 `ProtocolPack` graduations are safe and the 4 `Grammar`
withholdings are correct.

### 4b. ifc/2x3 — the SAME `pilot_resolve` single-fixture-slot-per-artifact gap gif/89a and pdf/1.7 hit

Independently re-confirmed live for ifc rather than assumed. `find_example_semio`'s own `artifact_rel`
(`PilotResolve` region) is computed from `components[..=artifacts_idx + 1]` — the
`✏️s/…/🗿️artifacts/🏗️ifc` directory, standard name dropped entirely — so ifc/4 (already graduated,
since P2-PC/FG1) and ifc/2x3 share exactly ONE artifact-level `📚️examples/🎬️demo/🖼️assets/` fixture
slot. Read both standards' own fixtures directly:

- Shared slot (`🏗️ifc/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`): `semio stdio.ifc.dsl v1` +
  `FILE_SCHEMA(('IFC4'))` — ifc/4's own real fixture, matching ifc/4's grammar's
  `envelope-mark = "stdio.ifc"`.
- ifc/2x3's OWN real fixture (`🏗️ifc/🏅️standards/🔖️2x3/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`):
  `semio stdio.ifc.2x3.dsl v1` + `FILE_SCHEMA(('IFC2X3'))` — matching ifc/2x3's own grammar's
  `envelope-mark = "stdio.ifc.2x3"` requirement (confirmed by direct read of
  `🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`).

ifc/2x3's own fixture sits at a per-standard location `find_example_semio` never looks at. Graduating
ifc/2x3's `Grammar`/`ProtocolPack` would therefore hard-fail
`m5_handcrafted_grammar_conformance`/`m5_handcrafted_protocol_conformance` for real against the
mismatched IFC4-schema shared fixture — not an ifc/2x3 content shortfall (its own
`⚙️engine::tests::conformance_laws::*` — 82/0 combined with ifc/4 in the stdio crate, both per this
closer's own run and `p2-fg4-verify-report.md` — pass for real), but the identical `pilot_resolve`
mechanism gap FG2/FG3 already documented and declined to fix. **This is now the THIRD standard to hit
this exact wall** (gif/89a, pdf/1.7, ifc/2x3) — worth prioritizing the real per-standard-aware resolver
fix soon. Never graduated ifc/2x3's tuples at all (no staged-then-reverted step needed here, unlike
4a — the shared-slot mismatch was confirmed by direct fixture inspection before staging anything).

**`ProtocolSpr` withheld for all 5** — `find … -iname "*.spr.semio"` under all 5 artifact trees
returned zero hits; none of the 5 standards shipped a real `.spr.semio` fixture this wave (every
fan-out report explicitly deferred it as optional/non-blocking) — same "no graduation theater" rule
every prior wave already established.

A precise, two-part doc comment was added directly above/around the appended tuples explaining both the
4 `ProtocolPack` graduations and the two distinct reasons `Grammar` is withheld — see the file itself,
`//#region 🔖️StdioTransition`.

## 5. Framework m5 harness — `cargo test -p semio-framework-os-kernel --lib`

**Scoped run** (`fg4-closer-m5-scoped2.txt`, after the final graduation-list edit):

- Grammar: **59 facet(s) found, 59 checked, 0 soft-skipped, 27 stdio-exempt soft failure(s), 4 hard
  failure(s)**. The 4 hard failures: `🏗️fem::◻2d::🔖️1`, `📕️norm::📘️en1992::🔖️1`, `🕸️dag::🕸️dag::🔖️1`
  (the SAME 3 pre-existing non-stdio pilots FG1/FG2/FG3's own closers already found red — confirmed by
  name) **plus ONE new regression, `🗄️stdio::🖊️dwg::🔖️ac1018`** (see below — confirmed unrelated to
  FG4). docx/xlsx/pptx/bcf/ifc-2x3 all correctly appear among the 27 soft failures (expected, §4).
- Protocol: **118 facet(s) found, 44 checked, 74 soft-skipped, 10 stdio-exempt-or-known-gap soft
  failure(s), 0 hard failure(s)**. Zero soft or hard failures for any of the 4 newly-graduated
  `ProtocolPack` facets — all 4 pass silently for real.

**dwg/ac1018 investigated, confirmed NOT an FG4 regression**: `git status --porcelain -- "✏️s/…/🖊️dwg/"`
shows 6 files under `dwg/ac1018` currently staged (`M `/`A `) — its own `⚙️engine`, `📸️snapshot`,
`🧬️mutations`, grammar file, and both example fixtures — live, in-progress edits from a different
concurrent session, not this closer or any of FG4's own 5 agents (none of which ever touched dwg).
Confirmed via a baseline A/B: temporarily commented out this closer's own 4 `ProtocolPack` tuples and
re-ran `m5_handcrafted_grammar` alone — dwg/ac1018 was ALREADY hard-failing in that baseline
(`fg4-closer-m5-baseline.txt`), proving it predates and is independent of any of this closer's own
edits. dwg is FG2's own already-closed standard, out of this wave's ownership boundary — flagged as a
follow-up, not fixed.

**Full-crate run** (`fg4-closer-m5-full.txt`): **796 passed, 2 failed** — the same 2 test functions
(`m5_handcrafted_grammar_conformance`, `m5_production_coverage`) failing for the same 4 artifacts (3
pre-existing non-stdio pilots + the unrelated dwg/ac1018 regression), confirming zero regression
attributable to this closer's own edits.

## 6. `git check-ignore -v`

`git status --porcelain` scoped to all 5 FG4 artifact trees shows **zero new (`??`) paths** — each
fan-out agent's own fixtures were already tracked additions by the time this closer ran (confirmed via
`git status --porcelain -- "✏️s/…/🗿️artifacts/<artifact>/"` per standard, all empty for `??`). Nothing
new to check-ignore this wave. Confirmed `🏗️ifc/🏅️standards/🔖️4/` shows zero `git status` changes —
ifc/4 completely untouched by this closer.

## 7. Final re-verification

Re-ran `cargo test -p semio-s-plugin-stdio --lib` one more time after all `📜️script.ts`/
`🧪️fixture-sweep` edits were in place: **1870 passed, 0 failed, 3 ignored** — clean, confirms the
framework- and tooling-level edits in this closer pass caused zero Rust-side regression.

## Ownership-boundary self-check

- `git diff --stat` on `📜️script.ts` shows a large diff (751 insertions), but grepping the diff for
  the 5 wave-specific allowlist keys confirms **exactly the intended 40 stale-line removals** are this
  closer's own edit — the remaining ~700 lines of diff are pre-existing, unrelated, uncommitted work
  from a different concurrent ticket (`26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`'s own
  `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` addition), ambient shared-tree state per this ticket's own
  "this repo NEVER git-commits" rule — not touched or introduced by this closer.
- `git diff --stat` on `🧪️fixture-sweep/🦀️component.rs` shows **0 removed lines, 67 added** — a pure
  append, exactly this closer's own graduation-block edit, no unrelated churn.
- No artifact-owned `.rs`/`.grammar.semio`/`.protocol.semio` file was touched by this closer.
  `📦️glue.rs` was not touched. `🏗️ifc/🏅️standards/🔖️4/**` was not touched (confirmed via `git status`).

## Program tally after FG4

**All 32 official stdio standards now have real grammar/protocol files, real fixtures, and
conformance-law tests.** `Grammar`+`ProtocolPack` fully graduated for 27 (pre-FG4 baseline).
`ProtocolPack`-only newly graduated this wave for docx/ecma-376, xlsx/ecma-376, pptx/ecma-376, bcf/2.1
(4 standards) — their `Grammar` facet is real-but-ungraduated due to the NEW container-vs-part harness
gap (§4a). ifc/2x3 is real-but-fully-ungraduated due to the pre-existing `pilot_resolve` shared-slot gap
(§4b), alongside gif/89a and pdf/1.7 (same gap, 3 standards total now). `ProtocolSpr` remains graduated
for only csv/txt (unchanged this wave). **Zero standards remain for future FG-waves — this was the
last fan-out wave per the plan's own FG4 roster.**

## Known follow-up, not fixed this wave

1. **NEW this wave**: `m5_handcrafted_grammar_conformance`'s `check_grammar_recognizes` has no
   container-vs-part awareness for OPC/zip-based artifacts (§4a) — affects docx/xlsx/pptx/bcf and any
   future OPC-family standard. Real fix: either a sibling check that decodes the real zip container and
   recognizes each part separately (same shape every affected standard's own `grammar_conformance_law`
   already uses), or teaching `check_grammar_recognizes` itself to detect and handle the container case.
   Out of a closer's append-only mandate for this file.
2. **ifc/2x3's `Grammar`/`ProtocolPack` graduation is blocked by the `pilot_resolve`
   single-fixture-slot-per-artifact mechanism gap** (§4b) — now the THIRD standard to hit this exact
   wall (gif/89a, pdf/1.7, ifc/2x3). The real fix is a per-standard-aware fixture resolver in
   `🧪️fixture-sweep/🦀️component.rs` (teach `pilot_resolve` to prefer a fixture slug whose name matches
   the facet's own standard id when more than one candidate location exists), out of a closer's
   append-only mandate. Worth prioritizing now that a third case has landed.
3. **NEW this wave, unrelated to FG4's own content**: `🗄️stdio::🖊️dwg::🔖️ac1018` is currently
   hard-failing `m5_handcrafted_grammar_conformance`/`m5_production_coverage` due to live, in-progress
   concurrent-session edits to its own grammar/fixture/engine files (§5) — not an FG4 regression, dwg
   already graduated in FG2 and is out of this wave's ownership boundary. Flagged for whichever session
   is mid-edit on it, or for a future closer once that work lands.
4. `stdio/ifc`'s artifact-level `POLICY_FIXTURE_HONESTY_ALLOWLIST` staleness (§3) predates FG4 (ifc/4's
   own shared fixture, not ifc/2x3's) — left untouched, ambiguous ownership (touches ifc/4).
5. gltf's `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` entry (FG3's own finding, masking a live
   false-positive substring match inside a doc comment) and FG1's own 7 standards remaining in all 5
   `POLICY_*_ALLOWLIST` sets (FG2's own finding) are both unchanged — still out of this wave's scope.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs` — 4-tuple
  `ProtocolPack` graduation append + two explanatory doc-comment blocks (§4). Only edit in this file,
  pure append (0 lines removed).
- `📜️script.ts` — 40 stale entries removed across `POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST` (15),
  `POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST` (15), `POLICY_FIXTURE_HONESTY_ALLOWLIST` (4 — `stdio/ifc`
  deliberately kept, §3), `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST` (5),
  `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` (1). No other part of this file touched by this closer
  (the large remaining diff is pre-existing, unrelated ambient churn, §"Ownership-boundary self-check").
- `.🦑️repo/🎫️tickets/…/STATUS.md` — appended FG4 closed-wave entry.
- `.🦑️repo/🎫️tickets/…/generators/policy_pc_breach_check_fg4.ts` (new, kept — ticket scratch script,
  §3, retargeted copy of FG2's/FG3's own, with the `id`/`summary`-based ifc/2x3-vs-4 disambiguation fix).
- `.🦑️repo/🎫️tickets/…/fg4-closer-full-crate-1.txt`, `-2.txt`, `-final.txt`,
  `fg4-closer-policy-run.txt`, `fg4-pc-breach-before.txt`, `-before2.txt`, `-after.txt`,
  `fg4-closer-m5-baseline.txt`, `-scoped.txt`, `-scoped2.txt`, `-full.txt` (new — raw command captures).
- `.🦑️repo/🎫️tickets/…/p2-fg4-closer-report.md` (new — this file).

No `glue.rs`, and no artifact-owned `.rs`/`.grammar.semio`/`.protocol.semio` file was touched by this
closer. **This was the final FG-wave — Phase 2's real grammar/protocol/binary-codec program now covers
all 32 official stdio standards.** Ticket left open for the orchestrator's own final PW/G2 gate summary.
