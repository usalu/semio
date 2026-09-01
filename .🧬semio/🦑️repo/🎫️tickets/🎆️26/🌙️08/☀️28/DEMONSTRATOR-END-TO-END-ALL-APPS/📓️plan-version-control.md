# Version-control implementation plan — six "Entwerfen mit Bestand" demonstrator apps

Evidence-based, file:line-cited investigation. Every claim below was directly verified by reading
the cited file at the cited line during this pass (2026-08-28). Where a prior exploration note in
this ticket folder (`📓️explore-version-control.md`) turned out to be stale or imprecise, that is
called out explicitly — **do not carry its conclusions forward**.

Repo root: `/Users/ueli/Documents/semio` (all paths below are relative to it).

---

## 0. Headline correction to the ticket's working assumption

The ticket frames this as "make six apps version-controlled," implying undo/redo/history/document
round-trip must be built per app. **That premise is largely false for local undo/redo/history.**
Every one of the six editors is instantiated as

```
VcsArtifactApp<EditorApp<XPlayApp>>
```

(see §4/§5), and `VcsArtifactApp<A, M>` — a single generic wrapper living in the plugin SDK,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — already implements, once, for
**any** `A: ArtifactApp`:

- `"undo"` / `"redo"` / `"commitCheckpoint"` action interception (line 20204-20206, dispatch at
  line 21498-21500) — an app's own `handle`/mutation code is **never reached** for these three verbs.
- `history_snapshot` / the running `HistoryPatch` projection (line 24249, delegating to
  `history_patch`).
- `document_pack` / `load_document_pack` / `document_text` / `load_document_text` (lines
  24409-24450), delegating to `store::print_document_pack` / `store::parse_document_pack` over
  `A::Snapshot` + `A::Mutation`.

So the real gaps are narrower and different in kind from "implement undo/redo per app":
1. A **framework/shell-level** document-round-trip wiring gap (the host never calls the correctly
   named client methods that already exist end-to-end) — see §6. This affects all six apps equally
   because it is above the per-app layer.
2. Three of six apps are missing the `mod+z` / `mod+shift+z` **keybindings** (the underlying action
   works from the History-panel buttons regardless) — a small per-app polish item, see §5/§7.
3. One in-flight repo-wide filename convention (`🦠️mutation/🦀️component.rs` → `🦀️.rs`) that touches
   every app's mutation folder — reported as observed, not as something to fix in this ticket, see §4.

---

## 1. WIT contract: no current `read-app-document`/`load-app-document` export

**Searched:** every `*.wit` file in the repo (`find . -iname "*.wit"`, excluding `node_modules`).
Only three live `.wit` files exist outside ticket-capture fixtures:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/🧬️schema/📜️world.wit`

Grepping `component.wit` for `read-app-document`/`load-app-document`/`load-app-document-pack`
returns exactly **one** hit, a doc comment, not a declaration:

`🧬️schema/📜️component.wit:1056-1058`:
> "MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §1). The one turn-loop entry point
> every actor exports — replaces `exchange`, the poll-backbone + refresh-ui heartbeat, and every
> per-verb surface (`handle-action`, `handle-command`, `update-window`, `refresh-ui`,
> `context-menu`, `apply-mutations[-text]`, `read/load-app-document-{text,pack}`,
> `attach/detach-backbone`, `consume/produce-media`)."

**Conclusion:** `read-app-document`/`load-app-document{-text,-pack}` were WIT-level exports in a
**pre-"B1 world-collapse"** design. They were deleted and replaced by the single export the world
now has:

`interface reactor` (`component.wit:1063` onward):
```wit
poll: async func(events: list<event>, command-page: option<command-ingress-page>, budget: budget) -> result<turn-result, plugin-error>;
```

There is **no current WIT function** named `read-app-document`/`load-app-document`. The document
round-trip today rides `poll`'s `command-ingress-page` as a typed `AppCommand::ReadDocument` /
`AppCommand::LoadDocument` payload (see §6), not a dedicated export.

### Host call sites the ticket asked about (`ShellHost/🟦️component.tsx` ~5460-5490)

That exact line range (5460-5490) is the **undo/redo/checkpoint auto-check-in machinery**, not a
document-read/load call site (see §3 — the doc-round-trip call sites the ticket assumed live there
are actually at lines 1429-1437, 2953-2967, 3806-3936, 4039; verified by direct grep, see §6).

---

## 2. Reference implementation: the flow plugin — corrected

The ticket assumes flow's undo/redo is a bespoke pattern to copy (`FlowHost` "history stacks").
**This is only partially true and needs correcting before anyone copies it:**

- `FlowPlayApp` (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1550-1554`)
  declares `type Snapshot = FlowSnapshot; type Mutation = FlowMutation;` exactly like the six
  demonstrator apps, and is wrapped in the same generic `VcsArtifactApp<EditorApp<FlowPlayApp>>`
  (`✏️s/🔌️plugins/🌊️flow/🦀️component.rs`). Its `mod+z`/`mod+shift+z` keybindings are registered at
  `🦀️component.rs:1932-1933` the identical way as procedural3d/process3d/gismap.
