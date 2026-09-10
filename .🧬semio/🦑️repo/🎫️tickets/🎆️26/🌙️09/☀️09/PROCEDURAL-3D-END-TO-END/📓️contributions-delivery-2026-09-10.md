# Contributions Delivery — Installing The Flow Extension Registry Into A Served Plugin

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "contributions delivery". Session ⚪9f5f6952
(Fable 5.1, Opus 5). Repo MCP was down all session (`invalid initialize params`); ticket bookkeeping
is on disk and no ticket was opened, closed or reopened.

Answers `📓️extension-addressing-2026-09-10.md` §6: **nothing installed the host's `contributionsJson`
into a served plugin, so the procedural plugin's flow extension registry was empty at runtime and
`brep`/`math` could not be addressed at all.**

---

## 1. TL;DR

The delivery now exists, end to end, and a law test drives it: the shell cuts
`buildContributionsJson`'s payload into a **page run**, dispatches each page as a real
`setContributions` retained command, and the guest reassembles the run and installs it through
`sync_host_flow_extension_contributions` — after which `flow_extension_invocation_address("brep")`
resolves and the hex-column chain finishes with **3 meshes**.

**§6.3's sizing was wrong, and the reason matters.** It proposed a 512 KiB
`ToolExecutionContract::max_raw_wire_bytes` on the assumption that the payload could cross whole. It
cannot, and no tool contract can ever make it:

> `plugin_handle_command` runs `validate_public_json_envelope` **before** the addressed tool's
> contract is consulted, and that envelope caps **every JSON string in the body at 4 096 bytes**
> (`MAX_PUBLIC_ACTION_STRING_BYTES`, `🔌️plugin/🦀️.rs:30416`) and the body at 256 KiB. A 512 KiB
> declaration would have been a ceiling nothing could ever reach — the exact "rubber stamp" this
> codebase refuses elsewhere.

So the real bound is the framework's own public-invocation envelope, and this lane made it a
first-class, language-neutral, schema-first contract that both the guest and the shell read:

| quantity | before | after |
|---|---|---|
| public-invocation string cap | a private literal `4_096` in `🔌️plugin/🦀️.rs`, invisible to the shell | `PUBLIC_INVOCATION_STRING_BYTES`, declared in `🎛️public-invocation/🧬️schema/🔣️.json`, mirrored in Rust and TypeScript, pinned by a law on each side |
| contributions transport | one command, refused as `command contains an oversized string` | a page run cut by `publicInvocationStringPages` / `public_invocation_string_pages` — the same cut on both sides |
| contributions tool contract | none (procedural declared no `setContributions` at all) | its OWN factory at `PUBLIC_INVOCATION_STRING_BYTES × PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR + envelope` = **12 288 B**, a reachable ceiling, leaving the 8 KiB gesture quota of the other 28 routes untouched |
| registry after boot | empty — `contributed: []` | the pushed closure, `brep`/`math` addressable |

Measured on the staged closure the law test builds: **124 085 characters → 31 pages**; the browser's
full nine-plugin closure (293 642 chars, §6.2) is **73 pages**. Each page is one bounded retained
command; the whole run is one boot-time cost, and `sync_host_flow_extension_contributions`
de-duplicates the assembled payload, so a re-push of an unchanged closure rebuilds no registry.

---

## 2. The mechanism, in one pass

```
ShellHost.refreshUi
  buildContributionsJson(loadedPlugins)                       293 642 chars
  → publicInvocationStringPages(json)                          73 pages ≤ 4 096 counted bytes each
  → per page: encodeAppCommandInvocation(plugin, app,
        "setContributions", { json, page, pageCount })
  → handle.handleCommand(instanceId, wire, viewState)
       ↓ wasm boundary
  plugin_handle_command
    validate_public_json_envelope        ← the 4 096 B/string cap that sized the page
    A::command_from_action("setContributions", args)
    admit_command_wire → Generation3dContributionsJobFactory   ← its OWN 12 288 B contract
    ArtifactRetainedCommandJob → Generation3dContributionsWork::step
      set_contributions::handle
        sync_host_flow_extension_contributions_page(page, pageCount, chunk)
          buffer … buffer … last page →
            sync_host_flow_extension_contributions(assembled)  ← the one existing installer
              build_flow_extension_registry → publish
```

Three properties are worth naming:

1. **The page budget is not a literal anywhere.** The shell reads
   `PUBLIC_INVOCATION_STRING_BYTES`; the guest's `validate_public_json_envelope` reads the same
   constant; both are pinned against `🎛️public-invocation/🧬️schema/🔣️.json` by a Rust law and a
   TypeScript law. The tool contract is *derived* from it too
   (`… * PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR + envelope`), never spelled.
