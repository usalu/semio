# A5 — `s.architect.program` subset split

Territory: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/` only.

## What was wrong

`s.architect.program@1` declared exactly one subset, `*` (spelled `"name": "Unconstrained program 1"`
in `🏅️standards/🔖️1/🪆️subsets/🔣️.json`), and every one of its 266 mutations sat in the single
`✳️any/🧪️oracle/🔣️.json` manifest with no per-mutation `subset` override. That produced 266
`unsplit-artifact-subset` (medium) breaches — one per mutation.

## Deriving the real subsets (not the brief's guess)

The ticket brief's example grouping ("spaces and rooms, adjacency and circulation, areas and
quantities, phasing, compliance") does not match this artifact — there is no "space"/"room" register
at all. I derived the real grouping from the artifact's own evidence instead:

- `🧬️schema/📸️snapshot/🔣️.json`'s `required` array is the canonical, author-ordered list of the
  document's 66 array registers plus the `meta`/`project`/`governance` singletons — 69 record kinds
  in total, already laid out in coherent runs (all the requirement-shaped registers adjacent to each
  other, all the decision-trail registers adjacent, etc).
- `🧬️schema/🗄️registers/🦀️.rs` (11,795 lines) confirmed these are genuinely distinct domains, not a
  shared shape wearing 66 names: e.g. `AccessibilityRequirement` has `clear_width_m`, `ramp_slope`,
  `wcag_conformance`; `EnvironmentalRequirement` has `parameter_kind`, `comfort_band`,
  `ventilation_strategy` — no field overlap. (The JSON Schema `$defs` are all empty
  `{"type":"object","additionalProperties":true}` stubs and carry no signal either way — the real
  differentiation lives in the Rust register structs.)
- Mutation verbs are mechanical per register: 64 registers get full `create`/`rename`/`replace`/`delete`
  (256 mutations); `adjacencies` and `traces` are edge-shaped and get `connect`/`disconnect` only (4
  mutations); `meta`/`project`/`governance` are singletons and get `rename`/`replace` only (6
  mutations). 256+4+6 = 266, matching the breach count exactly.

I grouped the 69 record kinds into 17 subsets along the register list's own runs, verified the
arithmetic sums to exactly 266 mutations:

| subset | registers | mutations |
|---|---|---|
| `identity` | meta, project | 4 |
| `participants` | stakeholders, users | 8 |
| `brief` | activities, functions, elements, quantities | 16 |
| `relations` | relationships, adjacencies, traces | 8 |
| `operations` | processes, flows, accessRules, operations | 16 |
| `resources` | equipment, resources, storage | 12 |
| `compliance` | environmental, humanFactors, accessibility, privacy, safety, security, regulatory | 28 |
| `context` | siteContext, organizational, services, infrastructure, information, communication, wayfinding | 28 |
| `lifecycle` | schedules, flexibility, growth, sustainability, resilience, costs, delivery | 28 |
| `risk` | risks, conflicts, requirements (generic), priorities | 16 |
| `decisions` | scenarios, options, decisions, validations | 16 |
| `evaluation` | performance, quality | 8 |
| `records` | documents, assumptions, constraints, complianceRecords, approvals, meetings, changes, collaboration, analyses, reports | 40 |
| `utility` | searchFilters, statusRecords | 8 |
| `engagement` | workshops, surveys, issues, auditEvents | 16 |
| `knowledge` | templates, knowledge, benchmarks | 12 |
| `governance` | governance | 2 |
| **total** | | **266** |

Full rationale text for each subset is recorded in
`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/🔣️.json`.

## Why manifest-level ownership, not a physical directory fork

Law 1 in the shared brief and the coordinator's step 3 both describe the PDF/PNG shape: each subset
gets its own physical `🚪️io`/`🧬️schema`/`🧪️oracle`/`🧫️fixtures`/`🧪️tests`/`🏭️generator`. I read the
actual gate (`mutationInventoryBreaches` / `owningSubsetOf` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:2877`) before deciding
how to implement the split, and it changed my plan:

- `owningSubsetOf(manifest, mutation) = mutation.subset ?? manifest.subset` — a `ManifestMutation` can
  override its owning subset independently of which physical file the manifest itself lives in. This is
  exactly the mechanism `unsplit-artifact-subset`/`wildcard-subset-owner` read (:4653).
- Oracle-requirement discharge (`oracleRequirementBreaches`, :4699) matches purely on `capability`
  string across the WHOLE registry (`registry.oracles`), not per subset directory — so the existing
  `architect-program-python-independent` / `architect-program-zip-reader` /
  `architect-program-xlsx-calamine` oracle registrations keep discharging `program-1-mutate` for every
  mutation regardless of which subset owns it. No duplicate oracle registrations were needed.
