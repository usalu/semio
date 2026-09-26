# Norm Artifact Schema Pipeline — Cross-Cutting Audit (read-only)

**Executive summary.** Changing a norm family's persisted shape is a **schema-first, multi-facet edit**: the Rust snapshot struct (`#[derive(ArtifactSchema)]`) is the behavioural source of truth; **`🔣️.json` is the normative parity anchor** for the five `schemaFormats` leaves (🦀️/🟦️/🔗️/🛰️/🔣️); TS/GraphQL/Proto and all wire grammars (ksy/spicy/abnf/protocol.semio/grammar.semio/ebnf/g4) are **hand-maintained twins**, not auto-regenerated on every edit. Mutations are **derived from snapshot field taxonomy** (one `change-<scalar>` per root scalar; list/table ops for composed children like `q_k`), each mounted as a **mutation leaf** with its own facet subtree. Verification is layered: per-crate `nx run @semio-tech/norm-<family>-rs:{check,test}`, plugin-wide gates (`mutation-leaf-taxonomy-check`, `describe`, policy field-parity), and language-agnostic oracle tests (`🥒️.feature` + `🐍️.py`). Use an **isolated `CARGO_TARGET_DIR`** — shared `target/` lock contention is routine when multiple agents build.

Worked case: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990` (~303 files under `🏅️standards/🔖️1/🪆️subsets/✳️any/`).

---

## 1. Architecture — what owns what

| Layer | Owner path (en1990) | Role |
|-------|---------------------|------|
| Crate root | `⚖️en1990/🦀️.rs` | `mod` tree, composition (`q_k` child slot), re-exports |
| Artifact schema | `…/🧬️schema/🦀️.rs` | `En1990Artifact`, conversions, compliance helpers, `derive_artifact_facets!`, descriptor |
| Snapshot | `…/🧬️schema/📸️snapshot/🦀️.rs` | `En1990Snapshot` — **persisted subject**; DSL/Pack codecs; JSON bridge fns |
| Diff | `…/🧬️schema/🔺️diff/🦀️.rs` + runtime apply | Sparse delta + `MutationDiff` impl |
| Mutations | `…/🧬️schema/🧬️mutations/` | Closed enum + 10 leaf modules |
| Inferences | `…/🧬️schema/💡️inferences/` | `evaluate()` / outline; fourth schema family |
| Subset surface | `…/✳️any/{✏️editor,👁️viewer,📚️examples,🖼️assets,🚪️io,🔮️oracles,🧪️tests}` | Apps, fixtures, oracles |
| Package | `⚖️en1990/📦️packages/🦀️rust/` | `Cargo.toml`, `📜️script.ts`, `📋️project.json` |
| Registry | `⚖️en1990/📜️artifact-definition.json` | `id`, `rust_package`, `nx_project`, deps |

Plugin-wide (not per family):

| Asset | Path | Generated? | Command |
|-------|------|------------|---------|
| Plugin JSON descriptor | `📕️norm/🔣️.json` (~1.27 MB) | **Yes** | `nx run @semio-tech/norm-plugin:describe` → `bun …/📦️packages/🦀️rust/📜️script.ts describe` → `describePluginComponent(…, "semio-s-plugin-norm", …)` after `wasm32-wasip2` build |
| Plugin semio descriptor | `📕️norm/🛂️.descriptor.semio` (~306 KB) | **Yes** | same `describe` target |
| Mutation leaf taxonomy | `📕️norm/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json` | **Yes** (from Rust sources) | `nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` |
| Taxonomy check | — | verify | `nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-check` |

---

## 2. Facet inventory — generated vs handcrafted (en1990 `✳️any/`)

Legend: **H** = handcrafted (committed, edited by agent). **G** = generated (deterministic output; regenerate when stale). **M** = macro/derive at **Rust compile time** (not a file on disk).

### 2.1 Core schema family (`🧬️schema/`)

| File kind | Path pattern | H/G | Produced by |
|-----------|--------------|-----|-------------|
| 🦀️ Rust artifact | `🧬️schema/🦀️.rs` | H | Agent; `#[derive(ArtifactSchema)]` on `En1990Artifact` |
| 🟦️ TS | `🧬️schema/🟦️.ts` | H | Agent; must mirror `🔣️.json` fields |
| 🔗️ GraphQL | `🧬️schema/🔗️.graphql` | H | Agent |
| 🔣️ JSON Schema | `🧬️schema/🔣️.json` | H | Agent; **normative for field-parity policy** |
| 🛰️ Proto | `🧬️schema/🛰️.proto` | H | Agent |
| Descriptor fn | `en1990_artifact_schema_descriptor()` in `🦀️.rs` | H | `include_str!` of all leaves above + snapshot/diff/mutations aggregates |

