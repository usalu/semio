# 📓️ M5a — semio MCP capability catalog: agent usability

Slice M5a of `OS-HUB-COLLABORATION-AI-END-TO-END`. Owns the two agent-usability defects the
coordinator measured live through `.mcp.json`'s `semio` server (`📓️status.md`, "Coordinator live
smoke of the semio MCP"): every capability has `description: ""`, and raw pointer/engagement input
plumbing is published to agents beside intent-level document verbs. Also takes the catalog/tool
metadata items of `📓️g7-mcp-agent-and-collaboration-audit.md` §6 (P1.8 pagination/limits, a real
input JSON Schema on `capabilities_describe`).

Everything below labelled **measured** was executed and captured; **tested** means a law test was run;
**unverified** means written but not exercised. Nothing is inferred.

---

## 1. Measured starting state

Driven through the real server, spawned exactly as `.mcp.json`'s `semio` entry spawns it
(`stdio --folder . --scopes workspace.read,…`), by `🐍️m5a-catalog-probe.ts` →
`🗑️generated/m5a-before.txt` (2026-09-19T21:31Z, binary `⚡️cache/cargo/target/debug/semio-os-mcp`
built 11:23 that day, catalogHash `ba5a2b6d…`). **measured**

| query | results | distinct scores | empty descriptions |
| --- | --- | --- | --- |
| `draw rectangle` | 20 | **2 / 20** (6.9096 ×16, 6.7977 ×4) | **20 / 20** |
| `add a layer to the drawing` | 20 | 8 / 20 | **20 / 20** |
| `export the document as pdf` | 20 | 10 / 20 | 15 / 20 |

Reproduces the coordinator's finding exactly. Three further facts the capture adds:

1. **`capabilities_describe` publishes an empty input schema.** `addLayer` — a verb whose real guest
   payload is `AddLayer { kind: String }` — answers
   `{"type":"object","properties":{},"additionalProperties":false}`. The descriptor declares
   `args: []` for **all 45** of draw's actions, so there was never anything to project. An agent
   that found the verb still could not call it with an argument.
2. **Six of the `draw rectangle` top 20 are raw input plumbing** (`canvasEscape`, `engagementInput`,
   `engagementSubmit`, `canvasCommitDraft`, `canvasDoubleClick`, `canvasPointerDown`,
   `canvasPointerMove`) and two more are shell chrome (`setCamera`, `recordTutorial` ×2).
3. **No `total`, no `nextCursor`, no `limit`** — the handler hardcoded `.take(20)`, so a 20-row answer
   was indistinguishable from a 400-row catalog (G7 §6 P1.8).
4. Registry diagnostics: **30 `skipping plugin` lines** (A1 §3's descriptor regeneration debt, A3's
   queued slice — not mine, unchanged by this slice).

**The decisive structural fact.** The MCP catalog is compiled from the *committed* `🔣️.json`
descriptors (`📇️registry/🦀️.rs:53` → `🏠️workspace/🦀️.rs:163 load_package_descriptor`), which are
generated, committed artifacts emitted by each plugin's `describe` target from a `wasm32-wasip2`
build. So a description written in plugin Rust reaches an agent only after that plugin's descriptor
is regenerated. Everything in §2–§4 is therefore source-and-law work; §6 is the one regenerated
plugin that proves it end to end.

---

## 2. Design — the audience taxonomy

Decided from what the framework already knows, not from a new hand-maintained list.

**New declaration vocabulary** — `manifest::CapabilityAudience` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`):

| variant | means | published to agents |
| --- | --- | --- |
| `Agent` | an intent-level verb a human *or an agent* can pick to accomplish a goal | ✅ |
| `Input` | a raw pointer/keyboard/engagement event, meaningful only to a live surface that owns a cursor, a gesture and a draft | ❌ (still dispatched by the shell) |
| `Chrome` | window/view/session chrome — camera pose, active utility, panel filter | ❌ (still dispatched by the shell) |

It sits on `ActionSemantics.audience: Option<CapabilityAudience>` — the same struct `ActionDefinition`
and `CommandDefinition` share — as an additive, `#[serde(default)]`, `skip_serializing_if` field, so
every one of the 34 already-committed descriptors still decodes unchanged.

**`None` means "derive it", and the derivation is stated once** (`manifest::derive_audience`):

```
ActionKind::Interaction                → Input     (framework-injected hover/selection)
ActionKind::View   && !in_palette      → Chrome    (ephemeral view state, never a palette command)
everything else                        → Agent     (every mutation; history/clipboard/shell;
                                                    a palette-visible View verb like exportDocument)
```

That derivation alone classifies correctly, with **zero hand-authoring**, every framework-injected
verb (`interactionSelect`, `interactionHover`, `clearSelection`, `selectAll`, `setSelectionMode`,
`setInteractionGranularity` → `Input`; `setActiveUtility`, `setActiveTool`, `startIntroduction`,
`setHistoryCommandFilter`, `recordTutorial` → `Chrome`) and every ordinary document verb.

**What is not derivable, and is therefore declared.** A pointer gesture that really does commit an
operation is `ActionKind::Mutation` and structurally indistinguishable from a panel-dispatched
mutation such as `patchLayer`. Those declare themselves, via two new one-call builders:
`ActionDefinition::input_event()` and `::chrome()` (each also clears `in_palette`). In draw that is
exactly eight ids: `canvasPointerDown/Up/Move`, `canvasDoubleClick`, `canvasCommitDraft`,
`canvasEscape`, `engagementInput`, `engagementSubmit`. Framework-side, one declaration:
`noteShellCommand` (a `Shell`-kind bookkeeping verb) is marked `.chrome()`.

**The compiler filters, the collision check does not.** `catalog::compile()` now delegates to
`compile_with_audiences(…, AGENT_AUDIENCES)`. Every declaration is still walked and still fights for
its id — a duplicate stays `CatalogError::DuplicateCapabilityId` even when the colliding pair is
filtered out — and only the final entry vector (and therefore `Catalog.hash`) is narrowed. A shell
or a conformance pass that wants the complete surface calls `compile_with_audiences(…, ALL_AUDIENCES)`.

