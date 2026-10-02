# 🧭️ Quiz product core — conventions to copy for a sibling product (`pets`)

Scope read: `🧰️framework/🛍️products/❓️quiz/` without the internals of `🎯️targets/⚛️react/🔨️modules`. Everything below was read on 2026-10-02 from the working tree (other agents were editing it concurrently; see section 8 for drift observed). All paths are repo-relative, `Q` abbreviates `🧰️framework/🛍️products/❓️quiz`, `T` abbreviates `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` (the repo test platform). References are `file:line`.

## 0. Decision-relevant summary

1. **Schema-first is hand-written, not generated.** `Q/🧬️schema/🔣️.json` (JSON Schema draft-07, only `$defs`, 74 entries) is normative; `🟦️.ts` (442 lines) and `🦀️.rs` (1047 lines) are hand-written twins, one type per `$defs` name. No codegen exists. They are held to the schema by (a) a Protocol v2 case where python-jsonschema judges every fixture document, (b) ajv in a TS unit suite, (c) a Rust serde round trip of every fixture document plus `deny_unknown_fields` tests, (d) the repo's generated schema catalog (export completeness per format).
2. **Twin layout is strictly "side by side"**: every module folder holds `🟦️.ts` + `🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`. The Rust test file is attached with `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` at the end of the module file. The Rust crate has no `[[test]]` entries; its `[lib] path` is the package glue `🦀️.rs` that `#[path]`-includes every module twin.
3. **A feature is specified once** as `Q/🧪️tests/<emoji><kebab>/🥒️.feature` (13 cases, 35 scenarios), with shared vectors in `Q/🧫️fixtures/<same name>/🔣️.json`, and executed by three adapters in the same folder (`🐍️.py` oracle, `🟦️.ts` subject, `🦀️.rs` subject). The harness (`T`) auto-discovers every `*.feature` (no registration), generates one nx project per case, and compares oracle ↔ TS ↔ Rust (`parity`).
4. **Third-party validation**: the oracle registry `Q/🔮️oracles/🔣️.json` names real libraries (numpy MT19937, scipy, python-jsonschema; JS: jstat, mathjs, ajv, …). Where no library implements the product policy, a second in-repo Python implementation is registered as `cross-semio-implementation` (a supplement, never third-party evidence). Vectors are generated from the Python adapters by a ticket-local script and the oracle phase asserts committed vector == library-backed recomputation.
5. **MT19937 lives in `Q/🔨️modules/🎲️randomness/{🟦️.ts,🦀️.rs}`** (`fnv1a32`, `runSeed`, `Mt19937`, `uniformIndex`/`uniform_index`, `shuffle`; integers only, no float API). Rust consumers use `quiz::{Mt19937, shuffle, uniform_index, fnv1a32, run_seed}`; TS consumers `@semio-tech/quiz`. The framework has a second, Rust-only RNG (xoshiro256** at `🧰️framework/🔨️modules/📐️geometry/🎲️random/🦀️.rs`) with no TS twin, so MT19937 is the only bit-exact cross-language RNG in the repo.
6. **Rust runtime deps are only `serde` (workspace-permitted) and optional `serde_json` behind feature `sut`**; the TS core has zero runtime deps (devDependencies: ajv, jstat, mathjs, typescript, vitest). A Rust dev-dependency (`unicode-normalization`) is used as an in-crate third-party oracle.
7. **Registries to touch for a new product** (details in section 7): `🛍️products/🔣️.json`, root `package.json` workspaces + scripts, root `Cargo.toml` members + `[workspace.dependencies]`, `🔣️taxonomy.json` member lists (`members-of-products/-modules/-tests/-fixtures`), `.vscode/🧩️launch.seed.jsonc` + generated `.vscode/launch.json`, generated `bun.lock`/`Cargo.lock`, generated schema catalog. nx project discovery and test-case discovery are automatic (file presence).
8. **The taxonomy area `🧰️framework` is enforced `clean`** (`🔣️taxonomy.json:30130` region): an unregistered product directory is a `directory-kind-unresolved` violation (verified: `🎤️presentation` currently reports 47 violations, quiz reports 1). Path rules: every segment carries exactly one emoji + U+FE0F; max 240 bytes per path.

## 1. Directory tree and file-name vocabulary

### 1.1 Tree (3 levels, `Q/`)

```
❓️quiz/
├─ README.md                      domain model + Layout table (README.md:10-30) + Commands (README.md:32-44)
├─ 🟦️.ts                          product barrel, 2 lines: `export * from "./📦️packages/🟦️typescript/🟦️.ts"`
├─ 🦀️.rs                          product façade: `pub use crate::{badges,lifecycle,presence,randomness,schema,scoring,sheet,validation,views}::*` (17 lines)
├─ 🧬️schema/                      the contract
│  ├─ 🔣️.json                     normative JSON Schema draft-07 (1130 lines)
│  ├─ 🟦️.ts                       TS twin (442 lines, `//#region` blocks)
│  ├─ 🦀️.rs                       Rust twin (1047 lines) + `#[cfg(test)] #[path]` at 1045-1047
│  └─ 🧪️tests/🔬️unit/🦀️.rs        serde round trips + shared test kit (160 lines)
├─ 🔨️modules/                     8 domain modules, each = 🟦️.ts + 🦀️.rs + 🧪️tests/🔬️unit/🦀️.rs
│  ├─ 🎲️randomness/               FNV-1a, MT19937, uniform index, Fisher–Yates
│  ├─ 🃏️sheet/                    sheetOf(quiz, seed) (pure, seeded)
│  ├─ ✅️validation/              owned structural+semantic validators (no schema library), handle policy
│  ├─ 📏️scoring/                 partial-credit scoring
│  ├─ 🏅️badges/                   badge rules
│  ├─ 🧾️lifecycle/               decide/evolve pairs (CQRS + event sourcing)
│  ├─ 👁️views/                    read models (catalog, learner, run, leaderboard, crowd)
│  └─ 👥️presence/                presence rooms, admission rules, roster
├─ 🔮️oracles/🔣️.json              owner oracle registry (314 lines): third-party references, comparison profiles, subject features, host packages
├─ 🧫️fixtures/<case>/🔣️.json      23 shared vectors (13 Protocol v2 vector files with a `$comment` "Generated by … never edit by hand"; the other 10 are React-side vectors, mostly with `schema` + `description`)
├─ 🧪️tests/                       41 dirs
│  ├─ 13 Protocol v2 cases        🥒️.feature + 🐍️.py + 🟦️.ts + 🦀️.rs  (answer-validation, sheet-assembly, seeded-randomness, badge-rules, leaderboard, shared-presence, crowd-view, sorting-concordance, matching-concordance, profile-similarity, schema-conformance, learner-lifecycle, identity-shapes)
│  ├─ 10 TS-core vitest suites    🟦️.ts only, listed in 🧪️tests/🎚️config/🟦️.ts:21-32 (mt19937-generator, sheet-randomization, document-validation, partial-credit-scoring, badge-awards, run-lifecycle, read-views, shared-vectors, presence-roster, crowd-answers)
│  ├─ 17 React vitest suites      🟦️.tsx only, run by the react target's config
│  └─ 🎚️config/🟦️.ts              vitest config of the TS core
├─ 📦️packages/
│  ├─ 🟦️typescript/              @semio-tech/quiz: package.json, 📋️project.json, 📜️script.ts, 🟦️.ts barrel (glue only)
│  └─ 🦀️rust/                    crate semio-framework-quiz (lib `quiz`): Cargo.toml, 📋️project.json, 📜️script.ts, 🦀️.rs glue (+ `dist/` build output)
└─ 🎯️targets/⚛️react/             @semio-tech/quiz-react
   ├─ 🟦️.tsx, 🎨️.css            target root (30 KB component barrel, styles)
   ├─ 🔨️modules/<26 modules>     React module twins (🟦️.tsx/🟦️.ts) — out of scope here
   ├─ 🧪️tests/🎚️config/🟦️.ts     vitest (jsdom) config
   └─ 📦️packages/🟦️typescript/  package.json, tsconfig.json, 📋️project.json, 📜️script.ts, 🟦️.tsx (glue only)
