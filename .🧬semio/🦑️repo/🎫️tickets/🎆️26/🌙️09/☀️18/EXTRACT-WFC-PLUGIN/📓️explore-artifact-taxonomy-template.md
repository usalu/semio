# 📓️ How to author a new artifact — definitive template

Read-only exploration, 2026-09-18. Two reference artifacts studied on disk:

- **Reference A** — `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling` (newest, real-world domain, 35 mutations, 1 371 files)
- **Reference B** — `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly` (procedural, 9 mutations, 306 files) — **this is the WFC artifact and it already contains a 14 499-LOC WFC engine.** See §0.

All paths below are repo-relative to `/Users/ueli/Documents/semio`. Quote every path in shell: they contain emoji and variation selectors.

> **Session caveat.** The MCP servers `repo` and `semio` failed to connect in this session (`CONNECTION_CLOSED`), so any `client entity-emojis` CLI was **not** reachable. Everything in §5 was therefore verified against the on-disk taxonomy catalog instead, which is the authority those tools read anyway.

---

## 0. READ THIS FIRST — three corrections to the brief

### 0.1 `📸️snapshot` / `🧬️mutations` / `🔺️diff` / `💡️inferences` live **under** `🧬️schema`, not beside it

The brief's tree is wrong. The real subset shape (verified by `find` on both references, and by `🔣️taxonomy.json`'s `schemaChildDirs`) is:

```
🗿️artifacts/<emoji>name/
├── 🦀️.rs                                    ← artifact crate root (the whole #[path] module tree)
├── 📦️packages/🦀️rust/{Cargo.toml,📜️script.ts,📋️project.json}
└── 🏅️standards/🔖️1/
    ├── 🦀️.rs                                ← standard() -> StandardDeclaration   (A only; B omits it)
    └── 🪆️subsets/✳️any/
        ├── 🦀️.rs                            ← subset() -> SubsetDeclaration        (A only; B omits it)
        ├── 🧬️schema/
        │   ├── 🦀️.rs 🟦️.ts 🔗️.graphql 🔣️.json 🛰️.proto      ← the "artifact" facet
        │   ├── 📸️snapshot/  { 🦀️.rs 🟦️.ts 🔗️.graphql 🔣️.json 🛰️.proto, 📝️text/, 💾️binary/, 🧪️tests/ }
        │   ├── 🔺️diff/      { same five leaves, 📝️text/, 💾️binary/, 🧪️tests/ }
        │   ├── 🧬️mutations/ { same five leaves, 📝️text/, 💾️binary/, 🧪️tests/, <emoji>verb-noun/ … }
        │   └── 💡️inferences/{ same five leaves, 📝️text/, 💾️binary/, 🧪️tests/, sub-engines… }
        ├── 🚪️io/  { 🦀️.rs, 📥️import/🧩️deserializers/…, 📤️export/🧵️serializers/… }
        ├── ✏️editor/ { 🦀️.rs, 🎭️modes/, 🎮️commands/, 🎚️config/, 👥️presence/, 🫧️transient/ }
        ├── 👁️viewer/ { same child set }
        ├── 📚️examples/<emoji>slug/{🦀️.rs, 🟦️.ts, 🖼️assets/<emoji>slug/🗣️.dsl.semio, 🧪️tests/, 🧫️fixtures/}
        ├── 🔮️oracles/🔣️.json
        ├── 🧫️fixtures/{🧬️mutations/<mut>/<case>/…, 🧩️mount-contract/🔣️.json}
        └── 🧪️tests/{🧩️mount-contract/🦀️.rs, …}
```

Authority, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`:

```json
"schemaChildDirs":  ["📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"],
"subsetChildDirs":  ["🧬️schema", "🚪️io", "📚️examples", "👁️viewer", "✏️editor"],
"artifactChildDirs":["🧬️schema", "🚪️io", "📚️examples", "🔨️modules"],
"newArtifactChildDirs": ["🏅️standards"],
"representationDirs":   ["📝️text", "💾️binary"]
```

### 0.2 There is **no generator** for any schema sidecar. All 20 schema leaves are hand-authored.

This is the single most load-bearing fact for the five execution agents. It is stated as a heading in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/📓️w8-schema-regeneration.md:7`:

> "## 1. Authority direction: the JSON Schema leaf is hand-authored normative truth. There is no generator."

and `:25-31` of the same file:

> "**No generator exists.** The only schema-shaped codegen target is `@semio-tech/framework-schema:generate` … the entity catalog, not artifact schemas. No `writeFileSync` in `📜️script.ts` targets a schema leaf; no `schemars`/`JsonSchema`/`$defs` emission exists … `describe` regenerates the plugin **owner-root** `🔣️.json`/`🛂️.descriptor.semio`, a different artifact."

Corroborated independently by the repo-wide generated-output audit at
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️h-generated-ref.md:65-83`, whose owner table lists **no** generator whose output root is any `✏️s/🔌️plugins/**/🧬️schema/**` leaf. The only writers that touch `✏️s/` at all are the surface scaffolder (`📌️.empty.md` markers) and the plugin registry (`.vscode/launch.json` + 8 files).

**And it is enforced structurally, not merely by convention.** `authorArtifactScaffold` refuses to author any leaf whose basename is not a markdown marker or a `componentFileKinds` language leaf —
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️builder/🟦️.ts:148`:

```ts
const allowed = new Set([taxonomy.windowEmptyFacetFileKindId, ...Object.values(taxonomy.componentFileKinds)].map((kind) => canonicalPrimaryFilenameForKind(kind, taxonomy)));
```

`…/🏗️builder/🟦️.ts:156`:

```ts
if (typeof leaf.content !== "string" || !leaf.path.startsWith(`${authority.path}/`) || !allowed.has(basename(leaf.path)) || targets.has(leaf.path) || generatorContractIdsForOutputPath(leaf.path, taxonomy).length) throw new Error(`Invalid authored leaf request: ${leaf.path}`);
```

A `🔣️.json` / `🔗️.graphql` / `🛰️.proto` / `🥋️.ksy` sidecar **cannot** be machine-authored through this path at all. `generatorContracts` in `🔣️taxonomy.json` holds 23 entries and **none** has an `outputRoots` path inside `✏️s/🔌️plugins/**/🧬️schema/**`; 0 of 4 372 spec sidecars live under any `🤖️generated`/`🗑️generated` folder.

The repo says so in prose too. `📜️script.ts:15937-15945`:

```
🎫️ Handcrafted `.grammar.semio` / `.protocol.semio` specs per artifact facet (see ticket
`26/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`). …
const POLICY_GRAMMAR_FILE_ALLOWLIST = new Set<string>([]);
const POLICY_PROTOCOL_FILE_ALLOWLIST = new Set<string>([]);
```

`📜️script.ts:16799` — `"Handcrafted grammar program: each artifact facet must commit a normative .grammar.semio checked by dsl_grammar::Recognizer."`
`📜️script.ts:16815` — `"Handcrafted protocol program: pack/spr facets must commit .protocol.semio verified by verify_protocol_bytes."`
`📜️script.ts:23605` — `"${childRel}" is still the scaffolded placeholder grammar, not a handcrafted spec`
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🧩️facet-completeness/🟦️.ts:46` — the breach's solution text is literally `` `Add handcrafted ${leafRel}.` ``

What *is* mechanically derivable is listed in §1.3.

### 0.3 The WFC engine already exists inside Reference B

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/` — **89 files, 14 499 LOC of Rust**, with exactly the module split the five new artifacts imply:

```
⚖️weights  ⚠️error  ⛏️extract  ⛓️constraint  🀄️tiled  🆔️ids  🌊️flow  🌐️domain  🍰️chunk
🎛️bitset  🎲️sample  🎼️motif  🏁️outcome  🏗️model  🐾️trail  💼️job  💾️serial  📣️propagate
🔁️prop-ac3  🔄️prop-ac4  🔍️search  🔗️constraints-conn  🔢️constraints-card  🔦️beam  🔧️repair
🔮️oracles  🔲️grid-2d  🔳️solver-grid-2d  🕳️sparse-3d  🕸️solver-graph  🗺️topology  🚫️nogood
🧊️grid-3d  🧬️evolve  🧭️heuristics  🧱️solver-grid-3d  🧵️parallel  🩺️diag  🪜️hierarchy
🪞️symmetry  🪶️soft  🧪️tests  🧫️fixtures
```

**Caution:** most of these are `#[cfg(test)]`-gated in `🧩️wfc-engine/🦀️.rs` and therefore compile **only** under `cargo test` — they are dead in a release build today:

```rust
// 🧩️wfc-engine/🦀️.rs:1-20
//! 🧩️ Wave function collapse implementation used by assembly inferences.

#[cfg(test)]
#[path = "🔦️beam/🦀️.rs"]
pub(crate) mod beam;
#[path = "🎛️bitset/🦀️.rs"]
pub mod bitset;
#[cfg(test)]
#[path = "🍰️chunk/🦀️.rs"]
pub(crate) mod chunk;
#[cfg(test)]
#[path = "⛓️constraint/🦀️.rs"]
pub mod constraint;
```

The five new `🌊️wfc` artifacts (`bitmap`, `2d-grid`, `2d`, `3d-grid`, `3d`) map onto `🔲️grid-2d`/`🔳️solver-grid-2d`, `🧊️grid-3d`/`🧱️solver-grid-3d`, `🕳️sparse-3d`, `🕸️solver-graph`, `🀄️tiled`. Read these before authoring anything new.

---

## 1. File-tree comparison and per-file-kind provenance

### 1.1 Census by basename (excluding build output)

| basename | A `📸️remodeling` | B `🧩️assembly` | kind |
|---|---:|---:|---|
| `🦀️.rs` | 421 | 158 | Rust source |
| `🔣️.json` | 732 | 79 | JSON (schema / descriptor / fixture) |
| `🟦️.ts` | 49 | 19 | TypeScript mirror |
| `📌️.empty.md` | 40 | 16 | empty-facet marker |
| `🚫️.absent` | 38 | 0 | deliberate-absence marker |
| `🛰️.proto` | 12 | 7 | protobuf |
| `🔗️.graphql` | 12 | 7 | GraphQL |
| `📖️.grammar.semio` | 5 | 2 | normative text grammar |
| `🔤️.ebnf` | 4 | 2 | EBNF sidecar |
| `🅰️.g4` | 4 | 2 | ANTLR sidecar |
| `🔠️.abnf` | 4 | 2 | ABNF sidecar |
| `🥋️.ksy` | 4 | 2 | Kaitai Struct |
| `🌶️.spicy` | 4 | 2 | Zeek Spicy |
| `📡️.protocol.semio` | 4 | 2 | normative binary protocol |
| `🗣️.dsl.semio` | 2 | 2 | example document asset |
| `🥒️.feature` | 1 | 2 | Gherkin |
| `🐍️.py` | 1 | 2 | Python reference impl |
| `Cargo.toml` `📜️script.ts` `📋️project.json` | 1 each | 1 each | package |
| media (`.png/.obj/.ply/.stl`) | 40 | 0 | example assets |

`📸️remodeling` additionally carries five Rust-only compute-internals directories under `🧬️schema/` (`➕️algebra-internals`, `🔷️lie-internals`, `🎯️optimize-internals`, `📶️signal-internals`, `🗺️spatial-internals`) that were relocated from `semio-framework-math`. `🧩️assembly` carries `💡️inferences/🧩️wfc-engine` instead. Both are the same precedent: **compute internals live under `🧬️schema/`, never in `✏️editor`**.

### 1.2 Provenance table — **generated vs handcrafted, per file kind**

| file kind | provenance | command if any | gate that validates it |
|---|---|---|---|
| `🦀️.rs` (schema/snapshot/diff/mutations/inferences/io/editor/viewer) | **handcrafted** | — | `cargo check -p <crate> --lib --tests`; `bun nx run workspace:verify-taxonomy-implementation-enforce` |
| `🦀️.rs` (mutation leaf stub only) | **scaffolded once, then hand-finished** | `bun ./📜️script.ts new mutation <mutations-root> <emoji-name> [flags]` | as above |
| `🔣️.json` — facet JSON Schema (`🧬️schema/🔣️.json`, `📸️snapshot/🔣️.json`, `🔺️diff/🔣️.json`, `🧬️mutations/🔣️.json`, `💡️inferences/🔣️.json`) | **handcrafted, NORMATIVE** | none | `bun nx run workspace:schema-check` / `:schema-verify`; `bun ./📜️script.ts verify artifact-field-parity enforce` |
| `🔣️.json` — per-mutation descriptor (`🧬️mutations/<mut>/🔣️.json`) | **scaffolded then hand-edited** | `new mutation` writes it | `mutationPayloadSchemaAuthority` contract in taxonomy; const-assert in `dsl::MutationLeaf` derive (see §3.6) |
| `🔣️.json` — per-mutation payload schema (`🧬️mutations/<mut>/🧬️schema/🔣️.json`) | **handcrafted** (scaffold writes a 3-line stub with `--json-schema`) | — | `schema-check`, payload-schema parity |
| `🔣️.json` — fixtures | **handcrafted or ticket-local Python** | no repo generator; A used ticket-local `🐍️w2c-generate.py --apply` | the per-case `🦀️.rs` test (§3.7) |
| `🔣️.json` — `🔮️oracles/🔣️.json` | **handcrafted** | — | `bun ./📜️script.ts contract` (nx `test-contract`), writes `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`, exit 1 on breach |
| `🟦️.ts` | **handcrafted mirror** | — | `bun nx run @semio-tech/<plugin>-js:test`; `bunx tsc --noEmit` |
| `🔗️.graphql` | **handcrafted mirror** | — | `artifact-field-parity` |
| `🛰️.proto` | **handcrafted mirror** | — | `artifact-field-parity` |
| `📖️.grammar.semio` | **handcrafted, NORMATIVE for text** | — | ticket-local `🐍️grammar-check.py`; runtime `parsed_grammar()` at load |
| `🔤️.ebnf`, `🅰️.g4` | **derived from `📖️.grammar.semio`** | ticket-local `python3 🐍️grammar-sidecars.py` (kebab → "space case" for ebnf, kebab → camelCase for g4). Re-running produced byte-identical output. | `🐍️grammar-check.py` |
| `📡️.protocol.semio` | **handcrafted, NORMATIVE for binary** | — | `🐍️grammar-check.py` binary lane |
| `🔠️.abnf`, `🥋️.ksy`, `🌶️.spicy` | **handcrafted sidecars of `📡️.protocol.semio`** | — | same |
| `🗣️.dsl.semio` (example asset) | **printed from the Rust example builder** | A: `bun ./📜️script.ts regenerate-example` (from `📦️packages/🦀️rust`), also `nx run @semio-tech/remodel-plugin:regenerate-example` | `🧩️mount-contract` test round-trips it |
| `🥒️.feature` | **handcrafted** (A regenerated it from the vector table) | — | `bun ./📜️script.ts contract` |
| `🐍️.py` (oracle reference) | **handcrafted second implementation** | — | `bun ./📜️script.ts contract`; runner `python3 🐍️.py --plan <plan.json> --out <results.jsonl> --adapter <component.py>` |
| `📌️.empty.md` | **generated scaffold** | `bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts new surface --all [--dry-run]` | plugin-registry `:check` |
| `🚫️.absent` | **handcrafted, zero bytes** | — | fixture harness treats it as "this facet is deliberately absent" |
| plugin-root `🔣️.json` + `🛂️.descriptor.semio` | **GENERATED** | `bun nx run @semio-tech/<plugin>-plugin:describe` | plugin registry `:check` byte-compares |

Literal `📌️.empty.md` body (identical everywhere):

```markdown
# Empty Surface config Facet

This facet currently declares no specific items. Generated by `bun ./📜️script.ts new surface`.
```

`🚫️.absent` is a **zero-byte** file. Example path:
`…/✳️any/🧫️fixtures/🧬️mutations/🪓delete-stream/🚫️refuses-to-3c20ff/🔺️diff/🚫️.absent` — a rejected mutation has no diff, so the absence is stated rather than left implicit.

### 1.3 Scaffolders — the only writers into `✏️s/`

Router registration: `📜️script.ts:15162` → `.register("new", CleanMechanismNewScript)`, imported at `📜️script.ts:196` from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts`.

Literal usage block, `🏗️authoring/🎮️command/🟦️.ts:33-36`:

```
usage: bun ./📜️script.ts new artifact <plugin> <new-artifact-dir>
   or: bun ./📜️script.ts new standard <plugin> <artifact-kind> <new-standard-dir>
   or: bun ./📜️script.ts new subset <plugin> <artifact-kind> <standard> <new-subset-dir> [--dry-run]
   or: bun ./📜️script.ts new mutation <owner-mutation-root> <emoji-semantic-name> [--composite] [--text] [--binary] [--typescript] [--graphql] [--protobuf] [--json-schema] [--dry-run]
```

Known flags (`:43`): `--dry-run --composite --text --binary --typescript --graphql --protobuf --json-schema`. Any other flag throws.

**What each emits** (`🏗️authoring/🗿️artifact-tree/🟦️.ts`):

- `new artifact` (`:80-89`) — **only two files**: `<artRel>/🦀️.rs` and `<artRel>/🟦️.ts`.
- `new standard` (`:66-78`) — `🦀️.rs`, `🟦️.ts`, and `🪆️subsets/🔣️.json` containing `{"standard": "<stripped>", "subsets": {"*": {}}}`.
- `new subset` (`:47-64`) — the full skeleton: subset-root `🦀️.rs`/`🟦️.ts`, `🧬️schema/🦀️.rs`/`🟦️.ts`, the whole `🚪️io` tree, one `🦀️.rs`+`🟦️.ts` per surface role (`👁️viewer`, `✏️editor`), and `📚️examples/📌️.empty.md`.
- `new mutation` (`🏗️authoring/🧬️mutation-tree/🟦️.ts:221-295`) — see §3.

**Recommended order for a new artifact:**

```zsh
cd /Users/ueli/Documents/semio
bun ./📜️script.ts new artifact 🌊️wfc "🖼️bitmap"            --dry-run   # inspect first
bun ./📜️script.ts new artifact 🌊️wfc "🖼️bitmap"
bun ./📜️script.ts new standard 🌊️wfc "🖼️bitmap" "🔖️1"
bun ./📜️script.ts new subset   🌊️wfc "🖼️bitmap" "🔖️1" "✳️any"
bun ./📜️script.ts new mutation "✏️s/🔌️plugins/🌊️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations" "🎲️change-seed" --json-schema
```

Note `new artifact`/`new standard` resolve `<plugin>`/`<artifact-kind>` **emoji-tolerantly** (`newResolveChildDir` strips emoji and compares, `🎮️command/🟦️.ts:9-16`), but the FINAL new segment is taken **literally** — it must already carry its emoji prefix, and for `standard`/`subset` the taxonomy's dir prefix (`🔖️` / `✳️`).

### 1.4 Gates — exact commands

All are workspace-level nx targets defined in `/Users/ueli/Documents/semio/📋️project.json`:

| gate | command | project.json line |
|---|---|---|
| schema catalog generate | `bun nx run workspace:schema-generate` → `bun ./📜️script.ts schema generate` | 853-866 |
| schema check | `bun nx run workspace:schema-check` → `bun ./📜️script.ts schema check` | 880-891 |
| schema verify | `bun nx run workspace:schema-verify` → `bun ./📜️script.ts schema verify` | 892-903 |
| schema audit | `bun nx run workspace:schema-audit` | 904-912 |
| schema oracle | `bun nx run workspace:schema-oracle` | 913-920 |
| taxonomy report | `bun ./📜️script.ts verify taxonomy report` | 1400 area |
| taxonomy enforce | `bun ./📜️script.ts verify taxonomy enforce` | 1405-1411 |
| taxonomy implementation enforce | `bun ./📜️script.ts verify taxonomy implementation enforce` | 1420-1426 |
| artifact field parity | `bun ./📜️script.ts verify artifact-field-parity enforce` (also `… report`, `… test`) | — |
| artifact contract ownership | `bun ./📜️script.ts verify artifact-contract-ownership` | — |
| dependency truth | `bun ./📜️script.ts verify dependencies` | — |
| test contract (oracles/fixtures) | `bun ./📜️script.ts contract` (nx `test-contract`) | — |
| mutation outcome law | `bun ./📜️script.ts verify mutation-outcome-law` | — |
| plugin registry | `bun nx run @semio-tech/plugin-registry:generate`; check with `:check` | — |
| plugin descriptor | `bun nx run @semio-tech/<plugin>-plugin:describe` | — |

**`bun ./📜️script.ts verify gate`** (nx `workspace:verify-gate`, `runGate()` at `📜️script.ts:8474`) is the aggregate pre-close gate. Its stages, with the console labels it prints:

```
[verify] dependency-cruiser boundaries…                     :8480
[verify] generated catalog freshness…                       :8482  → bun nx run @semio-tech/plugin-registry:check
[verify] framework owned-schema binding freshness…          :8488  → bun nx run @semio-tech/framework-rs:check
[verify] handcrafted grammar P3/M4 policies…                :8539  → policyHandcraftedSpecP3Breaches (:20514)
[verify] artifact-schema facet policies…                    :8551  → policyArtifactSchemaBreaches
[verify] app-schema facet policies…                         :8561  → policyAppSchemaBreaches
[verify] standards/subsets vocabulary…
[verify] window capability taxonomy…
[verify] mutation-outcome / merge-policy law…
[verify] indexed generated output (wasm/jco/wasm-pack)…
[verify] gate passed.                                       :8634
```

Bare `bun ./📜️script.ts verify` runs the gate **plus** `nx run-many -t test --all`.

**Single-rule policy gates** — `bun ./📜️script.ts verify policy-breach <rule>` (`runPolicyBreach`, `📜️script.ts:8100`; rule table `POLICY_BREACH_GATES` at `:6869-6882`). The 12 rules:

```
artifact-builder · artifact-decomposer · schema-representation · io-serializer-matrix
io-terminality · codec-fidelity · standards-coverage · artifact-analyzer · artifact-composer
artifact-builder-migrated · plugin-dependency-parity · contribution-target
```

An unknown rule prints `` [verify policy-breach] unknown rule "x". known: … ``.

**Schema subcommand set** (`SchemaScript`, `📜️script.ts:14872`, dispatch `:14874-14885`) — the error string enumerates them:

```
unknown schema subcommand: … (expected audit | check | compile | docs | entries | generate | oracle | test | verify)
```

`schema generate` writes **only** the repo catalog `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` (taxonomy `schemaExportResolution.catalogPath`, whose own `"generator"` field is the literal string `"bun ./📜️script.ts schema generate"`). It never touches an artifact sidecar. `bun ./📜️script.ts schema generate --check` verifies freshness.

Per-artifact crate targets (`📋️project.json` inside the artifact package):

```zsh
bun nx run @semio-tech/procedural-assembly-rs:check   # -> bun ./📜️script.ts check   in 📦️packages/🦀️rust
bun nx run @semio-tech/procedural-assembly-rs:test
bun nx run @semio-tech/procedural-assembly-rs:build
```

Direct cargo (from the REMODEL ticket's own scripts — note the SIGKILL-137 retry loop those scripts wrap around every call):

```zsh
cargo check -p semio-s-artifact-remodel-remodeling --lib --tests -j 4 --message-format=short
cargo test  -p semio-s-artifact-remodel-remodeling --lib -j 4 -- --test-threads=4
cargo check -p semio-s-plugin-remodel --lib --target wasm32-wasip2     # type-checks even when the native host is broken
```

**macOS has no `timeout`.** Every gated `timeout N cargo …` line silently fails. Use the retry-loop script shape instead.

---

## 2. Rust artifact crate contract

Reference B's crate root is 446 lines and is the cleanest complete model. Reference A's is 2 742 lines because it also carries the relocated math internals and the durable-content/asset machinery.

### 2.1 Crate-root preamble

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🦀️.rs:1-18`

```rust
//! 🧩️ Assembly artifact — a WaveFunctionCollapse-style rule/slot composition engine. …

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::ASSEMBLY_DOCUMENT_SCHEMA;

#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🦀️.rs"]
pub(crate) mod wfc_engine;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
```

Reference A's equivalent (`📸️remodeling/🦀️.rs:12-15`) aliases a fourth name and pulls in the schema crate:

```rust
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
```

The four aliases are **mandatory** — every leaf file in the tree says `protocol::MutationOutcome`, `store::ArtifactChild`, `dsl::MutationLeaf`, `vcs::apply_mutation`, and they all resolve to `semio_framework_os_kernel`.

### 2.2 `ASSEMBLY_DIALECT` — the surface-id coordinate

`🧩️assembly/🦀️.rs:31`

```rust
pub const ASSEMBLY_DIALECT: Dialect = Dialect { artifact_kind: ASSEMBLY_DOCUMENT_SCHEMA, standard: StandardId("1"), subset: SubsetId::ANY };
```

`📸️remodeling/🦀️.rs:143`

```rust
pub const REMODELING_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.remodel.remodeling", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };
```

Its own doc comment (`📸️remodeling/🦀️.rs:134-142`) is the normative naming rule:

> "canonical surface id grammar (`<artifact_kind>@<standard>/<subset>#<role>`). Lives at the ARTIFACT level (not under `editor`/`viewer`) so a viewer file can read it without ever importing through the sibling editor module. `artifact_kind = "s.remodel.remodeling"` is `s.<plugin-id>.<artifact-name>`, the fleet-wide dialect grammar (`s.trinity.jack`, `s.puzzle.puzzle3d`, `s.block.block2d`, `s.procedural.generation2d`) … It is NOT `artifact_kind()`'s OS-level `"3d.remodeling"` kind id (a different, unrelated namespace)."

**Trap in Reference B:** assembly's schema tree was authored with a *bare* `"s.assembly"` id, not `"s.procedural.assembly"`, and the crate root followed it rather than overriding (`🦀️.rs:21-30`). So B has **two** ids in play: `"s.assembly"` (dialect/schema/codec) and `"s.procedural.assembly"` (the `ArtifactDefinition` identity). **For the new `🌊️wfc` artifacts, use the regular `s.wfc.<artifact>` form throughout** — do not copy assembly's exception.

### 2.3 `artifact_kind()` — OS-level kind spec

`🧩️assembly/🦀️.rs:39-54`

```rust
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "data.assembly".into(),
        name: "Assembly".into(),
        source_format: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        component_kind: "assembly".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.txt".into()],
    }
}
```

`📸️remodeling/🦀️.rs:42-57`

```rust
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.remodeling".into(),
        name: "3D Remodeling".into(),
        source_format: "remodeling.scene".into(),
        component_kind: "remodeling".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: "remodeling.scene".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), /* … */],
    }
}
```

`id` is `<dimension>.<name>` — `data.assembly`, `3d.remodeling`. For the WFC set: `2d.wfcBitmap`-style or `data.*` depending on whether the artifact renders. `dimension` values observed: `"data"`, `"2d"`, `"3d"`.

### 2.4 `*_SCHEMA` const and the `Document` (snapshot) type

`…/🧬️schema/📸️snapshot/🦀️.rs:15`

```rust
pub const ASSEMBLY_DOCUMENT_SCHEMA: &str = "s.assembly";
```

`…/🧬️schema/📸️snapshot/🦀️.rs:72-106` — the complete snapshot contract:

```rust
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.assembly")]
pub struct AssemblySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub seed: u64,
    #[state(artifact)]
    #[value(default)]
    pub slots: Vec<AssemblySlot>,
    #[state(artifact)]
    #[value(default)]
    pub edges: Vec<AssemblySlotEdge>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub modules: Vec<store::ArtifactChild<SemioKitSnapshot>>,
    #[state(artifact)]
    #[value(default)]
    pub weights: Vec<AssemblyModuleWeight>,
    #[state(artifact)]
    #[value(default)]
    pub rules: Vec<AssemblyRule>,
}

