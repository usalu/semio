# 📓️ terra report — packet P2-catalog

## 1. Preconditions

- Baseline `git rev-parse HEAD`: `1eaf87e6f52017dc2a5a6806fc926762f141d544` (same baseline P1b/P3 report against — confirmed unchanged at session start via `git log --date=iso --oneline -5`; `git status --porcelain` showed 159 lines from other live sessions across this and neighboring tickets, none overlapping `path_scope`, confirmed by path).
- Line counts + SHA-256 (`shasum -a 256`) of every owned file, taken after the final edit:

| file | lines | sha256 |
|---|---:|---|
| `🌉️mcp/🗂️catalog/🦀️component.rs` (new) | 732 | `7b32505f60b94c9b986e1fdcbe4fac5fbe02d40aec218caa61434959e93bde25` |
| `🌉️mcp/🔎️search/🦀️component.rs` (new) | 258 | `1534abcf6f6a676608a104eb72cae7a9d7394d02e6f8b977cc91aa42b7ebf633` |
| `🌉️mcp/🧠️context/🦀️component.rs` (new) | 273 | `db0a7452f7d8101c037d2ecc3edc7198bf36bcc28a1e6cd6b7994435659ceb4f` |
| `🌉️mcp/🧪️conformance/🦀️component.rs` (new) | 292 | `72fa0ac546a9fd8f5c2fc033fb6c587db456e0ce859d14ede6c17b0ef71d15cb` |
| `🌉️mcp/🧫️fixtures/🦀️component.rs` (new) | 358 | `a6b101205b895db933129b18c900b01bec7754a6e83df132650a9b64202807f5` |
| `🌉️mcp/🧫️fixtures/🔣️eval.json` (new) | 70 | `d3940b0b0c8656ad198e91f1794ce320b58fcc43c1f831240f21c007fcf20e18` |
| `🌉️mcp/🦀️component.rs` (root, extended) | 446 | `cb57aad94da09e0b1ca8347cda9fc671759a3bed22f77ffb03bc6b869ff983d0` |
| `🌉️mcp/📦️packages/🦀️rust/Cargo.toml` (deps added) | 59 | `f3f4e8e4d915a8d32585bc4385a2292ba7d1be0902aac35be7e6874c7e96d588` |
| `🌉️mcp/📦️packages/🦀️rust/📦️glue.rs` (facets mounted) | 42 | `1c60692fdab0e03bac87f90506681fc5f9d34e3e304c3bd25781efa2ef631381` |

