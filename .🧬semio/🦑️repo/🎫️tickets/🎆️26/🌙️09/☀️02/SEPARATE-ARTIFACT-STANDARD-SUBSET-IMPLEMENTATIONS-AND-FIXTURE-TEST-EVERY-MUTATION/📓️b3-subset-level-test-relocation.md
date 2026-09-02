# B3 — Subset-level test relocation

The headline deliverable: every artifact-level mutation test case (`<artifact>/🧪️tests/mutate-<x>-1`)
for the six artifacts whose mutations were already split into real subsets is gone. Each subset now
owns its own case, claiming its own per-subset mutation catalog. Nothing was silently dropped: every
scenario from every original feature has a new home, accounted for below.

## Before / after (measured, not asserted)

`bun ./📜️script.ts test contract`, filtered to `unknown-mutation-catalog` and
`mutation-catalog-unclaimed` scoped to note/draw/mathematical/sequence/fem2d/fem3d:

| breach | before | after |
| --- | ---: | ---: |
| `unknown-mutation-catalog` (5 artifact-level features claiming a dead `-any` catalog) | 5 | 0 |
| `mutation-catalog-unclaimed` (per-subset catalogs claimed by no feature) | 23 → 31\* | 0 |

\* Draw's four per-subset catalogs (`draw-1-metadata/structure/style/transform`) did not exist yet
when I took my first measurement — shard B1 split draw's catalog mid-session (see
`📓️b1-per-subset-catalog-scoping.md`). My true starting count once B1's split landed was 28
(`5 unknown + 23 unclaimed`); accounting for draw's 4 additional newly-unclaimed catalogs the real
total closed is 5 + 27 = **32 breaches**, all zero now.

I did **not** raise any of: `case-slug`, `missing-capability`, `missing-comparison`, `missing-oracle`,
`no-scenarios`, `no-adapter`, `mutation-kind-undeclared`, `mutation-kind-uncovered`,
`mutation-inverse-uncovered`, `missing-fixture`, `unknown-case-child`, `unknown-adapter-filename`,
`mutation-catalog-capability-mismatch` — confirmed by grepping the full breach set after every
artifact's edit, scoped to that artifact's paths. Full detail per artifact below.

One breach I **did** cause and fixed in the same pass: `oracle-capability-mismatch` (8, note only) —
retagging note's features to the per-subset `@capability-note-1-<subset>-mutate` tags B1's table
specifies (see below) meant the shared `note-python-independent` oracle's registered `capabilities`
list (`["note-1-mutate"]`, in `✳️any/🧪️oracle/🔣️.json`) no longer covered them. I extended that
list to also declare the eight new per-subset capabilities (that file is otherwise B1's; this was a
one-line, mechanical, directly-caused-by-me fix, not a redesign of B1's work).

`bun ./📜️script.ts test discover` — all 33 new cases discovered, each with the subset as its owner.
Full listing: `🗑️generated/b3-discover-evidence.txt`. Sample:

```
test-s-plugins-note-artifacts-note-standards-1-subsets-block-c63cdd-mutate-note-1-block	…/✳️block/🧪️tests/mutate-note-1-block	[rust,python]
test-s-plugins-note-artifacts-note-standards-1-subsets-any-0c8a14-round-trips-the-committed-document	…/✳️any/🧪️tests/round-trips-the-committed-document	[rust,python]
test-s-plugins-fem-artifacts-2d-standards-1-subsets-mesh-66d68c-mutate-fem2d-1-mesh	…/✳️mesh/🧪️tests/mutate-fem2d-1-mesh	[rust,python]
test-s-plugins-draw-artifacts-draw-standards-1-subsets-structure-95359b-mutate-draw-1-structure	…/✳️structure/🧪️tests/mutate-draw-1-structure	[rust]
```

## Shared shape

Each artifact's single `🧪️tests/mutate-<x>-1` case (mutate + inverse Examples, sometimes a
spec-vector replay outline, sometimes an identity round trip) was partitioned by which subset each
mutation *kind* belongs to (read straight off each subset's own `🧪️oracle/🔣️.json`
`mutationCatalogs[].kinds` — authoritative, not the original prose). One new case per subset,
named `mutate-<artifact>-1-<subset>` (kebab-case, satisfies `testCaseSlugPattern`), claiming
`@mutations-<catalog-id>` and `@capability-<capability>` exactly as declared by that subset's own
catalog (**not** assumed from the old artifact-level tag — capabilities turned out to be per-subset
for note/mathematical/sequence/draw and shared for fem2d/fem3d; verified per-artifact from disk right
before writing each feature, since B1 was editing these concurrently).