impl Default for AssemblySnapshot {
    fn default() -> Self {
        Self { schema: ASSEMBLY_DOCUMENT_SCHEMA.into(), seed: 0, slots: Vec::new(), edges: Vec::new(), modules: Vec::new(), weights: Vec::new(), rules: Vec::new() }
    }
}
```

Rules an author must reproduce:

- first field is always `pub schema: String`, defaulting to the `*_DOCUMENT_SCHEMA` const;
- every field carries `#[state(artifact)]` (the state-lane marker);
- collection fields carry `#[value(default)]`;
- a child-artifact reference is `Vec<store::ArtifactChild<T>>` with `#[child(kind = "<dialect artifact kind>")]`;
- the container carries `#[value(rename_all = "camelCase")]` and `#[artifact_schema(id = "…")]`;
- `Default` is written by hand (not derived) so `schema` is populated.

Leaf records (`📸️snapshot/🦀️.rs:21-69`) use `#[derive(Clone, Debug, PartialEq, ToValue, FromValue, Default)]` + `#[value(rename_all = "camelCase")]`, and optional fields use `#[value(default, skip_serializing_if = "Option::is_none")]`.

### 2.5 `semio_framework_value_derive` — the ONLY serialization derive

`use semio_framework_value_derive::{FromValue, ToValue};` appears in every schema/mutation/diff/inference leaf in both references. `serde::{Serialize, Deserialize}` appears **only** in Reference A's crate root for the legacy packed-asset types; the assembly tree carries **no serde derives at all**, and the remodel ticket (`📓️w9-schema-engine-compile.md:115-130`) records a deliberate serde→`pack::json` cutover.

Gotcha recorded at `📓️w9-schema-engine-compile.md:97-98`:

> "`RemodelingMesh`'s `#[value(default)]` container attribute made `FromValue` demand `ArtifactChild: Default` (value_derive's container default is PER FIELD, unlike serde's `Self::default()`)."

### 2.6 `pack` encoding — `encode_record_body`, never `encode_json_value`

