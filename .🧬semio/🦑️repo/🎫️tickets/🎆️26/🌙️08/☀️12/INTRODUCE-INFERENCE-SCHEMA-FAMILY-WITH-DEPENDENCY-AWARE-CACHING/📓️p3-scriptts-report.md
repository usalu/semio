# P3 — `📜️script.ts` inference policy cluster report

## What changed (line anchors, current live file)

Single new region inserted between the existing `🔧️PolicyRuleMutationArtifactEngines` and
`🔧️PolicyRuleArtifactSchemas` regions (chosen precisely because it sits outside the peer-owned span the
brief warned off, and after re-grepping the region markers live at write time rather than trusting a
stale line number):

- `//#region 🔧️PolicyRuleInferenceFamily` … `//#endregion 🔧️PolicyRuleInferenceFamily` — currently
  `📜️script.ts:7166`–`7535` (line numbers move as peers land concurrent edits; anchor on the region
  marker text, not the numbers).
- Two one-line registrations elsewhere in the same file:
  - `📜️script.ts:795` — `...policyInferenceFamilyBreaches(this.root),` added inside `VerifyScript.runGate`'s
    `dissolveBreaches` array (never `osBreaches`).
  - `📜️script.ts:12211` — `breaches.push(...policyInferenceFamilyBreaches(repoRoot));` added inside the
    exported `policy` census (`workspace-app-plugin-consistency`), immediately after the existing
    `...policyMutationArtifactEngineBreaches(repoRoot)` line — the mutations cluster's own registration
    site, mirrored exactly.

## Rules added (all `priority: "medium"` or `"low"`, never `"high"`)

1. `POLICY_INFERENCES_FACET = "💡️inferences"`.
2. `policyListInferenceDirs(repoRoot, inferencesRel)` — slug dirs only, reserved set
   `["📚️examples", "💾️binary", "📝️text"]` (coordinator's own repo-wide subtraction check across all 112
   families confirmed this set is exactly sufficient — no undocumented third sibling exists anywhere).
3. `policyFindAllInferencesDirs` / `policyArtifactRootOfInferencesDir` — mirror
   `policyFindAllMutationsDirs` / `policyArtifactRootOfMutationsDir` exactly; the artifact-root derivation
   is reused directly (not duplicated) since its marker logic (`/🏅️standards/`) is generic to any
   `🧬️schema` child, not mutation-specific.
4. `policyInferenceFamilyRootCompletenessBreaches` — 5 `schemaFormats` root leaves +
   `textSpecFilenames` (8) + `binarySpecFilenames` (6), all read from `loadTaxonomy()` — no hardcoded
   list, so a taxonomy change updates the rule for free. `kind: inference-migration/family-root-completeness`.
5. `policyInferenceSlugLeafPresenceBreaches` — `🦀️component.rs` required; `🟦️component.ts` required and
   must be real (not a trivial `export {};`/empty stub, same bar `policyMutationTsMirrorBreaches` holds
   triad leaves to). `kind: inference-migration/slug-leaf-presence`.
6. `policyInferenceImplPresenceBreaches` — accepts EITHER `impl …InferredField<` OR any `pub fn` in the
   slug's `🦀️component.rs`. Docstring cites the binding coordinator ruling verbatim (a merkle dep-chain
   over a flat snapshot costs more than the fold it caches) with the current real ratio. `kind:
   inference-migration/impl-presence`.
7. `policyInferenceEmojiUniquenessBreaches` — emoji uniqueness scoped **within one family tree only**
   (verified this is correct against real cross-family repeats — `🧭topology` on flow/graph/raster/jack,
   `⏱duration` on six media subsets — which must NOT be flagged) + bare-emoji (no U+FE0F) check, citing
   `isEmojiPrefixedSlugDir`'s own docstring in `🔍️discovery/🟦️component.ts` as the convention source.
   `kind: inference-migration/emoji-uniqueness`.
8. `policyInferenceAssemblyCoverageBreaches` — kebab/snake/camel-normalized coverage between
   `💡️inferences/<slug>` dirs and fields of the family-root `<Prefix>Inference` struct, in both
   directions (orphan slugs, uncovered fields). Reuses `policyExtractRustSchemaFields` from the
   artifact-schema cluster (function hoisting makes the forward reference safe) instead of re-deriving a
   Rust field parser, and structurally mirrors `policyMutationDispatchCoverageBreaches`. `kind:
   inference-migration/assembly-coverage`.
9. `policyInferenceStateLeakBreaches` (`POLICY_INFERENCE_STATE = "#[state(inferred)]"`) — flags that
   marker appearing in any `🦀️component.rs` under a `📸️snapshot` facet. `kind:
   inference-migration/state-leak`.
10. `policyInferenceFamilyBreaches` — aggregates all six.