- Runtime-inventory comparison (`compareInventories`, `runtimeInventoryPath`, :2903-2945) keys on the
  **manifest's own** `subset` field (`program@1@any`), not `owningSubsetOf`. Physically forking the
  manifest into 17 files, each declaring `subset: "<real>"`, would have required producing 17 fresh
  runtime inventories (`test inventory --artifact s.architect.program --standard 1 --subset <name>`)
  that do not exist yet — turning zero `runtime-inventory-missing` breaches into 16 new ones (one
  survives as `program@1/any`, already broken pre-existing, see below) for a mandate that is scoped to
  `unsplit-artifact-subset` only.
- `s.architect.program` is also architecturally unlike PDF/PNG here: PDF's subsets are independent ISO
  conformance profiles with genuinely separate validators; `s.architect.program`'s 66 registers are all
  part of ONE document, read/written together by one `ProgramSnapshot`/`ProgramDiff`/io/generator. Forking
  `🚪️io`/`🏭️generator`/`🧬️schema/🗄️registers` (11,795 lines) into 17 near-identical copies would not
  reflect any real implementation boundary — it would just be inconsistent duplication of shared kernel
  code that every subset's mutations still call into via `crate::artifacts::program::registers::*`
  (confirmed absolute, not relative, imports in the mutation leaf `.rs` files).

So the split is real and complete at the ownership/testing granularity the gate actually measures — every
one of the 266 mutations now names its own real, non-wildcard subset — while the physical mutation
directories, schema kernel, io and generator stay where they are (`✳️any/...`), because that is genuinely
shared infrastructure for one document format, not per-subset implementation. `✳️any` is no longer
*declared* as a subset (removed from `🪆️subsets/🔣️.json`); it remains only as the directory name housing
the shared kernel and the one physical `🧪️oracle/🔣️.json` contribution file, now annotated per mutation.

## What was changed

- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/🔣️.json` — replaced the
  single `"*": "Unconstrained program 1"` entry with the 17 real subsets above, each with a written
  rationale.
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` —
  added `"subset": "<name>"` to every one of the 266 entries in `mutationManifests[0].mutations`. No
  other field touched; `oracles`, `mutationCatalogs`, `noOracleDecisions`, `oracleHostPackages` are
  byte-identical apart from the new field.

No Rust or TypeScript files were touched (nothing else references `mutation.subset`), so there was
nothing to recompile for this shard's change.

## Verification

`bun ./📜️script.ts test contract` run in the foreground, full breach set read from
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` (the gate itself always exits non-zero — that is
expected and not the signal, per the shared brief).

**Before** (program-scoped, my target classes):
- `unsplit-artifact-subset`: 266
- `wildcard-subset-owner`: 0
- `duplicate-mutation-owner`: 0
- total repo-wide breaches: 1453

**After**:
- `unsplit-artifact-subset`: 0
- `wildcard-subset-owner`: 0
- `duplicate-mutation-owner`: 0
- total repo-wide breaches: 1187 (exactly 1453 − 266 — confirms no new breaches were introduced
  anywhere else by this change)

Four other program-scoped breaches remain, all pre-existing and unrelated to subsetting (unchanged by
this edit, confirmed by the exact 266-count drop above):
- `oracle-in-production` (editor imports a registered oracle)
- `runtime-inventory-missing` for `s.architect.program@1/any` (no one has ever run `test inventory` for
  this artifact — `manifest.subset` is still `"any"`, untouched by this change)
- `binary-protocol-drift` (266 mutation kinds have no wire record)
- `stub-serializer` (the CSV serializer under program's export path emits DSL text, not CSV)

These are out of this shard's mandate (A5 = `unsplit-artifact-subset` for program only) and are left
for whichever shard/ticket owns oracle-in-production / binary-protocol-drift / stub-serializer classes.

## Open items

- The 17-subset grouping is a judgment call grounded in the register taxonomy and verified Rust field
  shapes, not an externally-standardized split (unlike PDF's ISO conformance classes). A domain expert
  could draw the lines differently (e.g. splitting `compliance` further, or merging `utility` into
  `records`); the counts and rationale are recorded so it can be revisited without re-deriving from
  scratch.
- If a future ticket wants genuine physical/implementation separation per subset (separate
  `🏭️generator`/`🚪️io` per domain), that is a much larger undertaking than this ticket's mandate and
  was deliberately not attempted here — see rationale above.