**Verified directly.** `grep -rn "encode_json_value"` across both plugins returns **zero** hits. `encode_record_body` / `decode_record_body` are the calls actually used:

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:132`

```rust
let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
```

`…/🎚️config/🦀️.rs:151`

```rust
let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
```

At the **document** level the artifact never calls pack directly; it goes through the `ArtifactPack` trait:

`…/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:12-20`

```rust
/// 📦️ Encodes an `AssemblySnapshot` to its binary pack form.
pub fn encode(document: &AssemblySnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes an `AssemblySnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<AssemblySnapshot, PackError> {
    <AssemblySnapshot as store::ArtifactPack>::decode_pack(bytes)
}
```

The wire layout the `📡️.protocol.semio`/`🥋️.ksy`/`🌶️.spicy` sidecars must describe is stated in `📓️w10-grammars.md:57-66`:

> "`os_pack::encode_record_fields` … = `field_count varint, field_count × (field_id varint, tagged value)`, **sorted by ascending field id**, `Absent` never written (that sort is the byte-determinism LAW). Value tags are a closed alphabet `0x00..0x17` … `#[derive(dsl::DslRecord)]` assigns `id = 0-based declaration index` … `encode_record_body` inlines the symbol table … Value bridge = `encode_record_body` of `value_bridge_spec()`: field id `1`, `Shape::Value`, tag `0x11`, one `encode_dsl_value` node."

### 2.7 Snapshot retirement — required for every artifact with collections

`…/📸️snapshot/💾️binary/🦀️.rs:22-60`. A displaced snapshot must be released in **bounded steps**, not dropped:

```rust
/// 🧮️ One retirement step's budget unit …
const ASSEMBLY_RETIREMENT_STEP_BYTES: usize = 4_096;

/// ♻️ The five collections a displaced `AssemblySnapshot` releases, in release order. `modules`
/// holds `store::ArtifactChild` handles into the kit store, so it is released FIRST: a child handle
/// outliving the document that named it is the one leak this artifact can actually have.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AssemblyRetirementStage { Modules, Rules, Weights, Edges, Slots }

impl AssemblyRetirementStage {
    const ORDER: [Self; 5] = [Self::Modules, Self::Rules, Self::Weights, Self::Edges, Self::Slots];
}

pub struct AssemblySnapshotRetirement {
    displaced: std::mem::ManuallyDrop<AssemblySnapshot>,
    stage: usize,
}

impl ErasedSnapshotRetirement for AssemblySnapshotRetirement { fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> { … } }
```

Child handles are released **first**. `Drop` asserts the terminal stage was reached.

### 2.8 `🧬️schema/🦀️.rs` — the artifact facet + the 20-leaf descriptor

`…/🧬️schema/🦀️.rs:7-64`

```rust
/// 🧬️ AssemblyArtifact facet — the persisted problem spec is the artifact.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.assembly")]
pub struct AssemblyArtifact {
    #[state(artifact)]
    pub snapshot: AssemblySnapshot,
}

/// 🧬️ Descriptor for `s.assembly` — twenty handcrafted schema leaves.
pub fn assembly_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.assembly",
        artifact: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema::FacetLeaves { rust: include_str!("🔺️diff/🦀️.rs"), typescript: include_str!("🔺️diff/🟦️.ts"), graphql: include_str!("🔺️diff/🔗️.graphql"), json_schema: include_str!("🔺️diff/🔣️.json"), proto: include_str!("🔺️diff/🛰️.proto") },
        mutations: ::semio_framework_schema::FacetLeaves { rust: include_str!("🧬️mutations/🦀️.rs"), typescript: include_str!("🧬️mutations/🟦️.ts"), graphql: include_str!("🧬️mutations/🔗️.graphql"), json_schema: include_str!("🧬️mutations/🔣️.json"), proto: include_str!("🧬️mutations/🛰️.proto") },
    }
}
```

**4 facets × 5 languages = 20 `include_str!` leaves that must all exist or the crate does not compile.** `ArtifactDeclaration::builder(…).schema(descriptor)` is typestate-MANDATORY — the builder cannot reach `.try_build()` without it.

> **Stale comment warning.** `🧩️assembly/🦀️.rs:119-133` still claims "no `🔣️.json`/`🔗️.graphql`/`🛰️.proto` anywhere in this artifact's schema tree". That is **no longer true** — all 20 leaves exist on disk today and `assembly_artifact_schema_descriptor()` references them. Do not trust that comment.

The same file also carries three derived facets an author must supply:

- `derived_construction` — `impl ArtifactBuilder` with `empty/from_snapshot/from_text/from_binary/mutate/absorb/build` (`🧬️schema/🦀️.rs:68-116`);
- `derived_analysis` — `impl ArtifactAnalysis` with `const DIALECT`, `sniff`, `analyze` (`:120-164`);
- the macro that welds them (`:169-178`):

```rust
semio_framework_plugin::derive_artifact_facets!(
    pub spec AssemblyBuilderFacets {
        construction: AssemblyBuilderConstruction,
        analysis: AssemblyAnalyzerAnalysis,
        composition: super::super::io::derived_composition::AssemblyComposerComposition,
    }
    builder: AssemblyBuilder,
    analyzer: AssemblyAnalyzer,
    composer: AssemblyComposer,
);
```

### 2.9 `definition()` and `declaration()`

Reference B spells capabilities out (`🧩️assembly/🦀️.rs:72-117`):

```rust
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural.assembly")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.assembly")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.assembly")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.assembly.solve")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.assembly.solve")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.assembly@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.assembly@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"s.assembly:assembly")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "s.assembly")?)?
                .claim(ArtifactIdentityClaim::codec_extension("s.assembly", "assembly")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Assembly")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Assembly")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Montage")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Montage")?)?,
        )
}
```

Reference A uses a compact **row table** instead — preferable when there are many capabilities (`📸️remodeling/🦀️.rs:71-106`):

```rust
let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
    ("s.remodel.remodeling.standard.v1", "standard", "1", &[], None),
    ("s.remodel.remodeling.standard.v1.profile.any", "profile", "any", &[], None),
    ("s.remodel.remodeling.schema.artifact", "schema", "s.remodel.remodeling", &[("schema", "s.remodel.remodeling")], None),
    ("s.remodel.remodeling.inference.artifact", "inference", "s.remodel.remodeling.inference", &[("schema", "s.remodel.remodeling.inference")], None),
    ("s.remodel.remodeling.composer.native", "composer", "s.remodel.remodeling@1/*", &[("dialect", "s.remodel.remodeling@1/*")], None),
    ("s.remodel.remodeling.composer.format-1", "composer", "s.stdio.las@1.0/*", &[("dialect", "s.stdio.las@1.0/*")], None),
    /* … format-2 … format-8 … */
    ("s.remodel.remodeling.grammar.1", "grammar", "remodeling.document", &[("grammar", "remodeling.document")], None),
    ("s.remodel.remodeling.grammar.2", "grammar", "remodeling.op", &[("grammar", "remodeling.op")], None),
    ("s.remodel.remodeling.grammar.3", "grammar", "remodeling.diff", &[("grammar", "remodeling.diff")], None),
    ("s.remodel.remodeling.grammar.4", "grammar", "remodeling.pack", &[("grammar", "remodeling.pack")], None),
    ("s.remodel.remodeling.grammar.5", "grammar", "remodeling.spr", &[("grammar", "remodeling.spr")], None),
    ("s.remodel.remodeling.codec.document-1", "codec", "remodeling.scene:remodeling", &[("codec", "remodeling.scene"), ("codec-extension", "16:remodeling.scene:remodeling")], None),
    ("s.remodel.remodeling.localization.en", "localization", "Remodeling", &[], Some(("en", "Remodeling"))),
    ("s.remodel.remodeling.localization.de", "localization", "Umbau", &[], Some(("de", "Umbau"))),
];
let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.remodel.remodeling")?);
for (identity, kind, descriptor, claims, localization) in rows {
    let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
    for (namespace, value) in *claims { capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?; }
    if let Some((locale, text)) = localization { capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?; }
    definition = definition.capability(capability)?;
}
Ok(definition)
```

`declaration()` (`🧩️assembly/🦀️.rs:109-117`) — **note the feature gate**:

```rust
/// 🔖️ Assembles `s.procedural.assembly`'s typed runtime declaration.
#[cfg(feature = "component-app-assembly")]
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(standards::v1::subsets::any::schema::assembly_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::assembly_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec_bare::<AssemblySnapshot, AssemblyMutation>(ASSEMBLY_DOCUMENT_SCHEMA)
        .try_build()
}
```

### 2.10 `component-app-assembly` feature gating

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/Cargo.toml` — **complete**:

```toml
[package]
name = "semio-s-artifact-procedural-assembly"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Procedural assembly artifact"

[package.metadata.semio]
role = "s-module"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[features]
default = []
component-app-assembly = ["semio-framework-plugin/component-guest", "dep:semio-framework-os-flow", "dep:semio-framework-os-infinite", "dep:semio-framework-ui", "dep:semio-framework-ui-contract", "dep:semio-framework-ui-styling"]

[dependencies]
semio-framework = { workspace = true }
semio-framework-artifact-flow-flow = { workspace = true }
semio-framework-geometry = { workspace = true }
semio-framework-graph = { workspace = true }
semio-framework-job = { workspace = true }
semio-framework-os-kernel = { workspace = true }
semio-framework-plugin = { workspace = true }
semio-framework-schema = { workspace = true }
semio-framework-value-derive = { workspace = true }
semio-s-artifact-stdio-semio = { workspace = true }
semio-s-artifact-stdio-txt = { workspace = true }
pack = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
semio-framework-dispatch-macros = { workspace = true }
semio-framework-os-flow = { workspace = true, optional = true }
semio-framework-os-infinite = { workspace = true, optional = true }
semio-framework-ui = { workspace = true, optional = true }
semio-framework-ui-contract = { workspace = true, optional = true }
semio-framework-ui-styling = { workspace = true, optional = true }

[dev-dependencies]
semio-framework-async-macros = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
```

Note: `[lib] path = "../../🦀️.rs"` — the crate root is the **artifact root** file, two levels above the package.

> ⚠️ **`component-app-assembly` is NOT universal, and Reference A does not have it.**
> `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/Cargo.toml` has **no `[features]` section at all**, and `grep -rn "component-app-assembly"` over artifact A's whole tree returns **zero hits** — its editor/viewer compile unconditionally. Repo-wide only **53 of 139** artifact-tree `Cargo.toml` files declare the feature; it is absent from `cad`, `raster`, `note`, `forms`, `flow`, `layout`, `drawing` and every `📕️norm` artifact.
> **Decision for the WFC artifacts:** copy Reference B and declare the feature. It keeps a bare `cargo check -p <crate>` fast (schema half only) and matches the newer convention, at the cost of having to pass `--features component-app-<artifact>` whenever you want the surfaces.

A's Cargo.toml instead carries its test oracle as a plain dev-dependency, with the rule spelled out (`…/📸️remodeling/📦️packages/🦀️rust/Cargo.toml:78-80`):

```toml
[dev-dependencies]
semio-framework-plugin = { workspace = true, features = ["component-guest", "artifact-app-testing"] }
semio-framework-async-macros = { path = "../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust" }
# 🔬️ Test oracle only (never `[dependencies]`): validates that our own encoded/decoded PNG bytes
# are readable by/match the reference decoder/encoder — see 🖼️images/🦀️.rs's test module.
png = "0.17.16"
```

It also renames path dependencies to bare aliases, which is how leaf files can say `pack::…`, `geometry::…` (`…/Cargo.toml:40-42,60,65`):

```toml
number = { path = "…/🔢️number/📦️packages/🦀️rust", package = "semio-framework-number" }
```

#### `[[test]]` — when a lane needs its own binary

Default: **no `[[test]]` entries.** Every test is a `#[cfg(test)] #[path = "…"] mod <ascii_name>;` inside the crate root. Emoji-named files are never picked up as implicit integration tests.

Repo-wide only **two** artifact crates declare `[[test]]`. The literal block, which also states the rule — `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml:77-96`:

```toml
# 🧪️ Emoji file names never compile as an implicit `tests/` target, so the example-geometry lane is
# declared with an explicit ASCII name and a package-root-relative path.
[[test]]
name = "example-geometry"
path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs"

[[test]]
name = "io-round-trip"
path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔁️round-trip/🦀️.rs"

[[test]]
name = "incremental-eval"
path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔁️incremental/🦀️.rs"
required-features = ["component-app-assembly"]
```

Use `[[test]]` only when the lane (a) must run from the crate's **public** surface, (b) must not share a test binary — e.g. it installs its own `#[global_allocator]`, or (c) needs `required-features`. Otherwise mount it as a `mod`.

The `path` is relative to the **Cargo.toml's own directory**, and `name` must be ASCII.

The feature gates three things in the crate root:

```rust
#[cfg(feature = "component-app-assembly")]   pub fn declaration() -> …            // :109
#[cfg(feature = "component-app-assembly")] #[path = "."] pub mod editor { … }     // :350
#[cfg(feature = "component-app-assembly")] #[path = "."] pub mod viewer { … }     // :377
#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;                                                                // :443-445
```

**Consequence:** a bare `cargo check -p <crate>` compiles only the schema half. To exercise the editor/viewer you must pass `--features component-app-assembly`.

### 2.11 The `#[path]` module tree — literal template

`🧩️assembly/🦀️.rs:136-347`. Every level uses `#[path = "."]` for the logical module and a full emoji path for the leaf. Copy this shape verbatim:

```rust
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]            // A only; B has no standard-root file
        mod standard_root;
        pub use standard_root::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]   // A only
                mod subset_root;
                pub use subset_root::*;

                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path =  "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                    pub mod diff;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        // ——— ONE BLOCK PER MUTATION (see §3.1) ———
                        #[path = "."]
                        pub mod change_seed {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds-the-solve-from-7-to-99/🦀️.rs"]
                            mod tests_reseeds_the_solve_from_7_to_99;
                        }
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }

                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import { #[path = "."] pub mod deserializers { #[path = "."] pub mod artifacts {
                        #[path = "."] pub mod txt { #[path = "."] pub mod v_utf_8 { #[path = "."] pub mod any {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        } } }
                    } } }
                    #[path = "."]
                    pub mod export { /* …/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs */ }
                }
            }
        }
    }
}
```

Followed by the **flat shim block** every artifact carries (`🧩️assembly/🦀️.rs:426-441`):

```rust
// ---- Shims: flat access from the artifact root, mirroring generation2d/generation3d ----
pub mod schema     { pub use super::standards::v1::subsets::any::schema::*; }
pub mod diff       { pub use crate::standards::v1::subsets::any::schema::diff::*; }
pub mod mutations  { pub use crate::standards::v1::subsets::any::schema::mutations::*; }
pub mod inferences { pub use crate::standards::v1::subsets::any::schema::inferences::*; }
pub use crate::standards::v1::subsets::any::schema::diff::AssemblyDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::AssemblyMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::AssemblySnapshot;
```

Reference A adds more shims for its five registered language surfaces (`📸️remodeling/🦀️.rs:2436-2470`): `io`, `op` (= mutations::text), `document_dsl` (= snapshot::text), `spr` (= mutations::binary), `diff::{schema,text}`, `snapshot::{schema,pack,text}`.

And the examples block (`🧩️assembly/🦀️.rs:404-424`):

```rust
#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🚪️two-room-corridor/🦀️.rs"]
    pub mod two_room_corridor;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️wall-roof-facade-strip/🦀️.rs"]
    pub mod wall_roof_facade_strip;

    /// 📇️ Every bundled example, in the order the editor's example picker offers them.
    pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![two_room_corridor::source(), wall_roof_facade_strip::source()]
    }

    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
    mod tests;
}
```

### 2.12 Standard root and subset root (Reference A pattern — recommended)

`📸️remodeling/🏅️standards/🔖️1/🦀️.rs` — **complete file**:

```rust
//! 🏅️ Standard root for `s.remodel.remodeling@1` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM design.md §2). Exports `standard() -> StandardDeclaration`, mounting subset `any` —
//! this artifact's only subset.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard<PA>() -> StandardDeclaration<PA>
where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::remodeling::RemodelingPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::remodeling::RemodelingViewer>>>,
{
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.remodeling+json"], extensions: &["remodeling"] }, subsets: vec![subsets::any::subset()] }
}
```

`📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:19-43`:

```rust
fn examples() -> &'static [ExampleSource] { editor::examples::example_source_slice() }

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: std::sync::OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = std::sync::OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::remodeling_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA>() -> SubsetDeclaration<PA> where PA: … {
    SubsetDeclaration {
        dialect: crate::REMODELING_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::remodeling_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::RemodelingViewer, PA>(viewer::create_remodeling_viewer()),
        editor: editor_surface::<editor::RemodelingPlayApp, PA>(editor::create_remodeling_app()),
        examples: examples(),
    }
}
```

And the artifact-level assembler (`📸️remodeling/🦀️.rs:115-124`):

```rust
pub fn artifact<PA>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA>
where PA: semio_framework_plugin::PluginApp + From<…EditorApp<…>> + From<…ViewerApp<…>>,
{
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.remodel.remodeling").expect("canonical remodeling kind"), localization: &[], standards: vec![standards::v1::standard()] }
}
```

### 2.13 Plugin-root wiring (`✏️s/🔌️plugins/<plugin>/🦀️.rs`)

`✏️s/🔌️plugins/🌀️procedural/🦀️.rs` is 143 lines and is the template for the new `🌊️wfc` plugin root.

`dyn_enum_close!` usage — **one variant pair per artifact** (`:34-44`):

```rust
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the procedural 2D, 3D, and assembly surfaces.
    pub enum ProceduralApps: PluginApp {
        Generation2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_procedural_generation2d::editor::generation2d::Generation2dPlayApp>>),
        Generation2dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_procedural_generation2d::viewer::generation2d::Generation2dViewer>>),
        Generation3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dPlayApp>>),
        Generation3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_procedural_generation3d::viewer::generation3d::Generation3dViewer>>),
        AssemblyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_procedural_assembly::editor::assembly::AssemblyEditor>>),
        AssemblyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_procedural_assembly::viewer::assembly::AssemblyViewer>>),
    }
}
```

Required prelude for that macro (`:5-8`):

```rust
use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, FlowExtensionDeclaration, …, Plugin, PluginApp};
```

The builder (`:89-133`), abridged to the per-artifact rows an author repeats five times:

```rust
pub fn plugin() -> Result<Plugin<ProceduralApps>, PluginAssemblyError> {
    let mut builder = Plugin::<ProceduralApps>::builder("procedural")
        .label("Procedural")
        .version("0.1.0")
        .package_id("semio:procedural")
        .routed_inference(semio_s_artifact_procedural_assembly::…::inferences::assembly_inference_metadata())
        .artifact(semio_s_artifact_procedural_assembly::declaration().map_err(PluginAssemblyError::definition)?)
        .editor_with_examples::<…::AssemblyEditor>(…::create_assembly_editor(), semio_s_artifact_procedural_assembly::examples::sources())
        .editor_mutation_roster::<…::AssemblyEditor>()
        .viewer::<…::AssemblyViewer>(…::create_assembly_viewer())
        .viewer_mutation_roster::<…::AssemblyViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_procedural_assembly::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist generation2d/generation3d/assembly editor edits to the open document".into(),
            optional: false,
        });
    builder.try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin, ProceduralApps);
```

> **`package_id` must match the plugin id** — `"semio:procedural"` vs builder `"procedural"`. Project memory records a real incident where builder id ≠ `semio:<id>` component metadata broke plugin loading.

### 2.14 "The ~12 registries one mutation kind touches"

Enumerated literally for `🎲️change-seed` in `🧩️assembly`. Every one of these must be edited/created, or the kind is invisible/broken:

| # | registry | exact path / literal |
|---|---|---|
| 1 | aggregate enum variant | `…/🧬️mutations/🦀️.rs:29` → `ChangeSeed(super::change_seed::ChangeSeed),` |
| 2 | `KINDS` const | `…/🧬️mutations/🦀️.rs:37` → `"change-seed"` in the kebab list |
| 3 | builder re-export | `…/🧬️mutations/🦀️.rs:42` → `pub use super::change_seed::change_seed;` |
| 4 | crate-root `#[path]` mount block | `🦀️.rs:274-286` (component + diff + inverse + test) |
| 5 | mutation descriptor | `…/🎲️change-seed/🔣️.json` |
| 6 | payload JSON Schema | `…/🎲️change-seed/🧬️schema/🔣️.json` |
| 7 | aggregate JSON Schema `oneOf` | `…/🧬️mutations/🔣️.json` → `{ "$ref": "./change-seed/schema.json" }` |
| 8 | aggregate GraphQL | `…/🧬️mutations/🔗️.graphql` → `type ChangeSeed { seed: Float! }` + union member |
| 9 | aggregate protobuf | `…/🧬️mutations/🛰️.proto` → `ChangeSeed change_seed = 9;` in the `oneof` + `message ChangeSeed { uint64 seed = 1; }` |
| 10 | aggregate TS union | `…/🧬️mutations/🟦️.ts` → `export interface ChangeSeed { seed: number; }` |
| 11 | text grammar (only if `requiredLanguageSurfaces` has `"text"`) | `…/🧬️mutations/📝️text/📖️.grammar.semio` + `🔤️.ebnf` + `🅰️.g4` |
| 12 | binary protocol (only if `"binary"`) | `…/🧬️mutations/💾️binary/📡️.protocol.semio` + `🥋️.ksy` + `🌶️.spicy` + `🔠️.abnf` |
| 13 | fixture quintet | `…/✳️any/🧫️fixtures/🧬️mutations/🎲️change-seed/<case>/{📸️snapshot/⬅️before,📸️snapshot/➡️after,🦠️mutation,🔺️diff,🎯️outcome}/🔣️.json` |
| 14 | per-case Rust test | `…/🎲️change-seed/🧪️tests/<case>/🦀️.rs` |
| 15 | oracle manifest vector | `…/✳️any/🔮️oracles/🔣️.json` → `mutationCatalogs[0].vectors[]`, `.kinds[]`, `mutationManifests[0].mutations[]` |
| 16 | Python reference implementation | `…/🧪️tests/mutate-<artifact>-1/🐍️component.py` |
| 17 | Gherkin row | `…/🥒️.feature` |
| 18 | editor command variant (if user-triggerable) | `…/✏️editor/🦀️.rs` `AssemblyEditorCommand::ChangeSeed` + `handle()` arm |
| 19 | taxonomy test-case name | `🔣️taxonomy.json` → `semanticDirectoryMemberKinds["members-of-tests"].memberNames` |

That is 19 touch points; the "~12 registries" figure in project memory counts only the code-side ones (1-12 + 14).

---

## 3. Mutation folder — complete template

### 3.1 `🎲️change-seed` (Reference B, simplest) — full file list

```
…/🧬️schema/🧬️mutations/🎲️change-seed/
├── 🦀️.rs                                   ← payload + builder + MutationKind impl
├── 🔣️.json                                 ← mutation DESCRIPTOR (not a schema)
├── 🧬️schema/🔣️.json                        ← payload JSON Schema (draft-07)
├── 🔺️diff/🦀️.rs                            ← (payload, base) -> MutationOutcome<Diff>
├── ↩️inverse/🦀️.rs                          ← (payload, base) -> Vec<Mutation>
└── 🧪️tests/🎲️reseeds-the-solve-from-7-to-99/🦀️.rs
```

Note: there is **no `🦠️mutation/` dir** in the source tree — `mutationBehaviorFacetDirs` lists `["🦠️mutation","🔺️diff","↩️inverse"]` but `🦠️mutation` appears only under `🧫️fixtures`.

### 3.2 `🎲️change-seed/🦀️.rs` — COMPLETE

```rust
//! 🎲 Assembly mutation — `ChangeSeed`: sets the deterministic WFC solve seed. PERSISTED snapshot
//! field, authored ONLY here — never ambient/`Math.random`-style — so `InferredField::compute`'s
//! `DepHash` caching stays sound (WFC is seeded-random internally).

use crate::diff::AssemblyDiff;
use crate::mutations::AssemblyMutation;
use crate::schema::snapshot::AssemblySnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ChangeSeed
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSeed {
    pub seed: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_seed(seed: u64) -> AssemblyMutation {
    AssemblyMutation::ChangeSeed(ChangeSeed { seed })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for ChangeSeed {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" };

    fn diff(&self, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change seed to {}", self.seed)
    }
}
//#endregion 🔖️ChangeSeed
```

A mutation that addresses a specific entity also implements `fn target()` (`🕳️delete-slot/🦀️.rs:33-35`):

```rust
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
```

`verb` **must be in `protocol::APPROVED_VERBS`** — asserted by the mutations unit test (§3.9). Observed verbs: `create`, `delete`, `change`, `remove`, `connect`, `disconnect`, `replace`, `add`, `update`, `commit`, `append`, `rename`.

Reference A adds DSL participation when the kind has a text surface (`🧱replace-mesh-result/🦀️.rs:12-19`):

```rust
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-mesh-result")]
pub struct ReplaceMeshResult {
    #[dsl(block)]
    pub mesh: Box<RemodelingMesh>,
}
```

### 3.3 `🎲️change-seed/🔣️.json` — the descriptor, COMPLETE

```json
{
  "schemaVersion": 1,
  "owner": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed",
  "semanticKind": "change-seed",
  "displayName": "Change Seed",
  "emoji": "🎲",
  "aggregateVariant": "ChangeSeed",
  "payloadSchema": "🧬️schema/🔣️.json",
  "textOpcode": null,
  "binaryTag": null,
  "invertibility": "explicit-mutation",
  "diffParticipation": "detect",
  "outcomeClasses": [
    "info",
    "applied"
  ],
  "composition": "atomic",
  "requiredLanguageSurfaces": [
    "rust",
    "json-schema"
  ]
}
```

Reference A's equivalent for a text+binary kind (`🧱replace-mesh-result/🔣️.json`):

```json
{
  "schemaVersion": 1,
  "owner": "✏️s/🔌️plugins/📸️remodel/…/🧬️mutations/🧱replace-mesh-result",
  "semanticKind": "replace-mesh-result",
  "displayName": "Replace Mesh Result",
  "emoji": "🧱",
  "aggregateVariant": "ReplaceMeshResult",
  "payloadSchema": "🧬️schema/🔣️.json",
  "textOpcode": "replace-mesh-result",
  "binaryTag": null,
  "invertibility": "explicit-mutation",
  "diffParticipation": "detect",
  "outcomeClasses": ["error", "info", "applied"],
  "composition": "atomic",
  "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]
}
```

Field semantics from the scaffolder (`🏗️authoring/🧬️mutation-tree/🟦️.ts:65-97`):

- `displayName` = semanticKind kebab parts title-cased and space-joined;
- `aggregateVariant` = `policyKebabToPascal(semanticKind)`;
- `payloadSchema` = `"🧬️schema/🔣️.json"` with `--json-schema`, else `"🦀️.rs#Mutation"`;
- `textOpcode` = semanticKind with `--text`, else `null`;
- `invertibility`/`diffParticipation`/`composition` = `"plan"/"plan"/"composite"` with `--composite`, else `"explicit-mutation"/"detect"/"atomic"`;
- `outcomeClasses` defaults to `["applied"]` — widen it by hand to match what the diff builder actually raises.

**Const-assert trap** (`📓️w9-schema-engine-compile.md:49-58`): `dsl::MutationLeaf` const-asserts `MutationLeaf::DESCRIPTOR.aggregate_variant == <variant ident>`. A mismatch is an **E0080 compile error**, not a test failure.

### 3.4 `🎲️change-seed/🧬️schema/🔣️.json` — COMPLETE

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/s/procedural/assembly/1/any/mutation/change-seed/schema.json",
  "title": "ChangeSeed",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "seed"
  ],
  "properties": {
    "seed": {
      "type": "integer",
      "minimum": 0
    }
  }
}
```

`$id` pattern: `https://json.schemas.assets.semio-tech.com/s/<plugin>/<artifact>/<standard>/<subset>/mutation/<kind>/schema.json`. Reference A drops the standard/subset segments: `…/s/remodel/remodeling/mutation/replace-mesh-result/schema.json` — inconsistent; follow B's fuller form for new work.

Constraints from `mutationPayloadSchemaAuthority` in the taxonomy:

```json
"jsonSchemaDialect": "http://json-schema.org/draft-07/schema#",
"targetAuthority": "owner-relative-regular-json-schema",
"descriptorField": "payloadSchema",
"descriptorOwnerField": "owner",
"descriptorIdentityField": "semanticKind",
"descriptorCardinality": "one-canonical-no-competing-descriptor"
```

Payload schemas must be **self-contained (no `$defs`)**, draft-07, and every keyword must be in `OwnedJsonSchemaValidator`'s allowlist. `contentEncoding` is **not** allowed (`📓️w8-schema-regeneration.md:59-63`) — use `"format": "base64"` for binary lanes.

### 3.5 `🎲️change-seed/🔺️diff/🦀️.rs` — COMPLETE

```rust
//! 🔺️ Sparse diff builder for `ChangeSeed` — a single-field scalar delta.

use crate::diff::AssemblyDiff;
use crate::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::ChangeSeed, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if base.seed == payload.seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seed is already {}.", payload.seed));
    }
    protocol::MutationOutcome::new(AssemblyDiff { seed: Some(payload.seed), ..Default::default() })
}
```

**Guard order is a fleet-wide law** (`📓️w12-semantics.md:29-30`):

> "**Guard order (F4), one for the whole vocabulary:** `target-missing` → invariant → no-op → apply. A malformed argument is a fault whether or not it happens to equal what is stored."

`MutationOutcome` constructors observed:

```rust
protocol::MutationOutcome::new(diff)                                       // applied
protocol::MutationOutcome::empty().warn("mutation.no-op", msg)             // warned, no change
protocol::MutationOutcome::error("mutation.target-missing", msg, [id])     // rejected
protocol::MutationOutcome::new(diff).info("mutation.cascade", msg)         // applied + info
```

A cascading builder (`🕳️delete-slot/🔺️diff/🦀️.rs`) — **complete**:

```rust
//! 🔺️ Sparse diff builder for `DeleteSlot` — removes the id from `slots` AND cascades to every
//! edge incident to it (real BASE lookup, not a whole-snapshot capture).

use crate::diff::AssemblyDiff;
use crate::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::DeleteSlot, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if !base.slots.iter().any(|slot| slot.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let incident_edge_ids: Vec<String> = base.edges.iter().filter(|edge| edge.from_slot_id == payload.id || edge.to_slot_id == payload.id).map(|edge| edge.id.clone()).collect();
    let outcome = protocol::MutationOutcome::new(AssemblyDiff { slots_removed: vec![payload.id.clone()], edges_removed: incident_edge_ids.clone(), ..Default::default() });
    if incident_edge_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting slot \"{}\" also removed {} connected edge(s): {}.", payload.id, incident_edge_ids.len(), incident_edge_ids.join(", ")))
    }
}
```

