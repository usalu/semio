# 📓️ sol packet brief — P2-catalog (verbatim)

You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in /Users/ueli/Documents/semio. Packet id: **P2-catalog**. Model: Sonnet 5. Coordinator ("sol") is the main chat.

## 0. First action
Read in full: `…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md`; `…/📓️design-decisions.md` (**D2, D3, D5 are your charter**); `…/📓️luna-actions-audit.md` (the census of everything you must catalog); `…/📓️terra-P1a-report.md` + `…/📓️terra-P1b-report.md` (the crate you extend and its published API); `…/📓️terra-P3-report.md` (the `ArgSchema`/`ActionSemantics` types you compile from — **already landed and green**); `📋️master.md` §3.1–3.2; `/Users/ueli/Documents/semio/CLAUDE.md`.
Save this brief verbatim as `…/📓️sol-P2-catalog-packet.md`.

Current state you build on: the crate `semio-framework-os-mcp` is a workspace member, serves both MCP eras over stdio and Streamable HTTP, has a handle table, an audit lane and a shell-bridge codec, and passes 77 Rust tests + 26 TS conformance tests. It registers **zero tools** — that is exactly what you fix.

## 1. Owned writable paths (EXCLUSIVE)
```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️component.rs      (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔎️search/🦀️component.rs       (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧠️context/🦀️component.rs      (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️conformance/🦀️component.rs  (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/**                 (new — descriptors + evals)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️component.rs                (mount new facets + register the core tools)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs}
.🧬semio/…/📓️sol-P2-catalog-packet.md, 📓️terra-P2-report.md, 📓️lease-P2-*.md, *.txt scratch
```
Do NOT edit `🧭️protocol`, `🚚️transport`, `🎫️handles`, `📒️audit`, `🧵️bridge`, `⚠️errors`, `🧬️schema` (other packets own them — extend via their public APIs), and nothing outside `🌉️mcp`.

## 2. Required result

### 2.1 `CapabilityDefinition` + the catalog compiler (`🗂️catalog`)
Per **D5** these types live in THIS crate, not in `🛂️manifest`. Implement `CapabilityRef`, `CapabilityOwner`, `CapabilityKind` (`From<manifest::ActionKind>` + `Query|Job|Ui|Meta`), `ToolExposure`, `CapabilityPresentation`, `CapabilityExample`, `CapabilityDefinition` and `CapabilitySource` exactly as `📋️master.md` §3.1 specifies, then:
```rust
pub struct CatalogSource { pub descriptors: Vec<PackageDescriptor>, pub os_commands: Vec<CommandDefinition>, pub shell: Vec<CapabilityDefinition>, pub gateway: Vec<CapabilityDefinition> }
pub fn compile(source: &CatalogSource, locale: Locale, terminology: Terminology) -> Result<Catalog, CatalogError>;
```
Sources to walk (all from `PackageDescriptor`/`PluginManifest`/`AppDefinition`, which are real types in `semio-framework`): per-app per-window-kind `actions`, app/mode/plugin `commands`, the framework-injected action sets (`history_action_definitions`, `clipboard_action_definitions`, `interaction_action_definitions`, active-utility/tool — dedup into `framework.*`), contributions (inference/mutation/io/composer → `Query`/`Job` capabilities), `examples` → the `artifact.create` template enum, `dialogs` → `ui.dialog.open`, plus the OS commands under `💻️os/🎮️commands`.
**Id grammar is mandatory (D3)**: `<plugin_id>.<app_id>.<action_id>`, `framework.*`, `os.*`, `ui.*`. 14 action ids are declared by more than one plugin, so a bare action id is never a capability id — add a test that compiles two plugins declaring the same action id and asserts distinct capability ids.
Input schema per capability comes from `ActionArgDef::json_schema()` (landed by P3) wrapped as `{type:"object", properties, required, additionalProperties:false}` with `$schema` 2020-12 and `$id: semio://capability/<id>/input`; effects/policy/execution come from `ActionDefinition.semantics`.
`Catalog` carries a **blake3 `hash`** and its entries are **sorted** — compiling twice must produce byte-identical output. Prove it in a test.

### 2.2 Deterministic search (`🔎️search`)
BM25 (k1 1.2, b 0.75) over id ×3, title ×3, use_when ×2, description ×1, category/owner ×0.5; tokenizer splits camelCase and kebab-case, lowercases, drops en+de stopwords; filters `kind[]`, `owner`, `artifactKind`, `requiresScope`; ties broken by capability id so results are stable. **No LLM, no randomness, no HashMap iteration order anywhere in the output.**