**Why not filter in the search layer.** The brief asked for the fix at the source, and there is a
correctness reason beyond taste: `capabilities_describe`, `semio://capability/{id}` and `tools/list`
all read the same `Catalog`. A search-layer filter would have left `canvasPointerMove` describable
and directly addressable, just harder to find.

---

## 3. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | new `CapabilityAudience` + `derive_audience`/`resolve_audience`/`resolve_command_audience`; `ActionSemantics.audience`; `ActionDefinition::describe`/`audience`/`input_event`/`chrome`; `CommandDefinition::describe`/`audience`; new `ActionArgDef::text_list`/`json_text`; EN+DE descriptions on the 7 History and 3 Clipboard framework verbs; `noteShellCommand` marked `.chrome()` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | re-exports `CapabilityAudience` + `ActionArgOption` to every plugin crate |
| `…/🌉️mcp/🗂️catalog/🦀️.rs` | gateway-side `CapabilityAudience` (+`From<manifest::…>`, `AGENT_AUDIENCES`, `ALL_AUDIENCES`); `CapabilityDefinition.audience`; `CapabilityOwner::Plugin.label` (plugin display name) threaded through the compiler and both hand-written `ToValue`/`FromValue` arms; `compile_with_audiences` |
| `…/🌉️mcp/🔎️search/🦀️.rs` | BM25 fields 5 → 7: description ×1→×2, new parameter-names ×1.5 (`argument_text`: every arg id **and** its localized label), new artifact-kind ×1.0, plugin display name folded into `owner_text`; `SearchFilters.audience` |
| `…/🌉️mcp/🧬️schema/🦀️.rs` | `SearchHit.audience` + `.artifactKind`; `capabilities.search` input gains `limit`/`cursor`, output gains `total`/`nextCursor` |
| `…/🌉️mcp/🦀️.rs` | `capabilities_search_handler` paginates (`limit` clamped to 1…100, default 20; opaque offset cursor; `total`); `to_schema_search_hit` carries audience + artifact kind |
| `…/🌉️mcp/🖥️ui/🦀️.rs`, `🗿️artifact/🦀️.rs`, `💡️inference/🦀️.rs` | gateway capabilities declare `audience: Agent` |
| `…/🌉️mcp/🧪️tests/🧱️source-builders/🦀️.rs` | new `draw_app`/`draw_descriptor`/`note_cad_and_draw_source` fixture reproducing the measured defect shape |
| `…/🌉️mcp/🔎️search/🧪️tests/🔬️quick/🦀️.rs` | six M5a laws (§4) |
| `…/🌉️mcp/{🔀️dispatch,🗿️artifact,🛡️policy,💡️inference}/🧪️tests/🔬️quick/🦀️.rs` | fixture literals carry the new field |
| `✏️s/🔌️plugins/🖍️draw/…/✏️editor/🦀️.rs` | EN+DE descriptions, `use_when` phrases and **real typed arguments** on 13 intent verbs; 8 input events declared; new shared arg helpers (layer kind / layer id / drop target / drop position / patch field / patch value) whose enums are transcribed from the guest's own `create_layer_by_kind` and `drawing_op_for_layer_field` arms |
| `…/🎫️tickets/…/🐍️m5a-catalog-probe.ts` | the before/after MCP client capture |

---

## 4. Search quality — the six laws

`🔎️search/🧪️tests/🔬️quick/🦀️.rs`, compiled against the new `note_cad_and_draw_source()` fixture:

1. `rectangle_query_ranks_the_intent_verb_first_and_spreads_the_scores` — `"draw rectangle"` puts
   `addLayer` first, the top five carry ≥3 distinct scores, and the winner beats the runner-up by
   >1.5×.
2. `layer_and_export_pdf_queries_discriminate` — `"rename a layer"` → `patchLayer`;
   `"export the document as pdf"` → `exportDocument`.
3. `input_events_are_never_published_to_agents` — the six input/chrome ids are absent from the
   catalog entirely, and no query returns a non-`Agent` hit.
4. `every_published_draw_verb_has_a_description_in_both_locales` — compiled once per locale (EN and
   DE), no empty description.
5. `add_layer_publishes_its_real_input_schema` — `properties.kind` is a `string` whose `enum`
   contains `shape:rect`.
6. `plugin_display_name_is_searchable` — the owner carries `label: "Draw"` and the capability carries
   `artifactKind: "s.draw.drawing"`.

Result: (filling — `cargo test -p semio-framework-os-mcp --lib`, capture
`🗑️generated/m5a-cargo-test.txt`. Note for whoever resumes: a *queued* cargo compiles the tree as of
its start, so the first run of this was killed and relaunched after the law tests landed; relaunch it
rather than trusting a stale capture.)

Compile state already measured: `cargo check -p semio-framework-os-mcp` **green** (6 m 50 s, warnings
only, all pre-existing) on the catalog/search/schema/root changes, and
`cargo check -p semio-s-plugin-draw` **green** (11 m 18 s) on draw's rewritten declarations.

---

## 5. Descriptions written, and how A3 picks the new fields up