Zero naming collisions verified against the live file (`grep -c` on every new symbol) both before writing
and again after.

## Registration site used and why

Per the brief's trap #1: registered **only** inside `dissolveBreaches` (`📜️script.ts:789-796`), which
filters to `priority === "high"` before throwing — since every rule here is `medium`/`low`, the array
inclusion is present but inert, exactly as instructed. Never touched `osBreaches`
(`📜️script.ts:733-741`), which throws on any breach regardless of priority.

Additionally registered inside the `policy` census export (`📜️script.ts:12211`) — this is not what the
brief's trap literally named, but it is where the mutations cluster this ticket told me to mirror
"exactly" is itself wired in (`policyMutationArtifactEngineBreaches` is absent from `dissolveBreaches`
entirely and lives only in the census export). Registering in both places satisfies the trap's explicit
instruction and gives the cluster a real, countable breach signal via `bun script.ts policy` — the
mechanism this report's counts below come from.

## Coordination note — write-slot wait

Before any edit, `git status --porcelain -- 📜️script.ts` showed `MM`: APA was actively staging/unstaging
a shrink-only breach-count ratchet (`POLICY_APA_RATCHET_CEILINGS`, `policyApaBreaches`,
`policyApaRatchetApply`) live. Verified by content (grepping the actual added symbols in the diff), not
by timing. Held with zero bytes written, polled `git log`/`git status` on the file in the background
until it went fully clean and a new commit (`62152fabcc`, flag 499) landed carrying APA's symbols and
zero of mine, then re-verified live immediately before writing. A concurrent peer subsequently
tightened two docstring numbers in my just-staged region (the `InferredField` ratio, the mesh-completeness
wording) without touching any logic — re-verified the file still parses and breach counts were unchanged
after that landed.

## Verification

### Parse check
```
$ bun --version
1.3.14
$ bun 📜️script.ts --help
unknown command "--help"
usage: bun ./📜️script.ts <os|semio|examples|nx|setup|start|dev|generate|lint|verify|format|test|build|cpp|publish|purge|query|micro-commit|commit> [args…]
```
Exits cleanly with a usage line — proves the file parses (a real syntax break, e.g. the Bun `**/` tokenizer
trap, fails hard here instead). Re-ran after the concurrent docstring edit landed; same clean result.

### `bun 📜️script.ts verify gate`
```
x 769 dependency violations (620 errors, 149 warnings). 8706 modules, 5394 dependencies cruised.
error: bunx dependency-cruiser compose 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config .dependency-cruiser.cjs --output-type err exited with status 108
      at runCmd (…/📦️index.ts:1390:31)
      at runGate (/Users/ueli/Documents/semio/📜️script.ts:713:5)
```
This fails at the **first** check in `runGate` — dependency-cruiser boundaries — before execution ever
reaches the `dissolveBreaches` block my rules are registered in. Confirmed pre-existing and unrelated:
ran the identical command before writing anything (same 769 violations, 8700→8706 modules is ambient
churn from other sessions, not from this edit) and it failed at the exact same step for the exact same
reason. My rules are unreachable from this failure and did not cause it.

### `bun 📜️script.ts policy` (the actual breach-census gate my rules feed)
Three runs, spanning the wait and the write:

| when | total | high | inference-migration total | inference by priority |
|---|---|---|---|---|
| baseline, before any edit | 30,472 (coordinator's last measurement) | 24,632 | 0 | — |
| my own pre-edit baseline run | — | 24,631 across 25 rules | 0 | — |
| immediately after writing | 30,631 | 24,801 across 26 rules | 34 | 33 medium, 1 low |
| ~10 min later (re-run) | 30,625 | 24,801 across 26 rules | 24 | 23 medium, 1 low |

**High-priority delta from my rules: 0**, both runs — `inference-migration/*` never appears in the
high-priority tally at all (26 rule kinds listed, all pre-existing). The ambient total/high drift between
rows is confirmed repo-wide churn (other sessions landing fixes and fan-out work), not my rules — e.g.
the assembly-coverage/completeness breach count itself dropped from 34→24 between runs because a peer
session finished stdio drawing's `📝️text`/`💾️binary` leaves live while I was verifying (checked on disk
before and after — 2/8 leaves present, then 8/8).

**Final captured count: 24 breaches from my rules — 23 medium, 1 low, 0 high**, across 4 of the 5 rule
kinds (`family-root-completeness` currently at 0 after the drawing fix landed; `slug-leaf-presence` 1,
`impl-presence` 4, `emoji-uniqueness` 2, `assembly-coverage` 17). This is comparable in scale to SMO's
"2 highs across their entire ticket" calibration bar the coordinator set, and nowhere near the
"hundreds would mean a rule design error" warning threshold.

## Spot-checks (4, not 3 — all true positives)