Historical note: ticket `26/08/08/ARTIFACT-SCHEMA-FACETS` used **`🧪wave5-norm-en1990-1994-generate.py`** (and siblings) to bulk-emit the five schemaFormats leaves from a parsed `Document` struct. Current tree uses **flat emoji filenames** (`🦀️.rs` not `🦀️component.rs`) and **composed `q_k` child** — re-run those scripts only as a **starting scaffold**, then reconcile manually.

### 2.2 Snapshot facet (`🧬️schema/📸️snapshot/`)

| File kind | H/G | Notes |
|-----------|-----|-------|
| 🦀️ `📸️snapshot/🦀️.rs` | H | **`En1990Snapshot`** — edit here first for field changes; `#[child(kind = "s.stdio.semio")]` on `q_k` drives slot table via derive |
| 🟦️/🔗️/🔣️/🛰️ | H | Same five leaves; snapshot JSON omits UI-only fields |
| 📝️text/📖️.grammar.semio, 🔤️.ebnf, 🅰️.g4 | H | Normative text wire; `📝️text/🦀️.rs` wraps grammar |
| 💾️binary/🥋️.ksy, 🌶️.spicy, 🔠️.abnf, 📡️.protocol.semio | H | Parallel binary specs; `💾️binary/🦀️.rs` implements codec |
| JSON bridge | H | `encode_en1990_snapshot_json` / `decode_en1990_snapshot_json` in `📸️snapshot/🦀️.rs` |

### 2.3 Diff facet (`🧬️schema/🔺️diff/`)

| File kind | H/G | Notes |
|-----------|-----|-------|
| Schema leaves (5 formats) | H | Under `🔺️diff/` directly (no nested `🧬️schema/` in current layout) |
| Wire mirrors | H | Same ksy/spicy/abnf/grammar/ebnf/g4 pattern as snapshot |
| Apply runtime | H | `En1990Diff::apply` / `MutationDiff` in `🔺️diff/🦀️.rs` |

### 2.4 Mutations aggregate (`🧬️schema/🧬️mutations/`)

| File kind | H/G | Notes |
|-----------|-----|-------|
| `🧬️mutations/🦀️.rs` | H | `En1990Mutation` enum, `KINDS`, JSON apply/inverse bridges |
| Five aggregate leaves | H | `🧬️mutations/{🦀️,🟦️,🔗,🛰️,🔣}.` |
| 💾️binary + 📝️text wire | H | Closed op vocabulary codecs |
| `🧪️tests/🔬️kinds-catalog/🦀️.rs` | H | **`KINDS` ↔ enum ↔ oracle manifest** parity |
| `🧪️tests/🔬️fixture/🦀️.rs` | H | Reads committed fixture JSON |

### 2.5 Per-mutation leaf (`🧬️mutations/<emoji><slug>/`)

Each of en1990's 10 kinds follows:

```
⚓️change-permanent-action/
  🦀️.rs              # MutationKind impl + payload struct (#[mutation_leaf])
  🧬️schema/           # five leaves for payload
  🔺️diff/🦀️.rs       # diff emission
  ↩️inverse/🦀️.rs     # inverse chain
  🧪️tests/<fixture>/  # one scenario dir per committed case
```

Fixture corpus mirror: `…/✳️any/🧫️fixtures/🧬️mutations/<same-path>/` with `📸️snapshot/{⬅️before,➡️after}/🔣️.json`, `🦠️mutation/🔣️.json`, `🔺️diff/🔣️.json`, `🎯️outcome/🔣️.json`.

Taxonomy fixture `📕️norm/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json` rows include: `aggregateVariant`, `artifact`, `entity`, `kind`, `module`, `physicalLayout` (`direct`|`split`), `record`, `source`, `verb`. Regenerated from all `impl MutationKind<` sources under `🗿️artifacts/*/🧬️schema/🧬️mutations/*/🦀️.rs`.

### 2.6 Inferences (`🧬️schema/💡️inferences/`)

| File kind | H/G | Notes |
|-----------|-----|-------|
| Root `💡️inferences/🦀️.rs` | H | `En1990Inference`, `evaluate`/`infer`, inference descriptor |
| `🧾outline/` subtree | H | Derived outline from snapshot fields |
| Five leaves + wire | H | Same schemaFormats + binary/text pattern |
| `🧪️tests/🔬️compliance-report/` | H | Asserts numeric compliance outputs |

### 2.7 Examples, assets, IO, oracles, tests

| Area | H/G | Notes |
|------|-----|-------|
| `📚️examples/🏢️high-consequence-office/` | H | Example subject; update when snapshot shape changes |
| `🖼️assets/…/🗣️.dsl.semio`, `🎒️.pack.semio` | H | Committed encodings of example |
| `🚪️io/` | H | Compose/import hooks |
| `🔮️oracles/🔣️.json` | H | Mutation catalog referenced by `kinds-catalog` test |
| `🧪️tests/⚖️mutate-en1990-1/` | H | `🥒️.feature` + `🐍️.py` independent oracle + `🦀️.rs` subject adapter |
| Editor/viewer under `✳️any/` | H | Bind snapshot fields to UI; not schemaFormats |

### 2.8 Compile-time derives (not files)

| Mechanism | Where | Effect |
|-----------|-------|--------|
| `#[derive(ArtifactSchema)]` | snapshot/artifact/diff/inference structs | `ArtifactSchemaFields` metadata, child slots |
| `#[derive(dsl::Mutations)]` | `En1990Mutation` | KINDS order, wire op mapping |
| `#[derive(dsl::MutationLeaf)]` | leaf payloads | Leaf contract |
| `semio_framework_plugin::derive_artifact_facets!` | `🧬️schema/🦀️.rs:185` | Generates `En1990Builder`, `En1990Analyzer`, `En1990Composer` |

---

## 3. Cargo / nx identifiers

| Crate | Package name | nx project | Script entry |
|-------|--------------|------------|--------------|
| en1990 artifact | `semio-s-artifact-norm-en1990` | `@semio-tech/norm-en1990-rs` | `⚖️en1990/📦️packages/🦀️rust/📜️script.ts` |
| norm plugin (wasm) | `semio-s-plugin-norm` | `@semio-tech/norm-plugin` | `📕️norm/📦️packages/🦀️rust/📜️script.ts` |
| shared contract | `semio-s-artifact-norm-contract` | `@semio-tech/norm-artifact-contract-rs` | contract package script |
| TS package | — | `@semio-tech/norm-js` | `📕️norm/📦️packages/🟦️typescript/📜️script.ts` |

All 15 families: `semio-s-artifact-norm-{din4108,din16798,din18599,en1990,…,vdi3805}` per each `📜️artifact-definition.json`.

**Recommended isolated build/test** (from ticket practice, e.g. `26/07/18/NORM-TECHNOLOGY-ABSOLUTELY-FEATURE-COMPLETE`):

