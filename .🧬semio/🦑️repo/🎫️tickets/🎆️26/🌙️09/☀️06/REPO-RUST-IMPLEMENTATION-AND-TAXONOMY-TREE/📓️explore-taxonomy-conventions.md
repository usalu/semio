# Explore: Taxonomy Conventions for `repo` Product (Go + Rust Side by Side)

Read-only survey. All paths are absolute-from-repo-root; emoji are load-bearing (they are the
statute-checked "leading grapheme" of each folder/file).

## 1. Canonical folder/file/module shape (from `AGENTS.md` identification schemes + exemplars)

### 1.1 Identification scheme (from `🧰️framework/🛍️products/🦑️repo/AGENTS.md` §🪪 Identification)
Hierarchy: `root → years/year/.../seconds` (temporal) and `repo → releases/release → versions/version → checkpoints/checkpoint` (release chain), and `repo → technologies → technology → bundles → bundle → folders → folder → files → file → lines/line`.
- `technology.kind ∈ {infrastructure, user, research}`; `repo` itself is a `technology` of kind `infrastructure`.
- `bundle.kind ∈ {library📚lib, schema🛂sch, binary⌨️bin, ui🖱️ui, example📔exa, site🌐site, assets🏪ast}` (§2.2 also adds `repo`).
- `folder.kind ∈ {organization, required}`. §9.3 **Folder-kind derivation**: a folder is `required` iff it (or a package leaf) contains one of `{package.json, pyproject.toml, go.mod, Cargo.toml}` or a file with extension `.csproj`/`.sln` — i.e. **a language package root is a `required` folder**, everything else domain-organizational.
- §9.4 **Generated-folder predicate**: `{generated, dist, build, node_modules, __pycache__, .next, coverage}` are always non-taxonomy/ignored — confirms the root `repo/` folder with `.next` and compiled binaries (found at `./repo/`) is a stray generated/legacy tree, not part of the taxonomy.
- IDs are built by concatenating parent id + this node's own emoji + flattened kebab/segment code (e.g. `🏘️compose📚js🗃️sketchpad💻designtsx`) — **the emoji is the node's "kind" tag, prepended to a domain-slug**, never a language name unless the node genuinely *is* a language package (see 1.3).