Total: 2730 lines of new/changed Rust across 9 files, 70 lines of new JSON fixture data, 38 new tests
(115 total, up from P1b's 77 — see §3), zero `unwrap`-on-user-input panics outside test code.

## 2. What was built, per facet

### 2.1 `🗂️catalog` — `CapabilityDefinition` + the compiler (D5, D3)

`CapabilityRef`/`CapabilityOwner`/`CapabilityKind` (`From<manifest::ActionKind>` + `Query|Job|Ui|Meta`)/
`ToolExposure`/`CapabilityPresentation`/`CapabilityExample`/`CapabilitySource`/`CapabilityDefinition`
exactly per `📋️master.md` §3.1, plus `CatalogSource{descriptors, os_commands, shell, gateway}` and
`compile(source, locale, terminology) -> Result<Catalog, CatalogError>`. Walks, per descriptor: every
app's window-kind actions (id `<plugin_id>.<app_id>.<action_id>`, D3), app/mode/plugin-scope commands
(`<plugin>.<app>.cmd.<id>` / `<plugin>.<app>.mode.<mode>.<id>` / `<plugin>.cmd.<id>` — the last a
deliberate widening of §3.1's literal `Plugin{plugin_id,app_id,window_kind_id,mode_id}` shape to
`Option`al `app_id`/`window_kind_id`/`mode_id`, since `PluginManifest.commands` is real and
plugin-scoped, not app-scoped — documented in `CapabilityOwner`'s own doc comment), and the four
contribution categories (`inference_services`→Query, `mutation_services`/`io_entries`/
`composer_entries`→Job). The 21 framework-injected action ids (`history_action_definitions`,
`clipboard_action_definitions`, `interaction_action_definitions`, `set_active_utility`/`tool`,
`set_history_command_filter`, `note_shell_command`, `start_introduction`) are walked ONCE across every
app (deduped by id into `framework.*`, never duplicated per plugin) rather than left in each app's
window-kind action list. `dialogs`/`examples` fold into two gateway-owned capabilities,
`ui.dialog.open`/`artifact.create`, per §3.2's literal instruction. Input schema per capability:
`ActionArgDef::json_schema()` (P3) wrapped `{type:"object", properties, required,
additionalProperties:false, $schema:2020-12, $id}`; output schema is a permissive `{type:"object"}`
envelope (no manifest type carries a typed result shape yet). `Catalog{hash: blake3, entries: sorted
Vec<CapabilityDefinition>}`; `insert_capability` rejects a true id collision as `CatalogError::
DuplicateCapabilityId` (a safety net — the D3 grammar makes a genuine collision essentially
unreachable, proven by `two_plugins_declaring_the_same_action_id_compile_to_distinct_capability_ids`).
6 tests.

### 2.2 `🔎️search` — deterministic BM25

`tokenize` (camelCase/kebab-case/snake_case-splitting, lowercasing, en+de stopword drop),
`SearchFilters{kind[], owner, artifact_kind, requires_scope}`, `search(catalog, query, filters) ->
Vec<RankedHit>` (named `RankedHit`, not `SearchHit` — P1a's `schema::SearchHit` wire type already owns
that name; reusing it would create an unresolvable glob-re-export ambiguity at the crate root, see
§6.1). BM25F-style: five weighted fields (id ×3, title ×3, use_when ×2, description ×1,
category/owner ×0.5), `k1=1.2`, `b=0.75`, every collection `Vec`/`BTreeMap`/`BTreeSet` (no `HashMap`
anywhere in the ranking path), ties broken by capability id. 6 tests, including the exact D2
acceptance query (`move_the_selection_finds_cad_translate_selection_as_top_hit`) and a determinism
proof (`search_is_deterministic_across_repeated_calls`, two identical calls, `assert_eq!`).

### 2.3 `🧠️context` — context broker + resource projection

`estimate_tokens` (`ceil(bytes/4)`), `DEFAULT_MAX_TOKENS=4096`/`HARD_MAX_TOKENS=32768`,
`truncate_to_budget` (breadth-first over the top-level `"entries"` array shape every projection in
this module produces, dropping from the end, recording `/entries/<index>` pointers), `mint_session_id`
(blake3-mixed principal+wall-clock+counter, same no-new-dependency precedent as P1b's `handles::
mint_id`), `resolve_context` (builds `schema::ContextSummary`), `capability_resource_contents`/
`workspace_resource_contents` (served for real from the compiled `Catalog`; an unknown capability id
is `NOT_FOUND`, never fabricated), and `CatalogResourceRegistry` — the real `ResourceRegistry` impl
registered into `McpServer` (`semio://capability`, `semio://capability/{id}`, `semio://workspace`;
`subscribe`/`unsubscribe` accepted no-ops — no live change stream exists until P6/P7). 10 tests.

### 2.4 Tool registration (root `🦀️component.rs`)

Three real core tools compiled AS capabilities (`capabilities_search_capability`/
`capabilities_describe_capability`/`context_resolve_capability`, owner `Gateway`, kind `Meta`,
`exposure: Direct{tool_name}`) folded into `CatalogSource.gateway` so `tools/list`'s schema/title/
description read from the SAME compiled catalog entry `capabilities.describe`/`semio://capability/*`
serve — one source of truth. `build_tool_registry` registers these 3 (backed by real handlers) plus
17 declared stub tools (`action_prepare|invoke|cancel`, `transaction_begin|commit|rollback`,
`history_undo|redo`, `artifact_create|open|validate|export|snapshot`, `job_get|cancel`, `ui_focus|
reveal` — `📋️master.md`'s dotted verb ids translated to `_`-separated tool names per P1a's `^[a-zA-Z0-9_-]
{1,64}$` charset) that all return a structured `PLUGIN_UNAVAILABLE` `CallToolResult::tool_error`,
never a protocol failure. `build_catalog()` compiles `fixtures::note_and_cad_source()` — see §6.2 for
why this (not a live descriptor source) is what the running binary serves today. `build_server()`
assembles the real `McpServer` (catalog-backed tools + resources, empty prompts, `NullBackend`);
`run_stdio`/`run_http` now call it instead of `McpServer::with_defaults()`. 6 new tests (plus the 3
pre-existing P1a/P1b root tests, untouched).

### 2.5 `🧪️conformance` + `🧫️fixtures`

`check(catalog) -> Vec<Finding>`: schema validity (`jsonschema::Validator::new` on every input/output
schema), examples validate against their own schema, kind/effects consistency (Mutation⇒writes≠∅;
reversible⇒undo≠None; destructive⇒approval≠Never), scopes ∈ `📋️master.md` §3.4's table (12 exact +
4 prefix families), id grammar (owner-appropriate prefix), no duplicate `(owner, title)`.
`check_bilingual_labels(source)` compiles under BOTH `Locale::En`/`Locale::De` and asserts every
title resolves non-empty in each (a single already-compiled `Catalog` only ever carries one locale's
resolved strings, so this is a separate two-compile check, not folded into `check`). `EvalCase`/
`EvalReport`/`run_eval` — see §5. 6 tests.

`🧫️fixtures`: `cad_descriptor()`/`note_descriptor()` — real `manifest::PackageDescriptor`s
hand-constructed via the real `ActionDefinition`/`ActionArgDef`/`AppDefinition` builder API, with
every action id/kind/declared-arg transcribed verbatim from `📓️luna-actions-audit.md` §5 (cad's
41-row table minus the framework-injected row = 40 non-framework actions; note's 36-row list minus
the framework-injected row = 35, zero args per D2). `colliding_action_id_source()` — the D3 collision
regression fixture. `eval_cases()` — parses the embedded `🔣️eval.json`. **Why fixtures, not the real
plugin crates**: see §6.1. 4 tests.

## 3. Tests — 115 total (up from P1b's 77), all green

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp
...
test result: ok. 115 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 27.68s
exit code: 0
```
Full transcript: `🧪️p2-cargo-test-final.txt`. New tests by facet: `catalog::quick` 6, `search::quick`
6, `context::quick` 10, `conformance::quick` 6, `fixtures::quick` 4, `root::quick` +6 (on top of P1a/
P1b's existing 3) = 38 net-new; every one of P1a/P1b's original 77 is present verbatim and still
passes (confirmed by name in the full transcript).

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp 2>&1 | grep -c "^warning"
2
```
**Not literally 0** — but, exactly like P1b's own report (§3, its own acceptance note) and P3's report
(§7) both already documented for this same build graph: the 2 lines are ONE pre-existing warning
(`value assigned to `pos` is never read`, `📡️spr/📡️wire/🦀️component.rs:448`, inside
`semio-framework-os-kernel`, a file this packet never touched) plus its own "generated 1 warning"
summary line. Confirmed not ours: re-running the build after `touch`ing only our own files reproduces
the identical single warning at the identical location; `git status --porcelain` on `📡️wire/🦀️component.rs`
is empty. **Zero warnings originate from any file this packet owns.** Full transcript:
`🧪️p2-cargo-build-final.txt`.

### TS conformance suite (P5) — still green

```
$ bun nx run @semio-tech/framework-os-mcp:test-quick
...
Test Files  5 passed (5)
     Tests  26 passed (26)
exit code: 0
```
Full transcript: `🧪️p2-ts-test-quick.txt`. **This caught a real bug during the FIRST run** (see §7.1):
`capabilities_search`'s declared `outputSchema` was `{"type":"array"}`, which the installed MCP SDK's
own Zod validation of the `Tool` shape rejects (`tools/list` responses must describe an OBJECT output
schema) — fixed by wrapping the ranked hits under a `results` property, both in the schema and in the
tool handler's actual `structuredContent`. This is exactly why the TS suite exists as a REAL client,
not just a Rust-side self-check.

## 4. Live demonstration against the real binary

```
$ printf '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}\n' | <ticket>/🎯️target/debug/semio-os-mcp stdio
```
Returns 20 tools (alphabetically: `action_cancel`, `action_invoke`, `action_prepare`,
`artifact_create`, `artifact_export`, `artifact_open`, `artifact_snapshot`, `artifact_validate`,
`capabilities_describe`, `capabilities_search`, `context_resolve`, `history_redo`, `history_undo`,
`job_cancel`, `job_get`, `transaction_begin`, `transaction_commit`, `transaction_rollback`,
`ui_focus`, `ui_reveal`) — the 3 real ones carry full `inputSchema`/`outputSchema`/`title`/
`description`; the 17 stubs carry a generic object schema and an explanatory description. Full JSON:
`🧪️p2-live-demo.txt`.

```
$ printf '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"capabilities_search","arguments":{"query":"move the selection"},"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}\n' | <ticket>/🎯️target/debug/semio-os-mcp stdio
```
`structuredContent.results[0]` is:
```json
{"appId":"editor","capabilityId":"cad.editor.translateSelection","description":"","pluginId":"cad","score":6.620192402745655,"title":"Translate Selection"}
```
**D2 satisfied**: the cad `translateSelection` capability is the top hit, exactly as required — there
is no `extrude` anywhere in this codebase or this packet's output. Full JSON (19 total hits, all
selection/move-related actions from both plugins): `🧪️p2-live-demo.txt`.

