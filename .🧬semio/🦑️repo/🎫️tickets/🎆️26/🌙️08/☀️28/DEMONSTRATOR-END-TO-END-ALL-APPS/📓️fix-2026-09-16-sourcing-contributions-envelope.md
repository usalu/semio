# 🧩️ `setContributions` envelope — sourcing (`aussuchen`) + cad (`koordinator`), 2026-09-16

Repo root `/Users/ueli/Documents/semio`. Follow-up to
[`📓️fix-2026-09-16-aussuchen-ui-chrome.md`](./📓️fix-2026-09-16-aussuchen-ui-chrome.md) §1c and
[`📓️fix-2026-09-16-cad-sourcing-extension-topics.md`](./📓️fix-2026-09-16-cad-sourcing-extension-topics.md) §5.1.

Path prefixes abbreviated: `S… = ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/`,
`C… = ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

---

## 1. Trace — the contributions path end to end

### 1.1 Host side (TS)

| step | file:line | what it does |
|---|---|---|
| assemble | `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:301` `scopeContributionsJson` | folds every loaded plugin's `manifest.topicContributions` into `ProgramContributionEntry[]` JSON, **cut by operator reachability** (`:290` `contributionReachesKinds`) |
| own the push | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧩️contributions/🟦️.ts:59` `createContributionsPublisher` | per `(pluginId, instanceId)`, joined on registry generation |
| resolve scope | `🏛️ShellHost/🟦️.tsx:4594` `resolveScope` | the receiver's OWN document, else its published examples; `unresolved` ⇒ the push is SKIPPED |
| build pack | `🏛️ShellHost/🟦️.tsx:4615` | `scopeContributionsJson(loaded, session.pluginId, kinds)` |
| install | `🏛️ShellHost/🟦️.tsx:4618-4649` | one pack-encoded `handleCommand` per receiver whose app owns `setContributions`; `appCommandTakesPageRun` (`:964`) only decides whether `page`/`pageCount` are addressed — the shell never cuts 4 KiB JSON pages |
| receiver gate | `🛠️ShellHelpers/🟦️.tsx:1761` `pluginShouldReceiveContributions` | in focused mode ONLY the session's own plugin receives (`demonstrator` → `demonstrator`) |

### 1.2 Guest side (sourcing)

`setContributions` → `S…✏️editor/🦀️.rs:185,245` (command row) → `S…✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs`
→ `Emit::config(SetContributions { json })` → retained **Config** publication lane
(`S…✏️editor/🦀️.rs:379` `ArtifactToolPublicationContract`) → `SourcingCurationConfigPreparation`
(`S…✏️editor/🦀️.rs:~500`) → `config.contributions_json` → read by
`S…🧬️schema/🦀️.rs:688 contributed_sourcing_modules` → `sourcing_modules` → `available_modules`, consumed by the
pool filter bar (`S…✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs:103`) and by `stockFromCatalogue`
(`S…✏️editor/🎮️commands/📇️stock-from-catalogue/🦀️.rs:23`).

There is a SECOND entry point the earlier report missed: the host bridge
`ArtifactEditor::host_configuration_mutation` (`S…✏️editor/🦀️.rs:1048`) builds the same config mutation
directly from the action args, bypassing the command handler. Both are fixed here.

### 1.3 The three ceilings that were in the way

1. **The filter-text envelope (confirmed, fixed).** `SOURCING_CURATION_CONFIG_TEXT_BYTES = 96` priced
   `SetContributions { json }` exactly like a typed-in search query (old `S…✏️editor/🦀️.rs:427`), and
   `sourcing_curation_config_bytes` charged `contributions_json.len()` against the same 96 bytes. The real
   demonstrator pack is **3 706 bytes**, so every real contribution was refused.
2. **A fixed per-turn grant coupled to the store maximum (found while fixing 1).** The preparation demanded
   `SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024` from a grant the host fixes at
   `TYPED_OPERATION_RESULT_PAGE_BYTES` = 4 096. Raising the store maximum alone therefore blocks EVERY
   config edit forever (filters, sort, module toggles) — reproduced: three pre-existing live-dispatch tests
   went red the moment the maximum moved. The grant is now its own constant, the way the document lane
   already documents (`SOURCING_CURATION_DOCUMENT_GRANT_BYTES`).