```bash
export CARGO_TARGET_DIR="/tmp/semio-norm-en1990-$$"
# or ticket-local: TICKET/🗑️generated/target-en1990

nx run @semio-tech/norm-en1990-rs:check
nx run @semio-tech/norm-en1990-rs:test
# equivalent:
bun "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/📜️script.ts" test
# cargo direct:
cargo test -p semio-s-artifact-norm-en1990 --manifest-path "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/Cargo.toml"
```

**Build duration (from ticket logs only — not re-measured here):**

| Scope | Evidence | Order of magnitude |
|-------|----------|-------------------|
| Full plugin tests | `26/08/08/ARTIFACT-SCHEMA-FACETS/🧪wave7-plugin-sweep.txt`: `semio-s-plugin-norm` **834 passed** | Compiles all 15 families + plugin shell; minutes on cold cache |
| 14-crate norm fleet (legacy names) | `26/07/18/NORM-TECHNOLOGY-…`: **214 tests** isolated target | Several minutes cold |
| Single family `check` | Much faster than full plugin; still pulls framework deps | Tens of seconds–minutes depending on cache |
| `describe` (wasm + descriptor) | Requires `cargo build -p semio-s-plugin-norm --target wasm32-wasip2 --profile wasm-dev` first | Heavy; avoid unless registry/descriptor work |

---

## 4. RECIPE — change snapshot fields / entities

### 4.1 Add/rename/remove a **scalar** snapshot field (e.g. new load parameter)

1. **Edit Rust snapshot** — `…/📸️snapshot/🦀️.rs`: add field with `#[state(artifact)]`, update `Default`, DSL/Pack if needed.
2. **Mirror artifact struct** — `…/🧬️schema/🦀️.rs`: same field on `En1990Artifact`; fix `to_snapshot` / `from_snapshot` / `set_snapshot`.
3. **Update five schema leaves** × **three facets** (artifact, snapshot, diff):
   - `🧬️schema/{🦀️,🟦️,🔗,🛰️,🔣}.`
   - `📸️snapshot/{🦀️,🟦️,🔗,🛰️,🔣}.`
   - `🔺️diff/{🦀️,🟦️,🔗,🛰️,🔣}.` + diff apply arms in `🔺️diff/🦀️.rs`.
4. **Wire codecs** — if field appears on wire: update `📸️snapshot/📝️text/*`, `💾️binary/*`, and mutation wire if referenced.
5. **Mutations** — add `change-<field>` leaf per `📓️derivation-rules.md` (en1990 pattern: one scalar → one change kind). Mount module in `🧬️mutations/🦀️.rs` enum + `KINDS`.
6. **Inferences** — `💡️inferences/🦀️.rs`: extend `InferenceFieldSpec.reads`; update `evaluate()` / outline if field affects compliance.
7. **Examples/assets** — refresh `📚️examples/…`, `🖼️assets/…/🗣️.dsl.semio` & `🎒️.pack.semio`.
8. **Fixtures** — update `🧫️fixtures/🧬️mutations/…` before/after JSON; add leaf test dir.
9. **Editor** — `✏️editor/…/📥️inputs` panels for editable field; localized labels.
10. **Oracles** — `🔮️oracles/🔣️.json` catalog if mutation kinds change.

### 4.2 Change **composed child** / nested table (en1990: `q_k` → `s.stdio.semio` table)

- Field is **`#[child(kind = "s.stdio.semio")]`**, not `Vec<QkEntry>` inline.
- Composition converters live in **`⚖️en1990/🦀️.rs`** (Composition region) — update when table shape changes.
- Mutations for lists: `insert-variable-action`, `remove-variable-action`, `reorder-variable-actions`, `change-variable-action-{category,value}` — adjust leaf logic, not necessarily add scalar `change-q_k`.
- Cross-plugin dependency: `semio-s-artifact-stdio-semio` (see `📜️artifact-definition.json`).