> **Diagnostic-code collision caution**, recorded in assembly's own `🔮️oracles/🔣️.json`: "`mutation.cascade` at level `info` means OPPOSITE things in this plugin — here it announces that `delete-slot` really did remove the edges naming the slot, while in the sibling `s.procedural.generation2d` the same code at the same level announces that `delete-widget` LEFT a dangling synapse standing." Pick unambiguous codes for the new plugin.

### 3.6 `↩️inverse/🦀️.rs` and **"retained rows must be point-invertible"**

Scalar case (`🎲️change-seed/↩️inverse/🦀️.rs`) — **complete**:

```rust
//! ↩️ Inverse for `ChangeSeed` — restores the PRIOR seed from a real BASE lookup (the seed field
//! always exists, so this is never a no-op).

use crate::mutations::{change_seed, AssemblyMutation};
use crate::schema::snapshot::AssemblySnapshot;

pub fn inverse(_payload: &super::ChangeSeed, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    vec![change_seed(base.seed)]
}
```

**The point-invertibility convention**, demonstrated by `🕳️delete-slot/↩️inverse/🦀️.rs` — **complete**:

```rust
//! ↩️ Inverse for `DeleteSlot` — recreates the slot AND every incident edge the delete cascaded
//! away, all from a real BASE lookup (missing id ⇒ empty: no-op, nothing to undo).

use crate::mutations::{connect_slots, create_slot, AssemblyMutation};
use crate::schema::snapshot::AssemblySnapshot;

pub fn inverse(payload: &super::DeleteSlot, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    let index = base.slots.iter().position(|entry| entry.id == payload.id).unwrap_or(base.slots.len());
    let mut restore = vec![create_slot(index, slot.clone())];
    for (edge_index, edge) in base.edges.iter().enumerate() {
        if edge.from_slot_id == payload.id || edge.to_slot_id == payload.id {
            restore.push(connect_slots(edge_index, edge.clone()));
        }
    }
    restore
}
```

The four rules this encodes:

1. **A cascade is inverted as a cascade of per-row point mutations**, never as one multi-row "restore everything" mutation. One `create_slot` for the slot, one `connect_slots` per cascaded edge — each a real, independently-valid member of the same vocabulary.
2. **Position is carried.** `index` is read off BASE with `.position(…)`, so the row comes back **exactly where it was**, not appended. The remodel ticket measured this: `📓️w12-semantics.md:9-13` — *"Every `create-*`/`add-*` inserts at that position … so a member removed from anywhere comes back exactly where it was. This is what F2 was really about: 12 of the 14 unrestorable inverses were position failures."* The pre-fix failure mode, `📓️w2c-fixtures.md:150-152`: *"Every `↩️inverse/🦀️.rs` in this vocabulary is BASE-derived and APPEND-shaped, so a member removed from anywhere but the end comes back at the end."*
3. **Every step is a single step of the SAME vocabulary** — no synthetic `__move-*`/`__restore-*` helper verbs. `📓️w12-semantics.md:34-38`: *"Every inverse is now a **single step of this same vocabulary** and restores `before` exactly. The Python reference lost all five of its synthetic `__move-*` / `__restore-stream-kind` steps."*
4. **A missing target inverts to the empty vector**, never to a fabricated row.

The law is asserted mechanically in the mutations unit test (`…/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:7-15`):

```rust
fn round_trip(projection: &AssemblySnapshot, mutation: &AssemblyMutation) -> AssemblySnapshot {
    let (forward, _) = apply_mutation(projection, mutation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in mutation.inverse(projection) {
        restored = apply_mutation(&restored, &back).expect("valid inverse mutation").0;
    }
    assert_eq!(&restored, projection, "inverse() must restore the pre-mutation document");
    forward
}
```

The diff type's index validation is what makes point-inversion *enforceable* (`…/🔺️diff/🦀️.rs:70-90`): an upsert of an **existing** id must carry `index == existing_index` or it raises `mutation.apply.invalid-index`; an upsert of a **new** id must carry an index `<= base.len() - removed.len() + preceding_additions`.

### 3.7 Per-case test module — COMPLETE template

`…/🎲️change-seed/🧪️tests/🎲️reseeds-the-solve-from-7-to-99/🦀️.rs` (126 lines). This is the single most copyable file in the repo:

```rust
//! 🧪️ `change-seed` fixture — `🎲️reseeds-the-solve-from-7-to-99`.
//!
//! `change-seed`'s diff builder sets the scalar `seed` field only — every id-keyed collection delta stays empty …
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::diff::AssemblyDiff;
use crate::mutations::{apply_assembly_mutation, inverse_assembly_mutation, AssemblyMutation};
use crate::schema::snapshot::AssemblySnapshot;

const BEFORE:   &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/📸️snapshot/⬅️before/🔣️.json");
const AFTER:    &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/🦠️mutation/🔣️.json");
const DIFF:     &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/🔺️diff/🔣️.json");
const OUTCOME:  &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/🎯️outcome/🔣️.json");

fn before() -> AssemblySnapshot { dsl::json::from_json_str(BEFORE).expect("before snapshot decodes") }
fn expected_after() -> AssemblySnapshot { dsl::json::from_json_str(AFTER).expect("after snapshot decodes") }
fn mutation() -> AssemblyMutation { dsl::json::from_json_str(MUTATION).expect("mutation decodes") }

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() { … assert_eq!(snapshot, expected_after(), …); }

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[test]
fn inverse_restores_before() { … assert_eq!(snapshot, base, …); }

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() { … }

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() { … }

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `change-seed` is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() { … }

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() { … }

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `change-seed` changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() { … }
```

Seven tests, always. Two of the bodies verbatim (the ones with non-obvious mechanics):

```rust
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <AssemblyMutation as protocol::Mutation<AssemblySnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised.messages().iter().map(|message| {
        let level = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&message.level)).expect("severity encodes");
        (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
    }).collect();
    assert_eq!(produced, declared, "…: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = apply_assembly_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied"  => { assert!(applied, "…"); assert_ne!(snapshot, before(), "…"); }
        "rejected" => { assert_eq!(snapshot, before(), "a rejected mutation must leave the snapshot untouched"); }
        other => panic!("unknown outcome status {other:?}"),
    }
}

#[test]
fn produces_committed_diff() {
    let base = before();
    let raised = <AssemblyMutation as protocol::Mutation<AssemblySnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "…: produced diff differs from the committed 🔺️diff/🔣️.json");
}
```

**The `include_str!` relative path is `../../../../../🧫️fixtures/…`** — five `..` from `🧬️mutations/<kind>/🧪️tests/<case>/` up to `✳️any/`. Count carefully; `include_str!` is compile-time, so a wrong path is a **crate-level compile error**, not a test failure (`📓️explore-mutations-schema.md:130-146` records 99/102 dangling at one point).

Reference A's case modules use `pack::from_json_str` instead of `serde_json` after its serde-elimination wave (`📓️w2c-fixtures.md:94-97`) — either works; `dsl::json::from_json_str` is what assembly uses today.

### 3.8 Fixture quintet — literal content

```
…/✳️any/🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/
├── 📸️snapshot/⬅️before/🔣️.json
├── 📸️snapshot/➡️after/🔣️.json
├── 🦠️mutation/🔣️.json
├── 🔺️diff/🔣️.json          ← or 🚫️.absent (0 bytes) for a rejected vector
└── 🎯️outcome/🔣️.json
```

`🦠️mutation/🔣️.json` — **externally tagged, PascalCase variant as the single key**:

```json
{
  "ChangeSeed": {
    "seed": 99
  }
}
```

`🎯️outcome/🔣️.json`:

```json
{
  "status": "applied"
}
```

For a rejected vector the shape is `{"status": "rejected", "code": "…", "path": […]}`. For a warned vector, add `"messages": [{"level": "…", "code": "…"}]`.

`🔺️diff/🔣️.json` — **every lane explicit, none omitted**:

```json
{
  "schema": null,
  "seed": 99,
  "slotsRemoved": [],
  "slotsUpserted": [],
  "edgesRemoved": [],
  "edgesUpserted": [],
  "weightsRemoved": [],
  "weightsUpserted": [],
  "rulesRemoved": [],
  "rulesUpserted": []
}
```

### 3.9 🔢 FIXTURE FLOAT CANONICAL FORM — the rule that broke 209 tests

`📸️snapshot/⬅️before/🔣️.json` (excerpt) — note **`0.0`, `2.0`, `1.0`, never `0`, `2`, `1`**:

```json
{
  "schema": "s.assembly",
  "seed": 7,
  "slots": [
    { "id": "slot-a", "x": 0.0, "y": 0.0, "z": 0.0 },
    { "id": "slot-b", "x": 2.0, "y": 0.0, "z": 0.0 }
  ],
  "edges": [ { "id": "edge-ab", "fromSlotId": "slot-a", "toSlotId": "slot-b" } ],
  "modules": [
    { "childId": "module-wall", "target": { "artifactId": "kit-wall", "dialect": { "artifactKind": "s.stdio.semio", "standard": "1", "subset": "kit" } } },
    { "childId": "module-roof", "target": { "artifactId": "kit-roof", "dialect": { "artifactKind": "s.stdio.semio", "standard": "1", "subset": "kit" } } }
  ],
  "weights": [ { "moduleId": "module-wall", "weight": 1.0 } ],
  "rules": [ { "id": "rule-wall-roof", "moduleAId": "module-wall", "moduleBId": "module-roof", "allowed": true, "params": { "kind": "null" } } ]
}
```

`seed: 7` stays an integer because the Rust field is `u64`. `x/y/z/weight` are `f64` and **must** carry `.0`.

The rule, from `.🧬semio/…/REMODEL-PLUGIN-END-TO-END/🐍️canonicalize-remodel-fixture-floats.py:2-7`:

> "🔢 Rewrite integer literals under the f32/f64 fields of the remodeling fixtures as floats (`2` → `2.0`): the Rust codec's decode→encode fixed point prints every float with a decimal, and the fixture oracle compares JSON values, which distinguish `2` from `2.0` (215 `committed … is not canonical` / `produced diff differs` failures on 2026-09-17). The float keys are DERIVED from the normative JSON schemas (`🧬️schema/🔣️.json` + every `🧬️mutations/*/🧬️schema/🔣️.json`: keys typed `number` — also inside a nullable `["number", "null"]` — and never `integer`), never hand-listed. Run from the repo root; idempotent."

and `📓️status.md:108`:

> "209 fixture failures (`committed … is not canonical` / `produced diff differs`) were the float canonical form (`2` vs `2.0` for f32/f64 fields) … rewrote 393 + 207 fixture files; the three `commit-reconstruction` mutation fixtures had a non-canonical key order (`sparse` last) — reordered to the struct's field order."

**Two further fixture laws from the same wave:**

- **Key order must match the Rust struct's declaration order.** `dsl::json` emits declaration order; a fixture that re-orders keys fails `committed_json_is_canonical`.
- **f32 fields widen.** `impl ToValue for f32` does `*self as f64`, so `0.42f32` reaches the wire as `0.41999998688697815` (`📓️w2c-fixtures.md:136-148`). Prefer `f64` in new snapshots; if you must use `f32`, pick values whose shortest f32 lexeme equals their shortest f64 lexeme (`📓️w12-semantics.md:128-133`).

### 3.10 Aggregate `🧬️mutations/🦀️.rs` — COMPLETE (73 lines)

```rust
//! 🧬️ Assembly artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload, one per
//! `🧬️mutations/<slug>/` triad leaf wired by `🦀️.rs`. `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<AssemblySnapshot>` and `impl protocol::SemanticMutation<AssemblySnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.

use crate::diff::AssemblyDiff;
use crate::schema::snapshot::AssemblySnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
// 🧵 Deliberately NOT `use super::{create_slot, ...};` — this file's own `pub use X::mutation::x;`
// builder re-exports below, glob-re-exported back into `mutations` by the sibling `pub use
// component::*;` in `🦀️.rs`, would collide with a bare-name import of the same sibling
// submodules (E0252, hit and fixed once already this wave) — fully qualifying each variant's
// payload path below instead breaks that self-referential loop.

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = AssemblySnapshot, diff = AssemblyDiff, schema = "assembly")]
pub enum AssemblyMutation {
    CreateSlot(super::create_slot::CreateSlot),
    DeleteSlot(super::delete_slot::DeleteSlot),
    CreateRule(super::create_rule::CreateRule),
    DeleteRule(super::delete_rule::DeleteRule),
    ChangeWeight(super::change_weight::ChangeWeight),
    RemoveWeight(super::remove_weight::RemoveWeight),
    ConnectSlots(super::connect_slots::ConnectSlots),
    DisconnectSlots(super::disconnect_slots::DisconnectSlots),
    ChangeSeed(super::change_seed::ChangeSeed),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`AssemblyMutation`] variant, in declaration order …
pub const KINDS: &[&str] = &["create-slot", "delete-slot", "create-rule", "delete-rule", "change-weight", "remove-weight", "connect-slots", "disconnect-slots", "change-seed"];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️Builders
pub use super::change_seed::change_seed;
pub use super::change_weight::change_weight;
/* … one per kind, alphabetical … */
//#endregion 🔖️Builders

pub type AssemblyEnvelope = store::ArtifactEnvelope<AssemblySnapshot, AssemblyMutation>;
pub type AssemblyStore    = store::ArtifactStore<AssemblySnapshot, AssemblyMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant.
pub fn apply_assembly_mutation(projection: &mut AssemblySnapshot, mutation: &AssemblyMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;
    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_assembly_mutation(projection: &AssemblySnapshot, mutation: &AssemblyMutation) -> Vec<AssemblyMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
```

**`#[derive(dsl::Mutations)]` writes the whole dispatch.** Do not hand-write apply/diff/inverse dispatch.

The aggregate's own unit test (`🧪️tests/🔬️unit/🦀️.rs:17-23`) — mandatory:

```rust
#[test]
fn dispatch_registers_semantic_descriptors_with_approved_verbs() {
    for kind in AssemblyMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(AssemblyMutation::kinds().len(), 9);
}
```

### 3.11 `🔺️diff/🦀️.rs` — the sparse diff type

`…/🧬️schema/🔺️diff/🦀️.rs:11-36`:

```rust
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.assembly")]
pub struct AssemblyDiff {
    #[state(artifact)] pub schema: Option<String>,
    #[state(artifact)] pub seed: Option<u64>,
    #[state(artifact)] pub slots_removed: Vec<String>,
    #[state(artifact)] pub slots_upserted: Vec<(usize, AssemblySlot)>,
    #[state(artifact)] pub edges_removed: Vec<String>,
    #[state(artifact)] pub edges_upserted: Vec<(usize, AssemblySlotEdge)>,
    #[state(artifact)] pub weights_removed: Vec<String>,
    #[state(artifact)] pub weights_upserted: Vec<AssemblyModuleWeight>,
    #[state(artifact)] pub rules_removed: Vec<String>,
    #[state(artifact)] pub rules_upserted: Vec<(usize, AssemblyRule)>,
}
```

The shape rule: **one scalar `Option<T>` per scalar field; one `<name>Removed: Vec<String>` + `<name>Upserted: Vec<(usize, T)>` pair per id-keyed collection.** The `usize` is the insertion/replacement index — that is what makes inverses position-exact. A collection whose members have no meaningful order (`weights`) drops the index.

The file also supplies `merge_upserts` (`:43-56`, the `absorb` primitive) and `apply_collection` (`:61-…`, which raises `mutation.apply.missing-target`, `mutation.apply.duplicate-target`, `mutation.apply.conflicting-target`, `mutation.apply.invalid-index`).

### 3.12 Aggregate sidecars — literal shapes

`🧬️mutations/🔣️.json`:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/s/procedural/assembly/1/any/mutations.json",
  "title": "AssemblyMutation",
  "oneOf": [
    { "$ref": "./create-slot/schema.json" },
    { "$ref": "./delete-slot/schema.json" },
    { "$ref": "./change-seed/schema.json" }
  ]
}
```

`🧬️mutations/🔗️.graphql`:

```graphql
# 🧬️ Assembly mutation vocabulary.

type CreateSlot { index: Int! }
type DeleteSlot { id: String! }
type ChangeSeed { seed: Float! }

union AssemblyMutation =
  CreateSlot
  | DeleteSlot
  | ChangeSeed
```

`🧬️mutations/🛰️.proto`:

```protobuf
syntax = "proto3";
package semio.s.procedural.assembly.mutation;

message AssemblyMutation {
  oneof kind {
    CreateSlot create_slot = 1;
    DeleteSlot delete_slot = 2;
    ChangeSeed change_seed = 9;
  }
}

message CreateSlot { uint64 index = 1; }
message DeleteSlot { string id = 1; }
message ChangeSeed { uint64 seed = 1; }
```

`🧬️mutations/🟦️.ts` — its own header states the wire contract precisely:

```typescript
/** 🧬️ AssemblyMutation — one discriminated-union member per `🧬️mutations/<slug>/` triad's payload
 * shape. Mirrors the Rust `🦀️.rs` sibling's `AssemblyMutation` enum, which carries only
 * `#[derive(dsl::Mutations)]` — no `#[serde(tag = ...)]` — so it serializes with serde's default
 * EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }` …
 * None of the 9 leaf structs carry `#[serde(rename_all = ...)]`, so every leaf's own field names
 * are the literal Rust snake_case names verbatim. */
import type { AssemblySlot, AssemblySlotEdge, AssemblyRule } from "../📸️snapshot/🟦️";

export interface CreateSlot { index: number; slot: AssemblySlot; }
export interface DeleteSlot { id: string; }
export interface ChangeSeed { seed: number; }
```

> **⚠️ Assembly's `.graphql` and `.proto` are DRIFTED.** They omit the `slot`/`rule`/`edge` payload fields that the Rust and TS surfaces carry, and `ChangeSeed.seed` is `Float!` in GraphQL but `uint64` in proto and `u64` in Rust. Do **not** copy assembly's graphql/proto as correct examples — copy their *shape*, derive their *content* from the Rust struct. `bun ./📜️script.ts verify artifact-field-parity enforce` is the gate that catches this.

### 3.13 Binary and text representation dirs

```
🧬️schema/📸️snapshot/💾️binary/  { 🦀️.rs, 🟦️.ts, 📡️.protocol.semio, 🥋️.ksy, 🌶️.spicy, 🔠️.abnf, 🧪️tests/🔬️unit/🦀️.rs }
🧬️schema/📸️snapshot/📝️text/    { 🦀️.rs, 🟦️.ts, 📖️.grammar.semio, 🔤️.ebnf, 🅰️.g4, 🔗️.graphql, 🔣️.json, 🛰️.proto, 🧪️tests/🔬️unit/🦀️.rs }
```

`🥋️.ksy` (assembly's, minimal — a sealed opaque payload):

```yaml
meta:
  id: procedural_assembly_snapshot
  endian: le
seq:
  - id: payload
    size-eos: true
```

`📖️.grammar.semio` (assembly's, minimal):

```
dialect grammar
grammar assembly.snapshot
extension assembly
start document