1. **`family-root-completeness`** (captured before the drawing fix landed): `✳️drawing/🧬️schema/💡️inferences/📝️text/`
   on disk had only `📖️component.grammar.semio` + `🦀️component.rs` — genuinely missing the other 6 of 8
   `textSpecFilenames`, exactly matching the 6 breaches reported for that dir. True positive.
2. **`slug-leaf-presence`**: `🧩️puzzle/🧊️3d/💡️inferences/🎛flatten/` on disk has only `🦀️component.rs`, no
   `🟦️component.ts` — matches the "no 🟦️component.ts mirror at all" breach exactly. True positive.
3. **`assembly-coverage`** (uncovered field): read `🌍️gis/🏔️gisterrain`'s family-root `🦀️component.rs` —
   `GisTerrainInference` has `position_count: usize` and `bounds: Option<GisTerrainBounds>`; only
   `📦bounds/` exists as a slug dir. `position_count` is genuinely computed inline in the family root
   rather than via a dedicated slug dir, which the file's own docstring convention ("each named inference
   gets its own `<slug>/` child") doesn't fully honor. True positive, and a legitimate architectural
   observation, not a rule bug.
4. **`emoji-uniqueness`** (dup): listed `🧩️puzzle/🧊️3d/💡️inferences/` on disk — both `🎛flat-position/` and
   `🎛flatten/` exist as sibling slug dirs sharing the same leading emoji `🎛` within one family. True
   positive, exactly the defect class the rule exists to catch (distinct from energy's legitimate
   cross-family repeats, which the rule correctly does NOT flag).

## Files touched

- `/Users/ueli/Documents/semio/📜️script.ts` — only file edited, as required (three edits: one ~370-line
  region insertion, two one-line registrations).
- Ticket-folder scratch (created, `.txt`): `scratch-p3-policy-baseline.txt`,
  `scratch-p3-verify-gate-baseline.txt`, `scratch-p3-policy-after.txt`,
  `scratch-p3-verify-gate-after.txt`, `scratch-p3-policy-final.txt`, `scratch-p3-policy-final2.txt`,
  `scratch-p3-policy-final3.txt`.
- Draft (outside ticket folder, in the session scratchpad per harness convention, referenced from
  `📓️status.md`): `inference-cluster-draft.ts` — kept as the pre-image; the live file is now the source
  of truth.

## Concurrent-churn observations

- **APA's ratchet** (`POLICY_APA_RATCHET_CEILINGS` etc.) held the write slot for roughly 10 minutes while
  authoring; verified free by content (their added symbols in `git diff --cached`, not by commit timing)
  before touching the file, per the ticket's own attribution rule.
- **A peer refined two of my docstring numbers** (`InferredField` ratio, mesh-completeness wording)
  immediately after my insertion landed, without touching logic. Accepted as-is per "work simultaneously
  on the same files" — did not revert or re-fight it, only re-verified parse + breach counts afterward.
- **`✳️drawing`'s `📝️text`/`💾️binary` leaves completed live** between my first and second `policy` run,
  dropping my own breach count 34→24 in real time — direct evidence the rules read live disk state
  correctly rather than a cached assumption.
- **Energy's 50 stray non-emoji dirs, present when I first surveyed the tree, were gone by the time I
  wrote the rules** — moved to `🔨️modules/⚡️simulation/⚙️engine/` by the energy team as `📓️status.md`
  anticipated. Confirmed on disk (`🗃entries` is now the only slug dir under energy's `💡️inferences/`).
  This is why the actual footprint (24) came in well under the coordinator's ~50 estimate, which was made
  before that move landed.
- **One `bun script.ts policy` run timed out** at the 2-minute Bash default under heavy concurrent
  cargo/bun load from the other five sessions (documented repo-wide in `📓️status.md`); re-ran in the
  background rather than treating the timeout as a code problem.
- **`verify gate`'s dependency-cruiser step is red** both before and after my edit, for the same
  pre-existing reason (769 violations, unrelated to inference work, never reached by my rules since it
  fails at the very first check in `runGate`). Not mine to fix; reported at both baseline and post-edit
  for an honest apples-to-apples comparison.

## Honest pass/fail

**Pass**, with one caveat. The file parses, the census gate (`bun script.ts policy`) runs my rules
successfully with **zero high-priority breaches added** and a small, plausible, spot-check-verified
medium/low footprint (24, all four true-positive spot-checks confirmed against the real tree). The
caveat: `bun script.ts verify gate` itself cannot be shown green end-to-end because it fails at an
unrelated, pre-existing, earlier step (dependency-cruiser) that exists independent of this work and was
already red before I touched anything — verified identically red before and after with the same error
signature, so this is not a regression I introduced, but it does mean `verify gate`'s exit code is not a
clean before/after signal for this change specifically; `bun script.ts policy`'s breach census is.