```

Layout rule (from the README table, `README.md:10-30`, and the team's rule "targets own packages; package roots glue only"): a **target** (`🎯️targets/⚛️react`) owns its sources and its own `📦️packages/🟦️typescript` glue package; the product-level `📦️packages` only re-export. The sibling `🎤️presentation` product violates this (`target-inside-package-boundary` in the inventory); quiz is the clean precedent.

### 1.2 Fixed file-name vocabulary

The leading emoji is the file kind from `🔣️taxonomy.json` `fileKinds` (`T/../📚️library/🔣️taxonomy.json`, 87 kinds); the emoji is always followed by U+FE0F (`variationSelectorPolicy.requiredAfterEmoji: true`).

| Name | Kind id (taxonomy) | Meaning in the quiz product |
|---|---|---|
| `🔣️.json` | `json` | The data file of a folder: schema (`🧬️schema/🔣️.json`), fixtures (`🧫️fixtures/<case>/🔣️.json`), oracle registry (`🔮️oracles/🔣️.json`), product collection (`🛍️products/🔣️.json`). No `schema.json` names allowed (`forbiddenPlacementPatterns` incl. `**/*.schema.json`) |
| `🟦️.ts` / `🟦️.tsx` | `typescript-source` | The module implementation (TS twin), package barrel, vitest suite or Protocol v2 TS adapter, depending on the folder |
| `🦀️.rs` | `rust-source` | Rust twin / package glue / unit tests (`🧪️tests/🔬️unit/🦀️.rs`) / Protocol v2 Rust adapter |
| `🐍️.py` | `python-source` | Protocol v2 Python **oracle adapter** (imports `semio_repo_test`, injected by the harness host `T/🖥️host/🐍️.py`) |
| `🥒️.feature` | `gherkin-feature` | The normative, language-neutral test contract of a case (`testFeatureFileKindId`) |
| `📋️project.json` | fixed filename | nx project config of a package (targets call `bun ./📜️script.ts …`) |
| `📜️script.ts` | fixed filename | The only script file allowed per package (`ScriptRouter` over `BundleScript` subclasses) |
| `🎨️.css` | `css` | Target stylesheet |
| `🌐️.html` | `html` | Site entry (teaching site only) |
| `package.json`, `Cargo.toml`, `tsconfig.json`, `README.md` | tool-fixed | Not emoji-prefixed |

Folder vocabulary (slug → emoji, `semanticDirectoryKinds` `🔣️taxonomy.json:3030`): `🧬️schema`, `🔨️modules`, `🔮️oracles`, `🧫️fixtures`, `🧪️tests`, `🔬️unit`, `📦️packages`, `🦀️rust`, `🟦️typescript`, `🎯️targets`, `⚛️react`, `🎚️config`, `🖼️assets`, `📚️examples`, `🚀️deploy`, `🏗️builder`. Test-case dirs are `<one emoji grapheme><kebab-slug>` (`testCaseSlugPattern`, `🔣️taxonomy.json:29739`); the case dir and the fixture dir carry the **same** name.

Docstring/section conventions seen in every file: each docstring starts with an emoji that is **unique within its file** (ticket checker `QUIZ-PRODUCT-AND-TEACHING-PROCTOR/domain_docstring_emojis.py`: "emojis that start more than one" docstring are findings); TS `/** 🎲️ … @see … */`, Rust `//!` module doc + `///` item docs, Python `"""🎲️ …"""`; regions `//#region 🔖️Name` / `//#endregion 🔖️Name` (TS and Rust) and `# region 🔖️Name` (Python); `@see` links to the twin and to the normative file; no comments inside definitions.

## 2. Schema-first

### 2.1 Dialect and shape of `Q/🧬️schema/🔣️.json`

* Header (`🔣️.json:1-7`): `"$schema": "http://json-schema.org/draft-07/schema#"`, `"$id": "https://json.schemas.assets.semio-tech.com/framework/product/quiz/schema.json"` (idBase + scope id `framework.product.quiz` as path + `schema.json`), `"title": "Quiz"`, `"description"`, `"$ref": "#/$defs/Quiz"`, then `"$defs": { … }`. Draft-07 is the single allowed dialect (`schemaJsonDialect`, `🔣️taxonomy.json`; breach `schema-dialect-not-draft-07`).
* Everything lives in `$defs`; **every `$defs` key is an "export"** (PascalCase, pattern `^[A-Z][A-Za-z0-9]*$` = `schemaExportResolution.exportIdPattern`) and must exist under the same name in every provided format (`🔣️jsonschema`, `🦀️rust`, `🟦️typescript` — `schemaFormats` at `🔣️taxonomy.json:29289`).
* Constructs used: `type: object` + `additionalProperties: false` + `required` + `properties`; `$ref`; `enum`; `const` as discriminator; `oneOf` for tagged unions (discriminators `kind` for tasks/answers/results/identities/badge rules and `type` for commands/events/queries); `propertyNames: {$ref: Slug}` + `additionalProperties` for id-keyed maps; `pattern`, `minLength/maxLength`, `minimum/maximum`, `minItems`, `uniqueItems`, `maxItems`.
* One custom keyword: **`x-semio-formats`** (`["🔣️jsonschema","🦀️rust","🟦️typescript"]`) on 56 of 74 `$defs` (all object-typed ones, e.g. `🔣️.json:36`, `:58`, `:69`); the 18 unannotated ones are scalars/enums/unions (`Slug, Id, Handle, Timestamp, Score, Scale, TaskKind, Motion, Profile, Task, BadgeRule, SheetTask, Answer, TaskResult, RunStatus, Screen, Anchor, ThinkingAnswer`). Semantics (library `T/../📚️library/🔍️discovery/🟦️.ts:3344-3351`, `:3622-3629`): the annotation *restricts* the formats an export must exist in; absent = all provided formats; a missing declaration is breach `schema-export-incomplete` (`T/🟦️.ts:4617`). It is declarative only — nothing is generated from it.
* Other keys: `"$schema": {"type":"string"}` is an allowed property of documents (`Quiz`, `Catalog`) so authored documents can point at the schema with a relative path; version tags are `const`s (`"schema": {"const":"semio.quiz/v1"}`, `"semio.quiz.catalog/v1"`).
* No `$ref` across files, no `definitions`, no `if/then`, no `format`.

### 2.2 How `🟦️.ts` and `🦀️.rs` relate

**Hand-written twins, one type per `$defs` name, validated by tests — not generated.** Stated in the files themselves: `🧬️schema/🟦️.ts:1-9` ("Schema-first — `🔣️.json` is the single source of truth; this module restates every `$defs` entry under the same name with `readonly` fields … No runtime dependency") and `🧬️schema/🦀️.rs:1-12` ("Hand-written Rust twin … One type per `$defs` entry, same names; fields are `camelCase` on the wire, tagged unions are internally tagged … optional fields are omitted when absent and unknown fields are refused like the schema's `additionalProperties: false`. Constraints serde cannot express … are checked by `crate::validation`").

Mapping table:

| JSON Schema | TS twin | Rust twin |
|---|---|---|
| object, `additionalProperties:false` | `export type X = { readonly … }` | `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(deny_unknown_fields)] pub struct X` (+ `rename_all = "camelCase"` when a field is multi-word) |
| string `enum` | `export const X = [...] as const; export type X = (typeof X)[number]` | `#[serde(rename_all = "kebab-case")] pub enum X` (+ sometimes a `pub const` array, e.g. `SCREENS: [Screen; 10]` at `🦀️.rs:960`) |
| `oneOf` tagged by `kind`/`type` const | union of ad-hoc named members (`ClassificationTask | SortingTask | …`; command/event variants are named `StartRunCommand`, `RunStartedEvent`, … although the schema leaves them anonymous) | `#[serde(tag = "kind"|"type", rename_all = "kebab-case")] pub enum X { Variant(Struct) | Variant { … } }`, struct variants add `rename_all_fields = "camelCase"` |
| optional property | `readonly x?: T` | `Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]` |
| `"$schema"` property | `readonly $schema?: string` | `#[serde(rename = "$schema", …)] json_schema: Option<String>` (`🦀️.rs:254`) |
| id-keyed map | `Readonly<Record<Slug, T>>` | `BTreeMap<Slug, T>` (deterministic order) |
| `integer`/`number` | `number` | `u32/u64/usize` / `f64` |
| pattern/length/min items/cross-field rules | not expressible | owned validator `Q/🔨️modules/✅️validation` (`quizIssues`/`quiz_issues`, … returning `{path, code}[]`) |

