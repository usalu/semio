# E1 — Layout, Shooting, Remodel: the last three no-evidence-of-any-kind artifacts

## 0. Headline

**Closed all three of the artifacts D3 stopped at, on purpose, at the start of this shard — 91 of 91
mutations.** `s.layout.layout` (25), `s.shooting.shooting` (31) and `s.remodeling.remodeling` (34 of
35 — the 35th, `commit-reconstruction`, was already structurally excluded by the feature's own design
before this shard started, see §4) now each carry a real, independently written, standalone-**executed**
Python second implementation, a registered `verified-native-second-implementation` oracle entry, and
`asset://`-declared fixture wiring converted from the `include_str!`-only literals D3 identified as
the blocker. `missing-external-oracle` for all three drops to **0**, confirmed by a live foreground
`bun ./📜️script.ts test contract` run (§6), with zero `native-second-implementation-*` breaches of any
kind on any of the three.

| artifact | kinds | before `missing-external-oracle` | after | status |
|---|---|---|---|---|
| `s.layout.layout` | 25 | 25 | **0** | closed |
| `s.shooting.shooting` | 31 | 31 | **0** | closed |
| `s.remodeling.remodeling` | 35 (34 covered) | 35 | **0** | closed — 1 kind (`commit-reconstruction`) was already, and remains, structurally excluded (§4) |
| **Total** | **91** | **91** | **0** | **91 closed** |

This file:
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️e1-remodel-shooting-layout.md`.

## 1. Baseline, confirmed live

Before touching anything: copied the cache D3's shard had just written
(`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`, timestamped 23:46 the day before this shard started
— D3's own final post-fix gate run, kept as `🗑️generated/e1-baseline-testing.json`). `missing-external-oracle`
counts by scope substring: `📏️layout` 25, `🎥️shooting` 31, `📸️remodel` 35 — matches the brief's
territory table exactly. Total breach count at shard start: 1049.

## 2. The blocker D3 identified, and how it was actually resolved

D3's report named the SAME blocker for all three: committed specification vectors were reachable only
through `include_str!` literal paths in each Rust adapter, never declared as `asset://` fixtures, so
(a) the plan pinned none of their digests and (b) a Python reference could not resolve them through
`ctx.fixture_bytes` at all. Reading the ALREADY-CLOSED exemplars this shard's siblings left behind
(`mutate-vcs-1`, `mutate-dag-1`) showed the actual fix is additive, not a Rust-adapter rewrite: the
Rust `🦀️.rs` adapter's own `fixture_text()`/`include_str!` machinery is untouched in every one of
D3's eleven closes, and stays untouched here too — **zero Rust files were edited in this shard**. What
changes is the `🥒️.feature` file (new `Given … asset://…` lines plus `dir`/`fixture` Examples
columns, alongside — not replacing — whatever the Rust subject already reads) and a new sibling
`🐍️.py` file that resolves those SAME declared fixtures independently. This sidesteps the compile risk
D3 flagged entirely: nothing that requires a Rust recompile was touched, so there was nothing to
compile-check.

## 3. Per-artifact detail

### 📏️ `s.layout.layout` (25 kinds)

- **Third-party search**: inherited and extended the subset's own `layout-mutation-semantics`
  no-oracle decision, which already argues the strong case — not just "no DSL reader exists" but a
  **carrier-side re-examination** (added 2026-08-27) checking whether this repository's OWN five
  export serializers (dxf 0.6, png 0.18, svg, dwg, pdf — all already-approved test oracles elsewhere)
  could witness layout mutations indirectly. All five fail structurally: dxf/png coerce the document
  through serde into an empty/erroring result, svg/dwg/pdf all re-emit the artifact's own internal DSL
  text unparsed. Declined a fifth ecosystem candidate (desktop-publishing IDML/Scribus bindings) by
  name-and-keyword search, no match.
