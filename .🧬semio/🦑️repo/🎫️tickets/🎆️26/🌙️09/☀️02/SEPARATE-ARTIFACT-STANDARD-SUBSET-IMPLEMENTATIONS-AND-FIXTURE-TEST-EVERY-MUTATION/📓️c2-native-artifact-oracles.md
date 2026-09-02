# C2 — Native-Artifact Oracles: The Mechanism, The Survey, The Recommendation

## 0. Headline

| id | before (my fresh baseline) | after |
| --- | --- | --- |
| `missing-external-oracle` | 1182 | **1182 (unchanged, honestly)** |
| `no-oracle-covers-mutation` | 24 | **0** |
| `reimplementation-registered-as-third-party` | 2 | 2 (unchanged — see §5) |
| `oracle-capability-mismatch` | 0 | 0 |
| `unknown-oracle` | 0 | 0 |
| `fixture-generated-by-non-qualifying-oracle` | 0 | 0 |
| **TOTAL breach count** | **2058** | **2034** |

Both numbers are from a live foreground `bun ./📜️script.ts test contract`, read back from
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` (run before touching anything, and again after).
My baseline matched the brief's stated 1182/24/2 exactly, confirming I picked up the state left by
shard B5/A10 with nothing lost in between.

**`missing-external-oracle` did not move, and that is the correct, honest answer to this shard's
mandate** — see §1 for why, and §4 for why I did not force it to move.

## 1. The mechanism (three sentences, as asked)

`oracleRequirementBreaches` (`🟦️.ts:4699`) fires unconditionally for every manifest mutation whose
`oracleRequirements` name a capability with no registered oracle of a `QUALIFYING_ORACLE_KIND`
(`third-party-library` / `third-party-cli` / `standards-reference-tool`, `🟦️.ts:2746`) —
`cross-semio-implementation` is explicitly a `SUPPLEMENTAL_ORACLE_KIND` (`🟦️.ts:2755`) and can never
discharge it, by design ("both halves read the same specification, so a misreading of it produces two
agreeing wrong answers"). **`din16798` — the exemplar the brief pointed me at — does NOT currently
discharge this rule**: I confirmed live that it carries 62 `missing-external-oracle` breaches today,
one per kind, exactly like every other native artifact; the brief's premise that it "legitimately
passes" was not true of `missing-external-oracle` and I want that stated plainly rather than quietly
worked around. What actually *is* true, and is the real difference between `din16798` and the 24
`no-oracle-covers-mutation` cases, is a second, independent rule: `noOracleMisuseBreaches`
(`🟦️.ts:5074`) fires only when an owner's `noOracleDecisions` entry *itself* still claims a capability
that a real manifest requires — `din16798` has **zero** `noOracleDecisions` (removed by an earlier
ticket, `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`, once its Python second implementation existed), so it
was never eligible to trip that second rule, while the 24 I found still carried a decision saying "no
oracle needed, this is handled" for a capability a manifest now requires — a claim the rule exists
specifically to prohibit, because it makes a real gap look closed.

**In short: `missing-external-oracle` and `no-oracle-covers-mutation` are independent checks that
happen to look related.** Writing a second implementation (`din16798`'s pattern) satisfies this
ticket's own broader law #2 (every mutation fixture-tested against a second producer) and, as a
side effect, removes the *reason* to keep a no-oracle decision around — but it does **not**, under
the code as written today, ever clear `missing-external-oracle` itself. Nothing does, for a format no
third party implements, without a rule change (§6).

## 2. What I actually did, concretely

### 2a. Closed `no-oracle-covers-mutation`, 24 → 0 (real, verified, zero risk to `missing-external-oracle`)

For every one of the 24 flagged decisions, `covered.length > 0` in `noOracleMisuseBreaches` means the
decision's `capabilities` list overlaps a capability a real `mutationManifests` entry already
requires — which, by definition, is *already* counted in `missing-external-oracle` (a manifest with
an unmet `oracleRequirements` entry breaches regardless of what any decision says). Removing the
capability from the decision therefore removes a **false "this is handled" signal** without touching
the real, already-counted gap. This is the exact fix A10 already established and verified for
`mathematical`/`sequence`/`draw` (3 cases, in `📓️a10-oracle-honesty.md`); I applied the same fix to
the remaining 21 the codebase had accumulated since (mostly newly exposed by B5's manifest-writing
wave), plus re-confirmed the mechanism against a live example rather than trusting the precedent
blind.

Script: `$TICKET/🩹️c2-narrow-no-oracle.py` (the exact driver that ran, kept here per house rules;
`$TICKET/🩹️c2-fix-ticket-ref.py` is the small follow-up that pointed each note's `$TICKET` placeholder
at this ticket's real path once it was written). For each decision it removed exactly the one flagged capability (never cleared an unrelated sibling
capability — `frozen-hound-pcm16` kept its `wave-audio` entry and lost only `wav-riff-pcm-mutate`)
and appended a dated note to the rationale explaining the narrowing and pointing back here, leaving
every word of the original investigation untouched. Verified: all 22 files parse as valid JSON: 24
decisions changed across 22 files (three files carry more than one decision:
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json` and
`🧰️framework/🛍️products/💻️os/🎚️config/🧪️oracle/🔣️.json` each carry several). Confirmed with the
live gate: `no-oracle-covers-mutation` 24 → 0, `missing-external-oracle` unchanged at 1182,
`oracle-capability-mismatch`/`unknown-oracle`/`fixture-generated-by-non-qualifying-oracle` all stayed
at 0, total breach count fell by exactly 24 (2058 → 2034).