2. **The assembler is process-wide, exactly like the registry it feeds.** `FLOW_EXTENSION_STATE` and
   `sync_host_flow_extension_contributions`' own de-duplication witness are already process-wide;
   the page buffer joins them, which is also why ONE delivered push serves every app instance of the
   plugin component.
3. **The route publishes nothing.** `ArtifactToolPublicationLane::HostOnly`, `Emit::default()` — the
   flow extension registry is runtime state, never a document, config, transient or window-transient
   lane.

---

## 3. What was built

### 3.1 The public invocation envelope becomes a contract (framework)

| file:line | change |
|---|---|
| `🧰️framework/🔨️modules/🛂️manifest/🎛️public-invocation/🧬️schema/🔣️.json` | **new** — the language-neutral envelope: `maxBodyBytes` 262 144, `maxStringBytes` 4 096, `maxDepth` 64, `escapePairWireFactor` 2 |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4421-4488` | `//#region 📏️PublicInvocationCapacity` — `PUBLIC_INVOCATION_BODY_BYTES` / `_STRING_BYTES` / `_DEPTH` / `_ESCAPE_PAIR_WIRE_FACTOR`, plus `public_invocation_char_cost` and `public_invocation_string_pages` (the pager) |
| `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️public-invocation-capacity/🦀️.rs` | **new** — 5 laws: schema parity, the escape factor bounds every JSON escape, the pager fills every page without exceeding the bound and reassembles exactly, an empty payload is one empty page, the cost function never undercharges |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30409-30417` | `MAX_PUBLIC_ACTION_BODY_BYTES` / `_STRING_BYTES` / `_DEPTH` now read the shared constants instead of respelling `262_144` / `4_096` / `64` |
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:809-866` | the TypeScript mirror: the three constants, `publicInvocationCharCost`, `publicInvocationStringPages`. Reaches the shell through `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`'s `export *` as `@semio-tech/framework` |

### 3.2 The paged assembler (framework flow registry)

| file:line | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:260-330` | `FLOW_EXTENSION_CONTRIBUTIONS_MAXIMUM_BYTES` (4 MiB), `FLOW_EXTENSION_CONTRIBUTIONS_MAXIMUM_PAGES` (4 096), the `CONTRIBUTIONS_ASSEMBLY` static, **`sync_host_flow_extension_contributions_page`**, `reset_host_flow_extension_contributions_pages`, `host_flow_extension_contributions_pending_bytes` |

Page 0 restarts a run; any gap or count mismatch discards the buffer and answers
`flow.contributions-page-out-of-order`; an address outside its own run answers
`flow.contributions-page-address-invalid` before a byte is buffered; only the final page reaches the
installer.

### 3.3 The route, on three apps

Identical shape in each: a second tool factory with its OWN `ToolExecutionContract`, its OWN
`bounded_first_step_tool_proofs!` row, a `HostOnly` publication contract, and an aggregating
`bounded_first_step_tool_proofs()` that joins both factories' catalogues (the precedent is the flow
plugin's three-factory editor).

| app | file:line |
|---|---|
| generation3d **editor** | `…/🧊️generation3d/…/✏️editor/🦀️.rs:766-908` (`//#region 🧩️ContributionsRoute`), proofs holder at `:719`, aggregator at `:1391`, `app_commands!` row at `:93`, decode arm at `:1449`, builder `.command`/`.action_interactive_job` at `:1683`/`:1766` |
| generation2d **editor** | `…/🌀️generation2d/…/✏️editor/🦀️.rs:482-625`, aggregator at `:1310`, `app_commands!` row at `:184`, decode arm at `:1464`, builder at `:1723` |
| generation3d **viewer** | `…/🧊️generation3d/…/👁️viewer/🦀️.rs:328-457`, aggregator at `:598`, `view_commands!` row at `:57`, `.command` at `:736`, and a **new `command_from_action`** at `:604` |

Payload modules (`{ json, page, page_count }`, `#[dsl(keyword = "set-contributions")]`):
`…/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` for both editors and
`…/👁️viewer/🎮️commands/🧩️set-contributions/🦀️.rs` for the viewer.

**The generation3d viewer had no `command_from_action` at all.** `plugin_handle_command` bridges
EVERY structurally addressed command through it (`🔌️plugin/🦀️.rs:22238`), so before this lane the
viewer's seven declared actions — and any host push — failed closed with `app.command.unsupported`.
The new override covers all eight tool ids.