document = header body
header = "schema" SP "procedural.assembly.snapshot" NL
body = payload NL?
payload = OCTET+
```

Reference A's grammars are real (5 registered `dsl::LanguageSpec`s). The framework rules (`📓️w10-grammars.md:17-26`):

> "`dsl::LanguageSpec { id, extension, role, grammar, grammar_path, protocol, protocol_path, hooks }`. `parsed_grammar()` **rejects any dialect but `grammar`**; `is_text_role()` = Document/Config/Ops/Embedded/Diff (must carry a grammar), `is_binary_role()` = Pack/Spr (must carry a protocol, never a grammar). … **`LanguageRole` has no `Inference` variant**, so the inference facet's grammar and protocol are authored and `include_str!`'d but reach no registry. Framework gap — do not invent a local role."

The Rust side just re-exports them as consts (`📸️snapshot/💾️binary/🦀️.rs:3-7`, `📸️snapshot/📝️text/🦀️.rs:15-19`):

```rust
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
```

**Foreign-type trap** in `📝️text/🦀️.rs:3-11`: if any snapshot field is a type owned by another crate (e.g. `SemioValue` from `s.stdio.semio@v1/value`), you cannot `impl DslRecord` for it. Assembly's solution is a **local structural twin** carrying the field as `dsl::DslValue` and bridging with `dsl::to_dsl_value`/`dsl::from_dsl_value`.

---

## 4. Examples, fixtures, oracles, tests

### 4.1 `📚️examples/<slug>/`

Real tree (Reference B, both slugs identical in shape):

```
📚️examples/
├── 🚪️two-room-corridor/
│   ├── 🦀️.rs                                         ← snapshot()/source()/label() builder — the AUTHORITY
│   ├── 🟦️.ts                                         ← TS mirror (plugin-registry :check HARD-FAILS if absent)
│   ├── 🖼️assets/🚪️two-room-corridor/🗣️.dsl.semio      ← printed FROM the Rust builder
│   ├── 🧪️tests/🧩️example/{🦀️.rs, 🟦️.ts}
│   └── 🧫️fixtures/🧩️example/🔣️.json
├── 🧱️wall-roof-facade-strip/   (same)
└── 🧪️tests/🧩️outcome/{🦀️.rs, 🟦️.ts}                  ← cross-example outcome test
```

The asset directory is **doubly named**: `🖼️assets/<same emoji slug>/🗣️.dsl.semio`. `exampleAssetsDirName` = `🖼️assets`, `exampleTestsDirName` = `🧪️tests`, `exampleSlugPattern` = `"^.+\\uFE0F[a-z0-9]+(?:-[a-z0-9]+)*$"` (emoji + VS16 + kebab).

The DSL asset is `include_str!`'d back into the text module (`📸️snapshot/📝️text/🦀️.rs:25-29`):

```rust
//#region 🔖️Examples
/// 📄️ The two authored WFC problem specs this subset ships, in their own `.assembly` DSL.
pub const ASSEMBLY_EXAMPLE_CORRIDOR_TEXT: &str = include_str!("../../../📚️examples/🚪️two-room-corridor/🖼️assets/🚪️two-room-corridor/🗣️.dsl.semio");
pub const ASSEMBLY_EXAMPLE_FACADE_TEXT:   &str = include_str!("../../../📚️examples/🧱️wall-roof-facade-strip/🖼️assets/🧱️wall-roof-facade-strip/🗣️.dsl.semio");
//#endregion 🔖️Examples
```

#### The five-item `🦀️.rs` template

Minimum viable example (`…/🧊️generation3d/…/📚️examples/📦️rectangle-extrude-volume/🦀️.rs`, **complete**, 13 lines):

```rust
//! 📚️ Example `rectangle-extrude-volume`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "rectangle-extrude-volume";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
```

Reference B extends it with a `snapshot()` builder that is **the authority the DSL asset is printed from** (`…/🚪️two-room-corridor/🦀️.rs`, 54 lines):

```rust
pub const ID: &str = "two-room-corridor";
pub const ICON: &str = "route";
pub const SEED: u64 = 7;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Two Rooms And A Corridor", "Zwei Räume und ein Korridor")
}

pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🚪️two-room-corridor/🗣️.dsl.semio");

pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

/// 🧩️ The authored problem spec, stated in Rust so the committed `🗣️.dsl.semio` asset is a PRINT of
/// this and never a second, drifting authority.
pub fn snapshot() -> AssemblySnapshot { AssemblySnapshot { schema: ASSEMBLY_DOCUMENT_SCHEMA.into(), seed: SEED, slots: vec![…], … } }

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
```

`🟦️.ts` is a flat const mirror (**complete**, 5 lines):

```ts
/** 📚️ Example `two-room-corridor` — a forced `room · corridor · room` path. */
export const id = "two-room-corridor";
export const label = { en: "Two Rooms And A Corridor", de: "Zwei Räume und ein Korridor" } as const;
export const icon = "route";
export const seed = 7;
```

`🧪️tests/🧩️example/🦀️.rs` — **complete** (25 lines). These four assertions are the contract:

```rust
//! 🧪️ `two-room-corridor` — the committed asset IS the print of the authored spec, and nothing else.

use super::{snapshot, PRIMARY_TEXT, SEED};
use crate::schema::snapshot::text::{parse_dsl, print_dsl};

#[test]
fn committed_asset_is_the_print_of_the_authored_spec() {
    assert_eq!(print_dsl(&snapshot()), PRIMARY_TEXT, "🗣️.dsl.semio must be regenerated from the Rust builder, never hand-edited");
}

#[test]
fn committed_asset_parses_back_to_the_authored_spec() {
    assert_eq!(parse_dsl(PRIMARY_TEXT).expect("committed asset parses"), snapshot());
}

#[test]
fn the_pack_codec_round_trips_the_same_document() {
    let bytes = crate::schema::snapshot::binary::encode(&snapshot());
    assert_eq!(crate::schema::snapshot::binary::decode(&bytes).expect("committed document decodes"), snapshot());
}

#[test]
fn the_seed_is_persisted_and_authored() {
    assert_eq!(snapshot().seed, SEED);
}
```

`🧫️fixtures/🧩️example/🔣️.json` — **complete**:

```json
{
  "schema": "s.procedural.assembly.example-outcome/v1",
  "example": "two-room-corridor",
  "seed": 7,
  "slots": 3,
  "edges": 2,
  "weights": 2,
  "modules": ["room", "corridor"],
  "rules": 3,
  "satisfiable": true,
  "assignments": { "room-a": "room", "corridor": "corridor", "room-b": "room" }
}
```

and the cross-example lane `📚️examples/🧪️tests/🧩️outcome/🟦️.ts` declares its type (**complete**, 12 lines):

```ts
/** 🎯️ The shape of one example's committed expected-outcome vector (`🔣️.json` beside each example). */
export type AssemblyExampleOutcome = {
  readonly schema: "s.procedural.assembly.example-outcome/v1";
  readonly example: string;
  readonly seed: number;
  readonly slots: number;
  readonly edges: number;
  readonly modules: readonly string[];
  readonly rules: number;
  readonly satisfiable: boolean;
  readonly assignments: Readonly<Record<string, string>>;
};
```

`📚️examples/🧪️tests/🧩️outcome/🦀️.rs` (145 lines) is the gate over all slugs, with five tests: `every_example_matches_its_committed_outcome`, `every_committed_assignment_is_consistent_with_its_own_spec`, `every_example_rule_and_weight_names_a_declared_module`, `every_example_edge_names_a_declared_slot`, `the_bundled_roster_is_exactly_the_committed_set` — plus a `#[test] #[ignore] fn debug_emit_example_assets()` regenerator writing into a scratchpad path. Its roster:

```rust
const OUTCOME_SCHEMA: &str = "s.procedural.assembly.example-outcome/v1";

struct Committed { example: &'static str, json: &'static str, snapshot: fn() -> AssemblySnapshot }

fn committed() -> Vec<Committed> {
    vec![
        Committed { example: super::two_room_corridor::ID, json: include_str!("../../🚪️two-room-corridor/🧫️fixtures/🧩️example/🔣️.json"), snapshot: super::two_room_corridor::snapshot },
        Committed { example: super::wall_roof_facade_strip::ID, json: include_str!("../../🧱️wall-roof-facade-strip/🧫️fixtures/🧩️example/🔣️.json"), snapshot: super::wall_roof_facade_strip::snapshot },
    ]
}
```

#### Asset-path variants that really exist (do not assume one)

| shape | used by |
|---|---|
| `🖼️assets/<slug>/🗣️.dsl.semio` | B (both slugs), gen3d (all 8) — **use this** |
| `🖼️assets/🗣️.dsl.semio` (flat) | A `🛰️synthetic-orbit`, beside `🔮️ground-truth.json` and `🎞️frame-00.png…🎞️frame-35.png` |
| **subset-level** `✳️any/🖼️assets/<slug>/🗣️.dsl.semio` | A `🎬️demo` — its own `📚️examples/🎬️demo/🖼️assets/` is empty and the `include_str!` reaches `"../../🖼️assets/🎬️demo/🗣️.dsl.semio"` |
| `🖼️assets/🎮️.cmd.semio` | A `✏️editor/📚️examples/🎬️demo-session` (command-replay session example) |
| `🖼️assets/🎒️.pack.semio`, `📡️<slug>.spr.semio`, `🔧️<slug>.op.semio` | gen3d, committed beside the DSL |

Reference A regenerates its asset from the Rust builder with `bun ./📜️script.ts regenerate-example <slug>` (run inside `📦️packages/🦀️rust`; nx `@semio-tech/remodel-plugin:regenerate-example`). It shells to an `#[ignore]`d test — `📜️script.ts:26-32` → `cargo test -p semio-s-plugin-remodel --lib regenerates_the_synthetic_orbit_example -- --ignored --nocapture`, whose writer is `…/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🦀️.rs:825`:

```rust
std::fs::write(assets.join("🗣️.dsl.semio"), <RemodelingSnapshot as store::ArtifactDsl>::print_dsl(&fixture_document()))
```

Assembly's are hand-printed and pinned by `committed_asset_is_the_print_of_the_authored_spec`.

`crate::examples::sources()` returns `Vec<ExampleSource>` and is passed to `.editor_with_examples::<E>(definition, sources)` at the plugin root.

> **`📇️registry:check` emits a hard finding for any example directory missing its `🟦️.ts`** (`🔌️plugin/📇️registry/📜️script.ts:1332`). Ship the TS mirror.

Reference A additionally keeps an **example registry** that B does not have (`…/✳️any/✏️editor/📚️examples/🦀️.rs:1-34`) — worth copying once an artifact has more than a handful of examples:

```rust
//! 📚️ The example registry — the ONE array every remodeling example registers in, and the only thing
//! `🎮️commands/🎬️set-active-example` and the manifest read.
//! ➕️ To add an example: mount its leaf in `📦️packages/🦀️rust/🦀️.rs` and append ONE row to
//! [`REMODELING_EXAMPLES`].
pub struct RemodelingExample { pub id: &'static str, pub text: &'static str, pub icon: &'static str, pub label_en: &'static str, pub label_de: &'static str }
pub const REMODELING_EXAMPLES: &[RemodelingExample] = &[ … ];
```

with `🧪️tests/🔬️unit/🦀️.rs` asserting `example_ids_are_unique_and_resolvable`, `every_source_carries_its_committed_text`, `the_boot_document_parses_the_demo_example`.

### 4.2 `🔮️oracles/🔣️.json` — one file, at the SUBSET level

Path: `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json`. Its top-level shape:

```json
{
  "$schema": "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json",
  "schemaVersion": 2,
  "_comment": "…",
  "oracles":          [ /* one entry per second implementation */ ],
  "noOracleDecisions":[],
  "mutationCatalogs": [ /* the vector index the harness globs against */ ],
  "mutationManifests":[ /* per-kind dispatch + oracle requirements */ ]
}
```

One oracle entry (fields verbatim, `rationale` elided — the real one is ~4 000 words and is the *argument* for the registration, not boilerplate):

```json
{
  "id": "assembly-python-independent",
  "ecosystem": "python",
  "package": "",
  "version": "",
  "capabilities": ["assembly-1-mutate"],
  "comparisonProfiles": ["ordered-json-v1"],
  "license": "AGPL-3.0-only",
  "testOnly": true,
  "rationale": "A second implementation of the `s.procedural.assembly` document and all nine typed mutations, in Python, at `../../../../../🧪️tests/mutate-assembly-1/🐍️component.py`. …",
  "kind": "verified-native-second-implementation",
  "engine": { "family": "none", "implementation": "in-repository second implementation", "version": "0" },
  "productionReachable": false,
  "networkDuringExecution": false,
  "nativeSecondImplementation": {
    "format": "s.assembly",
    "noThirdPartySurvey": {
      "ecosystemsSearched": ["python/pypi"],
      "candidatesConsidered": [
        { "package": "WFC-COLLAPSE", "reason": "a WFC library computes a COLLAPSE and none of them carries the problem statement as a document, let alone reads this carrier." }
      ]
    },
    "subjectImplementationLanguage": "rust",
    "secondImplementationLanguage": "python",
    "specificationSource": "…; ../🧬️schema/📸️snapshot/🔣️.json; …",
    "fixtureCoverage": { "vectors": 9, "capabilitiesCovered": ["assembly-1-mutate"] }
  }
}
```

`mutationCatalogs[0]` — the **vector index**; one entry per kind, with the on-disk directory names:

```json
{
  "id": "assembly-1-any",
  "capability": "assembly-1-mutate",
  "standardDirectoryName": "🔖️1",
  "subsetDirectoryName": "✳️any",
  "vectors": [
    {
      "mutationId": "change-seed",
      "sourceMutationDirectoryName": "🎲️change-seed",
      "mutationDirectoryName": "🎲️change-seed",
      "scenarios": [
        { "id": "reseeds-the-solve-from-7-to-99", "directoryName": "🎲️reseeds-the-solve-from-7-to-99" }
      ]
    }
  ],
  "kinds": ["create-slot", "delete-slot", "create-rule", "delete-rule", "change-weight", "remove-weight", "connect-slots", "disconnect-slots", "change-seed"]
}
```

`mutationManifests[0]` — one row per kind:

```json
{
  "schema": "semio.repository-test.mutation-manifest/v2",
  "artifact": "s.assembly",
  "standard": "1",
  "subset": "any",
  "standardDirectoryName": "🔖️1",
  "subsetDirectoryName": "✳️any",
  "mutations": [
    {
      "id": "change-seed",
      "capability": "assembly-1-mutate",
      "payloadSchema": "🧬️.schema.json",
      "outcomes": ["applied"],
      "productionDispatch": { "operation": "change-seed", "bridgeVersion": 1, "variant": "ChangeSeed" },
      "oracleRequirements": [ { "capability": "assembly-1-mutate", "qualifyingKind": "verified-native-second-implementation" } ]
    }
  ]
}
```

> ⚠️ `"payloadSchema": "🧬️.schema.json"` here disagrees with the mutation descriptor's `"payloadSchema": "🧬️schema/🔣️.json"`. The manifest value is a legacy filename. Reproduce assembly's value to stay consistent with the harness; do not "fix" it without running `bun ./📜️script.ts contract`.

### 4.2a Running the oracles — the REAL commands

The test platform is its own domain with its own router. Header, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts:9`:

```
// 🧪️ Router of the repository testing domain:
//   bun ./📜️script.ts <discover|contract|oracle|subject|parity|run|report|clean|dependency|nx|doctor> [args…]
```

Nx targets, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json` (project `@semio-tech/repo-test-domain`):

```json
"test-oracle":  { "options": { "command": "bun ./📜️script.ts oracle",  "forwardAllArgs": true } },   // :44-51
"test-subject": { "options": { "command": "bun ./📜️script.ts subject", "forwardAllArgs": true } },   // :52-58
"test-parity":  { "options": { "command": "bun ./📜️script.ts parity",  "forwardAllArgs": true } }    // :59-66
```

**The invocation actually used:**

```zsh
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
bun ./📜️script.ts parity <level> --owner "<owner path or plugin emoji>" [--case <case slug>] [--implementation <lang>]
bun ./📜️script.ts contract --owner "<owner path>"
bun ./📜️script.ts run <level> --owner "<owner path>"
```

- `<level>` ∈ `fundamental | quick | long | exhaustive` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1080`:
  `export const TEST_LEVELS = ["fundamental", "quick", "long", "exhaustive"] as const;`
- Selector flags parsed at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🔍️discovery/🎛️selection/🟦️.ts:4-25` (`--case`, `--owner`, `--project`, `--implementation`) and `:49-70` (`--artifact --standard --subset --mutation --outcome --fixture-class --fixture-family --oracle --probe --platform --agent --run --status`).
- `--owner` matching is loose — `:13-15`: *"`--owner 🗄️stdio` selects every artifact owned beneath that plugin"*.
- `parity` runs the **oracle phase and the subject phase and compares** (`⚖️parity/📋️orchestration/🟦️.ts:48`, verdicts `:99-107`); failed diffs land in `testCacheDir(repoRoot, "diffs")`. `oracle` / `subject` run one half each.

Real recorded invocations:

```zsh
bun ./📜️script.ts parity exhaustive --owner 🗒️note --case mutate-note-1
bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print" --case render-scene
bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/📓️print"
```

**Discovery needs no registration step.** `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts:912-931` walks the whole repo collecting every directory whose basename equals `taxonomy.testOraclesDirName` (`"🔮️oracles"`) and reads the `🔣️.json` inside; `loadOracleRegistry` (`:955-979`) merges them with the core registry under `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📇️registry/`. A new `🔮️oracles/🔣️.json` anywhere is picked up automatically.

**`bun ./📜️script.ts contract`** (nx `test-contract`, `validateAllContracts` at `🧰️framework/…/🧪️test/📜️script.ts:828`) writes `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` and exits 1 on any breach. Breach families (`TESTING_BREACH_KINDS`): `testing/taxonomy`, `testing/contract`, `testing/fixture`, `testing/oracle`, `testing/dependency`, `testing/discovery`; ids include `missing-external-oracle`, `insufficient-engine-independence`, `native-second-implementation-unearned|-not-native|-partial-coverage|-same-language`, `missing-fixture`, `orphan-fixture`.

The Python test-platform host is invoked as `python3 🐍️.py --plan <plan.json> --out <results.jsonl> --adapter <component.py>`.

### 4.2b Plugin-level `🔮️oracles/🔣️.json`

A plugin root may carry its own manifest, which registers no oracle but declares the shared native host package every case links. `✏️s/🔌️plugins/📸️remodel/🔮️oracles/🔣️.json` — **complete** (15 lines):

