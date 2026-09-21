# S10 — Home lists the user's studios, and every spawnable kind round-trips inside `s`

Slice S10, session 7, 2026-09-21. Inherits `📓️s9-home-studios-and-sweep-forms.md` (the
`sessionIdentity` fix, the staged-form filler, the `norm` bridge), `📓️s8-…` §3–§4,
`📓️s7-…` §4 (22/35), `📓️pb3-…` §3.6 (flow child lanes), `📓️s4-…` (`Migrated`).

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read only.

## 0. Infrastructure this slice owns

| what | value |
|---|---|
| hub | port **7641**, data root `.🧬semio/🌐hub/s10-boot`, binary `⚡️cache/hs1/os-hub-7611`, `OS_HUB_CREDENTIAL_SIGN_IN=true` |
| hub holder pid | `85137` (bun holder) → `85140` (`os-hub-7611` child, bound 127.0.0.1:7641) |
| serve | `s` react dev on **6071** → `http://127.0.0.1:7641`, staged tree reused, no re-activation |
| serve holder pid | `85175` (script) → `85364` (vite, bound 127.0.0.1:6071), log `🗑️generated/c2-serve-s-6071.txt` |
| humans | `user1@semio.dev` / `gm1-local-dev-pass-1`, `user2@semio.dev` / `gm1-local-dev-pass-2`, provisioned with `os-hub credential set` |

## 1. Home lists the user's studios — S9's fix works, and the NEXT root is found and fixed

### 1.1 Before (measured, `🗑️generated/s10-home-a.txt`, serve `:6071` → hub `7641`, 75 s settle)

S9's `sessionIdentity` fix is live in the repo source (`🏛️ShellHost/🟦️.tsx:5252`, `:7381`), and the
staged `s` tree holds only guest wasm — the host TS is served from source by vite — so the first
measurement of this slice is the first measurement of that fix.

| witness | S9 (before its fix) | S10 (after it, this slice) |
|---|---|---|
| `document.body` Studios table | `"No studios yet. Create one from the navbar."` | **`"Demo Studio  atelier  private  1  0  local  open"`** |
| `empty` (the "No studios yet" string is present) | `true` | **`false`** |
| `s.home.session-identity-required` | raised on every `createStudio` | **gone** |

**So S9's root fix is confirmed at runtime: the identity now crosses, `home_space_rows` runs, and
Home renders a row.** This is the first time any slice has observed that.

### 1.2 The next refusal, and its root — every durable `space` gesture was refused

With identity crossing, `createStudio` reached the store and was refused by a NEW sentence:

```
warning: input #1 createStudio refused: dispatch-failed (user window=s-home-main)
  — typed-operation failed: validation failed:
    batched item candidate failed its exact fixed fold contract
```

That sentence is `ArtifactStore::fold_batch_item`'s
(`🧰️framework/…/🏪️store/🦀️.rs:17002`), and the store's own docstring names the cause in as many
words (`🏪️store/🦀️.rs:14029`): *a point-invertible mutation costs **2** work items, not 1 — declaring
1 fail-closes EVERY single-mutation durable gesture with this exact message.*

`🪐️space` declared **1**:

```rust
// ✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs:624, before
Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
```

and `prepare_space_retained_one_item`, four lines below it, computes an `inverse` for every
mutation — so `forwards.len() + inverse.len()` is always 2 and the fold always refused. The
framework's OWN generic factory has always declared the other number
(`admit_bounded_config_mutation`, `🔌️plugin/🦀️.rs:14798`:
`ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes)`), so the space copy was the
outlier, not the law.

This one declaration is the store authority for **four** lanes —
`space-home-artifact-retained`, `space-index-artifact-retained`, `space-index-config-retained` and
`space-studio-artifact-retained` (`🦀️.rs:456`, `🪐️space/…/✏️editor/🦀️.rs:255,259`,
`⚙️engine/🪐️space/🦀️.rs:775`) — so **every durable home, space-index and studio gesture in the
product was refused by it**, which is why `createStudio` has never worked for any slice.

**Fixed** (`✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs:624`):

```rust
Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes))
```

### 1.3 The missing law (landed, GREEN)

`✏️s/🔌️plugins/🪐️space/🧪️tests/🔬️retained-store-footprint/🦀️.rs` (new, registered at
`✏️s/🔌️plugins/🪐️space/🦀️.rs`) —
`space_retained_preflight_declares_both_staged_rows_of_a_point_invertible_item` takes the lane's own
`preflight` footprint for a real `SHomeMutation`, computes that mutation's real `inverse` length, and
asserts the declaration covers `inverse + 1` rows and equals
`store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS`.

```
cargo test -p semio-s-plugin-space --lib retained_store_footprint
test retained_store_footprint_tests::space_retained_preflight_declares_both_staged_rows_of_a_point_invertible_item ... ok
test result: ok. 1 passed; 0 failed
```

### 1.4 Reload keeps the session AND the table (measured, `🐍️s10-boot-diagnose.mjs`, new)

