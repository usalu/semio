# S13 — 35/35 on rebuilt guests, and the hub document inside `s`

Slice S13, session 8, 2026-09-22. Inherits `📓️s12-thirty-five-of-thirty-five-and-hub-document-inside-s.md`
(30/35; §1.1 the five survivors; §3 the generic `bounded_presence_root_retirement_factory<P>` default
that cures 95 editors but needs a 60-component guest rebuild; §2 the hub journey accepted by 7651),
`📓️s11-…` §7 and `📓️status.md` session 8.

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read only.

## 0. Infrastructure this slice inherits (measured 23:1x–23:2x)

| what | value | state |
|---|---|---|
| serve | `s` react dev **6071** (bun `96947`) → hub 7641 | `200` — S11's, not restarted |
| serve | `s` react dev **6072** (bun `8555`) → hub 7651 | `200` — S12's, not restarted |
| hub | **7651** (`os-hub` `68878`), TC3e stdio/gis/note catalog | `/readyz` **200** |
| hub | **7681** (`os-hub` `11917`), HC1's | `/readyz` **200** |
| hub | 7621 (`93292`), 7671 (`99749`) | `200` · 7641 (`96468`) `503` (no published catalog) |
| load | 23:13 **36.5** · 23:25 **41.2** · 23:27 **48.6** (33 cargo, 7 rustc) | rising all slice |
| wasm mutex | held by **play since 22:17:48**; queue `play` → `gj1` → `fl3` → `px1` → (s13 `20260922231000`) | 4 ahead |

## 1. The five survivors — what each one actually is

Re-read at their declarations, against S12 §1.1 and the rail row ids S12 captured
(`🗑️generated/s6-sweep-s12{d,e}.txt`). Two were **probe defects with a named reason**, one is a
**product defect now fixed**, two are **declaration cases**.

### 1.1 🗄️stdio — a product defect: a palette Mutation row no app can bridge (**fixed**)

`stdio` spawns `s.stdio.csv@rfc4180/*#editor`, whose one window is
`TableWindowKit::editable_window_kind()`. That kit mints **`set-cell`** as an
`ActionKind::Mutation` palette row, and `window_kind_definition` stamps every kit action
`InteractiveJobClassification::Migrated` (`🔌️plugin/🦀️.rs:32577-32580`) — so the row is rendered,
enabled and dispatchable. Two things then make it dead for a human:

1. **It declares no arguments.** `ActionDefinition::bounded_catalog` gives `args: Vec::new()`
   (`🛂️manifest/🦀️.rs:993`), so the rail stages nothing, and `set-cell` addresses a cell by
   `row`/`column`/`value`.
2. **No stdio editor bridges it.** Every one of the nine stdio editors answered
   `stdio.<kind>.unhandled-action: action 'set-cell' is not one of this editor's declared verbs
   (setActiveExample)` — measured by reading all nine `*_command_from_action` bodies
   (csv `…/📊️csv/…/✏️editor/🦀️.rs:129`, and the same shape in tsv/txt/md/html/json×2/xml×2).

The framework's own bridge-conformance check **skips exactly these three ids**
(`🔌️plugin/🦀️.rs:7696`, `"replace-text" | "set-cell" | "set-node"`), which is why no test ever saw
it: the skip was written for apps that do not bridge the kit verb, and it hid the apps that should.

**Fixed at both roots** (files in §6): the three editable kits declare their verbs' arguments, and
every stdio editor that owns the matching command bridges it.

### 1.2 🔋️energy — a probe defect: one of two REQUIRED arguments was never staged

`create-zone` declares `name` **and** `volumeM3` as `.required()`
(`…/🪟️windows/📊️zones/🦀️.rs:31-34`), and the sweep staged only `{ name }` (S12's
`DEFAULT_ARGS["energy.create-zone"]`). Its guest arm is otherwise healthy: `volume_m3 <= 0.0` is the
only refusal, `next_entity_id` mints the id, the arm returns `("create-zone", "Create zone {name}")`
and the contract puts it on the `Artifact` lane (`✏️editor/🦀️.rs:1067-1073`, `:1854`). **Re-pinned
with all four fields.**