### 4.3 Remove a field

Reverse of §4.1; **remove** matching mutation kinds and purge fixtures/oracle catalog entries; run taxonomy generate.

---

## 5. RECIPE — add/remove mutations

1. Read **`📓️derivation-rules.md`** / **`📓️taxonomy.md`** under contract (referenced in `🧬️mutations/🦀️.rs` header).
2. Create leaf folder `🧬️mutations/<emoji><verb>-<entity>/` with payload struct + `MutationKind` impl (`SEMANTICS` verb/entity/kind/record).
3. Add `🧬️schema/🔣️.json` (+ four other leaves) for payload.
4. Implement `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, `🧪️tests/<scenario>/`.
5. Register variant on `En1990Mutation` enum; append to **`KINDS`** (order must match derive).
6. Copy fixture tree to `🧫️fixtures/🧬️mutations/…`.
7. Update `🔮️oracles/🔣️.json` kind list.
8. Regenerate & check taxonomy:
   ```bash
   nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate
   nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-check
   ```

---

## 6. RECIPE — regenerate vs handcraft checklist

After any schema edit, walk this table:

| Output | Action |
|--------|--------|
| Five schemaFormats leaves (× facets) | **Handcraft** — keep camelCase JSON, snake Rust, `@state` GraphQL aligned |
| `en1990_artifact_schema_descriptor()` | **Handcraft** — ensure `include_str!` paths match |
| Mutation taxonomy JSON | **Generate** — `mutation-leaf-taxonomy-generate` |
| Plugin `🔣️.json` + `🛂️.descriptor.semio` | **Generate** — `nx run @semio-tech/norm-plugin:describe` (after wasm build) |
| `derive_artifact_facets!` types | **Recompile** — automatic |
| wave5 Python scripts | **Optional scaffold** — `🧪wave5-norm-en1990-1994-generate.py` etc. in `26/08/08/ARTIFACT-SCHEMA-FACETS`; paths/naming outdated vs current tree |

Optional bulk scaffold (then manual reconcile):

```bash
python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS/🧪wave5-norm-en1990-1994-generate.py"
```

---

## 7. Verification commands (exact)

### 7.1 Single family (fast path)

```bash
export CARGO_TARGET_DIR="/tmp/semio-norm-en1990-$$"
nx run @semio-tech/norm-en1990-rs:check
nx run @semio-tech/norm-en1990-rs:test
nx run @semio-tech/norm-en1990-rs:test -- quick   # if supported via forwardAllArgs → SEMIO_TEST_LEVEL
```

### 7.2 Plugin-wide gates

```bash
nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-check
nx run @semio-tech/norm-plugin:results-window-config-source
nx run @semio-tech/norm-plugin:surface-render-source
nx run @semio-tech/norm-plugin:test-quick          # semio-s-plugin-norm, all families
```

### 7.3 Schema facet parity (repo policy)

Root router (`📜️script.ts`) runs artifact-schema policies on verify, including **field parity**: JSON Schema properties vs 🦀️/🟦️/🔗️/🛰️ extractors (`policyArtifactOwnershipFieldParity` in `🪪️ownership-field-parity/🟦️.ts`).

```bash
bun ./📜️script.ts verify    # includes artifact-schema facet policies
# or targeted:
bun ./📜️script.ts verify artifact-field-parity
```

### 7.4 TypeScript

```bash
nx run @semio-tech/norm-js:test
# runs bun test on retained-command-dispositions + descriptor cross-checks
```

### 7.5 Language-agnostic mutation oracle (en1990)

```bash
# Feature: …/🧪️tests/⚖️mutate-en1990-1/🥒️.feature
# Python oracle: …/🐍️.py (independent implementation)
# Rust subject: …/🦀️.rs (requires semio-s-plugin-norm / sut feature green)
cargo test -p semio-s-artifact-norm-en1990 --test mutate_en1990_1
```

Key parity tests inside crate:

| Test | File | Enforces |
|------|------|----------|
| `kinds_match_the_enum_and_the_catalog` | `🧬️mutations/🧪️tests/🔬️kinds-catalog/🦀️.rs` | `KINDS` ↔ `dsl::Mutations` ↔ oracle JSON |
| Compliance numerics | `🧪️tests/⚖️compliance/🦀️.rs` | DE/EN combination formulas |
| Inference report | `💡️inferences/🧪️tests/🔬️compliance-report/` | evaluate output |
| Leaf scenarios | `🧬️mutations/<leaf>/🧪️tests/<fixture>/` | before/mutation/after JSON |

### 7.6 Contract crate

```bash
nx run @semio-tech/norm-artifact-contract-rs:test
```

---

## 8. Pitfalls

1. **JSON vs GraphQL optionality** — ARTIFACT-SCHEMA-FACETS ticket recorded `artifact-schema/field-parity` breaches on GraphQL `!` vs JSON optional fields; GraphQL must match JSON cardinality.
2. **`KINDS` order** — Must match `#[derive(dsl::Mutations)]` declaration order; drifting breaks catalog tests silently if oracle not updated.
3. **`q_k` is not a Vec** — Do not model as inline JSON array; use child ref + table mutations.
4. **Forgotten diff apply arm** — New snapshot field needs `En1990Diff` optional field + apply branch.
5. **Inference reads list** — `InferenceFieldSpec.reads` must include new fields or caching/evaluate skips them.
6. **Taxonomy stale** — Adding a leaf without `mutation-leaf-taxonomy-generate` fails plugin check.
7. **Shared `target/` lock** — Always set `CARGO_TARGET_DIR` per agent/ticket.
8. **describe without wasm** — `component-budget-check` / registry expect built `semio-s-plugin-norm` wasm artifact.
9. **wave5 script paths** — Old generator expects `📘️en1990`, `🦀️component.rs`, `Document` struct — current emoji layout differs; blind re-run breaks tree.
10. **Plugin tests scope** — `@semio-tech/norm-en1990-rs:test` ≠ full cross-family integration; run plugin `test-quick` before closing wide changes.