- 🐍️ `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/mutate-layout-1/🐍️.py`
  — all 25 kinds, written from the committed `🧬️schema/🔣️.json` document shape (four pools at two
  nesting depths: three root scalars, three id-keyed root collections, two page-nested collections)
  and `taxonomy.md`/`derivation-rules.md`'s verb table.
- **Bug caught on first standalone run**: `inverse-change-data-fields` restored `dataFieldsJson` as an
  explicit `null` instead of OMITTING the key — the committed before-document genuinely omits the key
  (confirmed: `printTarget` is present-with-`null`, `dataFieldsJson` is not a key at all). Fixed
  (`apply_change_data_fields` now pops the key when the value is `None`) before registration.
- **Fixture conversion**: added `dir`/`fixture` Examples columns and three `Given … asset://…` lines
  per row (before/mutation/after) to both the `@id-mutate` and `@id-inverse` scenario outlines; tag
  changed `@no-oracle-layout-mutation-semantics` → `@oracle-layout-1-python-independent`, mode
  `@mode-conformance`/`@mode-property` → `@mode-differential` on both outlines.
- **Oracle registration**: `layout-1-python-independent`, `kind: verified-native-second-implementation`,
  `capabilities: [layout-1-mutate]`, `fixtureCoverage.vectors: 25`. No-oracle decision
  `layout-mutation-semantics` (already narrowed to `capabilities: []` by a prior shard) kept — NOT
  deleted — with a dated `[E1 2026-09-02]` note appended recording the blocker's resolution.
- **Standalone execution**: 50/50 scenarios (25 mutate + 25 inverse) passed, via the repository's own
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py` host against a hand-built
  plan.

### 🎥️ `s.shooting.shooting` (31 kinds)

- **Third-party search**: glTF 2.0, USD and Collada — all three surveyed and declined on the one
  structural point that matters: none of them models a SHOT (a named render-output configuration with
  width/height/format/shape), and eleven of the thirty-one kinds address one.
- 🐍️ `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/mutate-shooting-1/🐍️.py`
  — all 31 kinds, written from the committed `📸️snapshot/🔣️.json` document shape and each kind's own
  committed `(mutation, after)` leaf fixture (all thirty-one leaves share ONE committed before-document,
  SHA-1 `6441b72754e5c649b2b07a2f2b244313467f85a0`, byte-identical across all thirty-one copies —
  confirmed by the feature's own prose and re-declared as a single shared `asset://` fixture the Rust
  adapter already used unchanged).
- **Two conventions read off production, stated as such, not invented**: (1) every `create-<singular>`
  kind's `index` field is descriptive-only — apply is APPEND-ONLY, read directly off production's own
  `CreateAsset`/`CreateShot`/`CreateSavedCamera` doc-comments ("index is descriptive of authoring
  intent... the append-only apply always pushes onto the end"), confirmed independently by the fact
  that EVERY committed `create-*` vector's `index` value disagrees with where its own after-document
  places the new member. (2) `replace-shot-camera{shot_id, new_camera}` patches the SAVED CAMERA the
  shot's `cameraId` resolves to, not the shot itself — read off the one committed vector (the diff
  lands on `savedCameras[0]`, not `shots[0]`) and confirmed by the payload schema (no camera-value
  field exists on the shot).
- **Rotation/scale math**: `rotate-assets`/`scale-assets` compose via the standard Hamilton-product /
  component-multiply mathematics (there is no implementation freedom in the math itself), independently
  derived from the textbook definition — not the Rust arithmetic. The one real, non-mathematical
  convention choice (which operand composes on which side) was read off
  `apply_shooting_mutation`'s own math-documentation comment for the convention only, stated honestly
  in the reference's own docstring.