```json
{
  "$schema": "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json",
  "schemaVersion": 2,
  "_comment": "🧩️ This plugin's contribution to the repository test platform. It registers NO reference implementation: every artifact `📸️remodel` owns is a semio-NATIVE document (photogrammetric reconstruction), carried in `.dsl.semio`/`.pack.semio`, and no third party reads or writes that envelope — the per-subset manifests under `🗿️artifacts/**/🪆️subsets/*/🔣️oracle.json` record that as a `noOracleDecision` each. What DOES belong here is the native host package every case of this plugin links, because `oracleHostPackagesFor` walks up from the case owner and only an ancestor manifest is consulted.",
  "oracles": [],
  "noOracleDecisions": [],
  "comparisonProfiles": [],
  "oracleHostPackages": [
    {
      "implementation": "rust",
      "package": "semio-s-plugin-stdio-test-oracle",
      "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust"
    }
  ]
}
```

`✏️s/🔌️plugins/🌀️procedural` has **no** plugin-level `🔮️oracles`. **The new `🌊️wfc` plugin needs one**, or its Rust adapters have no host package to link.

`oracles[].kind` vocabulary observed: `verified-native-second-implementation`, `cross-semio-implementation`, `third-party-library`, `third-party-cli`, `standards-reference-tool`. `noOracleDecisions[]` entries are `{id, capabilities[], rationale, substitutes[]}` with substitutes drawn from `specification-vectors`, `metamorphic-laws`, `independent-implementations`.

Reference A additionally carries a `fixtureManifests[]` block (`…/📸️remodeling/…/🔮️oracles/🔣️.json:1632`) pinning content hashes:

```json
{ "schema": "semio.repository-test.fixture/v2", "id": "commit-reconstruction-refused", "class": "handcrafted",
  "target": { "artifact": "s.remodel.remodeling", "standard": "1", "subset": "any" },
  "mutation": "commit-reconstruction", "outcome": "rejected",
  "units": { "length": "unitless", "angle": "degree" },
  "files": [
    { "role": "expected-before",   "path": "../🧫️fixtures/🏁️commit-reconstruction/⬅️before.json",    "mediaType": "application/json", "sha256": "sha256:89482afc…", "bytes": 7786 },
    { "role": "mutation-payload",  "path": "../🧫️fixtures/🏁️commit-reconstruction/🦠️mutation.json", "mediaType": "application/json", "sha256": "sha256:51c2ef1f…", "bytes": 242  },
    { "role": "expected-after",    "path": "../🧫️fixtures/🏁️commit-reconstruction/➡️after.json",     "mediaType": "application/json", "sha256": "sha256:89482afc…", "bytes": 7786 }
  ],
  "provenance": { "source": "authored", "license": "public-domain (handcrafted by this repository)", "attribution": "…", "security": "scanned-clean", "privacy": "no-personal-data" },
  "comparisonProfile": "ordered-json-v1", "reproducible": true, "family": "remodeling-carrier"
}
```

> ⚠️ **Do not copy the path strings out of assembly's `rationale`/`specificationSource`.** They name `../../../../../🧪️tests/mutate-assembly-1/🐍️component.py` and `🦀️component.rs`, which **do not exist**. The real files are `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-assembly-1/{🥒️.feature, 🐍️.py, 🦀️.rs}`. Reference A's feature docstring has the same `🐍️component.py` drift. The oracle's `rationale` also concedes the subject half "links no plugin crate and replays the committed vectors" — i.e. today it proves the *specification* is implementable twice, not that the *codec* agrees with a second producer.

### 4.3 The `🥒️.feature` + `🐍️.py` + `🦀️.rs` case triplet

**Discovery rule** (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts:545-588`, `discoverTestCases`): walk to any directory named `🧪️tests`; **each immediate child directory that contains a `🥒️.feature` is a case**, and its adapters are the **sibling files in that same directory** (`🦀️.rs`, `🟦️.ts`, `🐍️.py`, `🐹️.go`, `🔷️.cs`). The case's `sharedFixtureDir` is `<owner>/🧫️fixtures` (`:573`) — the subset-level `✳️any/🧫️fixtures` — which is what `shared://…` URIs resolve against.

**Put the feature and every adapter in ONE directory.** (Assembly violates this: `…/✳️any/🧪️tests/mount-contract/` holds only the `🥒️.feature` while `…/🧪️tests/🧩️mount-contract/` holds the adapters, so the case discovers with zero adapters and the Rust half runs only because the crate root mounts it.)

Real triplet, Reference B — `…/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-assembly-1/`:
`🥒️.feature` (133 lines) · `🐍️.py` (333) · `🦀️.rs` (283).
Reference A — `…/📸️remodeling/…/✳️any/🧪️tests/📸️mutate-remodeling-1/`: `🥒️.feature` (513) · `🐍️.py` (1260) · `🦀️.rs` (518).

**`🥒️.feature` head and outline shape (literal, B):**

```gherkin
@capability-assembly-1-mutate
@oracle-assembly-python-independent
@comparison-ordered-json-v1
@mutations-assembly-1-any
Feature: Apply every typed assembly mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.procedural.assembly` document and all nine typed mutations …
  It imports nothing from this repository's Rust.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: The committed <id> vector declares its own kind and moves the document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before":   "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff":     "shared://🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome":  "shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after":    "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then the committed mutation payload declares the <id> kind
    And the after-snapshot differs from the before-snapshot, or the committed outcome declares the vector a no-op
    Examples:
      | id               | vector                                                        |
      | create-slot      | 🧩️create-slot/🧩️appends-slot-c-at-index-2                |
      | delete-slot      | 🕳️delete-slot/🚫️removes-slot-a-and-cascades-edge-ab     |
      | create-rule      | 🚦️create-rule/⛔️appends-a-rule-forbidding-roof-over-wall |
      | delete-rule      | ❌delete-rule/🚫️removes-the-wall-roof-rule              |
      | change-weight    | 🔢️change-weight/⚖️raises-the-wall-module-selection-bias  |
      | remove-weight    | 🪶️remove-weight/🪶️drops-the-wall-module-weight-override |
      | connect-slots    | 🔗️connect-slots/🔗️joins-slot-b-to-slot-c-at-index-1      |
      | disconnect-slots | ✂️disconnect-slots/✂️severs-edge-ab-leaving-both-slots  |
      | change-seed      | 🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99           |
```

plus a second outline `@id-inverse @level-long` over the same Examples table, and a plain `@id-identity-round-trip @level-long @mode-round-trip` scenario.

**Tag vocabulary:** `@capability-<cap>`, `@oracle-<oracle id>` (or `@no-oracle-<id>`), `@comparison-<profile>`, `@mutations-<catalog id>` at Feature level; `@id-<scenario id>`, `@level-{fundamental,quick,long,exhaustive}`, `@mode-{differential,round-trip,…}` per scenario.

**`🐍️.py` head (literal, B):**

```python
#!/usr/bin/env python3
"""🧩️ An INDEPENDENT second implementation of the `s.procedural.assembly` document and its nine typed
mutations, in Python, serving as this case's differential oracle.
…
**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half only.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports
```

**`🦀️.rs` head (literal, B)** — note how the shared law helpers are mounted by `#[path]`, not linked:

```rust
//! 🦀️ Assembly 1 exhaustive mutation case — Rust adapter. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`.
//! …
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate …
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
```

Shared law helpers available from that module: `law::divergence` (names the first divergence by JSON path), `law::mutation_is_observable`, `law::inverse_restores`, `law::round_trip_preserves`, `law::carrier_is_exact`.

**Roles:**

| member | role |
|---|---|
| `🥒️.feature` | **The authority.** Its `Examples` table is the ONLY place a fixture directory name appears, resolved at run time — so a directory rename cannot silently rot either half. |
| `🐍️.py` | The **oracle** half. `from semio_repo_test import Adapter, Outcome`; reads fixtures via `ctx.fixture_bytes`. Never imports the Rust. |
| `🦀️.rs` | The **subject** half. `use semio_repo_test_host::{Adapter, Context, Json, Outcome};`; reads via `ctx.fixture_json`; gated behind the `sut` feature when it links the plugin crate. |
| `🟦️.ts` (optional) | A TypeScript twin — either a second subject, or a third-party-twin model driven from the same fixture JSON (see `🧊️generation3d`'s `🔬️status-contract` / `🩹️gesture-rearm`). |

> **No `🥒️.feature` + `🐍️.py` + `🟦️.ts` triplet exists in any of the four artifacts studied.** The pairs that do exist are feature+py+rs (the mutation cases), py+ts+rs without a feature (assembly's mount-contract), and feature+py alone (`🧊️generation3d/…/📐️example-geometry-3d-1`). Stated so nobody copies a shape that isn't there.

The drift-avoidance rule behind all of this — `📓️w2-fixture-wiring.md:9-13`:

> "**One statement of where a vector lives, and it is the FEATURE file's.** … neither implementation carries a transcribed path that can drift away from the directory it names. That drift is exactly what the 2026-09-05 path-shortening pass caused here."

`📓️w2c-fixtures.md:100-101`:

> "`🦀️.rs` (subject) and `🐍️.py` (reference) registrations generated FROM the same table, so a row that gains or loses a vector cannot leave either half behind."

`📓️w3-ts-codec-oracle.md:100-101`:

> "Every case directory is **globbed off disk** (`readdirSync` …); no hash-suffixed name is transcribed, so W2b's in-flight renames cannot break it."

Reference A's feature file is "7 outlines + the identity scenario, `| id | kind | vector | code |` tables".

Taxonomy keys governing all of this: `testFeatureFileKindId`, `testAdapterFileKinds`, `testImplementationIds`, `testImplementationFileKindIds`, `testComparisonProfiles`, `testFeatureRequiredFeatureTagPrefixes`, `testFeatureOracleTagPrefixes`, `testFeatureRequiredScenarioTagPrefixes`, `testCaseSlugPattern`, `testLevels`, `testModes`, `testRoles`, `testFixtureSchemes`.

### 4.4 Mount-contract test — COMPLETE (Reference B; the only artifact that has one)

It is a **four-language contract over one fixture**: the same expectations stated in Rust, Python, TypeScript and JSON.

```
…/✳️any/🧪️tests/mount-contract/🥒️.feature       ← ASCII dir, feature ONLY
…/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs          ← emoji dir, adapters ONLY
…/✳️any/🧪️tests/🧩️mount-contract/🐍️.py
…/✳️any/🧪️tests/🧩️mount-contract/🟦️.ts
…/✳️any/🧫️fixtures/🧩️mount-contract/🔣️.json
```

> ⚠️ **This split is broken** against the discovery rule in §4.3 — the adapters must sit beside the feature. As committed, `mount-contract/` discovers as a case with zero adapters and `🧩️mount-contract/` is not a platform case at all; its Rust half runs only because the crate root mounts it. **Put yours in one directory.**

`mount-contract/🥒️.feature` — **complete** (12 lines):

```gherkin
Feature: Procedural assembly is a real mounted app
  Scenario: the plugin declares editor and viewer apps
    Given the procedural plugin manifest
    Then an app "s.assembly@1/*#editor" exists
    And an app "s.assembly@1/*#viewer" exists
    And the editor window kind "framework.window.tree" is labeled "Structure" / "Struktur"

  Scenario: bundled examples are real documents
    Given the two-room-corridor and wall-roof-facade-strip examples
    Then each example DSL is non-empty
    And each example pack envelope is non-empty
    And labels exist in English and German
```

`🧩️mount-contract/🟦️.ts` — **complete** (32 lines):

```ts
/** Language-agnostic assembly mount contract — apps, window kinds, examples. */
export const expected = {
  editorAppId: "s.assembly@1/*#editor",
  viewerAppId: "s.assembly@1/*#viewer",
  windowKindId: "framework.window.tree",
  windowLabel: { en: "Structure", de: "Struktur" },
  examples: [
    { id: "two-room-corridor", label: { en: "Two Rooms And A Corridor", de: "Zwei Räume und ein Korridor" }, minSlots: 3 },
    { id: "wall-roof-facade-strip", label: { en: "Wall And Roof Facade Strip", de: "Wand-Dach-Fassadenstreifen" }, minSlots: 4 },
  ],
} as const;

export function assertMountContract(input: {
  editorAppId: string;
  viewerAppId: string;
  windowKindId: string;
  windowLabel: { en: string; de: string };
  examples: ReadonlyArray<{ id: string; label: { en: string; de: string }; slots: number }>;
}): void {
  if (input.editorAppId !== expected.editorAppId) throw new Error(`editor ${input.editorAppId}`);
  if (input.viewerAppId !== expected.viewerAppId) throw new Error(`viewer ${input.viewerAppId}`);
  if (input.windowKindId !== expected.windowKindId) throw new Error(`window ${input.windowKindId}`);
  if (input.windowLabel.en !== expected.windowLabel.en || input.windowLabel.de !== expected.windowLabel.de) {
    throw new Error(`label ${JSON.stringify(input.windowLabel)}`);
  }
  for (const example of expected.examples) {
    const got = input.examples.find((row) => row.id === example.id);
    if (!got) throw new Error(`missing ${example.id}`);
    if (got.label.en !== example.label.en || got.label.de !== example.label.de) throw new Error(`example label ${example.id}`);
    if (got.slots < example.minSlots) throw new Error(`slots ${example.id}`);
  }
}
```

`🧩️mount-contract/🐍️.py` — **complete** (33 lines), the "json is the third-party twin" pattern:

```python
#!/usr/bin/env python3
"""Language-agnostic assembly mount contract — json is the third-party twin."""
import json
from pathlib import Path

EXPECTED = {
    "editorAppId": "s.assembly@1/*#editor",
    "viewerAppId": "s.assembly@1/*#viewer",
    "windowKindId": "framework.window.tree",
    "windowLabel": {"en": "Structure", "de": "Struktur"},
    "examples": [
        {"id": "two-room-corridor", "label": {"en": "Two Rooms And A Corridor", "de": "Zwei Räume und ein Korridor"}, "minSlots": 3},
        {"id": "wall-roof-facade-strip", "label": {"en": "Wall And Roof Facade Strip", "de": "Wand-Dach-Fassadenstreifen"}, "minSlots": 4},
    ],
}

def assert_mount_contract(payload: dict) -> None:
    raw = json.dumps(payload, sort_keys=True, ensure_ascii=False)
    parsed = json.loads(raw)
    assert parsed["editorAppId"] == EXPECTED["editorAppId"]
    assert parsed["viewerAppId"] == EXPECTED["viewerAppId"]
    assert parsed["windowKindId"] == EXPECTED["windowKindId"]
    assert parsed["windowLabel"] == EXPECTED["windowLabel"]
    by_id = {row["id"]: row for row in parsed["examples"]}
    for example in EXPECTED["examples"]:
        got = by_id[example["id"]]
        assert got["label"] == example["label"]
        assert got["slots"] >= example["minSlots"]

if __name__ == "__main__":
    vector = (Path(__file__).parent / "../../🧫️fixtures/🧩️mount-contract/🔣️.json")
    assert_mount_contract(json.loads(vector.read_text()))
    print("ok")
```

`…/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs`, mounted from the crate root at `🦀️.rs:443-445` under `#[cfg(all(test, feature = "component-app-assembly"))]`:

```rust
//! Language-agnostic assembly mount contract — apps, window kinds, codecs, examples.

use crate::examples::{two_room_corridor, wall_roof_facade_strip};
use crate::schema::snapshot::text::{parse_dsl, print_dsl};
use crate::{AssemblySnapshot, ASSEMBLY_DIALECT};
use store::ArtifactPack;

#[test]
fn editor_and_viewer_share_the_assembly_dialect() {
    assert_eq!(<crate::editor::assembly::AssemblyEditor as semio_framework_plugin::ArtifactEditor>::DIALECT, ASSEMBLY_DIALECT);
    assert_eq!(<crate::viewer::assembly::AssemblyViewer as semio_framework_plugin::ArtifactViewer>::DIALECT, ASSEMBLY_DIALECT);
}

#[test]
fn editor_declares_the_structure_window_kind() {
    let def = crate::editor::assembly::create_assembly_editor();
    assert_eq!(def.id, "s.assembly@1/*#editor");
    assert!(def.window_kinds.iter().any(|window| window.id == "framework.window.tree"));
}

#[test]
fn examples_round_trip_dsl_and_pack() {
    for snapshot in [two_room_corridor::snapshot(), wall_roof_facade_strip::snapshot()] {
        let text = print_dsl(&snapshot);
        assert!(!text.trim().is_empty());
        assert_eq!(parse_dsl(&text).expect("dsl"), snapshot);
        let pack = ArtifactPack::encode_pack(&snapshot);
        assert!(pack.len() > 64);
        assert_eq!(<AssemblySnapshot as ArtifactPack>::decode_pack(&pack).expect("pack"), snapshot);
    }
}

#[test]
fn example_labels_are_localized_en_and_de() {
    assert_eq!(two_room_corridor::label(), semio_framework_plugin::LocalizedLabel::native("Two Rooms And A Corridor", "Zwei Räume und ein Korridor"));
    assert_eq!(wall_roof_facade_strip::label(), semio_framework_plugin::LocalizedLabel::native("Wall And Roof Facade Strip", "Wand-Dach-Fassadenstreifen"));
}
```

Its fixture, `…/✳️any/🧫️fixtures/🧩️mount-contract/🔣️.json` — **complete**:

```json
{
  "editorAppId": "s.assembly@1/*#editor",
  "viewerAppId": "s.assembly@1/*#viewer",
  "windowKindId": "framework.window.tree",
  "windowLabel": {"en": "Structure", "de": "Struktur"},
  "examples": [
    {"id": "two-room-corridor", "label": {"en": "Two Rooms And A Corridor", "de": "Zwei Räume und ein Korridor"}, "slots": 3},
    {"id": "wall-roof-facade-strip", "label": {"en": "Wall And Roof Facade Strip", "de": "Wand-Dach-Fassadenstreifen"}, "slots": 4}
  ]
}
```

### 4.5 Test subfolder conventions

Observed `🧪️tests/<subfolder>` names, by owner:

| owner level | subfolder | in A | in B |
|---|---|:--:|:--:|
| any `🦀️.rs` module | `🔬️unit` | ✔ | ✔ |
| `🧬️schema/🧪️tests` | `🔬️unit`, `🧩️suite` | ✔ | ✔ (unit) |
| `🧬️mutations/<kind>/🧪️tests` | `<emoji><case-slug>` (one dir per vector) | ✔ | ✔ |
| subset `✳️any/🧪️tests` | `🧩️mount-contract` | — | ✔ |
| `📚️examples/<slug>/🧪️tests` | `🧩️example` | ✔ | ✔ |
| `📚️examples/🧪️tests` | `🧩️outcome` | ✔ | ✔ |
| artifact root `🧪️tests` | `🔬️unit` | ✔ | — |
| **plugin root** `🧪️tests` | `🔬️surface`, `🎚️config` | ✔ | — |
| **plugin root** `🧪️tests` | `🔬️surface`, `🔬️boot-deadline`, `😴️idle-turns`, `🚪️close-ladder` | — | ✔ |

#### Artifact-root `🧪️tests/` lanes, all four artifacts

| artifact | subfolders under the ARTIFACT-root `🧪️tests` |
|---|---|
| **A `📸️remodeling`** | `🔬️unit` |
| **B `🧩️assembly`** | *(no artifact-root `🧪️tests` directory at all)* |
| `🧊️generation3d` | `🔬️brep-extension`, `🔬️flow-operators`, `🔬️publication-authority`, `🔬️serial`, `🔬️store-fixture`, `🔬️unit` |
| `🌀️generation2d` | `🔬️publication-authority`, `🔬️store-fixture`, `🔬️unit` |

Mount site, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs:160-183`:

```rust
//#region 🧪️Tests
#[cfg(test)] #[path = "🧪️tests/🔬️serial/🦀️.rs"]                 pub(crate) mod test_serial;
#[cfg(test)] #[path = "🧪️tests/🔬️flow-operators/🦀️.rs"]          pub(crate) mod flow_operators;
#[cfg(test)] #[path = "🧪️tests/🔬️store-fixture/🦀️.rs"]           pub(crate) mod store_fixture;
#[cfg(test)] #[path = "🧪️tests/🔬️brep-extension/🦀️.rs"]          pub(crate) mod brep_extension;
#[cfg(test)] #[path = "🧪️tests/🔬️publication-authority/🦀️.rs"]   pub(crate) mod publication_authority;
#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"]                    mod tests;
```

> **Correction to the brief:** `store-fixture` and `publication-authority` **do** exist as artifact-level lanes — but only in the siblings `🌀️generation2d` and `🧊️generation3d`, never in A or B. **Both are worth copying for the WFC artifacts**, because both solve problems the WFC artifacts will hit.

**`🔬️store-fixture/🦀️.rs`** — the one `ArtifactStore` fixture every schema test uses, built the way the runtime builds it. Complete (gen2d, 36 lines):

```rust
//! 🏪️ The ONE `ArtifactStore` fixture the generation2d schema tests use, built exactly the way the
//! runtime builds it.
//!
//! A bare `ArtifactStore::new` carries NO owner catalog, so the first `Apply` fails closed with
//! `edit history insertion requires its exact mutation retirement factory`
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`). Production installs
//! `generation2d_document_store_owners()` through `ArtifactEditor::build_document_store_owners`,
//! and the owned document disposer walks the store to terminal-empty on the way out — this fixture
//! does both. 2d twin of `🧊️generation3d/🧪️tests/🔬️store-fixture/🦀️.rs`
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_document_store_owners;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_plugin::ArtifactOwnedDisposer;

pub type Generation2dFixtureStore = store::ArtifactStore<Generation2dSnapshot, Generation2dMutation>;

/// 🏗️ A document store over `snapshot` carrying the artifact's own owner catalog.
pub async fn document_store(snapshot: Generation2dSnapshot) -> Generation2dFixtureStore {
    let mut store = Generation2dFixtureStore::new(store::create_document_envelope(crate::GENERATION_2D_SCHEMA, "generation2d", snapshot, None)).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(generation2d_document_store_owners());
    store
}

/// 🧹️ Walks the fixture store to its terminal-empty ownership witness through the same
/// `ArtifactDocumentStoreDisposer` the editor declares.
pub fn close(mut store: Generation2dFixtureStore) {
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<Generation2dSnapshot, Generation2dMutation>::new();
    for _ in 0..1_000_000 {
        if matches!(disposer.close_step(&mut store, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("generation2d document store close step"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(std::thread::panicking() || disposer.terminal_is_empty(&store), "generation2d fixture store did not reach its terminal-empty ownership witness");
}
```

