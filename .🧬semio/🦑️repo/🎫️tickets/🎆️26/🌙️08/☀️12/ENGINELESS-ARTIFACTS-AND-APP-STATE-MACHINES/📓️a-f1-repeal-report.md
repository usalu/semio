# A/F1 — Repeal the artifact-engine mandate, install the forbidding ratchet

Workstream A, wave F1. **Status: PASS.** This is the gate that every A-packet waits on.

## Why a repeal comes first

`⚙️engine` under an artifact was not tolerated — it was **mandated**, by a rule that gated at `priority: "high"` on a *missing* folder:

```
policyArtifactEnginePresenceBreaches  →  "«artRel» is missing required ⚙️engine/ facet"   priority: high
```

So deleting even one engine directory before this rule was gone would have thrown `verify gate` **for all six sessions in this tree**, over a violation none of them created. That is the whole reason F1 is strictly first.

And the trait the mandate existed to serve never shipped: `grep -rn "trait ArtifactEngine"` → **0 hits** repo-wide, corroborated by `🏪️store/🦀️component.rs:3092`'s own comment that it *"never existed as a live trait"*. 95 directories and 153,457 LOC were built around a placeholder.

## Safety analysis done BEFORE editing (this is the load-bearing part)

The risk was the mirror image: does *removing* the word make the 95 existing dirs illegal at a gating priority? Measured, not assumed:

| Question | Answer | Evidence |
|---|---|---|
| Where do the 95 actually sit? | **All at exactly one depth**, `<artifact>/🏅️standards/<std>/🪆️subsets/<subset>/⚙️engine`. **Zero** outside `🪆️subsets` | depth histogram → `95 depth-4`; `grep -vc 🪆️subsets` → `0` |
| Is there a subset-level "may only contain" rule? | **No.** Restriction rules exist for artifact dirs, schema children, io directions, representation dirs and window children — not subsets. Subsets have only a *totality* (required-list) check at `:5604` reading `subsetChildDirs` | `grep -n "not a recognized\|may only contain"` |
| Can the artifact-level restriction reach them? | **No.** `:4148` walks only direct children of `🗿️artifacts/<a>/`, which are `🏅️standards` | `:4053` loop |
| Could that rule gate anyway? | **No.** `policyNewSurfacePriority(crate, "medium")` returns `"medium"` for plugin crates and `"low"` otherwise — **never `"high"`** | function body |

⇒ Shrinking the vocabulary is **inert on its own**: it makes the 95 no longer *required*, and nothing flags them as *extra*. Zero new breaches. The only thing that would have gone red is the rule being deleted.

## What changed

**`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`** — note this is **not** at the repo root; the standing cross-session write-queue named a repo-root path that does not exist, an error UCAS confirmed they had propagated to several sessions.

| Key | Before | After |
|---|---|---|
| `artifactComponentDirs` | `["🧬️schema","⚙️engine","🚪️io"]` | `["🧬️schema","🚪️io"]` |
| `artifactChildDirs` | `[…,"⚙️engine","🚪️io","📚️examples"]` | `["🧬️schema","🚪️io","📚️examples"]` |
| `subsetComponentDirs` | `["🧬️schema","⚙️engine","🚪️io"]` | `["🧬️schema","🚪️io"]` |
| `subsetChildDirs` | `[…,"⚙️engine","🚪️io","📚️examples"]` | `["🧬️schema","🚪️io","📚️examples"]` |
| `_standardsSubsetsComment`, `_engineListComment` | "subsets own schema, engine, IO…" | reworded, with the ticket cited |
| **`taxonomyLeafParentDirs`** | contains `⚙️engine` | **UNCHANGED** — this is what keeps `🧰️framework/🔨️modules/<domain>/⚙️engine/` legal |
| **`appChildDirs`, `appComponentDirs`** | contain `⚙️engine` | **UNCHANGED** — apps still *require* an engine; that is where behaviour is moving |
| **`schemaChildDirs`** | missing `💡️inferences` | **UNCHANGED** — IIF's vocabulary. Stood down after UCAS flagged the sequencing hazard: the gate asserts named dirs exist, so adding it before IIF's fan-out completes fails repo-wide |

**`🔍️discovery/🟦️component.ts`** — dropped `⚙️engine` from the required tuple at `:405`; **kept `:494` unchanged** (that is the `taxonomyLeafParentDirs` assert). Added the inverse: a check that the four artifact/subset vocabularies **must not** contain `⚙️engine`, with the reasoning inline.

**`📜️script.ts`** — both mandating rules deleted, **both halves each**:

| Deleted | Definition | Aggregator call |
|---|---|---|
| `policySubsetEnginePresenceBreaches` | `:5626` | `:5807` |
| `policyArtifactEnginePresenceBreaches` | `:6418` | `:7066` |

Added `policyArtifactEngineFacetForbiddenBreaches` at `priority: "low"`, registered in `policySubsetConformanceBreaches`, with a helper `policyArtifactEngineOwnerDirs` that walks **all three** levels an engine could appear at (artifact, standard, subset) so the shape cannot reappear one level up.

**`🧪️index.test.ts`** — four stale assertions updated (`:1117`, `:1154`, `:1165`, `:1166`) plus one comment.

## Why a forbidding rule and not just the repeal