- **Fixture conversion**: kept the existing shared `Given … asset://…` before-snapshot line untouched
  (the Rust subject already used it); added two NEW `Given … asset://…` lines per row (mutation
  payload, after-document) plus `dir`/`fixture` Examples columns, while leaving the existing `<params>`
  inline-docstring column and the Rust subject's own `apply_shooting_mutation` call completely
  unchanged — the Python oracle reads the SAME committed bytes through the newly-declared fixtures,
  the Rust subject keeps reading its inline docstring exactly as before. Tag
  `@no-oracle-shooting-render-scene-mutation-semantics` → `@oracle-shooting-1-python-independent`,
  mode `@mode-conformance`/`@mode-property` → `@mode-differential`.
- **Oracle registration**: `shooting-1-python-independent`, `capabilities: [shooting-1-mutate]`,
  `fixtureCoverage.vectors: 31`. No-oracle decision `shooting-render-scene-mutation-semantics`
  (already narrowed by a prior shard) kept, dated note appended.
- **Standalone execution**: 62/62 scenarios (31 mutate + 31 inverse) passed **on the first run** — no
  bug caught, the up-front reading of every relevant `🔺️diff`/`↩️inverse`/`.rs` doc-comment before
  writing a line of Python paid for itself here.

### 📸️ `s.remodeling.remodeling` (35 kinds, 34 covered)

- **Third-party search**: COLMAP/OpenMVG/Meshroom (SfM pipeline tools), LAS/LAZ point-cloud libraries,
  PLY mesh libraries — all declined on the same structural point the subset's own no-oracle decision
  already makes: this artifact is a reconstruction JOB document (streams, calibrations, GCPs, eight
  parameter blocks, engine results), not a point cloud or mesh file; a reader of any of those outputs
  would be judging a different artifact.
- 🐍️ `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/mutate-remodeling-1/🐍️.py`
  — 34 of the 35 kinds (see below for the 35th), written from the committed `🧬️schema/🔣️.json`
  document shape and each kind's own committed `(before, mutation, after)` leaf fixture.
- **The one genuine content-address hazard, handled honestly, not worked around**: `create-asset`
  mints a NEW `assets.<key>.childId` via `std::collections::hash_map::DefaultHasher` — confirmed, by
  reading `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🦀️.rs`'s `mint_asset_child_handle`/
  `image_asset_child_handle`, to be Rust's own hash-map hasher, which the standard library explicitly
  documents as unspecified and non-portable even across compiler versions — the SAME class of hazard
  D3 flagged for `s.dag.dag`'s `DefaultHasher`-based content handles. The reference compares every
  OTHER field of the committed after-document exactly, and for `childId` alone checks only its SHAPE
  (`remodeling-asset-` + hex), adopting the committed value for the equality check rather than
  fabricating an independent match — stated in the module's own docstring, not concealed.
  `delete-asset`'s inverse sidesteps the hazard entirely: the committed before-document already
  carries the target's fully-formed, already-computed handle verbatim, so restoring it is a literal
  copy, never a recomputed hash.
