# WP0 — Framework Modules Schema-Ownership Audit (Merged Slices A–D)

Ticket: `2026/09/08/SCOPE-OWNED-SCHEMA-CONTRACTS` · Work package: WP0 audit, framework-modules partition. This report merges four independent, read-only Sonnet 5 sub-audits — it performs no new auditing itself. Source data: `📊️wp0-framework-modules.json` (this merge's own JSON deliverable, 319 records total, concatenated from the four slices below with a `slice` field added; zero duplicate paths found across slices). Raw per-slice outputs remain at `/private/tmp/claude-501/-Users-ueli-Documents-semio/cd1780e5-e2df-474c-a459-b3835d585ab6/scratchpad/agent{A_ui,B_actor_kernel_value,C_rest_modules,D_products_mitbestand_tests}/findings.{json,md}`.

| Slice | Scope | Files |
|---|---|---|
| A | `🧰️framework/🔨️modules/🖱️ui/` | 102 |
| B | `🧰️framework/🔨️modules/{🎭️actor,🎠️kernel,🌱️value}/` | 62 |
| C | remaining `🧰️framework/🔨️modules/*` (not ui/actor/kernel/value) | 63 |
| D | `🦑️repo`, `📓️print`, `🖥️server` (top-level), `♻️mit-bestand`, root `🧪️tests/`, root files | 92 |
| **Total** | | **319** |

## Totals per classification per slice

| Slice | (a) | (b) | (c) | (d) | (e) | (f) | (g) | (h) | Total | Violations (b+c+d) |
|---|---|---|---|---|---|---|---|---|---|---|
| A | 11 | 36 | 16 | 33 | 1 | 4 | 1 | 0 | 102 | 85 |
| B | 12 | 9 | 19 | 21 | 0 | 1 | 0 | 0 | 62 | 49 |
| C | 16 | 14 | 7 | 7 | 3 | 10 | 6 | 0 | 63 | 28 |
| D | 54 | 10 | 3 | 18 | 2 | 5 | 0 | 0 | 92 | 31 |
| **All** | 93 | 69 | 45 | 79 | 6 | 20 | 7 | 0 | **319** | **193** |

### Classification legend

| Class | Meaning |
|---|---|
| a | authoritative scope schema in a 🧬️schema/ module — correct as-is |
| b | fixture-owned contract (violation) |
| c | non-canonical schema-directory name, e.g. 📐️schema/ or 📐️fixture-schema/ (violation) |
| d | flat-file schema outside a 🧬️schema/ module (violation) |
| e | inert test input |
| f | config/data instance correctly referencing a schema |
| g | third-party / framework's own schema-infra |
| h | generated copy |

**193 of 319 files (60%) across the four framework-modules slices are placement violations (b+c+d).** No cross-slice duplicate paths exist — each of the 319 files was audited by exactly one slice.

## Scope-registration summary

How each tree in this partition declares itself as a schema-owning scope (Cargo `[package.metadata.semio]` id, TS-only nx registration, or no registration at all), taken verbatim from each slice's own audit.

### Slice A — `🖱️ui`

## How `🖱️ui` registers scopes
Every registered scope in this slice is declared via `[package.metadata.semio]` in a Rust crate's `Cargo.toml` (`role = "framework"`, `id = "…"`). Found via `find 🧰️framework/🔨️modules/🖱️ui -iname 'Cargo.toml'` and grepping each for the metadata block:

| Crate path | `id` |
|---|---|
| `📦️packages/🦀️rust/Cargo.toml` (ui root) | `ui` |
| `🎨️styling/📦️packages/🦀️rust/Cargo.toml` | `ui-styling` |
| `🎬️scene/📦️packages/🦀️rust/Cargo.toml` | `ui-scene` |
| `🖌️render/📦️packages/🦀️rust/Cargo.toml` | `ui-render` |
| `🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/Cargo.toml` | `vulkan-backend` |
| `🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/Cargo.toml` | `metal-backend` |
| `🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/Cargo.toml` | `webgpu-backend` |
| `🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/Cargo.toml` | `d3d12-backend` |
| `🖥️host/📦️packages/🦀️rust/Cargo.toml` | `ui-host` |
| `🧠️runtime/📦️packages/🦀️rust/Cargo.toml` | `ui-runtime` |
| `🧬️contract/📦️packages/🦀️rust/Cargo.toml` | `ui-contract` |
TypeScript/other packages register via `📋️project.json` (nx) rather than a `[package.metadata.semio]` block — one exists per Rust-crate scope above plus `📦️packages/🐍️python`, `📦️packages/🟦️typescript`, and `📦️packages/🟦️typescript/🎯️targets/⚛️react`, all riding on the same `id`s (no independent identity).
Taxonomy's `semanticDirectoryKinds` registers several ui-specific *sub-path* kinds (`ui-retained-wire`, `ui-retained-scene`, `ui-retained-typed`, `ui-retained-pack`, `ui-retained-fixtures`, `ui-retained-transport`, `ui-retained-instance`, `ui-retained-surface`, `ui-retained-metadata`, `ui-contract-copy`, `ui-contract-typed`, `ui-contract-document`, `ui-contract-typed-retirement`, `ui-runtime-output`, `ui-runtime-handback`, `ui-render`) — these are **organizational directory kinds**, not independent scope owners: none carries its own `[package.metadata.semio] id`. Their `parentKindIds` chain back up to `retained` / `schema` / `members-of-modules` under the *one* enclosing crate scope (`ui-contract`, `ui-runtime`, etc.).
**Eligible COMPLETE scopes** (own registered `id`, valid `🧬️schema/` owner): `ui`, `ui-styling`, `ui-scene`, `ui-render`, `vulkan-backend`, `metal-backend`, `webgpu-backend`, `d3d12-backend`, `ui-host`, `ui-runtime`, `ui-contract`.
**Organizational sub-paths with no independent scope identity** (own no `🧬️schema/` of their own by right — any schema found under them belongs to their nearest enclosing crate scope above): everything under `🧬️contract/♻️retirement/`, `🧬️contract/🧵️retained/`, `🧬️contract/⚖️compare/`, `🧬️contract/🎟️resident/`, `🧬️contract/📃️document/`, `🧬️contract/📋️list/`, `🧬️contract/🔗️bindings/`, `🧬️contract/🪞️copy/`, `🧬️contract/📚️examples/`, `🖥️host/📥️input/🎟️admission/*`, `🧠️runtime/*` leaf dirs, and `🧱️elements/*` (element-level dirs like `📨️UIDialog` have no Cargo scope at all — they belong to the `ui` root scope).

### Slice B — `🎭️actor` / `🎠️kernel` / `🌱️value`

## How each module registers as a scope

### actor

A single Rust crate covers the entire module: `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml` declares

```toml
[package.metadata.semio]
role = "framework"
id = "actor"
```

with `[lib] path = "🦀️.rs"` rooted at the module's own top-level `🦀️.rs`, and a matching TypeScript package `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📋️project.json` (nx project name not independently captured in this pass, but the package sits under the same `actor` id). **There is no per-subdirectory Cargo.toml anywhere under actor.** This means every leaf directory audited here — `🎠️activation/`, `🏘️composition/`, `📃️page/`, `📤️return/` and its whole `📨️response/🎟️credit/…` subtree, `📥️cold-pair/`, `📮️shard-client/`, `🚪️lifetime/` and `🩹️patch/`, and the entire `🪪️activation/` subtree — is an **organizational sub-path inside the single `actor` scope**, not an independently registered scope in its own right. `mutationDirectoryPattern` (`^.+\uFE0F[a-z][a-z0-9]*(?:-[a-z0-9]+)+$`, i.e. an emoji-prefixed **hyphenated** kebab-case name) does not match most of these leaf names (`🚪️lifetime`, `🎠️activation`, `🏘️admission`, …, which are single words with no hyphen), so they do not independently qualify as taxonomy-recognized mutation-owner directories either — scope eligibility in actor is governed by the one crate-level `id = "actor"` registration, and every schema audited below is therefore actor-owned regardless of how deep it sits.

### kernel

**Kernel has no Cargo.toml anywhere in the tree** — confirmed by `find 🧰️framework/🔨️modules/🎠️kernel -iname 'Cargo.toml'` (zero results) and by a repo-wide grep for `id = "kernel"` in any `*.toml` (zero results). Kernel is a **TypeScript-only module**: its scope is registered via nx at `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/📋️project.json`, whose `"name"` is `"@semio-tech/framework-kernel"`. All kernel schemas in this slice are consumed exclusively by TS (`await import(...)`, dynamic) or by downstream Rust `include_str!`/oracle registrations in OTHER modules (e.g. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`), never by a kernel-owned Rust crate. As with actor, there is no per-subdirectory registration — `📤️return/`, `📦️content/`, `📥️input/` and its whole `🏗️builder/`, `📦️payload/`, `🧾️release/`, `🪪️authority/` subtree, and `📥️poll/🏘️composition/` are all organizational sub-paths inside the single kernel scope, not independent scopes.

### value

Value is **split across multiple owning crates**, unlike actor and kernel:

- `🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/Cargo.toml` — `role = "framework"`, `id = "value-resident"`: its own independently registered scope, covering everything under `value/💾️resident/`.
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` — a proc-macro crate (`semio-framework-value-derive`) with **no** `[package.metadata.semio]` block at all. It is not a registered semio scope and owns no application contracts (confirmed: no schema-shaped candidates fall under `✨️derive/`).
- `value/🗂️ordered/` and `value/🔁️codec/` (plus the top-level `value/🦀️.rs`) have **no `📦️packages/` of their own**. Their Rust sources are pulled in as `mod` children of an entirely different crate: `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml` (`role = "framework"`, `id = "replication"`) references them by relative path (confirmed via `git grep 🗂️ordered|🔁️codec -- '*Cargo.toml'`, which surfaced only the replication crate and one product crate). **This means every schema under `value/🗂️ordered/` and `value/🔁️codec/` in this audit is de-facto owned by the `replication` scope (outside this audit's slice), not by an independent `value` scope**, even though the files live physically inside the `🌱️value/` directory tree. This is flagged explicitly in each affected finding's `owner`/`intendedOwner` fields below.

No `project.json`/nx registration was found for `value/🗂️ordered/` or `value/🔁️codec/` as independent packages either — only `value/🗂️ordered/🔢️numeric/📋️project.json` exists as a nested TS test package, itself not carrying its own `[package.metadata.semio]`-equivalent registration.

### Slice C — remaining `🧰️framework/🔨️modules/*`

## Scope registration — how each module with schema-related files registers itself

- **⏱️trace** — `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/📋️project.json` (nx). No dedicated Cargo.toml scope id found for the trace crate itself in this slice's grep; trace types are consumed as part of the framework surface. `⏱️clock` and its `🏁️tail`/`🧪️contention`/`🧬️contention` leaves are organizational sub-paths, not independent mutation-owner scopes.
- **⏳️async** — `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📋️project.json` + `⏳️async/📦️packages/🟦️typescript`. `🔐️use`, `🔔️deferred-wake`, `🔔️maintenance`, `🤝️cooperative` are feature sub-directories under the async module, each with its own `🧪️fixtures(+s)/` + (ideally) `🧬️schema/` pair — organizational leaves, not separate top-level scopes.
- **🌉️abi** — no own `Cargo.toml`; mounted via `#[path = "../../🔨️modules/🌉️abi/🦀️.rs"]` in `🧰️framework/📦️packages/🦀️rust/🦀️.rs:1994`, which belongs to the top-level `framework` crate (`[package.metadata.semio] role="framework" id="framework"`, at `🧰️framework/📦️packages/🦀️rust/Cargo.toml`). `🌉️abi` is an organizational leaf of the framework-wide scope, not its own registered scope.
- **🎯️action-bus** — same pattern as 🌉️abi: mounted via `#[path = "../../🔨️modules/🎯️action-bus/🦀️.rs"]` at `🧰️framework/📦️packages/🦀️rust/🦀️.rs:1988`; part of the `framework` scope.
- **📡️replication** — `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml`: `[package.metadata.semio] role="framework" id="replication"`. This IS its own registered scope. `🎮️mutation`, `📐️format`, `📡️wire/🏠️local-interaction` (and its `🌳️root`, `🩹️update`, `📡️transport` sub-leaves), `🔗️causal`, and the `🧫️fixtures/*` collections are organizational sub-paths within the `replication` scope, not independent scopes of their own.
- **🕸️graph** — `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml`: `id="graph"`, `role="framework"`. Its own scope. `🛂️manifest` is an organizational build-manifest leaf inside it (not a mutation-owner directory).
- **🕹️interaction** — no own Cargo.toml; mounted at `🧰️framework/📦️packages/🦀️rust/🦀️.rs:2016` (`#[path=".../🔨️modules/🕹️interaction/🦀️.rs"]`) and its schema leaf separately at line 2020; TS re-export at `🧰️framework/📦️packages/🟦️typescript/🟦️.ts:13`. Organizational leaf of the `framework` scope, but with a fully correct exemplary multi-language `🧬️schema/` facet set (4 files: `.graphql`/`.json`/`.ts`/`.rs`).
- **🖼️assets** — no `Cargo.toml` at all (no Rust crate); registers as an nx TypeScript package via `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📋️project.json` (`"name": "@semio-tech/assets"`). Sub-directories `🌱️metabolism/🎨️representation`, `🔍️resolver`, `🔤️fonts`, `🥽️mesh` are organizational leaves inside the assets scope — none register independently.
- **🗺️surface** — `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml`: `role="framework"` (no explicit `id` key found — likely defaults to crate name). Its own scope (wasm-bindgen crate covering paint/board-2d/terrain/node-graph/tiled-map).
- **🚪️io** — no own Cargo.toml at the `🚪️io/` level (only its `🔤️base64` sub-package has one); the `🧬️schema/🦀️.rs` leaf is mounted directly into the `💻️os` product's Rust crate at `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:258`, and re-exported by `semio_framework` per its own doc comment. Organizational leaf, not an independent scope.
- **🛂️manifest** — top-level framework module; no Cargo.toml/project.json scope registration found distinct from the framework crate in this grep pass. `🧪️fixtures/` is an organizational leaf holding 4 fixture-owned schema violations (see below).
- **🧬️schema** — `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml`: `id="schema"`, `role="framework"`. Its own scope — and per the task brief, this module IS the schema-validation framework infrastructure itself (registry, validator engine, facet-descriptor types), not a source of application contracts. All 4 of its non-false-positive candidate files classify as (g).
- **🧵️job** — `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml`: `id="job"`, `role="framework"`. Its own scope. `⏱️budget` and `🧪️fixtures/` are organizational leaves inside it.

## Zero-candidate modules — confirmed via `find <module> -iname '*schema*'`

`◻️2d`, `⚠️diagnostic`, `✍️editor` (only stale Rust build artifacts under `target/` — gitignored, not source), `🎒️pack`, `🏗️mesh-engine`, `📏️intrinsic-size`, `📐️geometry`, `📚️compiler`, `🔀️dispatch`, `🔄️machine`, `🔏️hash`, `🔢️number`, `🔤️typeset`, `🔲️pixels`, `🔺️mesh`, `🖌️raster`, `🖥️platform`, `🗜️deflate`, `🧊️3d`, `🧩️action-argument-resolution`, `🧮️math` all returned no matches from a directory-name sweep and no `"$schema"` hits from a repo-scoped `git grep -l '"$schema"'` across all 34 module directories (see below) — genuinely no schema-related files in this slice.

### Slice D — `🦑️repo` / `📓️print` / `🖥️server` / `♻️mit-bestand` / root tests

## Scope registration per tree

**`🦑️repo` (repo product).** Mixed registration mechanism, by sub-tree:
- Rust packages register via `[package.metadata.semio]` in `Cargo.toml`. Found two live examples: `🔨️modules/⌨️cli/📦️packages/🦀️rust/Cargo.toml` declares `role = "tool"`; the `🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml` package carries no `[package.metadata.semio]` block at all (the `🧪️test` module's Rust package is currently unregistered by that mechanism).
- TypeScript/nx packages register purely through the nx project `"name"` field in `📋️project.json` (e.g. `@semio-tech/repo-lib` for `📚️library/📦️packages/🟦️typescript/`) — there is no observed `"tags"`-based role/id metadata block analogous to the Rust `[package.metadata.semio]` convention; the nx project name itself is the registration.
- Go packages (`🔌️mcp`, `🎛️coordinator`, `💻️client/🪶️sqlite`) register via `go.mod` + nx `📋️project.json`, no `[package.metadata.semio]`-equivalent found.
- **Eligible complete scopes** (own a `🧬️schema/` or should): `📚️library` (module root — owns the repo-wide mutation-descriptor meta-schema), `📚️library/⚡️caching`, `📚️library/🔍️discovery`, `📚️library/🧹️normalization`, `🖥️server` (nested, module root — owns `🧬️schema/🐘️postgres/`), `🧪️test` (module root — taxonomy-declared canonical `testSchemaLocation`), and every individual test-case directory under `📚️library/🧪️tests/*` (this repo's own test harness dogfoods the `🧬️schema/` convention at per-test-case granularity — see Coverage). **Organizational sub-paths** (not independent scopes): `🔨️modules/*/📦️packages/<lang>/🧫️fixtures/*` (fixture trees — never a scope owner, see the (b) findings below), `🎛️coordinator/🧫️fixtures/` likewise.

**`📓️print` (print product).** Registers only via nx `📋️project.json` (`"name": "@semio-tech/print"`); no Rust package, no `[package.metadata.semio]` anywhere in the tree. Eligible complete scopes: each `🔨️modules/<name>/` directory (`🔤print-font-catalog`, `🖨️tectonic-template-compilation/📇️catalog`, `.../📚️bundle`, `.../🔧️toolchain`) is treated by its own test harness (`🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts`) as an independent module scope, each expected to own one schema — but none of them currently do so in the canonical `🧬️schema/` shape (all four are flat-file violations, see below).

**`🖥️server` (top-level product, distinct from `🦑️repo/🔨️modules/🖥️server/`).** Registers via Rust: `📦️packages/🦀️rust/Cargo.toml` carries `[package.metadata.semio]` with `role = "product"`, `id = "server"` — a clean, canonical example of the Rust registration convention. No schema-ownership findings originate in this top-level product itself (the pre-pass and this audit's sanity sweep found zero `$schema`/`*schema*` files there beyond boilerplate `package.json`/`project.json` editor-tooling references) — all `🖥️server`-named findings in this report belong to the *nested* `🦑️repo/🔨️modules/🖥️server/` module instead, which is a different scope entirely (confirmed distinct per the task brief).

**`♻️mit-bestand`.** **No formal scope-registration mechanism at all.** Taxonomy's `semanticDirectoryKinds` recognizes `"mit-bestand"` (emoji ♻️, `slugPattern: "^mit-bestand$"`) purely as a directory-naming/path-parsing classifier — it carries no `role`/`id`/ownership semantics, unlike a Rust `[package.metadata.semio]` block. The tree's constituent packages (`📋️bericht`, `🧺️demonstrator`) register as ordinary nx projects via `package.json`/`📋️project.json` (`"name": "@semio-tech/mit-bestand-demonstrator"`, etc.) — normal nx build/test wiring, not scope/mutation-authority registration. No `Cargo.toml` exists anywhere under `♻️mit-bestand/`. Its `🔨️modules/*` sub-directories (`📄️documents`, `🧩️runtime`) follow the same file-layout *convention* as registered scopes elsewhere (flat data + flat schema + script) without ever being declared a scope in the taxonomy sense — this is itself worth flagging: mit-bestand's module contracts are validated by ad hoc, per-package TypeScript loaders (see the two `♻️mit-bestand` findings below) rather than anything the taxonomy/discovery system recognizes as authoritative.

**Root `🧪️tests/`.** Not a registered scope by itself; contains one test-case directory (`🧪️transaction-process-ownership/`) following the exact same per-test `🧬️schema/🔣️.json` convention used throughout `📚️library/🧪️tests/*` — legitimate, no violation.

## Duplicate-authority pairs

Two recurring duplicate-authority patterns surfaced across the partition, both matching the parent ticket's seed observations (`📋️master-plan.md`: a fixture-wrapper or fixture-shape schema squatting next to — or instead of — the real application contract).

### ui `📐️schema` vs flat `🧬️schema.json` families (Slice A)

The dominant violation pattern in slice A: one wrong-named `📐️schema/🔣️.json` (large, detailed, unconsumed by any literal reference found) sits beside one flat `🧬️schema.json` (small, also unconsumed for the resident-tree cases) for the **same owner**. Their JSON keys do not overlap — two genuinely different facets of one owner, not a stale copy, so the fix is consolidation into one `🧬️schema/` directory holding two named files rather than a delete-one-keep-one call. Three families:

- `🧬️contract/🧵️retained/💾️resident/*` — 10 owners × 2 files = 20 files (`🏗️builder`, `📃️page`, `📃️page/🔗️binding`, `📖️reader`, `📦️payload`, `📨️slot`, `🧾️evidence`, `🧾️evidence/📋️copied`, `🧾️evidence/🚫️cancellation`, `🪪️metadata`).
- `🧬️contract/♻️retirement/🌳️typed/*` — 2 files (`📐️schema/🔣️.json` vs `🧬️components.schema.json`).
- `🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/*` — 2 files.

Zero `📐️schema` directory-kind registrations exist anywhere in taxonomy.json (confirmed via a full JSON scan) — every `📐️schema/` directory found in slice A (16 of them) is confirmed non-canonical. See "Slice details" → Slice A below for every individual pair.

### transport `📐️schema` vs `🧬️schema` (Slice C)

### 4. `📡️replication/📡️wire/🏠️local-interaction/📡️transport/{📐️schema,🧬️schema}/🔣️.json` — duplicate-authority pair

**This is the clearest duplicate-authority instance in the slice.** `📡️transport/` has TWO schema-named
sibling directories:
- `🧬️schema/🔣️.json` — the REAL wire-protocol contract (`$id: "semio:local-interaction-query-transport"`,
  `oneOf` command/reply, `$ref`s into the canonical local-interaction schema). Classification (a), keep.
- `📐️schema/🔣️.json` — a DIFFERENT, narrower schema that validates the shape of the transport
  **test-fixture corpus** (`🧫️fixtures/🔣️.json`: `version`/`appCommandTag`/`unsigned`/`commandKinds`/...).
  Classification (c).

Both are actively consumed by the master validator `🧫️fixtures/📜️script.ts:180-181`
(`../📡️transport/🧫️fixtures/🔣️.json` validated against `../📡️transport/📐️schema/🔣️.json`, then the
protocol itself compiled from `../📡️transport/🧬️schema/🔣️.json`). They are not literally duplicate
content — but the directory-naming collision (two "…schema…" dirs, same parent) is exactly the
pattern flagged in the task brief.
- **Recommendation:** move the fixture-shape schema into `📡️transport/🧫️fixtures/🧬️.schema.json`
  (the same pattern already used for `♻️retirement`/`📃️query`/`🔐️topology-authority`/`🏠️local-interaction`

## High-fanout schemas

From slice B: two schemas are unusually high-fanout — cross-module `$ref`/`import` consumers outside their own owning module — and should be prioritized/sequenced carefully in any rename, preserving the exact `$id` string while only moving the file:

- `🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json` (`semio.value.resident.capacity.v1`) — the single most widely `$ref`'d schema found in the whole slice; imported from kernel, actor, and ui.
- `🧰️framework/🔨️modules/🎭️actor/📃️page/🧬️schema.json` (`semio.actor.byte-page.v1`) and `🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema.json` (`semio.actor.retained-return.v1`) — imported from kernel and `os/plugin` as well as within actor itself.

## Cross-module consumers (`🖼️assets` → `🖱️ui`)

From slice C: the entire `🖼️assets` module (`🌱️metabolism/🎨️representation`, `🔍️resolver`, `🔤️fonts`, `🥽️mesh`) has no Cargo.toml and registers only as an nx TypeScript package (`@semio-tech/assets`). It has **no `🧬️schema/` directory anywhere** — every schema in it is a flat `🧬️catalog.schema.json`/`🧬️delivery.schema.json` file (classification d). Two of the four moves (`🔤️fonts/🧬️catalog.schema.json`, `🥽️mesh/🧬️catalog.schema.json`) require coordinated updates in `🖱️ui/🎨️styling` and `🖱️ui/🧬️contract` — outside slice C's ownership, audited separately in slice A — because `🖱️ui` consumes these assets catalogs across the module boundary. Slice A independently confirms the mirror image of this finding: its inert fixture `🎨️styling/🧪️tests/🧊️mesh-collection.json` (classification e) has a `$schema` pointer naming a real schema owned by the sibling `🖼️assets` module, outside the ui slice — flagged by both slices as the same cross-module boundary, not resolved by either audit alone.

## `📚️library/🧬️.schema.json` mutation-descriptor meta-schema finding

From slice D — the single most consequential flat-file finding across the whole framework-modules partition: the repo-wide **mutation-descriptor meta-schema** (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️.schema.json`) lives as a hidden, anonymous dot-file directly in the `📚️library` module root, rather than in a `🧬️schema/` directory — even though it is the schema that literally standardizes the naming convention it itself violates. It defines the canonical, repo-wide "Direct Mutation Descriptor" contract (`$id https://semio.tech/schema/mutation-descriptor/1`) that every mutation owner's own companion `🧬️.schema.json` descriptor file must satisfy. Consumers: `📚️library/📦️packages/🟦️typescript/🔬️index.test.ts:7394` (hard-coded absolute repo-relative path) and `:52` (generic loop over every owner directory's own descriptor); `🔣️taxonomy.json:3358,3382` (`mutation-descriptor-specimen`/`mutation-descriptor-agreement` entries). The file has already moved once before without ever landing in the canonical `🧬️schema/` shape (earlier location per historical ticket notes: `🦑️repo/📚️library/🔣️mutation-descriptor.schema.json`). A move is repo-wide, high-blast-radius, and should be coordinated with whoever owns the mutation/DSL derive machinery, not executed unilaterally from within this partition alone.

## Full findings table (319 files, all four slices)

| Slice | Path | Class | Owner (today) | Intended owner | Decision |
|---|---|---|---|---|---|
| A | `🧰️framework/🔨️modules/🖱️ui/components.json` | g | n/a (tooling config, no semio scope) | n/a — out of semio schema-ownership scope entirely | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌐️favicon.json` | f | ui-styling | n/a — this is the data instance; keep as-is once its schema (see 🧬️favicon.schema.json) is relocated, updating the $schema pointer accordingly | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts` | f | ui-styling (Cargo id "ui-styling") | n/a — this is source code, not a data instance or a contract; no relocation needed | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧊️mesh-collection.json` | e | ui-styling (test fixture) | n/a — inert test input, not a contract | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧬️favicon.schema.json` | d | ui-styling | ui-styling::styling::favicon::Schema — should live at 🎨️styling/🧬️schema/🌐️favicon.json | move-to 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧬️schema/🌐️favicon.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🔮️oracle/🔣️.json` | f | repo test module (🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test) — not ui | n/a — this is a data instance owned by ui; its schema is owned elsewhere | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧫️fixtures/🔣️.json` | f | metal-backend | n/a — data instance, correctly co-located with (not competing with) its schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧬️schema/🔣️.schema.json` | a | metal-backend | metal-backend::objective_c::AbiFixture::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧬️schema/🔣️.json` | a | webgpu-backend | webgpu-backend::surface_adapter::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/✍️writer/🧬️schema/🔣️.json` | a | ui-host | ui-host::input::admission::writer::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧬️schema/🔣️.json` | a | ui-host | ui-host::input::admission::commit::enqueue::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/🧬️schema/🔣️.json` | a | ui-host | ui-host::input::admission::commit::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧬️schema/🔣️.json` | a | ui-host | ui-host::input::admission::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🧬️schema/🔣️.json` | a | ui-host | ui-host::input::admission::root::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧬️schema/🔣️.json` | a | ui-host | ui-host::event::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧬️schema.json` | d | ui-runtime | ui-runtime::retirement::tree::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧬️schema.json` | d | ui-runtime | ui-runtime::document::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧬️schema.json` | d | ui-runtime | ui-runtime::ownership::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧬️schema.json` | d | ui-runtime | ui-runtime::output::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🧬️schema.json` | d | ui-runtime | ui-runtime::transaction::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🚪️handback/🧬️schema.json` | d | ui-runtime | ui-runtime::handback::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🚪️handback/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🩹️patch/🧬️schema.json` | d | ui-runtime | ui-runtime::patch::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🩹️patch/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧬️schema.json` | d | ui-contract | ui-contract::retirement::built::Schema — should live at ♻️retirement/🌲️built/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retirement::typed::Schema — should live at ♻️retirement/🌳️typed/🧬️schema/ (consolidated with its sibling below) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️components.schema.json` | d | ui-contract | ui-contract::retirement::typed::ComponentsSchema — should live at ♻️retirement/🌳️typed/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️schema/🧬️components.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📋️list/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retirement::list::Schema — should live at ♻️retirement/📋️list/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📋️list/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retirement::Schema — should live at ♻️retirement/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📮️handback/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retirement::handback::Schema — should live at ♻️retirement/📮️handback/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📮️handback/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retirement::patch::Schema — should live at ♻️retirement/🩹️patch/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema.json` | d | ui-contract | ui-contract::retirement::patch::pending::whole::Schema — should live at 📦️whole/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧬️schema.json` | d | ui-contract | ui-contract::compare::document::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/🧬️schema.json` | d | ui-contract | ui-contract::compare::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🌳️root/🧬️schema.json` | d | ui-contract | ui-contract::resident::root::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🌳️root/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🗃️fixed/🧬️schema.json` | d | ui-contract | ui-contract::resident::fixed::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🗃️fixed/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧬️schema.json` | d | ui-contract | ui-contract::resident::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🧬️schema.json` | d | ui-contract | ui-contract::document::assembly::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧬️schema.json` | d | ui-contract | ui-contract::list::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧬️catalog.schema.json` | b | ui-contract (test corpus infrastructure) | ui-contract::examples::conformance::CatalogSchema — should live at 🧬️contract/🧬️schema/🔣️conformance-catalog.json, with 📚️examples/🧪️conformance/ kept as pure corpus data | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs (or a new 🧬️schema/🔣️conformance-catalog.json) |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🧬️schema.json` | d | ui-contract | ui-contract::bindings::copy::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️fixtures/🔣️.schema.json` | b | ui-contract | ui-contract::<node-ownership-ttl owner>::Schema — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs` | a | ui-contract | ui-contract::schema::TYPES (SchemaMetadata table) / ui-contract::schema::SchemaMetadata | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🔣️.schema.json` | d | ui-contract | ui-contract::retained::scene::typed::Schema — should live at 🎬️scene/🧾️typed/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::builder::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::builder::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::page::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::page::binding::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::page::binding::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::page::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::reader::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::reader::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::payload::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::payload::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::slot::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::slot::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧪️tests/🧬️schema/🔣️.json` | a | ui-contract | ui-contract::retained::resident::scalar::tests::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json` | a | ui-contract | ui-contract::retained::resident::scalar::Schema | keep |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::Schema — should live at 💾️resident/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::evidence::copied::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::evidence::copied::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::evidence::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::evidence::cancellation::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::evidence::cancellation::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::evidence::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::resident::resident::metadata::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json` | d | ui-contract | ui-contract::retained::resident::resident::metadata::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🏷️fields/🧬️.schema.json` | b | ui-contract | ui-contract::retained::wire::<owner for '🏷️fields'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/📤️decode/🧬️.schema.json` | b | ui-contract | ui-contract::retained::wire::<owner for '📤️decode'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🗺️surface-bytes/🧬️.schema.json` | b | ui-contract | ui-contract::retained::wire::<owner for '🗺️surface-bytes'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🧾️typed/🧬️.schema.json` | b | ui-contract | ui-contract::retained::wire::<owner for '🧾️typed'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/⚙️owned-operations/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '⚙️owned-operations'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌱️root-source/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🌱️root-source'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌳️owned-nodes/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🌳️owned-nodes'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎟️read-lease/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🎟️read-lease'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎬️owned-scene/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🎬️owned-scene'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/👶️native-child/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '👶️native-child'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/💾️resident/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '💾️resident'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📃️scene-json-document/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '📃️scene-json-document'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📤️read-publication/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '📤️read-publication'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📥️intake/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '📥️intake'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📦️scene-generic-pack/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '📦️scene-generic-pack'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📨️wire-operations/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '📨️wire-operations'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔏️owned-hash/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🔏️owned-hash'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔔️intake-notification/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🔔️intake-notification'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔗️scene-binding/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🔗️scene-binding'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔢️scene-numeric/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🔢️scene-numeric'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🔣️scene-json'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔤️scene-text-bytes/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🔤️scene-text-bytes'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🗺️owned-surface/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🗺️owned-surface'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚨️intake-close-fault/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🚨️intake-close-fault'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚪️instance-close/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🚪️instance-close'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛠️instance-maintenance/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🛠️instance-maintenance'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛡️owned-validation/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🛡️owned-validation'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧳️scene-pack-field/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🧳️scene-pack-field'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧵️scene-json-string/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🧵️scene-json-string'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧾️typed-scene/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🧾️typed-scene'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🩹️patch/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🩹️patch'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪆️surface-child/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🪆️surface-child'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪪️instance-owner/🧬️.schema.json` | b | ui-contract | ui-contract::retained::<owner for '🪪️instance-owner'> — real owner not identified by this audit; needs producer/consumer trace | fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified> |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/📐️schema/🔣️.json` | c | ui-contract | ui-contract::retained::operations::wire::pages::OracleSchema — should live at 📃️pages/🧬️schema/ (consolidated with its 🧬️schema.json sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema/🔣️.oracle.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema.json` | d | ui-contract | ui-contract::retained::operations::wire::pages::AdmissionSchema — should live at 📃️pages/🧬️schema/ (consolidated with its 📐️schema/ sibling) | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema/🔣️.admission.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪞️copy/🧬️schema.json` | d | ui-contract | ui-contract::copy::Schema — should live at <owner-dir>/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪞️copy/🧬️schema/🔣️.json |
| A | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧬️contracts/♿️modal/🧬️.schema.json` | b | ui-elements (UIDialog) | ui::elements::UIDialog::AccessibilityFixtureSchema — should live at 📨️UIDialog/🧬️schema/♿️modal.json (or a shared a11y-fixture schema module if this pattern repeats across other elements) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧬️schema/♿️modal.json |
| B | `🧰️framework/🔨️modules/🌱️value/💾️resident/📐️schema/🔣️.json` | c | value/resident (crate id=value-resident) | value/resident (crate id=value-resident) | move-to 🧰️framework/🔨️modules/🌱️value/💾️resident/🧫️fixture/🧬️schema/🔣️.json (rename non-canonical 📐️schema/ dir; cannot simply rename to 🧬️schema/ in place because the sibling flat 🧬️schema.json already claims that name for the domain capacity contract — see companion finding for that file, classification d) |
| B | `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/📐️schema/🔣️.json` | c | value/resident/admission (sub-path of crate value-resident) | value/resident/admission (sub-path of crate value-resident) | move-to 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the domain admission contract) |
| B | `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema.json` | d | value/resident/admission | value/resident/admission | move-to 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir exists at this level so this is a straight conversion) |
| B | `🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json` | d | value/resident | value/resident | move-to 🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json (flat file -> directory module). HIGH-FANOUT file: this is the single most widely $ref'd schema found in the whole slice (kernel, actor, and ui all depend on its $id string), so the rename must preserve the exact $id value untouched and only change file location. |
| B | `🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️fixtures/🧬️.schema.json` | b | value/🔁️codec (no independent Cargo crate; 🔁️codec/🦀️.rs is a mod compiled into the semio-framework-replication crate, id=replication) | value/🔁️codec (should still get its own 🧬️schema/ module even though the owning crate is 'replication') | move-to 🧰️framework/🔨️modules/🌱️value/🔁️codec/🧬️schema/🔣️.json (there is no 🧬️schema/ anywhere under 🔁️codec; this anonymous .schema.json sitting directly inside 🧪️fixtures/ is the ONLY schema defining this contract, so it must be promoted out of the fixtures directory into a proper canonical module rather than merely renamed in place) |
| B | `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧪️fixtures/🛡️references.schema.json` | b | value/🗂️ordered/🔢️numeric (no independent crate; compiled into semio-framework-replication, id=replication) | value/🗂️ordered/🔢️numeric | move-to 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧬️schema/🔗️references.json (no 🧬️schema/ directory exists anywhere under 🔢️numeric; two distinct named fixtures each need their own schema entry inside one shared canonical module) |
| B | `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧪️fixtures/🧬️numeric-index.schema.json` | b | value/🗂️ordered/🔢️numeric (compiled into semio-framework-replication, id=replication) | value/🗂️ordered/🔢️numeric | move-to 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧬️schema/🔢️numeric-index.json (same target module as the sibling references schema; both anonymous fixture schemas consolidate under one canonical 🧬️schema/ directory as named files) |
| B | `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/👥️shared-owner/🧬️.schema.json` | b | value/🗂️ordered (compiled into semio-framework-replication, id=replication) | value/🗂️ordered | move-to 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧬️schema/👥️shared-owner.json (no 🧬️schema/ directory exists anywhere under 🗂️ordered top level; consolidate with the sibling ordered-map schema finding below into one canonical module) |
| B | `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/🧬️.schema.json` | b | value/🗂️ordered (compiled into semio-framework-replication, id=replication) | value/🗂️ordered | move-to 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧬️schema/🔣️.json (this is the primary/anonymous schema for the ordered-map domain concept itself, not a secondary named one; it becomes the canonical anonymous file in the new module, with 👥️shared-owner and the numeric ones as named siblings if consolidated at a shared level, or its own dedicated module if kept scoped to 🗂️ordered only) |
| B | `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧺️set/🧬️schema/🔣️.schema.json` | a | value/🗂️ordered/🧺️set (compiled into semio-framework-replication, id=replication) | value/🗂️ordered/🧺️set (unchanged — already correctly placed) | keep — already an authoritative schema correctly placed inside a canonically-named 🧬️schema/ directory module beside its own 🧫️fixtures/🔣️.json example |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/🏠️source/📚️entries/🧬️schema/🔣️.json` | a | kernel (TS-only module, nx project @semio-tech/framework-kernel) | kernel/📤️return/🏠️source/📚️entries (unchanged) | keep — sits in a canonically-named 🧬️schema/ directory beside its own 🧫️fixture/🔣️.json; no competing schema exists at this leaf (id containing "fixture" is just a naming convention, not evidence of a duplicate authority) |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/🏠️source/🧬️schema/🔣️.json` | a | kernel | kernel/📤️return/🏠️source (unchanged) | keep — canonically-named 🧬️schema/ directory beside its own 🧫️fixture/🔣️.json; no competing schema at this leaf |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/💌️message/🧬️schema/🔣️.json` | a | kernel | kernel/📤️return/📦️content/💌️message (unchanged) | keep — canonically-named 🧬️schema/ directory, no competing schema at this leaf |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📐️fixture-schema/🔣️.json` | c | kernel/📤️return/📦️content | kernel/📤️return/📦️content | move-to 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ directory already claims 🧬️schema for the domain content-declaration contract, see companion finding classification a) |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/📐️fixture-schema/🔣️.json` | c | kernel/📤️return/📦️content/📥️input/🏗️builder | kernel/📤️return/📦️content/📥️input/🏗️builder | move-to 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ directory already claims 🧬️schema for the real domain builder-binding contract, kernel-return-builder-binding.v1) |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧬️schema/🔣️.json` | a | kernel/📤️return/📦️content/📥️input/🏗️builder | kernel/📤️return/📦️content/📥️input/🏗️builder (unchanged) | keep — correctly placed in a canonically-named 🧬️schema/ directory |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/📐️fixture-schema/🔣️.json` | c | kernel/📤️return/📦️content/📥️input/📦️payload | kernel/📤️return/📦️content/📥️input/📦️payload | move-to 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ already claims 🧬️schema for the real domain resident-payload-association contract) |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️schema/🔣️.json` | a | kernel/📤️return/📦️content/📥️input/📦️payload | kernel/📤️return/📦️content/📥️input/📦️payload (unchanged) | keep — correctly placed in a canonically-named 🧬️schema/ directory |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧬️schema/🔣️.json` | a | kernel | kernel/📤️return/📦️content/📥️input (unchanged) | keep — canonically-named 🧬️schema/ directory, no competing schema at this leaf (id containing "fixture" is naming convention only) |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/📐️fixture-schema/🔣️.json` | c | kernel/📤️return/📦️content/📥️input/🧾️release | kernel/📤️return/📦️content/📥️input/🧾️release | move-to 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ already claims 🧬️schema for the real release contract) |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧬️schema/🔣️.json` | a | kernel/📤️return/📦️content/📥️input/🧾️release | kernel/📤️return/📦️content/📥️input/🧾️release (unchanged) | keep — correctly placed in a canonically-named 🧬️schema/ directory |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧬️schema/🔣️.json` | a | kernel | kernel/📤️return/📦️content/📥️input/🪪️authority (unchanged) | keep — canonically-named 🧬️schema/ directory, no competing schema at this leaf (id containing "fixture" is naming convention only, confirmed by 3 real cross-module consumers) |
| B | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧬️schema/🔣️.json` | a | kernel/📤️return/📦️content | kernel/📤️return/📦️content (unchanged) | keep — correctly placed in a canonically-named 🧬️schema/ directory |
| B | `🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/📐️fixture-schema/🔣️.json` | c | kernel/📥️poll/🏘️composition | kernel/📥️poll/🏘️composition | move-to 🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ already claims 🧬️schema for the real poll-composition contract) |
| B | `🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧬️schema/🔣️.json` | a | kernel/📥️poll/🏘️composition | kernel/📥️poll/🏘️composition (unchanged) | keep — correctly placed in a canonically-named 🧬️schema/ directory |
| B | `🧰️framework/🔨️modules/🎠️kernel/🔮️oracle/🔣️.json` | f | kernel | kernel (unchanged — this is repo-test-platform registry data owned by kernel, not an application contract) | keep — this is a config/registry data instance (oracle catalog), not an application contract; its $schema key is a tooling reference to an external, out-of-slice schema module (🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/), which is itself the correct authoritative owner for THAT schema (outside this audit's slice) |
| B | `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📇️descriptor-load/🧬️.schema.json` | b | kernel (top-level 🧫️fixtures/ grab-bag, not a per-contract mutation-owner directory) | kernel/🧫️fixtures/📇️descriptor-load | move-to 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📇️descriptor-load/🧬️schema/🔣️.json (rename the anonymous 🧬️.schema.json into a proper canonically-named directory beside its sibling 🔣️.json fixture data; no independent domain-owner directory exists elsewhere for "descriptor-load") |
| B | `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📐️source-watch.schema.json` | b | kernel (top-level 🧫️fixtures/ grab-bag) | kernel/🧫️fixtures (or a new kernel/🧫️fixtures/📡️source-watch/ subdirectory) | move-to 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📡️source-watch/🧬️schema/🔣️.json (this is the most severe instance in kernel's fixtures grab-bag: not only non-canonically named but not even grouped in its own subdirectory the way descriptor-load/turn-patch-owner/app-router-plugin-faults are — recommend giving it a matching 📡️source-watch/ subdirectory holding both the 🔣️.json fixture and a 🧬️schema/ module) |
| B | `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🚪️turn-patch-owner/🧬️.schema.json` | b | kernel (top-level 🧫️fixtures/ grab-bag) | kernel/🧫️fixtures/🚪️turn-patch-owner | move-to 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🚪️turn-patch-owner/🧬️schema/🔣️.json |
| B | `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🧬️.schema.json` | b | kernel (top-level 🧫️fixtures/ grab-bag) | kernel/🧫️fixtures/🧫️app-router-plugin-faults | move-to 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🧬️schema/🔣️.json |
| B | `🧰️framework/🔨️modules/🎭️actor/🎠️activation/🧬️schema.json` | d | actor (single crate, role=framework id=actor) | actor/🎠️activation | move-to 🧰️framework/🔨️modules/🎭️actor/🎠️activation/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level, straight conversion) |
| B | `🧰️framework/🔨️modules/🎭️actor/🏘️composition/🏗️bootstrap/🧬️schema.json` | d | actor | actor/🏘️composition/🏗️bootstrap | move-to 🧰️framework/🔨️modules/🎭️actor/🏘️composition/🏗️bootstrap/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level) |
| B | `🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json` | d | actor | actor/🏘️composition | move-to 🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level) |
| B | `🧰️framework/🔨️modules/🎭️actor/📃️page/📐️schema/🔣️.json` | c | actor/📃️page | actor/📃️page | move-to 🧰️framework/🔨️modules/🎭️actor/📃️page/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real byte-page domain contract) |
| B | `🧰️framework/🔨️modules/🎭️actor/📃️page/🧬️schema.json` | d | actor | actor/📃️page | move-to 🧰️framework/🔨️modules/🎭️actor/📃️page/🧬️schema/🔣️.json (flat file -> directory module). HIGH-FANOUT file, second only to value/resident's capacity schema in cross-module reach — preserve the exact $id string. |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/🌿️framing/🧬️schema.json` | d | actor | actor/📤️return/🌿️framing | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/🌿️framing/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level) |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/📐️schema/🔣️.json` | c | actor/📤️return | actor/📤️return | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real retained-return domain contract) |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🌿️framing/🧬️schema.json` | d | actor | actor/📤️return/📨️response/🌿️framing | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🌿️framing/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level) |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧬️schema.json` | d | actor | actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level) |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/🧬️schema.json` | d | actor | actor/📤️return/📨️response/🎟️credit/📋️metadata | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level) |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📐️schema/🔣️.json` | c | actor/📤️return/📨️response/🎟️credit | actor/📤️return/📨️response/🎟️credit | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real credit domain contract) |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧬️schema.json` | d | actor | actor/📤️return/📨️response/🎟️credit | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level besides the non-canonical 📐️schema/, handled in its own finding) |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/📐️schema/🔣️.json` | c | actor/📤️return/📨️response | actor/📤️return/📨️response | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real return-response domain contract) |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧬️schema.json` | d | actor | actor/📤️return/📨️response | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧬️schema/🔣️.json (flat file -> directory module) |
| B | `🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema.json` | d | actor | actor/📤️return | move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema/🔣️.json (flat file -> directory module). HIGH-FANOUT file consumed by kernel and os/plugin cross-module — preserve the exact $id string. |
| B | `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧬️schema/🔣️.json` | a | actor | actor/📥️cold-pair (unchanged — already correctly placed) | keep — already an authoritative schema correctly placed inside a canonically-named 🧬️schema/ directory module; the ONLY canonical-directory instance found among actor's flat-schema.json cluster (all its siblings use the flat file anti-pattern, see companion findings) |
| B | `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧬️schema.json` | d | actor | actor/📮️shard-client | move-to 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level) |
| B | `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/📐️schema/🔣️.json` | c | actor/🚪️lifetime | actor/🚪️lifetime | move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real instance-lifetime-close domain contract) |
| B | `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧬️schema.json` | d | actor | actor/🚪️lifetime | move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧬️schema/🔣️.json (flat file -> directory module). HIGH-FANOUT file — preserve the exact $id string. |
| B | `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧯️fault.schema.json` | d | actor/🚪️lifetime | actor/🚪️lifetime | move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧫️fixture/🧬️schema/🔣️.json (this is a named flat file, not the anonymous domain contract; it validates the sibling 🚨️fault.fixture.json data file, so it belongs beside that fixture inside a canonically-named 🧬️schema/ module rather than as a bare named file at the lifetime root) |
| B | `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/📐️schema/🔣️.json` | c | actor/🚪️lifetime/🩹️patch | actor/🚪️lifetime/🩹️patch | move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real ui-patch-receipt domain contract) |
| B | `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧬️schema.json` | d | actor | actor/🚪️lifetime/🩹️patch | move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧬️schema/🔣️.json (flat file -> directory module) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📐️schema/🔣️.json` | c | actor/🪪️activation | actor/🪪️activation | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🧬️schema/🔣️.json (orphan case: no competing flat/dir 🧬️schema exists anywhere at this exact directory level — only 📐️schema/ and 🧪️fixture/ — so this is a straight rename, not a nesting relocation) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/📐️schema/🔣️.json` | c | actor/🪪️activation/📤️return/🏘️admission | actor/🪪️activation/📤️return/🏘️admission | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real captured-return-admission domain contract) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️schema.json` | d | actor | actor/🪪️activation/📤️return/🏘️admission | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️schema/🔣️.json (flat file -> directory module) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/📐️schema/🔣️.json` | c | actor/🪪️activation/📤️return | actor/🪪️activation/📤️return | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🧬️schema/🔣️.json (orphan case: only 🏘️admission/ subdir, 📐️schema/, and 🧪️fixture/ exist at this level — no competing 🧬️schema, straight rename) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📨️inbound/📐️schema/🔣️.json` | c | actor/🪪️activation/📨️inbound | actor/🪪️activation/📨️inbound | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📨️inbound/🧬️schema/🔣️.json (orphan case: only 📐️schema/ and 🧪️fixture/ exist at this level — no competing 🧬️schema, straight rename) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📐️schema/🔣️.json` | c | actor/🪪️activation/🚪️instance | actor/🪪️activation/🚪️instance | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/🧬️schema/🔣️.json (orphan case: only 📐️schema/, 📥️output/, and 🧪️fixture/ exist at this level — no competing 🧬️schema, straight rename) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/📐️schema/🔣️.json` | c | actor/🪪️activation/🚪️instance/📥️output/🏘️admission | actor/🪪️activation/🚪️instance/📥️output/🏘️admission | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real domain admission contract) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️schema.json` | d | actor | actor/🪪️activation/🚪️instance/📥️output/🏘️admission | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️schema/🔣️.json (flat file -> directory module) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧬️schema.json` | d | actor | actor/🪪️activation/🚪️instance/📥️output | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level) |
| B | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧯️fault/🧬️schema.json` | d | actor | actor/🪪️activation/🚪️instance/📥️output/🧯️fault | move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧯️fault/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level) |
| C | `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🏁️tail/🧬️schema/🔣️.json` | a | ⏱️trace (framework scope, folded into top-level framework crate) | ⏱️trace/⏱️clock/🏁️tail (watchdog tail fixture schema) | keep |
| C | `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧬️contention/🔣️.schema.json` | c | ⏱️trace (framework scope) | ⏱️trace/⏱️clock — rename 🧬️contention/ to 🧬️schema/ | move-to 🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧬️schema/🔣️.json (rename dir 🧬️contention→🧬️schema; update the one root script.ts reference) |
| C | `🧰️framework/🔨️modules/⏳️async/🔐️use/🧪️fixtures/🧬️.schema.json` | b | ⏳️async (framework scope) | ⏳️async/🔐️use/🧬️schema/🔣️.json (sibling of 🧪️fixtures/, not inside it) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/⏳️async/🔐️use/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/⏳️async/🔔️deferred-wake/🧪️fixtures/🧬️.schema.json` | b | ⏳️async (framework scope) | ⏳️async/🔔️deferred-wake/🧬️schema/🔣️.json (sibling of 🧪️fixtures/, not inside it) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/⏳️async/🔔️deferred-wake/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🧪️fixtures/🧬️.schema.json` | b | ⏳️async (framework scope) | ⏳️async/🔔️maintenance/🧬️schema/🔣️.json (sibling of 🧪️fixtures/, not inside it) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/⏳️async/🤝️cooperative/🧬️schema/🔣️.json` | a | ⏳️async (framework scope) | ⏳️async/🤝️cooperative (cooperative maintenance lane-budget schema) | keep |
| C | `🧰️framework/🔨️modules/🌉️abi/🧬️schema/🔣️.json` | a | framework (top-level framework crate; 🌉️abi has no own Cargo.toml, mounted via #[path]) | 🌉️abi (owned ABI message ledger — Request/Reply/Event/Page/Control) | keep |
| C | `🧰️framework/🔨️modules/🎯️action-bus/🧹️wire-retirement/🧪️fixture/🧬️.schema.json` | b | 🎯️action-bus (folded into top-level framework crate) | 🎯️action-bus/🧹️wire-retirement/🧬️schema/🔣️.json (sibling of 🧪️fixture/, not inside it) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🎯️action-bus/🧹️wire-retirement/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🤝️mutation-leaf-contract/🛂️schema/🔣️.json` | c | 📡️replication (Cargo.toml id="replication", role="framework") | 📡️replication/🎮️mutation/🧪️tests/🤝️mutation-leaf-contract/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🤝️mutation-leaf-contract/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧭️mutation-leaf-source-contract/🛂️schema/🔣️.json` | c | 📡️replication (Cargo.toml id="replication", role="framework") | 📡️replication/🎮️mutation/🧪️tests/🧭️mutation-leaf-source-contract/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧭️mutation-leaf-source-contract/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/📡️replication/📐️format/🔎️verification/🧬️schema/🔣️.json` | a | 📡️replication | 📡️replication/📐️format/🔎️verification (retained-verification wire-format schema) | keep |
| C | `🧰️framework/🔨️modules/📡️replication/📐️format/🔎️verification/🧾️record/🧬️schema/🔣️.json` | a | 📡️replication | 📡️replication/📐️format/🔎️verification/🧾️record (retained-record-observation wire-format schema) | keep |
| C | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/📐️schema/🔣️.json` | c | 📡️replication | 📡️replication/📡️wire/🏠️local-interaction/🌳️root/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/🩹️update/📐️schema/🔣️.json` | c | 📡️replication | 📡️replication/📡️wire/🏠️local-interaction/🌳️root/🩹️update/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/🩹️update/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/📡️transport/📐️schema/🔣️.json` | c | 📡️replication | 📡️replication/📡️wire/🏠️local-interaction/📡️transport/🧫️fixtures/🧬️.schema.json (fixture-shape validator; consolidate/rename so it stops literally colliding with the real 🧬️schema/ sibling) | move-to 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/📡️transport/🧫️fixtures/🧬️.schema.json (or merge its $defs into the real transport 🧬️schema/🔣️.json) — it must not keep sitting as a second 'schema'-named directory sibling to the authoritative 🧬️schema/ |
| C | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/📡️transport/🧬️schema/🔣️.json` | a | 📡️replication | 📡️replication/📡️wire/🏠️local-interaction/📡️transport (query/command/reply wire-protocol contract) | keep (but see finding #15 — its non-canonical sibling '📐️schema/' should be relocated so this remains the sole 🧬️schema/ under 📡️transport/) |
| C | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/♻️retirement/🧬️.schema.json` | b | 📡️replication | 📡️replication/📡️wire/🏠️local-interaction/🧬️schema/♻️retirement/🔣️.json OR consolidate as a named file under the module's existing 🧬️schema/ dir | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/♻️retirement/🔣️.json |
| C | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/🏠️local-interaction/🧬️.schema.json` | b | 📡️replication | 📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🏠️local-interaction/🔣️.json OR consolidate as a named file under the module's existing 🧬️schema/ dir | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🏠️local-interaction/🔣️.json |
| C | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/📃️query/🧬️.schema.json` | b | 📡️replication | 📡️replication/📡️wire/🏠️local-interaction/🧬️schema/📃️query/🔣️.json OR consolidate as a named file under the module's existing 🧬️schema/ dir | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/📃️query/🔣️.json |
| C | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/🔐️topology-authority/🧬️.schema.json` | b | 📡️replication | 📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🔐️topology-authority/🔣️.json OR consolidate as a named file under the module's existing 🧬️schema/ dir | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🔐️topology-authority/🔣️.json |
| C | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🏠️local-interaction/🔣️.schema.json` | a | 📡️replication | 📡️replication/📡️wire/🏠️local-interaction (the canonical local-interaction wire contract; the module's own name is redundantly repeated one level inside 🧬️schema/) | move-to 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🔣️.json (drop the redundant inner '🏠️local-interaction/' directory — the module itself is already named 🏠️local-interaction, so nesting it again inside 🧬️schema/ is pure duplication, THE flagged nested-schema oddity) |
| C | `🧰️framework/🔨️modules/📡️replication/🔗️causal/🧪️fixtures/🧬️mutations/➕️causal-add/🛂️schema/🔣️.json` | c | 📡️replication | This is a synthetic/mock mutation-owner structure entirely nested inside 🧪️fixtures/ (owner path literally starts with '.../🧪️fixtures/...'), used only as example/test data exercising the generic Mutation<T> trait machinery in 🔗️causal/🦀️.rs — not a real, discoverable production mutation owner | follow-up-needed |
| C | `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/👥️presence-peer-codec-v1/🧬️schema/🔣️.json` | a | 📡️replication | 📡️replication/🧫️fixtures/👥️presence-peer-codec-v1 (presence peer wire-codec neutral corpus schema) | keep |
| C | `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/🚀️artifact-bootstrap/🔣️.json` | f | 📡️replication | n/a — this is fixture data, not a contract | keep |
| C | `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/🚀️artifact-bootstrap/🧬️.schema.json` | d | 📡️replication | 📡️replication/🧫️fixtures/🚀️artifact-bootstrap/🧬️schema/🔣️.json (needs its own wrapping 🧬️schema/ directory; currently a fully flat file with no wrapping schema-dir AND sitting directly inside a 🧫️fixtures/ subtree — combines patterns (b) and (d)) | move-to 🧰️framework/🔨️modules/📡️replication/🧫️fixtures/🚀️artifact-bootstrap/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts` | g | 🕸️graph (Cargo.toml id="graph", role="framework") | n/a — this IS the module's own sanctioned 📜️script.ts (CLAUDE.md: all permanent scripts live in 📜️script.ts at their directory) | keep |
| C | `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️outputs.json` | f | 🕸️graph | n/a — build-manifest config data | keep |
| C | `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧬️outputs.schema.json` | d | 🕸️graph | 🕸️graph/🛂️manifest/🧬️schema/🔣️.json (or accept as legitimate build-manifest infra given 🛂️manifest is itself a recognized taxonomy directory kind — see open question) | follow-up-needed |
| C | `🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🔗️.graphql` | a | framework (top-level framework crate; 🕹️interaction has no own Cargo.toml) | 🕹️interaction (InteractionDefinition/InteractionState/PresenceInteraction canonical multi-language schema facet set) | keep |
| C | `🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🔣️.json` | a | framework (top-level framework crate; 🕹️interaction has no own Cargo.toml) | 🕹️interaction (InteractionDefinition/InteractionState/PresenceInteraction canonical multi-language schema facet set) | keep |
| C | `🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🟦️.ts` | a | framework (top-level framework crate; 🕹️interaction has no own Cargo.toml) | 🕹️interaction (InteractionDefinition/InteractionState/PresenceInteraction canonical multi-language schema facet set) | keep |
| C | `🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🦀️.rs` | a | framework (top-level framework crate; 🕹️interaction has no own Cargo.toml) | 🕹️interaction (InteractionDefinition/InteractionState/PresenceInteraction canonical multi-language schema facet set) | keep |
| C | `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/📇️catalog.json` | f | 🖼️assets (@semio-tech/assets nx package) | n/a — mesh-source catalog data | keep |
| C | `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🧬️catalog.schema.json` | d | 🖼️assets | 🖼️assets/🌱️metabolism/🎨️representation/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🚚️delivery.json` | f | 🖼️assets | n/a — delivery-authority config data | keep |
| C | `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🧪️delivery-cases.json` | e | 🖼️assets | n/a — inert test-case corpus (URL/path request-resolution table) | keep |
| C | `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🧬️delivery.schema.json` | d | 🖼️assets | 🖼️assets/🔍️resolver/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/📇️catalog.json` | f | 🖼️assets | n/a — font family/weight/subset catalog data | keep |
| C | `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧬️catalog.schema.json` | d | 🖼️assets | 🖼️assets/🔤️fonts/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧬️schema/🔣️.json — NOTE: cross-module consumers in 🖱️ui/🎨️styling must be updated in the same change |
| C | `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json` | f | 🖼️assets | n/a — mesh delivery catalog data (collections+entries) | keep |
| C | `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🟦️.ts` | g | 🖼️assets | n/a — module implementation code (hand-rolled catalog parser/composer), not a schema definition | keep |
| C | `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🦀️.rs` | g | 🖼️assets | n/a — Rust mirror of the TS implementation above | keep |
| C | `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🔣️.json` | e | 🖼️assets | n/a — inert test fixture (delivery+catalogs+expected+unknown test vectors) | keep |
| C | `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧬️catalog.schema.json` | d | 🖼️assets | 🖼️assets/🥽️mesh/🧬️schema/🔣️.json | move-to 🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧬️schema/🔣️.json — NOTE: multiple cross-module consumers in 🖱️ui must be updated in the same change |
| C | `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🔮️oracle/🔣️.json` | f | 🗺️surface (Cargo.toml role="framework") | n/a — third-party oracle registration, not an application schema; its OWN $schema authority lives outside this slice | keep |
| C | `🧰️framework/🔨️modules/🗺️surface/🧪️tests/🧬️bindings.schema.json` | d | 🗺️surface | 🗺️surface/🧪️tests/🧬️schema/🔣️.json (or 🗺️surface/🧬️schema/🔣️.json at module root) | move-to 🧰️framework/🔨️modules/🗺️surface/🧪️tests/🧬️schema/🔣️.json |
| C | `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs` | a | framework (top-level framework crate; 🚪️io has no own Cargo.toml at this level — only its 🔤️base64 sub-package does) | 🚪️io (StandardId/SubsetId/Dialect/ArtifactDialect/ArtifactKindId/ArtifactRef/IoPayload/Confidence/IoFidelity/IoError/IoOutcome/IoResult/IoEntryDescriptor/IoRoute) | keep |
| C | `🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🎛️tutorial-local-interaction.schema.json` | b | 🛂️manifest (top-level framework module) | 🛂️manifest/🧬️schema/🎛️tutorial-local-interaction.schema.json (or per-fixture 🧬️schema/ subdirectories) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🎛️tutorial-local-interaction.schema.json |
| C | `🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/📜️action-semantics.schema.json` | b | 🛂️manifest (top-level framework module) | 🛂️manifest/🧬️schema/📜️action-semantics.schema.json (or per-fixture 🧬️schema/ subdirectories) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🛂️manifest/🧬️schema/📜️action-semantics.schema.json |
| C | `🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🗄️artifact-kind-formats.schema.json` | b | 🛂️manifest (top-level framework module) | 🛂️manifest/🧬️schema/🗄️artifact-kind-formats.schema.json (or per-fixture 🧬️schema/ subdirectories) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🗄️artifact-kind-formats.schema.json |
| C | `🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🛤️tutorial-document-track.schema.json` | b | 🛂️manifest (top-level framework module) | 🛂️manifest/🧬️schema/🛤️tutorial-document-track.schema.json (or per-fixture 🧬️schema/ subdirectories) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🛤️tutorial-document-track.schema.json |
| C | `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs` | g | 🧬️schema (Cargo.toml id="schema", role="framework") | n/a — this module IS the schema-validation framework infrastructure itself, not an application contract | keep |
| C | `🧰️framework/🔨️modules/🧬️schema/✅️validator.rs` | g | 🧬️schema | n/a — JSON-schema validation ENGINE implementation (ValidationControl, cancellable traversal) | keep |
| C | `🧰️framework/🔨️modules/🧬️schema/🔣️.json` | f | 🧬️schema module directory (coincidental — file is not schema-related) | n/a — false positive: this is the repo's semantic-emoji icon-legend catalog (id/emoji/iconId/label/filterable rows), unrelated to JSON Schema despite living in the 🧬️schema module directory and using the 🔣️.json naming convention | keep |
| C | `🧰️framework/🔨️modules/🧬️schema/🟦️.ts` | g | 🧬️schema | n/a — meta-schema type definitions (FacetLeaves/ArtifactSchemaDescriptor/ArtifactInferenceDescriptor) describing the SHAPE of the artifact-schema-facet registration system itself | keep |
| C | `🧰️framework/🔨️modules/🧵️job/⏱️budget/🕰️clock.json` | f | 🧵️job (Cargo.toml id="job", role="framework") | n/a — host-clock conversion fixture data, correctly paired with 🧬️schema/⏱️clock.json via its own $schema pointer | keep |
| C | `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧫️fixture/🔣️.json` | e | 🧵️job | n/a — inert fixture data | keep |
| C | `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧬️schema/⏱️clock.json` | a | 🧵️job | 🧵️job/⏱️budget (semio.framework.job.host-clock.v1) | keep |
| C | `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧬️schema/🪢️binding.json` | a | 🧵️job | 🧵️job/⏱️budget (semio.framework.job.microsecond-binding.v1) | keep |
| C | `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧬️schema/🪫️budget.json` | a | 🧵️job | 🧵️job/⏱️budget (job microsecond-budget fuel/deadline schema) | keep |
| C | `🧰️framework/🔨️modules/🧵️job/⏱️budget/🪢️binding.json` | f | 🧵️job | n/a — mutation-admission-law fixture data, correctly paired with 🧬️schema/🪢️binding.json | keep |
| C | `🧰️framework/🔨️modules/🧵️job/🧪️fixtures/📡️shared-framework-action-routes.schema.json` | b | 🧵️job | 🧵️job/🧬️schema/📡️shared-framework-action-routes.schema.json (or 🧵️job/🧪️fixtures/🧬️schema/…) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🧵️job/🧬️schema/📡️shared-framework-action-routes.schema.json |
| C | `🧰️framework/🔨️modules/🧵️job/🧪️fixtures/🧬️fixed-operation-registry.schema.json` | b | 🧵️job | 🧵️job/🧬️schema/🧬️fixed-operation-registry.schema.json (or 🧵️job/🧪️fixtures/🧬️schema/…) | fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🧵️job/🧬️schema/🧬️fixed-operation-registry.schema.json |
| D | `♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema.json` | d | mit-bestand/bericht/documents module | mit-bestand/bericht/documents module (needs a 🧬️schema/ directory wrapper) | move-to ♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema/🔣️.json |
| D | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema.json` | d | mit-bestand/demonstrator/runtime module | mit-bestand/demonstrator/runtime module (needs a 🧬️schema/ directory wrapper) | move-to ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🧬️schema.json` | d | print/print-font-catalog module | print/print-font-catalog module (needs a 🧬️schema/ directory wrapper) | move-to 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🧬️schema.json` | d | print/tectonic-template-compilation/catalog module | print/tectonic-template-compilation/catalog module (needs a 🧬️schema/ directory wrapper) | move-to 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🧬️schema.json` | d | print/tectonic-template-compilation/bundle module | print/tectonic-template-compilation/bundle module (needs a 🧬️schema/ directory wrapper) | move-to 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🧬️schema.json` | d | print/tectonic-template-compilation/toolchain module | print/tectonic-template-compilation/toolchain module (needs a 🧬️schema/ directory wrapper) | move-to 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🔗️graphql/🔗️.graphql` | a | repo/client/mcp module | repo/client/mcp module | keep (follow-up-needed on directory convention) |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` | f | repo/library/caching module | repo/library/caching module | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/bun-dependencies/🧬️schema.json` | b | repo/library/caching test suite (bun-dependencies fixture) | repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/bun-dependencies/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it) |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/command-boundaries/🧬️schema.json` | b | repo/library/caching test suite (command-boundaries fixture) | repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/command-boundaries/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it) |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/native-inputs/🧬️schema.json` | b | repo/library/caching test suite (native-inputs fixture) | repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/native-inputs/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it) |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/native-preparation/🧬️schema.json` | b | repo/library/caching test suite (native-preparation fixture) | repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/native-preparation/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it) |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/nx-contract/🧬️.schema.json` | b | repo/library/caching test suite (nx-contract fixture) | repo/library/caching module's own 🧬️schema/ | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/nx-contract/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/runtime-components/🧬️schema.json` | b | repo/library/caching test suite (runtime-components fixture) | repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/runtime-components/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it) |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️policy.schema.json` | d | repo/library/caching module | repo/library/caching module (needs a 🧬️schema/ directory wrapper) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📦️extension-installation-owner/🧬️.schema.json` | b | repo/library TypeScript package test suite (extension-installation-owner fixture) | repo/library/📦️packages/🟦️typescript module's own 🧬️schema/ | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧬️schema/extension-installation-owner/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🧬️join-provenance/🔣️.json` | b | repo/library TypeScript package test suite (rust-physical-reference-context / join-provenance fixture) | repo/library/📦️packages/🟦️typescript module's own 🧬️schema/ | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧬️schema/join-provenance/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` | e | repo/library/discovery module | n/a (not a schema file) | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧬️compiler-imports.schema.json` | d | repo/library/discovery module | repo/library/discovery module (needs a 🧬️schema/ directory wrapper) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🧬️schema/🔣️.json` | a | test-case scope: ↪️rust-divergence-callback | test-case scope: ↪️rust-divergence-callback | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🧬️schema.json` | a | test-case scope: ⏱️process-budgets | test-case scope: ⏱️process-budgets | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🧬️schema/🔣️.json` | a | test-case scope: ♻️taxonomy-pattern-compiler-reuse | test-case scope: ♻️taxonomy-pattern-compiler-reuse | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🧬️schema/🔣️.json` | a | test-case scope: ♻️taxonomy-pattern-compiler-reuse | test-case scope: ♻️taxonomy-pattern-compiler-reuse | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🛂️schema.json` | d | test-case scope: ✅️mutation-test-presence | test-case scope: ✅️mutation-test-presence (rename to 🧬️schema/🔣️.json) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🧬️schema/🔣️.json` | a | test-case scope: ✍️rust-writable-path-authority | test-case scope: ✍️rust-writable-path-authority | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🧪️imported-data/🧬️schema/🔣️.json` | a | test-case scope: 🌐️registry-import-language | test-case scope: 🌐️registry-import-language | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌱️mutation-root-discovery/🧬️schema/🔣️.json` | a | test-case scope: 🌱️mutation-root-discovery | test-case scope: 🌱️mutation-root-discovery | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🛂️schema.json` | d | test-case scope: 🌳️workspace-taxonomy | test-case scope: 🌳️workspace-taxonomy (rename to 🧬️schema/🔣️.json) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🧬️schema/🔣️.json` | a | test-case scope: 🎟️reference-coverage-selection | test-case scope: 🎟️reference-coverage-selection | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🧬️.schema.json` | a | test-case scope: 🏎️nextest | test-case scope: 🏎️nextest | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🛂️schema.json` | d | test-case scope: 🏗️mutation-scaffolding | test-case scope: 🏗️mutation-scaffolding (rename to 🧬️schema/🔣️.json) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️owned-generator-preview-inventory/🧬️schema/🔣️.json` | a | test-case scope: 🏭️owned-generator-preview-inventory | test-case scope: 🏭️owned-generator-preview-inventory | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🛂️schema.json` | d | test-case scope: 🏷️metadata-source-provider | test-case scope: 🏷️metadata-source-provider (rename to 🧬️schema/🔣️.json) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🧬️schema/🔣️.json` | a | test-case scope: 👀️readme-reviewed-fixture-inputs | test-case scope: 👀️readme-reviewed-fixture-inputs | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧬️schema/🔣️.json` | a | test-case scope: 👀️readme-reviewed-fixture-inputs | test-case scope: 👀️readme-reviewed-fixture-inputs | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🧬️schema/🔣️.json` | a | test-case scope: 📈️reference-coordinate-progress | test-case scope: 📈️reference-coordinate-progress | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎫️ticket-role-routing/🧬️schema/🔣️.json` | a | test-case scope: 📋️mutation-inventory | test-case scope: 📋️mutation-inventory | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎭️source-roster-roles/🧬️schema/🔣️.json` | a | test-case scope: 📋️mutation-inventory | test-case scope: 📋️mutation-inventory | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/📸️source-index-capture/🧬️schema/🔣️.json` | a | test-case scope: 📋️mutation-inventory | test-case scope: 📋️mutation-inventory | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🛂️schema/🔣️.json` | c | test-case scope: 📋️mutation-inventory (root contract) | test-case scope: 📋️mutation-inventory (rename directory 🛂️schema/ to 🧬️schema/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🧬️schema/🔣️.json` | a | test-case scope: 📋️mutation-inventory | test-case scope: 📋️mutation-inventory | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🧪️registration/🧬️schema/🔣️.json` | a | test-case scope: 📍️draw-destination-observation | test-case scope: 📍️draw-destination-observation | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🧬️schema/🔣️.json` | a | test-case scope: 📍️draw-destination-observation | test-case scope: 📍️draw-destination-observation | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🛂️schema/🔣️.json` | c | test-case scope: 📡️mutation-reachability | test-case scope: 📡️mutation-reachability (rename directory 🛂️schema/ to 🧬️schema/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/💥️malformed/🧬️schema/🔣️.json` | a | test-case scope: 📣️typescript-declaration-facts | test-case scope: 📣️typescript-declaration-facts | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🚫️unsupported/🧬️schema/🔣️.json` | a | test-case scope: 📣️typescript-declaration-facts | test-case scope: 📣️typescript-declaration-facts | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🧬️schema/🔣️.json` | a | test-case scope: 📣️typescript-declaration-facts | test-case scope: 📣️typescript-declaration-facts | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️semantic-package-source-manifest-identity/🧬️schema/🔣️.json` | a | test-case scope: 📦️semantic-package-source-manifest-identity | test-case scope: 📦️semantic-package-source-manifest-identity | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🛂️schema.json` | d | test-case scope: 📽️cargo-provider-projection | test-case scope: 📽️cargo-provider-projection (rename to 🧬️schema/🔣️.json) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔍️filesystem/🧫️fixtures/🔣️.json` | e | test-case scope: 🔍️filesystem | test-case scope: 🔍️filesystem | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🔣️.json` | f | test-case scope: 🔏️path-emoji-statutes | test-case scope: 🔏️path-emoji-statutes | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🧬️schema/🔣️.json` | a | test-case scope: 🔏️path-emoji-statutes | test-case scope: 🔏️path-emoji-statutes | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🧬️schema/🔣️.json` | a | test-case scope: 🔖️readme-current-source-revision | test-case scope: 🔖️readme-current-source-revision | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🧬️schema/🔣️.json` | a | test-case scope: 🔗️markdown-inline-references | test-case scope: 🔗️markdown-inline-references | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🧪️registration/🧬️schema/🔣️.json` | a | test-case scope: 🔤️taxonomy-leading-grapheme | test-case scope: 🔤️taxonomy-leading-grapheme | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🧬️schema/🔣️.json` | a | test-case scope: 🔤️taxonomy-leading-grapheme | test-case scope: 🔤️taxonomy-leading-grapheme | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/📨️submitted-proof/🧬️schema/🔣️.json` | a | test-case scope: 🖍️draw-source-scenario | test-case scope: 🖍️draw-source-scenario | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🙈️residue-ignore/🧬️schema/🔣️.json` | a | test-case scope: 🖍️draw-source-scenario | test-case scope: 🖍️draw-source-scenario | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛟️residue-recovery/🧬️schema/🔣️.json` | a | test-case scope: 🖍️draw-source-scenario | test-case scope: 🖍️draw-source-scenario | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛣️commit-route/🧬️schema/🔣️.json` | a | test-case scope: 🖍️draw-source-scenario | test-case scope: 🖍️draw-source-scenario | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛫️preflight-context/🧬️schema/🔣️.json` | a | test-case scope: 🖍️draw-source-scenario | test-case scope: 🖍️draw-source-scenario | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🧬️schema/🔣️.json` | a | test-case scope: 🖍️draw-source-scenario | test-case scope: 🖍️draw-source-scenario | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🧬️schema/🔣️.json` | a | test-case scope: 🗺️testing-readme-coordinates | test-case scope: 🗺️testing-readme-coordinates | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🧬️schema/🔣️.json` | a | test-case scope: 🚚️readme-move-source-authority | test-case scope: 🚚️readme-move-source-authority | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🧬️schema/🔣️.json` | a | test-case scope: 🟢️readme-current-source-activation | test-case scope: 🟢️readme-current-source-activation | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json` | f | test-case scope: 🤝️package-language-kind-handoff / 💾️resident-package | same | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🧬️schema/🔣️.json` | a | test-case scope: 🤝️package-language-kind-handoff | test-case scope: 🤝️package-language-kind-handoff | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json` | f | test-case scope: 🤝️package-language-kind-handoff / 🖥️ui-host-package | same | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🖥️ui-host-package/🧬️schema/🔣️.json` | a | test-case scope: 🤝️package-language-kind-handoff | test-case scope: 🤝️package-language-kind-handoff | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🧬️schema/🔣️.json` | a | test-case scope: 🤝️package-language-kind-handoff | test-case scope: 🤝️package-language-kind-handoff | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🧬️schema/🔣️.json` | a | test-case scope: 🥒️gherkin-description-inline-code | test-case scope: 🥒️gherkin-description-inline-code | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🧬️schema/🔣️.json` | a | test-case scope: 🥤️rust-finite-target-consumption | test-case scope: 🥤️rust-finite-target-consumption | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🧬️schema.json` | a | test-case scope: 🦀️exact-cargo-laws | test-case scope: 🦀️exact-cargo-laws | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🛂️schema.json` | d | test-case scope: 🧬️mutation-type-origin | test-case scope: 🧬️mutation-type-origin (rename to 🧬️schema/🔣️.json) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🧬️fixture.schema.json` | d | test-case scope: 🧼️clean | test-case scope: 🧼️clean (rename to 🧬️schema/🔣️.json) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧫️fixture/🧬️fixture.schema.json` | b | test-case scope: 🪟️windows-checkout-paths | test-case scope: 🪟️windows-checkout-paths (own 🧬️schema/, not inside 🧫️fixture/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🛂️schema.json` | d | test-case scope: 🪢️cargo-provider-binding | test-case scope: 🪢️cargo-provider-binding (rename to 🧬️schema/🔣️.json) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🛂️schema/🔣️.json` | c | test-case scope: 🪪️mutation-metadata | test-case scope: 🪪️mutation-metadata (rename directory 🛂️schema/ to 🧬️schema/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📋️registration/🧬️schema/🔣️.json` | a | test-case scope: 🪶️artifact-empty-facet-authoring | test-case scope: 🪶️artifact-empty-facet-authoring | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📨️request/🧬️schema/🔣️.json` | a | test-case scope: 🪶️artifact-empty-facet-authoring | test-case scope: 🪶️artifact-empty-facet-authoring | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🧬️schema/🔣️.json` | a | test-case scope: 🪶️artifact-empty-facet-authoring | test-case scope: 🪶️artifact-empty-facet-authoring | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/📐️options.schema.json` | d | test-case scope: 🫙️artifact-empty-facet-authority | test-case scope: 🫙️artifact-empty-facet-authority (fold into 🧬️schema/ as a second facet, or rename+move) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧬️schema/📐️options/🔣️.json (or fold as an additional required field into the existing 🧬️schema/🔣️.json) |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧪️registration/🧬️schema/🔣️.json` | a | test-case scope: 🫙️artifact-empty-facet-authority | test-case scope: 🫙️artifact-empty-facet-authority | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧬️schema/🔣️.json` | a | test-case scope: 🫙️artifact-empty-facet-authority | test-case scope: 🫙️artifact-empty-facet-authority | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️.schema.json` | d | repo/library module (repo-wide mutation-descriptor authority) | repo/library module — 🧬️schema/ directory at the library module root | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🧬️schema/🔣️.json` | a | test-case scope: 🚪️source-admission | test-case scope: 🚪️source-admission | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🔣️.json` | a | repo/library/normalization module | repo/library/normalization module | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧫️fixtures/🧬️g3-event-schema.json` | b | repo/server module / 🎛️coordinator (Go event store) | repo/server module's own 🧬️schema/ (e.g. 🧬️schema/🎛️coordinator/) | move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🧬️schema/🎛️coordinator/🔣️.json (recast as a real JSON Schema draft-07 document if it is meant to validate the 📜️g3-event-log.jsonl fixture, or otherwise as clearly-labeled encoding-contract metadata) |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📇️registry/🔣️.json` | f | repo/test module | repo/test module | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧭️contribution-directory-ownership/🧬️schema/🔣️.json` | a | test-case scope: 🧭️contribution-directory-ownership | test-case scope: 🧭️contribution-directory-ownership | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json` | a | repo/test module | repo/test module | keep |
| D | `🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🧬️schema/🔣️.json` | a | test-case scope: 🧪️transaction-process-ownership | test-case scope: 🧪️transaction-process-ownership | keep |

## Coverage (merged)

### Slice A coverage

## Coverage note
- 102/102 candidate files from `candidates.txt` audited; every path appears exactly once in `findings.json`, no path added, no path dropped.
- Sanity sweep performed with `git grep -l '"$schema"' -- '🧰️framework/🔨️modules/🖱️ui/*'` and `find 🧰️framework/🔨️modules/🖱️ui -iname '*schema*'` — no additional schema-shaped files were found beyond the given candidate list.
- Two candidates turned out to be **prepass false positives** (matched the literal string `"$schema"` inside source code, not an actual schema document): `🎨️styling/📦️packages/🦀️rust/📜️script.ts` (a TS type declares a `$schema` field) and `components.json` (shadcn/ui tool config, references an external, non-semio schema URL). Both kept in `findings.json` (classification f/g) with `decision: keep` rather than silently dropped, per the audit's exhaustiveness requirement.
- One candidate (`🎨️styling/🧪️tests/🧊️mesh-collection.json`) is inert test fixture data (e) whose `$schema` pointer names a real schema, but that schema is owned by the sibling `🖼️assets` module, outside this ui slice — flagged, not resolved, here.
- Zero `📐️schema` directory-kind registrations exist anywhere in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (confirmed via a full JSON scan) — every `📐️schema/` directory found in this slice (16 of them) is confirmed non-canonical.
- The dominant violation pattern (20 files across 10 owners under `🧬️contract/🧵️retained/💾️resident/`, plus 2 more under `♻️retirement/🌳️typed/` and 2 under `🩹️operations/📥️wire/📃️pages/`) is a **duplicate-authority pair**: one wrong-named `📐️schema/🔣️.json` (large, detailed, unconsumed-by-any-literal-reference) beside one flat `🧬️schema.json` (small, also unconsumed for the resident-tree cases). The two files' JSON keys do not overlap — they read as two genuinely different facets of the same owner rather than a stale copy-paste duplicate, so the fix is consolidation into one `🧬️schema/` directory holding two named files, not a delete-one-keep-one call. This mirrors the parent ticket's own seed observations (`📋️master-plan.md`: "Relocation alone is insufficient; the contract must be re-derived from the producer/consumer code").
- The second-largest pattern (33 files: 29 under `🧬️contract/🧵️retained/🧪️fixtures/*/🧬️.schema.json` plus 4 under `🧬️contract/🧵️retained/📦️wire/🧪️fixtures/*/🧬️.schema.json`) is fixture-owned contracts: every one of these taxonomy-registered fixture-case directories (`ui-retained-fixture-cases`) holds both a data instance (`🔣️.json`) and its own schema (`🧬️.schema.json`) side by side, with no traced reference to a real owner-scope schema. This audit could not individually trace which owner directory each of the 33 fixture shapes actually belongs to (that requires a full producer/consumer pass, explicitly deferred to the master-plan's WP3/WP4) — flagged uniformly as classification b with the owner left as an open question rather than guessed.
- Confirmed real, load-bearing consumers (via `include_str!`/`readFileSync`/`import`) for the classification-a and several classification-d/c findings; classification c (`📐️schema/`) findings uniformly had **zero** literal-path consumers anywhere in the repo, which is itself notable evidence that this directory-naming pattern may be entirely dead weight rather than merely misplaced — flagged as an open question per finding rather than asserted as fact, since a generic/taxonomy-driven consumption mechanism (confirmed to exist for the mutation-leaf case, see `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` ~line 3580) could still be reaching these files by a dynamically-built path this audit's grep-based method cannot see.

### Slice B coverage

## Coverage note

- **62 of 62** candidate-list files audited; none skipped.
- The independent sanity sweep (`git grep -l '"$schema"'` + `find -iname '*schema*'` over the same three trees) surfaced **zero** files beyond the 62 already in the candidate list — the `find` sweep's extra hits were all parent *directories* of files already audited (e.g. `.../📐️schema` the directory vs `.../📐️schema/🔣️.json` the file), and the `git grep` sweep's extra hits were 5 `project.json`/`package.json` files whose `"$schema"` key is a boilerplate nx-tooling pointer (`../node_modules/nx/schemas/project-schema.json`) — confirmed by inspection and correctly excluded by the pre-pass, so **zero false negatives and zero files were added** in this pass.
- **Zero candidates were discarded as false positives** — all 62 are genuine schema-shaped or schema-adjacent-config files; none turned out to be plain data instances misclassified by the pre-pass.
- Classification breakdown: **21 × d** (flat-file schema outside a module — the dominant pattern in actor, confirmed as the "primary known oddity" the task briefing predicted), **19 × c** (non-canonically-named `📐️schema/`/`📐️fixture-schema/` directory — the dominant pattern in kernel and a large secondary pattern in actor, also as predicted), **9 × b** (fixture-owned contract, concentrated in value's `🧪️fixtures/`/`🧫️fixtures/` directories and kernel's top-level `🧫️fixtures/` grab-bag), **12 × a** (already correct), **1 × f** (kernel's oracle registry config, not a contract).
- A systemic pattern emerged across nearly every `c`/`d` finding: almost every leaf directory in actor and kernel carries a genuine *fixture-shape* schema (validates a sibling `🧪️fixture/`/`🧫️fixture/` test-data file) **alongside** the real *domain-contract* schema. The domain contract is consistently the anonymous `🧬️schema.json`/`🧬️schema/🔣️.json`; the fixture-shape schema is consistently placed under a non-canonical name (`📐️schema/`, `📐️fixture-schema/`) specifically *because* `🧬️schema` is already taken by the domain contract at that same directory level. Fixing the `d` (flat-file) violations first — converting every flat `🧬️schema.json` into a proper `🧬️schema/🔣️.json` directory — does not free up room for the fixture-shape schema to simply become `🧬️schema/` too, since `mutationPayloadSchemaAuthority.descriptorCardinality` mandates exactly one canonical schema per owner. Every affected `c`/`d` pair in this report therefore recommends nesting the fixture-shape schema one level deeper, inside the sibling `🧪️fixture/`/`🧫️fixture/` directory it validates, as its own canonically-named `🧬️schema/` module — this is a judgment call for the human-facing merge report to confirm, not an established taxonomy convention (flagged as an open question below).
- **Cross-module fan-out:** two schemas in this slice are unusually high-fanout and should be prioritized/sequenced carefully in any rename: `🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json` (`semio.value.resident.capacity.v1`) is `$ref`'d/imported from kernel, actor, and ui; `🧰️framework/🔨️modules/🎭️actor/📃️page/🧬️schema.json` (`semio.actor.byte-page.v1`) and `🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema.json` (`semio.actor.retained-return.v1`) are `$ref`'d/imported from kernel and `os/plugin` as well as within actor itself. Renaming these must preserve the exact `$id` string (which is location-independent) while moving the file, then update every `await import("...")` path literal listed in each finding's Consumers.
- **Open cross-cutting question for the merge:** several `a`/`c` findings in kernel (`📚️entries/🧬️schema/`, `🏠️source/🧬️schema/`, `💌️message/🧬️schema/`, `📥️poll/🏘️composition/📐️fixture-schema/`) and a handful of `c`/`d` findings in actor's deep `🪪️activation/` tree had no consumer located by this pass's targeted `import(".../🧬️schema...")`-literal greps — most are almost certainly consumed by a test harness this pass's grep patterns did not match (e.g. a loop that constructs the path dynamically, or a Rust `include_str!` this pass did not specifically target for kernel/actor's deep subtree). None of these were reclassified as dead/orphaned on that basis alone; each is flagged individually with an `open` note recommending a follow-up grep rather than asserted as unused.

### Slice C coverage

#### Not violations — worth noting

## Not violations — worth noting

- `🧬️schema/🔣️.json` (inside the `🧬️schema` module itself) is a **false positive**: it is the repo's
  semantic-emoji icon-legend catalog (`{id, emoji, iconId, label, filterable}` rows), not JSON-Schema
  shaped at all — it was almost certainly caught by the pre-pass purely because it is a bare `🔣️.json`
  file living in a directory literally named `🧬️schema`. Confirmed registered as taxonomy data at
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:24649`.
- `🧬️schema/{⚛️component.rs,✅️validator.rs,🟦️.ts}` — all three are the `🧬️schema` module's own
  schema-validation-framework infrastructure (registry, cancellable validation engine, facet-descriptor
  meta-types), exactly the "framework infrastructure, not an application contract" case the task
  brief anticipated for this module. Classification (g).
- `🖼️assets/🥽️mesh/{🟦️.ts,🦀️.rs}` and `🕸️graph/📦️packages/🦀️rust/📜️script.ts` — module implementation/
  build-script code that happens to reference `"$schema"` as a literal string (hand-rolled shape
  checks, not schema definitions). Classification (g).
- `🧵️job/⏱️budget/{🕰️clock.json,🪢️binding.json}` and `📡️replication/🧫️fixtures/🚀️artifact-bootstrap/🔣️.json`,
  `🖼️assets/{🌱️metabolism/🎨️representation,🔍️resolver,🔤️fonts,🥽️mesh}/📇️catalog.json`,
  `🖼️assets/🔍️resolver/🚚️delivery.json` — all data instances that correctly reference their schema via
  a `"$schema"` pointer; not violations.
- `🕸️graph/🤖️generated/🔣️manifest.schema.json` — found during the directory sweep, NOT in the
  candidate list, and **untracked/gitignored** (`.gitignore:91: **/🤖️generated/`). It is the build
  output of the manifest-schema object literal at `🕸️graph/📦️packages/🦀️rust/📜️script.ts:286`
  (`$id: "manifest"`, title `"GraphManifestDocument"`) — classification (h), generated copy, correctly
  excluded from git. No action needed.

#### Open questions

## Open questions

1. `🕸️graph/🛂️manifest/🧬️outputs.schema.json` — is build-manifest catalog infra like this meant to be
   subject to the `🧬️schema/`-module convention at all, or is `🛂️manifest` a taxonomy-recognized
   directory kind exempt from placement rule (d)? Needs a taxonomy-owner decision, not a mechanical move.
2. Several fixture-owned schemas (`⏳️async` × 3, `🛂️manifest/🧪️fixtures/{🎛️tutorial-local-interaction,📜️action-semantics,🛤️tutorial-document-track}`, `📡️replication/🎮️mutation/🧪️tests` × 2) have NO confirmed active validator consumer anywhere in the repo (only their paired fixture DATA is read). Before/while relocating them, confirm whether they are genuinely dead weight (candidates for deletion) or intended for future wiring.
3. `📡️replication/🔗️causal/.../➕️causal-add/🛂️schema/🔣️.json` — confirm with the causal-module owner whether this mock fixture is meant to model the CORRECT canonical layout (in which case rename to `🧬️schema`) or deliberately uses a stand-in name for unrelated reasons.
4. Moving `🖼️assets/🔤️fonts/🧬️catalog.schema.json` and `🖼️assets/🥽️mesh/🧬️catalog.schema.json` requires coordinated updates in `🖱️ui/🎨️styling` and `🖱️ui/🧬️contract` (both outside this slice's ownership — audited elsewhere). Flag to whoever merges the sibling `🖱️ui` slice.
5. Renaming `📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🏠️local-interaction/🔣️.schema.json` (flattening the redundant nested dir) should be checked against any generic taxonomy-driven policy schema-discovery logic (`policyDiscoverAppSchemaOwners`/`policyArtifactSchemaBreaches` in root `📜️script.ts`) that might assume the current nested shape for facet-leaf discovery.

#### Coverage note

## Coverage note

- All 63 files in the supplied candidate list were audited; `findings.json` contains exactly 63
  entries with a 1:1 path match to `candidates.txt` (verified programmatically — no missing, no
  duplicates, no extras).
- Sanity sweep performed as instructed: `git grep -l '"$schema"'` scoped explicitly to all 34 of this
  slice's module directories (`⏱️trace ⏳️async ◻️2d ⚠️diagnostic ✍️editor 🌉️abi 🎒️pack 🎯️action-bus
  🏗️mesh-engine 📏️intrinsic-size 📐️geometry 📚️compiler 📡️replication 🔀️dispatch 🔄️machine 🔏️hash
  🔢️number 🔤️typeset 🔲️pixels 🔺️mesh 🕸️graph 🕹️interaction 🖌️raster 🖥️platform 🖼️assets 🗜️deflate
  🗺️surface 🚪️io 🛂️manifest 🧊️3d 🧩️action-argument-resolution 🧬️schema 🧮️math 🧵️job`) returned 88 hits;
  after excluding `package.json`/`project.json`/`tsconfig.json` boilerplate every remaining hit matched
  a file already in the candidate list. `find <module> -iname '*schema*'` was run per-module as a second
  pass; the only addition found was the untracked/gitignored `🕸️graph/🤖️generated/🔣️manifest.schema.json`
  (classification h, no action needed, documented above).
- Zero-candidate modules (`◻️2d`, `⚠️diagnostic`, `✍️editor`, `🎒️pack`, `🏗️mesh-engine`,
  `📏️intrinsic-size`, `📐️geometry`, `📚️compiler`, `🔀️dispatch`, `🔄️machine`, `🔏️hash`, `🔢️number`,
  `🔤️typeset`, `🔲️pixels`, `🔺️mesh`, `🖌️raster`, `🖥️platform`, `🗜️deflate`, `🧊️3d`,
  `🧩️action-argument-resolution`, `🧮️math`) were each individually confirmed empty via
  `find <module> -iname '*schema*'` — no output for any of them (the only near-hit, `✍️editor`, returned
  only stale `target/` build artifacts, not source).
- No files outside `🧰️framework/🔨️modules/*` were modified; the only file read outside the module tree
  for cross-reference purposes was the root `📜️script.ts` (build/test/policy script, itself the
  confirmed consumer for the majority of findings) and a handful of `🖱️ui`/`🌎️hub`/`💻️os`/`🦑️repo`
  cross-module consumer files (read-only, cited as evidence, not edited).

### Slice D coverage

## Coverage note

**Files audited:** 92 total — the 87-file pre-pass candidate list in full, plus 5 additional files this audit's own sanity sweep surfaced:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧫️fixtures/🧬️g3-event-schema.json` — explicitly named in the task brief as a flagged oddity but not present in the candidate list (it has no literal `"$schema"` key, so the pre-pass's content filter missed it); found via `find -iname '*schema*'` on the repo product tree.
- Four `📚️library/🧪️tests/*/🛂️schema.json` files (`🏷️metadata-source-provider`, `📽️cargo-provider-projection`, `🧬️mutation-type-origin`, `🪢️cargo-provider-binding`) — same reason (no literal `"$schema"` key despite being unambiguously JSON-Schema-shaped: `type`/`required`/`properties`/`$defs`), also found via the `-iname '*schema*'` sweep. These bring the total count of the `🛂️schema`-instead-of-`🧬️schema` drift in `📚️library/🧪️tests/` to **10 instances** (6 from the original candidate list + these 4), all classified (c)/(d) in this report.

**False positives discarded from the pre-pass's implicit scope:** none — every one of the 87 original candidates was schema-shaped, config-instance, or inert-fixture content genuinely worth classifying; none were boilerplate `package.json`/`project.json`/`tsconfig.json` editor-tooling noise that had slipped through.

**Root-level confirmation:** `ls -la /Users/ueli/Documents/semio` was checked directly (non-recursive) — no stray `🧬️schema.json`/`*.schema.json`/`schema/` file or directory exists at the repository root. Confirmed clean.

**Sanity-sweep methodology:** ran `git grep -l '"\$schema"'` scoped separately to each of the four product/tree roots plus root `🧪️tests/`, and `find -iname '*schema*'` (excluding `node_modules`, `target`) across the same five roots, then diffed both result sets against the candidate list.

**Deliberately excluded from findings despite matching `*schema*`:**
- `♻️mit-bestand/🔎️recherche/**` — roughly 60+ matches (Neo4j graph-database schema exports, archived migration notes, research CSVs/JSON dumps under `_archive/`, `_neo4j/`, `intake/`). These are historical research artifacts for an unrelated knowledge-graph research project embedded in the repo, not application contracts in the taxonomy's sense (no `🧬️schema/` module semantics apply, nothing here is consumed by product code). The pre-pass candidate list already correctly excluded all of these; this audit confirms that exclusion was right and did not add any of them to findings.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📐️schema.sql` (and its near-duplicate sibling `🗄️.sql`) — a real, substantial SQLite DDL schema using the legacy `📐️` prefix explicitly named in the task brief's violation patterns, and consumed by `⌨️cli/🔬️component_test.go` (indirectly, via a differently-pathed `repo/postgres/🛢️schema.sql` deployment copy — see `🖥️server/🧬️schema/🐘️postgres/🗄️.sql`'s doc-links to the same name). SQL is not one of taxonomy's `schemaFormats` (jsonschema/protobuf/graphql/rust/typescript/wit), so this sits outside the `mutationPayloadSchemaAuthority` contract system this audit was scoped to enforce. Flagged here as a **follow-up-needed** item rather than a full finding: whether SQL DDL falls under the same directory-naming convention (and should move under a `🧬️schema/` module, dropping the `📐️` prefix) is a call for whoever owns the taxonomy's directory-kind registration, not something this slice should decide unilaterally.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔍️discovery-schema-handoff/🔣️.json` — matched the `*schema*` filename sweep only because its containing directory is named `🔍️discovery-schema-handoff`; the file's own content (`schemaVersion`/`module`/`fullSchemaExpression`/`consumers[]`) is a plain data descriptor, not a schema definition, and does not compete with any authoritative schema. Correctly excluded.

**Widespread flat-`🧬️schema.json` pattern outside this slice (context only, not a finding here):** `git grep` for the literal flat filename `"🧬️schema.json"` turned up dozens of hits under `🧰️framework/🔨️modules/🎭️actor/`, `🧰️framework/🔨️modules/🎠️kernel/`, and `🧰️framework/🔨️modules/🌱️value/` — all outside this slice's scope (owned by the other `🧰️framework/🔨️modules/` auditor). This suggests the flat-file pattern this report treats as violation (d) may be a repo-wide, load-bearing convention rather than an isolated mistake confined to this slice. This audit follows the task brief's explicit instruction to treat flat `🧬️schema.json` files as violation (d) regardless, but flags for the merged report that a repo-wide decision (accept the flat-file convention as legitimate, or schedule a repo-wide migration to `🧬️schema/` directories) affects far more files than the 18 in this slice alone.

**Not independently re-verified:** the exact list of `mutationPayloadSchemaLocation`/`schemaChildDirs`/`schemaFormats`/`mutationPayloadSchemaAuthority` taxonomy facts supplied in the task brief were trusted as given rather than re-parsed from the 1MB `taxonomy.json`, except where this audit specifically re-opened `taxonomy.json` to check the `mit-bestand` `semanticDirectoryKinds` entry and the `mutation-descriptor-specimen`/`mutation-descriptor-agreement` entries (both confirmed present as described).

## Slice details

Every slice's per-violation and duplicate-authority detail sections, copied verbatim from its own `findings.md`, so nothing found by the sub-audits is lost in the merge.

### Slice A — `🖱️ui` — per-finding detail

## Per-finding detail — violations (b/c/d) and duplicate-authority (a)
Every classification b, c, d finding below, in candidate-list order. (Class-a findings are all clean/correct except where explicitly noted as part of a duplicate-authority pair with a b/c/d sibling — those siblings are cross-referenced inline instead of duplicated as separate subsections.)

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧬️favicon.schema.json`
- **blob**: `f467952991ab9ab80c356adc1e7aed9c7e3f5bef`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) defining the favicon manifest shape (validates 🌐️favicon.json)
- **owner (current)**: ui-styling
- **intended owner**: ui-styling::styling::favicon::Schema — should live at 🎨️styling/🧬️schema/🌐️favicon.json
- **proposed exports**: `ui-styling::styling::favicon::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧬️schema/🌐️favicon.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌐️favicon.json` — data instance referencing this schema via $schema pointer
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧬️favicon.schema.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sits as a flat *.schema.json file directly under 🎨️styling/, not wrapped in a 🧬️schema/ directory module — violates mutationPayloadSchemaLocation (directoryName "🧬️schema")
- **open questions**:
  - Relocating requires updating the $schema pointer in 🌐️favicon.json from "./🧬️favicon.schema.json" to "./🧬️schema/🌐️favicon.json" (or similar) — out of scope for this read-only audit.

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧬️schema.json`
- **blob**: `78e3c6e9380d3bc39fb3a84e2b81e8a9e0ffd74f`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "(untitled)" — flat-file schema for ui-runtime's retirement::tree contract
- **owner (current)**: ui-runtime
- **intended owner**: ui-runtime::retirement::tree::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-runtime::retirement::tree::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/📜️script.ts:10` — new Ajv(...).compile(JSON.parse(read("./🧬️schema.json")))
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#", title "(untitled)"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory — violates mutationPayloadSchemaLocation (directoryName "🧬️schema" must be a directory, not a flat file)

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧬️schema.json`
- **blob**: `9f8a666b84ec00854e319327235e2289669eda89`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "(untitled)" — flat-file schema for ui-runtime's document contract
- **owner (current)**: ui-runtime
- **intended owner**: ui-runtime::document::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-runtime::document::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:77` — readFileSync("../../📃️document/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#", title "(untitled)"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory — violates mutationPayloadSchemaLocation (directoryName "🧬️schema" must be a directory, not a flat file)

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧬️schema.json`
- **blob**: `1c312ceaf9695b1c20539229907d9c7e28567314`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "(untitled)" — flat-file schema for ui-runtime's ownership contract
- **owner (current)**: ui-runtime
- **intended owner**: ui-runtime::ownership::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-runtime::ownership::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:22` — readFileSync("../../📏️ownership/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#", title "(untitled)"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory — violates mutationPayloadSchemaLocation (directoryName "🧬️schema" must be a directory, not a flat file)

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧬️schema.json`
- **blob**: `7ca31d21c49422b991ee5f25893a4221e8ef9669`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "(untitled)" — flat-file schema for ui-runtime's output contract
- **owner (current)**: ui-runtime
- **intended owner**: ui-runtime::output::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-runtime::output::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:89` — readFileSync("../../📤️output/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#", title "(untitled)"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory — violates mutationPayloadSchemaLocation (directoryName "🧬️schema" must be a directory, not a flat file)

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🧬️schema.json`
- **blob**: `7be772b0fbf18fd3f659e59939545a50af5fccb0`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "(untitled)" — flat-file schema for ui-runtime's transaction contract
- **owner (current)**: ui-runtime
- **intended owner**: ui-runtime::transaction::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-runtime::transaction::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:169` — readFileSync("../../🔄️transaction/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#", title "(untitled)"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory — violates mutationPayloadSchemaLocation (directoryName "🧬️schema" must be a directory, not a flat file)

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🚪️handback/🧬️schema.json`
- **blob**: `eb9785bbae3b15a4351fc56f4b02d4745874a318`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "(untitled, 3 lines — trivial/empty-ish body)" — flat-file schema for ui-runtime's handback contract
- **owner (current)**: ui-runtime
- **intended owner**: ui-runtime::handback::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-runtime::handback::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🚪️handback/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:70` — readFileSync("../../🚪️handback/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🚪️handback/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#", title "(untitled, 3 lines — trivial/empty-ish body)"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory — violates mutationPayloadSchemaLocation (directoryName "🧬️schema" must be a directory, not a flat file)

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🩹️patch/🧬️schema.json`
- **blob**: `7ddc94ab96761015d70689563c03a16ce71d5394`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "(untitled, 3 lines — trivial/empty-ish body)" — flat-file schema for ui-runtime's patch contract
- **owner (current)**: ui-runtime
- **intended owner**: ui-runtime::patch::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-runtime::patch::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🩹️patch/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:63` — readFileSync("../../🩹️patch/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🩹️patch/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#", title "(untitled, 3 lines — trivial/empty-ish body)"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory — violates mutationPayloadSchemaLocation (directoryName "🧬️schema" must be a directory, not a flat file)

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧬️schema.json`
- **blob**: `df2e48ca9354102a634eab6fc8f8661f6d0f0674`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat schema for ui-contract's retirement/built contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retirement::built::Schema — should live at ♻️retirement/🌲️built/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::retirement::built::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/📜️script.ts:9` — new Ajv(...).compile(JSON.parse(read("./🧬️schema.json")))
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧬️schema.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat file, no wrapping 🧬️schema/ directory

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📐️schema/🔣️.json`
- **blob**: `c23b16e65f6cf1a999076f29110599e1947dee22`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — grants/componentVariants/patchVariants/document/ownership oracle for retirement's typed-tree contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retirement::typed::Schema — should live at ♻️retirement/🌳️typed/🧬️schema/ (consolidated with its sibling below)
- **proposed exports**: `ui-contract::retirement::typed::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file or its wrapping directory name; may be validated only by a generic/taxonomy-driven mechanism not traceable by literal-string search, or may be unwired
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📐️schema/🔣️.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️components.schema.json is a SECOND, differently-shaped schema for the same 🌳️typed owner (componentVariants oneOf-list of 18 exact component consts) — duplicate authority: two independent schemas for one owner, neither in the sanctioned 🧬️schema/ directory form
- **open questions**:
  - Content of this file and its 🧬️components.schema.json sibling do not overlap (different keys) — they read as two complementary facets (byte/variant oracle vs. exact component-shape enumeration) of the same 🌳️typed contract rather than a stale duplicate of identical content; recommend consolidating both into one 🧬️schema/ directory as two named files rather than deleting either, but this needs confirmation from someone tracing the retirement/typed producer code.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️components.schema.json`
- **blob**: `cbed1b85e9ca1ab6228a35a5d3d4cdbe622dc67c`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — exact 18-variant component-shape enumeration (oneOf consts) for retirement's typed-tree contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retirement::typed::ComponentsSchema — should live at ♻️retirement/🌳️typed/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retirement::typed::ComponentsSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️schema/🧬️components.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file or its wrapping directory name; may be validated only by a generic/taxonomy-driven mechanism not traceable by literal-string search, or may be unwired
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️components.schema.json:1 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat named *.schema.json file, not wrapped in a 🧬️schema/ directory
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📐️schema/🔣️.json (see that entry) — two different wrong placements for the same 🌳️typed owner
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📋️list/📐️schema/🔣️.json`
- **blob**: `ae7dc1c576fdbb7d1231a7e6bf011af4843b3a46`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — schema for ui-contract's retirement/list contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retirement::list::Schema — should live at ♻️retirement/📋️list/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::retirement::list::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📋️list/🧬️schema/🔣️.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file or its wrapping directory name; may be validated only by a generic/taxonomy-driven mechanism not traceable by literal-string search, or may be unwired
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📋️list/📐️schema/🔣️.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📐️schema/🔣️.json`
- **blob**: `23fe818587aebddf7c5a651d164ef1d6c753370c`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — schema for ui-contract's retirement root contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retirement::Schema — should live at ♻️retirement/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::retirement::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🧬️schema/🔣️.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file or its wrapping directory name; may be validated only by a generic/taxonomy-driven mechanism not traceable by literal-string search, or may be unwired
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📐️schema/🔣️.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📮️handback/📐️schema/🔣️.json`
- **blob**: `6e21d8968a33d79df6fdeda4d40d5bf6a42ffc83`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — schema for ui-contract's retirement/handback contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retirement::handback::Schema — should live at ♻️retirement/📮️handback/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::retirement::handback::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📮️handback/🧬️schema/🔣️.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file or its wrapping directory name; may be validated only by a generic/taxonomy-driven mechanism not traceable by literal-string search, or may be unwired
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📮️handback/📐️schema/🔣️.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - No 📮️handback reference found anywhere in the ui-contract Rust crate (grepped for "📮️handback") — this owner directory may itself be orphaned
- **open questions**:
  - Could not find any consumer of the 📮️handback owner directory at all (not just its schema) — worth checking whether 📮️handback is dead code/dead contract before relocating its schema.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📐️schema/🔣️.json`
- **blob**: `aaa82b0c664e06ed735a5190115a04ee43f4266e`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — schema for ui-contract's retirement/patch contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retirement::patch::Schema — should live at ♻️retirement/🩹️patch/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::retirement::patch::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/🧬️schema/🔣️.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file or its wrapping directory name; may be validated only by a generic/taxonomy-driven mechanism not traceable by literal-string search, or may be unwired
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📐️schema/🔣️.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - The sibling 🩹️patch/🦀️.rs (Rust facet, wired via #[path] from 📃️document.rs:101) is the actively-used implementation; this JSON schema appears to be a separate, unwired facet

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema.json`
- **blob**: `e7063357240aae45a76b0132378b68f481b89d10`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat schema for ui-contract's retirement/patch/pending/whole contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retirement::patch::pending::whole::Schema — should live at 📦️whole/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::retirement::patch::pending::whole::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:96` — readFileSync("../../♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat file, no wrapping 🧬️schema/ directory

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧬️schema.json`
- **blob**: `8daa1b2810a7cd3d1f4eea3924c6fd8839c51ece`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat-file schema for ui-contract's compare::document contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::compare::document::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::compare::document::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:85` — readFileSync("../../⚖️compare/📃️document/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/🧬️schema.json`
- **blob**: `23d175c96649113320812581e3db1efaff8ddc0c`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat-file schema for ui-contract's compare contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::compare::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::compare::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:72` — readFileSync("../../⚖️compare/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🌳️root/🧬️schema.json`
- **blob**: `3eb2fc1b1615c9ba22de1a027dccccc9bfed6d82`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat-file schema for ui-contract's resident::root contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::resident::root::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::resident::root::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🌳️root/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:136` — readFileSync("../../🎟️resident/🌳️root/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🌳️root/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory
  - 3-line body — appears to be a trivial/degenerate schema (near-empty), verify intent before relocating

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🗃️fixed/🧬️schema.json`
- **blob**: `675979fa6f81cc678ffc6243261d83c0a8c9b011`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat-file schema for ui-contract's resident::fixed contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::resident::fixed::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::resident::fixed::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🗃️fixed/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:126` — readFileSync("../../🎟️resident/🗃️fixed/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🗃️fixed/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory
  - 3-line body — appears to be a trivial/degenerate schema (near-empty), verify intent before relocating

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧬️schema.json`
- **blob**: `b29d933441df2afc85ce4b400262f7bb6e066efa`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat-file schema for ui-contract's resident contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::resident::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::resident::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:114` — readFileSync("../../🎟️resident/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory
  - 3-line body — appears to be a trivial/degenerate schema (near-empty), verify intent before relocating

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🧬️schema.json`
- **blob**: `3973b16cd13c7a24bf1eec73ec332e063e0b6e25`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat-file schema for ui-contract's document::assembly contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::document::assembly::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::document::assembly::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:103` — readFileSync("../../📃️document/🎟️assembly/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory
  - 3-line body — appears to be a trivial/degenerate schema (near-empty), verify intent before relocating

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧬️schema.json`
- **blob**: `36775d0d538c229611ab69c5320b9c8a37b44216`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat-file schema for ui-contract's list contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::list::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::list::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:22` — readFileSync("../../📋️list/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🧬️schema.json`
- **blob**: `976d119a57a80c12cae674078d5046f0e001d592`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat-file schema for ui-contract's bindings::copy contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::bindings::copy::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::bindings::copy::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:50` — readFileSync("../../🔗️bindings/📋️copy/🧬️schema.json")
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪞️copy/🧬️schema.json`
- **blob**: `a297278f1e3bf277aff6c6f33a8b5d751efbc3bf`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — flat-file schema for ui-contract's copy contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::copy::Schema — should live at <owner-dir>/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::copy::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪞️copy/🧬️schema/🔣️.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:59` — readFileSync("../../🪞️copy/🧬️schema.json") (as componentCopySchema)
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪞️copy/🧬️schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory) directly under the owner directory

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧬️catalog.schema.json`
- **blob**: `0a853a2e61a41db3dabd8ab7f6ceec6781d8ea5a`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) describing the shape of the conformance test corpus's own 📇️catalog.json manifest — test-infrastructure metadata, not an application/mutation contract
- **owner (current)**: ui-contract (test corpus infrastructure)
- **intended owner**: ui-contract::examples::conformance::CatalogSchema — should live at 🧬️contract/🧬️schema/🔣️conformance-catalog.json, with 📚️examples/🧪️conformance/ kept as pure corpus data
- **proposed exports**: `ui-contract::examples::conformance::CatalogSchema`
- **decision**: fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs (or a new 🧬️schema/🔣️conformance-catalog.json)
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🔬️conformance.rs:280` — corpus_has_no_orphan_fixtures() test asserts the corpus dir contains exactly {group-dirs, 📇️catalog.json, 🧬️catalog.schema.json}
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧬️catalog.schema.json:1-3 — draft-07 schema validating {version, roles{snapshot,expect,patch}, groups}
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🔬️conformance.rs:280 — corpus_dir() is 📚️examples/🧪️conformance/; test enumerates exactly this schema + 📇️catalog.json + group dirs
  - 📚️examples is a taxonomy-registered fixture/example directory kind (ui-conformance-cases, ownerKindIds includes members-of-members-of-examples) — a *.schema.json defining structure directly inside it is the fixture-owned-contract pattern
- **open questions**:
  - This schema describes the TEST CORPUS's own manifest shape (self-referential test infrastructure), not a domain mutation/payload contract — lower severity than the other fixture-owned findings; worth confirming with the ui-contract crate owner whether it should be treated as a genuine scope schema (kept, just renamed/relocated) rather than migrated away entirely.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️fixtures/🔣️.schema.json`
- **blob**: `9101868088ae8df7010f4c5ab5d87a7bb49de203`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating a 4-case node-ownership-TTL fixture; sits directly inside the top-level ui-contract 🧪️fixtures/ directory alongside its own data instance
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::<node-ownership-ttl owner>::Schema — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file or its wrapping directory name; may be validated only by a generic/taxonomy-driven mechanism not traceable by literal-string search, or may be unwired (sibling data file 👥️presence-overlay.json in the same directory IS consumed by 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/👥️presence.rs:111, but this schema file itself has no found consumer)
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️fixtures/🔣️.schema.json:1-3 — draft-07 schema, 4 exact test cases validating selected/hovered/previewed TTL semantics
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️fixtures/ also directly holds 👥️presence-overlay.json (a second, unrelated fixture data instance) — the directory is a genuine top-level fixtures dir per its own README/usage, and this schema file is co-located with fixture data rather than referencing an owner-scope schema
- **open questions**:
  - Could not trace which owner directory's contract this fixture-shape actually belongs to (selected/hovered/previewed node overlay TTL) — likely 🧬️contract/🔗️bindings or a presence-adjacent owner; needs a full producer/consumer trace before relocation, consistent with master-plan's WP0→WP3/WP4 split.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🔣️.schema.json`
- **blob**: `b3c25b801250486ce3eafd173d30ea8d537fa80e`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — schema for the retained scene's typed reader/read-step contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::scene::typed::Schema — should live at 🎬️scene/🧾️typed/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::retained::scene::typed::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧬️schema/🔣️.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file or its wrapping directory name; may be validated only by a generic/taxonomy-driven mechanism not traceable by literal-string search, or may be unwired; the sibling TS module 🎬️scene/🧾️typed/🟦️.ts (types OwnedUiPreparedSceneReader etc.) is imported by 🧵️retained/📖️read-lease/🟦️.ts:5, but that imports the .ts type module, not this JSON schema
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🔣️.schema.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Named as "🔣️.schema.json" but sitting flat directly in 🧾️typed/ with no 🧬️schema/ wrapper directory

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/📐️schema/🔣️.json`
- **blob**: `18c944bd66aacc36968f6c4bddfe9318126af1e5`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Builder Ownership Oracle" (31 lines) — detailed byte/field/law oracle for ui-contract's resident::builder contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::builder::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::builder::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Builder Ownership Oracle"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema.json is a SECOND, differently-shaped schema (title "UI Resident Builder Admission") for the same resident::builder owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema.json`
- **blob**: `aaf79df4469859e62efd39285b7dca152f98cb53`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Builder Admission" (22 lines) — state-machine admission schema for ui-contract's resident::builder contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::builder::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::builder::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Builder Admission"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/📐️schema/🔣️.json`
- **blob**: `05c1f2009d9ddd5e489039ddf204926ef3778197`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Page Admission Oracle" (434 lines) — detailed byte/field/law oracle for ui-contract's resident::page contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::page::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::page::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Page Admission Oracle"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema.json is a SECOND, differently-shaped schema (title "UI Resident Page Admission") for the same resident::page owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema.json`
- **blob**: `3480b7e81e51f6f755d7b853b41059737b777d79`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Page Admission" (19 lines) — state-machine admission schema for ui-contract's resident::page contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::page::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::page::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Page Admission"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/📐️schema/🔣️.json`
- **blob**: `ecfb3c078ce835256b0d9d65541fd2a5cbb2fae3`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Original Page Binding Oracle" (251 lines) — detailed byte/field/law oracle for ui-contract's resident::page::binding contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::page::binding::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::page::binding::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Original Page Binding Oracle"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema.json is a SECOND, differently-shaped schema (title "UI Resident Original Page Binding Domain") for the same resident::page::binding owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema.json`
- **blob**: `9ff71e971aeb21d34d28474650d930f7170d506f`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Original Page Binding Domain" (64 lines) — state-machine admission schema for ui-contract's resident::page::binding contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::page::binding::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::page::binding::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Original Page Binding Domain"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/📐️schema/🔣️.json`
- **blob**: `722e2b8e28d63d34369a85cf6a7fdf77ac64e8d9`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Streaming Reader Oracle" (425 lines) — detailed byte/field/law oracle for ui-contract's resident::reader contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::reader::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::reader::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Streaming Reader Oracle"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema.json is a SECOND, differently-shaped schema (title "UI Resident Streaming Reader Domain") for the same resident::reader owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema.json`
- **blob**: `8e92b5a105e0ac2620de74fa6093adab8bc0c7c0`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Streaming Reader Domain" (76 lines) — state-machine admission schema for ui-contract's resident::reader contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::reader::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::reader::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Streaming Reader Domain"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/📐️schema/🔣️.json`
- **blob**: `145088f2396619da1d9816311dd1aae097008ced`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Payload Metadata And Ownership Oracle" (29 lines) — detailed byte/field/law oracle for ui-contract's resident::payload contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::payload::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::payload::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Payload Metadata And Ownership Oracle"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema.json is a SECOND, differently-shaped schema (title "UI Resident Field-Owned Payload Admission") for the same resident::payload owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema.json`
- **blob**: `7247b1380fca94fbcb2cc376224c462321a8565f`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Field-Owned Payload Admission" (28 lines) — state-machine admission schema for ui-contract's resident::payload contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::payload::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::payload::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Field-Owned Payload Admission"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/📐️schema/🔣️.json`
- **blob**: `e916561a1c7eec8e9738e2ddc8525aede6ed9ebb`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Parent Slot Neutral Oracle" (23 lines) — detailed byte/field/law oracle for ui-contract's resident::slot contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::slot::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::slot::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Parent Slot Neutral Oracle"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema.json is a SECOND, differently-shaped schema (title "UI Resident Exact Parent Slot") for the same resident::slot owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema.json`
- **blob**: `217aeb69d211e15e3f0ed3d55d9b7ad84320f92f`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Exact Parent Slot" (21 lines) — state-machine admission schema for ui-contract's resident::slot contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::slot::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::slot::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Exact Parent Slot"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📐️schema/🔣️.json`
- **blob**: `8725849c81f454c0d9fd818c53287fc897677e5d`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Input Evidence Ownership Oracle" (26 lines) — detailed byte/field/law oracle for ui-contract's resident::evidence contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::evidence::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::evidence::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Input Evidence Ownership Oracle"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema.json is a SECOND, differently-shaped schema (title "UI Resident Input Evidence") for the same resident::evidence owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema.json`
- **blob**: `8b35b4bef0c8f4a3704786519013f502cc5b642c`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Input Evidence" (21 lines) — state-machine admission schema for ui-contract's resident::evidence contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::evidence::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::evidence::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Input Evidence"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/📐️schema/🔣️.json`
- **blob**: `93e3a1ff2b8253bf82fd063ba86e2b8ed85961df`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Copied Range Fixture" (160 lines) — detailed byte/field/law oracle for ui-contract's resident::evidence::copied contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::evidence::copied::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::evidence::copied::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Copied Range Fixture"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema.json is a SECOND, differently-shaped schema (title "UI Resident Copied Range Domain") for the same resident::evidence::copied owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema.json`
- **blob**: `058d2d8a1d4d2d03330fc887e53f09388c52463f`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Copied Range Domain" (60 lines) — state-machine admission schema for ui-contract's resident::evidence::copied contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::evidence::copied::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::evidence::copied::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Copied Range Domain"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/📐️schema/🔣️.json`
- **blob**: `03905ef7045d185136cd3b049c3acd9622c87584`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Active Input Cancellation Fixture" (113 lines) — detailed byte/field/law oracle for ui-contract's resident::evidence::cancellation contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::evidence::cancellation::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::evidence::cancellation::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Active Input Cancellation Fixture"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema.json is a SECOND, differently-shaped schema (title "UI Resident Active Input Cancellation Domain") for the same resident::evidence::cancellation owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema.json`
- **blob**: `ef8bea5c930d6bc76a737a12b6bdc2a1eba5e917`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "UI Resident Active Input Cancellation Domain" (56 lines) — state-machine admission schema for ui-contract's resident::evidence::cancellation contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::evidence::cancellation::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::evidence::cancellation::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Resident Active Input Cancellation Domain"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/📐️schema/🔣️.json`
- **blob**: `1d73a8b23891043025dcb95b95367acbea305445`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07) — "UI Fixed Metadata Neutral Oracle" (29 lines) — detailed byte/field/law oracle for ui-contract's resident::metadata contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::metadata::OracleSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::metadata::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/📐️schema/🔣️.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "UI Fixed Metadata Neutral Oracle"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json is a SECOND, differently-shaped schema (title "Owned UI Fixed Metadata Admission") for the same resident::metadata owner — different required/properties keys, not a content duplicate — duplicate authority: two independently-placed schemas for one owner
- **open questions**:
  - Content of the 📐️schema/🔣️.json and 🧬️schema.json siblings does not overlap key-for-key (compared directly for the 🏗️builder pair) — they read as two complementary facets of the same owner (byte/law oracle vs. state-machine admission) rather than a stale duplicate; recommend consolidating both into one 🧬️schema/ directory as two named files, pending confirmation from a producer/consumer trace.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json`
- **blob**: `9218a367de71e8797347f1e50e1d2ee87dd8b2ee`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "Owned UI Fixed Metadata Admission" (24 lines) — state-machine admission schema for ui-contract's resident::metadata contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::resident::metadata::AdmissionSchema — should live at 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::resident::resident::metadata::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "Owned UI Fixed Metadata Admission"
  - Flat 🧬️schema.json FILE (no wrapping 🧬️schema/ directory)
  - Duplicate-authority pair with 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/📐️schema/🔣️.json (see that entry)
- **open questions**:
  - See sibling 📐️schema/🔣️.json entry — same consolidation recommendation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema.json`
- **blob**: `8921c93cff77eb9973cf410597a1670830f38828`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07) — "Owned UI Shared Composition Capacity" (9 lines) — flat schema for the resident owner root itself
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::resident::Schema — should live at 💾️resident/🧬️schema/🔣️.json
- **proposed exports**: `ui-contract::retained::resident::Schema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema/🔣️.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file, its owner directory name, or the wrapping schema-directory name; likely validated (if at all) only by a generic taxonomy-driven mutation-leaf mechanism, not traceable by literal-string search
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "Owned UI Shared Composition Capacity"
  - Flat file, no wrapping 🧬️schema/ directory; no 📐️schema/ sibling exists at this level (unlike its child owners above)

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🏷️fields/🧬️.schema.json`
- **blob**: `7daaf7a9713522d7badb13c9e687f4be4a72d5be`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🏷️fields' retained-wire fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::wire::<owner for '🏷️fields'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically (its sibling data instance 🔣️.json IS consumed, see evidence)
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🏷️fields/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🏷️fields/🔣️.json holds the actual fixture example
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🏷️fields' wire fixture not traced — recommend a full producer/consumer pass (per master-plan WP3/WP4) rather than blind relocation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/📤️decode/🧬️.schema.json`
- **blob**: `20d34cd08f13227e19b595d34392da0d6641bb29`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '📤️decode' retained-wire fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::wire::<owner for '📤️decode'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically (its sibling data instance 🔣️.json IS consumed, see evidence)
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/📤️decode/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/📤️decode/🔣️.json holds the actual fixture example
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '📤️decode' wire fixture not traced — recommend a full producer/consumer pass (per master-plan WP3/WP4) rather than blind relocation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🗺️surface-bytes/🧬️.schema.json`
- **blob**: `2712b1ca59f85c3b573b93d3a70e02b32423889b`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🗺️surface-bytes' retained-wire fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::wire::<owner for '🗺️surface-bytes'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically (its sibling data instance 🔣️.json IS consumed, see evidence)
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🗺️surface-bytes/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🗺️surface-bytes/🔣️.json holds the actual fixture example
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🗺️surface-bytes' wire fixture not traced — recommend a full producer/consumer pass (per master-plan WP3/WP4) rather than blind relocation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🧾️typed/🧬️.schema.json`
- **blob**: `682cf70d09ab1daaae951a00d6e8ad1aea7298c0`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🧾️typed' retained-wire fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::wire::<owner for '🧾️typed'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically (its sibling data instance 🔣️.json IS consumed, see evidence)
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🧾️typed/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🧾️typed/🔣️.json holds the actual fixture example
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🧩️component.rs:539 — include_str!("../../🧵️retained/📦️wire/🧪️fixtures/🧾️typed/🔣️.json") consumes the DATA instance (not this schema)
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🧾️typed' wire fixture not traced — recommend a full producer/consumer pass (per master-plan WP3/WP4) rather than blind relocation.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/⚙️owned-operations/🧬️.schema.json`
- **blob**: `4200e303ff03fe7cf03157da57cc19691850a7a4`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '⚙️owned-operations' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '⚙️owned-operations'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/⚙️owned-operations/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/⚙️owned-operations/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '⚙️owned-operations' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌱️root-source/🧬️.schema.json`
- **blob**: `a5e594d0328cb0f57b7be789afd56df50d36dc60`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🌱️root-source' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🌱️root-source'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌱️root-source/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌱️root-source/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🌱️root-source' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌳️owned-nodes/🧬️.schema.json`
- **blob**: `1a8389be41e4ab284caeb04bd8c598c876ca39a1`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🌳️owned-nodes' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🌳️owned-nodes'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌳️owned-nodes/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🌳️owned-nodes/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🌳️owned-nodes' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎟️read-lease/🧬️.schema.json`
- **blob**: `d91978354e131b7d6bafad9ad5d78b3bf44e5d3b`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🎟️read-lease' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🎟️read-lease'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎟️read-lease/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎟️read-lease/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🎟️read-lease' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎬️owned-scene/🧬️.schema.json`
- **blob**: `59234c3c9ed74975a1087045f18bde6d48b4706d`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🎬️owned-scene' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🎬️owned-scene'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎬️owned-scene/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🎬️owned-scene/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🎬️owned-scene' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/👶️native-child/🧬️.schema.json`
- **blob**: `d9134d4d0fe0bf70ec6f922ad0582cf941d39f25`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '👶️native-child' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '👶️native-child'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/👶️native-child/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/👶️native-child/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '👶️native-child' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/💾️resident/🧬️.schema.json`
- **blob**: `20a49714e9a8eda14ecf3ab4572ce2f6da527f6b`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '💾️resident' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '💾️resident'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/💾️resident/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/💾️resident/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '💾️resident' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📃️scene-json-document/🧬️.schema.json`
- **blob**: `64d5d06aed8a2c20ea52c74e5642a1aa3981561f`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '📃️scene-json-document' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '📃️scene-json-document'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📃️scene-json-document/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📃️scene-json-document/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '📃️scene-json-document' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📤️read-publication/🧬️.schema.json`
- **blob**: `0e2a9e5bfc09daf2efcfd599055c6eecd8b26d02`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '📤️read-publication' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '📤️read-publication'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📤️read-publication/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📤️read-publication/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '📤️read-publication' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📥️intake/🧬️.schema.json`
- **blob**: `b2fb2347d3f95e75bc5598a72dbe49d6ea4b96dd`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '📥️intake' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '📥️intake'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📥️intake/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📥️intake/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '📥️intake' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📦️scene-generic-pack/🧬️.schema.json`
- **blob**: `8d8869b04234cc70f7084d81c0dace281ff4f7a2`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '📦️scene-generic-pack' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '📦️scene-generic-pack'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📦️scene-generic-pack/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📦️scene-generic-pack/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '📦️scene-generic-pack' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📨️wire-operations/🧬️.schema.json`
- **blob**: `0a72f50e60443a34d84b00abe87bdf50edeed1c2`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '📨️wire-operations' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '📨️wire-operations'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📨️wire-operations/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/📨️wire-operations/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '📨️wire-operations' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔏️owned-hash/🧬️.schema.json`
- **blob**: `4b02108fcdd04346bae43435ec862b861b129e39`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🔏️owned-hash' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🔏️owned-hash'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔏️owned-hash/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔏️owned-hash/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🔏️owned-hash' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔔️intake-notification/🧬️.schema.json`
- **blob**: `f8edcfd19f753b3898d0dd9ca269a9ff7b467d48`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🔔️intake-notification' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🔔️intake-notification'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔔️intake-notification/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔔️intake-notification/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🔔️intake-notification' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔗️scene-binding/🧬️.schema.json`
- **blob**: `73232753826865b26cfdeb636e708d00055c419b`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🔗️scene-binding' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🔗️scene-binding'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔗️scene-binding/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔗️scene-binding/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🔗️scene-binding' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔢️scene-numeric/🧬️.schema.json`
- **blob**: `cc3abd830262d5dc0356d15e9117508b5b09b7a9`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🔢️scene-numeric' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🔢️scene-numeric'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔢️scene-numeric/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔢️scene-numeric/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🔢️scene-numeric' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json/🧬️.schema.json`
- **blob**: `7c3a6fbcc86910eecd4d044a5d2943e52bd4ce94`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🔣️scene-json' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🔣️scene-json'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🔣️scene-json' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔤️scene-text-bytes/🧬️.schema.json`
- **blob**: `001d5ba2f2e6020794fc29d7951fd4003c55ceda`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🔤️scene-text-bytes' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🔤️scene-text-bytes'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔤️scene-text-bytes/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔤️scene-text-bytes/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🔤️scene-text-bytes' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🗺️owned-surface/🧬️.schema.json`
- **blob**: `bdcd5bd28ec2d0f2c65944584912a2e33a0d4fbc`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🗺️owned-surface' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🗺️owned-surface'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🗺️owned-surface/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🗺️owned-surface/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🗺️owned-surface' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚨️intake-close-fault/🧬️.schema.json`
- **blob**: `56018227ce03afc4620a7ec4e77e272c15dac461`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🚨️intake-close-fault' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🚨️intake-close-fault'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚨️intake-close-fault/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚨️intake-close-fault/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🚨️intake-close-fault' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚪️instance-close/🧬️.schema.json`
- **blob**: `d424bd0cb8a6019ce8153f2380eb5ba054a43a99`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🚪️instance-close' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🚪️instance-close'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚪️instance-close/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🚪️instance-close/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🚪️instance-close' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛠️instance-maintenance/🧬️.schema.json`
- **blob**: `5963bb8ae9c128677f9c58d5aee0b57bb3998030`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🛠️instance-maintenance' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🛠️instance-maintenance'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛠️instance-maintenance/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛠️instance-maintenance/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🛠️instance-maintenance' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛡️owned-validation/🧬️.schema.json`
- **blob**: `18075c28f8bc22019e55b8c9e0163b128d65e2b9`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🛡️owned-validation' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🛡️owned-validation'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛡️owned-validation/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🛡️owned-validation/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🛡️owned-validation' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧳️scene-pack-field/🧬️.schema.json`
- **blob**: `6fb0ae774439856518abdba42f5d2a7c10d0f2ab`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🧳️scene-pack-field' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🧳️scene-pack-field'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧳️scene-pack-field/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧳️scene-pack-field/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🧳️scene-pack-field' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧵️scene-json-string/🧬️.schema.json`
- **blob**: `d04578e3188ba6092a5b2746af71d555e2c35808`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🧵️scene-json-string' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🧵️scene-json-string'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧵️scene-json-string/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧵️scene-json-string/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🧵️scene-json-string' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧾️typed-scene/🧬️.schema.json`
- **blob**: `026f511b1e09253745639e704cb2f85ed7419465`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🧾️typed-scene' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🧾️typed-scene'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧾️typed-scene/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🧾️typed-scene/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🧾️typed-scene' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🩹️patch/🧬️.schema.json`
- **blob**: `4a3c28c1baa6fbe15c76e03967ebe5516c9b299e`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🩹️patch' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🩹️patch'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🩹️patch/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🩹️patch/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🩹️patch' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪆️surface-child/🧬️.schema.json`
- **blob**: `457689bd212c04795cea8d438b9b0d7f0ada4735`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🪆️surface-child' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🪆️surface-child'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪆️surface-child/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪆️surface-child/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🪆️surface-child' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪪️instance-owner/🧬️.schema.json`
- **blob**: `98ab32c23466e1e6fb923402347376726dc4a0f2`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) validating the '🪪️instance-owner' retained-tree fixture case shape; co-located with its own data instance inside a 🧪️fixtures/ directory
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::<owner for '🪪️instance-owner'> — real owner not identified by this audit; needs producer/consumer trace
- **decision**: fixture-should-reference-scope-schema <owner 🧬️schema/ module, not yet identified>
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this schema file specifically
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪪️instance-owner/🧬️.schema.json — "$schema": "http://json-schema.org/draft-07/schema#"
  - Sibling data instance 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🪪️instance-owner/🔣️.json holds the actual fixture example (verified present on disk for '⚙️owned-operations' and '💾️resident')
  - Directory is a taxonomy-registered fixture directory (ui-retained-fixtures, semanticDirectoryKinds parentKindIds ["retained","ui-retained-wire"]) whose member names are enumerated in taxonomy.semanticDirectoryMemberKinds.ui-retained-fixture-cases — legitimately a fixtures/examples location, but the *.schema.json inside it is acting as the authoritative shape definition rather than referencing a real owner's 🧬️schema/ module
- **open questions**:
  - Exact owning mutation-leaf/read-path for the '🪪️instance-owner' fixture not traced individually — recommend a full producer/consumer pass (per master-plan WP3/WP4) covering all 28 fixture cases together, likely against the 🔢️scalar or 🩹️operations schemas in this same 🧵️retained tree.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/📐️schema/🔣️.json`
- **blob**: `5155b8e0239fa865afbfc76119042c3d3c25a792`
- **classification**: c — non-canonical schema-directory name, e.g. 📐️schema/ (violation)
- **role**: JSON Schema (draft-07, 325 lines) — detailed schema for the retained operations/wire/pages contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::operations::wire::pages::OracleSchema — should live at 📃️pages/🧬️schema/ (consolidated with its 🧬️schema.json sibling)
- **proposed exports**: `ui-contract::retained::operations::wire::pages::OracleSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema/🔣️.oracle.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts` — imports something from the 🩹️operations/📥️wire/📃️pages directory tree (module-level, not confirmed to be this JSON schema specifically)
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/📐️schema/🔣️.json:2 — "$schema": "http://json-schema.org/draft-07/schema#"
  - Directory name "📐️schema" is not the sanctioned "🧬️schema" name. taxonomy.json has zero registrations for "📐️schema" as a directoryKind anywhere in the repo (verified via python3 json scan) — this directory name is not a sanctioned schema-module name
  - Sibling 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema.json (title "Retained Operation Payload Admission") is a SECOND, differently-shaped schema for the same 📃️pages owner — duplicate authority
- **open questions**:
  - Could not confirm the 💾️resident/🟦️.ts import specifically targets this JSON schema vs. a TS type module in the same tree — flagged as a weak/unconfirmed consumer link.

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema.json`
- **blob**: `c4d91c62382757ef133853da3185a9a30efee698`
- **classification**: d — flat-file schema outside a module (violation)
- **role**: JSON Schema (draft-07, 17 lines) — "Retained Operation Payload Admission" — flat schema for the retained operations/wire/pages contract
- **owner (current)**: ui-contract
- **intended owner**: ui-contract::retained::operations::wire::pages::AdmissionSchema — should live at 📃️pages/🧬️schema/ (consolidated with its 📐️schema/ sibling)
- **proposed exports**: `ui-contract::retained::operations::wire::pages::AdmissionSchema`
- **decision**: move-to 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema/🔣️.admission.json
- **consumers**:
  - `n/a` — no literal path-string consumer found repo-wide via grep for this file
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema.json:2-3 — "$schema": "http://json-schema.org/draft-07/schema#", "title": "Retained Operation Payload Admission"
  - Flat file, no wrapping 🧬️schema/ directory
  - Duplicate-authority pair with the 📐️schema/🔣️.json sibling (see that entry)

### `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧬️contracts/♿️modal/🧬️.schema.json`
- **blob**: `50ea27af34d0cb3299470d967f2340d867d39661`
- **classification**: b — fixture-owned contract (violation)
- **role**: JSON Schema (draft-07) — self-identifies as "semio.ui.dialog-accessibility-fixture.v1" — validates the UIDialog accessibility test-case fixture (locale/title/description/field/kind/cancel/submit cases + expected a11y assertions); co-located with its own data instance
- **owner (current)**: ui-elements (UIDialog)
- **intended owner**: ui::elements::UIDialog::AccessibilityFixtureSchema — should live at 📨️UIDialog/🧬️schema/♿️modal.json (or a shared a11y-fixture schema module if this pattern repeats across other elements)
- **proposed exports**: `ui::elements::UIDialog::AccessibilityFixtureSchema`
- **decision**: fixture-should-reference-scope-schema 🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧬️schema/♿️modal.json
- **consumers**:
  - `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/🟦️.tsx:13-14` — import fixture from "../🧬️contracts/♿️modal/🔣️.json"; import schema from "../🧬️contracts/♿️modal/🧬️.schema.json";
- **evidence**:
  - 🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧬️contracts/♿️modal/🧬️.schema.json:5 — "schema": { "const": "semio.ui.dialog-accessibility-fixture.v1" } — self-identifies explicitly as a fixture schema
  - Directory is named "🧬️contracts" (plural, non-canonical — neither "🧬️schema" nor "📐️schema") and directly co-locates the schema with its own data instance 🔣️.json
  - 🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/🟦️.tsx:13-14 — both files imported together by the same test module
- **open questions**:
  - The 🧬️contracts/ (plural) naming convention appears local to this one UIDialog element; worth checking other 🧱️elements/*/🧬️contracts/ directories repo-wide for the same pattern (out of this audit's ui-only-candidate-list scope, but the pattern is likely to recur across sibling elements).

### Slice B — `🎭️actor` / `🎠️kernel` / `🌱️value` — violations in detail

## Violations in detail

Every classification-`b`, `c`, and `d` finding, in candidate-list order grouped by module. (No classification-`a` finding in this slice turned out to be a duplicate-authority case needing its own subsection — every `a` file is a single, uncontested schema in a correctly-named `🧬️schema/` directory; see the Findings table above for the full `a` list.)

### value module

### `🧰️framework/🔨️modules/🌱️value/💾️resident/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating value/resident's own 🧫️fixture (trace/law) test data ($id semio.value.resident.fixture.v1)
- **Current owner:** value/resident (crate id=value-resident)
- **Intended owner:** value/resident (crate id=value-resident)
- **Proposed named export(s):** `residentFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🌱️value/💾️resident/🧫️fixture/🧬️schema/🔣️.json (rename non-canonical 📐️schema/ dir; cannot simply rename to 🧬️schema/ in place because the sibling flat 🧬️schema.json already claims that name for the domain capacity contract — see companion finding for that file, classification d)
- **Consumers:**
  - `🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts:12` — typescript static import (fixtureSchema)
- **Evidence:**
  - 🧰️framework/🔨️modules/🌱️value/💾️resident/📐️schema/🔣️.json:3 — "$id": "semio.value.resident.fixture.v1" (id ends .fixture.v1, confirms fixture-shape role, not a competing domain contract)
  - 🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts:12 — import fixtureSchema from "./📐️schema/🔣️.json" (active consumer)
  - directory listing of 🧰️framework/🔨️modules/🌱️value/💾️resident/ shows sibling 📐️schema/, 🧬️schema.json (flat), 🧫️fixture/ — 📐️schema/ is a non-canonical directory name per taxonomy mutationPayloadSchemaLocation (directoryName must be 🧬️schema)

### `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating value/resident/admission's own 🧫️fixture test data ($id semio.value.resident.admission.fixture.v1)
- **Current owner:** value/resident/admission (sub-path of crate value-resident)
- **Intended owner:** value/resident/admission (sub-path of crate value-resident)
- **Proposed named export(s):** `admissionFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the domain admission contract)
- **Consumers:**
  - `🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts:17` — typescript static import (admissionFixtureSchema)
- **Evidence:**
  - 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/📐️schema/🔣️.json:3 — "$id": "semio.value.resident.admission.fixture.v1"
  - 🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts:17 — import admissionFixtureSchema from "./📨️admission/📐️schema/🔣️.json"
  - sibling 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema.json is the flat domain contract (see separate finding, classification d)

### `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema for resident admission charge/refusal ($id semio.value.resident.admission.contract.v1)
- **Current owner:** value/resident/admission
- **Intended owner:** value/resident/admission
- **Proposed named export(s):** `admissionContractSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir exists at this level so this is a straight conversion)
- **Consumers:**
  - `🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts:15` — typescript static import (admissionContractSchema)
  - `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema.json:9` — self $ref to semio.value.resident.capacity.v1#/definitions/resources
- **Evidence:**
  - 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema.json:3 — "$id": "semio.value.resident.admission.contract.v1"
  - 🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts:15 — import admissionContractSchema from "./📨️admission/🧬️schema.json"
  - file is a FILE (🧬️schema.json) not a 🧬️schema/ DIRECTORY — violates mutationPayloadSchemaLocation directoryName="🧬️schema" (a directory, not a file)

### `🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema for shared resident capacity accounting ($id semio.value.resident.capacity.v1, title "Shared Resident Capacity")
- **Current owner:** value/resident
- **Intended owner:** value/resident
- **Proposed named export(s):** `residentCapacitySchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json (flat file -> directory module). HIGH-FANOUT file: this is the single most widely $ref'd schema found in the whole slice (kernel, actor, and ui all depend on its $id string), so the rename must preserve the exact $id value untouched and only change file location.
- **Consumers:**
  - `🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts:13` — typescript static import (capacitySchema)
  - `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema.json:9` — json-schema $ref semio.value.resident.capacity.v1#/definitions/resources
  - `🧰️framework/🔨️modules/🌱️value/💾️resident/🤝️contract.json:4` — companion data file field "capacitySchema": "semio.value.resident.capacity.v1"
  - `🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/📐️fixture-schema/🔣️.json:11` — json-schema $ref (cross-module, kernel)
  - `🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/📜️contract/🔣️.json:4` — companion data field capacitySchema (cross-module, kernel)
  - `🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧬️schema/🔣️.json:9` — json-schema $ref (cross-module, kernel)
  - `🧰️framework/🔨️modules/🎭️actor/🏘️composition/🏗️bootstrap/🧬️schema.json:38` — json-schema $ref (cross-module, actor)
  - `🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json:27` — json-schema $ref (cross-module, actor)
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/🧬️schema.json:80,94,108` — json-schema $ref (cross-module, actor)
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:2247,2501,2519` — typescript dynamic import + Ajv addSchema (cross-module, actor)
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🤝️contract.json, 🧬️schema.json and further ui paths` — json-schema $ref / companion data (cross-module, ui; outside audited slice, listed for completeness)
- **Evidence:**
  - 🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json:3 — "$id": "semio.value.resident.capacity.v1", title "Shared Resident Capacity"
  - no competing 🧬️schema/ directory exists at this level (only 📐️schema/ which is a distinct fixture-shape schema, see companion finding)

### `🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️fixtures/🧬️.schema.json`

- **Classification:** b — fixture-owned contract (violation)
- **Role:** fixture-owned schema validating the exact-integer codec corpus ($id https://semio.tech/schemas/value/exact-integer-codec-v1.json, title "Exact Integer Value Codec Corpus")
- **Current owner:** value/🔁️codec (no independent Cargo crate; 🔁️codec/🦀️.rs is a mod compiled into the semio-framework-replication crate, id=replication)
- **Intended owner:** value/🔁️codec (should still get its own 🧬️schema/ module even though the owning crate is 'replication')
- **Proposed named export(s):** `exactIntegerCodecCorpusSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🌱️value/🔁️codec/🧬️schema/🔣️.json (there is no 🧬️schema/ anywhere under 🔁️codec; this anonymous .schema.json sitting directly inside 🧪️fixtures/ is the ONLY schema defining this contract, so it must be promoted out of the fixtures directory into a proper canonical module rather than merely renamed in place)
- **Consumers:**
  - `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:456` — rust include_str! of sibling fixture data 🧪️fixtures/🔣️.json validated against this schema at test time (schema itself not include_str!'d — validated only via the TS oracle below)
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2981` — rust include_str! of the SAME fixture data via a relative path reach-through from an unrelated product module (os/directory), confirming this corpus is treated as a shared oracle beyond value/🔁️codec itself
- **Evidence:**
  - 🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️fixtures/🧬️.schema.json:3 — $id https://semio.tech/schemas/value/exact-integer-codec-v1.json, title "Exact Integer Value Codec Corpus"
  - directory listing of 🧰️framework/🔨️modules/🌱️value/🔁️codec/ shows only 🦀️.rs and 🧪️fixtures/ — no 🧬️schema/ directory exists anywhere in this leaf
  - 🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:456 and 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2981 both include_str! the sibling fixture DATA file (🔣️.json), not this schema file directly — the schema is consumed by a TS-side Ajv oracle (not directly grepped by id since it uses a URL $id, not a bare semio.* token)
- **Open questions:**
  - No TS consumer of this specific schema file was located by direct import/require grep in this slice; it may be validated only ad hoc (e.g. an oracle script outside the 62-file candidate set) — recommend the merge report double-check for a 📜️script.ts under 🔁️codec/ or a sibling test harness that Ajv-compiles this $id.

### `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧪️fixtures/🛡️references.schema.json`

- **Classification:** b — fixture-owned contract (violation)
- **Role:** fixture-owned schema validating the 🔗️references.json lookup-case fixture (anonymous $id)
- **Current owner:** value/🗂️ordered/🔢️numeric (no independent crate; compiled into semio-framework-replication, id=replication)
- **Intended owner:** value/🗂️ordered/🔢️numeric
- **Proposed named export(s):** `numericReferencesFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧬️schema/🔗️references.json (no 🧬️schema/ directory exists anywhere under 🔢️numeric; two distinct named fixtures each need their own schema entry inside one shared canonical module)
- **Consumers:**
  - `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/📜️script.ts:14` — typescript static import (referenceSchema), compiled against referenceFixture from 🔗️references.json
- **Evidence:**
  - 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧪️fixtures/🛡️references.schema.json has no $id/title (verified via python3 json load — both None)
  - 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/📜️script.ts:13-14 — import referenceFixture from "./🧪️fixtures/🔗️references.json"; import referenceSchema from "./🧪️fixtures/🛡️references.schema.json"
  - directory listing of 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/ shows only 📋️project.json, 📜️script.ts, 🟦️.ts, 🧪️fixtures/ — no 🧬️schema/ directory

### `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧪️fixtures/🧬️numeric-index.schema.json`

- **Classification:** b — fixture-owned contract (violation)
- **Role:** fixture-owned schema validating the 🔢️numeric-index.json stress/ordinal fixture (anonymous $id)
- **Current owner:** value/🗂️ordered/🔢️numeric (compiled into semio-framework-replication, id=replication)
- **Intended owner:** value/🗂️ordered/🔢️numeric
- **Proposed named export(s):** `numericIndexFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧬️schema/🔢️numeric-index.json (same target module as the sibling references schema; both anonymous fixture schemas consolidate under one canonical 🧬️schema/ directory as named files)
- **Consumers:**
  - `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/📜️script.ts:12,286-288` — typescript static import (schema) + Ajv.compile + assert(validate(fixture))
- **Evidence:**
  - 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧪️fixtures/🧬️numeric-index.schema.json has no $id/title
  - 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/📜️script.ts:11-12 — import fixture from "./🧪️fixtures/🔢️numeric-index.json"; import schema from "./🧪️fixtures/🧬️numeric-index.schema.json"
  - 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/📜️script.ts:286-288 — new Ajv({...}).compile(schema); assert(validate(fixture), ...)

### `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/👥️shared-owner/🧬️.schema.json`

- **Classification:** b — fixture-owned contract (violation)
- **Role:** fixture-owned schema validating the shared-owner ordered-map aliasing fixture ($id semio.value.ordered-map.shared-owner)
- **Current owner:** value/🗂️ordered (compiled into semio-framework-replication, id=replication)
- **Intended owner:** value/🗂️ordered
- **Proposed named export(s):** `orderedMapSharedOwnerSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧬️schema/👥️shared-owner.json (no 🧬️schema/ directory exists anywhere under 🗂️ordered top level; consolidate with the sibling ordered-map schema finding below into one canonical module)
- **Consumers:**
  - `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/📜️script.ts:39-40` — typescript static-URL Bun.file(...).json() load + Ajv.compile, region //#region 📤️SharedOwnership
- **Evidence:**
  - 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/👥️shared-owner/🧬️.schema.json:3 — "$id": "semio.value.ordered-map.shared-owner"
  - 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/📜️script.ts:39-40 — const sharedSchema = await Bun.file(new URL("./👥️shared-owner/🧬️.schema.json", import.meta.url)).json(); const validateShared = new Ajv(...).compile(sharedSchema)
  - directory listing of 🧰️framework/🔨️modules/🌱️value/🗂️ordered/ (maxdepth 2) shows 🦀️.rs, 🧪️tests/, 🧫️fixtures/, 🧺️set/ — no top-level 🧬️schema/

### `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/🧬️.schema.json`

- **Classification:** b — fixture-owned contract (violation)
- **Role:** fixture-owned schema for the core ordered-map contract (insertion order, admission, lifecycle) — $id semio.value.ordered-map
- **Current owner:** value/🗂️ordered (compiled into semio-framework-replication, id=replication)
- **Intended owner:** value/🗂️ordered
- **Proposed named export(s):** `orderedMapSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧬️schema/🔣️.json (this is the primary/anonymous schema for the ordered-map domain concept itself, not a secondary named one; it becomes the canonical anonymous file in the new module, with 👥️shared-owner and the numeric ones as named siblings if consolidated at a shared level, or its own dedicated module if kept scoped to 🗂️ordered only)
- **Consumers:**
  - `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/📜️script.ts:7-8,20` — typescript static-URL Bun.file(...).json() load + Ajv.compile, region //#region 🧬️Contract
- **Evidence:**
  - 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/🧬️.schema.json:3 — "$id": "semio.value.ordered-map"
  - 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/📜️script.ts:7-8 — const fixture = await Bun.file(new URL("./🔣️ordered-map.json", ...)).json(); const schema = await Bun.file(new URL("./🧬️.schema.json", ...)).json()
  - this is the ONLY schema for the ordered-map domain concept anywhere in the value module — no separate 🧬️schema/ exists for 🗂️ordered at any level

### kernel module

### `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📐️fixture-schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating kernel return-content's own 🧫️fixture test data ($id semio.kernel.return-content.fixture.v1); references semio.actor.byte-page.v1 by $ref
- **Current owner:** kernel/📤️return/📦️content
- **Intended owner:** kernel/📤️return/📦️content
- **Proposed named export(s):** `returnContentFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ directory already claims 🧬️schema for the domain content-declaration contract, see companion finding classification a)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts:271` — typescript dynamic import (fixtureSchema), await import("./📐️fixture-schema/🔣️.json")
- **Evidence:**
  - 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📐️fixture-schema/🔣️.json:3 — "$id": "semio.kernel.return-content.fixture.v1"
  - 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts:269-274 — const {default:schema}=await import("./🧬️schema/🔣️.json"); const {default:fixtureSchema}=await import("./📐️fixture-schema/🔣️.json"); (both loaded together for the same test block, plus actor's page/lifetime/patch schemas)
  - directory listing: 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/ has both 📐️fixture-schema/ AND 🧬️schema/ as siblings — 🧬️schema/ already correctly named and occupied by the real domain contract (semio.kernel.return-content-declaration.v1)

### `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/📐️fixture-schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating kernel return/content/input/builder's own binding-trace test data ($id kernel-return-builder-binding-tests.v1)
- **Current owner:** kernel/📤️return/📦️content/📥️input/🏗️builder
- **Intended owner:** kernel/📤️return/📦️content/📥️input/🏗️builder
- **Proposed named export(s):** `builderBindingTestsSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ directory already claims 🧬️schema for the real domain builder-binding contract, kernel-return-builder-binding.v1)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:3261` — typescript dynamic import (fixtureSchema), cross-module test consumer in actor's shard-client oracle harness
- **Evidence:**
  - 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/📐️fixture-schema/🔣️.json:3 — "$id": "kernel-return-builder-binding-tests.v1"
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:3260-3261 — const {default:contract}=await import(".../🏗️builder/📜️contract/🔣️.json"); const {default:schema}=await import(".../🏗️builder/🧬️schema/🔣️.json"); const {default:fixture}=await import(".../🏗️builder/🧫️fixture/🔣️.json"); const {default:fixtureSchema}=await import(".../🏗️builder/📐️fixture-schema/🔣️.json")
  - directory listing shows sibling 🧬️schema/ dir (real domain contract) already occupying the canonical name

### `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/📐️fixture-schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema, title "Return Field Resident Payload Neutral Cases", validating the payload association's own test cases (no $id)
- **Current owner:** kernel/📤️return/📦️content/📥️input/📦️payload
- **Intended owner:** kernel/📤️return/📦️content/📥️input/📦️payload
- **Proposed named export(s):** `residentPayloadNeutralCasesSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ already claims 🧬️schema for the real domain resident-payload-association contract)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:3381` — typescript dynamic import (fixtureSchema), cross-module consumer
- **Evidence:**
  - 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/📐️fixture-schema/🔣️.json:5 — "title": "Return Field Resident Payload Neutral Cases" (no $id)
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:3380-3381 — sibling contract/schema/fixture/fixtureSchema quartet imported together for the payload test block

### `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/📐️fixture-schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating kernel return/content/input/release's own test data ($id semio.kernel.return-input.release.fixture.v1)
- **Current owner:** kernel/📤️return/📦️content/📥️input/🧾️release
- **Intended owner:** kernel/📤️return/📦️content/📥️input/🧾️release
- **Proposed named export(s):** `releaseFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ already claims 🧬️schema for the real release contract)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:3321` — typescript dynamic import (fixtureSchema), cross-module consumer
- **Evidence:**
  - 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/📐️fixture-schema/🔣️.json:3 — "$id": "semio.kernel.return-input.release.fixture.v1"
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:3320-3321 — sibling contract/schema/fixture/fixtureSchema quartet imported together

### `🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/📐️fixture-schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating kernel poll/composition's own test data ($id semio.kernel.poll.composition.fixture.v1); references semio.value.resident.capacity.v1 by $ref
- **Current owner:** kernel/📥️poll/🏘️composition
- **Intended owner:** kernel/📥️poll/🏘️composition
- **Proposed named export(s):** `pollCompositionFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling 🧬️schema/ already claims 🧬️schema for the real poll-composition contract)
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/📐️fixture-schema/🔣️.json:3 — "$id": "semio.kernel.poll.composition.fixture.v1"
  - 🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/📐️fixture-schema/🔣️.json:9 — $ref "semio.kernel.poll.composition.v1" and $ref "semio.value.resident.capacity.v1" (cross-module reference to value)
  - directory listing shows sibling 🧬️schema/ dir (real domain contract) already occupying the canonical name
- **Open questions:**
  - No direct TS import/require of this exact fixture-schema file was located by the targeted greps in this pass (the companion 🧬️schema/ file IS consumed, see its own finding) — recommend the merged report re-check os/plugin/📥️poll/🏘️composition/🟦️.ts and kernel's own test harness for an indirect load.

### `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📇️descriptor-load/🧬️.schema.json`

- **Classification:** b — fixture-owned contract (violation)
- **Role:** fixture-owned schema for the kernel-level descriptor-load test scenario (anonymous $id)
- **Current owner:** kernel (top-level 🧫️fixtures/ grab-bag, not a per-contract mutation-owner directory)
- **Intended owner:** kernel/🧫️fixtures/📇️descriptor-load
- **Proposed named export(s):** `descriptorLoadFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📇️descriptor-load/🧬️schema/🔣️.json (rename the anonymous 🧬️.schema.json into a proper canonically-named directory beside its sibling 🔣️.json fixture data; no independent domain-owner directory exists elsewhere for "descriptor-load")
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📇️descriptor-load/🧬️.schema.json and sibling 🔣️.json — anonymous schema+data pair sitting directly inside the top-level kernel 🧫️fixtures/ grab-bag directory
  - repo-wide grep for 'descriptor-load' outside 🧫️fixtures/ found only 🟦️.ts and 🦀️.rs (source implementation references, not a separate schema-owning directory)
- **Open questions:**
  - Could not confirm the specific TS/Rust consumer of this exact schema file within the effort budget of this pass; the concept name appears in kernel's own 🟦️.ts and 🦀️.rs sources but a literal import of this schema path was not matched by the targeted greps.

### `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📐️source-watch.schema.json`

- **Classification:** b — fixture-owned contract (violation)
- **Role:** fixture-owned schema for the kernel-level source-watch test scenario (anonymous $id), sitting flat directly in 🧫️fixtures/ (not even its own subdirectory)
- **Current owner:** kernel (top-level 🧫️fixtures/ grab-bag)
- **Intended owner:** kernel/🧫️fixtures (or a new kernel/🧫️fixtures/📡️source-watch/ subdirectory)
- **Proposed named export(s):** `sourceWatchFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📡️source-watch/🧬️schema/🔣️.json (this is the most severe instance in kernel's fixtures grab-bag: not only non-canonically named but not even grouped in its own subdirectory the way descriptor-load/turn-patch-owner/app-router-plugin-faults are — recommend giving it a matching 📡️source-watch/ subdirectory holding both the 🔣️.json fixture and a 🧬️schema/ module)
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📐️source-watch.schema.json sits as a bare named file directly under 🧫️fixtures/, alongside its data sibling 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📡️source-watch.json — both flat, unlike the other three fixtures in this directory which each get their own named subdirectory
  - repo-wide grep for 'source-watch' outside 🧫️fixtures/ found only 🟦️.ts and 🦀️.rs implementation references
- **Open questions:**
  - Could not confirm the specific consumer within the effort budget of this pass.

### `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🚪️turn-patch-owner/🧬️.schema.json`

- **Classification:** b — fixture-owned contract (violation)
- **Role:** fixture-owned schema for the kernel-level turn-patch-owner test scenario (anonymous $id)
- **Current owner:** kernel (top-level 🧫️fixtures/ grab-bag)
- **Intended owner:** kernel/🧫️fixtures/🚪️turn-patch-owner
- **Proposed named export(s):** `turnPatchOwnerFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🚪️turn-patch-owner/🧬️schema/🔣️.json
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🚪️turn-patch-owner/🧬️.schema.json and sibling 🔣️.json — anonymous schema+data pair
  - repo-wide grep for 'turn-patch-owner' outside 🧫️fixtures/ found only 🟦️.ts and 🦀️.rs implementation references
- **Open questions:**
  - Could not confirm the specific consumer within the effort budget of this pass.

### `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🧬️.schema.json`

- **Classification:** b — fixture-owned contract (violation)
- **Role:** fixture-owned schema, title "AppRouter per-plugin fault isolation vectors" (no $id)
- **Current owner:** kernel (top-level 🧫️fixtures/ grab-bag)
- **Intended owner:** kernel/🧫️fixtures/🧫️app-router-plugin-faults
- **Proposed named export(s):** `appRouterPluginFaultsFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🧬️schema/🔣️.json
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🧬️.schema.json:2 — "title": "AppRouter per-plugin fault isolation vectors"
  - repo-wide grep for 'app-router-plugin-faults' outside 🧫️fixtures/ found only 🟦️.ts and 🦀️.rs implementation references
- **Open questions:**
  - Could not confirm the specific consumer within the effort budget of this pass.

### actor module

### `🧰️framework/🔨️modules/🎭️actor/🎠️activation/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** flat schema validating actor activation ordinal/registration trace fixtures (no $id)
- **Current owner:** actor (single crate, role=framework id=actor)
- **Intended owner:** actor/🎠️activation
- **Proposed named export(s):** `activationTraceSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🎠️activation/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level, straight conversion)
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🎠️activation/🧬️schema.json has no $id/title (verified via python3 json load)
  - directory listing: 🧰️framework/🔨️modules/🎭️actor/🎠️activation/ contains 🦀️.rs, 🧪️tests/, 🧫️fixture/🔣️.json, 🧬️schema.json — flat file sits directly beside its own 🧫️fixture data, validating trace/event test vectors
- **Open questions:**
  - No direct TS/Rust import of this specific schema path was located by the targeted greps in this pass (actor/🎠️activation has no 🟦️.ts of its own visible in the directory listing) — likely consumed via Rust test harness reading fixture+schema together, or via a higher-level TS module; recommend a follow-up grep of activation-related Rust tests for include_str! of this path.

### `🧰️framework/🔨️modules/🎭️actor/🏘️composition/🏗️bootstrap/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema, "Original Client Worker Metadata Preparation" ($id semio.actor.worker-metadata-preparation.v1); $ref's semio.value.resident.capacity.v1
- **Current owner:** actor
- **Intended owner:** actor/🏘️composition/🏗️bootstrap
- **Proposed named export(s):** `workerMetadataPreparationSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🏘️composition/🏗️bootstrap/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:2246` — typescript dynamic import, await import("../🏘️composition/🏗️bootstrap/🧬️schema.json")
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🏘️composition/🏗️bootstrap/🧬️schema.json:3-4 — $id semio.actor.worker-metadata-preparation.v1, title "Original Client Worker Metadata Preparation"
  - 🧰️framework/🔨️modules/🎭️actor/🏘️composition/🏗️bootstrap/🧬️schema.json:38 — $ref "semio.value.resident.capacity.v1" (cross-module dependency on value/resident)
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:2246 — const {default:schema}=await import("../🏘️composition/🏗️bootstrap/🧬️schema.json")

### `🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema, "Exact Actor Resident Composition Binding" ($id semio.actor.resident-composition.v1); $ref's semio.value.resident.capacity.v1
- **Current owner:** actor
- **Intended owner:** actor/🏘️composition
- **Proposed named export(s):** `residentCompositionSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:2500,2518` — typescript dynamic import, paired with ../🏘️composition/🧪️fixture/🔣️.json
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json:3-4 — $id semio.actor.resident-composition.v1, title "Exact Actor Resident Composition Binding"
  - 🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json:27 — $ref "semio.value.resident.capacity.v1"
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:2500-2501,2518-2519 — const {default:fixture}=await import("../🏘️composition/🧪️fixture/🔣️.json"); const {default:schema}=await import("../🏘️composition/🧬️schema.json")

### `🧰️framework/🔨️modules/🎭️actor/📃️page/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating actor byte-page's own 🧫️fixture test data ($id semio.actor.byte-page.fixture.v1)
- **Current owner:** actor/📃️page
- **Intended owner:** actor/📃️page
- **Proposed named export(s):** `bytePageFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📃️page/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real byte-page domain contract)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📃️page/🟦️.ts:61` — typescript dynamic import (fixtureSchema), await import("./📐️schema/🔣️.json")
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📃️page/📐️schema/🔣️.json:3 — "$id": "semio.actor.byte-page.fixture.v1"
  - 🧰️framework/🔨️modules/🎭️actor/📃️page/🟦️.ts:60-61 — const {default:schema}=await import("./🧬️schema.json"); const {default:fixtureSchema}=await import("./📐️schema/🔣️.json")
  - 🧰️framework/🔨️modules/🎭️actor/📃️page/📐️schema/🔣️.json:65,68 — $ref "semio.actor.byte-page.v1#/definitions/word" (references the real contract's $id)

### `🧰️framework/🔨️modules/🎭️actor/📃️page/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** canonical domain contract schema, "Actor Byte Page" ($id semio.actor.byte-page.v1) — widely $ref'd across kernel/actor/ui
- **Current owner:** actor
- **Intended owner:** actor/📃️page
- **Proposed named export(s):** `bytePageSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📃️page/🧬️schema/🔣️.json (flat file -> directory module). HIGH-FANOUT file, second only to value/resident's capacity schema in cross-module reach — preserve the exact $id string.
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📃️page/🟦️.ts:60` — typescript dynamic import
  - `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📐️fixture-schema/🔣️.json:6` — json-schema $ref (cross-module, kernel)
  - `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts:272` — typescript dynamic import (pageSchema), cross-module (kernel)
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema.json:71` — json-schema $ref
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:358,473,535` — typescript dynamic import (pageSchema), multiple call sites
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:365,387` — typescript dynamic import (page), multiple call sites
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧪️tests/🧬️schema/🔣️.json:1332` — json-schema $ref (cross-module, ui; outside slice, listed for completeness)
  - `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🧬️schema.json:9` — json-schema $ref (cross-module, ui; outside slice)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📃️page/🧬️schema.json:3-4 — $id semio.actor.byte-page.v1, title "Actor Byte Page"
  - confirmed 8+ distinct consumer call sites across actor, kernel, and ui (outside slice) via both $ref and dynamic import

### `🧰️framework/🔨️modules/🎭️actor/📤️return/🌿️framing/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema, "Canonical Return Result Framing Projection" ($id semio.actor.return-result.framing.v1); $ref's semio.actor.retained-return.v1
- **Current owner:** actor
- **Intended owner:** actor/📤️return/🌿️framing
- **Proposed named export(s):** `returnResultFramingSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/🌿️framing/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:532` — typescript dynamic import, await import("./🌿️framing/🧬️schema.json")
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/🌿️framing/🧬️schema.json:3 — "$id":"semio.actor.return-result.framing.v1"
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/🌿️framing/🧬️schema.json:9 — $ref "semio.actor.retained-return.v1#/definitions/pageReceipt"
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:532 — const {default:schema}=await import("./🌿️framing/🧬️schema.json")

### `🧰️framework/🔨️modules/🎭️actor/📤️return/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating actor return's own retained-return wire/result vectors ($id semio.actor.retained-return.fixture.v1)
- **Current owner:** actor/📤️return
- **Intended owner:** actor/📤️return
- **Proposed named export(s):** `retainedReturnFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real retained-return domain contract)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:356` — typescript dynamic import (fixtureSchema)
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:3098,3767` — typescript dynamic import, cross-file consumer within actor
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📐️schema/🔣️.json:3 — "$id": "semio.actor.retained-return.fixture.v1"
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:355-356 — const {default:schema}=await import("./🧬️schema.json"); const {default:fixtureSchema}=await import("./📐️schema/🔣️.json")
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📐️schema/🔣️.json:12,14,16-18 — multiple $ref's to semio.actor.retained-return.v1#/definitions/*

### `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🌿️framing/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema, "Canonical Return Response Framing Projection" ($id semio.actor.return-response.framing.v1); $ref's semio.actor.retained-return.v1
- **Current owner:** actor
- **Intended owner:** actor/📤️return/📨️response/🌿️framing
- **Proposed named export(s):** `returnResponseFramingSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🌿️framing/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:150` — typescript dynamic import, await import("./🌿️framing/🧬️schema.json")
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🌿️framing/🧬️schema.json:3 — "$id":"semio.actor.return-response.framing.v1"
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🌿️framing/🧬️schema.json:9 — $ref "semio.actor.retained-return.v1#/definitions/pageReceipt"
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:150 — const {default:schema}=await import("./🌿️framing/🧬️schema.json"); const {default:framing}=await import("./🌿️framing/🧪️fixture/🔣️.json")

### `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema, "Shared Worker Inbox Source Inventory" ($id semio.actor.return-response.inbox-inventory.v1)
- **Current owner:** actor
- **Intended owner:** actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox
- **Proposed named export(s):** `inboxInventorySchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:216` — typescript dynamic import, paired with ./🎟️credit/📋️metadata/📥️inbox/🧪️fixture/🔣️.json
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧬️schema.json:3-4 — $id semio.actor.return-response.inbox-inventory.v1, title "Shared Worker Inbox Source Inventory"
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:216 — const {default:schema}=await import("./🎟️credit/📋️metadata/📥️inbox/🧬️schema.json"); const {default:fixture}=await import("./🎟️credit/📋️metadata/📥️inbox/🧪️fixture/🔣️.json")

### `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema, "Captured Return Response Metadata Inventory" ($id semio.actor.return-response.metadata.v1); $ref's semio.value.resident.capacity.v1 three times
- **Current owner:** actor
- **Intended owner:** actor/📤️return/📨️response/🎟️credit/📋️metadata
- **Proposed named export(s):** `returnResponseMetadataSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:331` — typescript dynamic import, paired with ./🎟️credit/📋️metadata/🧪️fixture/🔣️.json
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/🧬️schema.json:3-4 — $id semio.actor.return-response.metadata.v1, title "Captured Return Response Metadata Inventory"
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/🧬️schema.json:80,94,108 — $ref "semio.value.resident.capacity.v1#/definitions/resources" (cross-module dependency on value/resident)
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:331-332 — schema import paired with import residentSchema from "../../../🌱️value/💾️resident/🧬️schema.json"

### `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema, "Return Response Credit Laws", validating the credit subsystem's own test data ($id semio.actor.return-response.credit.fixture.v1); references semio.actor.return-response.credit.v1 and semio.actor.return-response.v1
- **Current owner:** actor/📤️return/📨️response/🎟️credit
- **Intended owner:** actor/📤️return/📨️response/🎟️credit
- **Proposed named export(s):** `returnResponseCreditFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real credit domain contract)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:384` — typescript dynamic import (fixtureSchema)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📐️schema/🔣️.json:3-4 — $id semio.actor.return-response.credit.fixture.v1, title "Return Response Credit Laws"
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:382-384 — const {default:schema}=await import("./🎟️credit/🧬️schema.json"); const {default:fixtureSchema}=await import("./🎟️credit/📐️schema/🔣️.json")
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📐️schema/🔣️.json:8,13-15 — multiple $ref's to semio.actor.return-response.credit.v1#/definitions/*

### `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** canonical domain contract schema, "Captured Return Response Credit" ($id semio.actor.return-response.credit.v1); union type (oneOf/definitions, no top-level "type")
- **Current owner:** actor
- **Intended owner:** actor/📤️return/📨️response/🎟️credit
- **Proposed named export(s):** `returnResponseCreditSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level besides the non-canonical 📐️schema/, handled in its own finding)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:382` — typescript dynamic import
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📐️schema/🔣️.json:8,13-16` — json-schema $ref from its own fixture-schema sibling
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧬️schema.json:3-4 — $id semio.actor.return-response.credit.v1, title "Captured Return Response Credit"

### `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating actor return-response's own wire/header vectors ($id semio.actor.return-response.fixture.v1); references semio.actor.return-response.v1
- **Current owner:** actor/📤️return/📨️response
- **Intended owner:** actor/📤️return/📨️response
- **Proposed named export(s):** `returnResponseFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real return-response domain contract)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:363` — typescript dynamic import (fixtureSchema)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/📐️schema/🔣️.json:3 — "$id":"semio.actor.return-response.fixture.v1"
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:361-366 — schema/fixtureSchema/lifetime/page/returned all imported together for the response test block
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/📐️schema/🔣️.json:8,10 — $ref "semio.actor.return-response.v1" and its definitions/header

### `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** canonical domain contract schema, "Captured Worker Return Response" ($id semio.actor.return-response.v1); union type; $ref's semio.actor.retained-return.v1
- **Current owner:** actor
- **Intended owner:** actor/📤️return/📨️response
- **Proposed named export(s):** `returnResponseSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧬️schema/🔣️.json (flat file -> directory module)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:361,385` — typescript dynamic import, multiple call sites
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:355,471` — typescript dynamic import from the parent 📤️return dir (returned)
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📐️schema/🔣️.json:16` — json-schema const embedding its own $id as a wireLayout literal
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/📐️schema/🔣️.json:8,10` — json-schema $ref
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧬️schema.json:3-4 — $id semio.actor.return-response.v1, title "Captured Worker Return Response"

### `🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** canonical domain contract schema, "Canonical Retained Actor Return" ($id semio.actor.retained-return.v1) — widely $ref'd/imported across actor, kernel, and os/plugin
- **Current owner:** actor
- **Intended owner:** actor/📤️return
- **Proposed named export(s):** `retainedReturnSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema/🔣️.json (flat file -> directory module). HIGH-FANOUT file consumed by kernel and os/plugin cross-module — preserve the exact $id string.
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:355,471,533,475` — typescript dynamic import + Ajv.addSchema/getSchema, multiple call sites
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:152,366,388` — typescript dynamic import (returned/returnedSchema)
  - `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts:273 (imports actor/🚪️lifetime, related cluster)` — cross-module TS consumer of sibling actor schema files in the same cluster
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🟦️.ts:157` — typescript Ajv.getSchema("semio.actor.retained-return.v1#/definitions/result") — cross-product consumer (os/plugin)
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/🌿️framing/🧬️schema.json:9` — json-schema $ref
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📐️schema/🔣️.json:12,14,16-18` — json-schema $ref (own fixture-schema sibling)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema.json:3-4 — $id semio.actor.retained-return.v1, title "Canonical Retained Actor Return"

### `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema, "Shard Liveness Policy And Watchdog Timelines" ($id semio.actor.shard-liveness.v1)
- **Current owner:** actor
- **Intended owner:** actor/📮️shard-client
- **Proposed named export(s):** `shardLivenessSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:4665` — typescript dynamic import (self-referential, same file's own test block)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧬️schema.json:3-4 — $id semio.actor.shard-liveness.v1, title "Shard Liveness Policy And Watchdog Timelines"
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:4665 — const {default:schema}=await import("./🧬️schema.json")

### `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating actor instance-lifetime-close's own test data (no $id/title)
- **Current owner:** actor/🚪️lifetime
- **Intended owner:** actor/🚪️lifetime
- **Proposed named export(s):** `instanceLifetimeCloseFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real instance-lifetime-close domain contract)
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/📐️schema/🔣️.json has no $id/title (verified via python3 json load)
  - directory listing: 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/ contains 📐️schema/, 🚨️fault.fixture.json, 🟦️.ts, 🦀️.rs, 🧪️fixture/, 🧬️schema.json (flat), 🧯️fault.schema.json, 🩹️patch/ — a dense cluster of at least 3 distinct schema-shaped files
- **Open questions:**
  - No direct TS import of this specific 📐️schema/🔣️.json path was located by the targeted greps in this pass (unlike its flat 🧬️schema.json sibling, which IS consumed extensively — see companion finding); recommend the merged report re-check actor/🚪️lifetime/🟦️.ts and 🦀️.rs directly for this exact subpath.

### `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** canonical domain contract schema, union type ($id semio.actor.instance-lifetime-close.v1) — widely consumed cross-module by kernel and actor
- **Current owner:** actor
- **Intended owner:** actor/🚪️lifetime
- **Proposed named export(s):** `instanceLifetimeCloseSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧬️schema/🔣️.json (flat file -> directory module). HIGH-FANOUT file — preserve the exact $id string.
- **Consumers:**
  - `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts:273` — typescript dynamic import (lifetimeSchema), cross-module (kernel)
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:357,472,534` — typescript dynamic import (lifetimeSchema), multiple call sites
  - `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:152,364,386` — typescript dynamic import (lifetime), multiple call sites
  - `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:230,280` — typescript dynamic import (lifetimeSchema)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧬️schema.json:3 — "$id": "semio.actor.instance-lifetime-close.v1" (definitions/oneOf shape, no top-level type)

### `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧯️fault.schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** flat named schema validating the sibling 🚨️fault.fixture.json fault-trace fixture ($id semio.actor.instance-close.fault-fixture.v1)
- **Current owner:** actor/🚪️lifetime
- **Intended owner:** actor/🚪️lifetime
- **Proposed named export(s):** `instanceCloseFaultFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧫️fixture/🧬️schema/🔣️.json (this is a named flat file, not the anonymous domain contract; it validates the sibling 🚨️fault.fixture.json data file, so it belongs beside that fixture inside a canonically-named 🧬️schema/ module rather than as a bare named file at the lifetime root)
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧯️fault.schema.json:3 — "$id": "semio.actor.instance-close.fault-fixture.v1" (fixture-suffixed id, confirming fixture-shape role)
  - directory listing shows sibling 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🚨️fault.fixture.json — the data file this schema validates
- **Open questions:**
  - No direct TS/Rust consumer of this exact path was located by the targeted greps in this pass; recommend the merged report re-check actor/🚪️lifetime/🟦️.ts and 🦀️.rs for an import/include_str! of 🧯️fault.schema.json specifically.

### `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating actor lifetime/patch's own test data (no $id/title)
- **Current owner:** actor/🚪️lifetime/🩹️patch
- **Intended owner:** actor/🚪️lifetime/🩹️patch
- **Proposed named export(s):** `uiPatchReceiptFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real ui-patch-receipt domain contract)
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/📐️schema/🔣️.json has no $id/title
  - directory listing: 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/ contains 📐️schema/, 🟦️.ts, 🦀️.rs, 🧪️tests/, 🧫️fixture/, 🧬️schema.json (flat)
- **Open questions:**
  - No direct consumer located by the targeted greps in this pass — recommend the merged report re-check actor/🚪️lifetime/🩹️patch/🟦️.ts directly.

### `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema, ($id semio.actor.ui-patch-receipt.v1), consumed cross-module by kernel
- **Current owner:** actor
- **Intended owner:** actor/🚪️lifetime/🩹️patch
- **Proposed named export(s):** `uiPatchReceiptSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧬️schema/🔣️.json (flat file -> directory module)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts:274` — typescript dynamic import (patchSchema), cross-module (kernel)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧬️schema.json:3 — "$id": "semio.actor.ui-patch-receipt.v1"
  - 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts:274 — const {default:patchSchema}=await import("../../../🎭️actor/🚪️lifetime/🩹️patch/🧬️schema.json")

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema, "Actor Activation Lease Cases", validating the top-level activation directory's own 🧪️fixture data (no $id)
- **Current owner:** actor/🪪️activation
- **Intended owner:** actor/🪪️activation
- **Proposed named export(s):** `actorActivationLeaseCasesSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🧬️schema/🔣️.json (orphan case: no competing flat/dir 🧬️schema exists anywhere at this exact directory level — only 📐️schema/ and 🧪️fixture/ — so this is a straight rename, not a nesting relocation)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:4278` — typescript dynamic import, cross-file consumer within actor
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📐️schema/🔣️.json:2 — "title": "Actor Activation Lease Cases" (no $id)
  - directory listing: 🧰️framework/🔨️modules/🎭️actor/🪪️activation/ top level contains only 📐️schema/, 📤️return/, 📨️inbound/, 🚪️instance/, 🧪️fixture/ — no 🧬️schema (flat or dir) competing for the name
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:4278 — const {default:schema}=await import("../🪪️activation/📐️schema/🔣️.json")

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema, "Captured Return Admission Laws", validating the admission fixture ($id semio.actor.captured-return.admission.fixture.v1); mirrors sibling 🤝️contract.json field-for-field
- **Current owner:** actor/🪪️activation/📤️return/🏘️admission
- **Intended owner:** actor/🪪️activation/📤️return/🏘️admission
- **Proposed named export(s):** `capturedReturnAdmissionFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real captured-return-admission domain contract)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:2931-2932` — typescript dynamic import (fixtureSchema), paired with 🤝️contract.json, 🧬️schema.json and 🧪️fixture/🔣️.json
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/📐️schema/🔣️.json:3-4 — $id semio.actor.captured-return.admission.fixture.v1, title "Captured Return Admission Laws"
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:2931-2932 — imports contract.json + 🧬️schema.json + 🧪️fixture/🔣️.json + 📐️schema/🔣️.json (fixtureSchema) all together for one test block
  - content diff shows 📐️schema/🔣️.json field-for-field mirrors 🤝️contract.json/🧪️fixture/🔣️.json's shape as "const" assertions — a genuine fixture-shape validator, not an independent domain concept

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** canonical domain contract schema, "Captured Return Parent Admission" ($id semio.actor.captured-return.admission.v1)
- **Current owner:** actor
- **Intended owner:** actor/🪪️activation/📤️return/🏘️admission
- **Proposed named export(s):** `capturedReturnAdmissionSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️schema/🔣️.json (flat file -> directory module)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:2931` — typescript dynamic import, cross-file consumer within actor
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️schema.json:3-4 — $id semio.actor.captured-return.admission.v1, title "Captured Return Parent Admission"

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema, "Captured Actor Return Authority", validating this leaf's own 🧪️fixture data (no $id)
- **Current owner:** actor/🪪️activation/📤️return
- **Intended owner:** actor/🪪️activation/📤️return
- **Proposed named export(s):** `capturedActorReturnAuthoritySchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🧬️schema/🔣️.json (orphan case: only 🏘️admission/ subdir, 📐️schema/, and 🧪️fixture/ exist at this level — no competing 🧬️schema, straight rename)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:3767` — typescript dynamic import, cross-file consumer within actor
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/📐️schema/🔣️.json:2 — "title": "Captured Actor Return Authority" (no $id)
  - directory listing: 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/ contains 🏘️admission/, 📐️schema/, 🧪️fixture/ only
  - 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:3767 — const {default:schema}=await import("../🪪️activation/📤️return/📐️schema/🔣️.json")

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📨️inbound/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema, "Actor Inbound Activation Authority", validating this leaf's own 🧪️fixture data (no $id)
- **Current owner:** actor/🪪️activation/📨️inbound
- **Intended owner:** actor/🪪️activation/📨️inbound
- **Proposed named export(s):** `actorInboundActivationAuthoritySchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📨️inbound/🧬️schema/🔣️.json (orphan case: only 📐️schema/ and 🧪️fixture/ exist at this level — no competing 🧬️schema, straight rename)
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📨️inbound/📐️schema/🔣️.json:2 — "title": "Actor Inbound Activation Authority" (no $id)
  - directory listing: 🧰️framework/🔨️modules/🎭️actor/🪪️activation/📨️inbound/ contains 📐️schema/, 🧪️fixture/ only
- **Open questions:**
  - No direct consumer located by the targeted greps in this pass — recommend the merged report re-check actor/📮️shard-client/🟦️.ts and actor/🪪️activation/*/🟦️.ts more broadly for this exact subpath.

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating this leaf's own 🧪️fixture data (no $id/title)
- **Current owner:** actor/🪪️activation/🚪️instance
- **Intended owner:** actor/🪪️activation/🚪️instance
- **Proposed named export(s):** `actorInstanceFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/🧬️schema/🔣️.json (orphan case: only 📐️schema/, 📥️output/, and 🧪️fixture/ exist at this level — no competing 🧬️schema, straight rename)
- **Consumers:** none located by the targeted greps run in this pass (see Open questions)
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📐️schema/🔣️.json has no $id/title
  - directory listing: 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/ contains 📐️schema/, 📥️output/, 🧪️fixture/ only
- **Open questions:**
  - No direct consumer located by the targeted greps in this pass — recommend the merged report re-check actor/📮️shard-client/🟦️.ts and actor/🪪️activation/🚪️instance/🟦️.ts for this exact subpath.

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/📐️schema/🔣️.json`

- **Classification:** c — non-canonical schema directory name (violation)
- **Role:** fixture-shape schema validating this leaf's own admission fixture ($id https://semio.tech/schema/actor/output/admission-fixture.v1); mirrors sibling 🤝️contract.json
- **Current owner:** actor/🪪️activation/🚪️instance/📥️output/🏘️admission
- **Intended owner:** actor/🪪️activation/🚪️instance/📥️output/🏘️admission
- **Proposed named export(s):** `outputAdmissionFixtureSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧫️fixture/🧬️schema/🔣️.json (competing name: sibling flat 🧬️schema.json already claims 🧬️schema for the real domain admission contract)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:171` — typescript dynamic import (fixtureSchema), paired with 🤝️contract.json, 🧬️schema.json, and 🧪️fixture/🔣️.json
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/📐️schema/🔣️.json:3 — "$id": "https://semio.tech/schema/actor/output/admission-fixture.v1"
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:171 — imports 🤝️contract.json + 🧬️schema.json + 🧪️fixture/🔣️.json + 📐️schema/🔣️.json (fixtureSchema) together, validated with Ajv + immer produce()

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** canonical domain contract schema ($id https://semio.tech/schema/actor/output/admission.v1)
- **Current owner:** actor
- **Intended owner:** actor/🪪️activation/🚪️instance/📥️output/🏘️admission
- **Proposed named export(s):** `outputAdmissionSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️schema/🔣️.json (flat file -> directory module)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:171` — typescript dynamic import, same call site as its fixture-schema sibling
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️schema.json:3 — "$id": "https://semio.tech/schema/actor/output/admission.v1"

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema, "Captured Actor Turn Output Reservation" (no $id; allOf composition); has a 🟦️.ts sibling co-located
- **Current owner:** actor
- **Intended owner:** actor/🪪️activation/🚪️instance/📥️output
- **Proposed named export(s):** `capturedActorTurnOutputReservationSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:229,279` — typescript dynamic import (self-directory), paired with lifetimeSchema import from ../../../🚪️lifetime/🧬️schema.json
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧬️schema.json:2 — "title": "Captured Actor Turn Output Reservation" (no $id present)
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:229,279 — const {default:schema}=await import("./🧬️schema.json")
- **Open questions:**
  - This file has no $id, unlike almost every other domain-contract schema in the slice — flagging as a consistency gap worth fixing alongside the location move (a schema this widely composed via allOf should carry an $id for $ref-ability).

### `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧯️fault/🧬️schema.json`

- **Classification:** d — flat-file schema outside a module (violation)
- **Role:** domain contract schema for actor output owned-fault ($id actor-output-owned-fault.v1)
- **Current owner:** actor
- **Intended owner:** actor/🪪️activation/🚪️instance/📥️output/🧯️fault
- **Proposed named export(s):** `actorOutputOwnedFaultSchema`
- **Decision:** move-to 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧯️fault/🧬️schema/🔣️.json (flat file -> directory module; no competing 🧬️schema/ dir at this level)
- **Consumers:**
  - `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:205` — typescript dynamic import, paired with ./🧯️fault/🧪️fixture/🔣️.json
- **Evidence:**
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧯️fault/🧬️schema.json:3 — "$id": "actor-output-owned-fault.v1"
  - 🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts:205 — const {default:fixture}=await import("./🧯️fault/🧪️fixture/🔣️.json"); const {default:schema}=await import("./🧯️fault/🧬️schema.json")

## Correct-as-is (classification a) and config (classification f)

Listed for completeness; no action needed.

- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧺️set/🧬️schema/🔣️.schema.json` (a) — canonical domain contract schema for OrderedSet ($id semio.value.ordered-set.v1)
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/🏠️source/📚️entries/🧬️schema/🔣️.json` (a) — canonical fixture/trace schema for kernel return-source entries ($id semio.kernel.return-source-entries.fixture.v1) — sole schema at this leaf
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/🏠️source/🧬️schema/🔣️.json` (a) — canonical fixture/trace schema for kernel return-source ($id semio.kernel.return-source.fixture.v1) — sole schema at this leaf
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/💌️message/🧬️schema/🔣️.json` (a) — canonical fixture schema for kernel return-content message framing ($id semio.kernel.return-message.fixture.v1) — sole schema at this leaf
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧬️schema/🔣️.json` (a) — canonical domain contract schema for kernel return/content/input builder binding lifecycle ($id kernel-return-builder-binding.v1)
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️schema/🔣️.json` (a) — canonical domain contract schema for original return-field resident-payload association ($id semio.kernel.return.input.resident-payload.v1, title "Original Return Field Resident Payload Association")
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧬️schema/🔣️.json` (a) — canonical fixture/trace schema for kernel return/content/input overall ($id semio.kernel.return-input.fixture.v1) — sole schema at this leaf
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧬️schema/🔣️.json` (a) — canonical domain contract schema for kernel return/content/input release ($id semio.kernel.return-input.release.contract.v1)
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧬️schema/🔣️.json` (a) — canonical fixture schema for kernel return/content/input authority ($id semio.kernel.return-input.authority-fixture.v1) — sole schema at this leaf
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧬️schema/🔣️.json` (a) — canonical domain contract schema, "Canonical Kernel Return Content Declaration" ($id semio.kernel.return-content-declaration.v1)
- `🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧬️schema/🔣️.json` (a) — canonical domain contract schema, "Required Poll Composition Projection" ($id semio.kernel.poll.composition.v1); $ref's semio.value.resident.capacity.v1
- `🧰️framework/🔨️modules/🎠️kernel/🔮️oracle/🔣️.json` (f) — kernel module's third-party test-oracle registry entry (semver package), referencing the repo test platform's own schema via a relative $schema pointer for editor tooling
- `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧬️schema/🔣️.json` (a) — canonical domain contract schema, "Actor Cold Document Pair Ingress Status" ($id semio.actor.cold-pair-ingress-status.v1); draft 2020-12 dialect

### Slice C — remaining `🧰️framework/🔨️modules/*` — detailed findings

## Detailed findings — violations and duplicate authorities

### 1. `⏱️trace/⏱️clock/🧬️contention/🔣️.schema.json` — class (c)

Sibling directories under `⏱️clock/`: `🧪️contention/` (the real feature dir — `🔣️.json` fixture data
+ `🦀️.rs` test) and `🧬️contention/` (holds only `🔣️.schema.json`). The schema dir is named after the
**feature** ("contention") with a `🧬️` prefix, instead of the canonical literal name `🧬️schema`.

- Consumer: `📜️script.ts` (repo root) `toolJobTelemetryContentionSelfTests()`, ~line 1969-1986 —
  `readFileSync(join(base, "🧪️contention/🔣️.json"))` for the fixture and
  `readFileSync(join(base, "🧬️contention/🔣️.schema.json"))` for the schema, `base = ⏱️trace/⏱️clock`.
- The exact same function block in that file validates two OTHER, correctly-named pairs immediately
  above/below it: `⏳️async/🤝️cooperative` (`🧪️fixture/` + `🧬️schema/🔣️.json`) and
  `🧵️job/⏱️budget` (`🧫️fixture/` + `🧬️schema/🪫️budget.json`) — only the trace/contention instance
  deviates.
- Prior-audit corroboration: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️goal-vocab2-census.md:15` already flagged this exact path as `⏱️clock/🧬️contention.schema.json — bare stem "contention"`.
- **Recommendation:** rename `🧬️contention/` → `🧬️schema/`, keep the file as `🔣️.json` (drop the
  redundant `.schema` infix to match every other instance in the module), update the one root
  `📜️script.ts` reference.

### 2. `📡️replication/🎮️mutation/🧪️tests/{🤝️mutation-leaf-contract,🧭️mutation-leaf-source-contract}/🛂️schema/🔣️.json` — class (c) × 2

Both use the legacy `🛂️schema/` name explicitly called out in the task brief. Neither has an active
consumer (`🔗️causal.rs:961/972` only reads the sibling `🧫️fixtures/🔣️.json` fixture data, never the
schema) — these look documentary/orphaned in addition to being misnamed.
- **Recommendation:** rename both to `🧬️schema/`; separately verify whether they should be wired
  into an actual Ajv/validator consumer, since neither is currently active.

### 3. `📡️replication/📡️wire/🏠️local-interaction/{🌳️root,🌳️root/🩹️update}/📐️schema/🔣️.json` — class (c) × 2

Both use the legacy `📐️schema/` name. Both ARE actively consumed by the module's master validator
`🧫️fixtures/📜️script.ts` (lines ~79-80 and ~104-105 respectively — `Bun.file().json()` +
`ajv.compile(...)`), alongside their sibling `🧫️fixture/🔣️.json` data.
- **Recommendation:** rename both to `🧬️schema/`, update the two references in `🧫️fixtures/📜️script.ts`.

### 4. `📡️replication/📡️wire/🏠️local-interaction/📡️transport/{📐️schema,🧬️schema}/🔣️.json` — duplicate-authority pair

**This is the clearest duplicate-authority instance in the slice.** `📡️transport/` has TWO schema-named
sibling directories:
- `🧬️schema/🔣️.json` — the REAL wire-protocol contract (`$id: "semio:local-interaction-query-transport"`,
  `oneOf` command/reply, `$ref`s into the canonical local-interaction schema). Classification (a), keep.
- `📐️schema/🔣️.json` — a DIFFERENT, narrower schema that validates the shape of the transport
  **test-fixture corpus** (`🧫️fixtures/🔣️.json`: `version`/`appCommandTag`/`unsigned`/`commandKinds`/...).
  Classification (c).

Both are actively consumed by the master validator `🧫️fixtures/📜️script.ts:180-181`
(`../📡️transport/🧫️fixtures/🔣️.json` validated against `../📡️transport/📐️schema/🔣️.json`, then the
protocol itself compiled from `../📡️transport/🧬️schema/🔣️.json`). They are not literally duplicate
content — but the directory-naming collision (two "…schema…" dirs, same parent) is exactly the
pattern flagged in the task brief.
- **Recommendation:** move the fixture-shape schema into `📡️transport/🧫️fixtures/🧬️.schema.json`
  (the same pattern already used for `♻️retirement`/`📃️query`/`🔐️topology-authority`/`🏠️local-interaction`
  fixtures in the sibling `🧫️fixtures/` dir one level up — classification (b), but at least it stops
  colliding with the real `🧬️schema/`), leaving `🧬️schema/` as the sole schema-named directory under
  `📡️transport/`.

### 5. `📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/{♻️retirement,🏠️local-interaction,📃️query,🔐️topology-authority}/🧬️.schema.json` — class (b) × 4

All four sit directly beside their fixture data (`🔣️.json`) inside `🧫️fixtures/<name>/`. All four ARE
actively used — the entire local-interaction cluster is validated by ONE master script:
`📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/📜️script.ts` (regions `♻️RetirementContract`
line ~139, `📃️QueryContract` line ~152, `🔐️TopologyInputAuthority` line ~166, and the top-of-file
`🧬️Contract` region line ~11 for `🏠️local-interaction`).
- **Recommendation:** relocate each into `📡️wire/🏠️local-interaction/🧬️schema/<name>/🔣️.json` (or as
  named files under the module's existing `🧬️schema/` dir) and update the 4 references in that one
  script.ts.

### 6. `📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🏠️local-interaction/🔣️.schema.json` — flagged nested oddity, class (a) but misplaced

This IS correctly inside `🧬️schema/` (unlike the other findings above) but then nests a REDUNDANT
`🏠️local-interaction/` directory one level deeper, repeating the owning module's own name for no
scoping reason (`$id` is `"semio:local-interaction"` — the whole-module identity, not a sub-scope).
No other `🧬️schema/` instance in this slice repeats its owner's name as an inner directory (contrast
`🕹️interaction/🧬️schema/🔣️.json`, not `🕹️interaction/🧬️schema/🕹️interaction/🔣️.json`).
- Consumer: `🧫️fixtures/📜️script.ts:10` — `Bun.file(new URL("../🧬️schema/🏠️local-interaction/🔣️.schema.json", ...))`, loaded via `ajv.addSchema()` for cross-schema `$ref` resolution (the transport schema `$ref`s into it as `semio:local-interaction#/$defs/...`).
- **Recommendation:** flatten to `🧬️schema/🔣️.json`, update the one reference. Verify no generic
  taxonomy-driven policy gate assumes the nested shape before moving (open question below).

### 7. `📡️replication/🔗️causal/🧪️fixtures/🧬️mutations/➕️causal-add/🛂️schema/🔣️.json` — class (c), but test/mock scaffolding

The entire owning path is inside `🧪️fixtures/` (`.../🔗️causal/🧪️fixtures/🧬️mutations/➕️causal-add`) and
is used only as example/test data exercising the generic `Mutation<T>` trait machinery in
`🔗️causal.rs` (`CAUSAL_ADD_DESCRIPTOR`, lines 978-993). It is not a real, discoverable production
mutation-owner directory. It happens to use the same non-canonical `🛂️schema` name flagged in finding
#2, and the mocked descriptor's `payloadSchema` field literally says `"🛂️schema/🔣️.json"`
(`🧪️descriptor/🔣️.json:9`), matching taxonomy's `mutationPayloadSchemaAuthority.descriptorField`.
- **Recommendation:** low-priority cosmetic fix (rename to `🧬️schema` for consistency) — flagged as
  `follow-up-needed` rather than a hard `move-to` because it is test scaffolding, not a live contract.

### 8. Flat schema files with no wrapping `🧬️schema/` — class (d), 7 instances

- `📡️replication/🧫️fixtures/🚀️artifact-bootstrap/🧬️.schema.json` — worst case: flat AND inside a
  `🧫️fixtures/` subtree (patterns (b)+(d) combined). No confirmed active Ajv consumer found; only the
  paired data file (`🔣️.json`) is read by Rust/TS (3 confirmed consumers, one cross-product in
  `💻️os`).
- `🕸️graph/🛂️manifest/🧬️outputs.schema.json` — flat in `🛂️manifest/`, describes a build-manifest
  catalog (not an application/mutation contract). The module's own `📜️script.ts` hand-validates the
  paired `📇️outputs.json` in TypeScript rather than loading/Ajv-compiling this schema — it currently
  has no active code consumer. Marked `follow-up-needed`: unclear whether this build-tooling class of
  schema is even meant to follow the `🧬️schema/`-module convention, or whether `🛂️manifest` is a
  taxonomy-exempt directory kind.
- `🖼️assets/{🌱️metabolism/🎨️representation,🔍️resolver,🔤️fonts,🥽️mesh}/🧬️{catalog,delivery}.schema.json`
  — all 4: the entire `🖼️assets` module has **no `🧬️schema/` directory anywhere** — every schema in it
  is a flat sibling of its data file. All 4 are actively consumed (confirmed via `readFileSync`/`import`
  in the module's own `.ts`/`.rs` implementation files); two of them (`🔤️fonts` and `🥽️mesh` catalogs)
  are additionally consumed **cross-module** by `🖱️ui/🎨️styling` and `🖱️ui/🧬️contract` (outside this
  slice — `🖱️ui/🧬️contract/📦️packages/🦀️rust/🔬️conformance.rs:280` even asserts the exact directory
  listing includes the literal filename `"🧬️catalog.schema.json"`, so relocating it requires a
  coordinated change with whoever owns the `🖱️ui` slice).
- `🗺️surface/🧪️tests/🧬️bindings.schema.json` — flat in `🧪️tests/`, validates wasm-bindgen output
  filenames; consumer confirmed at `🗺️surface/🧪️tests/🟦️.ts:6-7`.

### 9. Fixture-owned schemas (class b) — remaining instances not covered above

- `⏳️async/{🔐️use,🔔️deferred-wake,🔔️maintenance}/🧪️fixtures/🧬️.schema.json` — no active consumer
  found for any of the three schema files (only the paired fixture DATA is read, via
  `include_str!("🧪️fixtures/🔣️.json")` in the sibling `.rs` files); possibly dead weight.
- `🎯️action-bus/🧹️wire-retirement/🧪️fixture/🧬️.schema.json` — actively used by the module's own
  `📜️script.ts:10-11` (`Ajv.compile`), unlike the async instances above.
- `🛂️manifest/🧪️fixtures/{🎛️tutorial-local-interaction,📜️action-semantics,🛤️tutorial-document-track}.schema.json`
  — no consumer found for any of the three (only paired fixture data is `include_str!`'d by
  `🛂️manifest/🦀️.rs`).
- `🛂️manifest/🧪️fixtures/🗄️artifact-kind-formats.schema.json` — the one manifest-fixtures instance
  with a CONFIRMED active consumer, and it is cross-product: `🌎️hub/📦️packages/🦀️rust/📜️script.ts:8559`
  (`Ajv2020.compile`).
- `🧵️job/🧪️fixtures/{📡️shared-framework-action-routes,🧬️fixed-operation-registry}.schema.json` — both
  actively consumed by the root `📜️script.ts` policy-gate functions (`toolJobSharedFrameworkActionFixtureRun`
  and related, lines ~9206-9207 and ~9342-9343).

### Slice D — products / mit-bestand / root tests — detail

## Detail: (b) fixture-owned contract violations — 10 files

A schema-shaped file sitting inside (or directly named as) a `🧫️fixtures/`/`🧫️fixture/` directory, acting as the authoritative definition for the contract it validates, instead of that fixture directory containing only examples.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/bun-dependencies/🧬️schema.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** repo/library/caching test suite (bun-dependencies fixture)
- **Intended owner:** repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/)
- **Should export:** JSON Schema draft-07/2020-12 — bun-dependencies fixture caching contract (bun lock/patches/mutations/importers)
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts (typescript test runner (validates the sibling fixture data against this schema))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/bun-dependencies/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it)
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/bun-dependencies/🧬️schema.json is a schema-shaped JSON Schema file living directly inside a 🧫️fixtures/ directory — canonical fixture-owned-contract violation (b): the file defines/enforces the bun-dependencies contract from inside what the taxonomy designates as an examples-only fixtures tree.
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts references 🧫️fixtures/<name>/... paths throughout its test functions (readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/..."))) to load and validate each fixture bundle.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/command-boundaries/🧬️schema.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** repo/library/caching test suite (command-boundaries fixture)
- **Intended owner:** repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/)
- **Should export:** JSON Schema draft-07/2020-12 — command-boundaries fixture caching contract (per-entry esbuild import boundary cases)
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts (typescript test runner (validates the sibling fixture data against this schema))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/command-boundaries/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it)
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/command-boundaries/🧬️schema.json is a schema-shaped JSON Schema file living directly inside a 🧫️fixtures/ directory — canonical fixture-owned-contract violation (b): the file defines/enforces the command-boundaries contract from inside what the taxonomy designates as an examples-only fixtures tree.
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts references 🧫️fixtures/<name>/... paths throughout its test functions (readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/..."))) to load and validate each fixture bundle.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/native-inputs/🧬️schema.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** repo/library/caching test suite (native-inputs fixture)
- **Intended owner:** repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/)
- **Should export:** JSON Schema draft-07/2020-12 — native-inputs fixture caching contract (native file/include/exclude sets)
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts (typescript test runner (validates the sibling fixture data against this schema))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/native-inputs/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it)
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/native-inputs/🧬️schema.json is a schema-shaped JSON Schema file living directly inside a 🧫️fixtures/ directory — canonical fixture-owned-contract violation (b): the file defines/enforces the native-inputs contract from inside what the taxonomy designates as an examples-only fixtures tree.
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts references 🧫️fixtures/<name>/... paths throughout its test functions (readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/..."))) to load and validate each fixture bundle.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/native-preparation/🧬️schema.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** repo/library/caching test suite (native-preparation fixture)
- **Intended owner:** repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/)
- **Should export:** JSON Schema draft-07/2020-12 — native-preparation fixture caching contract (native preparation contracts/cases/targets)
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts (typescript test runner (validates the sibling fixture data against this schema))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/native-preparation/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it)
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/native-preparation/🧬️schema.json is a schema-shaped JSON Schema file living directly inside a 🧫️fixtures/ directory — canonical fixture-owned-contract violation (b): the file defines/enforces the native-preparation contract from inside what the taxonomy designates as an examples-only fixtures tree.
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts references 🧫️fixtures/<name>/... paths throughout its test functions (readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/..."))) to load and validate each fixture bundle.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/runtime-components/🧬️schema.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** repo/library/caching test suite (runtime-components fixture)
- **Intended owner:** repo/library/caching module's own 🧬️schema/ (or a per-test 🧬️schema/ next to caching's 🧪️tests/, not inside 🧫️fixtures/)
- **Should export:** JSON Schema draft-07/2020-12 — runtime-components fixture caching contract (plugin runtime component closures)
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts (typescript test runner (validates the sibling fixture data against this schema))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/runtime-components/🔣️.json (or an equivalent 🧬️schema/ sibling of 🧫️fixtures/, not nested inside it)
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/runtime-components/🧬️schema.json is a schema-shaped JSON Schema file living directly inside a 🧫️fixtures/ directory — canonical fixture-owned-contract violation (b): the file defines/enforces the runtime-components contract from inside what the taxonomy designates as an examples-only fixtures tree.
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts references 🧫️fixtures/<name>/... paths throughout its test functions (readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/..."))) to load and validate each fixture bundle.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/nx-contract/🧬️.schema.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** repo/library/caching test suite (nx-contract fixture)
- **Intended owner:** repo/library/caching module's own 🧬️schema/
- **Should export:** JSON Schema draft-07 for the Nx contract fixture vectors (version/policies/wasm/go/rust/lifecycle/generators/... 18.5KB, largest of the caching fixture schemas)
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts:349 (typescript (assert.equal(validate(vectors, JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/nx-contract/🧬️.schema.json")...))).valid, true)))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/nx-contract/🔣️.json
- **Evidence:**
  - Anonymous dot-prefixed 🧬️.schema.json (18512 bytes, largest caching fixture schema) sitting directly inside 🧫️fixtures/nx-contract/ — fixture-owned-contract violation (b).
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts:349 — `assert.equal(validate(vectors, JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/nx-contract/🧬️.schema.json"), "utf8"))).valid, true);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📦️extension-installation-owner/🧬️.schema.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** repo/library TypeScript package test suite (extension-installation-owner fixture)
- **Intended owner:** repo/library/📦️packages/🟦️typescript module's own 🧬️schema/
- **Should export:** JSON Schema draft-07 for the extension-installation-owner cargo-manifest fixture (schema const "semio.extension.cargo-installation-owner/v1", cases[])
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts:52 (typescript-test (new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(folder, "🧬️.schema.json"))))))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧬️schema/extension-installation-owner/🔣️.json
- **Evidence:**
  - Anonymous 🧬️.schema.json sitting inside 🧫️fixtures/📦️extension-installation-owner/ — fixture-owned-contract violation (b).
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts:52 loads it generically via a `folder` loop variable that iterates 🧫️fixtures/* subdirectories looking for a 🧬️.schema.json sibling — the same generic-loader pattern seen for the library's own root 🧬️.schema.json (see that finding); this loader is itself what normalizes the anonymous-dot-schema-in-fixtures convention across this package.
- **Open questions:**
  - The same index.test.ts:52 loader also picks up 🧫️fixtures/nx-contract/🧬️.schema.json and possibly other fixtures/*/🧬️.schema.json siblings under this typescript package tree — a full remediation should audit every 🧫️fixtures/*/🧬️.schema.json this loader walks, not just the two caught by this slice's candidate list.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🧬️join-provenance/🔣️.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** repo/library TypeScript package test suite (rust-physical-reference-context / join-provenance fixture)
- **Intended owner:** repo/library/📦️packages/🟦️typescript module's own 🧬️schema/
- **Should export:** JSON Schema draft-07 "Rust String Collection Join Provenance" (contract "rust-standard-string-collection-join-v1", ownership/retention/cases)
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts (typescript-test (loads 🧲️rust-physical-reference-context fixtures generically))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧬️schema/join-provenance/🔣️.json
- **Evidence:**
  - This is a schema-shaped file (has "$schema", "title", "required", "properties", defines contract "rust-standard-string-collection-join-v1") using the generic anonymous 🔣️.json filename, sitting inside 🧫️fixtures/🧲️rust-physical-reference-context/🧬️join-provenance/ — a nested subdirectory whose own name (🧬️join-provenance) mimics a mutation/contract-leaf name but is not the canonical "🧬️schema" directory name, AND the whole thing lives under 🧫️fixtures/ — combined violation of both (b) fixture-owned-contract and (c) non-canonical directory naming.
- **Open questions:**
  - Could not find a line-level grep hit tying this specific file to a named consumer function (only the general 🔬️index.test.ts test harness for this fixtures tree); a closer read of 🔬️index.test.ts's rust-physical-reference-context test block would pin the exact call site.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧫️fixture/🧬️fixture.schema.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** test-case scope: 🪟️windows-checkout-paths
- **Intended owner:** test-case scope: 🪟️windows-checkout-paths (own 🧬️schema/, not inside 🧫️fixture/)
- **Should export:** JSON Schema draft-07 for windows-checkout-paths cases (version/checkout.root/maxPathUnits/ticketRoot/components)
- **Consumers:** sibling test runner in 🪟️windows-checkout-paths/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧬️schema/🔣️.json
- **Evidence:**
  - 🧬️fixture.schema.json sits literally inside a 🧫️fixture/ directory (singular, one of the fixture-dir spellings the task brief calls out) — textbook fixture-owned-contract violation (b), compounded by the non-canonical "fixture.schema.json" filename.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧫️fixtures/🧬️g3-event-schema.json`

- **Classification:** (b) fixture-owned contract (violation)
- **Owner (current):** repo/server module / 🎛️coordinator (Go event store)
- **Intended owner:** repo/server module's own 🧬️schema/ (e.g. 🧬️schema/🎛️coordinator/)
- **Should export:** Compact wire-format descriptor for the coordinator's canonical event-log encoding (schema "semio.coordinator.event/1": encoding/scalar/fields/checksum) — NOT a JSON-Schema-draft-07 document, a bespoke one-line manifest
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️g3_event_store_test.go:103 (go-test (os.ReadFile(filepath.Join("🧫️fixtures", "🧬️g3-event-schema.json"))))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🧬️schema/🎛️coordinator/🔣️.json (recast as a real JSON Schema draft-07 document if it is meant to validate the 📜️g3-event-log.jsonl fixture, or otherwise as clearly-labeled encoding-contract metadata)
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧫️fixtures/🧬️g3-event-schema.json is the ONLY place in the repo that defines the coordinator event log's wire format ("semio.coordinator.event/1": stream/sequence/id/generation/type/payload/checksum fields and the checksum formula) — sitting inside a 🧫️fixtures/ directory alongside the actual fixture data it governs (📜️g3-event-log.jsonl) — this is the fixture-embedded-schema violation (b) explicitly named in the task brief.
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️g3_event_store_test.go:103-124 reads both 🧬️g3-event-schema.json and 📜️g3-event-log.jsonl from 🧫️fixtures/ as the sole consumer/enforcer.
  - The 🖥️server module already has its own correctly-named 🧬️schema/ directory (🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🧬️schema/🐘️postgres/) for the Postgres persistence schema — a different contract (DB schema, not the event wire format), so this is NOT a duplicate-authority case with that directory, but the coordinator's event contract should live in an equivalent scope-owned location rather than inside 🧫️fixtures/.
- **Open questions:**
  - This descriptor is not itself JSON-Schema-draft-07-shaped (no "$schema"/"type"/"properties") — it is a compact bespoke manifest. Moving it to 🧬️schema/ is still correct per the fixture-ownership rule, but whoever owns the coordinator should decide whether to also formalize it as real JSON Schema at that point.

## Detail: (c) non-canonical schema directory name violations — 3 files

A per-contract schema directory using `🛂️schema/` instead of the taxonomy-sanctioned `🧬️schema/` directory name (`mutationPayloadSchemaLocation.directoryName`). All three sit inside `📚️library/🧪️tests/` and all three are otherwise correctly structured (directory + `🔣️.json`) — only the directory name is wrong, making these the cheapest fixes in the whole slice (pure rename, no consumer-path surgery beyond the rename itself).

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🛂️schema/🔣️.json`

- **Classification:** (c) non-canonical schema directory name (violation)
- **Owner (current):** test-case scope: 📡️mutation-reachability
- **Intended owner:** test-case scope: 📡️mutation-reachability (rename directory 🛂️schema/ to 🧬️schema/)
- **Should export:** JSON Schema draft-07 (compact, single-line) for mutation-reachability cases (schemaVersion/cases[].name/source/accepted)
- **Consumers:** sibling test runner in 📡️mutation-reachability/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🧬️schema/🔣️.json
- **Evidence:**
  - Same pure (c) non-canonical-directory-name violation as 📋️mutation-inventory/🛂️schema/ and 🪪️mutation-metadata/🛂️schema/ — three instances of the identical 🛂️schema-instead-of-🧬️schema drift across the same 📚️library/🧪️tests/ tree.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🛂️schema/🔣️.json`

- **Classification:** (c) non-canonical schema directory name (violation)
- **Owner (current):** test-case scope: 📋️mutation-inventory (root contract)
- **Intended owner:** test-case scope: 📋️mutation-inventory (rename directory 🛂️schema/ to 🧬️schema/)
- **Should export:** JSON Schema draft-07 for the root mutation-inventory record (schemaVersion 2, kind "mutation", sourceTreeDigest/roots/sourceRoster/records/unresolved/violations)
- **Consumers:** sibling test runner in 📋️mutation-inventory/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧬️schema/🔣️.json
- **Evidence:**
  - 📋️mutation-inventory/🛂️schema/🔣️.json uses a correctly-structured directory+🔣️.json layout but the directory is named 🛂️schema instead of the taxonomy-sanctioned 🧬️schema — this is the pure (c) violation pattern called out explicitly in the task brief.
  - This is a SIBLING of 4 correctly-named 🧬️schema/ subdirectories in the same 📋️mutation-inventory/ test tree (🎫️ticket-role-routing/🧬️schema, 🎭️source-roster-roles/🧬️schema, 📸️source-index-capture/🧬️schema, 🧾️source-file-facts/🧬️schema) — i.e. this one file breaks an otherwise-consistent naming convention within its own parent directory, making it easy to spot and fix.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🛂️schema/🔣️.json`

- **Classification:** (c) non-canonical schema directory name (violation)
- **Owner (current):** test-case scope: 🪪️mutation-metadata
- **Intended owner:** test-case scope: 🪪️mutation-metadata (rename directory 🛂️schema/ to 🧬️schema/)
- **Should export:** JSON Schema draft-07 for mutation-metadata cases (schemaVersion/source/expected.declarations/aliases/manualMutationLeafImpls)
- **Consumers:** sibling test runner in 🪪️mutation-metadata/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🧬️schema/🔣️.json
- **Evidence:**
  - Same pure (c) non-canonical-directory-name violation as 📋️mutation-inventory/🛂️schema/.

## Detail: (d) flat-file schema violations — 18 files

A schema-shaped `🧬️schema.json` / `🛂️schema.json` / `🧬️.schema.json` / `🧬️fixture.schema.json` FILE sitting directly in a module or test-case directory, instead of being wrapped in a `🧬️schema/` directory MODULE. This is the single largest violation category in the slice (18 of 31 violations), and includes the most consequential single finding: the repo-wide mutation-descriptor meta-schema at `📚️library/🧬️.schema.json`.

### `♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** mit-bestand/bericht/documents module
- **Intended owner:** mit-bestand/bericht/documents module (needs a 🧬️schema/ directory wrapper)
- **Should export:** JSON Schema draft-2020-12 for the bericht document catalog (version/documents[].id/texPath/sources/actorNetwork)
- **Consumers:** ♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts:20 (typescript-generic-validator (readFileSync(join(modulePath, "🧬️schema.json"))))
- **Decision:** move-to ♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema/🔣️.json
- **Evidence:**
  - ♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema.json is a flat FILE (schema-shaped: "$schema":"https://json-schema.org/draft/2020-12/schema", "type":"object", "properties") sitting directly in the module directory, not wrapped in a 🧬️schema/ directory module — this is the exact 'flat 🧬️schema.json FILE' violation pattern (d) explicitly flagged in the task brief.
  - ♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts:20 — `const schema = JSON.parse(readFileSync(join(modulePath, "🧬️schema.json"), "utf8"));` — generic per-module loader iterating 🔨️modules/* directories, so every sibling module (including this one) is expected to expose a flat 🧬️schema.json; this is itself a repo-wide anti-pattern baked into the harness, not just a one-off mistake.
- **Open questions:**
  - The consuming loader (♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts) hard-codes the flat filename convention for ALL bericht 🔨️modules/* directories — a real fix must either update that loader to look for 🧬️schema/🔣️.json, or confirm whether this flat-file convention is deliberately different from the mutationPayloadSchemaLocation convention for the mit-bestand tree specifically (mit-bestand has no formal scope-registration mechanism at all, see coverage notes).

### `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** mit-bestand/demonstrator/runtime module
- **Intended owner:** mit-bestand/demonstrator/runtime module (needs a 🧬️schema/ directory wrapper)
- **Should export:** JSON Schema draft-2020-12 for demonstrator runtime catalog (schemaVersion/host/assetsDirectory/panes[])
- **Consumers:** none found
- **Decision:** move-to ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema/🔣️.json
- **Evidence:**
  - ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema.json is a flat schema-shaped FILE, sibling of the runtime module's data file 🔣️.json and its 🟦️.ts API — same flat-file violation (d) as the bericht/documents case.
  - ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🟦️.ts imports only ./🔣️.json (the data), never ./🧬️schema.json — no confirmed runtime or test consumer of the schema file was found by repo-wide grep for '🧩️runtime/🧬️schema' or literal 'runtime/🧬️schema.json'.
- **Open questions:**
  - No consumer of this schema file could be confirmed by grep (unlike the bericht sibling, whose loader is explicit). It may be validated only by a project-wide/dynamic convention (same generic pattern as bericht) or may be currently orphaned/unenforced — flag for the demonstrator package owner to confirm.

### `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🧬️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** print/print-font-catalog module
- **Intended owner:** print/print-font-catalog module (needs a 🧬️schema/ directory wrapper)
- **Should export:** JSON Schema draft-2020-12 for the font catalog array (family/directory/filename/texFilename)
- **Consumers:** 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts:28 (typescript-test (ajv .validate against 🔣️.json catalog))
- **Decision:** move-to 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🧬️schema/🔣️.json
- **Evidence:**
  - 🧬️schema.json is flat in the module root next to 🔣️.json (data) — flat-file violation (d), same repo-wide anti-pattern as the other print/mit-bestand cases.
  - 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts:28 — `assert.equal(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(modulePath, "🧬️schema.json"), "utf8")), catalog), true);` — confirmed live, current consumer.

### `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🧬️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** print/tectonic-template-compilation/catalog module
- **Intended owner:** print/tectonic-template-compilation/catalog module (needs a 🧬️schema/ directory wrapper)
- **Should export:** JSON Schema draft-2020-12 for the tectonic document catalog (version/sourceDateEpoch/documents[])
- **Consumers:** 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts:79 (typescript-test (ajv .compile/.validate against 🔣️.json catalog))
- **Decision:** move-to 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🧬️schema/🔣️.json
- **Evidence:**
  - Flat 🧬️schema.json sibling of 📇️catalog/🔣️.json data file — violation (d).
  - 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts:79 — `const validate = new (require("ajv/dist/2020").default)().compile(JSON.parse(readFileSync(join(root, modulePath, "🧬️schema.json"), "utf8")));` confirmed consumer inside verifyPrintDocumentCatalog().

### `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🧬️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** print/tectonic-template-compilation/bundle module
- **Intended owner:** print/tectonic-template-compilation/bundle module (needs a 🧬️schema/ directory wrapper)
- **Should export:** JSON Schema draft-2020-12 for the tectonic upstream bundle manifest (version/url/archiveBytes/upstreamIdentity/files[])
- **Consumers:** 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts:110 (typescript-test (ajv .validate against 🔒️dependencies.json manifest))
- **Decision:** move-to 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🧬️schema/🔣️.json
- **Evidence:**
  - Flat 🧬️schema.json sibling of 📚️bundle/🔒️dependencies.json — violation (d).
  - 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts:110 — `assert.equal(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(modulePath, "🧬️schema.json"), "utf8")), manifest), true);` confirmed consumer.

### `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🧬️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** print/tectonic-template-compilation/toolchain module
- **Intended owner:** print/tectonic-template-compilation/toolchain module (needs a 🧬️schema/ directory wrapper)
- **Should export:** JSON Schema draft-2020-12 for the tectonic toolchain release manifest (version/release/platforms[])
- **Consumers:** none found
- **Decision:** move-to 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🧬️schema/🔣️.json
- **Evidence:**
  - Flat 🧬️schema.json sibling of 🔧️toolchain/🔣️.json and 🔧️toolchain/📜️script.ts — same flat-file violation (d) as its 3 sibling print modules.
  - 🔧️toolchain/📜️script.ts (registered via 🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📋️project.json:216,240 as `prepare`/`verify` nx targets) contains no literal reference to "schema" — no confirmed active validator of this specific file was found, unlike its 3 sibling print schemas which are all validated inside 🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts.
- **Open questions:**
  - No active consumer confirmed for this schema file specifically (its 3 print siblings all have one) — worth confirming with the print product owner whether toolchain manifest validation was dropped or lives somewhere this sweep did not find.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️policy.schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** repo/library/caching module
- **Intended owner:** repo/library/caching module (needs a 🧬️schema/ directory wrapper)
- **Should export:** JSON Schema draft-07 "Nx Cache Policy" (version/uncached/continuous/generatedDirectories/toolchains)
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts:332 (typescript (readFileSync(join(SCRIPT_ROOT, "🧬️policy.schema.json")))); 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json:2 (data ($schema pointer "./🧬️policy.schema.json"))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/🔣️.json
- **Evidence:**
  - 🧬️policy.schema.json sits flat in the ⚡️caching module root (sibling of 🔣️policy.json, 📜️script.ts, 📋️project.json) rather than in a 🧬️schema/ directory — flat-file violation (d), with a real, heavily-used, load-bearing consumer.
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts:332 — `const schema = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧬️policy.schema.json"), "utf8"));`
- **Open questions:**
  - A move requires updating both the hard-coded path in 📜️script.ts:332 and the relative "$schema" pointer in 🔣️policy.json:2.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧬️compiler-imports.schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** repo/library/discovery module
- **Intended owner:** repo/library/discovery module (needs a 🧬️schema/ directory wrapper)
- **Should export:** JSON Schema draft-07 for compiler-import test cases (name/source/expected[])
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts:836 (typescript-test (reads sibling 📥️compiler-imports.json fixture data))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧬️schema/🔣️.json
- **Evidence:**
  - 🧬️compiler-imports.schema.json sits flat in the 🔍️discovery/ module root, sibling to its data file 📥️compiler-imports.json and the module's own 🟦️.ts — flat-file violation (d).
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts:836 — `const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🔍️discovery/📥️compiler-imports.json"), "utf8")) as {...}` reads the DATA file; the schema itself is presumably validated by the same generic 🧬️.schema.json-style loader pattern used elsewhere in this test file, though a literal grep for "compiler-imports.schema" found no separate direct hit.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🛂️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** test-case scope: ✅️mutation-test-presence
- **Intended owner:** test-case scope: ✅️mutation-test-presence (rename to 🧬️schema/🔣️.json)
- **Should export:** JSON Schema draft-07 for mutation-test-presence cases (schemaVersion/mutationRoot/leaf/cases)
- **Consumers:** sibling test runner in the same test directory (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🧬️schema/🔣️.json
- **Evidence:**
  - 🛂️schema.json is a flat file using the non-canonical 🛂 prefix instead of the taxonomy-sanctioned 🧬️schema directory name — combined (c)+(d) violation: wrong name AND flat (no directory).

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🛂️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** test-case scope: 🌳️workspace-taxonomy
- **Intended owner:** test-case scope: 🌳️workspace-taxonomy (rename to 🧬️schema/🔣️.json)
- **Should export:** JSON Schema draft-07 for workspace-taxonomy cases (schemaVersion/canonicalLocator/childLocator/cases/startCases/anchorCases)
- **Consumers:** sibling test runner in the same test directory (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🧬️schema/🔣️.json
- **Evidence:**
  - Same 🛂 non-canonical-prefix, flat-file pattern as ✅️mutation-test-presence — (c)+(d).

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🛂️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** test-case scope: 🏗️mutation-scaffolding
- **Intended owner:** test-case scope: 🏗️mutation-scaffolding (rename to 🧬️schema/🔣️.json)
- **Should export:** JSON Schema draft-07 for mutation-scaffolding cases (schemaVersion/mutationRoot/name/*Aggregate fields)
- **Consumers:** sibling test runner in the same test directory (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🧬️schema/🔣️.json
- **Evidence:**
  - Same 🛂 non-canonical-prefix, flat-file pattern — (c)+(d).

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🛂️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** test-case scope: 🏷️metadata-source-provider
- **Intended owner:** test-case scope: 🏷️metadata-source-provider (rename to 🧬️schema/🔣️.json)
- **Should export:** JSON Schema (no explicit $schema dialect declared, but type/required/properties/$defs shaped) — cases[] for metadata-source-provider (schemaVersion/cases, $defs/case)
- **Consumers:** sibling test runner in 🏷️metadata-source-provider/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🧬️schema/🔣️.json
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🛂️schema.json was NOT in the pre-pass candidate list because it has no literal "$schema" key (the pre-pass filtered on that literal), but its content is unambiguously JSON-Schema-shaped (top-level "type":"object", "required", "properties", "$defs"/"$ref") — found during this audit's `find -iname '*schema*'` sanity sweep of the repo product tree.
  - Same 🛂-prefix flat-file pattern as the other 6 already-flagged 🛂️schema* files in 📚️library/🧪️tests/ — 10 total instances of this drift in the tree.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🛂️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** test-case scope: 📽️cargo-provider-projection
- **Intended owner:** test-case scope: 📽️cargo-provider-projection (rename to 🧬️schema/🔣️.json)
- **Should export:** JSON Schema (no explicit $schema dialect declared, but type/required/properties/$defs shaped) — accepted[]/rejected[] cargo-provider-projection cases
- **Consumers:** sibling test runner in 📽️cargo-provider-projection/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🧬️schema/🔣️.json
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🛂️schema.json was NOT in the pre-pass candidate list because it has no literal "$schema" key (the pre-pass filtered on that literal), but its content is unambiguously JSON-Schema-shaped (top-level "type":"object", "required", "properties", "$defs"/"$ref") — found during this audit's `find -iname '*schema*'` sanity sweep of the repo product tree.
  - Same 🛂-prefix flat-file pattern as the other 6 already-flagged 🛂️schema* files in 📚️library/🧪️tests/ — 10 total instances of this drift in the tree.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🛂️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** test-case scope: 🧬️mutation-type-origin
- **Intended owner:** test-case scope: 🧬️mutation-type-origin (rename to 🧬️schema/🔣️.json)
- **Should export:** JSON Schema (no explicit $schema dialect declared, but type/required/properties/$defs shaped) — mutationRoot/leaf/rustFilename/cases for mutation-type-origin
- **Consumers:** sibling test runner in 🧬️mutation-type-origin/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🧬️schema/🔣️.json
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🛂️schema.json was NOT in the pre-pass candidate list because it has no literal "$schema" key (the pre-pass filtered on that literal), but its content is unambiguously JSON-Schema-shaped (top-level "type":"object", "required", "properties", "$defs"/"$ref") — found during this audit's `find -iname '*schema*'` sanity sweep of the repo product tree.
  - Same 🛂-prefix flat-file pattern as the other 6 already-flagged 🛂️schema* files in 📚️library/🧪️tests/ — 10 total instances of this drift in the tree.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🛂️schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** test-case scope: 🪢️cargo-provider-binding
- **Intended owner:** test-case scope: 🪢️cargo-provider-binding (rename to 🧬️schema/🔣️.json)
- **Should export:** JSON Schema (no explicit $schema dialect declared, but type/required/properties/$defs shaped) — accepted[]/rejected[]/traces[] for cargo-provider-binding
- **Consumers:** sibling test runner in 🪢️cargo-provider-binding/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🧬️schema/🔣️.json
- **Evidence:**
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🛂️schema.json was NOT in the pre-pass candidate list because it has no literal "$schema" key (the pre-pass filtered on that literal), but its content is unambiguously JSON-Schema-shaped (top-level "type":"object", "required", "properties", "$defs"/"$ref") — found during this audit's `find -iname '*schema*'` sanity sweep of the repo product tree.
  - Same 🛂-prefix flat-file pattern as the other 6 already-flagged 🛂️schema* files in 📚️library/🧪️tests/ — 10 total instances of this drift in the tree.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🧬️fixture.schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** test-case scope: 🧼️clean
- **Intended owner:** test-case scope: 🧼️clean (rename to 🧬️schema/🔣️.json)
- **Should export:** JSON Schema draft-07 for clean-ticket cases (version/ticketsRoot/tickets/special/cases/reopened)
- **Consumers:** sibling test runner in 🧼️clean/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🧬️schema/🔣️.json
- **Evidence:**
  - Flat file directly in the test directory root (not inside a fixtures/ subdirectory, so not (b)) using the filename "fixture.schema.json" rather than a 🧬️schema/ directory module — flat-file violation (d).

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/📐️options.schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** test-case scope: 🫙️artifact-empty-facet-authority
- **Intended owner:** test-case scope: 🫙️artifact-empty-facet-authority (fold into 🧬️schema/ as a second facet, or rename+move)
- **Should export:** JSON Schema draft-07 "Independent Current Options Facet Ownership Oracle" (sourcePath/fileKindId)
- **Consumers:** sibling test runner in 🫙️artifact-empty-facet-authority/ (test-harness)
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧬️schema/📐️options/🔣️.json (or fold as an additional required field into the existing 🧬️schema/🔣️.json)
- **Evidence:**
  - 📐️options.schema.json is a flat file using the 📐 prefix explicitly named as a legacy/non-canonical pattern in the task brief ("📐️schema/", "📐️fixture-schema/") — combined (c)-style wrong-prefix and (d) flat-file violation.
  - It is a SIBLING of this same test directory's correctly-named 🧬️schema/🔣️.json ("Independent Empty-Facet Fixture Owner Oracle") — two competing schema artifacts for closely related but distinct contracts (options-facet ownership vs fixture-owner) inside one test scope, which is exactly the kind of drift the taxonomy's descriptorCardinality rule ("one-canonical-no-competing-descriptor") is meant to prevent even at test-fixture granularity.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️.schema.json`

- **Classification:** (d) flat-file schema outside a module (violation)
- **Owner (current):** repo/library module (repo-wide mutation-descriptor authority)
- **Intended owner:** repo/library module — 🧬️schema/ directory at the library module root
- **Should export:** JSON Schema draft-07 "Direct Mutation Descriptor" ($id https://semio.tech/schema/mutation-descriptor/1) — the canonical, repo-wide contract every mutation owner's own 🧬️.schema.json companion file must satisfy (schemaVersion/owner/semanticKind/displayName/emoji/aggregateVariant/payloadSchema/textOpcode/binaryTag/invertibility/diffParticipation/outcomeClasses/composition/requiredLanguageSurfaces)
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts:7394 (typescript-test (hard-coded absolute path to this exact file)); 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts:52 (typescript-test (generic loop reading every owner directory's own 🧬️.schema.json descriptor and presumably cross-checking against this meta-schema)); 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:3358,3382 (taxonomy (mutation-descriptor-specimen / mutation-descriptor-agreement entries reference this contract))
- **Decision:** move-to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json
- **Evidence:**
  - This is THE most consequential flat-file finding in the slice: the library module's own repo-wide 'mutation descriptor' meta-schema — the schema that defines what a valid mutation-descriptor JSON file looks like anywhere in the whole repository — lives as a hidden, anonymous dot-file (🧬️.schema.json) directly in the library module root, rather than in a 🧬️schema/ directory, even though it is the schema that literally standardizes the naming convention it violates.
  - 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts:7394 — `const descriptorSchema = JSON.parse(readFileSync(join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️.schema.json"), "utf8"));` — hard-coded full repo-relative path, so any move requires updating this test.
  - Historical ticket notes (.🧬semio/🦑️repo/🎫️tickets/.../SEMANTIC-MUTATIONS-OVERHAUL/*.md) refer to an even earlier location, `🦑️repo/📚️library/🔣️mutation-descriptor.schema.json`, showing this file has already moved once without ever landing in the canonical 🧬️schema/ directory shape.
- **Open questions:**
  - A move affects every mutation-owner directory repo-wide that carries a companion 🧬️.schema.json descriptor (validated against this meta-schema) — this is a repo-wide, high-blast-radius rename that should be coordinated with whoever owns the mutation/DSL derive machinery (outside this slice, under 🧰️framework/🔨️modules/), not executed unilaterally from this slice alone.

## Detail: duplicate-authority / notable (a) cases specifically checked

These three were explicitly named in the task brief as pairs to check for duplicate authority. All three resolved as **not duplicates** — distinct contracts, correctly scoped — but are documented here in full because the brief called them out by name and because one (the mcp GraphQL directory) surfaces a real naming-convention question worth a follow-up decision.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🔗️graphql/🔗️.graphql`

- **Classification:** (a) authoritative scope schema (correct)
- **Owner (current):** repo/client/mcp module
- **Intended owner:** repo/client/mcp module
- **Should export:** GraphQL SDL: Node interface, DateTime scalar, and the MCP GraphQL API surface
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🔗️graphql/📋️project.json (nx project registration)
- **Decision:** keep (follow-up-needed on directory convention)
- **Evidence:**
  - 🔗️.graphql is a real, substantial (12.8KB) GraphQL SDL schema — legitimate authoritative contract for the mcp module's GraphQL surface, not a fixture or generated copy.
  - It sits in a bare 🔗️graphql/ directory directly under 🔌️mcp/, not nested inside a 🧬️schema/ module directory, unlike the sibling 🖥️server/🧬️schema/🐘️postgres/ (SQL) and 🧪️test/🧬️schema/🔣️.json patterns which DO nest their format-specific content one level under 🧬️schema/.
- **Open questions:**
  - Taxonomy's schemaFormats maps 🔗️graphql → graphql(camel) as one of the sanctioned schemaFacetKinds formats, implying GraphQL schema content belongs inside a scope's 🧬️schema/ module (e.g. 🧬️schema/🔗️.graphql) alongside other language surfaces, the way 🖥️server/🧬️schema/🐘️postgres/🗄️.sql does for SQL. Whether mcp's top-level 🔗️graphql/ directory is a deliberately different, older convention or itself a drift from the canonical pattern needs a call from whoever owns the schema/taxonomy directory-kind registration.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🔣️.json`

- **Classification:** (a) authoritative scope schema (correct)
- **Owner (current):** repo/library/normalization module
- **Intended owner:** repo/library/normalization module
- **Should export:** JSON Schema draft-2020-12 ($id urn:semio:taxonomy:source-admission:v1) — the normalization module's own source-admission input contract
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🧬️schema/🔣️.json (nested test-local schema (distinct, narrower contract for one io test case, not a duplicate))
- **Decision:** keep
- **Evidence:**
  - 🧹️normalization/🧬️schema/🔣️.json (10772 bytes, $id urn:semio:taxonomy:source-admission:v1) is the normalization module's own scope-level canonical schema, correctly placed directly in a 🧬️schema/ directory at the module root — classification (a), no violation.
  - It is NOT a duplicate of the narrower 🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🧬️schema/🔣️.json, which validates only one nested unit test's io cases[] — different $id/scope, correctly test-local.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`

- **Classification:** (a) authoritative scope schema (correct)
- **Owner (current):** repo/test module
- **Intended owner:** repo/test module
- **Should export:** JSON Schema draft-2020-12 ($id https://semio-tech.com/schema/repo/test/v2) "Semio Repository Test Protocol v2" — the domain-neutral, cross-language test coordination contract
- **Consumers:** 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📇️registry/🔣️.json:1 (data ($schema pointer "../🧬️schema/🔣️.json")); taxonomy.json testSchemaLocation fact (taxonomy declaration (directoryPath 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema, fileKindId json))
- **Decision:** keep
- **Evidence:**
  - This is the taxonomy-declared canonical testSchemaLocation, confirmed live and authoritative: correctly named 🧬️schema/ directory at the 🧪️test module root, 46886 bytes, $id https://semio-tech.com/schema/repo/test/v2, and its sibling 📇️registry/🔣️.json data file references it directly via a relative "$schema" pointer.
  - The task brief asked whether 🧪️test/🧪️tests/🧭️contribution-directory-ownership/🧬️schema/🔣️.json (a nested per-unit-test schema inside this same module's own 🧪️tests/ suite) competes with this file — it does not: that file governs an unrelated, narrower contract (defaultDirectory/overrides/cases for one ownership unit test) with no shared $id, contractId, or subject matter overlap. No duplicate-authority violation between the two.