Deleting the mandate leaves the directory merely *optional* — an agent scaffolding a subset by copying a sibling recreates it and nothing objects. Inverting the polarity makes the shape **unconstructible rather than unfashionable**. The three parts together (shrink the vocabulary so it is unmentionable in the SSOT, delete the mandate so there is no reason to create it, add the prohibition so reappearance is a breach) are the same construction APA used for `🔧️setup`.

## Verification (real commands, real output)

**1. Taxonomy self-consistency and the discovery validator:**

```
$ bun -e '… loadTaxonomy() / validateTaxonomy() …'
artifactComponentDirs == true
artifactChildDirs     == true
subsetComponentDirs   == true
subsetChildDirs       == true
structural-minus-completeness == true
validateTaxonomy problems: 0
```

`validateTaxonomy` returning **0 problems** simultaneously proves the required-tuple edit landed, the new prohibition does not fire, and the `taxonomyLeafParentDirs` requirement is still satisfied.

**2. The census — the burn-down baseline** (`scratch-policy-after-f1.txt`, 22,205 lines):

```
$ bun ./📜️script.ts policy          # exit 0
# blocking output: zero engine entries of any kind

$ python3 … .🦑️repo/⚡️cache/breaches/compose.json
top-level keys: ['entityId', 'script', 'breachs']
using key: breachs
total breaches: 26151

engine-forbidden count:      95      ← exactly matches the disk census
old engine-presence count:    0      ← repealed
old artifact-engine count:    0      ← repealed
  low ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any
  low ✏️s/🔌️plugins/➗️mathematical/…/🪆️subsets/✳️any
  low ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/…/🪆️subsets/✳️any
```

**95 at `low`, both old rules at 0, nothing blocking.** The count is now the burn-down chart for every A-packet.

> ⚠️ **The cache key really is `breachs`, not `breaches`.** Confirmed live above. A query on the correctly-spelled key returns `[]`, which reads as *total success*. Any future "the count is zero" claim must be made by counting directories on disk, not from this file.

**3. Test-suite honesty — 19 failures, none mine.** `bun nx run @semio-tech/repo-lib:test-quick` → 135 pass, 19 fail. Rather than assert they were pre-existing, I proved it for the three that could plausibly have been mine:

```
$ git show HEAD:🔣️taxonomy.json | python3 …
HEAD snapshotChildDirs: MISSING
HEAD artifactComponentDirs: ['🧬️schema', '⚙️engine', '🚪️io']
```

`loadTaxonomy > parses …` fails at **line 1119** on `snapshotChildDirs` being `undefined` — a key absent from the taxonomy **at HEAD as well**, so that test was already failing before this ticket existed, on an assertion I did not touch. My own assertion sits at `:1117` and passed (the runner reached `:1119`), and the direct check in (1) confirms all four of my values. The remaining 16 failures (`dependency-boundary`, `ui scrollbar`, `micro-commit`, `playground ports`, `package boundary`, `commit message`, `command budgets`, `resolveCargoPackageName`, `discoverPackages` ×4, `computeWorkspaces`) touch no taxonomy key and belong to other sessions' surfaces.

**No claim is made that this suite is green. It is not, and it was not before.**

## Files touched

- **Updated**: `🔣️taxonomy.json` (6 edits) · `🔍️discovery/🟦️component.ts` (2 edits) · `📜️script.ts` (2 rules + 2 call sites deleted, 1 rule + 1 helper added) · `🧪️index.test.ts` (4 assertions + 1 comment).
- **Created** in this folder: `scratch-policy-after-f1.txt`, `scratch-repo-lib-test-f1.txt`, `scratch-repo-lib-test-f1-clean.txt`.
- **Removed**: nothing on disk — no engine directory was deleted in this wave. That is packet work.

## sharedFileRequests

None outstanding. `📜️script.ts` and `🔣️taxonomy.json` were taken with an explicit slot grant from UCAS (queue head, ahead of their W6), announced before starting; completion announced after.

## Concurrent-churn observations

- UCAS granted the slot and independently verified all three of my claims (no repo-root taxonomy file; `trait ArtifactEngine` → 0 hits; both rules at the stated lines with second registration sites). They confirmed the "repo-root" wording was **their** error, propagated to four sessions.
- Their warning that each rule has a second registration site was correct and acted on. A dangling spread would be a hard TS error rather than a silent skip, but finding that out inside their W6 would have been a poor way to learn it.
- Three of UCAS's messages misrouted to this session (they were addressed to APA/IIF) because peer names rotated after a session-limit restart. Root cause they accepted: a fact true when sent, consumed as if still true, with no acknowledgement closing the loop. They have adopted "release is not released until acked".

## Result

**PASS.** The mandate is repealed, the prohibition is installed at non-gating priority, the census reads exactly 95, and no peer's gate went red. **Workstream A packets are now unblocked.**

## Next

- Dispatch the exemplar packet **P-A2 (`🧱️block`, 3 dirs, ~400 work LOC)** and publish its diff as the pattern every other packet follows.
- Then Tier A in parallel; Tier B/C gated on SMO's live predicate; `🗄️stdio` last, gated on UCAS's roster freeze (**not** frozen — `✳️text` has 6 failing law tests, `table`/`graph` unmounted, spatial `object`/`kit` not started).
- The forbidding rule rises to `"high"` only when the on-disk count reaches 0.