### 1.3 🔱️trinity — a probe defect: the pin named a verb the rail never offers

S11 pinned `setParameter`; S12 measured `knownVerbOffered: false` and 23 rail rows without it. The
editor's real document verb is **`patchNodes`** — `Artifact` lane (`🔌️jack/…/✏️editor/🦀️.rs:568`),
`Migrated` (`:1112`), three `.required()` arguments `nodeIds`/`field`/`value` (`:1145`). S12 drove it
with **no arguments at all**, so the staged form was incomplete. **Re-pinned to `patchNodes`, with
`nodeIds` resolved live off the spawned graph window.**

### 1.4 ✒️writer and 🪵️sourcing — declaration cases, named at their lines

Both kinds publish their real document verbs with `in_palette: false` **by design**, so no Actions
rail row of theirs can move the document:

| kind | palette Mutation rows | the document verbs, and where they are hidden |
|---|---|---|
| ✒️writer | `formatDocument` (`Artifact`), `lintDocument` (`WindowTransient`), `setActiveExample` (`HostOnly`) | `textEdit`, `setText`, `commitRename`, `setSnapshot` are all minted by `writer_hidden_operation` — `ActionDefinition { in_palette: false, .. }` (`✒️writer/…/✏️editor/🦀️.rs:106`, used at `:1382-1393`) |
| 🪵️sourcing | `stockFromCatalogue` (`HostOnly`), `setActiveExample` (`HostOnly`) | `curationAdd`, `curationSetCount`, `curationRemove`, `dropOnPool`, `dropOnCurated` are all `hidden_operation` (`🪵️sourcing/…/✏️editor/🦀️.rs:1171`, used at `:1272-1279`) although every one of them IS declared on the `Artifact` lane (`:387-391`) |

And `formatDocument`, writer's one palette `Artifact`-lane verb, is **correctly** a no-op on the
seeded document: it emits a mutation only when `format_writer_text(text, language)` differs from the
buffer (`✏️editor/🦀️.rs:476-480`), and the seeded jack document is already canonical. That is the
right behaviour for a formatter, not a defect.

**So `writer` and `sourcing` are the honest residue**: their round trip exists, but it is reached by
a direct-manipulation gesture inside the window (typing; a curated-table stepper; a drag), not by a
rail row. Measured state and the gap are in §5.

## 2. The 60-component guest rebuild (filling)

## 3. The full sweep on rebuilt guests — en + de (filling)

## 4. The hub-document journey (filling)

## 5. Honest gaps (filling)

## 6. Files changed

**Product source:**

| file | change | § |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `TextWindowKit`/`TableWindowKit`/`TreeWindowKit`'s **editable** window kinds declare their verbs' arguments: `replace-text{text}`, `set-cell{row,column,value}`, `set-node{nodeId,value}`. Without them the rail renders a Mutation row a human can press and never stage | §1.1 |
| `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs` | `window_kit_text_argument` / `window_kit_index_argument` — one reader for the kit verbs' staged arguments, admitting a `String` or a `Number` reading and the pre-declaration key spellings | §1.1 |
| nine `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/…/✏️editor/🦀️.rs` (csv, tsv, txt, md, html, json×2, xml×2) | each bridges its own kit verb in `command_from_action` (`set-cell` / `replace-text` / `set-node`) instead of refusing it, and each refusal message names the verbs it really has | §1.1 |
| `✏️s/🔌️plugins/🔋️energy/…/✏️editor/🦀️.rs` | `set-node` accepts the kit's declared `nodeId` beside this editor's own `id` | §1.1 |

**Ticket folder:** `🐍️s13-window-kit-bridge-patch.py` (anchored, idempotent),
`📜️s13-restage-all.sh` (the 60-component chain), and `🐍️s6-all-kinds-sweep.mjs` (shared) gained the
`stdio` → `set-cell` pin with its arguments, the `trinity` → `patchNodes` re-pin, and `energy`'s two
missing required fields.