**`🔬️publication-authority/🦀️.rs`** — a process-global serial lock so two tests cannot saturate the fixed publication-lease table. Complete (gen2d, 15 lines):

```rust
//! 🔐️ Serialises every test that admits a `generation2d_admit_publication_authority` lease.
//!
//! The lease table is a PROCESS-GLOBAL fixed registry of `GENERATION2D_PUBLICATION_SLOTS` (4)
//! entries (`🧬️schema/🧬️mutations/💾️binary/🦀️.rs`), so two tests holding leases at the same time
//! saturate it and the loser fails with `generation2d-publication.saturated` — an order- and
//! thread-count-dependent failure that has nothing to do with what either test asserts
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use std::sync::{Mutex, MutexGuard};

static SERIAL: Mutex<()> = Mutex::new(());

pub fn lock() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
```

`🧊️generation3d`'s version (16 lines) instead delegates to the crate's single `🔬️serial` lock, with the fix documented in the comment: *"🐛️ This used to be its OWN mutex, which serialised publication laws against each other and against nothing else … One lock over one shared state."* — prefer that shape.

#### Conventions that exist ONLY in a sibling

- `🔬️store-fixture`, `🔬️publication-authority` — gen2d + gen3d artifact root.
- `🔬️serial`, `🔬️flow-operators`, `🔬️brep-extension` — gen3d artifact root only.
- `🔬️status-contract`, `🩹️gesture-rearm` — gen3d subset `🧪️tests`, **`🟦️.ts` only, no `🥒️.feature`**: third-party-twin TS models driven from the same fixture JSON the Rust laws use. Not discovered platform cases.
- `📐️example-geometry-3d-1`, `🚪️io-procedural-3d-1` — gen3d subset, extra capability-level Gherkin cases beyond the mutation case.
- `🔬️snapshot-fixture-asset`, `🔬️retained-authority-laws`, `🔬️retained-mounted-laws`, `🔬️mounted-registry` — nested under `✏️editor/🗣️terminology/🧪️tests/`, `🧬️schema/🧬️mutations/💾️binary/🧪️tests/`, `✏️editor/🌉️wasm/🧪️tests/` in **both** siblings, in **neither** A nor B.
- Subset-level **loose fixture JSONs** (`🧫️fixtures/⏱️evaluate-budget.json`, `🎚️slider-values.json`, `🛑️preview-cancel.json`…) — 18 in gen3d, 5 in gen2d. A and B have only directory-shaped fixtures.