The acceptance's reload clause, driven on the restarted pair at 10:44 (sign in → 80 s settle →
`page.reload()` → 90 s settle):

```
BEFORE-RELOAD  … Studios Create Space Name Kind Visibility Members Updated Origin Actions
                 Demo Studio atelier private 1 0 local open …
AFTER  READY s   PROBE present
BODY           … Studios Create Space Name Kind Visibility Members Updated Origin Actions
                 Demo Studio atelier private 1 0 local open …
```

So the signed-in session survives a full page reload and Home re-renders its studios table with the
row still in it. Signed OUT the same probe reads the empty case (`"No studios yet…"`, `signed out`),
which is the by-design `None`-identity branch — the two readings together are the honest proof that
the table is driven by the identity and not by luck.

**Still open, and behind the one wasm rebuild:** the row above is the LOCAL seeded `Demo Studio`
(`origin: local`). Creating a SECOND studio needs `createStudio`, whose store refusal §1.2 fixes in
the `🪐️space` guest — so "create a studio → reload → it is listed" and "enter one" are measured only
after the re-activation (§0). **NOT MEASURED** — blocked on the guest rebuild (§7.1).

### 1.5 A live-only host↔guest skew found on the way (measured, attributed, not mine)

Between 04:05 and 10:35 the sweep stopped reaching the studio at all. Attributed, not guessed
(`🗑️generated/s10-sweep-before.txt`, `🐍️s10-boot-diagnose.mjs`):

| fault | where it comes from |
|---|---|
| `TypeError: Cannot read properties of undefined (reading 'en')` | a `LocalizedLabel` read on the host side |
| `Error: plugin-handle.closed`, `noteShellCommand refused: instance-retired (user window=s-home-main) — space#2` | the consequence: Home's instance retires under its own live window |

`git status` shows a peer mid-refactor on exactly that shape — `📡️spr/🎮️command/🦀️.rs` and eleven of
its mutation fixtures are uncommitted with `fn label(&self) -> String` becoming
`fn label(&self) -> LocalizedLabel`. The `s` host TS is served from SOURCE by vite while the staged
guests are wasm from 2026-09-20, so the host already speaks the new label shape and every staged
guest still speaks the old one. **That is a staging skew, not a defect of this slice's four fixes,
and the re-activation in §0 is its cure** — recorded here because the sweep's numbers below are
meaningless until it is gone. Two of its symptoms were the sweep's own, and those ARE fixed:

- the sweep's `signIn` left the overlay by `[data-semio-hub-workspace] button[aria-label]`'s FIRST
  match and then called `page.goBack()` — a history navigation that is no part of signing in and that
  left the shell throwing the `'en'` error with no `window.__semioOsCatalogProbe` at all, so every row
  died at `studio: command palette never opened`. Replaced with the named control
  `[id="os.hub.signIn.cancel"]` and no navigation, the shape `🐍️s9-home-actions-probe.mjs` proved
  (`🐍️s6-all-kinds-sweep.mjs`). After the fix: `sign-in ok`, palette opens, studio entry found and
  dispatched.

## 2. The 35-kind sweep

### 2.1 One unguarded label read was unmounting the whole shell (found, root-fixed, MEASURED)

Before any sweep number could mean anything, the sweep had to reach a studio at all — and from
~10:35 it could not. `🐍️s10-studio-entry.mjs` (new) drives Home → palette → studio and prints every
console line and the full stack of every `pageerror`. Before:

```
settled windows=["s-home-main"]   notes=1 (an info line — a CLEAN signed-in Home)
palette opens, studio entry count=1, click
pageerror: Cannot read properties of undefined (reading 'en')
    at historyEntryLabelText (🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1840)
    at …/🏛️ShellHost/🟦️.tsx  ← inside a useMemo of FrameworkOsShellInner's RENDER
warning: input #2 noteShellCommand refused: instance-retired (user window=s-home-main) — space#2
pageerror: plugin-handle.closed
NEVER-OPENED   windows still []          ← every window gone
BODY  (empty)                            ← the whole product unmounted
```

The function, at `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1838`:

```ts
export function historyEntryLabelText(label: LocalizedLabel, terminology: string, locale: string): string {
  if (terminology !== "native" && terminology !== "reuse") return "";
  const row: Readonly<Record<string, string>> = label[terminology];   // ← may be undefined
  return row[locale] ?? "";                                          // ← throws
}
```

Its own docstring states the contract it breaks: *"an axis value the carrier does not carry is a wire
defect and **renders as empty — visibly wrong** — rather than silently as English."* The **locale**
axis was optional-chained; the **terminology** axis was not. A guest whose label carries the
pre-`LocalizedLabel` shape (`{en, de}` with no `native`/`reuse` level — exactly what §1.5's in-flight
peer refactor produces against a guest staged on 2026-09-20) makes `label["native"]` `undefined`, and
because this runs inside a render `useMemo`, the throw does not cost one ledger row its text: **it
unmounts the entire shell**, which then retires the Home instance and closes the plugin handle.