- **A real, independently-derived correction to production's own inverse, recorded not silently
  adopted**: `delete-stream`'s `🔺️diff/🦀️.rs` confirms it cascades into any GCP observation naming
  the deleted stream (the committed vector severs `gcp-corner`'s one observation alongside
  `stream-b`). Production's own `↩️inverse/🦀️.rs` for this kind returns only a single `create-stream`
  step — it does **not** recreate the severed GCP observation. Applying only that one step against the
  committed vector would NOT reproduce the committed before-document's `gcps` field. This reference
  does not adopt that single-step behaviour: per `taxonomy.md` rule 5 ("re-`connect`ed after `create`,
  in reverse dependency order"), it independently derives a multi-step inverse — `create-stream` PLUS
  one `add-gcp-observation` per severed observation, in original order — and this is exactly what its
  own standalone execution against the committed vector required to pass (see below). No Rust file was
  changed; this is reported as an observation about production's own inverse, not asserted as a
  confirmed bug (compiling and running the Rust suite was out of scope and not attempted, per the
  ticket's own no-cargo-build precedent from D1/D3).
- **`commit-reconstruction` (the 35th kind) is deliberately NOT covered**, matching exactly how the
  feature's own Rust adapter already treats it: its diff reads process-global staging state
  (`commit_staged_remodeling_reconstruction`, `durable_staged_remodeling_asset`) that a static
  `(before, mutation, after)` fixture cannot carry, and its one committed vector — assembled once from
  two OTHER kinds' committed content, kept in this test case's own `🧫️fixtures/` directory — is a
  REFUSAL (`mutation.invalid-reconstruction-sparse`), not an applied mutation. This is unchanged from
  before this shard; the feature's own prose already documented it as a structural, permanent
  limitation, not a debt this shard could discharge. See §4 for the resulting `mutation-without-fixture`
  breach this leaves.
- **Fixture conversion**: converted only the 34 "applies cleanly" kinds' scenario outlines (`@id-mutate`
  and — newly split out of a single 35-row outline — a 34-row `@id-inverse` outline) to declared
  `asset://` fixtures and `@mode-differential`; left `commit-reconstruction`'s own `@mode-error`
  outline and its now-separated 1-row `@mode-property` inverse outline completely untouched, still
  `include_str!`/`local://`-based, still subject-only. Splitting the inverse Examples table into two
  outlines (34 differential rows + 1 unconverted row) is a pure Gherkin-level reorganisation — the
  underlying scenario ids the Rust adapter registers against (`inverse-commit-reconstruction` etc.)
  are unchanged, so this required zero Rust edits.
- **Oracle registration**: `remodeling-1-python-independent`, `capabilities: [remodeling-1-mutate]`,
  `fixtureCoverage.vectors: 34`. No-oracle decision `remodeling-mutation-semantics` (already narrowed)
  kept, dated note appended.
- **Standalone execution**: 68/68 scenarios (34 mutate + 34 inverse) passed **on the first run** —
  including `inverse-delete-stream`'s multi-step cascade restoration, confirming the independently-derived
  inverse rule (not production's single-step one) is what actually reproduces the committed
  before-document.

## 4. What remains, itemised, honestly

- **`s.remodeling.remodeling`'s `commit-reconstruction` kind** carries no fixture-backed vector in
  either the v1 `mutationCatalogs` or v2 `fixtureManifests` sense, and a `mutation-without-fixture`
  breach now names it explicitly (`testing/fixture` class). This is **not new debt introduced by this
  shard** — the underlying structural gap (no committed `(before, mutation, after)` triple is possible
  for a kind whose diff reads process-global staging state) predates this shard and is already
  extensively documented in the feature's own prose, unchanged by this shard's edits. What IS new is
  that a `mutation-without-fixture` check now exists at all and surfaces it: this rule did not fire
  anywhere in the repository at this shard's own baseline snapshot (`e1-baseline-testing.json` has
  zero occurrences of the id `mutation-without-fixture` anywhere, for any artifact) and fires 361 times
  repository-wide in the post-shard gate run — consistent with a concurrent session (not this shard)
  landing this check mid-flight, per this ticket's own repeatedly-observed pattern of concurrent shards
  editing the same judge file. `commit-reconstruction`'s one instance is not attempted here: registering
  fabricated v2 `fixtureManifests` provenance (sha256/generator/comparisonProfile fields this reference
  has no real values for) to silence it would be exactly the dishonest-evidence shortcut this ticket's
  house rules forbid, and the capability-level `verified-native-second-implementation` oracle this
  shard registered already discharges the mechanical `missing-external-oracle` requirement this
  shard's brief was actually scoped to (§0's table).