Conversely `🧩️outcome` (B's collective example lane) and `🧩️mount-contract` exist only in B; `🧩️suite` (`🧬️schema/🧪️tests/🧩️suite/🟦️.ts`) only in A.

`🔬️unit` counts, for scale: A 53, B 52, gen3d 59, gen2d 31. `🧩️example`: A 3, B 2, gen3d 9, gen2d 2.

The taxonomy member kinds `store-fixture-cases` (5), `store-fixture-mutations` (13) and `publication-authority-verification` (1) are a **separate**, framework-module-level registry — not the artifact-level directories above.

### 4.6 Plugin-level (not artifact-level) tests and fixtures

These live **beside** `🗿️artifacts`, at the plugin root:

```
✏️s/🔌️plugins/🌀️procedural/
├── 🦀️.rs                     ← mounts 🧪️tests/🔬️surface
├── 🔣️.json                   ← GENERATED by `describe`
├── 🛂️.descriptor.semio        ← GENERATED by `describe` (binary)
├── 🎮️commands/
├── 📦️packages/🦀️rust/
├── 🧪️tests/{🔬️surface, 🔬️boot-deadline, 😴️idle-turns, 🚪️close-ladder}/🦀️.rs
├── 🧫️fixtures/🚪️close-ladder/🔣️.json
└── 🗿️artifacts/
```

`✏️s/🔌️plugins/📸️remodel/🧪️tests` has only `{🎚️config, 🔬️surface}` and **no** `🧫️fixtures` — the close-ladder/idle-turns/boot-deadline lanes are `🌀️procedural`'s (added by ticket 26/09/09). Copy **procedural's** set for the new WFC plugin; it is the newer and fuller convention.

`🧫️fixtures/🚪️close-ladder/🔣️.json` — **complete** (the `lawNote` is the interesting part: it explains why the law is expressed as a *dilution ratio* rather than a turn count):

```json
{
  "contract": "semio.procedural.close-ladder/v1",
  "app": "s.procedural.generation3d@1/*#editor",
  "previewWindow": "procedural-preview",
  "previewBody": "procedural.play.preview",
  "instance": 7,
  "sessions": [
    { "label": "cold", "documents": 0, "rendersPerDocument": 0 },
    { "label": "warm", "documents": 3, "rendersPerDocument": 2 },
    { "label": "long", "documents": 8, "rendersPerDocument": 4 }
  ],
  "maximumCloseTurns": 768,
  "turnBudget": 16384,
  "minimumRetainedWorkDilutionPercent": 140,
  "note": "🚪️ The cost of closing one generation3d editor instance, as a function of the session it accumulated. Each close turn is one browser worker round trip, so a ladder whose turn count grows with the retained session is a role switch whose wall time grows with it — measured on 6018 as 62–87 s and then plugin-ui.lifecycle-close-budget-exhausted (ticket 26/09/09, 🗑️generated/journey-5/console.txt). Before the fix every session cost exactly 2052 turns: the reactor close walked its fixed slot geometry one slot per reactor turn.",
  "lawNote": "📏️ Two statements, because the turn count is a LATENCY PROXY — how many empty reactor polls fit while the close pump grinds on the maintenance lane — so the absolute numbers move with machine load, all three sessions together. (1) maximumCloseTurns is the browser's own close budget and every session must fit the SAME one, whatever it retained; that is what the pre-fix 2052-turn constant blew. (2) minimumRetainedWorkDilutionPercent is the load-INVARIANT half: retainedWork = documents x rendersPerDocument, and the dearest session's turns-per-retained-unit must be at least this percent cheaper than the cheapest non-cold session's. … The cold session is deliberately excluded from the dilution: it retains nothing, so it measures only the noise floor."
}
```

**What each plugin lane asserts:**

| lane | asserts | how it is compiled |
|---|---|---|
| `🔬️surface` | the manifest builds synchronously; `assert_viewer_never_mutates::<V>()`; `assert_editor_and_viewer_share_dialect::<E, V>()`; that `.editor_with_examples` really stamped the examples onto the manifest | `#[cfg(test)] mod` in the plugin root |
| `🔬️boot-deadline` | a real `generation3d` editor boot's **first reactor step** stays under `semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US`, phase by phase, driving the real `poll_kernel` | its own `[[test]]` |
| `🚪️close-ladder` | every session (`cold`/`warm`/`long`) reaches `Retired` inside `maximumCloseTurns` (768) of the same budget, and retained-work dilution ≥ 140 % | its own `[[test]]` |
| `😴️idle-turns` | after settling, 512 consecutive idle turns answer `TurnStatus::Idle` and guest heap growth ≤ 64 KiB | its own `[[test]]` — it installs a `#[global_allocator]`, so it **cannot** share a binary |

Plugin-root mount for the surface lane (`🌀️procedural/🦀️.rs:136-140`; `📸️remodel/🦀️.rs:57-61` is identical):

```rust
//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests
```

The other three are `[[test]]` targets — `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:87-105`, **literal**:

```toml
# ⏱️ The guest-lifecycle boot deadline law runs from the crate's PUBLIC surface as its own `[[test]]`,
# independent of the lib test target, so a peer's in-flight editor/viewer refactor can never hide a
# first-step regression.
[[test]]
name = "boot_deadline"
path = "../../🧪️tests/🔬️boot-deadline/🦀️.rs"

# 🚪️ The instance close law for both procedural editors runs from the crate's PUBLIC surface as its
# own `[[test]]`, so a close-ladder regression can never hide behind the lib test target.
[[test]]
name = "close_ladder"
path = "../../🧪️tests/🚪️close-ladder/🦀️.rs"

# 😴️ The idle-turn authority (settled status + zero retention) runs from the crate's PUBLIC surface
# as its own `[[test]]` and installs its own `HeapWitness` global allocator, which no other test
# target may carry.
[[test]]
name = "idle_turns"
path = "../../🧪️tests/😴️idle-turns/🦀️.rs"
```

with the dev-deps that make them possible (`:79-85`):

```toml
[dev-dependencies]
semio-framework-plugin = { workspace = true, features = ["component-guest", "artifact-app-testing"] }
semio-framework-async = { workspace = true }
semio-framework-async-macros = { workspace = true }
semio-framework-job = { workspace = true }
semio-framework-trace = { workspace = true }
serde_json = { workspace = true }
```

`✏️s/🔌️plugins/📸️remodel/🧪️tests/🔬️surface/🦀️.rs` — **complete** (14 lines), the minimum every plugin owes:

```rust
//! 🧪️ Contract §2.5's two cross-surface guarantees (`assert_viewer_never_mutates`,
//! `assert_editor_and_viewer_share_dialect`), landed for real in `semio_framework_plugin::artifact_app_laws`
//! per `📓️w0-f-report.md` gap 2 — used directly, no local stand-ins.
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn remodeling_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::remodeling::RemodelingViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn remodeling_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::remodeling::RemodelingPlayApp, crate::viewer::remodeling::RemodelingViewer>().await;
}
```

**The JS test lane** is remodel-only and worth copying. `✏️s/🔌️plugins/📸️remodel/🧪️tests/🎚️config/🟦️.ts` — **complete** (17 lines):

```ts
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** 📸️ Runs the committed Remodel example tests and the cross-language schema fixture oracle. */
export default defineConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/remodel-js",
    environment: "node",
    include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts", "🗿️artifacts/**/🧬️schema/🧪️tests/🧩️suite/🟦️.ts"],
    passWithNoTests: false,
  },
});
```

Wired by `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/📜️script.ts` → `runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts")`, exposed as nx target **`bun nx run @semio-tech/remodel-js:test`** (`cwd: ✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript`), whose `namedInputs.default` explicitly lists the config file. The procedural plugin has no such lane.

---

## 5. Naming conventions

### 5.1 Where the vocabulary lives

**`/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`** (1.14 MB) is the single source of truth. Loaded by `loadCatalogTaxonomy()` at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:1339`, which strictly validates it and throws `Invalid taxonomy schema: …` on any problem. Its own doc comment (`🔍️discovery/🟦️.ts:888`):

> "🔣️ Shape of `🔣️taxonomy.json` — the single source of truth for taxonomy directory-name/role/lang …"

Header, `🔣️taxonomy.json:2`, and `"schemaVersion": 7` at `:12`:

```json
"_comment": "🔣️ SSOT taxonomy vocabulary. Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS: authored implementation lives at <semantic-collection>/<specific>/<file-kind-emoji>.<registered-extension>; reusable modules require two independent production semantic consumers at their lowest common owner."
```

Design rules are in its `_standardsSubsetsComment` (`:4`), `_subsetVocabularyComment` (`:5`), `_surfaceComment` (`:10`) and `_cleanMechanismComment` (`:11`) — the last documents the **module-path slug algorithm**: `🔖️1` → `v1`, `◻️2d` → `_2d`, `✳️any` → `any`. That is what `standards::v1::subsets::any` in the crate root comes from.

#### A SECOND, unrelated emoji registry exists — do not confuse them

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🪪️identity/🧬️schema/🔣️entity-emojis.json` (153 lines) is the **identity / compose-id** vocabulary — tickets, sections, definitions, sessions — **not** folder prefixes. Lines 2-5:

```json
"$comment": "🔣️ Single source of truth for the repository entity-emoji vocabulary. Go reads it through a module-relative path at first use; Rust embeds it with include_str!. Every emoji is written WITHOUT the U+FE0F presentation selector — the codec re-adds it only for the text-default code points listed in textDefaultEmojis.",
"schema": "semio.repo.identity.entity-emojis/1",
"textDefaultEmojis": ["🏗", "⌨", "🖱", "🗃", "⚙", "⚖", "🏷", "🛠", "✂", "🛡", "🗑", "☀", "⏱", "✏", "👮", "⬅", "⬆", "⬇"],
"entities": {
```

Sample entries (`:6-34`): `"technology-infrastructure": "🧰"`, `"bundle-library": "📚"`, `"bundle-schema": "🛂"`, `"file-code": "💻"`, `"section": "🔖"`, `"definition-test": "🧪"`, `"year": "🎆"`, `"month": "🌙"`, `"day": "☀"`. **For artifact folder prefixes use `🔣️taxonomy.json`, not this file.**

#### The `entity-emojis` CLI — exists on disk, unverified this session

The `repo` and `semio` MCP servers both **failed to connect** (`CONNECTION_CLOSED`), so nothing could be run through them. The command does exist as Go source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🐹️.go:726-745`:

```go
func entityEmojisCommand(config *Config) *Command {
	cmd := &Command{
		Use:   "entity-emojis",
		Short: "List all entity-identifying emojis",
		RunE: func(cmd *Command, args []string) error {
			emojis := model.AllEntityEmojis()
```

wired at `:473-475`, with a golden usage fixture at `…/⌨️cli/🧪️tests/📖️usage-text/🧫️fixtures/📖️usage-goldens.json:13-14`. Entry point `…/📦️packages/🐹️go/🚀️bin/📦️main.go`; MCP adapter `…/📦️packages/🐹️go/🔌️mcp.go`. **No compiled binary is checked in.** It is a top-level `entity-emojis` subcommand — there is **no `client` parent command** (`"client"` appears only as a string *flag*, e.g. `🐹️.go:1615`). Its runtime output could not be verified here; and it would print §5.1's *identity* vocabulary anyway, not the folder-prefix one. **Read `🔣️taxonomy.json` directly.**

### 5.2 Directory-name vocabulary (all verified)

```json
"artifactsDirName":  "🗿️artifacts",
"standardsDirName":  "🏅️standards",   "standardDirPrefix": "🔖️",
"subsetsDirName":    "🪆️subsets",     "subsetDirPrefix": "✳️",
"subsetAnyId": "*",                    "subsetAnyDirName": "✳️any",
"surfaceDirNames": { "viewer": "👁️viewer", "editor": "✏️editor" },
"surfaceRoles":    ["viewer", "editor"],
"surfaceChildDirs":["🎭️modes", "🎮️commands", "📌️panels", "🎚️config", "👥️presence", "🫧️transient", "🗣️terminology", "🌉️wasm", "📚️examples"],
"surfaceRequiredChildDirs": ["🎭️modes", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"],
"modesDirName": "🎭️modes",  "windowsDirName": "🪟️windows",
"schemaChildDirs":  ["📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"],
"representationDirs": ["📝️text", "💾️binary"],
"ioSemanticCollectionDirNames": ["📸️snapshot", "🔺️diff", "💡️inferences", "🧬️mutations"],
"ioDirectionDirs": …  (📥️import / 📤️export on disk),
"exampleAssetsDirName": "🖼️assets",  "exampleTestsDirName": "🧪️tests",
"testsDirName": "🧪️tests", "testFixturesDirName": "🧫️fixtures", "testOraclesDirName": "🔮️oracles",
"mutationBehaviorFacetDirs":      ["🦠️mutation", "🔺️diff", "↩️inverse"],
"mutationOrganizationalFacetDirs":["🧩️plan", "📝️text", "💾️binary", "🧬️schema"],
"mutationDirectoryPattern": "^.+\\uFE0F[a-z][a-z0-9]*(?:-[a-z0-9]+)+$",
"exampleSlugPattern":       "^.+\\uFE0F[a-z0-9]+(?:-[a-z0-9]*)*$"
```

**Mutation directory names must be `<emoji><VS16><verb>-<noun>`** — at least two kebab segments, all lowercase alphanumeric. `newMutationSemanticParts` (`🧬️mutation-tree/🟦️.ts:39-46`) throws otherwise:

> ``new mutation: "${name}" must be an emoji-prefixed semantic verb-noun kebab name.``

and it refuses a name whose **emoji**, **semanticKind**, or **PascalCase variant** collides with a sibling (`:231-237`).

### 5.3 File-kind emoji → extension map

From `🔣️taxonomy.json` `fileKinds`:

| fileKind id | emoji | extension |
|---|---|---|
| `rust-source` | `🦀️` | `.rs` |
| `typescript-source` | `🟦️` | `.ts` (also `.tsx/.mts/.cts/.d.ts`) |
| `json` | `🔣️` | `.json` |
| `graphql` | `🔗️` | `.graphql` |
| `protobuf` | `🛰️` | `.proto` |
| `grammar-semio` | `📖️` | `.grammar.semio` |
| `ebnf` | `🔤️` | `.ebnf` |
| `antlr` | `🅰️` | `.g4` |
| `kaitai` | `🥋️` | `.ksy` |
| `spicy` | `🌶️` | `.spicy` |
| `abnf` | `🔠️` | `.abnf` |
| `protocol-semio` | `📡️` | `.protocol.semio` |
| `python-source` | `🐍️` | `.py` |
| `markdown` | `📝️` | `.md` |

Plus the observed literals not in `fileKinds` directly: `🥒️.feature` (Gherkin), `🗣️.dsl.semio` (document asset), `📌️.empty.md` (empty-facet marker), `🚫️.absent` (deliberate absence), `📜️script.ts`, `📋️project.json`, `🛂️.descriptor.semio`.

Schema-format table (`schemaFormats`) — **field casing per language**:

```json
"🦀️rust":       { "fileKindId": "rust-source",       "fieldCasing": "snake" },
"🟦️typescript": { "fileKindId": "typescript-source", "fieldCasing": "camel" },
"🔗️graphql":    { "fileKindId": "graphql",           "fieldCasing": "camel" },
"🔣️jsonschema": { "fileKindId": "json",              "fieldCasing": "camel" },
"🛰️protobuf":   { "fileKindId": "protobuf",          "fieldCasing": "snake" },
"📜️wit":        { "fileKindId": "wit",               "fieldCasing": "kebab" }
```

```json
"schemaFacetKinds": {
  "🧬️data": { "normativeFormat": "🔣️jsonschema", "formats": ["🔣️jsonschema","🦀️rust","🟦️typescript","🔗️graphql","🛰️protobuf"] }
},
"schemaDefaultFacetKind": "🧬️data"
```

**`🔣️jsonschema` is normative; the other four mirror it.** That is why §0.2's "no generator" matters — the mirrors are checked against the normative leaf by `artifact-field-parity`, never written from it.

### 5.4 Crate name — `semio-s-artifact-<plugin>-<artifact>` ✅ CONFIRMED

`…/🧩️assembly/📦️packages/🦀️rust/Cargo.toml:2` → `name = "semio-s-artifact-procedural-assembly"`.
Reference A's crate is `semio-s-artifact-remodel-remodeling` (from the ticket's cargo lines).
Plugin crates are `semio-s-plugin-<plugin>` (`semio-s-plugin-remodel`).

Nx project name is separate: `@semio-tech/<plugin>-<artifact>-rs` (`📋️project.json` `"name": "@semio-tech/procedural-assembly-rs"`), with `tags: ["language:rust", "role:artifact", "family:<plugin>"]`.

Package router (`📦️packages/🦀️rust/📜️script.ts`) — **complete, 4 lines**:

```typescript
#!/usr/bin/env bun
/** 📦️ procedural-assembly Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-procedural-assembly");
```

`📋️project.json` — **complete**:

```json
{
  "name": "@semio-tech/procedural-assembly-rs",
  "$schema": "../../../../../../../node_modules/nx/schemas/project-schema.json",
  "projectType": "library",
  "tags": ["language:rust", "role:artifact", "family:procedural"],
  "targets": {
    "build": {
      "executor": "nx:run-commands", "cache": true,
      "inputs": ["production", "^production"],
      "outputs": ["{projectRoot}/dist/build"],
      "options": { "cwd": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust", "command": "bun ./📜️script.ts build", "forwardAllArgs": true }
    },
    "check": { "executor": "nx:run-commands", "options": { "cwd": "…/📦️packages/🦀️rust", "command": "bun ./📜️script.ts check", "forwardAllArgs": true } },
    "test":  { "executor": "nx:run-commands", "options": { "cwd": "…/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test",  "forwardAllArgs": true } }
  }
}
```

Count the `../` in `$schema` and in the router import — they are depth-sensitive and will differ for a differently-nested plugin.

### 5.5 Artifact kind id — `s.<plugin>.<artifact>` ⚠️ with one exception

- Dialect / schema / codec id: `s.<plugin>.<artifact>` — `s.remodel.remodeling`, `s.procedural.generation2d`, `s.trinity.jack`, `s.puzzle.puzzle3d`, `s.block.block2d`.
- **Assembly is the exception**: its schema tree uses bare `s.assembly`, while its `ArtifactDefinition` identity is `s.procedural.assembly`. Documented at `🧩️assembly/🦀️.rs:21-31`. **Use the regular form for the new WFC artifacts.**
- `artifact_kind().id` is a **different namespace**: `<dimension>.<name>` — `3d.remodeling`, `data.assembly`. Do not conflate.

### 5.6 App id — `<artifact_kind>@<standard>/<subset>#<role>` ✅ CONFIRMED

Literal assertions from the mount-contract test and editor code:

```rust
assert_eq!(def.id, "s.assembly@1/*#editor");                  // 🧪️tests/🧩️mount-contract/🦀️.rs:17
controller: "s.remodel.remodeling@1/*#editor",                // 📸️remodeling/…/✏️editor/🦀️.rs:773
"id": "s.procedural.generation2d@1/*#editor"                  // plugin-root 🔣️.json
```

**Nobody hand-writes the id.** Canonical builder, `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3501-3512`:

```rust
/// 🪪️ The one canonical spelling of a surface id: `<artifact_kind>@<standard>/<subset>#<role>`.
pub fn surface_app_id(dialect: &ArtifactDialect, role: AppRole) -> String {
    format!("{}#{}", dialect.to_coordinate(), role.as_str())
}

/// 🪪️ Inverse of `surface_app_id`; rejects anything not matching the grammar.
pub fn parse_surface_app_id(id: &str) -> Result<(ArtifactDialect, AppRole), String> {
    let (coordinate, role_str) = id.rsplit_once('#').ok_or_else(|| format!("surface id {id:?} missing '#'"))?;
```

(`to_coordinate()` at `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:84`.)

`Editor::builder` / `Viewer::builder` derive it from the `Dialect` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32758-32772`:

```rust
impl Viewer {
    pub fn builder(dialect: Dialect) -> ViewerBuilder {
        let dialect: ArtifactDialect = dialect.into();
        let inner = Box::new(resolve_ready(AppBuilder::new(surface_app_id(&dialect, AppRole::Viewer), LocalizedLabel::native("Viewer", "Betrachter"))));
…
impl Editor {
    pub fn builder(dialect: Dialect) -> EditorBuilder {
        let dialect: ArtifactDialect = dialect.into();
        let inner = Box::new(resolve_ready(AppBuilder::new(surface_app_id(&dialect, AppRole::Editor), LocalizedLabel::native("Editor", "Editor"))));
```

and it is re-validated on build — `…/🔌️plugin/🦀️.rs:5597`:

```rust
let (dialect, role) = semio_framework::parse_surface_app_id(&self.id).map_err(|error| PluginAssemblyError::new("app-definition.invalid", format!("app id {} must be a canonical surface id: {error}", self.id)))?;
```

The `@1/*` segment comes from the dialect's `StandardId("1")` + `SubsetId::ANY` and **must match the on-disk `🏅️standards/🔖️1/🪆️subsets/✳️any` path** (stated at `📸️remodeling/🦀️.rs:142` and `🧩️assembly/🦀️.rs:29-30`).

### 5.7 Breadcrumb ✅ LOCATED

The builder method is **`.document([...])`**, and the field it lands in is `AppDefinition.breadcrumb`:

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:4836-4842`

```rust
pub fn document<I, S>(mut self, document: I) -> Self
where I: IntoIterator<Item = S>, S: Into<String>,
{
    self.document = document.into_iter().map(Into::into).collect();
    self
}
```

`…/🔌️plugin/🦀️.rs:5603` — inside `build_definition()`:

```rust
let mut definition = AppDefinition {
    id: self.id,
    role,
    dialect,
    label: self.label,
    breadcrumb: self.document,
    icon_id: self.icon_id,
    …
```

Real values (grepped across all plugins):

| artifact | editor + viewer breadcrumb |
|---|---|
| `🧩️assembly` | `.document(["semio", "assembly"])` |
| `📸️remodeling` | `.document(["semio", "remodeling"])` |
| `🔌️jack` | `.document(["semio", "trinity", "jack"])` |
| `♻️rewriting` | `.document(["semio", "trinity", "rewriting"])` |
| `🧊️process3d` | `.document(["semio", "process", "3d"])` |
| `🪵️en1995` | `.document(["semio", "norm", "en1995"])` |
| `🖨️raster` | `.document(["semio", "raster"])` |
| `🌊️flow` | `.document(["semio", "flow"])` |
| `🌀️generation2d` | `["semio", "procedural", "2d"]` (seen in the generated plugin `🔣️.json`) |

Pattern: `["semio", <plugin>, <artifact-short-name>]`, collapsing to `["semio", <name>]` when the plugin and artifact are effectively the same word. **Editor and viewer of one artifact must use the same breadcrumb.**

For the five WFC artifacts, use `["semio", "wfc", "bitmap"]`, `["semio", "wfc", "2d-grid"]`, `["semio", "wfc", "2d"]`, `["semio", "wfc", "3d-grid"]`, `["semio", "wfc", "3d"]`.

### 5.8 iconId

Set through the same builder: `.icon_id("network")` for assembly (`✏️editor/🦀️.rs:126`, `👁️viewer/🦀️.rs:79`), `.icon_id("remodeling-app")` for remodeling (`✏️editor/🦀️.rs:993`, `👁️viewer/🦀️.rs:87`). Mode and window definitions carry their own:

| site | value |
|---|---|
| assembly editor app | `"network"` |
| assembly viewer app | `"network"` |
| assembly `✏️edit` mode | `"pencil"` |
| assembly `👁️view` mode | `"eye"` |
| assembly `🌳️structure` window | `"list-tree"` |
| remodeling editor/viewer app | `"remodeling-app"` |
| remodeling `📷️capture` mode | `"camera"` |
| remodeling `🔍️analyze` mode | `"search"` |
| remodeling `🧊️model` mode | `"box"` |
| remodeling `🖼️frames` window | `"layout-grid"` |
| remodeling `📊️report` window | `"document-report"` |
| remodeling `🧊️model` window | `"remodeling-model"` |
| remodeling `asset` granularity | `"image"` |

#### There are exactly 249 valid iconIds

**Source of truth = the SVG file set**: one file per icon at
`🧰️framework/🔨️modules/🖼️assets/🔣️icons/<emoji-category>/<emoji-prefix><icon-id>.svg` — **249 `.svg` files** (excluding `🤖️generated`) across 28 category folders (`✍️editing`, `🌍️geography`, `🎛️controls`, `🎨️drawing`, `🎬️media`, `🏗️construction`, `👥️people`, `💻️devices`, `📊️data`, `📐️projection`, `📚️documents`, `📝️notes`, `🔄️transforms`, `🔎️viewing`, `🔐️security`, `🔗️connections`, `🔷️shapes`, `🕰️time`, `🕸️graphs`, `🖱️interaction`, `🗂️filing`, `🚦️feedback`, `🛠️applications`, `🧑‍💻development`, `🧭️navigation`, `🪟️layout`, `🏗️builder`, `🤖️generated`).

**Projector:** `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts` — `:122` `export const ICONS = {`, `:126` `export type IconName = keyof typeof ICONS;`, `:180` `pub enum IconName {`.

**Three generated authoritative lists, all 249 entries and set-identical:**

| artifact | path |
|---|---|
| Rust enum | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs` — `pub enum IconName {` at `:4`, 249 `#[serde(rename = "…")]` variants, `IconName::ALL` at `:185` |
| TS catalogue | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🟦️.ts` — 249 `"<id>": \`<svg…\`` entries (`"network"` at `:1933`) |
| frozen union | `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs:671` — `export type IconName = "alert-circle" \| … \| "zoom-out";` |

Generator owner: `bun nx run @semio-tech/assets:build` (`📓️h-generated-ref.md:50`; the 257-file `🤖️generated` root). A near-duplicate 255-file tree at `🧰️framework/🔨️modules/🖱️ui/🖼️assets/🔣️icons/🤖️generated` has **no current owner reference** and is a probable stale copy (`:51`).

> ⚠️ **`"remodeling-app"` — Reference A's iconId — IS NOT A VALID ICON ID.** It appears nowhere in the 249-entry list. The catalogue has `"remodel-app"` (`🪪️icon-name/🦀️.rs:369`, file `🪚️remodel-app.svg`) and `"remodel-model"` (`:371`, file `🏠️remodel-model.svg`). Three live call sites carry the bad string: `…/✏️editor/🦀️.rs:993`, `…/👁️viewer/🦀️.rs:88`, and `…/✏️editor/🎭️modes/🧊️model/🛠️tools/🏗️reconstruction/🦀️.rs:17`. `"remodeling-model"` is likewise absent.
>
> **Why it compiles anyway:** the `IconName` that reaches `AppBuilder` is a **string newtype, not the generated enum** — `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🦀️.rs:348-352`:
> ```rust
> /// 🖼️ Stable icon identifier. Local newtype mirror — the real `IconName` lives in the UI token
> /// crate this module must not depend on (§4).
> pub struct IconName(pub String);
> ```
> with the builder signature `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:4831` `pub async fn icon_id(mut self, icon_id: impl Into<IconName>) -> Self {`. **There is no compile-time or build-time validation of iconId strings on this path.** A's icon simply fails to resolve at render time. (The strongly-typed re-export is `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs:26` `pub use icon_name_gen::IconName;`.)

**Rule for the WFC artifacts:** grep the id against `🤖️generated/🪪️icon-name/🦀️.rs` before writing it. Reuse a generic existing name (`network`, `box`, `grid`, `image`, `layout-grid`, `pencil`, `eye`, `search`, `route`, `list-tree`, `file`) rather than minting one — a new name requires adding the SVG under a category folder and re-running `bun nx run @semio-tech/assets:build`. Note A's own editor gets this right one line away: `…/✏️editor/🦀️.rs:1007` `icon_id: "image".into()` — `"image"` IS in the catalogue.

### 5.9 The registry gate — new names must be added to `🔣️taxonomy.json`

`semanticDirectoryMemberKinds` holds **266 registries**, each `{ ownerKindIds, memberNames, source: "registry" }`. The ones a new artifact touches:

| registry | current size | what it lists |
|---|---:|---|
| `members-of-artifacts` | 91 | artifact directory names — contains `"🌀️generation2d"`, `"📸️remodeling"`, `"🧊️generation3d"`, `"🧩️assembly"` |
| `members-of-schema` | 1553 | every child dir under a `🧬️schema` |
| `members-of-tests` | 1246 | every `🧪️tests/<case>` dir name (e.g. `"🎲️reseeds-the-solve-from-7-to-99"`, `"🧩️mutate-assembly-1"`) |
| `members-of-fixtures` | 124 | `🧫️fixtures` children |
| `members-of-examples` | 57 | example slugs |
| `members-of-inferences` | 53 | inference sub-dirs |
| `members-of-io` | 8 | io children |
| `members-of-windows` | 75 | window dir names |
| `members-of-modes` | 12 | mode dir names |
| `members-of-commands` | 844 | command dir names |
| `members-of-plugins` | 32 | plugin dir names — **add `🌊️wfc` here** |

Mutation directory names are **not** in a memberNames list: `semanticCollections["🧬️mutations"] = { "kind": "mutation" }` and they are validated by `mutationDirectoryPattern` instead. Likewise `semanticCollections["🗿️artifacts"] = { "kind": "artifact" }`.

`mutationDomainOwners` has only **two** entries today (`🏛️architect/🏛️program` and `🗄️stdio/🧊️gltf`). Assembly and remodeling are absent, so `mutationOwnerIdentity` returns `null` for them and `new mutation` accepts free-form verb-noun names. **The WFC artifacts do not need a `mutationDomainOwners` entry** — only add one if you want a fixed per-entity operation matrix enforced.

Gate: `bun ./📜️script.ts verify taxonomy enforce` and `bun ./📜️script.ts verify taxonomy implementation enforce`. A missing memberName makes discovery silently return zero for that owner — a known failure mode where downstream gates read green while nothing is checked.

---

## 6. Ordered recipe for one new artifact

```zsh
cd /Users/ueli/Documents/semio
```

1. **Register names** in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`:
   `members-of-plugins` += `"🌊️wfc"`; `members-of-artifacts` += the five artifact dir names; later, `members-of-tests` += every case dir name you create.
2. **Scaffold the tree** (§1.3): `new artifact` → `new standard` → `new subset`.
3. **Author the snapshot** — `🧬️schema/📸️snapshot/🦀️.rs` first (§2.4). Everything else follows from it.
4. **Author the diff** — `🧬️schema/🔺️diff/🦀️.rs` (§3.11): one `Option<T>` per scalar, one `Removed`/`Upserted` pair per collection, with the `usize` index.
5. **Scaffold each mutation** (`new mutation … --json-schema`), then hand-write its `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, and finish the descriptor's `outcomeClasses`.
6. **Mount everything** in the crate root's `#[path]` tree (§2.11) — one block per mutation.
7. **Write the 20 schema leaves** by hand (§2.8). JSON Schema first (normative), then Rust/TS/GraphQL/proto mirroring it exactly.
8. **Write the text/binary representation leaves** (§3.13) — a sealed-payload minimal grammar is acceptable for a first pass; Reference B ships exactly that.
9. **Write fixtures** (§3.8) — floats as `X.0`, keys in struct declaration order, one quintet per vector, `🚫️.absent` for rejected diffs.
10. **Write the per-case tests** (§3.7) — seven tests each, `include_str!` with the right number of `..`.
11. **Write `🔮️oracles/🔣️.json`** (§4.2) and the Python second implementation.
12. **Write two examples** (§4.1) with `🗣️.dsl.semio` assets, `🟦️.ts` mirrors, and en/de labels.
13. **Write the mount-contract test and fixture** (§4.4).
14. **Wire `Cargo.toml`, `📜️script.ts`, `📋️project.json`** (§5.4) and add the crate to the workspace members.
15. **Wire the plugin root** (§2.13): `dyn_enum_close!` variants, `.artifact(…)`, `.editor_with_examples(…)`, `.viewer(…)`, `.activation(…)`.
16. **Run the gates**, in this order:

```zsh
cd /Users/ueli/Documents/semio

# structure
bun ./📜️script.ts verify taxonomy enforce
bun ./📜️script.ts verify taxonomy implementation enforce

# schema leaves
bun nx run workspace:schema-check
bun nx run workspace:schema-verify
bun ./📜️script.ts verify artifact-field-parity enforce
bun ./📜️script.ts verify policy-breach schema-representation
bun ./📜️script.ts verify artifact-contract-ownership

# compile + unit/fixture tests  (NO `timeout`; wrap in the 137-retry loop)
cargo check -p semio-s-artifact-wfc-<artifact> --lib --tests --features component-app-<artifact> -j 4 --message-format=short
cargo test  -p semio-s-artifact-wfc-<artifact> --lib        --features component-app-<artifact> -j 4 -- --test-threads=4
cargo check -p semio-s-plugin-wfc --lib --target wasm32-wasip2      # type-checks even when the native host is broken

# oracles / cross-language differential
cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
bun ./📜️script.ts contract --owner "✏️s/🔌️plugins/🌊️wfc"
bun ./📜️script.ts parity exhaustive --owner 🌊️wfc --case mutate-<artifact>-1
cd /Users/ueli/Documents/semio

# descriptor + registry
bun nx run @semio-tech/wfc-plugin:describe
bun nx run @semio-tech/plugin-registry:check

# the aggregate gate, last
bun ./📜️script.ts verify gate
```

17. **Add a plugin-level `✏️s/🔌️plugins/🌊️wfc/🔮️oracles/🔣️.json`** declaring `oracleHostPackages` (§4.2b), or the Rust adapters have no host package to link.
18. **Copy `🌀️procedural`'s plugin-level lanes**: `🧪️tests/{🔬️surface, 🔬️boot-deadline, 😴️idle-turns, 🚪️close-ladder}` plus `🧫️fixtures/🚪️close-ladder/🔣️.json`, and declare the last three as `[[test]]` targets (§4.6).

---

## 7. Pitfalls checklist

1. **`timeout` does not exist on macOS.** Every `timeout N cargo …` line dies with `command not found` and leaves an empty log. Use the ticket's retry-loop shape (`[ "$code" -ne 137 ] && break`, `sleep 90`).
2. **Exit 137 = SIGKILL under swap.** Long cargo runs need the retry wrapper; `CARGO_PROFILE_WASM_DEV_DEBUG=false` cuts rustc RSS by ~50×.
3. **`include_str!` is compile-time.** A wrong fixture path is a crate-level compile error, not a test failure. 99/102 dangling at one point in Reference A.
4. **`aggregateVariant` mismatch is E0080.** The `dsl::MutationLeaf` derive const-asserts it against the Rust variant ident.
5. **`#[value(default)]` is per-field, unlike serde's `Self::default()`.** It makes `FromValue` demand `T: Default` for that field's type.
6. **`#[dsl(table)] Vec<String>` is wrong** — `VecTable` requires `T: DslRecord`. A `Vec<String>` is a `VecList`.
7. **`contentEncoding` is not in the JSON-Schema validator allowlist.** Use `"format": "base64"`. Every `$ref` must resolve locally; payload schemas must have no `$defs`.
8. **An orphaned `#[cfg(test)]` before a `//#region` comment binds to the next *item*,** silently cfg-gating a production function out of release builds.
9. **`store::ArtifactChild`'s `DslRecord` prints `child_id` in snake_case** among kebab-case neighbours, and the value bridge emits every unset diff lane as an explicit `null`. Both are encoded into the committed grammars — do not "fix" either without regenerating them.
10. **Renaming a DSL wire envelope (`semio <x>.dsl v1`, `artifact-mark`) is a wire-format change** and must go with a full fixture regeneration.
11. **Emoji filenames are never picked up as implicit integration tests.** Mount them with `#[cfg(test)] #[path = "…"] mod <ascii>;`, or declare an explicit `[[test]] name = "<ascii>" path = "<relative to the Cargo.toml dir>"` when the lane needs the public surface, its own binary (e.g. a `#[global_allocator]`), or `required-features`.
12. **A bare `cargo check -p <crate>` compiles only the schema half** when the crate declares `component-app-<artifact>` — editor/viewer/mount-contract need `--features component-app-<artifact>`. Reference A has no `[features]` at all, so its surfaces always compile; don't assume either shape.
13. **Emoji/semanticKind/variant collisions with a sibling mutation are refused** by the scaffolder, with a named error.
14. **`bun ./📜️script.ts new …` will not overwrite.** Existing files are reported as `skipped`; the mutation scaffolder rolls back every file it created if any step fails.
15. **iconIds are unvalidated strings at the builder.** A wrong name compiles and silently fails to render — Reference A ships three such call sites. Grep the id against `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs` first.
16. **A test case's `🥒️.feature` and its adapters must share ONE directory** or the case discovers with zero adapters (assembly's `mount-contract` is split and broken).
17. **Don't copy path strings out of assembly's `🔮️oracles/🔣️.json` `rationale`/`specificationSource`** — they name `🐍️component.py` / `🦀️component.rs`, which do not exist; the real leaves are `🐍️.py` / `🦀️.rs`.
18. **A new artifact/plugin/test-case directory name that is missing from `🔣️taxonomy.json`'s `semanticDirectoryMemberKinds` makes discovery silently return zero** for that owner, while downstream gates still read green.
19. **`verify gate` is the pre-close gate, not `verify`.** Bare `bun ./📜️script.ts verify` additionally runs `nx run-many -t test --all`, which will take hours.
20. **Do not sweep `🗑️generated` folders**, and do not place scratch files outside the ticket folder — both get swept.