### 1.2 Module anatomy observed across exemplars (`💻️os/🔨️modules/🌉️mcp`, `🔨️modules/🔀️dispatch`, repo's own `📚️library/🧹️normalization`)
A domain module `🔨️modules/<emoji><domain-slug>/` contains, at its own root:
- `README.md` (optional prose)
- One godfile per language directly implementing that node's own logic: `🟦️.ts`, `🦀️.rs`, `🐹️.go` — filename is **only the extension's emoji + `.ext`**, never a descriptive name, unless it's a true leaf entrypoint (`🚀️bin.rs`, `📦️main.rs`).
- `🧬️schema/` — schema-first contracts. Formats seen: JSON Schema (`draft/2020-12`) under `🧬️schema/🔣️.json` (the manifest filename is *always* `🔣️.json` — "collection manifest" convention), or a language godfile `🦀️.rs`/`🐹️.go` when the schema is expressed as typed structs consumed by multiple packages (e.g. `mcp/🧬️schema/🦀️.rs`).
- `📦️packages/<lang>/` — one dir per language runtime package:
  - `🦀️rust/`: `Cargo.toml`, `📋️project.json` (nx target wiring; **only** calls `bun ./📜️script.ts …`), `📜️script.ts` (extends `BundleScript`/`ScriptRouter` from repo's `📚️library` typescript helpers), the package's own godfile `🦀️.rs`, optional `tests/*.rs` (rust integration tests, one file per scenario, emoji-named), optional `📦️main.rs`/`🚀️bin.rs` for binaries.
  - `🟦️typescript/`: `package.json`, `📋️project.json`, `📜️script.ts`, godfile `🟦️.ts`, and `*.test.ts` files (also emoji-prefixed) plus a `🧪️tests/🟦️.ts` subfolder for larger suites.
  - `🐹️go/` (seen at repo product level): would carry `go.mod` + `🐹️.go` godfile.
- Deeper **domain** subfolders (not language subfolders) recurse the same shape: e.g. `mcp/🏠️workspace/🔗️remote/🧩️pair/{🧪️oracle,🧫️fixtures,🧬️schema}`, each again with its own single-language-agnostic godfile per implementing language at that level.
- `🧫️fixtures/` — data fixtures (json/jsonl) shared by tests across languages.
- `🧪️tests/` (or `🧪️test` module) — cross-cutting/e2e test module, itself following the same module shape (`🧪️tests/<scenario-slug>/{🔣️.json, 🟦️.ts, 🧬️schema/🔣️.json}`, optionally per-language files `🐹️….go`/`🦀️….rs` beside the `.ts` oracle).

### 1.3 Godfile `#region` sections ↔ domain subfolders
Inside a module's root godfile, `//#region 🔖️Name` / `//#endregion 🔖️Name` blocks (nested `//#region 💡️Sub` allowed) partition the file into named sections that mirror sibling **domain** subfolders one level down, or concerns that haven't yet been promoted to their own subfolder. Example, `mcp/🦀️.rs` (914 lines) top-level regions: `Facets`, `CoreCapabilities`, `Catalog`, `Tools`, `MutationProtocolTools` (nested `💡️Inference`), `WorkspaceOptions`, `StdioEntrypoint`, `HttpEntrypoint`, `Tests`. Sibling folders that *do* exist as their own module (`🏠️workspace`, `🔀️dispatch`, `🧠️context`, `📇️registry`, …) are *not* re-duplicated as regions in the parent godfile — once a concern earns its own folder+godfile, the parent only glues/re-exports it. This is the "split when it grows" rule: a `#region` is a pre-folder; a folder is a graduated region.
- `🎮️commands/🌊️workflow/🦀️.rs` shows the same pattern at leaf granularity: `Command`, `Workflow`, `Scheduler`, `AgentRunner`, `Tests`.

### 1.4 `📋️project.json` + `📜️script.ts` contract (root `AGENTS.md`, confirmed in every package)
- `project.json` (per root AGENTS.md) **MUST only call** `📜️script.ts <command> <subcommand...> <args>` — verified: every `📋️project.json` target's `command` is exactly `bun ./📜️script.ts <cmd>`.
- `📜️script.ts` is the **only** permitted script file per directory; it imports shared helpers (`BundleScript`, `ScriptRouter`, `resolveTestLevel`, `runCargoTestBudgeted`, `runBundleScriptMain`) from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` — i.e. **all Rust/Go packages in the monorepo run their tests through the repo product's own TypeScript library**, not raw `cargo`/`go` invocations from CI.

## 2. Collection manifests (`🔣️.json`) and normalization module

`🔣️.json` is the fixed filename for "this directory is a data collection" (test-vector fixture or schema document), always paired with a `🧬️schema/🔣️.json` JSON-Schema sibling that validates it. Root package `🔣️taxonomy.json` (referenced by `🧅️layering.json`/`🚚️migration.json` comments) is the master registry of `areaLayers`/`areas` used repo-wide for the layering ratchet.

`📚️library/🧹️normalization/` (source-admission + package-boundary + generic-stem-collision) is the analyzer that *governs* what folders/files even count as taxonomy nodes:
- `🧬️schema/🔣️.json`: `sourceAdmissionInput → sourceAdmission` — classifies every git-tracked/untracked/ignored/generator/gitlink source candidate into `origins` (`tracked | nonignored-untracked | ignored-generator | explicit-ticket`) and diagnostics; `explicitDirectory`/`unsafeAncestor`/`repositoryBoundary: gitlink` flags gate whether a path is even eligible to be a taxonomy folder.
- `🧪️tests/📦️package-boundary-classification/🔣️.json`: classifies a file's "glue role" (`declaration | implementation | thin-delegation`) per analyzer (`rust`, `typescript`, …) — used to decide whether a godfile is a real implementation leaf vs. a re-export/glue shim, and encodes a `scopeSpecificityOrder` (`path-pattern > repository-root > directory-kind > package-root > fixed-directory-contract > sibling-fixed-filename-contract > exact-path`) for resolving which schema/contract governs a given path.
- `🧪️tests/💥️generic-stem-collision-resolution/`: guards against two sibling folders reducing to the same "generic stem" once emoji+diacritics are stripped (i.e. domain names must be distinguishable after normalization, not just visually).

## 3. Path/emoji statutes enforced by the analyzer

`📚️library/🧪️tests/🔏️path-emoji-statutes/` (fixture `🔣️.json` + schema) drives `createTaxonomyPathMatcher`, `leadingEmojiIdentity`, `pathEmojiStatuteFindings`, `mutationOwnerIdentity`, `semanticDirectoryKindId`, `reservedDocumentationBasename`, `subsetIdForDirectoryName` from `📚️library/🔍️discovery/🟦️.ts` — i.e. every folder/file name is checked to (a) start with exactly one identity-establishing leading grapheme (emoji, incl. ZWJ/variation-selector sequences — see `🧪️tests/🔤️taxonomy-leading-grapheme/`), (b) match the expected semantic directory/file kind for its position, (c) not shadow a reserved documentation basename (`README.md`, `AGENTS.md`).

`🧪️tests/🔤️taxonomy-leading-grapheme/`: validates the `graphemes`/`isEmojiGrapheme` helper against an Intl.Segmenter oracle across both `bun` and `tsc` transpilers — the leading-emoji split must be Unicode-grapheme-correct (multi-codepoint emoji, skin tones, ZWJ sequences), not naive `charAt(0)`.

## 4. Known-bad patterns in current `🦑️repo` tree (language/tech-named, not domain-named)

| Path | Why it violates the rules |
|---|---|
| `🔨️modules/💻️client/⌨️cli/` (Go module: `go.mod`, `cmd/repo/🐹️.go`, `internal/{command,eventstore,glob,graphql,humanize,id,ignore,mcp,mcpserver,search,templatefunc,yaml}/*.go`) + sibling `📦️packages/🟦️typescript/` | `internal/*` is Go-idiom package-privacy, not a domain taxonomy — each of `graphql`, `humanize`, `id`, `ignore`, `search`, `yaml` is a *domain* concern that per the rules should be its own `🔨️modules/<emoji>domain/` with `🧬️schema` + `🧪️tests` + `📦️packages/{🐹️go,🦀️rust,🟦️typescript}`, not a Go-only `internal/` folder. `cmd/repo/` is Go-toolchain convention (must be `cmd/<binary>` for `go build`), not an emoji/domain folder — collides with §9.4 generated-folder awareness and with `folder.kind: required` semantics (a `cmd/` dir isn't a package root by the schema, it's a go-only escape hatch). The TypeScript half lives in a *different* module (`⌨️cli/📦️packages/🦀️rust` is yet another, separate, top-level `🔨️modules/⌨️cli/`) — the same domain ("cli") is split across two unrelated module roots (`💻️client/⌨️cli` vs `⌨️cli`), so language implementations of the *same* domain aren't siblings under one `📦️packages/`. |
| `🔨️modules/⌨️cli/📦️packages/🦀️rust/` (separate top-level module) | Duplicate "cli" domain root vs `💻️client/⌨️cli`; only has a Rust package, no shared `🧬️schema`/`🧪️tests` alongside the Go implementation — Go and Rust implementations of the same domain are not co-located, breaking "if code is repeated, it MUST be close to each other." |
| `🎮️commands/*/🦀️.rs` (8 command modules, e.g. `🌊️workflow`, `🎛️terminal-dashboard`, `🖥️terminal-dashboard-daemon`, `🌳️command-tree-discovery`, `📇️playground-catalog-query`, `📜️root-script-delegation`, `🔌️plugin-registry`, `⌨️cli-usage-presentation`) | Rust-only; no `🐹️go` or `🟦️typescript` sibling packages despite the product needing multi-implementation parity, and no per-command `📦️packages/` wrapper — the godfile sits directly under the domain folder with no package/schema/tests split, so these are effectively single-language leaves masquerading as domain modules. |
| `🔨️modules/🖥️server/🎛️coordinator/` (Go: `go.mod`, `.env.example`, `Caddyfile`, `Dockerfile`, `🐚️durability_unix.go`, `🪟️durability_windows.go`, `📚️repository.go`, `🗄️event_store.go`, `🛡️durability.go`, `🧩️component.go`) + sibling `📦️packages/🟦️typescript/` | Mixes a full Go server implementation directly at the module root (not under `📦️packages/🐹️go/`) with deployment artifacts (`Caddyfile`, `Dockerfile`, `.env.example`) that belong under an infra/ops facet, not the domain module root. `durability_unix.go`/`durability_windows.go` are Go-build-tag file-splitting (OS, not domain) living as siblings instead of being expressed as a `🖥️server/🎛️coordinator/🛡️durability/` domain folder with per-OS packages/targets. |
| `🔨️modules/🖥️server/🧬️schema/🐘️postgres/🗄️.sql` | Schema keyed by *technology* (`🐘️postgres`) rather than by *domain*; fine as a package **target** under a domain's schema, but here it *is* the domain folder — no domain name at all, just the DB engine. |
| root `./repo/` (found at repo root, sibling of `.🧬semio`) containing `assets/fixtures`, `client/cli`, `client/client.exe` (compiled binary checked into tree), `client/vscode`, `lib/js`, `server/coordinator` | Entirely non-emoji, non-taxonomy legacy tree (pre-migration naming: `repo`, `client`, `lib`, `server` as plain English tech words) with a **committed compiled binary** (`client.exe`) — violates "no legacy support/compat layers" and the generated-folder predicate intent; this is exactly the dead tree the new taxonomy must replace/delete, not extend. |
| `🔨️modules/📚️library/📦️packages/🐹️go/`, `🔨️modules/💻️client/⌨️cli/📦️packages/🐹️go/`, `🔨️modules/🧪️test/📦️packages/🐹️go/` (the only 3 `🐹️go` package dirs in the whole product) | Go coverage is sparse and inconsistent — most domain modules (`⌨️cli` proper, `🖥️server`, `🎮️commands/*`) have **no** `🐹️go` package at all, some have Go living outside `📦️packages/` entirely (see coordinator, client/cli above). Confirms Go/Rust are not actually side-by-side anywhere in `🦑️repo` today. |

## 5. Prior ticket decisions still binding

- `🎆️26/🌙️01/☀️14/REPO-TREE-REFACTOR` and `🌙️01/☀️21/REPO-BINARY-REFACTOR-PLAN` / `REPO-BINARY-CONSOLIDATION`: **pre-date the emoji taxonomy entirely** (they talk about `compose/BUNDLE/...` ID strings, `./repo/cli`, `cmd/repo`, GraphQL `Range`/`Position` types, VS Code tree provider wiring). They are historical/superseded — the "compose/…" ID scheme they proposed was replaced by the current `AGENTS.md` §🪪 Identification hierarchy (`repo://…` URIs + emoji-concatenated IDs). Nothing in them should be reused verbatim; they do however confirm `./repo/cli`, `cmd/repo` are long-standing legacy paths, consistent with what's still on disk today (see §4) — i.e. **that migration was never finished**, it just got a new, better taxonomy layered on top elsewhere while the old `./repo/` tree was left in place.
- `🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION`: a large, still-**open**, actively-worked ticket (many parallel goal-report files, e.g. `📓️goal-facetsplit-architect-report.md`) about splitting inlined mutation `diff`/`inverse` functions in the **`✏️s` (sketchpad?) product's `🔌️plugins`** into per-mutation `🔺️diff/🦀️.rs` + `↩️inverse/🦀️.rs` facet folders. This is the same *kind* of graduation rule as §1.3 above (region → folder once a concern is real) applied at large scale, but its content is about a different product (`✏️s`), not `🦑️repo`. Relevant transferable decision: **facet-splitting is done by restoring/regenerating from git history plus scripted rewrites, never hand-written migrations**, and cross-session coordination happens via a shared "coordinator" doing repo-wide sweeps — take this as precedent for how the repo-product Go/Rust split should be executed (script-driven, verified by grep/build, not manual copy-paste), but it does not fix any repo-product-specific naming decision.
- No ticket found that already redesigns `🦑️repo`'s own module tree for Go+Rust parity — this ticket (`REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE`) is greenfield for that specific tree.

## 6. Proposed target taxonomy for `🧰️framework/🛍️products/🦑️repo/🔨️modules/`

Legend: **[MOVE]** = relocates existing content; **[NEW]** = net-new; **[KEEP]** = already compliant, no change.

```
🔨️modules/
├── 🪪️identity/                          [NEW]            domain: contributor/session/checkpoint identity
│   ├── 🧬️schema/🔣️.json
│   ├── 🧪️tests/
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
├── 🎫️tickets/                           [MOVE: from 💻️client/⌨️cli/internal/* ticket-shaped logic if any exists there]
│   ├── 🧬️schema/🔣️.json
│   ├── 🧪️tests/
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
├── 🎯️goals/                             [NEW]
│   └── … (same shape)
├── 🧑️devs/                              [MOVE: parts of internal/command tied to contributor identity]
│   └── 🤖️agents/  ⧸  📡️sessions/       [MOVE: session logging currently implicit in config.toml logging.*]
├── 🪝️hooks/                             [NEW – currently only config-driven, not modularized]
├── 📡️events/                            [MOVE: 💻️client/⌨️cli/internal/eventstore, 🖥️server/🎛️coordinator/🗄️event_store.go, 📤️event_export.go]
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
├── 🧪️test/                              [KEEP – already multi-part: 🏃️runner,📇️registry,📡️protocol,🖥️host,🧫️fixtures,🧬️schema,📦️packages/{🐹️go,🦀️rust}]
│   └── + add 📦️packages/🟦️typescript alongside existing 🐹️go/🦀️rust for full parity
├── 📜️statutes/                          [MOVE: 💻️client/⌨️cli/internal/{command? no} + net-new home for policy/statute/breach engine, currently scattered across 📚️library]
│   ├── 🔎️analyze/
│   ├── 🩹️fix/
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
├── 🌳️tree/                              [MOVE: repo/folder/file/section/definition tree-building, currently inside 📚️library + old ./repo/lib/js]
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
├── 🔗️graphql/                           [MOVE: 💻️client/⌨️cli/internal/graphql/🔗️graphql.go]
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
├── 📦️integrate/                         [NEW – currently no dedicated module; import/export/extract logic implicit]
│   ├── 📥️import/ ⧸ 📤️export/ ⧸ 🧲️extract/
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
├── 🚚️move/                              [MOVE: 📚️library/🧹️normalization → rename/relocate under domain-neutral "move" if it is specifically about renames; KEEP 🧹️normalization as-is if it stays admission/classification-focused — recommend KEEP normalization where it is, since it's already exemplary, and add 🚚️move as a *new* sibling for actual file/folder move operations if distinct]
├── 📊️metrics/                           [MOVE: LOC/count/tree metrics referenced in AGENTS.md §10, currently only spec'd, not modularized]
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
├── 🔌️mcp/                               [MOVE: 💻️client/⌨️cli/internal/mcp + internal/mcpserver, merge with 🔨️modules/💻️client/🔌️mcp]
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
├── ⌨️cli/                                [MOVE+MERGE: unify 💻️client/⌨️cli (Go) and top-level ⌨️cli (Rust) into ONE module]
│   ├── 🧬️schema/🔣️.json
│   ├── 🧪️tests/
│   └── 📦️packages/
│       ├── 🐹️go/            (from 💻️client/⌨️cli, cmd/repo/🐹️.go → package main entrypoint inside 🐹️go/, no more cmd/)
│       ├── 🦀️rust/           (from ⌨️cli/📦️packages/🦀️rust)
│       └── 🟦️typescript/     (from 💻️client/⌨️cli/📦️packages/🟦️typescript)
├── 🎛️dashboard/                         [MOVE: 🎮️commands/🎛️terminal-dashboard + 🖥️terminal-dashboard-daemon]
│   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/
└── 🖥️server/                            [MOVE+CLEANUP]
    ├── 🎛️coordinator/
    │   ├── 🧬️schema/  (from 🧬️g3-event-schema.json, keep postgres SQL as a 📦️packages/🐘️postgres target, not the domain folder itself)
    │   ├── 🧪️tests/
    │   ├── 🛡️durability/         [MOVE: durability_unix.go/durability_windows.go → OS-target packages under this domain folder]
    │   └── 📦️packages/{🐹️go,🦀️rust,🟦️typescript}/  (Dockerfile/Caddyfile/.env.example → move to a 🚀️deploy/ facet, not module root)
    └── 📚️library/👷️worker/       [KEEP]
```

Additional standing rules confirmed to apply when building this tree:
1. Every `📋️project.json` under a new `📦️packages/<lang>/` must call only `bun ./📜️script.ts …`; every `📜️script.ts` must extend the shared `BundleScript`/`ScriptRouter` helpers from `📚️library/📦️packages/🟦️typescript/🟦️.ts`.
2. Every module gets exactly one godfile per language at each level (`🐹️.go`/`🦀️.rs`/`🟦️.ts`), split into `#region 🔖️Name` sections; a section graduates to its own subfolder once it grows a real fixture/test/schema of its own (mirrors `mcp` and `🎮️commands/🌊️workflow`).
3. `🔣️.json` is reserved for collection-manifest/fixture files, always paired with `🧬️schema/🔣️.json`.
4. No `cmd/`, `internal/`, `.next`, `dist`, binaries, or OS-suffixed filenames (`_unix`, `_windows`) at module root — OS/build-tag splits become explicit target subfolders instead (see `mcp/🖱️ui/🖌️render/🎯️targets/{🌋️vulkan,🍎️metal,🧊️webgpu,🪟️d3d12}` as the established pattern for engine/OS-specific targets).
5. The stray root `./repo/` tree (client/cli, client.exe, lib/js, server/coordinator) is legacy pre-taxonomy content and should be deleted/absorbed into the new tree above, not left standing — per root `AGENTS.md`, "no migration scripts", hand-craft the replacement and delete the old tree in the same change.

## Files referenced (not modified)
- `🧰️framework/🛍️products/🦑️repo/AGENTS.md`, `README.md`
- `🧰️framework/🛍️products/AGENTS.md`, root `AGENTS.md`
- `🔒️dependencies.json`, `🧅️layering.json`, `🚚️migration.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/**`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/{🔏️path-emoji-statutes,🔤️taxonomy-leading-grapheme}/**`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/**` (exemplar)
- `🧰️framework/🔨️modules/🔀️dispatch/**`, `🧰️framework/🛍️products/🦑️repo/🎮️commands/🌊️workflow/🦀️.rs` (exemplars)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/{💻️client/⌨️cli,⌨️cli,🖥️server,🎮️commands}/**` (bad patterns)
- `./repo/**` (legacy root tree)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️01/☀️14/REPO-TREE-REFACTOR/ticket.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️01/☀️21/{REPO-BINARY-REFACTOR-PLAN,REPO-BINARY-CONSOLIDATION}/ticket.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️goal-facetsplit-architect-report.md`