**Fixed** — both axes read defensively, the no-English-fallback law untouched:

```ts
const row: Readonly<Record<string, string>> | undefined = label?.[terminology];
return row?.[locale] ?? "";
```

**After, on the same probe, same serve, no rebuild** (the host is served from source):

```
settled windows=["s-home-main"]   notes=1 (the same info line — ZERO faults)
studio entry count=1, click
OPENED ["space-3::s-workflow","space-3::s-media-vfs","space-3::s-compiled-dag"] after 5s
BODY  … ← Back to Workflow · semio · s · studio  Workflow  Media VFS  Compiled DAG …
```

So **Home → enter a studio now works end to end with zero faults** — outcome 1's "enter one" clause
— and the sweep can run again. Laws: `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🏷️history-entry-label/🟦️.ts`
(new, registered in the kernel vitest `include`), five cases — both axes read when filled, an
unfilled LOCALE renders empty (the existing promise), an unfilled TERMINOLOGY and an absent label
each render empty **and do not throw**, an unknown terminology renders empty. **5 passed**
(`🗑️generated/s10-kernel-label-law.txt`; the one unrelated red in that project,
`KernelReturnContentFraming`, is a pre-existing ajv `$ref` resolution failure this slice did not
touch).

### 2.2 Two probe defects in the permanent sweep, both measured and fixed

| defect | evidence | fix |
|---|---|---|
| `signIn` left the hub overlay by `button[aria-label]`'s FIRST match and then `page.goBack()` | the navigation re-entered the crash of §2.1; `studio: command palette never opened`, `window.__semioOsCatalogProbe` never published, **every row dead** | leave by the named `[id="os.hub.signIn.cancel"]`, no navigation — `🐍️s9-home-actions-probe.mjs`'s proven shape |
| `enterStudio` looked for the studio row WITHOUT typing a query | the palette renders a window of rows; unfiltered it showed **20** and the studio's own entry (`spawn.space.s.space.studio@1/*#editor`) was not among them → `no studio palette entry` | type `"studio"` first and address the entry by its exact `data-command-item-id`, as `spawnProgram` already did for a plugin id |

### 2.3 The third shape — staged values read off the LIVE document (landed)

S9 §3.3 named it: `energy rename-zone zone=1` now reaches the guest and is refused
`mutation.target-missing the energy model has no zone with id 1`, because `1` is b3d's value for
b3d's seeded document. Landed in `🐍️s6-all-kinds-sweep.mjs`:

- a `LIVE_ID` sentinel in the arg map instead of a literal (`energy.rename-zone.zone`,
  `trinity.setParameter.parameterId`);
- `fillStagedArgument` now treats a `<select>`'s `options` and a combobox's `[role="option"]` list as
  **the live document** — they are what the guest enumerated from the snapshot it is rendering — so a
  requested value that is not among them is a stale static guess and the first real option wins;
- for a free-text id field, `liveDocumentIds(page)` (new) harvests candidates from the spawned
  window bodies' `data-row-id`/`data-node-id`/`data-feature-id`/`data-item-id`/`data-zone-id`, the
  measure-tree rows, and the `id=…` tokens the guest printed into History `op_lines`;
- an empty harvest is reported as `<key>:no-live-id` rather than papered over with a guess.

### 2.4 Baseline, re-measured on this slice's own pair (`🗑️generated/s6-sweep-s10-before.txt`)

Four kinds, BEFORE the guest rebuild (so with the old `applied` predicate live):

| kind | verb | `mutated` | `edits` | `applied` (ledger rows) | reading |
|---|---|---|---|---|---|
| 🌊️flow | `addWidget` | **true** | `[0,0,0,0]` | `[1,3,6,9]` | the ledger moves on the verb, the undo and the redo; `#s-checkin` never does |
| 🎬️sequence | `addStep` | **true** | `[0,0,0,0]` | `[2,4,6,9]` | identical shape |
| 🌀️procedural | `generate` | **true** | `[0,0,0,0]` | `[1,3,3,3]` | identical shape |
| 🔋️energy | `rename-zone` | false | `[0,0,0,0]` | `[5,5,5,5]` | S8 §4.2's shape 2 — the applied rows are the shell's own `Activate Window` |

**This is §4's defect, reproduced directly**: three kinds whose document demonstrably moves and whose
`#s-checkin` count cannot see it, because every one of their verbs publishes on the `Child` or
`Config` lane. `sign-in ok`, `studio space-3::s-workflow`, `registry 60 rows, 60 loaded, 148
spawnable programs`, `fatal None` — the harness itself is healthy.

### 2.5 The full 35-kind sweep — **24/35 measured**, all five chunks, on the pre-rebuild guests

Five chunks as S7 did (`🗑️generated/s6-sweep-s10{a,b,c,d,e,e3}.txt`), every row driven inside the real
`s` host after signing in as `user1@semio.dev` and entering a studio. `edits` is `#s-checkin`'s count
at `[before, verb, undo, redo]`; a PASS needs a spawn, a rail, a real mutation, `redo ≠ undo` and 0
fault lines.