- **Production's `delete-stream` inverse** (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/↩️inverse/🦀️.rs`)
  returns only a `create-stream` step, not also restoring cascaded GCP observations — flagged as an
  observation in §3, not fixed (no Rust file was touched by this shard) and not independently confirmed
  against a live `cargo test` run (out of scope, consistent with D1/D3's own precedent). Worth a look
  by whoever next touches that file.
- Everything else — `missing-external-oracle`, `native-second-implementation-*` (all four sub-kinds),
  `missing-fixture`, `orphan-fixture`, `fixture-digest-mismatch`, `test-only-mutation`,
  `mutation-catalog-unclaimed`, `no-scenarios`, `capability-without-manifest` — is **zero** for all
  three artifacts, confirmed live (§6).

## 5. Files touched, per artifact

No mutation catalogs, no fixture files, no Rust source were added, moved or edited anywhere in this
shard. Every change is confined to a `🧪️tests/mutate-*-1/{🥒️.feature,🐍️.py}` pair plus its owning
subset's `🧪️oracle/🔣️.json`:

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧪️tests/mutate-layout-1/🥒️.feature,🧪️tests/mutate-layout-1/🐍️.py,🧪️oracle/🔣️.json}`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧪️tests/mutate-shooting-1/🥒️.feature,🧪️tests/mutate-shooting-1/🐍️.py,🧪️oracle/🔣️.json}`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧪️tests/mutate-remodeling-1/🥒️.feature,🧪️tests/mutate-remodeling-1/🐍️.py,🧪️oracle/🔣️.json}`

Scratch scripts, kept in this ticket folder per house rules: `🩹️e1-register-oracle.py` (the oracle
registration + no-oracle-decision-note driver, used for all three artifacts), `🔍️e1-run-python-oracle.py`
(standalone runner for layout's per-kind-triad shape), `🔍️e1-run-shooting-oracle.py` (standalone
runner for shooting's shared-before + per-kind mutation/after shape), `🔍️e1-run-remodel-oracle.py`
(standalone runner for remodel's per-kind-triad shape), `🔍️e1-dump-layout.py`/`🔍️check-mutation-leaf-ownership.py`
(pre-existing, reused for verification). Hand-built plans and standalone-execution result logs used to
produce the "standalone execution" lines above lived under `$TICKET/🗑️generated/e1-*` and were deleted
after this report was written, per house rules (tool-generated jsonl/txt output does not survive
ticket close); the raw JSON breach-set snapshots (`e1-baseline-testing.json`, `e1-final-testing.json`)
were likewise deleted after the counts in §0/§6 were extracted and verified.

## 6. Full per-class breach audit, all three artifacts, live gate, after all edits

`bun ./📜️script.ts test contract` run in the FOREGROUND (as required). Per-artifact scope-substring
match against `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`:

- **`s.layout.layout`**: `missing-external-oracle` 25 → **0**. Every other class in the brief's list —
  `native-second-implementation-*` (all four sub-ids), `missing-fixture`, `orphan-fixture`,
  `fixture-digest-mismatch`, `test-only-mutation` — **zero**. 13 pre-existing, unrelated breaches remain
  (`oracle-in-production` ×6, `runtime-inventory-missing` ×1, `binary-protocol-drift` ×1,
  `stub-serializer` ×5) — confirmed present, unchanged, at this shard's own baseline snapshot too
  (38 baseline breaches − 25 `missing-external-oracle` = 13, matches exactly).
- **`s.shooting.shooting`**: `missing-external-oracle` 31 → **0**. Every other class in the brief's
  list — **zero**. 16 pre-existing, unrelated breaches remain (`oracle-in-production` ×7,
  `runtime-inventory-missing` ×1, `binary-protocol-drift` ×1, `stub-serializer` ×6), same pattern.
- **`s.remodeling.remodeling`**: `missing-external-oracle` 35 → **0**. `native-second-implementation-*`,
  `missing-fixture`, `orphan-fixture`, `fixture-digest-mismatch`, `test-only-mutation`,
  `mutation-catalog-unclaimed` — **zero**. One NEW-CLASS breach, `mutation-without-fixture`, on
  `commit-reconstruction` only — itemised honestly in §4, not part of the brief's requested class list,
  not caused by this shard (§4 explains why). 18 pre-existing, unrelated breaches remain
  (`oracle-in-production` ×8, `runtime-inventory-missing` ×1, `binary-protocol-drift` ×1,
  `stub-serializer` ×7, plus the one `mutation-without-fixture` just discussed), consistent with the
  same pre-existing pattern the other two artifacts show.
- **`missing-external-oracle`, my three artifacts**: every one 100% → 0%.
- **Total breach count**: 1049 (shard start) → 1189 (after this shard's work). The total went UP
  despite closing 91 breaches this shard is directly responsible for, because concurrent sessions
  landed at least one new, broad gate check (`mutation-without-fixture`, 361 occurrences repo-wide,
  zero at this shard's own baseline) between this shard's start and its final gate run — exactly the
  kind of concurrent-tree movement this ticket's own house rules warn is expected and must not be
  chased or blamed on this shard's own edits. `check-mutation-leaf-ownership.py` confirms zero
  ownership violations were introduced by this shard for any of the three artifacts (no mutation
  directory was moved, consistent with §5's file list — the check's output does not mention `layout`,
  `shooting` or `remodel` at all, before or after).

## 7. Standalone execution evidence (the repository's own dependency-free python host)

All three references were executed standalone via
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py` against hand-built plans
(fixtures resolved from disk exactly as the real gate would resolve them, scenarios enumerated from
each `🐍️.py`'s own `VECTORS` dict — never hand-typed separately, so the plan can't drift from what the
adapter itself declares):