Deliberate non-1:1 spots to copy knowingly: `IdentityClaim` is its own `$defs` entry but a type alias of `Identity` in Rust (`🦀️.rs:547`); `Text` is `{en, de}` with **no default language** (`🔣️.json:35-45`); TS has runtime constants (`LANGUAGES`, `SCALES`, `TASK_KINDS`, `MOTIONS`, `REJECTIONS`, `SCREENS`, `DEFAULT_LIMITS`, `LEADERBOARD_TOP`) that Rust mirrors as enums/consts; client-only helpers have **no Rust twin** (`presenceRoster`, `thinkingAnswer`, `thinkingCrowd` exist only in `👥️presence/🟦️.ts`, grep of `pub fn` in `👥️presence/🦀️.rs` lists admission functions only).

### 2.3 Representative trio (`Motion` + `TaskIcon`; the animation-adjacent entry)

JSON, `🧬️schema/🔣️.json:53-67`:

```json
"Motion": {
  "description": "The looping microanimation of a task icon: bounce hops, pulse breathes, … Always still for learners who prefer reduced motion.",
  "enum": ["bounce", "pulse", "spin", "sway", "float", "flip"]
},
"TaskIcon": {
  "x-semio-formats": ["🔣️jsonschema", "🦀️rust", "🟦️typescript"],
  "description": "The icon of a task: one emoji grapheme that pictures what the task asks about, and the microanimation it plays.",
  "type": "object",
  "additionalProperties": false,
  "required": ["emoji", "motion"],
  "properties": {
    "emoji": { "type": "string", "minLength": 1, "maxLength": 16 },
    "motion": { "$ref": "#/$defs/Motion" }
  }
}
```

TS, `🧬️schema/🟦️.ts:45-49` and `:65-66`:

```ts
/** 🎞️ The looping microanimations of a task icon, still for learners who prefer reduced motion. */
export const MOTIONS = ["bounce", "pulse", "spin", "sway", "float", "flip"] as const;
/** 🎬️ One of {@link MOTIONS}. */
export type Motion = (typeof MOTIONS)[number];
/** 🖼️ The icon of a task: one emoji grapheme picturing what it asks about and the {@link Motion} it plays. */
export type TaskIcon = { readonly emoji: string; readonly motion: Motion };
```

Rust, `🧬️schema/🦀️.rs:95-113`:

```rust
/// 🎞️ The looping microanimation of a task icon, still for learners who prefer reduced motion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Motion { Bounce, Pulse, Spin, Sway, Float, Flip }   // one variant per line in the file

/// 🖼️ The icon of a task: one emoji grapheme picturing what it asks about and the [`Motion`] it plays.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskIcon { pub emoji: String, pub motion: Motion }  // one field per line in the file
```

A tagged union (`Task`): JSON `🔣️.json:219-225` (`oneOf` of three `$ref`s, members carry `"kind": {"const": …}`), TS `🟦️.ts:117-118` (`export type Task = ClassificationTask | SortingTask | MatchingTask`), Rust `🦀️.rs:209-216` (`#[serde(tag = "kind", rename_all = "kebab-case")] pub enum Task { Classification(ClassificationTask), … }`) plus accessor `impl Task { id(), kind(), title(), icon() }` (`🦀️.rs:218-248`).

### 2.4 What keeps the twins honest

1. `Q/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:58-115` — every fixture document (quizzes, catalogs, sheets, tasks, answers, results, badges, commands, events, views) is deserialized into the typed twin and re-serialized; `assert_close` (`:32-52`, tolerance 1e-12) must reproduce the JSON, so a missing/renamed field fails. `:135-145` pins `deny_unknown_fields` ≙ `additionalProperties:false`; `:117-133` pins wire tags.
2. Protocol v2 case `Q/🧪️tests/🧬️schema-conformance/` — python-jsonschema `Draft7Validator` reads the normative file and judges (a) every typed document in the committed vectors via a table of `(id, fixture, pointer-with-*, $defs entry, quizzes)` rows (`🥒️.feature:35-82`), (b) every authored `❓️quiz/🔣️.json` under `🎓️teaching`, (c) 24 deliberately broken documents each naming the violated keyword; TS (`quizIssues`/`catalogIssues`) and Rust (`quiz_issues`/`catalog_issues` after decoding into the twin) must accept/reject identically.
3. TS unit suite `Q/🧪️tests/🩺️document-validation/🟦️.ts:1-17` — ajv 8 compiles the normative file and must agree with the owned validator on every tested document.
4. Repo-level generated catalog `T/../📚️library/🔣️schema-catalog.json` (+ `📓️schema-catalog.md`): "Generated by `bun ./📜️script.ts schema generate` — never hand-edited"; the quiz scope is `framework.product.quiz` (`🔣️schema-catalog.json:7497`, `path: …/❓️quiz/🧬️schema`, `level: product-root`, formats `🦀️.rs/🟦️.ts/🔣️.json`, per-file SHA-256). **Currently stale for quiz**: 68 exports vs 74 `$defs` (missing `Handle, Motion, TaskIcon, IdentityClaim, Limits, HandleView`) and all three hashes differ — regenerate with `bun nx run workspace:schema-generate` (`package.json:231`).

## 3. Module twin convention (`🃏️sheet`, `📏️scoring`, `🎲️randomness`, `🏅️badges`)

### 3.1 Structure

```
🔨️modules/<emoji><name>/
├─ 🟦️.ts                       TS implementation; imports ONLY ../../🧬️schema/🟦️.ts and sibling modules' 🟦️.ts (e.g. scoring imports ../🃏️sheet/🟦️.ts, ../✅️validation/🟦️.ts)
├─ 🦀️.rs                       Rust implementation; imports crate::schema, crate::<sibling> (e.g. `use crate::sheet::…`); ends with the test attach
└─ 🧪️tests/🔬️unit/🦀️.rs        Rust unit tests; `use super::*;`
```

* **TS file head**: `/** 🃏️ <summary>\n *\n * @see ../🎲️randomness/🟦️.ts — … \n * @see ./🦀️.rs — the Rust twin\n */` (`🃏️sheet/🟦️.ts:1-5`, `📏️scoring/🟦️.ts:1-11`, `🏅️badges/🟦️.ts:1-5`).
* **Rust file head**: `//! 🃏️ <summary> (design §4) …\n//!\n//! @see ../🎲️randomness/🦀️.rs — …\n//! @see ../🃏️sheet/🟦️.ts — the TypeScript twin` (`🃏️sheet/🦀️.rs:1-7`). Note the Rust `@see` points at the TS twin.
* **Naming**: TS camelCase (`sheetOf`, `scoreTask`, `earnedBadges`, `uniformIndex`), Rust snake_case (`sheet_of`, `score_task`, `earned_badges`, `uniform_index`); types identical (`Sheet`, `Quiz`, `TaskResult`). Pure functions, no I/O, no classes except `Mt19937`. Module-private helpers are non-exported TS functions / private Rust fns. A function returning "can't score" is `T | undefined` in TS and `Option<T>` in Rust, with the doc stating the correspondence (`📏️scoring/🟦️.ts:3-10`).
* **Exports**: everything exported by a module is re-exported flat by the package barrels (section 5): TS `export * from "../../🔨️modules/…/🟦️.ts"`; Rust `pub use crate::<module>::*` in the product façade. Names must therefore be unique across modules.
* **Normative order of operations is part of the contract** and written into both file heads (e.g. RNG consumption order in `🃏️sheet/🟦️.ts:52` / `🦀️.rs:1-4`; summation order in `📏️scoring`), because bit-exactness across languages depends on it.
* **Docstrings**: first token is a fitting emoji on every docstring, meant to be unique per file (the ticket checker `domain_docstring_emojis.py` reports duplicates). Examples in `🃏️sheet/🟦️.ts`: `🪧️` (`sheetItem`), `🖼️` (`iconOf`), `✂️` (`drawn`), `📶️` (`ascendingItems`), `🧱️` (`sheetTask`), `🃏️` (module head `:1` and `sheetOf` `:52` — the quiz code itself has this one duplicate, so run the checker on new files); Rust `🃏️sheet/🦀️.rs`: `🃏️` (module), `🎰️` (`sheet_of`).