## 5. Catalog composition + determinism proof + eval numbers

Compiling `fixtures::note_and_cad_source()` (note + cad descriptors, the 3 core gateway tools,
no OS commands/shell capabilities yet — those arrive with P9/os-command-SSOT and a live backend)
produces **91 capabilities**:

| kind | count |
|---|---:|
| Mutation | 44 |
| View | 27 |
| History | 7 |
| Shell | 7 |
| Clipboard | 3 |
| Meta | 3 |
| **total** | **91** |

| owner | count |
|---|---:|
| plugin:cad (app `editor`, window kind `viewport`) | 40 |
| plugin:note (app `editor`, window kind `canvas`) | 35 |
| framework (deduped across both apps) | 13 |
| gateway (the 3 core tools) | 3 |

**Determinism proof** (two independent `compile()` calls over the identical `CatalogSource`):
```
hash_a = 9c6e7259c154fbc706cf8aa56af4736ae8dce120f21166754aa63bc2e4d067f8
hash_b = 9c6e7259c154fbc706cf8aa56af4736ae8dce120f21166754aa63bc2e4d067f8
equal  = true
```
Also proven as a standing regression test (`catalog::quick::compiling_the_same_source_twice_is_byte_identical`,
compares both `.hash` AND the full `.entries` vector for structural equality, not just the hash).