- `FlowHost` (a **different, framework-level** struct, `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs:135`)
  is an in-memory, gesture-coalescing mirror of the live graph used for direct-manipulation editing
  (drag, inline note edit) with its own camera-preserving `begin_change()` (line 1959),
  `undo()` (line 1989), `redo()` (line 2008), `can_undo()`/`can_redo()`. Critically, its own doc
  comment says explicitly (`🌿️vcs/🦀️component.rs:17-18`):
  > "because `FlowHost`'s own undo/redo (see `impl FlowHost`'s `🔖️History` region) dispatches
  > through them [`create_document_envelope`/`ArtifactCommand`] in every build."
  And indeed `FlowHost::undo()`/`redo()` (lines 1989-2019) call
  `store.dispatch(ArtifactCommand::Undo)` / `ArtifactCommand::Redo` — **the exact same generic
  command** `VcsArtifactApp::history_command()` maps the `"undo"`/`"redo"` action strings to
  (`🔌️plugin/🦀️component.rs:20204-20205`).

**Conclusion:** `FlowHost` is a convenience UI-state mirror layered *on top of* the generic
mechanism for interactive-gesture ergonomics (camera preservation across undo, gesture coalescing
so a drag is one undo step) — it is **not** a parallel undo/redo engine and is **not required** for
the six apps to get undo/redo. Nothing needs to be "copied" from flow for baseline undo/redo/
history/document-round-trip; only the optional gesture-coalescing UX pattern is flow-specific, and
only worth adopting if an app has continuous-drag mutations it wants to coalesce into one undo step
(candidate: cad, puzzle3d — see §7).

---

## 3. Host side: `applyHistoryPatch`, the auto-injected History panel, undo/redo dispatch

**State + patch application** — `ShellHost/🟦️component.tsx:1116-1129`:
```
1116: const [historyProjection, setHistoryProjection] = useState<{ readonly cursor: number; readonly entries: Readonly<Record<number, HistoryEntry>>; readonly canUndo: boolean; readonly canRedo: boolean; readonly currentCheckpointId: string | undefined }>({ cursor: 0, entries: {}, canUndo: false, canRedo: false, currentCheckpointId: undefined });
...
1119: const applyHistoryPatch = useCallback((patch: HistoryPatch | undefined, replace = false) => {
1120:   if (!patch) return;
1121:   setHistoryProjection((current) => {
1122:     if (!replace && patch.cursor <= current.cursor) return current;
1123:     const entries = replace ? {} as Record<number, HistoryEntry> : { ...current.entries };
1124:     for (const entry of patch.upserts ?? []) entries[entry.seq] = entry;
```
Wire shape a plugin must emit for entries to appear (`🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1367-1385`, mirrored 1:1 from Rust `HistoryEntry`/`HistoryPatch` at `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:682-722`):
```ts
export type HistoryEntry = { readonly seq: number; readonly actionId: string; readonly label: string; readonly kind: string; readonly timestamp: string; readonly opLines?: readonly string[]; readonly applied?: boolean; readonly revertible?: boolean; readonly count?: number; };
export type HistoryPatch = { readonly cursor: number; readonly upserts?: readonly HistoryEntry[]; readonly canUndo?: boolean; readonly canRedo?: boolean; readonly activeAlternativeId?: string; readonly currentCheckpointId?: string; readonly commandFilter?: string; };
```
This `HistoryPatch` is produced generically by `VcsArtifactApp::history_patch`/`history_snapshot`
(`🔌️plugin/🦀️component.rs:24249`) on every accepted invocation and on the initial
`plugin.readHistory(instanceId)` call the shell fires once per session
(`ShellHost/🟦️component.tsx:1148-1157`, guarded against refiring per the lane-5-A fix noted in
its own comment at lines 1141-1148).