The one scenario with no mutation kind and no vector — `identity-round-trip` — has no catalog to
join (once every real kind has a subset, `✳️any` owns zero mutation catalogs), so it got its own new
case at `✳️any/🧪️tests/round-trips-the-committed-document/`, carrying the same `@capability`/
`@oracle`/`@no-oracle`/`@comparison` tags as before but no `@mutations-` tag.

### The fixture-URI rewrite that mattered

`asset://` and `shared://` resolve against the case's OWNER, and `resolveFixtures`'s path-escape
guard requires the resolved path to stay **inside** the owner directory (`join(...).startsWith(guard
+ sep)`). The original artifact-level case's owner was the ARTIFACT ROOT, so a `<vector>` cell like
`../../../✳️document/🧬️schema/🧬️mutations/…` could reach sideways into any subset — the whole
reason one case could address all of them. Once a case is OWNED by a subset, that same kind of
sideways reach is **rejected** (verified empirically on sequence: `asset://../✳️any/…` → immediate
`missing-fixture`). Two different fixes were needed depending on how each artifact's vectors reached
their fixtures:

- **note / mathematical(spec-vector) / fem2d(spec-vector) / fem3d(spec-vector) / draw**: vectors
  already lived physically INSIDE the target subset's own `🧬️schema/🧬️mutations/…` tree (earlier
  waves had already relocated the physical fixtures, only the TEST CASE lagged). Rewriting each
  `asset://` URI (or `include_str!` path) to drop the now-redundant
  `🏅️standards/🔖️1/🪆️subsets/✳️<subset>/` prefix was enough — the reference becomes subset-local
  automatically.