### 3.2 Rust tests attach

End of each module file (`🎲️randomness/🦀️.rs:94-96`, `🃏️sheet/🦀️.rs:92-94`, `🏅️badges/🦀️.rs:34-36`, `📏️scoring/🦀️.rs:167-169`, `🧬️schema/🦀️.rs:1045-1047`):

```rust
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;            // `pub(crate) mod tests;` in schema, sheet, lifecycle — their helpers are reused by sibling test modules
```

Test-file head: `//! 🪀️ Unit tests of … \n//!\n//! @see ../../🦀️.rs — the implementation under test\n\nuse super::*;`. Shared kit: `crate::schema::tests::{fixture, typed, json, entries, assert_close}` (`🧬️schema/🧪️tests/🔬️unit/🦀️.rs:12-52`) and `crate::sheet::tests::{text, quantity, classification, sorting, matching, quiz}` builders (`🃏️sheet/🧪️tests/🔬️unit/🦀️.rs:10-72`), imported by scoring/badges tests (`📏️scoring/🧪️tests/🔬️unit/🦀️.rs:8`). `fixture(case)` finds the folder under `CARGO_MANIFEST_DIR/../../🧫️fixtures` whose name `ends_with(case)` (skips the emoji prefix) — `🧬️schema/🧪️tests/🔬️unit/🦀️.rs:12-18`. **Level gating** is by submodule names: tests in `mod quick { … }`, `mod long`, `mod exhaustive` inside the test file (`🎲️randomness/🧪️tests/🔬️unit/🦀️.rs:125-137`), unscoped tests are `fundamental` (`runCargoTestBudgeted` doc, `T/../📚️library/🟦️.ts:1664-1672`, appends `--skip <level>::` for higher levels).

### 3.3 `🃏️sheet` — pure seeded function (best template for "pet spawn / fidget schedule from a seed")

* TS `sheetOf(quiz: Quiz, seed: number): Sheet` (`🃏️sheet/🟦️.ts:52-58`): `const random = new Mt19937(seed); const order = shuffle(random, quiz.tasks.map((_, i) => i)); const tasks = quiz.tasks.map((task) => sheetTask(task, random)); … seed: seed >>> 0`.
* Rust `sheet_of(quiz: &Quiz, seed: u32) -> Sheet` (`🃏️sheet/🦀️.rs:14-25`): same sequence with `&mut random`.
* Rust unit tests pin the consumption order by replaying the RNG by hand (`🃏️sheet/🧪️tests/🔬️unit/🦀️.rs:95-131`), purity (`:78-83`), and compare to the shared vectors (`:183-193`: `assert_close(&format!("sheet-assembly/{}", …), &json(&sheet_of(quiz, seed)), &vector["sheet"])`).

### 3.4 `📏️scoring` — floating-point twin

Order of float summation is normative so both cores agree "within 1e-12 (`log10` may differ by one ulp)" (`📏️scoring/🦀️.rs:1-4`, README:149); the comparison profile `quiz-score-v1` (tolerance 1e-12) encodes that (`🔮️oracles/🔣️.json:299-305`). Public: `scaled`, `scoreTask`, `scoreRun` / `scaled`, `score_task`, `score_run` (`🟦️.ts:40,157,166`; `🦀️.rs:18,26,40`).

### 3.5 MT19937 — where it lives and its API (reuse target)

Both in `Q/🔨️modules/🎲️randomness/`; both re-exported by the package barrels so consumers import from the package, never from the module path.

| Capability | TypeScript `@semio-tech/quiz` (`🎲️randomness/🟦️.ts`) | Rust crate `quiz` (`🎲️randomness/🦀️.rs`) |
|---|---|---|
| FNV-1a 32 over UTF-8 | `fnv1a32(text: string): number` `:19-23` | `fnv1a32(text: &str) -> u32` `:17-19` |
| Run seed | `runSeed(run: string): number` `:26-28` | `run_seed(run: &str) -> u32` `:22-24` |
| Generator | `class Mt19937 implements RandomSource { constructor(seed: number); next(): number }` `:36-67` (`seed >>> 0`; `init_genrand`; tempered u32 as unsigned number) | `pub struct Mt19937 {…}` `#[derive(Clone, Debug)]`; `Mt19937::new(seed: u32)`, `next_u32(&mut self) -> u32` `:28-66` |
| Generator abstraction | `interface RandomSource { next(): number }` `:31-33` | none — functions take `&mut Mt19937` directly |
| Bounded draw | `uniformIndex(random: RandomSource, n: number): number` `:70-77` (rejection sampling; `n ≤ 1 → 0` **without** a draw; limit `2³² − (2³² mod n)`) | `uniform_index(random: &mut Mt19937, n: usize) -> usize` `:70-82` (same, in u64 arithmetic) |
| Shuffle | `shuffle<T>(random, items: readonly T[]): T[]` `:80-87` (copy; Fisher–Yates from the end, `j = uniformIndex(i+1)` for `i = len−1 … 1`) | `shuffle<T: Clone>(random: &mut Mt19937, items: &[T]) -> Vec<T>` `:85-92` |
| Float in [0,1) | **not provided** | **not provided** |

Reuse notes: (1) a sibling product depends on the quiz product exactly as `🎓️teaching/🛂️proctor` does (Cargo `semio-framework-quiz = { workspace = true }`, `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/Cargo.toml:25`; TS `"@semio-tech/quiz": "workspace:*"`, see `…/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/package.json:16`) — or the RNG is extracted into a framework module first (AGENTS.md "domain-neutral framework + domain-specific extensions"; none exists today). (2) A float/unit draw (needed for jitter, fidget timers, blink intervals) must be added identically in both twins (e.g. build on `next()` only) and given a numpy/CPython oracle: numpy and CPython `random.Random` both implement `genrand_res53` (`random_sample`/`random()`), which is the natural third-party check next to the existing raw-word oracle. (3) Consumption order of the stream must be documented in the file heads and pinned by a replay test in Rust like `sheet_follows_the_rng_consumption_order`.
(4) Independent third-party checks of the generator already exist: numpy `RandomState(seed)` + `MT19937.random_raw` (`quiz-numpy-mt19937`, `🔮️oracles/🔣️.json:6-22`), CPython `random.Random.setstate` with the same 624 words (`🧪️tests/🎲️seeded-randomness/🐍️.py:57-61`), the C++ standard check value (10000th word of seed 5489 = 4123659995; `🎲️randomness/🧪️tests/🔬️unit/🦀️.rs:22-28`, vitest `🧪️tests/🌀️mt19937-generator/🟦️.ts:56-60`).

## 4. Language-agnostic tests and third-party validation

### 4.1 Two layers, same fixtures

* **Protocol v2 cases** (`Q/🧪️tests/<case>/{🥒️.feature,🐍️.py,🟦️.ts,🦀️.rs}`): executed by the repo test platform `T`, discovered automatically (`T/🟨️.mjs:183-257`: every `**/*.feature` under a `🧪️tests/<emoji><kebab>/` dir → one nx project with targets `lint, test-contract, test-oracle, test-subject, test-parity, test, test-quick, test-long, test-exhaustive`; project name `test-<owner-segments>-<sha256(owner)[0:6]>-<case>`; for quiz owner the hash is `410898`).
* **Unit layers** that read the same `🧫️fixtures`: TS vitest suites (`🧪️tests/<name>/🟦️.ts`, configured by `🧪️tests/🎚️config/🟦️.ts:21-32`, **explicit include list — new suites must be added there**) and Rust `cargo test` unit modules (`🧪️tests/🔬️unit/🦀️.rs` per module).

### 4.2 Feature grammar (enforced by `contract`, `T/🟦️.ts:429-472`, `:1750-1790`)