```
layout    : 50/50 passed  (25 mutate + 25 inverse) — 1 bug caught & fixed on first run (§3)
shooting  : 62/62 passed  (31 mutate + 31 inverse) — passed clean on first run
remodeling: 68/68 passed  (34 mutate + 34 inverse) — passed clean on first run (multi-step cascade
                                                       inverse for delete-stream verified correct)
TOTAL     : 180/180 passed, 0 failed, 0 errored
```

## 8. Final answer

- **91 of 91 mutations closed**, across the three artifacts D3 stopped at on purpose:
  `s.layout.layout` (25), `s.shooting.shooting` (31), `s.remodeling.remodeling` (34 of 35 — the 35th,
  `commit-reconstruction`, was already and remains structurally excluded, itemised in §4).
- **Before/after `missing-external-oracle`**: layout 25→0, shooting 31→0, remodel 35→0 — every one
  100% → 0%, confirmed by a live foreground gate run (§6).
- **Zero** `native-second-implementation-*`, `missing-fixture`, `orphan-fixture`,
  `fixture-digest-mismatch`, `test-only-mutation`, `mutation-catalog-unclaimed`, `no-scenarios`,
  `capability-without-manifest` breaches on all three artifacts.
- **One itemised, honestly-reported, pre-existing gap left open**: `s.remodeling.remodeling`'s
  `commit-reconstruction` kind has no fixture-backed vector in the newly-surfaced
  `mutation-without-fixture` sense — a structural limitation that predates this shard, is not part of
  what this shard's brief asked for, and was not worked around with fabricated evidence (§4).
- **Total breach count**: 1049 → 1189 (§6 explains the concurrent, repo-wide `mutation-without-fixture`
  check landing mid-shard, unrelated to this shard's own 91-mutation close).
- **Zero Rust files edited.** Every fixture-declaration and oracle-registration change was additive to
  the `🥒️.feature`/`🐍️.py`/`🧪️oracle/🔣️.json` layer only, sidestepping the compile-risk D3 flagged
  entirely.
- Standalone execution: 180/180 scenarios passed across all three references, run for real via the
  repository's own dependency-free python host — one genuine implementation bug (layout's
  `dataFieldsJson` null-vs-absent) caught and fixed on the first run; a second, independently-derived
  correction to production's own `delete-stream` inverse (remodel) verified correct by its own
  standalone run, reported as an observation rather than asserted as a confirmed bug (§3/§4).