**Auto-injected History panel + undo/redo/checkpoint buttons** — `ShellHost/🟦️component.tsx:5514-5545`:
```
5514: const frameworkUtilitiesHistoryTab = useMemo((): PanelTabNode | null => {
...
5533:   { id: "framework.history.undo", label: "", control: <button type="button" disabled={isViewer || !historyProjection.canUndo} onClick={() => onAction({ controllerId: session.app.controllerId, action: "undo" })}>Undo</button> },
5534:   { id: "framework.history.redo", label: "", control: <button type="button" disabled={isViewer || !historyProjection.canRedo} onClick={() => onAction({ controllerId: session.app.controllerId, action: "redo" })}>Redo</button> },
...
5544:   { id: "framework.history.checkpoint", label: "", control: <button type="button" onClick={() => onAction({ controllerId: session.app.controllerId, action: "commitCheckpoint" })}>Checkpoint</button> },
```
This tab is built from `session.app.panelTabs.find(... FRAMEWORK_PANEL_TAB_HISTORY_ID ...)` — i.e.
**every** app manifest gets this panel automatically (confirmed generically injected per the
comment at line 5511: "the framework-injected `framework.panel.history` panel tab (every app gets
one — see `AppBuilder::build_definition`)"). No per-app UI work is needed for the History panel
itself.

**Dispatch path from click to Rust** — `onAction` (`ShellHost/🟦️component.tsx:3426`) routes through
`session…handleAction(session.instanceId, encodeWindowActionInvocation(...))` (e.g. line 3687) →
wire `AppCommand::Command`/action envelope → `PluginApp::handle_action_invocation`/`handle_action`
on the guest → `VcsArtifactApp::handle_action` (`🔌️plugin/🦀️component.rs:24034-24037`) →
`dispatch_action` → `history_command("undo"/"redo"/"commitCheckpoint", ...)`
(`🔌️plugin/🦀️component.rs:20197-20208`) → `ArtifactCommand::Undo`/`Redo`/`CommitCheckpoint` against
`self.store` (a generic `ArtifactStore<A::Snapshot, A::Mutation>`), never touching the app's own
`handle`/mutation code.

---

## 4. Mutation/envelope layer: what's generic vs. per-app

### `MutationEnvelope` / inverse / `UndoPolicy` (generic, framework)
- `struct MutationEnvelope` — `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️component.rs:34-42`:
  `mutation_id`, `document_id`, `actor`, `dependencies`, `diff: ArtifactDiff`,
  `inverse: InverseMutation`, `timestamp`. Both `ArtifactDiff`/`InverseMutation` are schema-tagged
  opaque binary payloads (lines 46-54).
- `UndoPolicy` (imported at `🔌️plugin/🦀️component.rs:236`, used at line 20312/20691) — used when
  replaying edits into the running `HistoryView` cache; default `UndoPolicy::ExactBaseOnly`
  (strict — undo only applies against the exact base version), confirmed at line 20741/32190.