3. **A measured host close cliff at ~one envelope page (found, worked around, NOT fixed).** Once a body has
   RENDERED against a retained config, a config carrying more than ~3 KiB of contributions text never reaches
   its terminal-empty shell: `PluginApp::close_step(1, 4_096)` returns `Pending { released_items: 0,
   released_bytes: 0 }` forever (997 926 of 1 000 000 drop-loop turns; last productive turn #2244). Measured
   with the module roster and the rendered chrome held constant, varying ONLY `contributions_json`:

   | retained `contributions_json` | dispatch | render | close |
   |---|---|---|---|
   | 1 452 B | ok | ok | ok |
   | 2 909 B | ok | ok | ok |
   | 3 302 B | ok | ok | **livelock** |
   | 3 706 B (real pack) | ok | ok | **livelock** |
   | 4 232 B / 4 360 B | ok | ok | **livelock** |
   | 3 820 B, **no render** | ok | — | ok |

   Both halves are required: a big config alone closes, a render alone closes, the two together do not.
   This is framework-level (`store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES = 4 096`,
   `BoundedConfigValueRetirement::close_step`, `ArtifactStoreCursorDisposer`), not sourcing-specific — cad and
   process3d use the identical `bounded_config_store_owners`/`bounded_config_store_disposer` pair. See §5.

### 1.4 How process / procedural "avoid" it — they do not

- **`procedural`** is the only topic that really crosses: `flow.extension` payloads carry dotted operator
  kinds, so `contributionReachesKinds` matches the document graph, and the packs are installed (console
  `contributions publish … status installed`).
- **`process`** does NOT. I ran `collectOperatorKinds` over the built `process.machines` payloads of
  `process-extension-{wood,metal}` (`.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu/🔌️plugin-modules/*/🔣️.json`):
  **zero** operator kinds, against 60+ for `flow-extension-brep`. `contributionReachesKinds` can therefore
  never be true for a foreign `process.machines` entry, so `scopeContributionsJson` cuts all four process
  extensions out of the pack. The "11 installed machines + Concrete / Metal / Robotic catalog sections" the
  DEV-PROCESS-REACT-E2E report observed are `builtin_installed_catalogs()`
  (`✏️s/🔌️plugins/🏭️process/…/✏️editor/🦀️.rs:2040` — generic + wood + concrete + metal + robotic are compiled
  in), not contributions. **`process.machines`, `cad.computer` and `sourcing.module` have all three never
  landed a host push.** `process3d`'s larger numbers (`PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES = 16_384`,
  `admit_process3d_config_mutation` priced on `PROCESS3D_RETAINED_RAW_BYTES = 8_192`) are simply untested by a
  live push, and under §1.3.3 they would livelock the same way.
- **`cad`** already prices `SetContributions` on its own lane (`C…✏️editor/🦀️.rs:1566`,
  `CAD_CONFIG_STORE_MAXIMUM_BYTES = 65_536`), so it never had sourcing's 96-byte bug. Its real pack is small
  enough to stay under the §1.3.3 cliff (§4).

---

## 2. Design

**The app retains the module roster it can act on, never the host's pack.** A host pack is cut from the whole
loaded closure and is unbounded; a retained config lane is a fixed envelope. Distilling at the app boundary is
what makes the two compatible, and it is also the correct semantics: a contributed module whose id an
installed module already serves installs nothing.

1. **Split the pricing.** `SOURCING_CURATION_CONFIG_TEXT_BYTES` (96) is now strictly the FILTER-text envelope
   (query, module ids, typology path, sort column). Contributions get their own lane,
   `SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES = 2 048`, sized from the §1.3.3 measurement with a full page
   of margin. `SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES = 768 + 2 048`.
2. **Decouple the per-turn grant.** `SOURCING_CURATION_CONFIG_GRANT_BYTES = 4 096` — a fixed constant, the
   only figure the grant is ever compared against, exactly like `SOURCING_CURATION_DOCUMENT_GRANT_BYTES`. The
   config's own size stays a validation, never the gate.
3. **Distil at the boundary.** `schema::installable_contributions(json, maximum_bytes)` keeps the
   `sourcing.module` entries addressed to `sourcing-curation` whose module id no installed module already
   serves, in host order, while the re-encoded roster still fits the lane; everything else is dropped. Both
   entry points (the command handler and the host bridge) go through it.