### 2.3 Context broker + resource projection (`🧠️context`)
`context.resolve` returning the token-cheap `ContextSummary` (from P1a's `🧬️schema`), and the `semio://capability[/{id}]` + `semio://workspace` resources. Token estimate `ceil(bytes/4)`, `maxTokens` default 4096 / hard cap 32768, breadth-first truncation recording `omitted` pointers. With the `NullBackend` there is no live workspace yet — serve the **catalog-derived** resources for real and return well-formed `NOT_FOUND`/empty for the live ones; do not fake workspace data.

### 2.4 Register the real tools
Register into P1a's `ToolRegistry` the discovery/meta tools that need no OS backend: `capabilities_search`, `capabilities_describe`, `context_resolve`. Wire the remaining core names (`action_prepare`, `action_invoke`, `artifact_*`, `history_*`, `job_*`, `ui_*`) as **declared tools that return a structured `PLUGIN_UNAVAILABLE` tool-error** until P6/P7 land — so `tools/list` is already the real, stable surface and P5's suite can finally exercise the tool-error path. Every tool name must satisfy `^[a-zA-Z0-9_-]{1,64}$` (P1a enforces it — do not fight it, name accordingly).

### 2.5 Conformance runner + eval fixtures (`🧪️conformance`, `🧫️fixtures`)
`pub fn check(catalog: &Catalog) -> Vec<Finding>`: every input/output schema is valid 2020-12 (`jsonschema::Validator::new`); examples validate against their own schema; kind/effects consistency (Mutation ⇒ writes non-empty; reversible ⇒ undo ≠ None; destructive ⇒ approval ≠ Never); scopes are known; id grammar holds; en+de labels non-empty; no duplicate (owner, title).
Fixtures: capture **real** `PackageDescriptor`/manifest data for `🗒️note` and `📐️cad` into `🧫️fixtures/📇️descriptors/` (generate them from the actual crates if you can do so without editing plugin code; otherwise construct them from the real `AppDefinition` builders in a test and say exactly how). Plus `🧫️fixtures/🔣️eval.json`: **≥60** natural-language requests (English and German, per CLAUDE.md's en-first/de-second rule) each mapping to an expected capability id, drawn from the real cad/note action lists in the actions audit. Add an eval runner reporting top-1/top-3 accuracy; **record the numbers you actually measure — do not tune the thresholds to whatever you happen to get, and do not weaken a fixture to make a score look better.** If accuracy is poor, report it as a finding with your diagnosis.

## 3. Acceptance (FOREGROUND ONLY, paste output + exit codes)
```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp
CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp 2>&1 | grep -c "^warning"    # → 0
bun nx run @semio-tech/framework-os-mcp:test-quick     # P5's conformance suite must still pass
```
plus a live demonstration against the real binary, pasted verbatim:
```
printf '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}\n' | <ticket>/🎯️target/debug/semio-os-mcp stdio
printf '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"capabilities_search","arguments":{"query":"move the selection"},"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}\n' | <ticket>/🎯️target/debug/semio-os-mcp stdio
```
The second one must return the cad `translateSelection` capability as a top hit (**D2** — there is no `extrude` in this codebase; if you find yourself writing "extrude" anywhere, you are working from the stale master plan). All of P1a/P1b's 77 tests must still pass.

## 4. Hard rules
All of `📌️important.md`: no git-modifying commands; no ticket MCP write tools; nothing outside §1 (lease instead); **never background a build or a test** (no `&`, no `run_in_background`, no poll loop — use a long foreground timeout); scratch `.txt`/`.md`/`.json` in the ticket folder only, never `.log`; `[DEBUG] ` prefix removed before done; never claim a result you did not run and paste; never edit `AGENTS.md`; no compat shims or deprecations. Add no new external crates without a lease. Docstrings start with a unique emoji; no comments inside definitions; `//#region` structure.
Note: `cargo check --workspace` currently fails inside `🔌️plugin/🖥️host` — that is the peer ticket's in-flight WIT rewrite, not yours. Do not attempt to fix it and do not let it block you; your crate does not depend on it.

## 5. Report
`…/📓️terra-P2-report.md`: baseline HEAD + SHA-256s + line counts; how many capabilities compiled from the fixtures and their breakdown by kind/owner; the determinism proof (two hashes); the conformance findings; the **measured** eval top-1/top-3 with your honest read of them; the live tool-call transcripts; leases; and a "what P6 needs from me" section describing the exact catalog API the action adapter will consume.
</content>