| kind | | verb | `edits` | named refusal / reason |
|---|---|---|---|---|
| `animate` | PASS | `addTile` | `[0, 1, 0, 1]` |  |
| `architect` | PASS | `setAdjacencyKind` | `[0, 1, 0, 1]` |  |
| `block` | PASS | `addHandleKind` | `[0, 1, 0, 1]` |  |
| `cad` | PASS | `addNode` | `[0, 1, 0, 1]` |  |
| `dag` | PASS | `addNode` | `[0, 1, 0, 1]` |  |
| `demonstrator` | PASS | `changeSchema` | `[0, 1, 0, 1]` |  |
| `draw` | PASS | `addLayer` | `[0, 1, 0, 1]` |  |
| `energy` | FAIL | `set-surface-property` | `[0, 0, 0, 0]` | verb dispatched and moved the document; redo/undo pair did not read two different documents |
| `fem` | PASS | `addNode` | `[0, 1, 0, 1]` |  |
| `flow` | FAIL | `addWidget` | `[0, 0, 0, 0]` | typed-operation failed: retained command work refused the command before any capacity was  |
| `forms` | PASS | `addStep` | `[0, 1, 0, 1]` |  |
| `gis` | PASS | `addFeature` | `[0, 1, 0, 1]` |  |
| `imperative` | PASS | `addStep` | `[0, 1, 0, 1]` |  |
| `layout` | PASS | `addPage` | `[0, 1, 0, 1]` |  |
| `lowpoly` | PASS | `addPrimitive` | `[0, 1, 0, 1]` |  |
| `mathematical` | PASS | `nodeGraphEdit` | `[0, 1, 0, 1]` |  |
| `norm` | FAIL | `setSnapshot` | `[0, 0, 0, 0]` | action 'setSnapshot' is not a framework-reserved action (history/clipboard/revert/filter/n |
| `note` | PASS | `addBlock` | `[0, 1, 0, 1]` |  |
| `playbook` | PASS | `addStep` | `[0, 1, 0, 1]` |  |
| `playbook-module-procedural` | FAIL | `importSolidGeometry` | `[0, 0, 0, 0]` | UI dispatch rejected action:importSolidGeometry with interactive-job classification BatchO |
| `procedural` | FAIL | `generate` | `[0, 0, 0, 0]` | verb dispatched and moved the document; redo/undo pair did not read two different documents |
| `process` | PASS | `addStep` | `[0, 1, 0, 1]` |  |
| `puzzle` | PASS | `addNode` | `[0, 1, 0, 1]` |  |
| `raster` | PASS | `addLayer` | `[0, 1, 0, 1]` |  |
| `reasoning` | PASS | `addNode` | `[0, 1, 0, 1]` |  |
| `remodel` | PASS | `addStream` | `[0, 1, 0, 1]` |  |
| `sequence` | FAIL | `addStep` | `[0, 0, 0, 0]` | Sequence command does not match its exact retained route or payload envelope |
| `shooting` | PASS | `addShot` | `[0, 1, 0, 1]` |  |
| `sourcing` | FAIL | `stockFromCatalogue` | `[0, 0, 0, 0]` | verb dispatched and moved the document; redo/undo pair did not read two different documents |
| `space` | FAIL | `—` | `—` | home: unhandled action id set-cell |
| `stdio` | FAIL | `paste` | `[0, 0, 0, 0]` | action 'set-cell' is not a framework-reserved action (history/clipboard/revert/filter/note |
| `trinity` | FAIL | `patchNodes` | `[0, 0, 0, 0]` | verb dispatched and moved the document; redo/undo pair did not read two different documents |
| `vcs` | PASS | `incrementCounter` | `[0, 1, 0, 1]` |  |
| `wfc` | PASS | `change-seed` | `[0, 1, 0, 1]` |  |
| `writer` | FAIL | `paste` | `[0, 0, 0, 0]` | verb dispatched and moved the document; redo/undo pair did not read two different documents |

**24/35.** The eleven FAILs, attributed:

| # | kinds | why | owned by |
|---|---|---|---|
| 1 | `flow`, `sequence`, `procedural` | the `Child`/`Config`-lane `applied` defect of §4 — all three read `mutated: true` with the ledger growing `[1,3,6,9]` / `[2,4,6,9]` / `[1,3,3,3]` while `#s-checkin` never moves | **fixed in source by this slice**, needs the guest rebuild |
| 2 | `norm` | `action 'setSnapshot' is not a framework-reserved action` — verbatim S8 §4.3, the guest trait default, on a correctly targeted `norm-din16798` | **fixed in source by S9** (+ S10's two-shape macro, §5), needs the guest rebuild |
| 3 | `playbook-module-procedural` | `UI dispatch rejected action:importSolidGeometry with interactive-job classification …` | **fixed in source by this slice** (§3), needs the guest rebuild |
| 4 | `space` | `createStudio` refused by the store fold contract (§1.2) and `set-cell` is not a Home action at all | **fixed in source by this slice** (§1.2), needs the guest rebuild |
| 5 | `energy` | `rename-zone` still stages nothing the probe can fill; the row that "mutates" is `set-surface-property`, whose applied rows are the shell's own | **open** — a probe gap, §7 |
| 6 | `sourcing` | its best verb `stockFromCatalogue` is declared `HostOnly` — it publishes no store lane at all, so no `#s-checkin` count can ever move for it | **open, and not a defect of the oracle**: the verb is host-only by declaration |
| 7 | `stdio`, `trinity`, `writer` | `mutated: true` on a parent-`Artifact`-lane verb (`patchNodes` is `Artifact`) with `edits [0,0,0,0]` — a DIFFERENT shape from group 1, not cured by §4 | **open**, named with evidence, §7 |

Groups 1–4 are seven kinds whose root is fixed in source and whose guest has not been rebuilt, so the
honest projection is **24 → up to 31/35** once the rebuild lands; that number is not claimed until it
is measured. Groups 5–7 are four kinds this slice did not fix.

### 2.6 Two more probe defects found and fixed while measuring

| defect | evidence | fix |
|---|---|---|
| `norm` could not be spawned at all | `no spawn.norm palette entry` **while the registry offered thirty norm programs** — the probe only ever looked for the bare `spawn.<pluginId>` id | fall back to `spawn.<pluginId>.…`, and a new `DEFAULT_APPS` map naming the exact app a kind must spawn as |
| the wrong norm standard was spawned | with the fallback alone it opened `norm-en1990`, not the `din16798` the acceptance names | `DEFAULT_APPS.norm = "s.norm.din16798@1/*#editor"`, and the palette query is narrowed to the app id's own kind token (`din16798`) so its row is rendered before the exact-id locator runs. Measured: `windows = ['norm-4::norm-din16798-inputs','norm-4::norm-din16798-results']` |

### 2.7 After the rebuild — **NOT MEASURED**, blocked on the mutex (§7.1, §7.9)

### 2.8 U3b's locale gap: the stale guests carry NO terminology axis at all (measured)

`🐍️s10-locale-history.mjs` (new) signs in, enters a studio, spawns `dag`, drives `addNode`, raises
the History panel and photographs it, then switches the shell locale through the palette's own
`os.setLocale` verb and photographs it again. On the pre-rebuild guests:

```
studio windows ["space-3::s-workflow","space-3::s-media-vfs","space-3::s-compiled-dag"]
spawned        ["dag-4::dag-main","dag-4::dag-compiled-dag"]
verb ok
EN rows        ["", "", "", ""]        ← four rows RENDER, every label EMPTY
NOTES          []                      ← and zero pageerrors
```

That is the exact behaviour §2.1's contract promises and the sharpest possible statement of U3b's
gap: the host already speaks `HistoryEntry.label: LocalizedLabel`, every staged guest still sends the
old shape, so **every** ledger label resolves to nothing. Before §2.1's fix this same state threw and
unmounted the shell; after it, the rows are visibly wrong and the product keeps running. A German
screenshot taken now would photograph four empty rows and prove nothing, so it is deferred to the
rebuild (§7.5). Also recorded for whoever takes it: the palette's `os.setLocale` row opened no
option list headlessly (`de option count=0`), so the locale toggle needs a different lane than a
palette click — the screenshot is not a one-liner.

## 3. `playbook-module-procedural` — root-fixed, not bypassed

S8 §4.3 recorded `UI dispatch rejected action:importSolidGeometry` and attributed it to
`InteractiveJobClassification::BatchOnlyPendingRewrite`, the same class S4 cured for
`applyDirectoryEventPage` by declaring `Migrated`. The app's own comment said why it could not
simply be relabelled: *"These two verbs own no bounded tool-job factory here, so
`BatchOnlyPendingRewrite` is the truthful disposition until one exists."* `Migrated` without the
factory is refused twice over — by `ArtifactToolFactoryRegistry::register`
(`interactive-job.owner-classification`) and, for an `Artifact`-lane verb with no publication
authority, by `interactive-job.publication-authority-missing`.

So the factory was written, not the label changed
(`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs`, new `//#region 🧵️RetainedCommands`
plus four `ArtifactApp` hooks):

| piece | detail |
|---|---|
| `MODULE_RETAINED_TOOL_IDS` | `["exportSolidGeometry", "importSolidGeometry"]` — the two declared verbs |
| `MODULE_RETAINED_PUBLICATION_CONTRACTS` | both `Artifact`: each handler answers `Emit::mutations(vec![SetPayload(..)])` and the app declares `NoConfig`, so no other lane is reachable |
| `ModuleRetainedCommandJobFactory` | `ToolJobFactory` (`classification() == Migrated`) + `ArtifactOwnedToolJobFactory` with `Owner = ModuleApp` — the plain-`ArtifactApp` shape `🪐️space`'s `SpaceCommandJobFactory` uses, not the `EditorApp<_>` one |
| `bounded_first_step_tool_proofs!` | the proof rows `validate_tool_job_rows` joins the live registry against |
| `build_artifact_store_one_item_preparation_factory` | the framework's own generic `bounded_config_store_one_item_preparation_factory::<Snapshot, Mutation>` — a publication authority on the artifact lane, without which `Migrated` alone only moves the refusal one stage later |
| `register_tool_job_factories` / `build_tool_job` | registration + the `ArtifactRetainedCommandPayload` ladder |
| `.action_interactive_job(…)` | `BatchOnlyPendingRewrite` → **`Migrated`** for both verbs, with the stale comment rewritten |
| `📦️packages/🦀️rust/Cargo.toml` | `semio-framework-job` added (the factory signature names `Operation`) |

`cargo check -p semio-s-plugin-playbook-procedural` → **Finished, 0 errors** (warnings in this file: 0).

Runtime proof is behind the one wasm rebuild — **NOT MEASURED** (§7.1).

## 4. `flow`, `sequence`, `procedural` — ONE root under all three

### 4.1 What the three have in common (measured by reading, at file:line)

PB3 §3.6 named flow's half: a `Child`-lane edit *"is journalled and revertible but invisible to the
parent's edit count"*. The lanes of the three failing verbs are:

| kind | verb | declared publication lane | file:line |
|---|---|---|---|
| 🌊️flow | `addWidget` | **`Child`** | `✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs:1576` |
| 🎬️sequence | `addStep` | **`Child`** | `✏️s/🔌️plugins/🎬️sequence/…/✏️editor/🦀️.rs:1423` |
| 🌀️procedural | `generate` | **`Config`** | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/…/✏️editor/🦀️.rs:538` |

and the host's `#s-checkin` count is
`entry.kind === "mutation" && entry.applied !== false`
(`🏛️ShellHost/🟦️.tsx:9561-9572`). So all three turn on ONE field: `applied`.

### 4.2 The root

`build_history_view` (`🧰️framework/…/🔌️plugin/🦀️.rs:24584`, before):

```rust
let applied = entry.edit_id.as_deref().is_some_and(|edit_id| applied_ids.contains(edit_id));
```

`applied_ids` is the PARENT document store's applied stack alone. A `Child`-lane row carries no
`entry.edit_id` at all (its ids are in `entry.child_edit_ids`) and a `Config`-lane row carries its id
in `entry.config_edit_ids` — so **both report `applied: false`**, which the host reads as *undone*:
the History panel dims the row (`dimmed: entry.applied === false`, `🏛️ShellHost/🟦️.tsx:9783`) and
`uncommittedEditCount` never counts it. The very next expression, `revertible`, already consulted all
three lanes — PB2 taught it `child_applied` and B1 taught it `config_applied` — which is why these
rows were simultaneously *revertible* and *unapplied*, a state that cannot be true.

**Fixed** — the same law for `applied`, four lines later:

```rust
let applied = document_applied || config_applied || child_applied;
```

(the old expression is renamed `document_applied` and `revertible`'s first clause keeps reading that
narrower one, which is correct: only a parent-document edit can be reverted through the parent's own
inverse.)

`cargo check -p semio-framework-plugin -p semio-s-artifact-flow-flow -p semio-s-artifact-sequence-sequence -p semio-s-artifact-space-home`
→ **Finished, 0 errors**.

### 4.3 The missing law (landed)

`🧰️framework/…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`,
`config_lane_row_reports_itself_applied_so_the_host_can_count_it`: dispatches the test app's
`Config`-lane `select`, asserts the row is config edit-linked, carries **no** parent `edit_id` (the
exact shape the old predicate missed), and reads `applied == true` **and** `revertible == true`.

### 4.4 The law is written but NOT executed (honest)

`cargo test -p semio-framework-plugin --lib config_lane_row_reports` does not build: the peer
refactor of §1.5 has invalidated **12 lines of that same test file** (`LocalizedLabel: From<&str>`,
`LocalizedLabel::as_str`, `LocalizedLabel::contains` at lines 4662, 4779–4780, 4857, 4910, 4954,
5051, 5066, 5122, 5153, 5187, 5339). None of the 23 errors falls in this law's own lines
(4337–4358), and `cargo check -p semio-framework-plugin` (lib) is **green**, so the product change
compiles and only the peer's test tree is red. The law is therefore **verified by compilation of the
lib and by reading, not by execution** — it must be run once the peer's refactor lands.

### 4.5 Runtime — **NOT MEASURED**, blocked on the guest rebuild (§7.1)

## 5. `norm` — the refusal reproduced exactly, and S9's macro repaired for two more editors

### 5.1 The before, on the app the acceptance names (measured)

With §2.6's app targeting, `norm-din16798` spawns for the first time in this ticket
(`🗑️generated/s6-sweep-s10e3.txt`):

```
windows = ['norm-4::norm-din16798-inputs', 'norm-4::norm-din16798-results']
setSnapshot refused: dispatch-failed (user window=norm-4::norm-din16798-inputs)
  — action 'setSnapshot' is not a framework-reserved action (history/clipboard/revert/…)
evaluate    refused: … same sentence
```

Verbatim S8 §4.3, and exactly S9 §3.2's attribution: the sentence is
`ArtifactEditor::command_from_action`'s own trait default raised by the GUEST, so the staged
`din16798` guest still predates S9's bridge. The bridge is in source (14 editors carry
`norm_command_from_action!`), so this is a staging gap, not a code gap.

### 5.2 S9's macro knew only one of the two `setSnapshot` payload shapes (fixed)

U3b's 106-crate sweep left `semio-s-artifact-norm-en1990` and `-din18599` red. Cause: thirteen norm
editors declare `ReplaceSnapshot { snapshot: XSnapshot }` and decode the shell's camelCase JSON into
it, but `⚖️en1990` and `⚡️din18599` declare `ReplaceSnapshot { text: String }` — their snapshot types
stopped implementing `dsl::DslField` when `q_k` / `climate` became composed `ArtifactChild<S>` slots,
so their payload carries the artifact's own `.en1990` / `.din18599` DSL text on one op-text line.
S9's single-arm macro expanded into the wrong body for both.

Fixed in `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` with a second macro arm,
`norm_command_from_action!($command, text)`, placed **before** the `$decode:path` arm (a bare `text`
token would otherwise match `$decode:path` and expand wrongly). It reads a `text` argument, falling
back to `snapshot`, and passes it verbatim — the handler's own `unescape_op_text_field` is the
identity for text carrying no backslash escapes, so both a plain and an escaped caller arrive
correctly; `escape_op_text_field` itself is `#[cfg(any(test, feature = "compliance-testing"))]` and
is deliberately not reached from the component build. The two editors now invoke
`norm_command_from_action!(XCommand, text)`.

**All fifteen norm crates check green**, in batches with the private target dir
(`🗑️generated/s10-norm-check-{1,2,3}.txt`):

```
cargo check -p semio-s-artifact-norm-en1990 -p …-din18599                    → Finished, 0 errors
cargo check -p …-din4108 -p …-din16798 -p …-din18599 -p …-en1990 … (8 crates) → Finished, 0 errors
cargo check -p …-en1995 … -p …-vdi3805                          (7 crates)   → Finished, 0 errors
```

### 5.3 `setSnapshot` landing inside `s` — **NOT MEASURED**, blocked on the guest rebuild (§7.1)

## 6. S9 §4–§6 — filled, in S9's own report

Written into `📓️s9-home-studios-and-sweep-forms.md` and marked **"S10 fills"**:

- **§4 `playbook-module-procedural`** — S9 left it empty; filled with §3's root fix and why the
  `Migrated` label alone could not have worked.
- **§5 Honest gaps** — five entries: S9 §2.5's "After" is now measured and S9's `sessionIdentity`
  fix is **confirmed live**; S9 §3.4's sweep number was never S9's to claim and is now this slice's
  24/35; S9 §3.1's three-kind attribution was right and its cause is ONE root, not three; S9's
  `norm` bridge is compiled but still unproven at runtime; and the `historyEntryLabelText` crash
  appeared after S9 ended and invalidates any sweep number taken in its window.
- **§6 Files changed** — corrected: two further defects in the shared sweep probe (S9's `signIn`
  `goBack`, `enterStudio`'s unfiltered palette look) are this slice's, not S9's; S9's staged-form
  filler is confirmed working and is what exposed the third shape §2.3 implements. **No S9 product
  change was reverted or edited.**

## 7. Honest gaps

1. **The guest rebuild did not land inside this slice.** `📜️s10-restage.sh` (corrected, see below)
   has been queued on the fleet wasm mutex since 12:04; at 13:04 the holder was `rb1` (since 11:56,
   1 h 07 m) and the queue was `rb1, tc3c, pz1, s10` — **fourth**. So every runtime clause that
   depends on a guest — `createStudio` (§1.4), `importSolidGeometry` (§3), the `applied` lanes
   (§4.5), `setSnapshot` (§5.3) and the sweep's 24 → 31 projection (§2.7) — is **fixed in source and
   unproven at runtime**. Nothing in this report claims otherwise.
2. **The first `📜️s10-restage.sh` was wrong and wasted its hold.** `bun ./📜️script.ts activate s
   react dev` only PUBLISHES staged components — it computes a receipt over what is on disk and never
   builds — so it answered `60 completed components (unchanged)` in seconds, and a sweep immediately
   afterwards read the identical `edits [0,0,0,0]`. Its staleness view is keyed on `✏️s/🔌️plugins/**`,
   so a change in `semio-framework-plugin`, which every guest LINKS, marks nothing stale at all. The
   script now builds each of the six components through cargo (whose dependency tracking does see the
   framework crate), materializes each, and only then activates. **Recorded because it is a trap any
   slice changing framework code will hit.**
3. **The `applied` law is written but not executed** (§4.4): the peer `LocalizedLabel` refactor has
   12 other lines of that same test file red. `cargo check -p semio-framework-plugin` (lib) is green.
4. **Four sweep FAILs this slice did not fix**, each named with its evidence (§2.5 groups 5–7):
   `energy` (the probe still cannot stage `rename-zone`; the live-id resolution finds no control to
   fill), `sourcing` (its best verb is declared `HostOnly` — it publishes no store lane, so no
   `#s-checkin` count can move; not an oracle defect), and `stdio` / `trinity` / `writer`, which
   report `mutated: true` on a parent-`Artifact`-lane verb with `edits [0,0,0,0]` — a **different**
   shape from §4's, not cured by it and not diagnosed here.
5. **The German History screenshot the coordinator asked for is blocked on the same rebuild.** With
   the stale guests the labels carry no terminology axis at all, so after §2.1's fix they render
   correctly as EMPTY; a locale toggle would photograph empty rows, which proves nothing. It must be
   taken after the re-activation.
6. **`semio-s-plugin-flow`'s test red is a peer's, not this slice's** (coordinator item 2).
   `cargo check -p semio-s-plugin-flow --all-targets` is **green**. The red is in
   `semio-s-artifact-flow-flow` (lib test): `cannot find module or crate semio_framework_async` at
   `…/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs:43`, in a file carrying **102 uncommitted insertions**
   that this slice never touched — a missing `semio-framework-async` dev-dependency for new peer test
   code, not the `host_snapshot` → `fixture` rename the hand-off described (no partially-renamed
   `fixture` symbol exists in that crate). This slice's flow work is entirely in
   `🧰️framework/…/🔌️plugin/🦀️.rs`; no flow plugin file was edited.
7. **`historyEntryLabelText` is fixed in the host, which unblocks the fleet's serves NOW** — the
   coordinator's note that "every serve throws in `historyEntryLabelText` on first History render" is
   this slice's §2.1, and the guard is live TS requiring no rebuild. What the re-activation adds is
   the *content* of those labels; the crash is already gone (measured, §2.1 "After").
8. Hub `7641` and serve `6071` were started twice by this slice (03:39 and, after the 04:10 outage,
   10:28) and nothing of any peer's was stopped. No `git commit`/`stash`/`checkout` was run, no
   `🗑️generated` file this slice did not create was removed, and `📌️important.md` / `🎫️ticket.json`
   were not touched.

## 8. Files changed

**Product source (6 files, 5 root fixes):**

| file | change | § |
|---|---|---|
| `✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs:624` | `admit_space_retained_mutation` declares `for_one_invertible_item` instead of `work_items: 1` — the store authority for all FOUR space lanes, so every durable home/studio/space-index gesture was fail-closed | §1.2 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24584-24722` | `build_history_view`'s `applied` asks all three publication lanes (`document_applied \|\| config_applied \|\| child_applied`), as `revertible` already did; the old expression renamed `document_applied` | §4.2 |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1838-1855` | `historyEntryLabelText` reads BOTH axes defensively — an unfilled terminology axis rendered one row empty instead of throwing inside a render `useMemo` and unmounting the shell | §2.1 |
| `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` | new `//#region 🧵️RetainedCommands` (tool ids, publication contracts, `ModuleRetainedCommandJobFactory`) + four `ArtifactApp` hooks + `Migrated` for both verbs | §3 |
| `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-job` dependency (the factory signature names `Operation`) | §3 |
| `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` | second `norm_command_from_action!` arm for the `ReplaceSnapshot { text }` shape, ordered before the `$decode:path` arm | §5.2 |
| 2 × `✏️s/🔌️plugins/📕️norm/🗿️artifacts/{⚖️en1990,⚡️din18599}/…/✏️editor/🦀️.rs` | invoke the `text` arm | §5.2 |

**Laws (3 files):**

| file | laws |
|---|---|
| `✏️s/🔌️plugins/🪐️space/🧪️tests/🔬️retained-store-footprint/🦀️.rs` (new, + registration in `🪐️space/🦀️.rs`) | the declared footprint covers `inverse + 1` staged rows — **1 passed** |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🏷️history-entry-label/🟦️.ts` (new, + the kernel vitest `include`) | five cases: both axes read; unfilled locale empty; unfilled terminology and absent label empty **and non-throwing**; unknown terminology empty — **5 passed** |
| `🧰️framework/…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | `config_lane_row_reports_itself_applied_so_the_host_can_count_it` — **written, not executed** (peer-red file, §7.3) |

**Ticket folder (permanent probes and scripts):**
`🐍️s6-all-kinds-sweep.mjs` (shared — live-document staged values, `DEFAULT_APPS`, the `signIn` and
`enterStudio` repairs, the `spawn.<plugin>.…` fallback), `🐍️s10-studio-entry.mjs`,
`🐍️s10-boot-diagnose.mjs`, `📜️s10-restage.sh`. Captures: `🗑️generated/s10-*`,
`🗑️generated/s6-sweep-s10{a,b,c,d,e,e2,e3,-before,-mid}.txt`.

**Also updated:** `📓️s9-home-studios-and-sweep-forms.md` §4–§6 (§6 of this report).