**The generation2d viewer deliberately gets no route.** It runs no flow evaluation at all (no
`FlowEvalSession`, no `FlowHost`; its preview window's own docstring says so), so it never reads the
registry.

### 3.4 The shell pages, from the declared contract

| file:line | change |
|---|---|
| `…/🏛️ShellHost/🟦️.tsx:862-871` | **new** `appCommandTakesPageRun` — reads the ADDRESSED app's own declaration of the command and asks whether it takes a `pageCount` argument |
| `…/🏛️ShellHost/🟦️.tsx:4261-4278` | the push loop cuts `contributionsJson` with `publicInvocationStringPages` and sends `{ json, page, pageCount }` per page |
| `…/🧪️tests/🔬️engine-contract/🟦️.ts` (tail) | `describe("public invocation paging")` — 4 laws incl. a `JSON.stringify` oracle for the cost function |

The shell sends exactly the arguments the addressed command declares. `cad`, `process3d`, `forms`
and `playbook` still declare a single-`json` `setContributions`, so they keep receiving one command,
unchanged — they are **not** silently handed a page run they would ignore while overwriting
themselves with the last page. Their own contributions channel therefore remains capped at one
4 KiB string; see §6.

---

## 4. Tests

### 4.1 The delivery law, through the real retained route

`…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` —
`host_pushed_contribution_pages_install_the_registry_the_served_chain_needs`:

- builds the payload exactly as `buildContributionsJson` does
  (`{pluginId, topicContribution:{topic:"flow.extension", payload:{manifestJson}}}` per entry, from
  the staged extension crates' own `extension_manifest_json()`), plus **one witness manifest nothing
  else in the binary ever installs**, so the registry state it reads is provably THIS run's delivery
  and not a leftover `install_flow_extension_manifest`;
- cuts it with `public_invocation_string_pages` and dispatches every page through
  `testkit::dispatch` — the real `dispatch_typed` → retained-job → publication ladder;
- asserts the witness resolves to its own plugin id, `brep`/`math` resolve to contributing plugin
  ids, the assembler retains 0 bytes, then drives the full tick chain and asserts `profile`,
  `extrusion-axis`, `extrude` and `column-preview` all read `ok` and the preview paints ≥ 1 mesh;
- removes the witness again, because the registry is process-wide and the law is a guest in it.

```
[STATS] contributions payload chars=124085 pages=31 page-bound=4096
[STATS] host-pushed chain finished: meshes=3
test …::host_pushed_contribution_pages_install_the_registry_the_served_chain_needs ... ok
```

### 4.2 The route's own laws

`…/✏️editor/🎮️commands/🧩️set-contributions/🧪️tests/🔬️unit/🦀️.rs` — a paged run installs the
contributed registry and publishes no store lane; an out-of-order run is refused and discarded; an
invalid page address is refused before a byte is buffered.

`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `contributions_route_declares_a_reachable_wire_ceiling`: the
widest page the envelope admits encodes to ≤ the declared ceiling **and** the ceiling is within 2× of
it (a bound nothing can reach is not a bound), and it exceeds the gesture quota, which is why the
second factory exists at all.

The law also pins the **argument decode** the host actually uses: `command_from_action` is fed the
exact `{json, page, pageCount}` shape the shell sends for page 0 and must round-trip into the typed
command, and every page's cost against `PUBLIC_INVOCATION_STRING_BYTES` is measured with the same
`public_invocation_char_cost` the pager cut with — the shell's route is
`plugin_handle_command → command_from_action`, never the typed binary channel, so a decode arm that
drops `page`/`pageCount` would otherwise pass every other test in the file.

### 4.3 Results

| # | gate | result |
|---|---|---|
| 1 | `cargo test -p semio-framework --lib -- public_invocation view_context` | **9 passed / 0 failed** |
| 2 | `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- set_contributions contributions_route_declares host_pushed_contribution_pages` | **5 passed / 0 failed** |
| 3 | `cargo test -p semio-s-artifact-procedural-generation3d … --lib` (full) | **326 passed / 3 failed** — all three pre-existing, §5 |
| 4 | `cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib` (full) | **228 passed / 2 failed** — the same two pre-existing rows |
| 5 | `cargo test -p semio-s-artifact-procedural-generation3d … --lib -- viewer::` | **34 passed / 0 failed** |
| 6 | `cargo check -p semio-framework-os-flow --keep-going` | **Finished, 0 errors** (1 m 12 s) |
| 7 | `cargo check -p semio-s-plugin-procedural --keep-going` (native) | **Finished, 0 errors** (34 s) |
| 8 | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev --keep-going` | **Finished, 0 errors** (2 m 30 s) |
| 9 | `bunx vitest run` (renderer react target, exhaustive) | 975 passed / 19 failed — every failure in `📃️UiDocumentStore` (5 s test timeouts under a loaded box) and `🧩️package-integration`, neither touched here; **`🔬️engine-contract` passed** |
| 10 | `bunx vitest run -t "public invocation paging"` | **4 passed / 0 failed** |

All Rust commands from the repo root with a private `CARGO_TARGET_DIR` seeded from `target/debug`,
`RUSTC_WRAPPER=""`, `RUST_MIN_STACK=134217728`, `--keep-going`. Raw logs in `🗑️generated/contrib-*.txt`.

---

## 5. Pre-existing failures this lane did not cause and did not touch

Reproduced in isolation (`--test-threads=1`, one test at a time), so they are not an ordering
artifact of anything added here, and each names a framework gate in a file this lane never edited:

| test | message |
|---|---|
| `…generation3d/…::two_instances_converge_disjoint_widget_moves` and its generation2d twin | `module.vcs: remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized` (`🔌️plugin/🦀️.rs:6807`) |
| `…::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` (both) | `generation{2,3}d-publication.contended` — the 4-slot publication lease, stranded by the panic above (that law's own docstring documents the cascade) |
| `…generation3d::…::generation_preview_is_one_app_transient_shared_by_two_generation_windows` | `Generation3d preview operation did not finish` inside its 30 s deadline |
| `semio-framework-os-flow --lib`: 127 of 205 | `final Dictionary ownership must be explicitly retired or owned by a cold boundary` (`🧠️neural/⚙️engine/🦀️.rs:101`) — a neural-engine ownership gate, currently failing across `host::`/`drawing::`/`extensions::`, none of which reach the registry |

---

## 6. What is still not delivered

**`cad`, `process3d`, `forms` and `playbook` still declare a single-`json` `setContributions`.** The
shell now sends them one unpaged command, exactly as before, because
`appCommandTakesPageRun` reads their own declaration — they are never silently corrupted by a run
they cannot reassemble. But their contributions channel is still capped at one 4 KiB string by the
same public-invocation envelope, so the same closure that reached procedural does not reach them.
Giving each of them the paged route is a mechanical repeat of §3.3 plus a page assembler for their
own config lane (their handler stores the JSON in a config mutation, so the assembly must land
before the mutation is emitted, not on every page). It was deliberately left out of this lane: four
unrelated plugins, each with its own store envelope law and its own `host_configuration_mutation`
arm, and no evidence that any of them is on the 3d preview's path.

`install_flow_extension_manifest` and `register_linked_flow_extension_installer` still have no
production caller — correctly so now: they are the **test** installers, and
`sync_host_flow_extension_contributions_page` is the production one.

---
## 7. Files changed

### Framework

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🛂️manifest/🎛️public-invocation/🧬️schema/🔣️.json` | **new** — the language-neutral public invocation envelope |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | `//#region 📏️PublicInvocationCapacity`: four constants + `public_invocation_char_cost` + `public_invocation_string_pages` |
| `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️public-invocation-capacity/🦀️.rs` | **new** — the five capacity/pager laws |
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` | the TypeScript mirror of all of the above |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `MAX_PUBLIC_ACTION_{BODY_BYTES,STRING_BYTES,DEPTH}` now read the shared constants |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` | the paged contributions assembler + its two witnesses |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `appCommandTakesPageRun`; the push loop pages |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | `describe("public invocation paging")` |

### Procedural plugin — generation3d editor

| file | change |
|---|---|
| `…/🧊️generation3d/🦀️.rs` | two `#[path]` module rows (editor + viewer `set_contributions`) |
| `…/🧊️generation3d/…/✏️editor/🦀️.rs` | `app_commands!` row, `//#region 🧩️ContributionsRoute` (constants, contract, work, factory, publication contract, proofs), `Generation3dBoundedCommandJobFactoryProofs` holder, aggregating `bounded_first_step_tool_proofs()`, both factories registered, `build_tool_job` routing + per-route wire bound, `command_from_action` arm, builder `.command`/`.action_interactive_job` |
| `…/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` | **new** payload + handler |
| `…/✏️editor/🎮️commands/🧩️set-contributions/🧪️tests/🔬️unit/🦀️.rs` | **new** — three route laws |
| `…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | `staged_flow_extension_contributions_json(extra)` + the witness manifest helpers |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | the delivery law, `contributions_route_declares_a_reachable_wire_ceiling`, and the exhaustiveness/keyword/bijection rows updated for the second factory |

### Procedural plugin — generation2d editor

| file | change |
|---|---|
| `…/🌀️generation2d/🦀️.rs` | `#[path]` module row |
| `…/🌀️generation2d/…/✏️editor/🦀️.rs` | the same route, verbatim in shape |
| `…/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` | **new** payload + handler |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | exhaustiveness/keyword/bijection rows updated |

### Procedural plugin — generation3d viewer

| file | change |
|---|---|
| `…/🧊️generation3d/…/👁️viewer/🦀️.rs` | `view_commands!` row, `//#region 🧩️ContributionsRoute`, both proofs holders + aggregator, both factories registered, `build_tool_job` routing, the hidden `.command`, and a **new `command_from_action`** covering all eight tool ids |
| `…/👁️viewer/🎮️commands/🧩️set-contributions/🦀️.rs` | **new** payload + handler |
| `…/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | the four-table bijection now spans both factories |

### Generated

| file | change |
|---|---|
| `✏️s/🔌️plugins/🌀️procedural/🔣️.json` | regenerated by `bun nx run @semio-tech/procedural-plugin:describe` — `setContributions` now on `…generation2d@1/*#editor`, `…generation3d@1/*#editor` and `…generation3d@1/*#viewer` |
| `.🧬semio/…/PROCEDURAL-3D-END-TO-END/📓️contributions-delivery-2026-09-10.md` | this report |
| `.🧬semio/…/PROCEDURAL-3D-END-TO-END/🗑️generated/contrib-*.txt` | raw gate + restage logs |

## 8. Cost, and what boot #9 should show

The push is **73 sequential bounded retained commands** for the browser's nine-plugin closure
(31 for the two-crate closure the law test builds), each ≤ 4 096 counted bytes, awaited in order so
the run cannot interleave. It is paid **once**: the shell's push guard is keyed on
`${instanceId}::${contributionsJson}`, and even when that guard opens again (a session switch with
unchanged content) `sync_host_flow_extension_contributions` compares the assembled payload against
the last one installed and returns without building a registry or spending a replacement generation.

Boot #9, on the restage below, should show:

- **no** `flowTessellate skipped` / `flow.extension-not-contributed` fault on the preview — the
  registry is populated before the first `flowEvalTick` settles;
- the flow window's per-widget status reaching `extrude: ok` instead of `queued`;
- `data-meshes-json` carrying at least one mesh instead of `[]`;
- `setContributions` visible in the staged component's strings, and 73 `setContributions` command
  dispatches in the boot trace.

## 9. Restage

`ps`/`lsof` showed the shared `target/wasm32-wasip2` free before the run; `target/debug` was held by a
peer's wgpu build for ~5 minutes (`Blocking waiting for file lock on build directory`), which was
polled, not chased.

```
CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" SEMIO_BUILD_BUDGET_MS=14400000 \
SEMIO_CMD_BUDGET_MS=14400000 NX_DAEMON=false SEMIO_RENDERER=react \
bun nx run @semio-tech/procedural-plugin:describe            # exit 0, 08:15:46 → 08:25:41 (9 m 55 s)
bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev
                                                             # exit 0, 08:25:41 → 08:35:12 (9 m 5 s)
```

`Activated generation3d react dev: 11 completed components (changed)`;
`Successfully ran target activate-generation3d-react-dev … and 38 tasks it depends on`.

Freshness witness on the staged component the browser actually loads
(`…🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm`):

| | before | after |
|---|---|---|
| mtime | `2026-09-10 07:14` | **`2026-09-10 08:35`** |
| size | 80 684 237 | **81 151 877** |
| sha256 (first 32) | `5c04f5944185a9fc4b986720d809fd45` | **`a3b22bda71c4488767d06dddf0670c95`** |
| `strings … \| grep -c setContributions` | **0** | **3** (generation3d editor, generation3d viewer, generation2d editor) |
| `dist/runtime/dev/generation3d/activation/🔣️receipt.json` | 07:14 | **08:35** |
| `✏️s/🔌️plugins/🌀️procedural/🔣️.json` | no `setContributions` on the viewer | `setContributions` with `json`/`page`/`pageCount` on all three apps |

Neither serve was restarted: `lsof -ti :6018` still holds pid 50998 (`curl` → `200`) and
`lsof -ti :6118` still holds pid 5219. A browser reload picks the new component up.