4. **Dedupe + cap the roster.** `sourcing_modules` skips a contributed id an installed module already serves
   and stops at `SOURCING_MAXIMUM_MODULES = 8`. Every module costs the filter bar one toggle plus one `select`
   item per typology node, so an unbounded roster is an unbounded retained surface.

Consequence for the demonstrator: the three shipped `sourcing-module-{beams,slabs,windows}` extensions
re-contribute exactly the three modules the crate already authors, so the pack **distils to `[]`** — the
mechanism runs end to end, installs nothing new, and the Pool keeps all ten authored kinds. A module id the
app does not author (the `reuse` fixture) installs for real and reaches the filter bar and, after
`stockFromCatalogue`, the Pool rows.

---

## 3. Edits

### `S…✏️editor/🦀️.rs`
- `:388-402` — `SOURCING_CURATION_CONFIG_FILTER_STORE_BYTES` (768) + `pub(crate) SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES` (2 048); `SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES` is now their sum.
- `:405-415` — new `SOURCING_CURATION_CONFIG_GRANT_BYTES = 4_096` with the §1.3.2 law in its docstring.
- `:432-447` — `sourcing_curation_config_bytes` prices the filter text against `TEXT_BYTES` and `contributions_json` against `CONTRIBUTIONS_BYTES` separately, then both against the store maximum.
- `:450-456` — new `sourcing_curation_config_footprint(work_items, retained_bytes)`, the shared tail.
- `:458-474` — `sourcing_curation_config_mutation_footprint` routes `SetContributions` to the contributions lane before the filter-text match; the filter arms are unchanged.
- `:537`, `:563` — the preparation grant and the `advance` gate use `SOURCING_CURATION_CONFIG_GRANT_BYTES` instead of `STORE_MAXIMUM * 4 + 1_024`.
- `:1048-1051` — `host_configuration_mutation` distils through `schema::installable_contributions`.

### `S…🧬️schema/🦀️.rs`
- `:682-684` — new `pub const SOURCING_MODULE_TOPIC` (was an inline literal at `:688`).
- `:712-720` — new `pub const SOURCING_MAXIMUM_MODULES = 8` with the retained-surface law.
- `:722-737` — `sourcing_modules` dedupes by module id and stops at the cap.
- `:739-780` — new `pub fn installable_contributions(contributions_json, maximum_bytes) -> String`.

### `S…✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs`
- `:20-23` — `handle` retains `schema::installable_contributions(&payload.json, SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES)` instead of the raw pack.

### `C…✏️editor/🧪️tests/🔬️unit/🦀️.rs` (cad — TEST ONLY, no cad production code changed)
- `:1863-1892` — new `shipped_cad_computer_contributions()`, a Rust mirror of `shippedCadComputerContributionsJson` (`C…✏️editor/⚙️engine/🏃️runtime/🟦️.ts:51`) with all four `cad-extension-*` entries verbatim.
- `:1898` — new law `the_shipped_cad_computer_pack_is_admitted_by_the_retained_config_envelope`: the real pack fits `CAD_CONFIG_STORE_MAXIMUM_BYTES` **and** stays under one `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` (§1.3.3), `admit_cad_config_mutation`/`prepare_cad_config` accept it, the production `setContributions` dispatch stores it verbatim, `validate_cad_computer_contributions` decodes all four modules in host order, and the shape panel still assembles with the pack installed (that render path runs the validator, `C…✏️editor/🦀️.rs:1278`).