The 24, and what each one now honestly is:

| decision | capability removed | what remains true |
| --- | --- | --- |
| `flow-widget-graph-mutation-semantics` | `flow-1-mutate` | debt, no 2nd impl yet — §3 |
| `vcs-1-checkpoint-mutation-semantics` | `vcs-1-mutate` | debt, no 2nd impl yet |
| `present-figure-deck-mutation-semantics` | `present-1-mutate` | debt, no 2nd impl yet |
| `shooting-render-scene-mutation-semantics` | `shooting-1-mutate` | debt, no 2nd impl yet — §3 |
| `playground-mutation-semantics` | `playground-1-mutate` | debt, no 2nd impl yet |
| `process3d-mutation-semantics` | `process3d-1-mutate` | debt, no 2nd impl yet |
| `wires-1-argument-board-mutation-semantics` | `wires-1-mutate` | debt, no 2nd impl yet |
| `layout-mutation-semantics` | `layout-1-mutate` | debt, no 2nd impl yet — §3 |
| `imperative-1-nested-step-list-mutation-semantics` | `imperative-1-mutate` | debt, no 2nd impl yet |
| `remodel-mutation-semantics` | `remodel-1-mutate` | debt, no 2nd impl yet — §3 |
| `energy-model-mutation-semantics` | `energy-model-1-mutate` | debt, no 2nd impl yet |
| `dag-1-port-directed-graph-mutation-semantics` | `dag-1-mutate` | debt, no 2nd impl yet |
| `frozen-hound-pcm16` | `wav-riff-pcm-mutate` | real format (WAV/RIFF); needs a real codec-level oracle survey, not a native-artifact question |
| `jpg-jfif-1-01-baseline-conformance-class-semantics` | `jpg-jfif-1-01-baseline-mutate` | real format, already well-argued: real JPEG crates exist but expose no frame-header/DHT/DAC API — see the file's own rationale |
| `dwg-ac1018-proprietary-container` | `dwg-ac1018-mutate` | real but proprietary binary format, no open reader |
| `dwg-ac1024-proprietary-container` | `dwg-ac1024-mutate` | same |
| `tiff-6-0-baseline-conformance-class-semantics` | `tiff-6-0-baseline-mutate` | real format, same shape as jpg's argument |
| `s-home-mutation-semantics` | `s-home-1-mutate` | semio-native, debt |
| `s-space-index-mutation-semantics` | `s-space-1-mutate` | semio-native, debt |
| `os-config-opening-preferences-mutation-semantics` | `os-config-opening-1-mutate` | repo-owned OS behaviour, genuinely no third party can ever exist (session/preference semantics) |
| `os-config-merge-policy-mutation-semantics` | `os-config-merge-policy-1-mutate` | same |
| `os-config-identity-mutation-semantics` | `os-config-identity-1-mutate` | same |
| `txt-utf-8-line-structure` | `txt-utf-8-mutate` | plain text; "third party" is not a coherent concept for line-structure edits |
| `raw-buffer-no-format` | `binary-raw-mutate` | by definition no format, no oracle possible |