- `rollback_envelope` — the **collaboration-sync** rollback (distinct from local undo/redo, used
  when the hub rejects/transforms a speculative outbound batch): Rust
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:788-799`, TS twin
  `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:398` (test at line 1205-1207 confirms parity:
  "rollbackEnvelope synthesizes an undo from the original inverse").
- `commitCheckpoint` / `ArtifactCommand::CommitCheckpoint` — generic, dispatched the same way as
  undo/redo (`🔌️plugin/🦀️component.rs:20206`, job factory at line 14157/14346).
- The per-mutation `apply`/inverse **contract** each app's `Mutation` type must satisfy is
  `protocol::Mutation<P>` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:105`),
  requiring `Clone + Serialize + DeserializeOwned` plus (per the `ArtifactApp::Mutation` bound at
  `🔌️plugin/🦀️component.rs:11074`) `PartialEq + OpText + OpBinary`.

### The "vcs plugin" the ticket names is NOT the generic engine — naming collision
`✏️s/🔌️plugins/🌿️vcs/🦀️component.rs` is itself just another ordinary editor/viewer **app plugin**
(`VcsApps::Editor(VcsArtifactApp<EditorApp<VcsPlayApp>>)`, line 11-12) for a "vcs" artifact kind —
structurally identical to the six demonstrator apps, not a shared library. The actual generic
version-control engine every app (including this "vcs" plugin itself) rides is the
`VcsArtifactApp<A, M>` type living in the **plugin SDK**
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:22925` onward — the
`impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M>`
block spanning §3's cited methods). There is a **separate**, non-plugin, framework-level VCS
algebra module at `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs` (Author/Change/Checkpoint/
Alternative primitives) — that one is genuinely generic/shared, distinct from both the plugin SDK
wrapper and the `🌿️vcs` app plugin. Three different things share the name "vcs"; do not conflate
them when reading the codebase further.

### Per-app mutation file shape observed on disk RIGHT NOW (pre-rename)
The ticket warns of an in-flight repo-wide rename of mutation component files from
`🦠️mutation/🦀️component.rs` to `🦀️.rs`. Observed current shape, identical across all six apps and
also confirmed for `procedural3d`'s `🌱create-widget` leaf
(`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-widget/`):
```
<mutation-kind>/🔣️payload.schema.json
<mutation-kind>/🦠️mutation/🦀️component.rs      ← payload struct, impl MutationKind<Snapshot, Op>
<mutation-kind>/🦠️mutation/🟦️component.ts
<mutation-kind>/🔺️diff/🦀️component.rs          ← forward diff computation
<mutation-kind>/🔺️diff/🟦️component.ts
<mutation-kind>/↩️inverse/🦀️component.rs        ← inverse computation
<mutation-kind>/↩️inverse/🟦️component.ts
<mutation-kind>/🧪️tests/<scenario>/…
```
The enum-level file `🧬️schema/🧬️mutations/🦀️component.rs` (one per app) wires these leaves in via
`#[path = "..."]` modules and derives the enum-wide `impl protocol::Mutation` via
`#[derive(dsl::Mutations)]` (confirmed at
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:9-12,151-154`,
doc comment: "`#[derive(dsl::Mutations)]` below generates `impl protocol::Mutation`/
`impl protocol::SemanticMutation` by delegating to each payload's own `diff`/`inverse`"). This same
`🦠️mutation/🔺️diff/↩️inverse` triad shape was confirmed present (not yet renamed) for **all six**
apps' `🧬️schema/🧬️mutations/🦀️component.rs` root file — every one still ends in `🦀️component.rs`,
none use the flatter `🦀️.rs` name yet (checked: `ls .../🧬️mutations/` for each app returns
`🦀️component.rs`, not `🦀️.rs`). The only place `🦀️.rs`-shaped mutation files exist on disk today
is inside unrelated `SEMANTIC-MUTATIONS-OVERHAUL` ticket-capture fixtures under `.🧬semio/...`,
which are test-run artifacts, not live app code. **No rename work is needed to unblock this
ticket** — just be aware future work may move these files.

---

## 5. Per-app table

Base path pattern for all six: `<plugin>/🗿️artifacts/<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

| App (pane) | Editor struct | `type Snapshot`/`type Mutation` declared | Mutation count (subdirs in `🧬️schema/🧬️mutations/`) | Wired via `VcsArtifactApp<EditorApp<_>>` | has-undo-redo (generic, via framework interception) | mod+z/mod+shift+z keybinding | has-document-roundtrip (generic `document_pack`/`document_text`, requires `Snapshot: ArtifactDsl+ArtifactPack`) | has-history-emission (generic `history_snapshot`) | Stub/`todo!()`/`unimplemented!()` found in mutations |
|---|---|---|---|---|---|---|---|---|---|
| **generator** (`s.procedural.procedural3d@1`) | `Procedural3dPlayApp` — `.../🧊️procedural3d/.../✏️editor/🦀️component.rs:126-128` | yes | 16 | yes — `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:14,302-303` | **yes** | **yes** (`✏️editor/🦀️component.rs:640-641`) | yes — `ArtifactDsl for Procedural3dSnapshot` in `.../🧬️schema/📸️snapshot/📝️text/🦀️component.rs` | yes | none found |
| **koordinator** (`s.cad.cad@1`) | `CadPlayApp` — `.../📐️cad/.../✏️editor/🦀️component.rs:1668-1669` | yes | 22 | yes — `✏️s/🔌️plugins/📐️cad/🦀️component.rs:12,31-32` | **yes** | **no** (no `mod+z`/`mod+shift+z` binding found in editor file) | yes — `ArtifactDsl for CadSnapshot` in `.../🧬️schema/📸️snapshot/🦀️component.rs` | yes | none found |
| **aggregator** (`s.puzzle.puzzle3d@1`) | `Puzzle3dPlayApp` — `.../🧊️3d/.../✏️editor/🦀️component.rs:6326-6327` | yes | 37 | yes — `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs:14,63` | **yes** | **no** (editor binds `escape`/`delete`/`mod+d`/`tab`/`shift+tab`/`f`, no `mod+z` — `🦀️component.rs:6785-6791`) | yes — `ArtifactDsl for Puzzle3dPlaySnapshot`(impl lives in `.../🧬️schema/🧬️mutations/🦀️component.rs`) | yes | 4 `apply`/`inverse` fns matched by grep, but 0 `todo!`/`unimplemented!` occurrences |
| **aussuchen** (`s.sourcing.curate@1`) | `SourcingCurateApp` — `.../🗂️curate/.../✏️editor/🦀️component.rs:541-542` | yes (Mutation type is `SourcingMutation`) | 5 | yes — `✏️s/🔌️plugins/🪵️sourcing/🦀️component.rs:12,38` | **yes** | **no** | yes — `ArtifactDsl for CurateSnapshot` in `.../🧬️schema/📸️snapshot/🦀️component.rs` | yes | none found |
| **bearbeiten** (`s.process.process3d@1`) | `Process3dPlayApp` — `.../🧊️process3d/.../✏️editor/🦀️component.rs:883-884` | yes | 18 | yes — `✏️s/🔌️plugins/🏭️process/🦀️component.rs:12,30-31` | **yes** | **yes** (`✏️editor/🦀️component.rs:1341-1342`) | yes — `ArtifactDsl for Process3dSnapshot` in `.../🧬️schema/📸️snapshot/🦀️component.rs` | yes | none found |
| **verfolgen** (`s.gis.gismap@1`) | `Gis2dPlayApp` (struct name; artifact/app id is `gismap`) — `.../🗺️gismap/.../✏️editor/🦀️component.rs:256,573-574` | yes | 14 | yes — `✏️s/🔌️plugins/🌍️gis/🦀️component.rs:13,30,41` (Note: this plugin also builds a second, sibling artifact/editor pair, `gisterrain`/`Gis3dPlayApp`, which is **not** the `s.gis.gismap@1` app the ticket names — do not conflate the two when touching this plugin) | **yes** | **yes** (`✏️editor/🦀️component.rs:941-942`) | yes — `ArtifactDsl for GisMapSnapshot` in `.../🧬️schema/📸️snapshot/🦀️component.rs` | yes | none found |

All six also have a real undo/redo **exercise site** in their own editor test module (grep for
`assert_undo_redo_round_trip`/`"undo"` inside each `✏️editor/🦀️component.rs` returned hits for
every app), and none showed `todo!()`/`unimplemented!()`/`NotImplemented` anywhere under their
`🧬️schema/🧬️mutations/` tree (`grep -rn` returned 0 for five of six; puzzle3d returned 0 for the
stub markers specifically, its 4 `apply`/`inverse` hits are real implementations, not stand-ins).

**UNCERTAIN / not independently re-verified in this pass:**
- Whether every one of the 16/22/37/5/18/14 mutation kinds per app has a *semantically correct*
  inverse (i.e., whether `undo` restores exact prior state for every kind, not just that the
  `fn inverse` exists and compiles) — that requires running each app's own fixture-backed mutation
  tests, which this read-only pass did not execute.
- Whether `curate`'s `SourcingMutation` enum is shared with sibling `sourcing` extensions
  (`🧩️extensions/🪵️beams`, `🧱️slabs`, `🪟️windows`) in a way that affects its undo semantics —
  flagged for the implementer to check before assuming `curate` is fully self-contained.

---

## 6. The actual document-round-trip gap (framework/shell level, affects all six equally)

This is the one substantive, verified gap and it sits **above** the per-app layer.

**A real, fully-wired, generic document round-trip already exists end-to-end:**
1. TS client — `🧰️framework/🛍️products/💻️os/🟦️component.ts:2326-2338`:
   ```ts
   async readDocument(): Promise<AppFrameValue[]> { return this.sendCommand({ ReadDocument: { seq: this.nextSeq() } }); }
   async loadDocument(pack: Uint8Array, spr: Uint8Array): Promise<AppFrameValue[]> { this.cachedPack = pack; this.cachedSpr = spr; return this.sendCommand({ LoadDocument: { seq: this.nextSeq(), pack: Array.from(pack), spr: Array.from(spr) } }); }
   ```
2. Wire enum — `AppCommand::LoadDocument{seq,pack,spr}` / `AppCommand::ReadDocument{seq}`, Rust
   definition `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:1447,1473-1480`,
   TS mirror `🧰️framework/🛍️products/💻️os/🟦️component.ts:1221-1222`.
3. Guest handler — `🔌️plugin/🦀️component.rs:31045-31057`:
   ```rust
   protocol::AppCommand::LoadDocument { seq, pack, spr } => { let files = ...; match plugin_load_document_pack(runtime, instance_id, &files).await { ... } }
   protocol::AppCommand::ReadDocument { seq } => match plugin_document_pack(runtime, instance_id).await { ... }
   ```
   which call `plugin_document_pack`/`plugin_load_document_pack` (`🔌️plugin/🦀️component.rs:29824-29839`),
   which call `instance.app.document_pack()`/`load_document_pack(files)` — the exact generic
   `VcsArtifactApp` methods from §0/§3.

**But nothing in the demonstrator's shell actually calls `readDocument()`/`loadDocument()`.**
`grep -rn "\.readDocument(\|\.loadDocument("` across every non-`node_modules`, non-`target`,
non-ticket `.ts`/`.tsx` file in the repo returns **zero** call sites outside
`🟦️component.ts`'s own unit tests. `ShellHost/🟦️component.tsx` never calls them either.

Instead, `ShellHost/🟦️component.tsx` still references three **differently-named, always-guarded**
methods — `plugin.readAppDocument`, `plugin.loadAppDocument`, `pluginEntry.handle.loadAppDocumentPack`
— at lines 1429, 1437, 2956, 2960, 3806, 3811, 3828, 3859, 3936, 4039. Every call site is guarded
(`if (plugin.readAppDocument) ...`), and no file anywhere in the live (non-ticket-fixture) codebase
declares a method with any of these three names (`grep -rn "readAppDocument(" . | grep -v
node_modules | grep -v target` returns only these same guarded call sites, no declaration). These
are vestigial names from the pre-B1-world-collapse WIT surface named in §1's doc comment
(`read/load-app-document-{text,pack}`) that were never migrated to the current `readDocument`/
`loadDocument` naming when the ABI was collapsed to `poll`. They are used for exactly two things:
- The **tutorial sandbox** feature (`ShellHost/🟦️component.tsx:3796-3833`): snapshot the live
  document before entering a tutorial, load the tutorial's `base` document, restore the snapshot on
  exit.
- One `Effect::LoadDocument` host-effect handler (`ShellHost/🟦️component.tsx:2953-2967`) — an
  app-initiated request to swap in a new document.

Both are effectively **dead on the current ABI**: since no plugin handle actually exposes
`readAppDocument`/`loadAppDocument`/`loadAppDocumentPack`, every guard evaluates falsy and neither
code path ever executes its intended effect on any of the six apps today. This exactly matches (and
explains) the demonstrator's own comment at `♻️mit-bestand/🧺️demonstrator/📦️index.tsx:207-209`:
> "there is no document round-trip yet (`readAppDocument`/`loadAppDocument` are an unimplemented,
> documented Wave-1 gap in the framework core)"

**Corrected framing:** the gap is not "the framework core has no document round-trip" (it does —
§6 items 1-3), it is "the shell/host layer (`ShellHost.tsx`, and by extension the demonstrator,
which drives six standalone `FrameworkOsShell` instances with no Hub/Space backing) never invokes
it." The prior exploration note's per-app checklist item ("Each app must implement WIT export
handlers + serialize document state") is **wrong** — no per-app WIT handler work is needed; the fix
is entirely in the shell/host TS layer (and, for the demonstrator specifically, in deciding what
"save"/"load" should even mean for an unattended kiosk pane with no backing Hub document).

There is also a separate, more capable `openDocument`/`DocumentHost`/backbone-worker sync mechanism
(`ShellHost/🟦️component.tsx:1298,3322-3361`) used for Hub/Space-connected documents — that path is
real and used elsewhere in the framework, but the demonstrator's per-pane `FrameworkOsShell`
instances are ephemeral/standalone (`♻️mit-bestand/🧺️demonstrator/📦️index.tsx:35-36`,
`isEphemeralShellBrand`), so they are not currently associated with a Hub document and would need
either (a) a minimal `readDocument()`/`loadDocument()` wiring against `scope.storage` (the
per-pane `createScopedStoragePort`, `📦️index.tsx:421`, already used for UI-chrome persistence), or
(b) full `openDocument` Hub/Space wiring, which is a materially bigger scope change.

---

## 7. Concrete, ordered per-app task list

### 7.1 Framework/shell layer (do this first — unblocks all six apps identically)
1. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`:
   - Replace the dead `plugin.readAppDocument`/`plugin.loadAppDocument`/`handle.loadAppDocumentPack`
     guarded call sites (lines 1429-1437, 2953-2967, 3806-3936, 4039) with the real, already-wired
     `plugin.readDocument()`/`plugin.loadDocument(pack, spr)` client methods
     (`🧰️framework/🛍️products/💻️os/🟦️component.ts:2326-2338`).
   - Decide and implement a save/load trigger for the demonstrator's ephemeral panes: on
     `commitCheckpoint`/auto-checkin (the existing `dispatchCheckpoint`/`AutoCheckinScheduler`
     machinery at lines 5459-5476 already fires on the right cadence) persist
     `await plugin.readDocument()`'s pack/spr bytes into the pane's own scoped `StoragePort`
     (`createScopedStoragePort`, `🧰️framework/🔨️modules/🖥️platform/🟦️component.ts:633-639`); on pane
     boot/resume, if a stored pack exists, call `plugin.loadDocument(pack, spr)` before the first
     paint.
   - Update the now-inaccurate comment at
     `♻️mit-bestand/🧺️demonstrator/📦️index.tsx:207-209` once this lands.
2. Add a unit/integration test exercising "edit → commitCheckpoint → simulate reload →
   `readDocument`/`loadDocument` → assert identical snapshot" once per plugin family (procedural3d
   is the cheapest to wire first since it already has a full undo/redo keybinding and an existing
   `assert_editor_and_viewer_share_dialect`-style testkit to extend from).