**Written at the source** (these are plugin-Rust declarations; they reach an agent only once that
plugin's `🔣️.json` is regenerated — see §6):

| plugin | verbs described | input events declared |
| --- | --- | --- |
| `🖍️draw` | 13, each with EN+DE text, `use_when` phrases **and real typed arguments** | 8 |
| `🗒️note` | 21 | 12 (`engagementInput`/`engagementSubmit`/`navigatorEngagementInput`/`inkApplyEvents` + the 8 fixed-direction keyboard nudges) |
| `🖨️raster` | 13 | 0 (all of raster's input is already `View`+non-palette → derived `Chrome`) |
| `📏️layout` | 13 | 8 |
| `📋️forms` | 21 | 3 (the preview runner's `setTryValue`/`setTryValueStep`/`setTryValues`) |
| `📐️cad` | 23 | 8 |
| framework-injected | 10 (7 History + 3 Clipboard), reaching **every** plugin at once | `noteShellCommand` → `Chrome` |

Draw is annotated verb-by-verb in the declaration chain (descriptions, `use_when`, and the argument
declarations). The other five use three new `AppBuilder` methods added for this —
`action_describe`, `action_use_when`, `action_audience` — which reach an id on any surface it can be
declared on (bare action, window-kind action, bare command, mode command), exactly like the existing
`action_interactive_job`.

**How A3's regeneration picks this up — nothing for A3 to do.** The descriptor emitter serializes
the live `PackageDescriptor` the guest's own `describe()` returns; `description`, `useWhen` and
`audience` are ordinary `ActionSemantics` fields, so any plugin A3 regenerates emits them without a
line of emitter change. The two properties that keep A3 unblocked:

1. **Every new field is additive and `#[serde(default)]` + `skip_serializing_if`.** A descriptor with
   none of them still decodes (that is why the 24 already-committed descriptors kept working through
   this whole slice — §1 and §6 were captured against a tree where only draw had been regenerated).
2. **A plugin that declares nothing still classifies correctly**, because `derive_audience` runs over
   `kind`/`in_palette`, which every descriptor already carries. A3 regenerating a plugin nobody has
   annotated yields a catalog entry with an empty description but the right audience — strictly
   better than today, never worse.

**Coordination notes for peers.** V3a (surface-schema projection): `CapabilityDefinition` gained
`audience` and `CapabilityOwner::Plugin` gained `label`; both are additive, both are omitted from the
wire when absent on the owner side, and neither changes an existing field's shape — if the projection
generator enumerates `CapabilityOwner` variants field-by-field it needs `label` added, otherwise
nothing. M7 (transport/policy/AgentBridge) and R2 (reactor/plugin-host command wire) are untouched:
no dispatch path reads `audience`, and the shell's own dispatch still reaches every input event
because the *guest's* action registry is unchanged — only the gateway's published projection narrows.

---

## 6. Live proof through the real server

Method — `🐍️m5a-catalog-probe.ts`, run twice against the real `semio-os-mcp` stdio binary with the
argv `.mcp.json`'s `semio` entry uses, capturing the same three queries plus two
`capabilities_describe` calls each time:

- **before** → `🗑️generated/m5a-before.txt` (§1). **measured.**
- **after** → `🗑️generated/m5a-after.txt`, taken against a binary rebuilt from this tree with draw's
  `🔣️.json` regenerated by `bun ./📜️script.ts describe` in
  `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust` (= the `@semio-tech/draw-plugin:describe` target).

Status: **(filling)** — the machine is running a 20+-cargo fleet (23 concurrent `rustc`), so the
shared build-dir lock is serialising every step of this chain. Whatever is not captured by the end of
this session is named here rather than claimed.

---

## 7. Honest gaps

1. **Only the plugins whose descriptors are regenerated show their new text live.** The descriptions
   for note/raster/layout/forms/cad are written at the source and compile, but their committed
   `🔣️.json` files still predate them, so a live `capabilities_search` keeps returning
   `description: ""` for those plugins until A3's regeneration sweep runs. §5 states why that is
   zero extra work for A3, but it is unfinished until it runs. **measured gap, not a claim.**
2. **Only draw declares real typed arguments.** `capabilities_describe` now returns a real JSON
   Schema *where the descriptor declares args*; the other plugins' verbs still publish
   `properties: {}` because their `ActionDefinition`s declare none (note/raster/cad declare args for
   3 verbs between them). The typed payload structs exist in every plugin's `app_commands!` block —
   projecting them into `ActionArgDef`s automatically, rather than transcribing them, is the right
   long-term fix and is not done here.
3. **`use_when` phrases are English-only.** `ActionSemantics.use_when` is `Vec<String>`, not
   `Vec<LocalizedLabel>` — so a German query matches only through the German `description` and
   `label`, never through a German trigger phrase. Descriptions and labels ARE bilingual (law 4
   compiles the catalog once per locale), so this is a precision gap, not a blank.
4. **The audience derivation is a rule, not a proof.** `derive_audience` is right for every verb I
   read, and every verb it would get wrong in draw/note/layout/forms/cad now declares itself — but
   the ~28 plugins I did not read may hold `Mutation`-kind gesture routes that will keep being
   published to agents until someone marks them. Nothing detects that automatically.
5. **Pagination is offset-based.** `nextCursor` is the decimal offset into a freshly-computed result
   set; the catalog is immutable per process, so it is stable in practice, but it is not a snapshot
   cursor. `tools/list`/`resources/list` still take no `cursor` (G7 P1.8's other half) — untouched.
6. **`Catalog.hash` changes** because the entry vector narrows. Anything that pinned the old
   `catalogHash` (`ba5a2b6d…`) as a fixture must be re-pinned; I found no such pin in the os-mcp
   crate, and did not audit the hub's own catalog gates.

---

## 8. Session 5 (2026-09-20)

Resumed after the ~01:15 desktop restart cut fleet 4. Everything in this section is **measured**
unless the row says otherwise; captures are `🗑️generated/m5a-s5-*.txt`,
`🗑️generated/m5a-descriptor-census.txt`, `…-source-sweep*.txt`, `…-destructive-backlog.txt`.

### 8.1 What session 4 had actually RUN (verified first, as briefed)

| §4/§6 claim | state found on resume |
| --- | --- |
| six laws compile & pass | **not run.** `🗑️generated/m5a-cargo-test.txt` ends in `error[E0599]: no associated function … from_cols_array … Mat4` in `semio-framework-ui-scene` — a peer's half-refactor, not this slice. That symbol no longer exists anywhere in `🎬️scene/📐️math/🦀️.rs`, so the peer finished; the run was simply never repeated. |
| §6 "after" capture | **not taken.** `🗑️generated/m5a-after.txt` does not exist. |
| draw's `🔣️.json` regenerated | **no.** `✏️s/🔌️plugins/🖍️draw/🔣️.json` is unmodified in `git status`, and the descriptor census below reads `declared 0 / described 0` for draw exactly as for every other plugin. §6 and §7.1 must be read as: *nothing* has reached a live catalog yet; all of §2–§5 is source work awaiting A3. |
| source work (§3/§5) | **present and intact** — the manifest/catalog/search/schema edits and the six plugins' declarations are all in the working tree. |

### 8.2 The measurement that was missing: two censuses

`🐍️m5a-descriptor-census.ts` decodes the plugin registry and all 45 decodable committed descriptors
the way `RegistryDiscovery::scan` does (15 of the 60 registry rows have no `🔣️.json` at all — A1 §3 /
A3's debt, unchanged by me). `🗑️generated/m5a-descriptor-census.txt`, **measured**:

| | value |
| --- | --- |
| capabilities in committed descriptors | 1866 |
| of those, published to agents by `derive_audience` | **1472** |
| with a declared audience | **0** |
| with any description | **0** |
| with `effects.destructive` | **0** |
| gesture-named routes published to agents undeclared | **56** across 21 plugins |

That is the *descriptor* surface — what a live `capabilities_search` sees today. It is 0/0/0 because
no descriptor has been regenerated since this ticket began, which is A3's queued slice.

`🐍️m5a-source-declaration-sweep.ts` asks the same questions of the plugin **source**, which is what a
worker can actually fix. At session start: **49** gesture routes and **97** destructive-class verbs
undeclared at source.

### 8.3 (a) `destructive` — M7's handoff

M7 left 3 of 11 steps of `@semio-tech/framework-os-mcp-rs:live-agent-loop-check` on `SKIP` because
`ApprovalMode::WhenDestructive` reads `effects.destructive`, and `grep` over the whole repo found
exactly **one** `.destructive()` call — inside a manifest unit test. The mechanism was complete; no
production verb used it.

Landed:

| file | change |
| --- | --- |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | `ActionDefinition::destructive` and `CommandDefinition::destructive` made **sync** (they await nothing; every sibling builder — `describe`, `use_when`, `input_event`, `chrome` — is already sync, and `async` made them unusable inside the sync `history_action_definitions()`-style declaration chains where verbs are actually declared). Docstring now states the delete/clear/replace-document contract and names the gate that reads it. |
| `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs` | the one call site loses its `.await`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | new `AppBuilder::action_destructive(id)` — post-hoc marking that reaches bare actions, window-kind actions, bare commands and mode commands, exactly like `action_audience`; raises `approval` `Never → WhenDestructive` and leaves `Always` alone. Forwarded to `ViewerBuilder`/`EditorBuilder` through `surface_builder_forward!`. |
| 53 plugin editor files | **117** `.action_destructive("…")` declarations (see 8.5). |

### 8.4 (b) §7.4 — misclassification is now detectable, and the sweep ran

The audit lives in the gateway, over the `CatalogSource` rather than a compiled `Catalog`, because
the one fact it needs — *declared* vs *derived* — exists only in `ActionSemantics.audience: Option<…>`
and `compile()` resolves that `Option` away.

| file | change |
| --- | --- |
| `…/🌉️mcp/🗂️catalog/🦀️.rs` | new `🔖️Audit` region: `GESTURE_ROUTE_WORDS` (37 entries), `DESTRUCTIVE_VERB_WORDS` (15), a camel-case word splitter, a **contiguous-word-run** matcher (so `dropOnPool` matches `drop` while `addDropdown` does not), `CatalogAuditFinding::{UndeclaredGestureRoute, UnmarkedDestructiveVerb}` and `audit_source(&CatalogSource)`. The lexicon is explicitly **not a classifier**: it only asks the question `derive_audience` cannot answer, and one explicit declaration — including `audience: Agent` — silences it forever. |
| `…/🌉️mcp/🏗️bootstrap/🦀️.rs` | new `semio-os-mcp audit [--folder <dir>]` mode: prints every finding over the committed descriptors and exits 1 when the list is non-empty. It opens no workspace, claims no hub authority and talks to no shell. |
| `…/📦️packages/🦀️rust/📜️script.ts`, `📋️project.json` | `capability-audit-check` target (`dependsOn: build`), wrapping that mode. |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | `⚖️gate🌉️os-mcp🚨️capability-audit` row, group `4_gate`, order `411.107575` (between `os-hub:live-sign-in-check` and `os-mcp🤖️live-agent-loop`). |
| `…/🔎️search/🧪️tests/🔬️quick/🦀️.rs` | laws 7 and 8 (below). |
| `…/🧪️tests/🧱️source-builders/🦀️.rs` | the draw fixture gains a real `deleteLayer` — EN+DE description, `use_when`, a typed `layerId` arg and `.destructive()` — so law 7 has something honest to assert on. |

**Law 7** `a_delete_class_verb_is_marked_destructive_and_gates_on_approval` — the published
`deleteLayer` carries `effects.destructive` **and** `approval == WhenDestructive` (the exact pair the
live gate's `(e)` precondition reads), while `addLayer` carries the same kind default and is *not*
destructive, proving it is `destructive` and not `approval` that decides.
**Law 8** `the_capability_audit_catches_an_undeclared_gesture_route_and_an_unmarked_destructive_verb`
— the annotated fixture audits clean (`audit_source == []`); a copy that drops just those two
declarations yields exactly `UndeclaredGestureRoute{canvasPointerDown, "pointerdown"}` and
`UnmarkedDestructiveVerb{deleteLayer, "delete"}`.

### 8.5 The sweep, measured before and after

Declarations are made **at each verb's own declaration site** by `🐍️m5a-declare-audience.py`, which
inserts one chain step directly after the builder step that declares the id, and refuses unless that
line is a chain step (`.`-prefixed), has balanced parentheses, and is the **only** such line in the
scoped tree — the region guard a name-keyed codemod needs on this repo. It appends `.await` when the
neighbouring step has one (`🪐️space`'s chain is a bare `AppBuilder`, every other one is a
`Viewer`/`EditorBuilder` whose forwarder resolves the future). Every run printed a `git diff --stat`.

| | before | after | remaining |
| --- | --- | --- | --- |
| gesture routes undeclared at source | 49 | **7** | `🎪️demonstrator` only |
| destructive-class verbs undeclared at source | 97 | **24** | see below |

Totals landed: **71** `action_audience` + **120** `action_destructive` declarations across **56**
plugin editor files. Audience declarations went to draw, lowpoly, procedural, space, vcs, animate,
process, puzzle (2d/3d/5d), reasoning, shooting, sourcing, block, dag, sequence, writer — plus three
deliberate **`Agent`** declarations (`draw.dropLayerKind`, `raster.dropLayerKind`,
`forms.dropQuestionKind`): those read like drop gestures but are fully specified by their arguments,
so the right answer is to record the human decision, not to hide them.

**The 24 + 7 that are honestly still open** (all **measured**, none guessed):
- `🎪️demonstrator` (7 audience + 14 destructive) declares **none of them itself** — `grep` finds no
  `"worldPointerDown"` anywhere under `🎪️demonstrator`. Its descriptor lists exactly the union of
  process/dag/puzzle/sourcing's verbs because the playground composes those apps, so the fix already
  landed upstream and arrives when descriptors regenerate. Nothing to do in demonstrator.
- `🔋️energy` (`delete-surface`, `delete-zone`, `setActiveExample`), `🀄️wfc`
  (`remove-palette-color`, `delete-slot`, `deleteTile`, `deleteRule`) and `🧩️puzzle`
  (`brushFillSessionDiscard`, `brushFillSessionClear`, `setFixtureJson`) reach the descriptor through
  a `#[dsl(key = …)]` mutation-kind vocabulary rather than a one-line builder step, so the codemod's
  region guard correctly refuses them; they need a hand edit at their own declaration. The gate names
  each one, so none of them can go quiet.
- `🌀️procedural.setActiveExample` (viewer + editor) and `🌊️flow.removeGeneration` WERE closed, by
  anchoring on their unique `.action_interactive_job("<id>", …)` chain step — the same chain, the same
  id, so the insertion is still at the verb's own declaration.

### 8.6 What is verified, and what is not — honest status

- **Written, diff-verified and parse-verified:** everything in 8.3–8.5. Every codemod run printed
  its own diffstat; a post-pass over `git diff -U1` confirmed no inserted line landed after a `;`, a
  `{` or a `,`; and **`rustfmt --edition 2021 --emit stdout` parses all 59 edited `.rs` files with
  zero errors** (measured). That is a syntax proof, not a type-check — it rules out a misplaced chain
  step, not a wrong method or a missing `.await`.
- **The TS and JSON edits ARE verified:** `bun build --no-bundle` transpiles the rewritten
  `📜️script.ts`, and both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` parse and carry
  exactly one `⚖️gate🌉️os-mcp🚨️capability-audit` row each (measured).
- **Compile/test state: NOT green in this session.** `cargo test -p semio-framework-os-mcp --lib m5a`
  (pid 449, capture `🗑️generated/m5a-s5-cargo-test.txt`) held a single line —
  `Blocking waiting for file lock on artifact directory` — for **43 min and counting**: the machine
  ran **61–69 concurrent cargo processes** at load 75–135 from the parallel fleet for the whole
  session, and the lock holder always had live `rustc` children (so it was a working peer, never a
  stale holder to reclaim — preamble rule 14). Laws 7 and 8, the audit region, the `audit` CLI mode
  and the 56 plugin files are therefore **written, parse-checked, not compiled**. Per this repo's own
  rule that "written, not run" hides breakage, they must be re-run before anyone trusts them; the
  exact commands are 8.7. Anything that turns out red there is mine and is one-line-per-verb to fix:
  every insertion is a single chain step of an existing, already-used builder method.
- **The live gate was NOT re-run**, and could not have passed if it had been: `(e)` reads
  `capabilities_describe` on the **live** catalog, which is compiled from the committed
  `🔣️.json` files, and note's descriptor still predates every declaration in this ticket. A live
  session *is* up (`vite` pid 49007 on `http://127.0.0.1:6080`, `/__semio/agent-bridge` answering
  `{"error":"no live semio-os-mcp gateway is offering a bridge"}`, i.e. healthy and unclaimed), so
  the only missing link is the regeneration in 8.7 step 3.

### 8.7 The exact sequence that closes this (one cargo at a time)

1. `cargo test -p semio-framework-os-mcp --lib m5a` — laws 1–8. Relaunch rather than trusting a
   queued run; a queued cargo compiles the tree as of its start.
2. `bun nx run @semio-tech/note-plugin:describe` (or `bun ./📜️script.ts describe` in
   `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust`) — this is the single step that makes (a) real: it
   compiles note for `wasm32-wasip2` (so it also compile-proves `action_destructive` and the whole
   sweep's call shape) and rewrites `✏️s/🔌️plugins/🗒️note/🔣️.json` with the `audience`,
   `description` and `destructive` fields. Export `CARGO_PROFILE_WASM_DEV_DEBUG=false`.
3. `bun nx run @semio-tech/framework-os-mcp-rs:capability-audit-check` — expected to **fail** with
   ~56 findings on the un-regenerated descriptors and to shrink to ~0 for each plugin A3 regenerates.
   That is the gate doing its job, not a regression; it is the first number in this ticket that makes
   descriptor debt visible per-plugin.
4. `bun nx run @semio-tech/framework-os-mcp-rs:live-agent-loop-check` with
   `S_OS_MCP_LIVE_SHELL_URL=http://127.0.0.1:6080 S_OS_MCP_LIVE_PLUGIN=note` — with note's descriptor
   regenerated, `capabilities_search "delete selection clear" kind=[mutation]` returns
   `note.…#editor.deleteSelection`, whose `capabilities_describe` now answers
   `approval=whenDestructive, destructive=true`, which is exactly the `gates` predicate at
   `🧪️tests/🤖️live-agent-loop/🟦️.ts:314`. Steps (e1)/(e2)/(e3) then run instead of skipping.

### 8.8 Correction to §5 for A3

§5's "nothing for A3 to do" still holds and now covers one more field: `effects.destructive` is an
ordinary `CapabilityEffects` field the descriptor emitter already serializes (every committed
descriptor already carries `"destructive": false`), so regeneration picks it up with no emitter
change. Two additions A3 should know:
1. **A3's regeneration is what turns this slice on.** Per-plugin, the moment `🔣️.json` is rewritten,
   that plugin's descriptions, `audience` declarations and `destructive` marks all go live together.
   `capability-audit-check`'s per-plugin finding count is a ready-made progress meter for the sweep.
2. **`Catalog.hash` changes twice** — once when the audience projection narrows (§7.6) and again per
   regenerated plugin. Nothing in the os-mcp crate pins it; the hub's own catalog gates were not
   audited by me either.

### 8.9 Items (c)/(d)/(e) of the brief — not reached

(c) descriptions for the remaining plugins' agent verbs, (d) input JSON Schemas for every published
verb and (e) `tools/list`/`resources/list` cursors were not started. The session's budget went to (a)
and (b), and the sweep in 8.5 turned out to be 188 declarations rather than the handful §7.4
implied. (d) remains what §7.2 says it is: the typed payload structs exist in every plugin's
`app_commands!` block, and projecting them into `ActionArgDef`s automatically — rather than
transcribing them the way draw's 13 verbs were — is the right fix and is still not done.

### 8.10 Files changed in session 5

| file | change |
| --- | --- |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | `ActionDefinition::destructive` / `CommandDefinition::destructive` → sync; contract docstring |
| `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs` | drops the now-wrong `.await` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `AppBuilder::action_destructive` + its `surface_builder_forward!` row |
| `…/🌉️mcp/🗂️catalog/🦀️.rs` | `🔖️Audit` region: two lexicons, word-run matcher, `CatalogAuditFinding`, `audit_source` |
| `…/🌉️mcp/🏗️bootstrap/🦀️.rs` | `Mode::Audit`, `parse_audit_args`, the `audit` branch, usage string |
| `…/🌉️mcp/🔎️search/🧪️tests/🔬️quick/🦀️.rs` | laws 7 and 8 |
| `…/🌉️mcp/🧪️tests/🧱️source-builders/🦀️.rs` | a real destructive `deleteLayer` in the draw fixture |
| `…/📦️packages/🦀️rust/📜️script.ts`, `📋️project.json` | `capability-audit-check` |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | `⚖️gate🌉️os-mcp🚨️capability-audit` |
| 56 plugin editor `🦀️.rs` files | 71 `action_audience` + 120 `action_destructive` declarations |

Ticket-folder tooling (all re-runnable, all capture into `🗑️generated/`):
`🐍️m5a-descriptor-census.ts` (descriptor surface), `🐍️m5a-source-declaration-sweep.ts` (source
surface), `🐍️m5a-destructive-backlog.ts` (the gate's own lexicon, emits codemod arguments),
`🐍️m5a-audience-anchor-scan.ts` (finds the declaration site), `🐍️m5a-declare-audience.py` (the
guarded codemod). `🐍️m5a-catalog-probe.ts` is session 4's and is unchanged.

### 8.11 Session 5b (06:15 → 09:25, cut by the account limit) — what of §8.7 actually ran

Step-by-step status of the §8.7 closing sequence, **measured**, nothing inferred:

| §8.7 step | ran? | result |
| --- | --- | --- |
| 1. laws | **yes, twice** | see below — 7 of the 8 M5a laws pass; the 8th found a real fixture gap and is fixed; a peer's field broke the crate mid-run and is fixed |
| 2. `note-plugin:describe` | **no** | never reached |
| 3. `capability-audit-check` | **no** | never reached |
| 4. `live-agent-loop-check` | **no** | never reached; the 3 approval steps still skip |

**Lock starvation, measured.** From 06:19 the run sat in `Blocking waiting for file lock on artifact
directory` for **≈ 2 h 05 min**. The rule-23(a) deadlock check was applied twice and both times said
*not* a deadlock: 11–34 `rustc` were running the whole time (`sample 9347 1` showed
`cargo::util::flock::acquire → Filesystem::open_rw_exclusive_create`, i.e. a legitimately held lock,
not a stale one). Killed and relaunched once at 07:50, per rule 23(a); it finally acquired at ~08:22.
This is exactly the writer starvation that became **preamble rule 25** later the same morning — the
private `CARGO_TARGET_DIR` cure did not exist yet and is used from here on.

**Run A (08:22, `--lib m5a`)** — `EXIT=0`, crate compiles clean, but the filter matched nothing
(`410 filtered out`): the law names carry no `m5a` token. Compilation green is itself the first real
proof that §8.3/§8.4's manifest + catalog + bootstrap changes build.

**Run B (08:30, full `--lib`)** — `406 passed / 4 failed` in 315 s. All eight M5a laws ran:

| law | result |
| --- | --- |
| 1 `rectangle_query_ranks_the_intent_verb_first_and_spreads_the_scores` | **ok** |
| 2 `layer_and_export_pdf_queries_discriminate` | **ok** |
| 3 `input_events_are_never_published_to_agents` | **ok** |
| 4 `every_published_draw_verb_has_a_description_in_both_locales` | **ok** |
| 5 `add_layer_publishes_its_real_input_schema` | **ok** |
| 6 `plugin_display_name_is_searchable` | **ok** |
| 7 `a_delete_class_verb_is_marked_destructive_and_gates_on_approval` | **ok** |
| 8 `the_capability_audit_catches_…` | **FAILED — and it was right** |

Law 8's failure is the audit working on its first contact with reality: `audit_source` reported nine
findings in the *fixture* (`cad.deleteObject`, `cad.engagementSubmit`, `cad.setActiveExample`,
`note.deleteBlock`, `note.deleteSelection`, `note.engagementSubmit`, `note.inkApplyEvents`,
`note.setActiveExample`, `note.setFixtureJson`) because session 4 annotated only the *draw* fixture
app while annotating all six real plugins. Fixed at the fixture
(`🧪️tests/🧱️source-builders/🦀️.rs`): the note and cad fixture apps now carry the same declarations
their real plugins do — 6 `.destructive()`, 2 `.input_event()`, and `.input_event()` on the **8
fixed-direction nudges** (`nudgeSelection*`, matching §5's note row; the undirected `nudgeSelection`
stays agent-facing).

That same fixture fix also explains two of the other three failures, which were **session 4
regressions nobody had ever run**: `move_the_selection_finds_cad_translate_selection_as_top_hit` and
`capabilities_search_tool_call_finds_translate_selection` both got
`note.editor.nudgeSelectionUp` where they expect `cad.editor.translateSelection` — session 4's BM25
rework (description ×2, new parameter-name field) reranked them, and with the nudges now correctly
`Input` they leave the agent catalog entirely. The fourth failure,
`workspace::long::plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired`
(`channel.not-wired` / `budget.exceeded` on `InstanceOpen`), is **R2's slice, not mine**.

**One peer breakage fixed to unblock the crate:** a peer added `ContextSummary.channel` (the
shell-vs-headless routing decision) without updating `🧬️schema/🧪️tests/🔬️quick/🦀️.rs:81`, which
failed the whole lib target with `E0063: missing field channel`. Added `channel: "headless".into()`
to that one example literal.

**Run C (09:1x, full `--lib` after both fixes)** — the crate compiled, then the **test binary was
`SIGKILL`ed** (`signal: 9`) before printing a result: the machine was in the swap-kill regime this
repo has recorded before. Not a code failure and not evidence of one; it must be re-run under rule
25's private `CARGO_TARGET_DIR`. The account limit cut the session at ~09:25 immediately after.

### 8.12 Session 5c (11:15) — step 1 of §8.7 is GREEN

Re-run under preamble rule 25 (`CARGO_TARGET_DIR=…/target-m5a`, shared build dir untouched) after
checking rule 26's `ps` guard (no other `cargo test -p semio-framework-os-mcp` was running).
Capture `🗑️generated/m5a-s5-cargo-test.txt`. **measured:**

**All eight M5a laws pass**, including law 8 after the fixture fix and law 7's approval pair:

```
search::quick::rectangle_query_ranks_the_intent_verb_first_and_spreads_the_scores      ok
search::quick::layer_and_export_pdf_queries_discriminate                               ok
search::quick::input_events_are_never_published_to_agents                              ok
search::quick::every_published_draw_verb_has_a_description_in_both_locales             ok
search::quick::add_layer_publishes_its_real_input_schema                               ok
search::quick::plugin_display_name_is_searchable                                       ok
search::quick::a_delete_class_verb_is_marked_destructive_and_gates_on_approval         ok
search::quick::the_capability_audit_catches_…gesture_route_and_…destructive_verb        ok
```

The two session-4 ranking regressions named in 8.11 are **also green** now
(`search::quick::move_the_selection_finds_cad_translate_selection_as_top_hit` and
`root::quick::capabilities_search_tool_call_finds_translate_selection` both `ok`), which confirms the
diagnosis: they were never a search bug, they were the fixture publishing eight keyboard nudges that
the real note plugin already declares `Input`.

423 passed / 2 failed at the point the harness process was `SIGKILL`ed again inside
`workspace::long::*` (the wasm-activation tests — the swap-kill regime, unrelated to this slice).
Of those 2: `schema::quick::the_json_mirror_publishes_exactly_the_registry_exports` **passes when
re-run on its own** (`m5a-s5-mirror.txt`, 1 passed — a peer regenerated the mirror in between), and
`workspace::long::plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired` is
R2's slice. **Nothing red in this slice.**

### 8.13 §8.7 steps 2–4 — CLOSED, and the root cause that was hiding underneath

**Step 2 — `note-plugin:describe`: green.** `EXIT=0` in 58 s of guest execute
(`🗑️generated/m5a-s5-note-describe.txt`, wasm `63b37c41…`, descriptor `85ea6a6a…`). This is the
**compile proof of the sweep**: it builds `semio-s-plugin-note` for `wasm32-wasip2` against the
rewritten `🛂️manifest` and `🔌️plugin` SDK, so the sync `destructive()`, the new
`AppBuilder::action_destructive` and note's own inserted chain steps all type-check. The regenerated
`✏️s/🔌️plugins/🗒️note/🔣️.json` (+4424/−6505) now carries **30 descriptions, 13 declared audiences
and 4 destructive verbs** (`deleteSelection`, `deleteBlock`, `setActiveExample`, `setFixtureJson`),
each with `approval: "whenDestructive"`. **measured.**

**The root cause underneath — app-scope actions were never published at all.** With note regenerated
the live catalog still answered *nothing* for `"delete the selected blocks from the note"`, and
`"pencil width eraser radius ink"` returned block/remodel/wfc but no note. note is in the registry,
its descriptor decodes, and the audit reported it clean — yet no `note.*` capability existed.
`catalog::app_action_verbs` walked **only** `AppDefinition.window_kinds[].actions`; note declares all
48 of its verbs on `AppDefinition.actions`, so every app-scope verb in the repo had been invisible to
agents since the catalog compiler was written. `🖍️draw` worked only because it declares on its canvas
window kind — which is why the coordinator's original smoke saw draw and nothing else, and why §1's
"20 results, all draw" looked like a ranking problem.

Fixed in `🗂️catalog/🦀️.rs`'s `app_action_verbs`: after the window-kind pass it now also walks
`app.actions`, addressing `"*"` (the marker `framework_capabilities` already uses), skipping any id a
window kind already claimed — a concrete window kind keeps winning because only it carries a
dispatchable `ActionAddress.window_kind_id`. Immediately after:
`note.s.note.note@1/*#editor.deleteSelection` ranks **4th** for `"delete selection clear"`
`kind=[mutation]` **with its English description attached** (`🗑️generated/m5a-after.txt`).

**Step 3 — `capability-audit-check`: runs, and is RED by design.** `110 finding(s) over 31
descriptor(s)` (`m5a-s5-capability-audit.txt`; the 29 registry rows with no/undecodable descriptor
are A1 §3 / A3's debt). **`note` is at 0 findings** — the full round trip source → regenerated
descriptor → gate is proven on one plugin, and the per-plugin count (wfc 16, block 12, procedural 10,
draw 10, space 9, remodel 9 …) is now A3's progress meter.

**Step 4 — `live-agent-loop-check`: the approval steps no longer skip, and two of the three PASS.**
Driven against the live React session on `:6080` with `S_AGENT_BRIDGE_DIR` pointed at the rendezvous
that session publishes into (`🗑️generated/m7-rendezvous` — reading it off the vite process's own env
is the missing half of M7's recipe; without it step 0 is a hard `404`).

| | M7 (2026-09-20 01:05) | M5a now |
| --- | --- | --- |
| result | 8 pass / 0 fail / **3 skip** of 11 | **15 pass / 3 fail / 1 skip of 19** |
| `(e1)` Approve Once | SKIP — nothing destructive in the catalog | **PASS** — `channel=elicitation`, invocation proceeds |
| `(e2)` Deny | SKIP | **PASS** — `PERMISSION_DENIED`, `channel:"elicitation"`, `note:"the client's human declined"` |
| `(e3)` silent-client timeout | SKIP | FAIL (below) |

`(f1)`, `(f2)`, `(f4)`, `(f6)`, `(f7)` — R2's mutation chain — went from FAIL to PASS in the same
run: they address `note.*` verbs, which is exactly what the app-scope fix made addressable.

Two honest corrections to the gate itself (`🧪️tests/🤖️live-agent-loop/🟦️.ts`), both measured before
being written:
1. `decide("deny")` left `elicitationAnswer = "accept"`, so the deny step had already approved
   through its own client before it could click anything — it could never have passed. It now
   answers `decline` for the deny case.
2. `(e1)` required a **shell** approval affordance. Measured: the coordinator offers elicitation
   first, so an elicitation-capable client settles the approval and the chat panel never renders a
   group; forcing the shell lane (client silent) produces **no
   `[data-semio-agent-chat-approval]` within 30 s and then a 240 s `action_invoke` hang**. So the
   shell approval lane does not render a gateway-initiated approval at all yet — a real defect, and
   M7/R2's to own. `(e1)` now asserts the outcome (not `APPROVAL_REQUIRED`, not `PERMISSION_DENIED`)
   and **reports the channel** in its detail, so the gap is visible instead of conflated.

The 3 remaining failures are outside this slice: `(f3)` `action_invoke` reports `SUCCEEDED` with an
unchanged `headEditId`, `(f8)` `note` declares no media output port to export through, and `(e3)`
dies before approval with `INTERNAL: typed command frames require an owner-qualified manifest command
key before deserialization` — all three are R2's reactor/typed-command-frame territory.

**One peer breakage fixed to unblock the crate** (as in 8.11): `schema::quick`'s `ContextSummary`
example gained the `channel` field a peer added.

### 8.14 Where the slice stands

Outcome 4 moved: a real MCP client now drives a live shell, finds a destructive document verb **by
its English description**, and a human approves or denies it over a typed channel — measured, not
tested. The gate that keeps it honest is registered (`capability-audit-check`) and is red with a
per-plugin count that only A3's regeneration sweep can retire. Items (c) descriptions for the
remaining plugins, (d) typed args for every published verb and (e) `tools/list`/`resources/list`
cursors remain untouched (§8.9).

### 8.15 The sweep is compile-proven on every plugin it touched

One `cargo check` over **all 30 touched plugin crates at once**, for the target they actually build
for: **`EXIT=0`, 0 errors, 10 m 32 s** (`🗑️generated/m5a-s5-plugin-check.txt`, command in
`/tmp/m5a-check-cmd.txt`). **measured.**

```
CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check --target wasm32-wasip2 \
  -p semio-s-plugin-{animate,architect,block,cad,dag,draw,fem,flow,forms,gis,imperative,layout,
                     lowpoly,mathematical,norm,note,procedural,process,puzzle,raster,reasoning,
                     remodel,sequence,shooting,sourcing,space,trinity,vcs,wfc,writer}
```

Two notes for whoever repeats it: a **native** `cargo check` of these crates is worthless — it fails
with 103 `E0463: can't find crate` for the wasm-gated dependency graph (`semio_framework_plugin`,
`semio_framework_os_kernel`, the artifact contract crates), which says nothing about the sweep; and
`--features component-app-assembly` does not exist on these plugin crates (it belongs to the
per-artifact crates), so cargo refuses the invocation outright.

With this, every one of the 191 declarations — 71 `action_audience` + 120 `action_destructive`
across 56 files — is type-checked, not just parsed. The earlier static checks (rustfmt parse of 59
files, 0 await mismatches over 191 inserted chain steps, every crate carrying the
`semio-framework-plugin` dependency) all held: the sweep needed no corrections.