Feature tags: `@capability-<id>`, `@oracle-<registered id>` (or `@no-oracle-<decision id>`), `@comparison-<profile id>`; per scenario: `@id-<kebab>` (the adapter key), exactly one `@level-<fundamental|quick|long|exhaustive>`, exactly one `@mode-<differential|conformance|round-trip|property|error>`. Comparison profiles: framework ones in `T/🟦️.ts:73-84` (`ordered-json-v1`, `floating-point-v1` tolerance 1e-9, …) plus owner profiles in `🔮️oracles/🔣️.json:299-305` (`quiz-score-v1`, 1e-12). Fixture URIs in steps are `shared://<case>/🔣️.json` (resolved under the owner's `🧫️fixtures`).

### 4.3 One complete small example: `🎲️seeded-randomness`

**Feature** `Q/🧪️tests/🎲️seeded-randomness/🥒️.feature:1-4, 31-37`:

```gherkin
@capability-quiz-randomness
@oracle-quiz-numpy-mt19937
@comparison-ordered-json-v1
Feature: Seeded randomness is bit-exact across languages
  … The vectors shared://🎲️seeded-randomness/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`

  @id-hashes
  @level-fundamental
  @mode-differential
  Scenario: FNV-1a hashes the UTF-8 bytes of a text
    Given the committed vectors shared://🎲️seeded-randomness/🔣️.json
    When every committed text is hashed with fnv1a32
    Then every implementation projects the same 32-bit hash per vector, including the empty text's offset basis 2166136261
```

(5 scenarios: `hashes`, `run-seeds`, `raw-outputs`, `uniform-draws`, `shuffles`.)

**Fixture** `Q/🧫️fixtures/🎲️seeded-randomness/🔣️.json:1-8, 57-70`: `{"$comment": "Generated by … never edit by hand.", "hashes": [{"id":"empty","text":"","hash":2166136261}, …], "runSeeds": [{"id","run","seed"}], "rawOutputs": [{"id":"seed-5489","seed":5489,"skip":0,"count":8,"outputs":[3499211612, 581869302, …]}], "uniformDraws": [{id,seed,bounds,expected:{draws,next}}], "shuffles": [{id,seed,length,expected:{permutation,next}}]}`. Inputs + expected outputs live side by side per vector id; every draw/shuffle vector also projects the *next* raw word so consuming one word too many/few is detected.

**TS adapter** `Q/🧪️tests/🎲️seeded-randomness/🟦️.ts:44-52`:

```ts
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    hashes: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).hashes.map((vector) => [vector.id, fnv1a32(vector.text)])) }) },
    "run-seeds": { subject: (ctx) => ({ projection: … runSeed(vector.run) … }) },
    "raw-outputs": { subject: (ctx) => ({ projection: … words(vector.seed, vector.skip, vector.count) … }) },
    "uniform-draws": …, shuffles: …,
  },
});
```

(imports `defineTestAdapter` from `../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` and the subject from `../../📦️packages/🟦️typescript/🟦️.ts`; the handler key is the `@id-` of the scenario; fixtures via `ctx.fixtureBytes("shared://…")`.)

**Rust adapter** `Q/🧪️tests/🎲️seeded-randomness/🦀️.rs:6-9, 47-49, 85-91`:

```rust
use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, json, Map, Value};
    use quiz::{fnv1a32, run_seed, shuffle, uniform_index, Mt19937};
    use semio_repo_test_host::{parse_json, Context, Outcome};
    pub fn hashes(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "hashes")?, |vector| Ok(json!(fnv1a32(text(vector, "text")?))))
    }
    // …
}
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("hashes", subject::hashes).subject("run-seeds", subject::run_seeds) /* … */;
    built
}
```

The adapter is compiled into a generated host crate under `.🧬semio/🦑️repo/⚡️cache/tests/hosts/` that links the owner's `📦️packages/🦀️rust` crate (`rustSubjectPackage`, `T/🕸️dependencies/🟨️.mjs:25-32` — the nearest ancestor `📦️packages/🦀️rust`) with the features named in `🔮️oracles/🔣️.json:306-308` (`subjectFeatures: [{implementation: "rust", features: ["sut"]}]`); that is why the crate re-exports `serde_json` only under `sut` (`📦️packages/🦀️rust/🦀️.rs:34-36`).

**Python oracle** `Q/🧪️tests/🎲️seeded-randomness/🐍️.py:34-39, 42-61, 88-104, 153-155`:

```python
from semio_repo_test import Adapter, Outcome        # injected by T/🖥️host/🐍️.py
import numpy, random
class Mt19937:   # numpy's own bit generator seeded via RandomState(seed) (init_genrand), read with random_raw
    def __init__(self, seed):
        state = numpy.random.RandomState(seed).get_state()
        self.generator = numpy.random.MT19937(); self.generator.state = {"bit_generator": "MT19937", "state": {"key": state[1], "pos": state[2]}}
    def cpython_twin(self): …   # CPython random.Random.setstate((3, key + (pos,), None)) — an unrelated MT19937
def agree(scenario, produced, vectors, field):   # the committed vector may never drift from the reference
    for vector in vectors:
        if produced[vector["id"]] != vector[field]: raise AssertionError(…)
    return Outcome(produced)
def adapter():
    return Adapter("python").oracle("hashes", hashes).oracle("run-seeds", run_seeds)…
```

`agree()` (`🐍️.py:93-98`) is the guard: the oracle phase recomputes with the library and fails if the committed vector differs; the `raw-outputs` handler additionally requires numpy's words == CPython's words (`:113-125`).

**Vector generation**: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py` (1428 lines; ticket-local script, deliberately kept in the ticket folder) loads each case's `🐍️.py` as a module (`:22-47`, `semio_repo_test` shim = `T/🖥️host/🐍️.py`), composes inputs, computes expected outputs **with the oracle code**, and writes `🧫️fixtures/<case>/🔣️.json` as `json.dumps(…, ensure_ascii=False, indent=2) + "\n"` (`write()` at `:50-68`); `randomness()` is at `:301-334`; entry point `:1415-1428`. Run: `.venv/Scripts/python.exe <script>` from the repo root (`.venv/bin/python` elsewhere). A pets ticket should keep its own generator in `.🧬semio/…/QUIZ-PETS/` the same way and name it in each feature header.

### 4.4 Oracle kinds used (`Q/🔮️oracles/🔣️.json`)

| id | kind | ecosystem | judges |
|---|---|---|---|
| `quiz-numpy-mt19937` | `third-party-library` (numpy 2.5.0) | python | randomness (`:6-22`) |
| `quiz-scipy-stats` | `third-party-library` (scipy 1.18.0 + numpy) | python | sorting/matching/classification scores: Kendall τ, `scipy.spatial.distance` (`:23-42`) |
| `quiz-jsonschema` | `third-party-library` (jsonschema 4.26.0) | python | schema-conformance, presence, identity shapes (`:43-59`) |
| `quiz-python-reference` | **`cross-semio-implementation`** (second implementation from the design text, numpy stream) | python | sheet assembly, validation, badges, leaderboard, lifecycle, crowd (`:60-76`) — "a supplement, never third-party evidence" |
| `quiz-jstat-spearman`, `quiz-mathjs-distance`, `quiz-ajv-structure` | third-party-library | javascript, `hostPath` = TS package | judged inside vitest unit suites (`:77-130`) |
| `quiz-react-*` (d3-scale, opentype.js, lightningcss, lodash×2, colord, aria-query, i18next) | third-party-library | javascript, `hostPath` = react package | React-side suites (`:131-296`) |

Registry fields every entry carries: `kind, ecosystem, package, version, source{repository,license}, engine{family,implementation,version}, capabilities[], comparisonProfiles[], license, testOnly:true, productionReachable:false, networkDuringExecution:false, homepage, rationale` (survey of declined candidates belongs in the rationale), `hostPath` for JS oracles hosted in a vitest suite. Top-level blocks: `schemaVersion: 2`, `$schema: ../../🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`, `oracles`, `noOracleDecisions: []`, `comparisonProfiles`, `subjectFeatures`, `oracleHostPackages` (python numpy/scipy/jsonschema with versions; these are already declared in root `pyproject.toml` dev/test groups and `uv.lock`, so a new Python oracle library needs a `pyproject.toml` + `uv.lock` entry — and a `🔒️dependencies.json` classification as `test-oracle`, which is a generated file, stale for quiz).

### 4.5 Other third-party validation layers inside the unit suites