- **sequence / fem2d(mutate+inverse) / fem3d(mutate+inverse)**: these scenarios apply an inline JSON
  mutation to ONE SHARED derived-model fixture (`local://🎬️base-scene.json`,
  `local://🏗️timber-portal-frame.snapshot.json`, `local://🧊️steel-frame.snapshot.json`) that every
  kind, regardless of subset, mutates against. That fixture — and, for sequence, the real committed
  `.dsl.semio` example — got a **duplicated local copy** placed under each new case's own
  `🧫️fixtures/`, matching the precedent already on disk (note's `✳️asset/🧫️fixtures/` already held
  its own before/after SVGs distinct from `✳️block`'s). This is the lawful, separate-implementation
  answer, not a workaround: each subset's case is now a fully self-contained implementation that
  happens to need the same real-world input as its siblings.

### Adapter splitting strategy

Every original adapter (Rust and, where present, Python) was **generic and data-driven** — dispatch
by kind string, never a hard per-kind function — so splitting was mechanical: duplicate the shared
plumbing (fixture readers, report-shape helpers, the observability/inverse/divergence law calls) into
each subset's own adapter file, and scope only the `KINDS` list, the vector-lookup arms/`include_str!`
paths, and the registration loop to that subset's own kinds. Where a subset's kinds are a genuine
SLICE of one shared document schema (fem2d/fem3d's nine-member model; note's twelve-member document),
the model-shape knowledge (`RECORDS`, `COLLECTIONS`, `MEMBERS`, `validate()`) had to stay FULL in
every subset's copy — trimming it would have been wrong, not just unclean, since every scenario
validates the untouched siblings too, not only the fields its own kind writes. This is genuine
duplication of test-only Python/Rust across subsets (by design — "every subset implementation is
separate"), not code sharing.

All `include_str!` paths were verified to resolve on disk (Python-side filesystem check, not just
compiled) before moving on; all `asset://`/`local://` URIs were verified against the same rule
`resolveFixtures` uses (owner + path, escape-guard respected).

---

## Per artifact

### 🎬️ sequence — 2 subsets + 1 identity case (3 new cases from 1 old one)

Original: `🧪️tests/mutate-sequence-1` (`@mutations-sequence-1-any`, `@capability-sequence-1-mutate`,
`@no-oracle-sequence-step-graph-mutation-semantics`) — 8 kinds × (mutate + inverse) + 1
identity-round-trip = 17 scenario instances.

| new case | subset | kinds | catalog | capability |
| --- | --- | --- | --- | --- |
| `mutate-sequence-1-step` | `✳️step` | create-step, delete-step, move-step, edit-step-params, change-step-collapsed, duplicate-step (6) | `sequence-1-step` | `sequence-1-step-mutate` |
| `mutate-sequence-1-dependency` | `✳️dependency` | connect-steps, disconnect-steps (2) | `sequence-1-dependency` | `sequence-1-dependency-mutate` |
| `round-trips-the-committed-document` | `✳️any` | — (identity-round-trip) | none | `sequence-1-mutate` (kept) |

Fixture rewrite: `local://🎬️base-scene.json` and (new) `local://🗣️.dsl.semio` — a local copy of the
committed demo `.dsl.semio` — duplicated into `✳️step` and `✳️dependency`'s own `🧫️fixtures/`,
replacing the original `asset://…✳️any/📚️examples/…` reference that could no longer resolve from a
subset-owned case. `✳️any`'s own case keeps the direct `asset://📚️examples/…` reference (no owner
change). Accounting: 8+8+1 = 17 scenario instances, all present across the 3 new cases (6+6, 2+2, 1).

Rust-only (no oracle role — recorded no-oracle decision).

### ➗️ mathematical — 3 subsets + 1 identity case (4 new cases from 1 old one)

Original: `🧪️tests/mutate-mathematical-1` — 15 kinds × (mutate + inverse) + 1 identity-round-trip = 31.
Vectors reached via `include_str!`, not `asset://`, so the fixture-escape-guard issue never applied
here — only path-prefix rewriting.

| new case | subset | kinds | catalog | capability |
| --- | --- | --- | --- | --- |
| `mutate-mathematical-1-graph` | `✳️graph` | change-graph-directed, update-graph-algorithm, replace-graph, create-node, delete-node, delete-nodes, change-node-label, move-node, connect-nodes, disconnect-nodes (10) | `mathematical-1-graph` | `mathematical-1-graph-mutate` |
| `mutate-mathematical-1-geometry` | `✳️geometry` | replace-points, insert-point, remove-point, move-point (4) | `mathematical-1-geometry` | `mathematical-1-geometry-mutate` |
| `mutate-mathematical-1-equation` | `✳️equation` | change-coefficient (1) | `mathematical-1-equation` | `mathematical-1-equation-mutate` |
| `round-trips-the-committed-document` | `✳️any` | — | none | `mathematical-1-mutate` (kept) |

Accounting: 10+4+1 = 15 kinds × 2 (mutate+inverse) = 30, + 1 identity = 31, matches.

Rust-only (no-oracle decision; the oracle role in the original file answered with the committed
vector literally and is preserved per subset — never dispatched, since `@no-oracle-` stays).

### 🏗️ fem2d (`◻2d`) — 5 subsets + 1 identity case (6 new cases from 1 old one)

Original: `🧪️tests/mutate-fem2d-1` — 25 kinds × (mutate + inverse + spec-vector) + 1
identity-round-trip = 76 scenario instances. Cross-language differential (Rust subject + Python
oracle).

| new case | subset | kinds | catalog |
| --- | --- | --- | --- |
| `mutate-fem2d-1-mesh` | `✳️mesh` | create/delete/replace-{node→n/a,element,section,region}, create-node, delete-node (11: create-node, delete-node, create-element, delete-element, replace-element, create-section, delete-section, replace-section, create-region, delete-region, replace-region) | `fem2d-1-mesh` |
| `mutate-fem2d-1-material` | `✳️material` | create/delete/replace-material (3) | `fem2d-1-material` |
| `mutate-fem2d-1-boundary` | `✳️boundary` | create/delete/replace-support (3) | `fem2d-1-boundary` |
| `mutate-fem2d-1-load` | `✳️load` | create/delete-load-case, add/remove-load, change-load-case-self-weight, create/delete-combination (7) | `fem2d-1-load` |
| `mutate-fem2d-1-analysis` | `✳️analysis` | update-analysis-settings (1) | `fem2d-1-analysis` |
| `round-trips-the-committed-document` | `✳️any` | — | none |

Capability stayed shared (`fem2d-1-mutate`) across all subsets — confirmed live from
`🧪️oracle/🔣️.json` (fem2d/fem3d were not B1's territory, unlike note/draw/mathematical/sequence).

Fixture rewrite: `local://🏗️timber-portal-frame.snapshot.json` (the shared derived model every
mutate/inverse scenario applies its kind to) duplicated into all 6 new cases' own `🧫️fixtures/`.
`spec-vector-<kind>` scenarios read committed vectors through `asset://🧬️schema/🧬️mutations/<dir>/…`,
subset-local after dropping the artifact-level prefix — verified all 125 `include_str!` targets
(25 kinds × 5 fields) resolve on disk, split 55/15/15/35/5 across the five subsets.

Accounting: 11+3+3+7+1 = 25 kinds × 3 (mutate+inverse+spec-vector) = 75, + 1 identity = 76, matches.

### 🏗️ fem3d (`🧊️3d`) — 5 subsets + 1 identity case (6 new cases from 1 old one)

Structurally identical to fem2d (same generic Rust/Python architecture), with `solid`/`solids`
replacing `region`/`regions`, a `frame`/`bar` element-kind variant (`ELEMENTS` dict, not folded into
`RECORDS`), and combination `terms` as a case-keyed dict rather than a list — preserved verbatim in
every subset's Python copy, not simplified away.

| new case | subset | kinds | catalog |
| --- | --- | --- | --- |
| `mutate-fem3d-1-mesh` | `✳️mesh` | create/delete-node, create/delete/replace-element, create/delete/replace-section, create/delete/replace-solid (11) | `fem3d-1-mesh` |
| `mutate-fem3d-1-material` | `✳️material` | create/delete/replace-material (3) | `fem3d-1-material` |
| `mutate-fem3d-1-boundary` | `✳️boundary` | create/delete/replace-support (3) | `fem3d-1-boundary` |
| `mutate-fem3d-1-load` | `✳️load` | create/delete-load-case, add/remove-load, change-load-case-self-weight, create/delete-combination (7) | `fem3d-1-load` |
| `mutate-fem3d-1-analysis` | `✳️analysis` | update-analysis-settings (1) | `fem3d-1-analysis` |
| `round-trips-the-committed-document` | `✳️any` | — | none |

Fixture rewrite: `local://🧊️steel-frame.snapshot.json` duplicated into all 6 cases; 125
`include_str!` targets verified resolving (55/15/15/35/5 split). Accounting: same 11+3+3+7+1=25 ×
3 + 1 = 76, matches.

### 🖍️ draw — 4 subsets + 1 identity case (5 new cases from 1 old one)

Original: `🧪️tests/mutate-draw-1` — 14 kinds; mutate outline (13 kinds), a SEPARATE error outline
(1 kind: `duplicate-layer`, refused by clause — its only committed vector is a rejection), inverse
outline (all 14), + 1 identity-round-trip = 13+1+14+1 = 29 scenario instances. Single-language
(Rust only — vectors read via `include_str!` and compared against the committed after-document; no
Python file existed, `@no-oracle-draw-mutation-semantics` stays throughout).

| new case | subset | kinds | catalog | capability |
| --- | --- | --- | --- | --- |
| `mutate-draw-1-metadata` | `✳️metadata` | rename-layer, set-layer-locked, set-layer-visible (3) | `draw-1-metadata` | `draw-1-metadata-mutate` |
| `mutate-draw-1-structure` | `✳️structure` | create-layer, delete-layer, duplicate-layer, reorder-layer (4) | `draw-1-structure` | `draw-1-structure-mutate` |
| `mutate-draw-1-style` | `✳️style` | replace-layer-fill, replace-layer-stroke, set-layer-blend-mode, set-layer-opacity (4) | `draw-1-style` | `draw-1-style-mutate` |
| `mutate-draw-1-transform` | `✳️transform` | set-layer-boolean-operation, update-layer-trace-params, update-layer-transform (3) | `draw-1-transform` | `draw-1-transform-mutate` |
| `round-trips-the-committed-document` | `✳️any` | — | none (was already emptied by B1) | `draw-1-mutate` (kept) |

`duplicate-layer`'s error-outline row moved into `✳️structure`'s own error outline, alongside its
`UNOBSERVABLE`/`DECLARED_CODE` machinery (unique to that subset — the only draw kind with no
accepting vector). Every other subset's `UNOBSERVABLE`/`DECLARED_CODE` is empty. Accounting: 3+4+4+3
= 14 kinds; mutate scenarios = 13 (normal outline) + 1 (structure's error outline) = 14; inverse = 14;
+1 identity = 29, matches (3+3, 4+1+4, 4+4, 3+3 = 27 mutate+inverse across the four subsets +
2 identity feature/scenario... precisely: metadata 3+3=6, structure 3+1+4=8, style 4+4=8,
transform 3+3=6, any 1 = 29).

### 🗒️ note — 8 subsets + 1 identity case (9 new cases from 1 old one) — the headline case

Original: `🧪️tests/mutate-note-1` — 33 kinds × (mutate + inverse) + 1 identity-round-trip = 67
scenario instances. Cross-language differential (Rust subject + Python oracle, "independent second
implementation" of the whole document).

| new case | subset | kinds (count) | catalog | capability |
| --- | --- | --- | --- | --- |
| `mutate-note-1-document` | `✳️document` | rename-note (1) | `note-1-document` | `note-1-document-mutate` |
| `mutate-note-1-canvas` | `✳️canvas` | change-grid-visible/spacing/subdivisions/opacity, change-snap-enabled/grid-spacing (6) | `note-1-canvas` | `note-1-canvas-mutate` |
| `mutate-note-1-ink` | `✳️ink` | change-pencil-width, change-eraser-radius, change-block-ink-width, edit-block-ink-stroke (4) | `note-1-ink` | `note-1-ink-mutate` |
| `mutate-note-1-asset` | `✳️asset` | create-asset, replace-asset-payload, delete-asset (3) | `note-1-asset` | `note-1-asset-mutate` |
| `mutate-note-1-block` | `✳️block` | create/delete/delete(s)/duplicate/duplicate(s)/move-to-container/drag/rename/visible/locked/move/resize/font-size (13) | `note-1-block` | `note-1-block-mutate` |
| `mutate-note-1-text` | `✳️text` | edit-block-text (1) — refused by both implementations (see below) | `note-1-text` | `note-1-text-mutate` |
| `mutate-note-1-math` | `✳️math` | edit-block-math (1) | `note-1-math` | `note-1-math-mutate` |
| `mutate-note-1-table` | `✳️table` | insert/remove-table-row, insert/remove-table-column (4) | `note-1-table` | `note-1-table-mutate` |
| `round-trips-the-committed-document` | `✳️any` | — (identity-round-trip, refused by the Python side) | none | `note-1-mutate` (kept) |

1+6+4+3+13+1+1+4 = 33, matches the full kind list exactly.

**Known, preserved debt (not mine to fix, carried across intact):**
- `edit-block-text` (`✳️text`) — the Python reference still refuses it by clause (composed child
  handle addressing rule not documented anywhere in the repo); the Rust subject still implements it.
  This is a documented, expected differential mismatch, unchanged by the relocation.
- `identity-round-trip` (`✳️any`) — the Python reference still refuses it by clause (grammar/artifact
  disagreement on block kinds); Rust subject still does the carrier fixpoint + pack-cross-check.
- `duplicate-blocks`'s pre-mutation-indexed insertion order (documented finding, reproduced by both
  implementations, unchanged).

**Fixture rewrite:** all 33 kinds' vectors already lived under their target subset's own
`🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>/` (confirmed on disk before writing anything); the
`asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>` template — where `<vector>`
carried the `../../../✳️<subset>/…` traversal that made ONE artifact-owned case reach every
subset — became `asset://🧬️schema/🧬️mutations/<vector>` with `<vector>` now the bare
`<mutation-dir>/🧪️tests/<case>` relative to the subset itself. All 33 × 4 leaf files (before/
mutation/after/outcome) verified resolving on disk against every new subset owner (132 checks, 0
missing).

**Oracle fallout (fixed):** retagging to per-subset `@capability-note-1-<subset>-mutate` (per B1's
table) left the shared `note-python-independent` oracle registry entry's `capabilities: ["note-1-mutate"]`
stale, producing 8 `oracle-capability-mismatch` breaches. Extended that one array in
`✳️any/🧪️oracle/🔣️.json` to also list the 8 new per-subset capabilities (kept the original entry —
`✳️any`'s own round-trip case still uses it).

---

## Files touched

Created (new cases, 33 total — feature + adapter(s) + fixtures where applicable) under:
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/{✳️step,✳️dependency,✳️any}/🧪️tests/…`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/{✳️graph,✳️geometry,✳️equation,✳️any}/🧪️tests/…`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/{✳️mesh,✳️material,✳️boundary,✳️load,✳️analysis,✳️any}/🧪️tests/…`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/{✳️mesh,✳️material,✳️boundary,✳️load,✳️analysis,✳️any}/🧪️tests/…`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/{✳️metadata,✳️structure,✳️style,✳️transform,✳️any}/🧪️tests/…`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/{✳️document,✳️canvas,✳️ink,✳️asset,✳️block,✳️text,✳️math,✳️table,✳️any}/🧪️tests/…`

Deleted (the six old artifact-level cases, entirely):
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🧪️tests/`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🧪️tests/`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🧪️tests/`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🧪️tests/`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🧪️tests/`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🧪️tests/`

Edited (one file, mechanical, directly caused by my retagging):
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`
  (`note-python-independent` oracle's `capabilities` array extended by 8 entries)

Ticket-scratch generator scripts (kept, per house rules — not deleted, they're inputs not tool
output): `🔨️b3-note-data.py`, `🔨️b3-emit-note-features.py`, `🔨️b3-emit-note-rust.py`,
`🔨️b3-emit-note-python.py`, `🔨️b3-fem2d-data.py`, `🔨️b3-emit-fem2d.py`,
`🔨️b3-emit-fem2d-rust.py`, `🔨️b3-emit-fem2d-python.py`, `🔨️b3-emit-fem3d.py`,
`🔨️b3-emit-fem3d-rust.py`, `🔨️b3-emit-fem3d-python.py`, `🔨️b3-emit-draw.py`.

## Verification performed

- `bun ./📜️script.ts test contract` run before any edit (baseline) and after every artifact's edit
  (6 additional runs), each time filtering the full breach set to that artifact's paths and to the
  forbidden-category list from the brief.
- `bun ./📜️script.ts test discover` run after the final edit — all 33 new cases present, each with
  its subset as owner, adapters as expected (`[rust]` or `[rust,python]`). Evidence:
  `🗑️generated/b3-discover-evidence.txt`.
- Every `include_str!`/`asset://`/`local://` reference was checked to resolve on disk (Python
  filesystem checks mirroring `resolveFixtures`'s owner+path logic, including its escape guard) before
  considering an artifact done — not just left to the gate to discover.
- Every generated `🐍️.py` adapter was `python3 -m py_compile`d (13 files, all pass).