---

## 9. en1990 current snapshot shape (reference)

`En1990Snapshot` fields (`📸️snapshot/🦀️.rs`):

| Field | Type | Notes |
|-------|------|-------|
| `g_k` | `f64` | permanent action |
| `q_k` | `En1990QkChild` | composed `s.stdio.semio` table child |
| `resistance_kn` | `f64` | design resistance |
| `consequence_class` | `u8` | CC1–CC3 |
| `annex` | `AnnexChoice` | DE/EN national annex |
| `seismic_a_ed_kn` | `f64` | 0 disables seismic check |

Mutations (10): `change-annex`, `change-permanent-action`, `change-resistance`, `change-consequence-class`, `change-seismic-action`, `insert-variable-action`, `remove-variable-action`, `change-variable-action-category`, `change-variable-action-value`, `reorder-variable-actions`.

---

## 10. Related tickets & scripts

| Resource | Path |
|----------|------|
| Schema facet wave | `.🧬semio/…/26/🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS/` |
| en1990–en1994 generator | `…/🧪wave5-norm-en1990-1994-generate.py` |
| en1995–en1999 generator | `…/🧪wave5-norm-en1995-1999-generate.py` |
| DIN/ISO generator | `…/🧪wave5-norm-din-iso-generate.py` |
| Policy rule source | `…/🧬️policy-rule-artifact-schemas.region.ts` |
| Norm feature-complete timing | `.🧬semio/…/26/🌙️07/☀️18/NORM-TECHNOLOGY-ABSOLUTELY-FEATURE-COMPLETE/` |

---

*Read-only audit for Wave A cross-cutting concern. No builds executed.*
