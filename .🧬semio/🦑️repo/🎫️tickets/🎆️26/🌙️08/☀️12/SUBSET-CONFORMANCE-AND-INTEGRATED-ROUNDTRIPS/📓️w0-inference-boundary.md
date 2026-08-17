# W0 — Inference Ownership Boundary

Audited: 2026-08-12. Primary sources: `INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING` (`📓️status.md`, `📓️summary.md`, `📌️important.md`, `📓️audit-matrix.md`).

Placement rule for this ticket (**S6**): **consume** the inference ticket's API and registry; **do not** add spine traits, OS cache engine code, taxonomy flips, or a parallel cache model.

---

## What IIF (#2546) owns

### Framework spine (shipped)

| Surface | Location | Notes |
|---|---|---|
| `Inference<P>`, `DiffRegions`, `TouchedPaths`, `InferenceFieldSpec`, `InferenceSpec<P>` | `📡️spr/🎮️command/🦀️component.rs` `//#region 🔖️Inference` | 9 law tests at spine |
| `StateClass::Inferred` | `📡️spr/🧾️wire/🦀️component.rs` + schema derive `#[state(inferred)]` | dual-copy derive glue must stay mirrored |
| `ArtifactInferenceDescriptor`, `ArtifactInferenceRegistry`, `artifact_inference_graphql_sdl` | `🧬️schema/🦀️component.rs` | **sibling registry**, not embedded in `ArtifactSchemaDescriptor` |
| `KernelArtifactInferenceDescriptor` | `📡️spr/🧾️wire/🦀️component.rs` | wire catalog |
| OS cache engine | `💡️inference/🦀️component.rs` (mounted `os_inference`) | `DepHash`, `InferredField<P>`, `InferenceCache`, `InferenceSession`, `infer_field` / `infer_field_after_diff` |
| `ArtifactInferrer` trait | `🔌️plugin/🦀️component.rs` | plugin registration surface |

### Taxonomy + policy (not yet flipped — IIF P3)

| Change | Owner | Blocker |
|---|---|---|
| `schemaChildDirs += 💡️inferences` | IIF | runtime `assert!` at `🔌️plugin/🦀️component.rs:2226-2235` — panics if any owner lacks facet dir |
| ~10 `📜️script.ts` inference policy regions | IIF | writer queue position 4 (after APA, UCAS-W6, SMO) |
| `POLICY_INFERENCE_*` family entries | IIF | unresolved pure-fn vs `InferredField` policy conflict (see below) |

### Fan-out scope (IIF)

| Phase | Scope | State |
|---|---|---|
| P0 audit | 72 existing families | substantially done — see audit matrix |
| P1 pilots | puzzle 5d slimming, trinity `recompute_derived` removal | puzzle verifying; trinity hold (APA relocation) |
| P2 stdio | 22 target subsets (revised roster) | **blocked on UCAS roster** |
| P3 seal | taxonomy flip + policy + verify | not started |