### Tests changed / added (sourcing)
- `S…✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs:35` — new `demonstrator_contributions()` built from the schema's own modules exactly as `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/*/🦀️.rs` builds them.
- `:233` — `the_real_sourcing_module_pack_installs_through_the_live_contributions_lane` **replaces** the deleted gate `set_contributions_is_refused_by_its_retained_config_envelope`: the real pack is past the 96-byte envelope, distils to `[]`, leaves the roster at three, is admitted by the footprint, crosses a LIVE dispatch, and the Pool still renders all ten kinds.
- `:252` — `dispatching_set_contributions_installs_the_module_into_the_live_catalogue`: a NEW module id is installable, crosses the live lane, and its kind appears in the catalogue `stockFromCatalogue` publishes (proof the retained store really kept it).
- `S…✏️editor/🧪️tests/🔬️unit/🦀️.rs:284` — `the_contributions_lane_is_priced_apart_from_the_filter_text_envelope`: at-ceiling admitted, one byte past refused (never truncated), and a filter edit over a full lane never disturbs the pack.
- `S…✏️editor/🧪️tests/🔬️unit/🦀️.rs:275-276` — the retained-config oracle now pins `STORE_MAXIMUM == 768 + CONTRIBUTIONS_BYTES` and `GRANT_BYTES == 4_096` (was `STORE_MAXIMUM * 4 + 1_024 == 4_096`).
- `S…✏️editor/🧪️tests/🔬️unit/🦀️.rs:462,476` — `host_contributions_resolve_to_the_event_sourced_config_lane` now asserts distillation on both sides (a pack with nothing installable ⇒ `[]`; a real `reuse` module ⇒ the installable roster).
- `S…🧬️schema/🧪️tests/🔬️unit/🦀️.rs:131` — `available_modules_tracks_contributed_modules` contributes `reuse` (4 modules) and additionally pins that a re-contributed authored id installs nothing (3 modules).

---

## 4. Verification

```sh
cd /Users/ueli/Documents/semio
export DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true \
       CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4 NX_TUI=false NX_TASKS_RUNNER_DYNAMIC_OUTPUT=false

cargo test  -p semio-s-artifact-sourcing-curation --lib
cargo test  -p semio-s-artifact-cad-cad --lib
cargo check --target wasm32-wasip2 -p semio-s-artifact-sourcing-curation -p semio-s-plugin-sourcing
cargo check --target wasm32-wasip2 -p semio-s-artifact-cad-cad          -p semio-s-plugin-cad

bun nx run @semio-tech/sourcing-plugin:materialize-dev
bun nx run @semio-tech/cad-plugin:materialize-dev
bun nx run @semio-tech/demonstrator-plugin:materialize-dev
bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev
```

