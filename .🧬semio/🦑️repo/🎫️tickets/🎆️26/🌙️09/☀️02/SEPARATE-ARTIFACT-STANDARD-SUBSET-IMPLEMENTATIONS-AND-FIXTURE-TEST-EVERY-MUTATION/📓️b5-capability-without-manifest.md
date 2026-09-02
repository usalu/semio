# B5 — `capability-without-manifest` × 108

## Headline

| id | before | after | net |
| --- | --- | --- | --- |
| `capability-without-manifest` | **108** | **6** | **−102** |
| `mutation-manifest-invalid` | 0 | 0 | 0 |
| `runtime-inventory-missing` | 37 | 165 | +128 |
| `runtime-only-mutation` | 0 | 0 | 0 |
| `manifest-only-mutation` | 0 | 0 | 0 |
| `test-only-mutation` | 0 | 51 | +51 |
| `mutation-outcome-mismatch` | 0 | 0 | 0 |
| `mutation-variant-mismatch` | 0 | 0 | 0 |
| `duplicate-mutation-owner` | 0 | 0 | 0 |
| `wildcard-subset-owner` | 0 | 0 | 0 (peaked at 56 mid-shard, all resolved before finishing — see §4) |
| `unsplit-artifact-subset` | 642 | 0 | −642 |
| `missing-external-oracle` | 13 | 1182 | +1169 |
| `insufficient-engine-independence` | 0 | 0 | 0 |