**Reassigned to DKM (#2550):** `🧿️semio ✳️brep`, `✳️drawing`, `✳️mesh` inference facets (tessellation, mass props, validation report, flattened scene).

**IIF-owned stdio failures (baseline 5):**

1. csv `inference_default_law`
2. html `inference_default_law`
3. json `inference_default_law`
4. pdf `inference_default_law`
5. md `outline::collects_headings_and_counts_words_and_blocks`

UCAS stdio long-profile baseline: **2021 pass / 5 fail / 3 skip** — anything beyond these 5 is a new regression.

---

## What this ticket (subset conformance) may do

### Allowed (S6 consumption)

- Require every **owning** subset to declare ≥1 inference slug in manifest (`📓️plan.md` contract).
- Wire subset roundtrip stage 7 through **`infer_field` / `ArtifactInferrer`** already registered for that subset.
- Add **subset-local** `💡️inferences/<slug>/` facet trees (rs/ts/graphql/json/proto + optional grammar leaves) that implement honest derivations.
- Extend existing test regions (no new test files) with determinism, dependency-hit/miss laws **using IIF helpers**.
- Declare inference metadata in subset `🔣️component.json` manifest (dependency fields, slug list) — not parallel registry types.
- For **derived profile** subsets: require a real inference gate (outline, bounds, conformance summary) distinct from `✳️any` — can be pure-fn leaf if whole-snapshot fold (pending IIF policy decision).

### Forbidden (IIF exclusive)

- New cache engine, `DepHash` implementation, or second inference registry.
- Editing `schemaChildDirs` to add `💡️inferences` (IIF P3 atomic flip).
- Adding `Inference<P>` trait methods or changing `InferredField` associated type contract.
- Fixing the 5 stdio inference test failures (IIF P0/P2 unless IIF explicitly delegates).
- Authoring `🧿️semio ✳️brep|drawing|mesh` inference facets (DKM).
- Migrating puzzle5d snapshot fields or trinity PropertyBag (IIF P1).

### Gray zone (coordinate first)

| Item | Default |
|---|---|
| Subset roundtrip harness in `🏪️store` calling inference APIs | OK if read-only consumer of public `os_inference` exports |
| Derived subset needing per-entity merkle cache | ask IIF — may belong in IIF fan-out, not subset migration worker |
| Promoting inference policies to high in W6 | only after IIF P3 lands; diff against 22188 pre-existing high breaches |

---

## Unresolved IIF design question affecting this ticket

**Pure-fn vs `InferredField` policy** (`📓️audit-matrix.md`):

- 8/72 families use real `InferredField` + merkle incrementality.
- 64/72 use sanctioned pure-fn folds (`XOutline::compute(&snapshot)`).
- `📌️important.md` rule 13 says pure-fn without `InferredField` is a breach; approved plan says pure-fn is legal.

**Impact on subset conformance:** W3 reference subsets and derived-profile gates must not assume all inferences are merkle-cached. Stage 7 should:

1. Prove determinism for all subsets.
2. Prove dependency-hit/miss **only where `InferredField` is used**.
3. Treat pure-fn subsets as "recompute equals cache-disabled path" until policy settles.

Escalation owner: IIF coordinator / parent session — not this ticket.

---

## Subsets with `💡️inferences` today

**Repo total:** 72 `💡️inferences/` facet directories under `✏️s/🔌️plugins`.

**Stdio plugin:** 19 directories — all on **`✳️any`** (format-level artifacts), **zero on named profile subsets** (pdf/a, xml/valid, etc.).

### Stdio artifacts with `💡️inferences` (✳️any only)

| Artifact | Standard | Slug / notes |
|---|---|---|
| csv | rfc4180 | any |
| html | 5 | any |
| json | rfc8259 | any — **failing** `inference_default_law` |
| md | commonmark | outline — **failing** heading test |
| pdf | 1.4, 1.7 | any — **failing** `inference_default_law` |
| tsv | iana | any |
| txt | utf-8 | any |
| xml | 1.0 | any |
| svg | 1.1 | any |
| bmp | v3 | any |
| png | 1.2 | any |
| jpg | jfif-1.01 | any |
| tiff | 6.0 | any |
| gif | 87a, 89a | any |
| docx, xlsx, pptx | ecma-376 | any |

### Non-stdio plugins with `💡️inferences` (53 families)

Includes: puzzle (2d/3d/5d), block (2d/3d/5d), cad, gis (map/terrain), lowpoly, raster, fem (2d/3d), norm (en1990–en1999, din*, iso*, vdi*), flow, vcs, writer, trinity (jack/rewrite), dag, shooting, playground, present, sequence, mathematical, procedural2d/3d, process3d, remodel, note, energy, curate, forms, playbook, imperative, layout, cad-adjacent artifacts, etc.

**Known gap:** puzzle `🖐️5d` `🎛flat-position` is a re-export shim (anti-pattern) — IIF P1 deletes via snapshot slimming.

**Families with real `InferredField` (8):** gis gisterrain, gis gismap, lowpoly, cad, puzzle 3d, block 2d/5d/3d.

---

## IIF P2 stdio target roster (when unblocked)

Target **22 subsets** (revised from 34):

- `🧿️semio` v1: **11** (14 minus brep/drawing/mesh → DKM)
- geometry/BIM: 11 (ifc×2, step/ap214, dwg×2, dxf, stl, gltf, obj, ply, las)
- media: 4
- containers: 3
- bcf/epw: 2

**Overlap with this ticket:** both tickets touch stdio subset bodies. Sequencing:

1. UCAS roster frozen.
2. IIF fixes 5 failing any-level inference tests.
3. This ticket W3 references may add subset-owned inferences on derived profiles **without** changing spine.
4. IIF P2 fan-out completes remaining owning subsets.
5. IIF P3 taxonomy flip — then this ticket W6 may promote inference-related policies to high.

**Carve-out requested:** IIF asked UCAS to allow narrow edits to the 5 csv/html/json/pdf/md inference files (orthogonal to semio roster) — awaiting UCAS reply at audit time.

---

## Anti-patterns this ticket must not reintroduce

| Anti-pattern | Owner to fix | Subset ticket stance |
|---|---|---|
| `flatten_snapshot_inplace` writing derived poses into puzzle5d snapshot | IIF P1 | remove from roundtrip "authored snapshot" stages when migrating puzzle |
| trinity `recompute_derived` / `DerivedPropertyReadonly` in PropertyBag | IIF P1 | do not copy pattern into other graph artifacts |
| Identity inference / empty slug dir | this ticket policy | reject in W2 policy (medium) |
| Memoized `RefCell`/`Mutex` caches in apps (APA found 115) | APA → IIF escalation | convert to inference when touching plugin, not in W0 |

---

## Checklist before subset inference work

- [ ] Confirm IIF P3 policy decision on pure-fn vs `InferredField`
- [ ] Confirm UCAS roster frozen
- [ ] Do not add `💡️inferences` to taxonomy — wait for IIF flip
- [ ] For each subset: use existing registration pattern (`register_artifact_inferences` in engine/glue)
- [ ] Stage 7 tests: use IIF law names or behavior per P3 decision
- [ ] Leave DKM triple (`brep`, `drawing`, `mesh`) inference empty until DKM lands