### 7.2 Per-app: keyboard shortcuts (small, independent, no dependency on §7.1)
3. `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` —
   add `.keybinding("mod+z", "undo")` / `.keybinding("mod+shift+z", "redo")` to the `AppBuilder`
   chain (mirror `✏️s/🔌️plugins/🌀️procedural/.../✏️editor/🦀️component.rs:640-641`).
4. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` —
   same addition; note this file already binds `mod+d`/`tab`/`shift+tab` (lines 6785-6791) so
   confirm `mod+z`/`mod+shift+z` don't collide with any existing brush/duplicate bindings before
   adding.
5. `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` —
   same addition.

### 7.3 Per-app: verify inverse correctness (medium effort, do before calling any app "done")
6. For each of the six apps, run (or write, where absent) a fixture-backed round-trip test per
   mutation kind of the shape already present under
   `.../🧬️schema/🧬️mutations/<kind>/🧪️tests/<scenario>/{🦠️mutation,🔺️diff,📸️snapshot/⬅️before,📸️snapshot/➡️after}`
   (pattern confirmed under procedural3d's `🌱create-widget`), asserting `apply(inverse(diff)) ==
   before`. This is the only way to close the "UNCERTAIN" item in §5 (existence of `fn inverse` was
   verified; semantic correctness for every one of the 16/22/37/5/18/14 kinds per app was not).
   Prioritize `puzzle3d` (37 mutation kinds — the largest surface — and the only app where
   `apply`/`inverse` fn bodies were found directly inline in the mutations-root
   `🦀️component.rs` rather than in per-leaf `🔺️diff`/`↩️inverse` files, so its shape diverges
   slightly from the other five and deserves its own read-through before trusting it).

### 7.4 Optional UX parity with flow (only if product wants it — not required for baseline VC)
7. If `cad`/`puzzle3d` want a single undo-step per continuous drag/gesture (matching flow's
   ergonomics), introduce a `FlowHost`-style gesture-coalescing wrapper
   (`begin_gesture`/`commit_gesture_history` pattern,
   `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs:1966-1982`) that still
   dispatches through `ArtifactCommand::Apply`/`Undo`/`Redo` — do **not** invent a parallel history
   mechanism.

### 7.5 Do not touch as part of this ticket
8. The `🦠️mutation/🦀️component.rs` → `🦀️.rs` rename (§4) is a separate, already-in-flight, repo-wide
   ticket (`SEMANTIC-MUTATIONS-OVERHAUL`); do not fold it into this ticket's diff — it touches every
   plugin in the repo, not just these six.

---

## References (all independently verified, file:line as cited inline above)
- WIT: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit`
- Plugin SDK / generic VC engine: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- History types: `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`, `…/🎠️kernel/🟦️component.ts`
- SPR wire/channel: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs`,
  `🧰️framework/🛍️products/💻️os/🟦️component.ts`
- Mutation/replication primitives: `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️component.rs`,
  `…/📡️replication/🎮️mutation/🦀️component.rs`
- Sync/rollback: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`,
  `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`
- Shell/renderer: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- Flow (framework-level host + vcs bridge, distinct from the flow plugin app):
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs`,
  `…/🌊️flow/🌿️vcs/🦀️component.rs`
- Demonstrator: `♻️mit-bestand/🧺️demonstrator/📦️index.tsx`
- Six apps: `✏️s/🔌️plugins/{🌀️procedural,📐️cad,🧩️puzzle,🪵️sourcing,🏭️process,🌍️gis}` (see §5 for exact
  editor/mutations paths per app)
- Prior (stale in places) exploration note, superseded by this document's §0/§2/§4/§6:
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️explore-version-control.md`