* TS: `ajv` (`🩺️document-validation`), `jstat.spearmancoeff` and `mathjs.distance` (`⚖️partial-credit-scoring/🟦️.ts:1-17`), numpy-recorded MT19937 vectors pasted with provenance (`🌀️mt19937-generator/🟦️.ts:4-12`).
* Rust: `unicode-normalization` as dev-dependency oracle of the handle tables (`📦️packages/🦀️rust/Cargo.toml:26-29`).
* Already installed and usable for an animation/rig product (checked in `node_modules`): `three` 0.182, `gl-matrix` 3.4.3, `d3-ease` 3.0.1, `d3-interpolate` 3.0.1, `motion`/`framer-motion` 12.40, `mathjs` 14, `colord` 2.10; Python (in `pyproject.toml`): `numpy`, `scipy` (e.g. `scipy.spatial.transform.Rotation`, `scipy.interpolate`), `shapely`, `networkx`, `jsonschema`.

## 5. Packages

### 5.1 Rust — `Q/📦️packages/🦀️rust/`

`Cargo.toml:1-29` (complete):

```toml
[package]
name = "semio-framework-quiz"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Quiz product core: …"

[package.metadata.semio]
role = "product"
id = "quiz"

[lints]
workspace = true

[lib]
name = "quiz"
path = "🦀️.rs"

[features]
sut = ["dep:serde_json"]

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true, optional = true }

[dev-dependencies]
serde_json = { workspace = true }
# 🔮️ Third-party oracle of the handle tables: NFC, NFKC and the composition of pairs.
unicode-normalization = "0.1.25"
```

* **No `[[test]]` entries.** `[lib] path = "🦀️.rs"` is the package-local glue file (`🦀️rust/🦀️.rs:1-36`, "Package glue — wiring only"), which mounts each twin by relative `#[path]`:

```rust
#[path = "../../🧬️schema/🦀️.rs"]
pub mod schema;
#[path = "../../🔨️modules/🎲️randomness/🦀️.rs"]
pub mod randomness;
… (sheet, validation, scoring, badges, lifecycle, views, presence)
#[path = "../../🦀️.rs"]
mod component;
pub use component::*;
#[cfg(feature = "sut")]
pub use serde_json;
```

* Workspace membership: root `Cargo.toml:155` (`members`, grouped with the other framework product crates, not sorted) and `Cargo.toml:366` (`[workspace.dependencies] semio-framework-quiz = { path = "🧰️framework/🛍️products/❓️quiz/📦️packages/🦀️rust" }`, used by consumers with `{ workspace = true }`). Workspace-wide: `[workspace.package] rust-version = "1.95"` (`:295-298`), `serde 1.0.228` / `serde_json 1.0.149` at `:424-425`, `[workspace.lints.rust|clippy]` at `:814`/`:821`. Toolchain `rust-toolchain.toml` = `nightly-2026-07-07`; `rustfmt.toml` `max_width = 250` (explains the long lines). `Cargo.lock:6218` lists the crate.
* `serde` is the one external runtime crate; the test platform README exempts workspace-pinned crates (`T/README.md:168-179`, "Workspace-permitted crates").
* `📋️project.json` (65 lines): name `@semio-tech/quiz-rs`, `root`/`sourceRoot` = the package dir, `tags: ["lang:rust","role:product","family:quiz"]`, `namedInputs.default` = `{workspaceRoot}/🧰️framework/🛍️products/❓️quiz/**/*.rs`, `…/🧫️fixtures/**/*`, `{projectRoot}/**/*`; targets `build` (`bun ./📜️script.ts build`, cache, outputs `{projectRoot}/dist/build`), `test`, `test-quick`, `test-long`, `test-exhaustive` (all `nx:run-commands` with `cwd` = the package dir and `forwardAllArgs: true`).
* `📜️script.ts` (24 lines): `TestScript` → `runCargoTestBudgeted(["semio-framework-quiz"], this.repoRoot, rest)` after `resolveTestLevel(segments)`; `BuildScript` → `buildCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot)`; `new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript)`; `await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" })`. Imports go through `../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` (5 levels up — identical for any sibling product).

### 5.2 TypeScript core — `Q/📦️packages/🟦️typescript/`

`package.json:1-32`: `"name": "@semio-tech/quiz"`, `"version": "0.1.0"`, `"type": "module"`, `"private": true`, `"exports": { ".": "./🟦️.ts" }` (source is consumed directly, no build), `"license": "LGPL-3.0-or-later"`, `"semio": {"role":"framework","id":"quiz"}`, `"repository": {…,"directory": "<package path>"}`, `"bundleKind": "library"`, `devDependencies: ajv 8.20.0, jstat 1.9.6, mathjs 14.0.0, typescript ^5.9.3, vitest ^4.0.17` (no `dependencies` — "Zero runtime imports", README:24), `"nx": {"includedScripts": []}`.

`🟦️.ts` (barrel, glue only, 10 lines):

```ts
/** ❓️ `@semio-tech/quiz`: package glue over the quiz schema twin and its render-independent domain modules. */
export * from "../../🧬️schema/🟦️.ts";
export * from "../../🔨️modules/🎲️randomness/🟦️.ts";
… (sheet, validation, scoring, badges, lifecycle, views, presence)
```

`📋️project.json` (48 lines): `name: "@semio-tech/quiz"`, `namedInputs.default` = `{projectRoot}/**/*`, product `🟦️.ts`, `🧬️schema/**/*`, `🔨️modules/**/🟦️.ts`, `🧪️tests/**/🟦️.ts`, `🧫️fixtures/**/*`; targets `test`, `test-quick`, `test-long`, `test-exhaustive` (`bun ./📜️script.ts test [level]`, `cwd` = package dir). `📜️script.ts` (14 lines): `TestScript` → `runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts")`.

**No `tsconfig.json` for the core**; it resolves through bun workspaces (`node_modules/@semio-tech/quiz` link) and the root `tsconfig.json` (module `ESNext`, `moduleResolution: bundler`, `strict`, `allowImportingTsExtensions`, `noEmit`; includes `**/*.ts`). Imports always spell the `.ts` extension. `🧪️tests/🎚️config/🟦️.ts` (36 lines): vitest `root` = package dir, `resolve.alias["@semio-tech/quiz"] → …/🟦️.ts`, `environment: "node"`, explicit `include` list of suites, `passWithNoTests: false`.

### 5.3 React target — `Q/🎯️targets/⚛️react/📦️packages/🟦️typescript/`

`package.json`: `@semio-tech/quiz-react`, `exports: {".": "./🟦️.tsx", "./🎨️.css": "../../🎨️.css"}`, `bundleKind: "ui"`, deps `@semio-tech/{framework,framework-server,quiz,ui-react}` as `workspace:*`, `react`/`react-dom ^19.2.3`; devDeps the React-side oracles (d3-scale, opentype.js, lightningcss, lodash, colord, aria-query, i18next, jsdom, testing-library, vitest). `tsconfig.json` extends `../../../../../../../tsconfig.json` (7 levels) with explicit `paths` for `@semio-tech/quiz-react`, `@semio-tech/quiz`, `@semio-tech/framework`, `@semio-tech/framework-server`, `@semio-tech/ui-react{,/i18n,/chrome}` and an explicit `include` list of every `🧪️tests/<case>/🟦️.tsx`. `📋️project.json` adds a `typecheck` target (`bun ./📜️script.ts typecheck` → `tsc --noEmit -p tsconfig.json`). Vitest config `Q/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` aliases the same workspace packages and lists the suites (`include`, `environment: "jsdom"`, `setupFiles` = the ui react-environment).

## 6. How tests are run

All commands from `C:\git\semio` unless a `cd` is shown; devs normally use the launch rows (section 7).