None of these were relabelled or hidden — every one still shows up, correctly, in
`missing-external-oracle`. The only thing that changed is that a false "no-oracle decision has this
covered" signal is gone.

### 2b. What I deliberately did NOT do

I did not write any new second implementations this session (see §3 for the size of that work and
why I'm reporting it rather than attempting a partial, unverified slice of it), and I did not touch
`reimplementation-registered-as-third-party`'s 2 remaining `ifc` entries — both my brief and
`📓️a10-oracle-honesty.md` are explicit that the sanctioned fix there is a larger `judgedByProbes`
schema retrofit needing its own qualification evidence, the same shape as the PNG precedent, not a
copy-paste (§5).

## 3. Per-artifact survey — the ranked list

I checked, for every owner behind the 1182, whether a second independent implementation (the
`din16798` pattern: a `🐍️.py` or `🟦️.ts` file beside the Rust adapter in the same `🧪️tests/mutate-*`
case directory, differential-mode, reading committed fixtures) already exists, and whether its
feature file documents an actual third-party search (not an assumption). This is a full census, not a
sample — every owner behind every one of the 1182 breaches was checked.

**Already has a real second implementation, third-party search documented in its own feature file —
correctly, honestly open, nothing further for this shard to do without a rule change (§6):**

| artifact | kinds | 2nd impl | third-party search documented |
| --- | --- | --- | --- |
| `s.stdio.semio` (all 13 v1 subsets: kit 15, presentation 14, animation 12, flow 12, image 12, graph 11, model 10, audio 9, object 9, table 8, value 8, video 8, text 7) | 135 | yes (mixed 🐍️/🟦️ across subsets, e.g. `mesh`'s TS impl documented in `📓️a10-oracle-honesty.md`) | yes |
| `s.norm.din16798` | 62 | yes | yes — PyPI checked, no `din16798`/`eurocode`/`vdi3805`/`iso16757` distribution; nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`) implement formulae, not documents |
| `s.norm.en1998` | 49 | yes | yes, same survey text as din16798 (shared Eurocode-family finding) |
| `s.block.5d` | 41 | yes | yes — semio-native kind definition |
| `s.block.3d` | 37 | yes | yes |
| `s.norm.en1992` | 35 | yes | yes |
| `s.puzzle.3d` | 35 | yes | yes |
| `s.norm.en1991` | 32 | yes | yes |
| `s.puzzle.5d` | 28 | yes | yes |
| `s.norm.en1999` | 26 | yes | yes |
| `s.puzzle.2d` | 26 | yes | yes |
| `s.block.2d` | 26 | yes | yes |
| `s.norm.din4108`, `en1994`, `en1996`, `en1997` | 22 each (88) | yes | yes |
| `s.norm.iso16757` | 21 | yes | yes |
| `s.cad.cad` | 20 | yes | yes |
| `s.norm.en1995` | 20 | yes | yes |
| `s.norm.vdi3805` | 19 | yes | yes |
| `s.lowpoly.lowpoly` | 17 | yes | yes |
| `s.norm.en1993` | 17 | yes | yes |
| `s.procedural.procedural2d`, `procedural3d` | 14 each (28) | yes | yes |
| `s.norm.din18599` | 13 | yes | yes |
| `s.gis.gismap` | 12 | yes | yes |
| `s.raster.raster` | 12 | yes | yes |
| `s.forms.forms` | 10 | yes | yes |
| `s.norm.en1990` | 10 | yes | yes |
| `s.procedural.assembly` | 9 | yes | yes |
| `s.playbook.playbook` | 9 | yes | yes |
| `s.trinity.jack` | 8 | yes | yes |
| `s.trinity.rewrite` | 7 | yes | yes |
| `s.writer.writer` | 4 | yes | yes |
| `s.sourcing.curate` | 3 | yes | yes |
| `s.gis.gisterrain` | 2 | yes | yes |

Sum: **861 of the 1182** (≈73%) are artifacts where the honest, established, ticket-sanctioned
pattern is already fully in place. There is nothing to "close" here without §6.

**Semio-native, genuinely no second implementation yet — a real, documented debt, not a hidden one:**

| artifact | kinds | status |
| --- | --- | --- |
| `s.remodel.remodel` | 35 | Feature file has a self-written, unusually explicit "THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT" note naming the exact blocker: its specification vectors are inline in the `Examples` table, not declared `asset://` fixtures, so a Python reference cannot resolve them at all. Names the recipe to follow (`mutate-cad-1`, `mutate-lowpoly-1`). |
| `s.shooting.shooting` | 31 | Same debt note. Its no-oracle decision's third-party survey is real and argued (glTF 2.0/USD/Collada checked and declined — none models a SHOT, and 11 of 31 kinds address one) but the second-implementation half was never done. |
| `s.layout.layout` | 25 | Same debt note, plus a second blocker: the committed snapshot's grammar doesn't match what `identity-round-trip` would need (no `layers` block). |
| `s.dag.dag` | 14 | No debt note, no second implementation. |
| `s.process.process3d` | 16 | No second implementation. |
| `s.flow.flow` | 10 | Debt note present (see the `no-oracle-covers-mutation` fix above — its own rationale independently proved every one of this repo's 5 export serializers is non-functional for the format, an orthogonal blocker). |
| `s.reasoning.wires` | 10 | No second implementation. |
| `s.animate.present` | 9 | No second implementation. |
| `s.vcs.vcs` | 6 | No second implementation. |
| `s.imperative.imperative` | 4 | No second implementation. |
| `s.space.space` | 4 | No second implementation. |
| `s.space.home` | 1 | No second implementation. |
| `s.demonstrator.playground` | 1 | No second implementation. |
| `s.energy.model` | 1 | No second implementation. |

Sum: **168** genuinely open, no-second-implementation-yet native artifacts (35+31+25+14+16+10+10+9+6+4+4+1+1+1).
`s.mathematical.mathematical` (9) and `s.sequence.sequence` (4) add 13 more, but these are **already
correctly disposed** by A10 (partially discharged by real CSV oracles, partially genuinely gapped —
see `📓️a10-oracle-honesty.md`, unchanged by me).

**Real (non-native) interchange formats where a genuine third party exists but was reclassified away
from qualifying by A10's `reimplementation-registered-as-third-party` fix** (`svg` 19, `pptx` 16,
`ifc/2x3` 16, `xlsx` 14, `docx` 14, `xml` 8, `zip` 7, plus the smaller `jpg`/`tiff`/`dwg`/`txt`/`wav`/
`binary` cases in §2a's table) — 84 kinds. These are a **different problem** from this shard's native-
artifact remit: the fix there is not "write a second implementation" (a third party already sits in
the crate) but "write a real semantic-comparison oracle that actually uses the third-party crate's
own parser to judge the result, instead of the crate being present only for codec while a hand-written
dispatch predicts the answer" — squarely the debt A10's investigation already named and left open,
not something I duplicated.

## 4. Why I did not attempt to write the 168 missing second implementations

Writing one of these (`din16798` at 62 kinds, `en1998` at 49) is a substantial, careful piece of work:
a from-scratch second-language implementation of the mutation vocabulary from written specification
documents, one committed `(before, mutation, after, outcome)` fixture quintet per kind, wired into a
`mode-differential` Cucumber scenario, cross-checked kind by kind against the Rust for near-collision
field names — exactly the kind of work this ticket's own house rules say must be done thoroughly, not
partially ("You MUST NOT be pragmatic... You MUST be extremely thorough", and "You MUST NOT say a
feature is working when you didn't confirm runtime behaviour"). Three of the fourteen open native
artifacts (`remodel`, `shooting`, `layout`) additionally need a **prerequisite** fix first — converting
their inline `Examples` tables to declared `asset://` fixtures — before a Python reference could even
resolve the committed material, which is itself real work, not a one-line change, and risks breaking
the currently-passing Rust-only assertions in that same feature if done carelessly.

168 kinds across 14 artifacts, done to that standard, is multiple shards' worth of work (compare:
B5 alone spent a full shard writing 102 *manifests* from *already-existing* leaf descriptors, and
A10 spent a full shard on 20 files' oracle classification). Attempting a rushed, partial slice of it —
say, one artifact done sloppily — would produce exactly what this ticket's second law exists to
prevent: an implementation nobody has verified agrees with production, wearing the appearance of
evidence. **I chose the smaller, mechanically verifiable, unambiguously-correct fix (§2a) over a
larger, higher-risk, unverifiable one**, and I am reporting the remaining 168 as an honest, itemized,
actionable backlog rather than a number I quietly shaved.

Recommended next step for whoever picks this up: `remodel`, `shooting`, `layout` first (their own
feature files already state the exact blocker and the exact recipe to copy), then `dag`/`process3d`/
`flow`/`wires` (no blocker beyond doing the work), then the five single-digit-kind artifacts
(`present`, `vcs`, `imperative`, `space`, `home`, `playground`, `energy-model` — cheap, ~1-9 kinds
each, good first targets for a small shard).

## 5. `reimplementation-registered-as-third-party` (2, `ifc`) — status quo, correctly

Both remaining flags (`ifc/2x3/✳️any`, `ifc/4/✳️any`) are, per A10's investigation, **verified
legitimate** third-party oracles (`ifcopenshell`, a real independent Part-21 producer) caught by the
detector's file-level rather than entry-level granularity, because the same `🦀️oracle.rs` also hosts
a correctly-reclassified `ruststep-*` predicting dispatch. The sanctioned fix (`judgedByProbes`:
registered `probes` + `comparisonPipelines` with a `qualified` status, the same shape as the PNG
precedent in `.../26/08/27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/`) needs its own measured
qualification evidence and is a schema retrofit, not a copy-paste — I did not attempt it, consistent
with both my brief and A10's own conclusion. Left at 2, documented, not touched.

## 6. Is the framework rule itself too strict for native artifacts? Yes — and here is the precise, evidenced case for changing it, not working around it

**The evidence.** 861 mutations across 34 artifacts already carry the exact discharge this repository
has, independently and repeatedly, decided is the right answer for a format no third party can ever
implement: a second, from-scratch implementation in a different language, written from this
repository's own specification documents rather than from the subject's source, run in differential
mode against the same committed fixture bytes. This pattern predates this ticket (`din16798`'s
`26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`) and has been reapplied by at least three different tickets
since. It is not a one-off judgment call; it is the repository's settled position on how to get real
evidence for a format that, by construction, no vendor will ever publish a reader for. And yet, today,
every one of those 861 mutations is indistinguishable in `missing-external-oracle` from a mutation
with *no* evidence at all — a completely untested `no-mutation` catalog artifact and `din16798`'s
62-kind, differentially-cross-checked, fixture-pinned vocabulary report the identical breach. That is
a real loss of signal, and it is the kind of loss this ticket's own second law says must not be
tolerated.

**The proposed change**, precisely: add a new `QUALIFYING_ORACLE_KIND` —
`"verified-native-second-implementation"` — accepted by `isQualifyingOracleKind` **only** when the
oracle entry additionally carries a structured `noThirdPartySurvey` field (not free rationale text)
recording: the ecosystems searched, the candidate packages considered and the specific structural
reason each was declined (mirroring what `din16798`/`en1998`/`block`/`puzzle`'s prose already argues,
made machine-checkable instead of prose-only), a `differentLanguage` assertion checked against the
subject's own implementation language, and a requirement that the oracle's `capabilities` cover
**100%** of the owning manifest's kinds (a partial second implementation must stay
`cross-semio-implementation`, discharging nothing). This is deliberately narrower than simply
"promoting" `cross-semio-implementation` wholesale — it would not touch the 20 codec-only Rust
reimplementations A10 correctly reclassified *down* to `cross-semio-implementation` in the same
session (they read the subject's own logic, not an independent specification, which is exactly what
the new kind must keep excluding).

**Why I am not implementing this myself.** `oracleRequirementBreaches` and `isQualifyingOracleKind`
are the ground truth every other shard on this ticket is measuring against, live, right now — B5 just
finished writing 102 manifests specifically to make `missing-external-oracle` visible, A10 spent a
shard getting the 13/24/2 baseline honest, and C1 is concurrently auditing every `🪆️subsets/🔣️.json`.
Changing what counts as qualifying, in the shared judge file, mid-flight, would silently change the
denominator every other shard's numbers are measured against, without their knowledge — precisely the
"invisible" failure mode this ticket exists to eliminate, just moved one level up. This is a decision
for whoever owns the ticket as a whole, made once, communicated to every concurrent shard, not a
unilateral edit from inside one shard's turn.

## 7. Files touched

- 22 `🧪️oracle/🔣️.json` files, 24 `noOracleDecisions` entries narrowed (capability claim removed,
  dated note appended, rationale otherwise untouched) — listed in full in §2a's table, file paths in
  the script below.
- Scratch scripts, kept in this ticket folder: `🩹️c2-narrow-no-oracle.py` (the narrowing driver),
  `🩹️c2-fix-ticket-ref.py` (pointed each note's `$TICKET` placeholder at this ticket's real path).
- No manifests, no oracle registrations, no fixtures, no `🪆️subsets/🔣️.json` were added or changed —
  `no-oracle-covers-mutation` was the only rule I had a safe, verified, always-correct mechanical fix
  for; everything else in my remit needs either real engineering (§3/§4) or a policy decision outside
  my authority (§6).

## 8. Final answer

- Before: `missing-external-oracle` 1182, `no-oracle-covers-mutation` 24,
  `reimplementation-registered-as-third-party` 2, `oracle-capability-mismatch` 0, `unknown-oracle` 0,
  `fixture-generated-by-non-qualifying-oracle` 0, **total 2058**.
- After: `missing-external-oracle` **1182 (unchanged, honestly)**, `no-oracle-covers-mutation`
  **0**, `reimplementation-registered-as-third-party` **2 (unchanged, documented)**,
  `oracle-capability-mismatch` 0, `unknown-oracle` 0, `fixture-generated-by-non-qualifying-oracle` 0,
  **total 2034** (−24).
- Mechanism in three sentences: `missing-external-oracle` fires per-mutation whenever no
  `third-party-library`/`third-party-cli`/`standards-reference-tool` oracle backs its capability, and
  `cross-semio-implementation` (a second implementation written inside this repo) is explicitly
  disqualified from ever satisfying it; `din16798` does **not** escape this — it carries 62 such
  breaches today, verified live, contrary to the brief's premise — the only thing that made it *look*
  different is that it has no `noOracleDecisions` entry left to also trip the separate
  `no-oracle-covers-mutation` check, while the 24 I fixed still did; narrowing those 24 decisions'
  claimed capabilities closes that second, independent rule honestly without ever touching (or hiding)
  the real, larger gap.
- Recommending a rule change: **yes** — a new, narrowly-gated `verified-native-second-implementation`
  qualifying kind (§6), evidenced by 861 already-conforming mutations across 34 artifacts, proposed
  precisely rather than implemented, because implementing it would silently move the shared ground
  truth every concurrent shard on this ticket is measuring against.
- This file: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️c2-native-artifact-oracles.md`.