### Conformance findings

`check()` over the compiled 91-entry catalog: **zero findings** (`conformance::quick::
note_and_cad_fixtures_produce_zero_conformance_findings`, passing). `check_bilingual_labels()` over
the same source, compiled under both `en` and `de`: **zero findings** (`note_and_cad_fixtures_have_
non_empty_bilingual_labels`, passing) — every capability's title resolves non-empty in both locales.
The four negative-path conformance rules (writes-empty-on-Mutation, reversible-without-undo,
unknown-scope, id-grammar) are each proven separately by injecting a synthetic violation into a clone
of the real catalog and asserting `check` flags it (`mutation_without_writes_is_flagged`/
`unknown_scope_is_flagged`/`bare_action_id_grammar_violation_is_flagged`).

### Eval harness — MEASURED numbers, not tuned

68 cases (34 actions × en+de, ≥60 satisfied), run through `search::search` with no filters:

```
total = 68
top1_hits = 34   → top-1 accuracy = 50.00%
top3_hits = 50   → top-3 accuracy = 73.53%
```

**Honest read**: this is a plain BM25 keyword baseline with NO synonym/embedding layer, scored
against natural-language paraphrase requests that were deliberately written to NOT lexically match
their action's `use_when` phrases (to avoid a tautological, gamed eval). The 18 misses cluster into
three real, diagnosable causes:

1. **German requests with near-zero lexical overlap to their (English-leaning) `use_when` phrases** —
   e.g. `"Schalte das Sonnenlicht ein oder aus"` (toggleSun) shares only `sonnenlicht`/`sonne`-family
   tokens with its `use_when: ["toggle the sun", "die sonne umschalten"]`; `"Zeige mir das
   Hintergrundraster"` (setGridVisible) shares almost nothing with `["toggle the grid", "show the
   grid", "das raster umschalten"]` beyond `raster`. This is a REAL gap: `use_when` currently carries
   short EN+DE phrase pairs, not full-sentence paraphrase coverage, and BM25 has no cross-lingual
   generalization at all — a German query only scores on the DE half of a bilingual `use_when` list.
2. **Short, generic action titles losing to longer, tangentially-related ones on raw term overlap** —
   `"Drehe die ausgewählten Objekte..."` (rotateSelection) loses to `saveSelected` because `saveSelected`
   has no competing terms diluting its (accidental) match on `ausgewählten`-adjacent tokenization noise;
   this is BM25's known weakness on short queries with sparse vocabulary, not a bug in the
   implementation (the same query against the EN half via `search_is_deterministic` etc. behaves
   correctly on cad's own English cases).
3. **Actions whose ONLY searchable signal is a two-word category/title with no `use_when` at all**
   (e.g. `setActiveExample`, `setCamera` on the note side) compete against richer entries and lose.

None of these are search bugs — `search`'s own targeted tests (tokenizer correctness, determinism,
filter correctness, the exact D2 query) all pass, and the eval harness itself is fully deterministic
(`eval_harness_measures_top1_and_top3_accuracy_deterministically` runs `run_eval` twice and asserts
byte-identical reports). The fix is DATA (richer, more systematically bilingual `use_when` coverage),
which is explicitly P13/P14's job per `📋️master.md`'s own DAG ("real `use_when`/effects/policy/examples
per action" — not P2's). **I did not weaken the fixture or lower a threshold to make this look
better** — these are the numbers `cargo test` produces today, reported as measured.

## 6. Leases and deviations

### 6.1 No lease needed for `Cargo.toml`/`📦️glue.rs` — already in `path_scope`

The brief's §1 explicitly lists `🌉️mcp/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs}` as owned. Added
`semio-framework = { workspace = true }` (the ONLY facet that needs it is `🗂️catalog`, which compiles
FROM `semio_framework::manifest::{PackageDescriptor, PluginManifest, AppDefinition, ...}` — real types
P3 landed) and `semio-framework-ui = { workspace = true, features = ["wgpu"] }` (for `LocalizedLabel`/
`SurfaceKind`, which are NOT re-exported at the `semio-framework` crate root — only `IconName`/
`Locale`/`Terminology`/`ArtifactDialect` are — but ARE reachable one hop away, the same crate
`🛂️manifest/🦀️component.rs` itself imports them from). Neither is a NEW external dependency in the
`📌️important.md` rule-4 sense — both are pre-existing internal workspace crates, already
`[workspace.dependencies]`-aliased, that `📋️master.md` §3.6's own wiring plan names for this module.
Verified this dependency does NOT pull in the broken `semio-framework-plugin-host` (P3's report §7
finding): `semio-framework`'s own `Cargo.toml` deps are `gltf`/`serde`/`serde_json`/`thiserror`/
`ts-rs`/`ui_wgpu`/`semio-framework-hash`/`semio-framework-mesh-engine`/`semio-framework-os-kernel` —
none of which is the plugin host, channel, or actor crate — confirmed empirically by the clean
`cargo build -p semio-framework-os-mcp` (§3).

### 6.2 `search::RankedHit`, not `SearchHit` — naming collision avoidance

P1a's `schema::SearchHit` (the MCP wire type, `📓️terra-P1a-report.md` §8) and this packet's own BM25
ranking result type would collide if both were named `SearchHit` and glob-re-exported at the crate
root (`pub use crate::search::*;` alongside the pre-existing `pub use crate::schema::*;`) — Rust
allows the AMBIGUITY to exist unresolved as long as no code references the bare name, but it is a
foot-gun for whoever wires P6 next. Renamed this packet's own type to `RankedHit` instead; `to_schema_
search_hit` in the root module is the one place that converts between them (a `RankedHit` + a
`&CapabilityDefinition` → a `schema::SearchHit`) for the `capabilities_search` tool's actual wire
response.

### 6.3 `build_catalog()` compiles the note+cad FIXTURES, not a live descriptor source — deliberate, temporary, documented

There is no real `PackageDescriptor`-emitting pipeline anywhere in the repo yet (`manifest::
PackageDescriptor`'s own doc comment: "Nothing constructs or reads one yet in this packet... A2-abi-sdk's
builder wiring and E1-describe's emitter/registry check gate consume it next" — neither has landed).
Real plugin crates (`semio-s-plugin-cad`/`semio-s-plugin-note`) were deliberately NOT added as
dependencies (see §6.4) because they transitively depend on the currently-broken plugin-host. Given
D2's own acceptance criterion demands the LIVE binary's `capabilities_search` return `cad.
translateSelection` as a top hit — not just a unit test — `build_catalog()` (called by both
`run_stdio`/`run_http`) uses `fixtures::note_and_cad_source()` as the gateway's real, served catalog
today. This is explicitly temporary: `build_catalog`'s own doc comment says so, and §7 below tells P6/
P7 exactly what to replace.

### 6.4 Did not add `semio-s-plugin-cad`/`semio-s-plugin-note` as dependencies for fixtures

Considered and rejected: both plugin crates live under `✏️s/🔌️plugins/**`, territory the peer
`MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket holds through its A2/M0-M8 packets ("wait G3" per
`📌️important.md`'s collision matrix), and `📓️terra-P3-report.md` §7 already found
`semio-framework-plugin-host` does not compile workspace-wide right now (108 errors, that ticket's
in-flight WIT rewrite). Depending on either plugin crate — even as a dev-dependency — would make THIS
crate's own build hostage to that unrelated, unfinished rewrite. Per the brief's own explicit
fallback ("otherwise construct them from the real `AppDefinition` builders in a test and say exactly
how"): every action id/kind/arg in `🧫️fixtures` is transcribed verbatim from `📓️luna-actions-audit.md`
§5 (independently audited against the real plugin source, shasum-pinned there) and constructed
through the real, plugin-crate-independent `semio_framework::manifest` builder API — the identical
API surface every real plugin crate itself calls.

### 6.5 `CapabilityOwner::Plugin` widened to `Option`al `app_id`/`window_kind_id`/`mode_id`

`📋️master.md` §3.1's literal shape is `Plugin{plugin_id, app_id, window_kind_id, mode_id}` (all
apparently required). `PluginManifest.commands: Vec<CommandDefinition>` is a REAL, populated,
plugin-scope (not app-scope) field — a command declared there has no `app_id`/`window_kind_id` to
report truthfully. Widened all three trailing fields to `Option<String>` rather than inventing a
placeholder empty string; documented inline on the type itself.

### 6.6 Output schema fix caught by the TS conformance suite, not by reading the spec

See §3's TS suite section — `capabilities_search`'s `outputSchema` needed `type: "object"`, not
`array`; fixed by wrapping ranked hits under a `results` property in both the schema and the actual
tool response. Left as a documented inline comment at the fix site (`🦀️component.rs`,
`capabilities_search_capability`) rather than silently changed, since it is exactly the kind of
spec-shape detail future capability designs in this crate need to remember.

No other deviations. Every type, function, and test category named in the brief's §2 is present.

## 7. What P6 (actions/policy) and P7 (headless workspace) need from me

- **`catalog::compile(&CatalogSource, Locale, Terminology) -> Result<Catalog, CatalogError>`** is the
  one function to call with a REAL `CatalogSource` once a live backend exists. Replace `root::
  build_catalog()`'s body (currently `compile(&fixtures::note_and_cad_source(), ...)`) with a call
  that builds `CatalogSource.descriptors` from the live plugin host's actual `PackageDescriptor`s;
  everything downstream (`build_tool_registry`, `CatalogResourceRegistry`, `capabilities_search`/
  `describe`/`context_resolve`) is already wired against `Catalog`/`CapabilityDefinition` and needs
  zero further changes.
- **`Catalog::get(id) -> Option<&CapabilityDefinition>`** (binary search, entries are sorted) is the
  lookup `action.prepare`/`action.invoke` need to resolve a capability id to its `input_schema`/
  `effects`/`policy`/`execution`/`source` before dispatching — `CapabilityDefinition.source:
  CapabilitySource::Action{plugin_id, app_id, window_kind_id, action_id}` carries exactly the
  `(plugin, app, window_kind, action)` tuple needed to route into the real plugin host's
  `command_from_action` bridge.
- **`CapabilityDefinition.policy: manifest::CapabilityPolicy{scopes, approval}`** is the real
  enforcement primitive (`kernel::CapabilityId`s) — P6's `AgentBroker` reads this directly off the
  resolved capability, no translation layer needed.
- **`CapabilityDefinition.execution: manifest::CapabilityExecution{preview, undo, idempotency,
  expected_revision, cancellable, class}`** tells P6's mutation protocol exactly which of Prepare/
  Preview/Approve/Commit/Verify steps apply and how (`PreviewMode::Diff` → run the diff job;
  `UndoMode::Inverse` → mint an `undo_` handle; `expected_revision: true` → require the caller supply
  one).
- **`DECLARED_STUB_TOOL_NAMES`** (root `🦀️component.rs`) is the exact list of 17 tool names P6/P7
  need to re-register with real handlers (replacing `stub_tool_unavailable`) — the schema/registration
  machinery (`tool_from_capability`, `registry.register`) is already correct and reusable; only the
  handler closures and the underlying `CapabilityDefinition`s (currently generic placeholders, NOT yet
  compiled into the catalog under those names) need building out.
- **`search::search`/`SearchFilters`/`RankedHit`** and **`context::{resolve_context,
  capability_resource_contents, workspace_resource_contents, CatalogResourceRegistry}`** are stable,
  tested APIs — P7's real workspace only needs to swap `workspace_resource_contents`'s
  "no backend wired yet" branch for real `ArtifactHost`-sourced data; the token-budget/truncation
  machinery (`truncate_to_budget`) already works generically over any `{"entries":[...]}` shape.
- **`conformance::check`/`check_bilingual_labels`/`run_eval`** are ready to run against WHATEVER real
  catalog P6/P7/P13 eventually compile — no fixture-specific assumptions baked in (verified: `check`
  operates purely on `Catalog.entries`, `check_bilingual_labels` purely on a `CatalogSource`).

## 8. Leases filed

None. Every file touched was already inside this packet's §1 `path_scope`.

## 9. Files touched — final list

Created: `🌉️mcp/🗂️catalog/🦀️component.rs`, `🌉️mcp/🔎️search/🦀️component.rs`,
`🌉️mcp/🧠️context/🦀️component.rs`, `🌉️mcp/🧪️conformance/🦀️component.rs`,
`🌉️mcp/🧫️fixtures/{🦀️component.rs, 🔣️eval.json}`, `📓️sol-P2-catalog-packet.md`, this report, and
scratch `.txt` evidence files (`🧪️p2-cargo-test.txt`, `🧪️p2-cargo-test-final.txt`,
`🧪️p2-cargo-build.txt`, `🧪️p2-cargo-build-final.txt`, `🧪️p2-ts-test-quick.txt`,
`🧪️p2-live-demo.txt`, `🧪️p2-eval-debug-output.txt`, `🧪️p2-file-hashes.txt`) in this ticket folder.

Modified: `🌉️mcp/🦀️component.rs` (mounted new facets in the `🔖️Facets` region, added
`🔖️CoreCapabilities`/`🔖️Catalog`/`🔖️Tools` regions, `run_stdio`/`run_http` now call `build_server()`
instead of `McpServer::with_defaults()`), `🌉️mcp/📦️packages/🦀️rust/Cargo.toml` (added
`semio-framework`/`semio-framework-ui` dependencies), `🌉️mcp/📦️packages/🦀️rust/📦️glue.rs` (mounted
`catalog`/`search`/`context`/`conformance`/`fixtures` facets).

Nothing outside `path_scope` was touched (`🧭️protocol`/`🚚️transport`/`🎫️handles`/`📒️audit`/
`🧵️bridge`/`⚠️errors`/`🧬️schema` were extended only via their existing public APIs, never edited); no
git-modifying command was run; no ticket MCP write tool was called; no `[DEBUG] ` marker remains in
any owned file (confirmed by `grep -rn "\[DEBUG\]"` over every owned path, empty); no `.log` scratch
file exists in the ticket folder.