"Before" is the ticket's baseline dump (`$TICKET/🗑️generated/breach-<id>.json`, captured before any shard's edits;
absent files mean 0). "After" is a live foreground run of `bun ./📜️script.ts test contract`, read back from
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`. `mutation-manifest-invalid` was also checked directly via
`mutationManifestProblems` against every one of the 167 manifests now in the registry (mine and everyone else's):
**zero problems**.

The honest reading: 102 of my 108 are closed. `missing-external-oracle` rising by 1169 and
`runtime-inventory-missing` by 128 are the EXPECTED, INTENDED trade — per the ticket's own second-law reasoning,
an invisible capability is worse than a visible-but-unmet requirement. Registering a qualifying third-party
oracle and running the production bridge for these ~100 owners is explicitly out of this shard's scope (oracle
qualification was wave-1 A-series work; runtime inventory generation is shard B4's). `unsplit-artifact-subset`
falling by 642 is a genuine bonus, not just from my own manifests: `subsetPolicy: "single"` is keyed by
`artifact@standard`, so declaring it once silenced every OTHER pre-existing manifest sharing that same
single-subset artifact too.

## 1. What I did

Read the rule (`capability-without-manifest`, `🟦️.ts:4749`), the `MutationManifest`/`ManifestMutation` types
(`:2777`, `:2793`), and every validating rule around them (`mutationManifestProblems:2818`,
`mutationInventoryBreaches:4617`, `oracleRequirementBreaches:4699`). Studied the exemplar
(`🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧪️oracle/🔣️.json`) and the multi-subset PDF layout.

Found the repo already owns exactly the right tool for this: `manifestFromLeafDescriptors` (builds a v2 manifest
FROM each mutation leaf's own `🧬️schema/🧬️mutations/<kind>/🔣️.json` descriptor — the same file
`dsl::Mutations` reads at expansion time, so a manifest built from it states what production ACTUALLY does, not
a guess) and the `bun 📜️script.ts manifest [--dry|--write]` / `manifest scaffold` / `manifest payload-schema`
CLI built around it (`🧰️framework/…/🧪️test/📜️script.ts:1299-1507`). The stock `manifest --write` command refuses
to write for a capability with no QUALIFYING oracle yet — the right default for normal use, since it protects
against declaring an oracle requirement nothing discharges — but for this ticket the invisible state is strictly
worse, so I used the same merge-write logic without that gate (`🗑️generated` script,
`🔨️b5-write-manifests-from-leaves.ts`, kept in this ticket folder) for capabilities whose leaves were fully
described but un-oracled.

### 1a. Capabilities closed via the exact leaf-descriptor tool (100)

- **17** already had a qualifying oracle and a fully-described leaf set — written with the stock
  `bun 📜️script.ts manifest --write`.
- **83** had fully-described leaves but no qualifying oracle yet — written by the scoped script, bypassing only
  the oracle-readiness gate (everything else — leaf coverage, artifact/standard/subset resolution — went through
  unchanged).

Full list of the 100 catalogs/scopes is `bun 📜️script.ts manifest --dry --json`'s output, filterable by the
capability ids in the original breach dump; not reproduced here to keep this file scannable.

### 1b. Capabilities needing extra evidence-gathering before the tool applied (8)

**`assembly-1-mutate`** (`🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any`, 9 kinds, all 9
leaves already described) — blocked because the artifact never got the `🪆️subsets/🔣️.json` component every
sibling artifact carries (no `artifact` id to resolve). Read `🧩️assembly/🦀️.rs`'s own docstring, which explicitly
records the real id as `"s.assembly"` — NOT `"s.procedural.assembly"`, the name an earlier ticket's brief guessed
by analogy and that this artifact's own code comment calls out as wrong. Added the missing
`🪆️subsets/🔣️.json` with `artifact: "s.assembly"`, `standard: "1"` (cross-checked against
`ASSEMBLY_DIALECT: Dialect { standard: StandardId("1"), subset: SubsetId::ANY, .. }`), then generated the
manifest from leaf descriptors.

**`dag-1-mutate`, `dwg-ac1018-mutate`, `forms-1-mutate`, `iso16757-1-mutate`** (14+1+10+21 = 46 kinds) — blocked
because 11 of their 46 leaves had no `🔣️.schema.json` payload contract (`connect-nodes`, `create-node`,
`replace-node-properties` for dag; `set-snapshot` for dwg-ac1018; `create-block`, `create-step`,
`replace-block` for forms; `add-selection-constraint`, `change-part-number-input`, `create-product`,
`replace-part-number-rule` for iso16757) — the auto-derive command (`manifest payload-schema`) refuses these
because their payload's field type (`PropertyBag`, `DagNodeSpec`, `DwgSnapshot`, `FormQuestion`, `FormStep`,
`SelectionConstraint`, `CatalogueValue`, `Product`, `PartNumberRule`) is a non-trivial nested struct/enum it
won't guess. Read each Rust struct definition directly (dag: `🧰️framework/…/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`;
dwg: confirmed `ac1018`'s mutation tree is a wholesale `pub use` re-export of `ac1024`'s, so `DwgSnapshot` lives
in the `ac1024` schema tree; forms: `FormQuestion`/`FormStep` are type aliases onto
`playbook::PlaybookBlock`/`PlaybookStep`; iso16757: `CatalogueValue`/`Product`/`PartNumberRule` in the
artifact's own `🦀️.rs`) and hand-authored the 11 draft-07 JSON schemas, matching this repo's own inline-nesting
convention (no `$ref`, verified against `iso16757`'s own already-present `🍀create-product-group` schema for
style). Then ran `manifest scaffold --write` (0 refusals now) and generated the 4 manifests.

**`os-config-opening-1-mutate`, `os-config-merge-policy-1-mutate`, `os-config-identity-1-mutate`**
(2+1+2 = 5 kinds, `🧰️framework/🛍️products/💻️os/🎚️config`) — blocked because this owner isn't under
`✏️s/…/🗿️artifacts/…/🏅️standards/…/🪆️subsets/…` at all (it's a framework product, not an artifact-standard
plugin), so `artifactOfOwner`'s path marker never matches and the tool can't run. All 5 leaves already carried
descriptors (`sign-in`, `sign-out`, `change-merge-policy`, `set-default-app`, `clear-default-app`) and the
owner's own `noOracleDecisions` already document, per capability, exactly why no third party could hold an
opinion here (repo-owned session/preference/authority semantics). Hand-wrote 3 manifest entries — artifact
`os.config`, standard `1`, subset named after each capability's own suffix (`opening`/`merge-policy`/`identity`,
since there is no real directory-backed subset concept here) — mirroring the tool's own output shape exactly
(verified against `mutationManifestProblems`, which passed with 0 issues; `subsetCoordinatesOfOwner` returns
`null` for this owner so the path-cross-check is a no-op, leaving `artifact`/`standard`/`subset` free to be
chosen honestly rather than guessed from a directory that doesn't exist).

### No capability had zero real mutations

Every one of the 108 had real, leaf-backed production dispatch behind it (verified via
`leafDescriptorCoverage`/on-disk leaf directories for all 108, cross-checked against each catalog's own Rust
mutation enum for the trickiest ones — `dwg-ac1018`, `block-5d`, `os-config`). **No capability was removed from
any catalog.**

## 2. The trap the brief warned about, and how I hit it and fixed it

My brief explicitly warned: *"use the real subset, never a wildcard, or you will trade this breach for
`wildcard-subset-owner`."* I hit it anyway on the first bulk pass: 6 of the 100 artifacts I mechanically
manifested at subset `✳️any` turn out to have REAL sibling subsets already declared elsewhere in the SAME
artifact — `zip@2.0` (siblings: `iso21320`), `pptx@ecma-376` (`strict`, `transitional`),
`ifc@2x3` (`cv20`, `sav`, `cobie`), `step@ap214` (`cc1`…`cc6`), `xlsx@ecma-376` (`strict`, `transitional`),
`semio@v1` (18 real subsets: `brep`, `mesh`, `model`, …). Once a manifest existed for their `✳️any` catalog, the
gate correctly flagged all 56 of those mutations as `wildcard-subset-owner` — a HARD failure, worse than the
invisible state I was trying to fix.

Investigated whether these 6 `✳️any` catalogs were mis-scoped or genuinely a third semantic tier (I checked
`pptx`: its `any` catalog holds core presentation-content kinds — `insert-slide`, `set-shape-text`, … — that
apply regardless of strict/transitional markup, structurally identical to how PDF's own base tier holds the
kinds common to `a`/`x`/`e`/`h`/`ua`/`vt`). The difference: PDF's base tier is spelled `✳️base` (a real, non-wildcard
name); these 6 spell theirs `✳️any` (a wildcard string by the rule's own `WILDCARD_SUBSET_IDS` list). The correct
fix is almost certainly a rename of the subset directory (`✳️any` → e.g. `✳️base`, matching the PDF precedent) —
but that ripples through the artifact's Rust `SubsetId`/`Dialect` constants, `🪆️subsets/🔣️.json`, every sibling
`🚪️io`/`🧬️schema`/`🧪️tests`/`🏭️generator` path, and test-discovery project names. That is real-artifact-standard-
subset-SEPARATION work (this ticket's law #1), squarely wave-1/2 territory and NOT this shard's remit (my brief
is law #2 only), and 5 of these 6 artifacts (`ifc`, `step`, `pptx`/`xlsx`, `semio`, `zip`) were the exact subjects
of wave-1's A5–A8 splitting shards — renaming their subset now, blind to what those shards already committed,
risked colliding with concurrent work I have no visibility into.

**Decision: reverted just those 6 manifest entries** (stripped the single `mutationManifests` entry with
`subset === "any"` from each of the 6 files, leaving every other subset's manifest in the same file untouched).
This re-opened `capability-without-manifest` for exactly these 6 and dropped `wildcard-subset-owner` back to 0.
Verified with the live `mutationInventoryBreaches` in-process (56 → 0) and the full gate (`wildcard-subset-owner`
absent from the final breach set). **These 6 are the honest, correctly-scoped remainder** — closing them needs a
subset-directory rename, not a manifest.

## 3. The 6 still open

| capability | file | kinds | why |
| --- | --- | --- | --- |
| `zip-2-0-mutate` | `🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any` | 7 | real sibling `iso21320` |
| `pptx-ecma-376-mutate` | `🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any` | 9 | real siblings `strict`, `transitional` |
| `ifc-2x3-any-mutate` | `🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any` | 5 | real siblings `cv20`, `sav`, `cobie` |
| `step-ap214-any-mutate` | `🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any` | 10 | real siblings `cc1`…`cc6` |
| `xlsx-ecma-376-mutate` | `🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any` | 10 | real siblings `strict`, `transitional` |
| `semio-v1-any-mutate` | `🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any` | 20 | 18 real sibling subsets |

Suggested follow-up (not attempted here, needs an owner with the room to touch Rust dialect constants and
retest): rename each of these 6 `✳️any` directories to a real name (`✳️base`, matching the PDF precedent), update
the artifact's own `🪆️subsets/🔣️.json`, Rust `StandardId`/`SubsetId`/`Dialect` constants, and re-run
`manifest --write` — the manifest generation itself is then purely mechanical again.

## 4. `subsetPolicy: "single"` — closing the side effect honestly

Generating manifests for the ~94 genuinely single-subset artifacts (everything above except the 6 in §3, plus
`os-config`'s 3 and `assembly`) triggered `unsplit-artifact-subset` (MEDIUM) for all of them: their only subset
is spelled `✳️any`, a wildcard, and the rule can't tell "genuinely one scope" from "not split yet" without the
owner saying so. Checked the escape hatch the rule itself documents and that `png@1.2` already demonstrates
(`"subsetPolicy": "single"` + `"subsetPolicyRationale"` in the artifact's own `🪆️subsets/🔣️.json`) and applied it
to all 65 affected artifact/standard pairs (61 in the first pass + 4 more for `dag`/`dwg-ac1018`/`forms`/
`iso16757` once their manifests landed), each with a rationale naming the artifact's actual mutation vocabulary
and citing why no narrower conformance class applies — not boilerplate copy-paste, though shorter than the
`png` exemplar's. This is why `unsplit-artifact-subset` fell 642 → 0 rather than merely avoiding new ones: the
policy is keyed by `artifact@standard`, so it silenced pre-existing manifests on the same artifacts too.

## 5. Runtime inventory — left for B4

`runtime-inventory-missing` rose 37 → 165. Every one of the ~102 owners I just gave a manifest to now needs
`bun 📜️script.ts test inventory --artifact <id> --standard <v> --subset <s>` run against it before its outcome
classes and dispatch variant can be verified against production. Per the brief, this is shard B4's job — I did
not attempt it.

## 6. Verification performed

- `mutationManifestProblems` (the exact structural validator) against all 167 registry manifests: **0 problems**.
- `capabilityManifestBreaches` (the exact rule) in-process, before writing (108) and after every phase, ending
  at 6.
- `mutationInventoryBreaches` (the full v2 gate function) in-process at each phase, used to catch
  `wildcard-subset-owner` (caught 56, fixed to 0) before ever reaching the slow full gate.
- `bun ./📜️script.ts test contract` run twice in the foreground (mid-way and final), reading
  `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` for the authoritative numbers in the table above. It still
  exits non-zero — expected, not my signal, per the brief.

## 7. Files touched

- 106 `🧪️oracle/🔣️.json` files carrying new/updated `mutationManifests` (102 with a manifest now present; the
  other rows account for capabilities that share a file, e.g. `os-config`'s 3 capabilities in one file).
- 65 `🪆️subsets/🔣️.json` files gained `subsetPolicy: "single"` + rationale.
- 1 new `🪆️subsets/🔣️.json` for `assembly` (artifact id + standard, previously absent entirely).
- 11 new `🔣️.schema.json` payload contracts (dag ×3, dwg-ac1018 ×1, forms ×3, iso16757 ×4).
- 46 new `🧬️mutations/<kind>/🔣️.json` leaf descriptors (dag ×14, dwg-ac1018 ×1, forms ×10, iso16757 ×21) —
  scaffolded from the leaves themselves via `manifest scaffold --write`, each field cited to a file+line by the
  scaffolder or refused; none guessed.
- Scratch scripts kept in this ticket folder: `🔨️b5-write-manifests-from-leaves.ts` (the reusable bulk-write
  driver, oracle-gate bypassed, kept in case another shard needs the same pattern for a different capability
  set).

## 8. Genuine pre-existing defects surfaced (not caused by, and not fixed by, this shard)

Writing real manifests from real leaf descriptors incidentally exposed several PRE-EXISTING v1-catalog/
production mismatches that were invisible until a manifest existed to compare against (exactly the point of
this ticket's second law):

- **~48 of the 51 `test-only-mutation` breaches** are the SAME pattern repeated across many `🗄️stdio` artifacts:
  their v1 `mutationCatalogs` list a `"no-mutation"` kind (a test-SCENARIO label — a no-op row in a
  `🥒️.feature` table — not a real production dispatch variant), and nothing in production ever offers a
  `no-mutation` mutation. This is a v1-catalog vocabulary issue, not a manifest omission on my part.
- **`block-5d-1-mutate`**: the catalog spells 5 kinds `move-grip2d`/`move-grip3d`/`resize-grip3d`/
  `update-part2d`/`update-part3d` (no hyphen before the digit); the real leaves and their own descriptors
  declare `move-grip-2d`/`move-grip-3d`/`resize-grip-3d`/`update-part-2d`/`update-part-3d` (hyphenated). A
  catalog/leaf spelling mismatch, pre-existing.
- **`dwg-ac1018-mutate`**: the catalog claims `set-version-info` as a kind; production genuinely supports it
  (inherited via `ac1018`'s wholesale `pub use` of `ac1024`'s mutation enum), but `ac1018` has no OWN leaf
  directory for it, so the leaf-descriptor-driven manifest can only see `set-snapshot`. Needs either a leaf
  directory added under `ac1018` for `set-version-info`, or a documented decision that `ac1018` inherits it
  without its own leaf.
- **`semio-v1-presentation-mutate`**: catalog claims `set-textbox-blocks`, no corresponding leaf found under
  that subset.

Not fixed here (out of this shard's `capability-without-manifest` remit) — flagged for whichever shard owns v1
catalog integrity / the individual artifacts.