| What | Command | Notes |
|---|---|---|
| TS core (vitest) | `bun nx run @semio-tech/quiz:test` (= `bun run test:quiz`, `package.json:222`) or `cd "🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript" && bun ./📜️script.ts test quick` | levels `fundamental|quick|long|exhaustive`; `exhaustive` also enables coverage (`resolveTestLevel`, `T/../📚️library/🟦️.ts:1035-1045`) |
| Rust core (cargo) | `bun nx run @semio-tech/quiz-rs:test` (= `bun run test:quiz:rs`, `package.json:224`) | `runCargoTestBudgeted(["semio-framework-quiz"])`; nextest profile per level; `RUST_MIN_STACK` 128 MiB |
| React target | `bun nx run @semio-tech/quiz-react:test` / `:typecheck` (`package.json:221,223`) | jsdom |
| Python oracles (Protocol v2 oracle phase) | `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | runs only the `🐍️.py` adapters (numpy/scipy/jsonschema from the harness venv `.🧬semio/🦑️repo/⚡️cache/tests/hosts/`, base `.venv`); fails if a committed vector differs from the library-backed recomputation |
| Subjects only | `… bun ./📜️script.ts subject exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | TS + Rust adapters; Rust host compiled with feature `sut` |
| **Parity** | `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | the full check; narrow with `--case <slug>`, `--implementation <id>`, `--project <nx name>` (`T/🔍️discovery/🎛️selection/🟦️.ts:4-24`) |
| Contract only (no execution) | `… bun ./📜️script.ts contract --owner "…"` | feature grammar, registered oracle/profile ids, adapters present |
| One case through nx | `bun nx run test-framework-products-quiz-410898-seeded-randomness:test-parity` | generated project, targets listed in 4.1 |

**What `parity` checks** (`T/⚖️parity/📋️orchestration/🟦️.ts:73-215`, README "Adding a feature" `T/README.md:69-95`): for every selected case it (1) resolves the oracle from the `@oracle-` tag and runs it in the adapter language that hosts it (Python), (2) dispatches a subject run for **every language in which the owner ships a package** (`ownerShipsImplementation`: an ancestor `📦️packages/🦀️rust` / `🟦️typescript` must exist — `T/🖥️host/🏗️materialization/🟦️.ts:65-75`; a missing subject handler then fails loudly per scenario), (3) compares each subject projection with the oracle projection under the case's comparison profile (`evaluateParity`, diffs written as `*.diff.txt`), and (4) compares the subjects pairwise (`evaluateCrossSubjectParity`) so two implementations cannot exploit different oracle ambiguities. A selected case with no result is reported `not-exercised`, never as a pass. Summary line: `[test] level=… cases=… executed=… passed=… failed=… errored=… parity=<equal>/<total>`; reports under `.🧬semio/🦑️repo/⚡️cache/tests/…/{📊️summary.json,📤️results.jsonl,📋️junit.xml}`. Recorded results for quiz: 75/75 (first revision), 84/84, 93/93 (11 cases) per `…/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️closing-summary.md:32,77,152`; the tree has 13 cases / 35 scenarios now.

**What registers a test with parity**: only *file presence* — a directory `<owner>/🧪️tests/<emoji><kebab>/` containing `🥒️.feature` (taxonomy `testFeatureFileKindId = gherkin-feature`), an owner `🔮️oracles/🔣️.json` entry matching the `@oracle-` tag, the adapters named by `testAdapterFileKinds` (`🦀️.rs, 🟦️.ts, 🐹️.go, 🐍️.py, 🔷️.cs`), and the fixture dir with the same name under `<owner>/🧫️fixtures/`. No hand-written `📋️project.json` for tests; the nx plugin `T/🟨️.mjs` (registered at `nx.json:60`) globs `**/*.feature`.

## 7. Checklist: adding a sibling product `🧰️framework/🛍️products/<emoji>pets`

Names below use `P` = `🧰️framework/🛍️products/<emoji>pets`, product slug `pets`, TS package `@semio-tech/pets`, crate `semio-framework-pets` (lib `pets`), nx `@semio-tech/pets-rs`, react `@semio-tech/pets-react`. Every folder/file name needs its emoji **plus U+FE0F**; max 240 bytes per path; one emoji grapheme per segment; names unique among siblings.

### 7.1 Files to create (mirror of quiz)

```
P/README.md                                   layout table + commands + domain model (copy the quiz README sections 10-44 shape)
P/🟦️.ts                                      export * from "./📦️packages/🟦️typescript/🟦️.ts";
P/🦀️.rs                                      //! façade; `pub use crate::<module>::*;` for every module + schema
P/🧬️schema/🔣️.json                           draft-07; "$id": "https://json.schemas.assets.semio-tech.com/framework/product/pets/schema.json"; "$ref": "#/$defs/<Root>"; every $defs PascalCase, objects with additionalProperties:false and "x-semio-formats":["🔣️jsonschema","🦀️rust","🟦️typescript"]; Text = {en,de}, no default language
P/🧬️schema/🟦️.ts                             twin (//#region blocks; readonly types + as-const arrays)
P/🧬️schema/🦀️.rs                             twin (serde derive, deny_unknown_fields, tagged enums) + `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] pub(crate) mod tests;`
P/🧬️schema/🧪️tests/🔬️unit/🦀️.rs              fixture(case)/typed/json/entries/assert_close kit + round-trip of every fixture document
P/🔨️modules/<m>/{🟦️.ts,🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}   ×3-4 twin modules (+ `✅️validation`-style owned validator module for the document schema, no schema library at runtime)
P/🔮️oracles/🔣️.json                           oracles (third-party first), noOracleDecisions, comparisonProfiles (own float profile), subjectFeatures [{rust, ["sut"]}], oracleHostPackages
P/🧫️fixtures/<emoji><case>/🔣️.json            "$comment": "Generated by <ticket script> … never edit by hand." + inputs with expected outputs
P/🧪️tests/<emoji><case>/{🥒️.feature,🐍️.py,🟦️.ts,🦀️.rs}   one dir per case, same name as the fixture dir
P/🧪️tests/<emoji><suite>/🟦️.ts                TS-core vitest suites (third-party devDependencies allowed)
P/🧪️tests/🎚️config/🟦️.ts                     vitest config with explicit `include` of the suites above
P/📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts,🟦️.ts}
P/📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts,🦀️.rs}     (+ `[features] sut = ["dep:serde_json"]`, `[lib] name = "pets" path = "🦀️.rs"`, `[lints] workspace = true`)
P/🎯️targets/⚛️react/{🟦️.tsx,🎨️.css,🔨️modules/…,🧪️tests/🎚️config/🟦️.ts,📦️packages/🟦️typescript/{package.json,tsconfig.json,📋️project.json,📜️script.ts,🟦️.tsx}}
```

Relative-path depths are identical to quiz (sibling products): core/rust package scripts import the repo library via `../../../../../🧰️framework/…/📚️library/📦️packages/🟦️typescript/🟦️.ts`, `$schema` `../../../../../node_modules/nx/schemas/project-schema.json`; react package uses 7 levels; test adapters import the harness via `../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`; oracles `$schema` is `../../🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`.

### 7.2 Registries to touch (exact paths and where quiz sits today)

| # | Registry | Quiz entry today | Action for pets | Hand-edit or generated |
|---|---|---|---|---|
| 1 | `🧰️framework/🛍️products/🔣️.json` | `x-semio.members[]` item at lines 23-28: `{"directory":"❓️quiz","id":"framework.product.quiz","kind":"product","responsibility":"…"}` | add `{"directory":"<emoji>pets","id":"framework.product.pets","kind":"product","responsibility":"…"}` (the id becomes the schema scope id) | hand |
| 2 | root `package.json` `workspaces` | lines 111-112: `…/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript`, `…/❓️quiz/📦️packages/🟦️typescript` (list is sorted by code point; ❓ U+2753 < 🎤 U+1F3A4 < 💻 U+1F4BB — insert pets at its code-point position) | add both `P/📦️packages/🟦️typescript` and `P/🎯️targets/⚛️react/📦️packages/🟦️typescript` | hand |
| 3 | root `package.json` `scripts` | lines 221-224: `typecheck:quiz:react`, `test:quiz`, `test:quiz:react`, `test:quiz:rs` (each `bun nx run <project>:<target>`) | add `test:pets`, `test:pets:rs`, `test:pets:react`, `typecheck:pets:react` | hand |
| 4 | root `Cargo.toml` `[workspace] members` | line 155 `"🧰️framework/🛍️products/❓️quiz/📦️packages/🦀️rust"` | add `P/📦️packages/🦀️rust` | hand |
| 5 | root `Cargo.toml` `[workspace.dependencies]` | line 366 `semio-framework-quiz = { path = "…/❓️quiz/📦️packages/🦀️rust" }` | add `semio-framework-pets = { path = … }` (only needed if another crate depends on it) | hand |
| 6 | `bun.lock` / `Cargo.lock` | `bun.lock:1061,1094,2420,2422` (workspace + `@semio-tech/quiz`, `@semio-tech/quiz-react`), `Cargo.lock:6218` | regenerate by `bun install` / cargo | generated |
| 7 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | `members-of-products` `:12352` (`["📓️print","🖥️server","❓️quiz"]`), `members-of-modules` `:12071`, `members-of-tests` `:14114`, `members-of-fixtures` `:10644` (each `{ownerKindIds, memberNames[], source:"registry"}`) | add the product dir to `members-of-products`, each new module dir to `members-of-modules`, each new case/suite dir to `members-of-tests`, each fixture dir to `members-of-fixtures`; new non-generic folder names (e.g. `🎞️`-style kinds) need `semanticDirectoryKinds` (`:3030`) entries. Use the ticket's idempotent helper pattern `…/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/site_infra_register_taxonomy.ts` (runs `inventoryTaxonomy({repoRoot, scope})` and appends every `directory-kind-unresolved` name to the member list of its parent) | hand (list edits, NFC names) |
| 8 | `.vscode/🧩️launch.seed.jsonc` | rows at lines 1197-1258: `🧪️test❓️quiz🟦️` (`bun nx run @semio-tech/quiz:test`, group `3_dev`, order 213.63), `🧪️test❓️quiz⚛️react` (213.64), `🧪️test❓️quiz🦀️` (213.65), `🛠️dev❓️quiz⚛️react🪁️typecheck` (213.68) | add rows `🧪️test<emoji>pets🟦️`, `…⚛️react`, `…🦀️`, `🛠️dev<emoji>pets⚛️react🪁️typecheck` with the same shape (`type: node-terminal`, `request: launch`, `cwd: ${workspaceFolder}`, `presentation.group: "3_dev"`, a fresh unique `order` such as 213.69+) following "verb-emoji + verb + product + language-emoji" naming | hand |
| 9 | `.vscode/launch.json` | same rows at 2088-2150 (+ project names in the `inputs` pickString option lists, e.g. `@semio-tech/quiz-rs` at 20458, 21226-21228, 21559-21560, …) | regenerate: `bun nx run @semio-tech/plugin-registry:generate` (writes `.vscode/launch.json` from seed + nx targets; `GenerateScript` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts:358-377`, row/dropdown synthesis at `…/📇️registry/🚀️launch/🟦️.ts:285-330`; freshness enforced by `…:check-generated`) | generated from #8 |
| 10 | schema catalog `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/{🔣️schema-catalog.json,📓️schema-catalog.md}` | scope `framework.product.quiz` | `bun nx run workspace:schema-generate` (`package.json:231`), then `schema:check` | generated |
| 11 | nx project graph | none to edit — plugin `🟨️.mjs` (`nx.json:55-57`) infers projects from `**/📋️project.json`, `**/Cargo.toml`, `bun.lock`; test cases from `**/*.feature` (`nx.json:60`) | nothing | automatic |
| 12 | `tsconfig` paths | only the **react** package tsconfig has `paths` (`Q/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json:7-16`); the core relies on the bun workspace link | write the react `tsconfig.json` with `paths` for `@semio-tech/pets{,-react}` and `include` of its tests | hand |
| 13 | `🔒️dependencies.json` | quiz devDeps (ajv, jstat, mathjs) are classified there; file is generated (`generatedAt 2026-09-27`) and has no quiz `users` entries today | classify new external test libraries as `test-oracle` (generated by the repo dependency command) | generated |
| 14 | Python oracle libs | `pyproject.toml` (dev: numpy, scipy; test: jsonschema, shapely, networkx, …) + `uv.lock` | add only if a new library is needed | hand + `uv lock` |
| 15 | consumers of the product | teaching site depends on `@semio-tech/quiz{,-react}` (`🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/package.json:16-17`) and lists quiz product globs in `📋️project.json` `namedInputs` (`:18-21`); proctor depends on `semio-framework-quiz` | only if the site/proctor consume pets: add `workspace:*` dep and input globs there; the Docker/CI allowlist `🎓️teaching/…/🚀️deploy/Dockerfile.dockerignore` admits Rust sources and only matters if the proctor image builds pets | hand |
| 16 | README indexes | none besides the product's own `README.md`; `🛍️products/AGENTS.md` and root `README.md` do not list products | own README only | hand |
| not needed for a library product | `.claude/launch.json`, `.gitignore`, `.dockerignore` (these hold the teaching *site* dev servers / allowlists) | – | – | – |

### 7.3 Order that keeps each step verifiable (test-first, as `T/README.md:69-95` prescribes)

1. Schema (`🔣️.json`) + TS/Rust twins + Rust round-trip unit test of one hand-written fixture.
2. Oracle registry entry + `🥒️.feature` + Python adapter; `bun ./📜️script.ts contract --owner P` then `oracle` green before any subject code.
3. Subject in one language, then the other; `parity exhaustive --owner P`.
4. Registrations 1-8 above (taxonomy last; verify with `inventoryTaxonomy`-style fast inventory or `bun run verify:taxonomy:implementation:report`, `package.json:240`).
5. React target and its vitest/tsconfig.

## 8. Observed state, drift and traps

* **Concurrent edits in flight**: `git status` shows ~60 modified files under `Q/` (react modules, `README.md`, `🧬️schema`, `✅️validation`, `🃏️sheet`, `👁️views`, `👥️presence/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`, …). The task-icon feature (`Motion`, `TaskIcon`, `TaskIcon` on tasks, `🧪️tests/🖼️task-icons/🟦️.tsx`) is brand-new and is the nearest existing "animation preference" precedent (reduced-motion and a preference switch, `🖼️task-icons/🟦️.tsx:1-40`).
* **Taxonomy drift right now**: fast inventory of scope `Q` reports 1 violation, `directory-kind-unresolved 🧰️framework/🛍️products/❓️quiz/🧪️tests/🖼️task-icons` (the new test dir is not yet in `members-of-tests`). My list comparison also shows `👥️presence` (module) and `🧪️tests/🎚️config` absent from the raw member lists although the inventory does not flag them — the inventory resolves them through other kinds, so always use `inventoryTaxonomy` as the oracle, not the raw list. `🎤️presentation` is unregistered and reports 47 violations (target-inside-package-boundary, package-implementation-file, …); don't copy that layout.
* **Stale generated artifacts**: schema catalog (quiz scope 68/74 exports, hashes differ) and `🔒️dependencies.json` (2026-09-27, no quiz users).
* **Two fixture dialects** in `Q/🧫️fixtures`: Protocol v2 vectors carry `$comment: "Generated by … never edit by hand"` and are produced by the ticket script; React-side vectors carry `schema: "semio.quiz.<name>/v1"` + `description` (e.g. `🌗️contrast-states`) and are hand-authored.
* **Fixture lookup by suffix**: Rust unit tests locate `🧫️fixtures/<dir ending with case>` so adapters/tests refer to cases without the emoji (`fixture("sheet-assembly")`), while Protocol v2 URIs use the full dir name (`shared://🎲️seeded-randomness/🔣️.json`).
* **Vitest include lists are explicit** (core config `:21-32`, react config `:32-50`, react tsconfig `include`); a new suite that is not listed silently does not run (`passWithNoTests: false` only protects against an empty run).
* **Protocol v2 adapters `🟦️.ts` live next to vitest suites in `🧪️tests/`** but are *not* in the vitest include; they are executed only by the harness via `defineTestAdapter`.
* **Shell traps on this host** (from memory notes): Bash heredocs and `node -e` strip backslashes, so write emoji-heavy files with the editor tools; never use `cpSync` recursively on emoji paths; always `cd /c/git/semio` first and `git -c core.quotepath=false` for git.
* **MCP**: the `repo` and `semio` MCP servers were unreachable during this exploration (`CONNECTION_CLOSED`, `CONNECT_TIMEOUT`); ticket bookkeeping for QUIZ-PETS is manual on disk (see `🎫️ticket.json:10`).
* Nothing outside this report file was created, edited or deleted; no git-modifying commands, builds or tests were run (only read-only `bun -e` calls to `inventoryTaxonomy` and `node -e` JSON inspections).