| gate | before | after |
|---|---|---|
| `semio-s-artifact-sourcing-curation --lib` | 142 tests, 141 passed, 1 ignored (previous pass's report) | **144 tests, 143 passed, 0 failed, 1 ignored** |
| `semio-s-artifact-cad-cad --lib` | — | **340 tests, 320 passed, 19 failed, 1 ignored** (all 19 pre-existing, see below); `…--lib contributions` = **3 passed, 0 failed** |
| `cargo check --target wasm32-wasip2` sourcing (2 crates) | — | exit 0 |
| `cargo check --target wasm32-wasip2` cad (2 crates) | — | exit 0 |
| `@semio-tech/sourcing-plugin:materialize-dev` | — | ok, 2m 45s |
| `@semio-tech/cad-plugin:materialize-dev` | — | ok, 1m 30s |
| `@semio-tech/demonstrator-plugin:materialize-dev` | — | ok, 4m 24s |
| `@semio-tech/mit-bestand-demonstrator:activate-dev` | — | **ok, 16m 24s, 83/83 tasks** — includes `framework-os-dev:prepare-aussuchen-react-dev` + `prepare-koordinator-react-dev`, all three `sourcing-extension-*` and all four `cad-extension-*` `materialize-dev`, and `Prepared Demonstrator dev: 7 runtime variants` |

⚠️ The **19 cad failures are pre-existing and none of them is mine**: this pass changed *no* cad production
code (the only cad edit is an additive test plus its fixture), and the failures are in
`add_object_through_wrapper_grows_the_composed_pane`, `undo_redo_round_trips_*`,
`two_instances_converge_disjoint_edits_via_backbone`, `cad_document_contract_round_trips_exact_child_identities`,
the binary round-trips, and the window-ownership runtime law — the same
"object mutations still no-op (child re-materialisation seam)" family the 09-16 CAD baseline records. Full log:
`🗑️generated/cad-contributions-test-1.txt`.

Logs (all under `🗑️generated/`): `sourcing-contributions-test-{1..7}.txt`, `cad-contributions-test-{1,2}.txt`,
`sourcing-contributions-wasm-check.txt`, `cad-contributions-wasm-check.txt`,
`sourcing-materialize-dev.txt`, `cad-materialize-dev.txt`, `demonstrator-materialize-dev.txt`,
`demonstrator-activate-dev.txt`.

---

## 5. What the coordinator should verify — and decide

### 5.1 Browser, `:6029`

**aussuchen** (`s.sourcing.curation@1/*#editor`)

1. The Pool still lists **all ten authored kinds** — 4 beams (`beam-glulam-gl24h`, `beam-kvh-c24`,
   `beam-steel-ipe200`, `beam-steel-hea160`), 3 windows (`window-casement-100x120`, `window-fixed-150x150`,
   `window-tilt-turn-120x140`), 3 slabs (`slab-concrete-240`, `slab-clt-160`, `slab-hollow-core-265`).
2. The filter bar shows **exactly three** module toggles — Beams / Windows / Slabs. **Six would be the bug**:
   the three `sourcing-module-*` extensions re-contribute the modules the crate already authors, and a
   duplicate must install nothing.
3. Console: `[DEBUG] contributions push` / `contributions publish … status installed` for `demonstrator`.
   `chars` is the host pack size. **If you instead see `contributions push skipped unresolved document
   operators` or `refused empty pack`, §5.2 is the live blocker, not this fix.**
4. Nothing regressed: search box narrows, sortable headers reorder, `plus`/`minus` row actions, Restock.

**koordinator** (`s.cad.cad@1/*#editor`)

5. Console should show four `cad.computer` modules registering: `spatial-shape`, `aec-building`,
   `aec-building-energy`, `aec-building-structure` (`syncCadComputerContributions`,
   `C…✏️editor/⚙️engine/🏃️runtime/🟦️.ts:127`). Note the guest falls back to
   `shippedCadComputerContributionsJson` when no host push arrives, so the four modules appearing is NOT by
   itself proof the push landed — check the `[DEBUG] contributions` lines for that.

### 5.2 🚨 DECISION 1 — the host reachability cut still blocks every capability topic

`scopeContributionsJson` (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:301`) forwards a foreign plugin's contribution
only when `contributionReachesKinds` finds one of the open document's dotted OPERATOR kinds inside the
contribution. That is the right cut for `flow.extension` (operator-keyed) and structurally impossible for
`process.machines`, `cad.computer` and `sourcing.module`, which declare **zero** operator kinds (§1.4,
measured). The guest side is now correct for sourcing and cad; the host will still cut their packs to `[]`.

Proposed minimal fix, NOT applied here (shared framework file, and it is a cross-app policy call):

```ts
function contributionReachesKinds(topicContribution: unknown, kinds: ReadonlySet<string>): boolean {
  const contributed = new Set<string>();
  collectOperatorKinds(topicContribution, contributed);
  if (contributed.size === 0) return true; // capability pack: no operator graph can scope it
  for (const kind of contributed) if (kinds.has(kind)) return true;
  return false;
}
```

It cannot regress procedural (every `flow.extension` pack declares kinds, so it stays cut by reachability),
and the existing law "drops every foreign contribution when the graph names no operator kind"
(`🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🔬️scope-contributions/🟦️.ts:44`) still passes unchanged, because all
four fixtures there declare operator kinds. A second gate remains even then: `resolveScope`
(`🏛️ShellHost/🟦️.tsx:4594`) returns `unresolved` and SKIPS the whole push when the receiver's document and
examples yield no operator graph — which is the normal state for a curation document.

### 5.3 🚨 DECISION 2 — the ~4 KiB retained-config close cliff is a framework bug

§1.3.3. Reproducible from this crate in seconds (retain >3 KiB of config text, render one body, drop the app).
It caps every app's retained config lane at about one `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` page regardless of
the app's declared maximum — so `CAD_CONFIG_STORE_MAXIMUM_BYTES = 65_536` and
`PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES = 16_384` are both writing cheques the close protocol will not cash.
Until it is fixed, `SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES` stays at 2 048 and the distillation in §2.3
is what keeps sourcing inside it. Raising the lane is a one-line change once the close path pages properly.

### 5.4 Smaller follow-ups

- `@semio-tech/cad-extension-aec-building-rust:describe` is still broken (previous report §3); the owner-root
  descriptor is only refreshable through the materialize emitter.
- The 19 pre-existing cad `--lib` failures (§4) are unowned by this pass.
