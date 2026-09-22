# S11 — every spawnable kind round-trips inside `s`, in two locales, with a hub document open

**Headline, all measured:** the 35-kind sweep goes **24 → 26/35** with `📕️norm` and
`⚙️playbook-module-procedural` passing for the first time in this ticket; **G17 is closed** (a locale
switch now re-renders the WHOLE ledger, law GREEN, before/after photographed); the play session's
`🎬️sequence` diff lands at **205/2 with zero `Dictionary ownership` panics** (from 200/7 with five);
and the hub-document journey is proven to its last link, where the host's own single-origin gate —
which refuses **silently** — is named with its console line.

Slice S11, session 8, 2026-09-22. Inherits `📓️s10-s-host-studios-and-sweep.md` (27/35 claimed,
three guests proven stale by mtime, five undiagnosed FAILs), `📓️s9-…`, `📓️s8-…` §3–§4.

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read only.

## 0. Infrastructure this slice owns

| what | value | evidence |
|---|---|---|
| hub | port **7641**, data root `.🧬semio/🌐hub/s10-boot`, binary `⚡️cache/hs1/os-hub-7611` | inherited from S10, **not restarted** — holder `69504`, child `69521`, alive since 01:29 |
| serve | `s` react dev on **6071** → `http://127.0.0.1:7641` | restarted by this slice onto each fresh staging: 10:59, 15:53, 16:49, 17:50, **20:53** (last: wrapper `96923`, vite `96947`) |
| hub restarts | 7641 was **restarted by this slice at 20:53** after the 18:00 cut, by S10's recorded line (`🐍️ds1-hub-hold.ts 7641 …/🌐hub/s10-boot …/⚡️cache/hs1/os-hub-7611`, `OS_HUB_CREDENTIAL_SIGN_IN=true`) | holder `96463` → `os-hub` `96468`; same `503`/`artifactAuthority` state as §0.1, unchanged |
| humans | `user1@semio.dev` / `gm1-local-dev-pass-1` | S10's provisioning, unchanged |

### 0.1 The 7641 `503` is NOT a fault and needed no restart (measured, 10:58)

`curl -s :7641/readyz` answers `503` with one blocked gate and nothing else:

```
"artifactAuthority":{"ready":false,"reason":"trusted-catalog-never-published-in-this-data-root"}
"blockedBy":[{"gate":"artifactAuthority","reason":"trusted-catalog-never-published-in-this-data-root"}]
```

Every other gate (`directory`, `storage`, `artifactCasBarrier`, `artifactPublication`,
`artifactCasSweeper`, `adminAssets`, bootstrap auth) reads `ready: true`. The data root is **8.4 MiB**
against `gm1-boot` 620 MiB / `jc1-boot` 522 MiB / `hs1-boot` 619 MiB — it holds a directory and
credentials and **no published catalog at all**. So sign-in, spaces and studios work (they did for
every measurement below) and only the artifact-catalog surface is closed. The hub process was
therefore left alone; the pid was not killed.

## 1. The restage S11 was queued for had ALREADY landed — no mutex hold was needed

S10's `📜️s10-restage4.sh` did not die with its session: its own capture
(`🗑️generated/s10-restage4.txt`) ends

```
[s10-restage] (3/3) materialize rc=0 2026-09-22 04:54:51
[s10-restage] activate  2026-09-22 04:54:51
[s10-restage] DONE 2026-09-22 04:54:55
```

with `component rc=0` **and** `materialize rc=0` for all three guests, and the activate log's last
line is `Activated s react dev: 60 completed components (changed)` — `(changed)`, not
S10 §7.2's false-green `(unchanged)`. Proven against S10 §2.8b's own mtime table, which is the
staleness test that slice wrote:

| source fix | source mtime | staged artifact | artifact mtime | fresh? |
|---|---|---|---|---|
| Home presence retirement | 09-21 **20:46** | `semio_s_plugin_space_component.core.wasm` | 09-22 **04:52** | **yes** |
| `TOOL_JOB_IDS` (procedural module) | 09-21 **20:37** | `semio_s_plugin_playbook_procedural_component.core.wasm` | 09-22 **02:46** | **yes** |
| norm `snapshot` arg | 09-21 **20:48** | `semio_s_plugin_norm_component.core.wasm` | 09-22 **04:54** | **yes** |

`🌊️flow` (02:37), `🎬️sequence` (03:03) and `🌀️procedural` (02:43) are fresh too. **So step 2 of this
slice's brief was already satisfied and no wasm-mutex hold was taken for it** — the serve was simply
restarted at 10:59 onto the tree that was already on disk, because the vite process had been running
since 01:29 and predated every one of those artifacts.

Confirmed at runtime, not by mtime alone: the three kinds' refusals all CHANGED on the first sweep
(§2.1), which only a new guest can do.

## 2. The 35-kind sweep, re-measured on this slice's own pair

Every row driven inside the real `s` host on serve `6071` → hub `7641`, signed in as
`user1@semio.dev`, after entering a studio. `edits` is `#s-checkin`'s uncommitted-mutation count at
`[before, verb, undo, redo]`. Captures: `🗑️generated/s6-sweep-s11{a,b,c,d,e,f,g,h}.txt` (the probe's
own JSON) and `🗑️generated/s11-sweep-*.txt` (the console).

### 2.1 What the rebuilt guests changed — three refusals, all NEW (measured 11:00, `s11a`)

The first chunk was run before this slice touched any source, purely to see what the 04:54 restage
moved. All three previously-blocked kinds answered with a DIFFERENT sentence than S10 recorded,
which is the runtime proof that the new guests are live:

| kind | S10's refusal (stale guest) | on the rebuilt guest (this slice) | reading |
|---|---|---|---|
| `playbook-module-procedural` | `UI dispatch rejected action:importSolidGeometry with interactive-job classification BatchOnlyPendingRewrite` | **`presence local read requires a live exact local retirement owner`** | S10 §3's tool factory + `TOOL_JOB_IDS` LANDED; a second root was behind it |
| `norm` | `action 'setSnapshot' is not a framework-reserved action` | **`setSnapshot needs a 'snapshot' argument carrying the document's camelCase JSON`** | S9's bridge and S10's `with_args` codemod LANDED; the probe simply had no value for the argument |
| `space` | `home: unhandled action id set-cell` | `no Actions rail row after unfolding` on `s.space.home@1` | the Home surface offers no rail at all when spawned as a program — §3.3 |

### 2.2 Running result — **25 / 35 PASS** (measured, this slice, 11:00–12:14)

| kind | | verb | `edits` | note |
|---|---|---|---|---|
| `animate` | PASS | `addTile` | `[0,1,0,1]` | |
| `architect` | PASS | `setAdjacencyKind` | `[0,1,0,1]` | |
| `block` | PASS | `addHandleKind` | `[0,1,0,1]` | |
| `cad` | PASS | `addObject` | `[1,2,1,2]` | |
| `dag` | PASS | `addNode` | `[0,1,0,1]` | |
| `demonstrator` | PASS | `changeSchema` | `[0,1,0,1]` | FAILed once under load, PASSed on retry — §6.1 |
| `draw` | PASS | `addLayer` | `[0,1,0,1]` | |
| `energy` | FAIL | `set-surface-property` | `[0,0,0,0]` | no refusal, no ledger row — §3.4 |
| `fem` | PASS | `addNode` | `[0,1,0,1]` | |
| `flow` | FAIL | `addWidget` | `[0,0,0,0]` | **regression vs S10 §2.7** — §3.5, peer-owned files |
| `forms` | PASS | `addStep` | `[0,1,0,1]` | |
| `gis` | PASS | `addFeature` | `[0,1,0,1]` | FAILed once under load, PASSed on retry |
| `imperative` | PASS | `addStep` | `[0,1,0,1]` | |
| `layout` | PASS | `addPage` | `[0,1,0,1]` | FAILed twice under load, PASSed on a third run — §6.1 |
| `lowpoly` | FAIL | `addPrimitive` | `[0,1,1,1]` | verb lands, undo does not retract — §3.6 |
| `mathematical` | PASS | `nodeGraphEdit` | `[0,1,0,1]` | |
| `norm` | **PASS** | `setSnapshot` | `[0,1,0,1]` | **first norm PASS of this ticket** — §3.1 |
| `note` | PASS | `addBlock` | `[0,1,0,1]` | |
| `playbook` | PASS | `addStep` | `[0,1,0,1]` | |
| `playbook-module-procedural` | FAIL | `importSolidGeometry` | `[0,0,0,0]` | presence owners — fixed in source §3.2, needs a rebuild |
| `procedural` | PASS | `addWidget` | `[0,1,0,1]` | |
| `process` | PASS | `addStep` | `[0,1,0,1]` | |
| `puzzle` | PASS | `addNode` | `[0,1,0,1]` | |
| `raster` | PASS | `addLayer` | `[0,1,0,1]` | |
| `reasoning` | PASS | `addNode` | `[0,1,0,1]` | |
| `remodel` | PASS | `addStream` | `[0,1,0,1]` | |
| `sequence` | PASS | `addStep` | `[0,1,0,1]` | S10's `applied` fix holds |
| `shooting` | FAIL | `addShot` | `[0,1,1,1]` | verb lands, undo does not retract — §3.6 |
| `sourcing` | FAIL | `stockFromCatalogue` | `[0,0,0,0]` | `HostOnly` by declaration (S10 §2.5 group 6) |
| `space` | FAIL | `createArtifact` | `[0,0,0,0]` | presence owners — fixed in source §3.3, needs a rebuild |
| `stdio` | FAIL | `paste` (fallback) | `[0,0,0,0]` | `paste` is a reserved clipboard verb, not a document verb — §3.6 |
| `trinity` | FAIL | `textEdit` | `[0,0,0,0]` | `setParameter` not offered — §3.4 |
| `vcs` | PASS | `incrementCounter` | `[0,1,0,1]` | |
| `wfc` | PASS | `resize-output` | `[0,1,0,1]` | |
| `writer` | FAIL | `paste` (fallback) | `[0,0,0,0]` | same as `stdio` — §3.6 |

**25/35 measured** (`layout` PASSed on re-run at 12:14). Only §3.1's fix is in that number.

### 2.3 After the 17:50 restage — **`playbook-module-procedural` PASSES, 26/35** (measured 17:52)

The restage that carries §3.2's second fix landed at 17:50 (`component rc=0`, `materialize rc=0`,
`activate … 60 completed components (changed)`), the serve was restarted onto it, and:

| kind | | verb | `edits` | `applied` | witness |
|---|---|---|---|---|---|
| `playbook-module-procedural` | **PASS** | `importSolidGeometry` | **`[0,1,0,1]`** | `[1,3,4,5]` | `…entry.3:Import Solid↶`, **0 refusals** |
| `sequence` | PASS | `addStep` | `[0,1,0,1]` | `[2,4,7,9]` | `…entry.4:Add Step↶` — no regression from §8's `Drop` |

**`playbook-module-procedural` has never passed in this ticket before.** Its ladder took three roots,
each invisible until the one in front of it was removed: S10's interactive-job classification, this
slice's presence owners, this slice's document-store owners.

**`space` is the one this slice leaves measured-but-not-scored.** Its own verbs DO round-trip on the
rebuilt guest — `renameArtifact` `edits [0,1,0,1]` with `…:Rename Artifact↶` (20:54) and
`touchArtifact` `edits [1,2,1,2]` with `…:Touch Artifact↶` (20:57), both with **no refusal on the
verb itself** — but the sweep would not score it PASS on either run: the first because it then
scanned the framework table-kit's `set-cell` row and counted its `unhandled action id` refusal as
the KIND's fault (fixed — §3.6a's three window-kit ids are now excluded from the scan), and later
runs because the space index's action rail intermittently renders zero rows. Six consecutive runs
between 20:55 and 21:05 split 2 good / 4 empty on the same build, so **the number stays 26/35 and
`space` is reported as what it is: a kind whose document verbs are proven to round-trip and whose
sweep row is not yet reproducible.**

**So the sweep number this slice leaves is 26/35**, and it is not padded with anything unmeasured.

## 3. Roots found and fixed by this slice

### 3.1 `norm` — the probe could not fill a 63-field document, so it now reads the artifact's own fixture (**PASS measured**)

`din16798`'s ONLY `ActionKind::Mutation` verb is `setSnapshot`, and after S10's codemod it correctly
declares one staged argument (`ActionArgDef::text("snapshot", …)`). The sweep had no value for it
(`filled: []`) so the guest refused, correctly, with its own sentence. A literal in the probe would
rot the day the schema moves, so the probe now loads the artifact's **committed fixture** —
`…/🌬️din16798/…/🧫️fixtures/🧬️mutations/🌍️change-annex/🌍️switches-the-check-to-the-en-annex/📸️snapshot/⬅️before/🔣️.json`
— which is the codec's own canonical output and differs from the seeded document in three fields, so
the replace really moves the document.

**The fixture is folded to one line by deleting LINE BREAKS only.** `JSON.parse` → `JSON.stringify`
was the first spelling and is wrong: JavaScript has one number type, so it rewrites the fixture's
`22.0` as `22` and the `pack` codec on the other side reads an f64 carrier's exact JSON form.

```
norm setSnapshot  edits [0,1,0,1]  applied [1,2,3,4]
lastLedgerRow "framework.history.entry.2:Set Snapshot↶"   undoLane rail  redoLane rail
refusals []   faultCount 0   PASS
```

### 3.2 `playbook-module-procedural` — TWO missing retirement catalogs, one behind the other (both fixed)

With S10's tool factory live, the next refusal is
`presence local read requires a live exact local retirement owner` — **the same root S10 cured for
🏠️home (§2.9a) and 📖️playbook (§2.8c), in a third app.** `ModuleApp` declares `Presence = NoPresence`
and no presence hooks at all, so `PresenceStore::local_read` fails closed and the first command whose
ephemeral leg reads local presence is refused however correct the command is.

Fixed at `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` — `build_presence_store_disposer`,
`build_presence_local_root_retirement_factory`, `build_presence_peer_retirement_factory`, all three
the framework's own `NoPresence`-typed owners, exactly as 📖️playbook's editor declares them.

**And behind it, on the rebuilt guest, a second one of the same class (measured 16:00,
`🗑️generated/s6-sweep-s11k.txt`).** The presence sentence is gone and `importSolidGeometry` is now
refused with

```
typed-operation failed: plugin.internal validation failed:
  batched fold lacks exact snapshot or mutation retirement authority
```

— `ArtifactStore`'s batch stage (`🧰️framework/…/🏪️store/🦀️.rs:17288`) copies the store's
`mutation_retirement` / `snapshot_retirement` into its staging buffer and refuses when either is
`None`, which is what `ArtifactApp::build_document_store_owners`'s trait default returns. `ModuleApp`
declared none. **Fixed** with the framework's own one-page catalog
(`bounded_document_store_owners::<Self::Snapshot, Self::Mutation>()`), the hook 🖨️raster, 🌊️flow,
🔱️trinity and 📕️norm each declare for the same reason. Needs the rebuild queued in §6.3 to measure.

So this one kind has now surfaced THREE roots in sequence, each hidden by the one in front of it:
S10's interactive-job classification, this slice's presence owners, and this slice's document-store
owners. That ladder is the honest shape of the work, and each step was measured before the next was
visible.

### 3.3 `space` — the same root again, in the space INDEX editor, under ALL THREE durable verbs (fixed in source)

Two findings, in order.

**(a) The sweep was driving the wrong space app.** `space`'s first program is `s.space.home@1/*#editor`,
and Home spawned as a program renders **no action rail at all** (`railRows: 0`, `railToggles: 1`),
so no verb of any kind was reachable. The space INDEX editor is the app that owns this plugin's
document mutations, so `DEFAULT_APPS.space = "s.space.space@1/*#editor"` and
`DEFAULT_VERBS.space = "createArtifact"` (+ its two staged args) are now pinned in the shared probe.
Measured effect: `railRows 0` → **24**, window `space-3::framework.window.table`.

**(b) With the index reachable, EVERY durable verb it owns is refused by one sentence:**

```
createArtifact  refused: presence local read requires a live exact local retirement owner
renameArtifact  refused: presence local read requires a live exact local retirement owner
touchArtifact   refused: presence local read requires a live exact local retirement owner
```

`SpaceIndexEditor` declares `Presence = NoPresence` and no presence retirement owners — §3.2's root,
in a fourth app, and the one that gates the hub-catalog clause (§5): a space cannot hold a document
nobody may create. Fixed at
`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/…/✏️editor/🦀️.rs` with the same three hooks.

### 3.3b The audit behind those two fixes: **95 editors declare a presence type and no retirement owner**

Not a guess — counted (`grep -rln "type Presence = " ✏️s/🔌️plugins`, minus test trees, minus the
files that declare `build_presence_local_root_retirement_factory`): **230 of 298** app/editor/viewer
files declare no owner. 134 of those are VIEWERS, which are safe: the viewer wrapper's `ArtifactApp`
impl forces `type Presence = NoPresence` and already answers
`V::build_presence_local_root_retirement_factory().or_else(|| Some(no_presence_local_root_retirement_factory()))`
(`🧰️framework/…/🔌️plugin/🦀️.rs:7836`). **The EDITOR wrapper has no such `or_else`, so all 95 editors
below it are exposed**, and four of them have now been caught by four different slices in two days
(🏠️home, 📖️playbook, this slice's two). Only an app whose commands actually read local presence
fails, which is why the other 91 are quiet rather than correct.

The framework-level fix is real but is NOT this slice's to land blind: `EditorApp<V>` cannot
`or_else` into a `NoPresence`-typed factory while `Self::Presence = V::Presence`, so it needs a
`Presence`-implemented default (a trait on the presence type, defaulting to `None` and overridden for
`NoPresence`) plus an impl on every real presence type — and
`cargo check -p semio-framework-plugin` is **red from a peer's in-flight edit right now** (§6.2), so
no such change could be verified today. **Named with its count and its file:line, not attempted.**

### 3.4 `energy` and `trinity` — the rail row opens, and NOTHING is dispatched (re-diagnosed with the rail dump)

S10 read these two as "the verb lands, nothing is journalled". With the rail row ids now captured
(`🗑️generated/s6-sweep-s11j.txt`) the reading is sharper and less flattering to the probe:

```
energy   rail 34 rows incl. rename-zone, set-surface-property, set-zone-property, create-zone …
         rename-zone            mutated false  edits [0,0,0,0]           (staged form, filled [])
         set-surface-property   mutated true   edits [0,0,0,0]  applied [5,5,5,5]  refusal null
trinity  rail 21 rows incl. textEdit, patchNodes, runQuery …
         textEdit               mutated true   edits [0,0,0,0]  applied [3,4,4,4]  refusal null
```

`mutated: true` here is the probe's `render !== before.render` clause, which an arg-bearing form
opening and closing also satisfies — and `applied` does not move, so **no command was journalled at
all**. Both verbs are arg-bearing and both were submitted with `filled: []`:
`set-surface-property` is declared on the `Artifact` lane
(`🔋️model/…/✏️editor/🦀️.rs:1870`), so a real one WOULD journal; it addresses a surface by id and the
probe supplied none. `trinity`'s `textEdit` is declared on the **`WindowConfig`** lane
(`🔌️jack/…/✏️editor/🦀️.rs:384`) — a window's configuration is not a document edit, so `#s-checkin` is
right never to count it; trinity's document verb is `patchNodes` (`Artifact`), which the probe also
reached and which also carries arguments it could not fill.

So this is **§2.3's third shape, still unresolved for these two**: `LIVE_ID` harvesting finds no
candidate on an `energy` or `trinity` surface (`rename-zone` reports `filled: []`, not
`zone:no-live-id`, because no `DEFAULT_ARGS` row names it as live for this app). It is a probe gap
with a named cause, not a product defect — and it is the honest correction of S10 §7.4's reading.

### 3.4b `lowpoly` and `shooting` — S10 §4.2's `applied` fix regressed every MULTI-LANE row (root found, fixed)

The rail dump settles what three sweeps could not. Both kinds' verbs publish on MORE THAN ONE lane:

| kind | verb | declared lanes | file:line |
|---|---|---|---|
| 💠️lowpoly | `addPrimitive` | `Artifact` + `Config` + `Transient` | `💠️lowpoly/…/✏️editor/🦀️.rs:1288` |
| 🎥️shooting | `addShot` | `Artifact` + `Config` | `🎥️shooting/…/✏️editor/🦀️.rs:570` |

and the framework says in its own docstring what `undo` does with the other lanes
(`🧰️framework/…/🔌️plugin/🦀️.rs:21735`, `interaction_store`):

> the general "undo"/"redo" actions above only ever dispatch against `self.store` (the DOCUMENT
> store), never this one

So after an undo a multi-lane row's **document** edit is gone while its **config** edit is still
applied — and S10 §4.2's predicate `document_applied || config_applied || child_applied` therefore
answered `applied: true` for ever. The host reads exactly that field twice: `dimmed: applied === false`
in the History panel and `entry.kind === "mutation" && entry.applied !== false` in `#s-checkin`. Hence
the measured `edits [0,1,1,1]` with the ledger still growing on the undo AND the redo
(`applied [0,1,3,6]` / `[1,2,4,7]`, `undoClick: ok`, `redoClick: ok`, rail rows 59 and 47 both
carrying `undo` and `redo`) — reproduced at load averages **40, 61 and 82**.

This is a regression of a fix made in THIS ticket, and it is why both kinds passed for S10 before its
own change reached their guests and fail now that it has.

**Fixed** in the same expression (`🧰️framework/…/🔌️plugin/🦀️.rs:24944`):

```rust
let applied = if entry.edit_id.is_some() { document_applied } else { config_applied || child_applied };
```

The parent lane is authoritative whenever the row published into it; the other two answer only for a
row that carries no parent edit at all — which is exactly the `Child`/`Config`-only case S10's fix
exists for (🌊️flow `addWidget`, 🎬️sequence `addStep`, 🌀️procedural `generate` all have
`edit_id: None`), so that cure is kept intact.

**The law could NOT be written in that suite, and this is measured rather than assumed.** Two
attempts, both run:

1. `compositeEdit` (the suite's `Artifact + Child` verb) — the row it logs carries
   `edit_id: None`, so it is not a proxy for a row that published a parent document edit:
   `the composite row publishes a PARENT document edit — …` fails.
2. A purpose-built `MultiLaneEdit` verb emitting `artifact_mutations` AND `config_mutations` in one
   `Emit` — refused before it reaches the store with
   `interactive-job.missing-factory: typed command 'multiLaneEdit' has no exact
   controller/owner/factory/tool/schema proof`. A multi-lane verb needs a retained factory whose
   `PUBLICATION_CONTRACTS` name both lanes (which is exactly what 💠️lowpoly and 🎥️shooting declare),
   and the suite's proof roster is additionally pinned by a fixture count
   (`…:7665`, `restartAuthority.defaultProofs`), so adding one is a fixture change, not a test.

Both attempts were **reverted**; the contract suite is left exactly as it was found plus S10's own
rows. What IS captured is the regression guard that mattered most — S10's
`config_lane_row_reports_itself_applied_so_the_host_can_count_it` still passes against the narrowed
predicate, so the `Child`/`Config`-only cure is provably intact:

```
cargo test -p semio-framework-plugin --lib -- config_lane_row_reports   (private target dir)
test …::config_lane_row_reports_itself_applied_so_the_host_can_count_it ... ok
test result: ok. 1 passed; 0 failed; 823 filtered out
```

(`🗑️generated/s11-law-applied-lane.txt`.) The runtime half — 💠️lowpoly and 🎥️shooting reading
`edits [0,1,0,1]` — needs those two guests rebuilt, which this slice did not take the mutex for.
**Stated as an honest gap, not as a result.**

### 3.5 `flow` — a regression against S10 §2.7, in files this slice may not touch

S10 measured `flow addWidget` **PASS `[0,1,0,1]`** with `lastLedgerRow: …:Add Widget↶` after its
20:11 rebuild. On the 02:37 guest it reads `[0,0,0,0]` with `applied [1,1,1,1]` and no refusal, while
`sequence` — the other `Child`-lane kind S10 fixed in the same change — still PASSes. S10's framework
fix is present in source (`🧰️framework/…/🔌️plugin/🦀️.rs:24944` — the `child_applied` branch, which
§3.4b narrows but does not remove for a row like this one with `edit_id: None`), so the change did
not revert.
`✏️s/🔌️plugins/🌊️flow/**` is a peer session's topic and was read only. **Named, not touched.**

### 3.6a `stdio` offers NO rail verb that can move an edit count — by declaration (measured)

The sweep now records the rail's own row ids (`railRowIds`, added to `🐍️s6-all-kinds-sweep.mjs` by
this slice — the earlier captures said `railRows: 11` and nothing about WHICH 11). For
`s.stdio.csv@rfc4180/*#editor`, the spawned app, the rail is exactly:

```
set-cell, setActiveExample, copy, cut, paste, undo, redo,
commitCheckpoint, createAlternative, switchAlternative, checkoutCheckpoint
```

Nine of those eleven are framework-reserved. The other two cannot move `#s-checkin` either:

- **`set-cell` is the framework TABLE KIT's own row**, minted by
  `TableWindowKit::editable_window_kind()` (`🧰️framework/…/🔌️plugin/🦀️.rs:32396`) as
  `ActionDefinition::bounded_catalog("set-cell", …)` — **with no arguments at all**. The CSV editor
  refuses it (`stdio.csv.unhandled-action: action 'set-cell' is not one of this editor's declared
  verbs (setActiveExample)`), and the refusal is consistent: `csv_command_from_action` bridges only
  `setActiveExample` while `CsvEditorCommand::SetCell` — fully implemented in `handle`, which emits a
  real `CsvMutation::SetField` — is reachable only from op-TEXT parsing. 🔋️energy's editor is the
  counter-example that shows the intended shape: it bridges `SET_CELL_ACTION_ID` in
  `command_from_action` (`🔋️model/…/✏️editor/🦀️.rs:302`) and declares it in both its tool proofs and
  its publication contracts. **Named, not fixed**: an argument-less catalog row cannot carry a cell
  address either way, so bridging it in `stdio` alone would move the refusal rather than cure it, and
  the fix belongs to whoever owns the editable window kits.
- **`setActiveExample` is a document REPLACE with no store edit, on purpose.** Its route emits
  `Effect::LoadDocument` and the contract's own docstring says why: *"no stdio subset mints a
  whole-snapshot edit for it"*, which is also why it publishes on the `HostOnly` lane.

And the declaration is repo-wide, not csv-specific: `grep` for `ActionDefinition::new("…")` /
`.mutation("…")` across **all 35** `🗄️stdio` artifacts returns **nothing** — no stdio artifact
declares an action of its own. So `stdio` is the same class as `sourcing`: the sweep's verdict is
correct and the kind has no rail-reachable document verb to pass with. `paste` is a
framework-reserved CLIPBOARD verb and `#s-checkin` is right never to count it.

### 3.6b The rest, named with their evidence

| kind | measured | reading |
|---|---|---|
| `writer` | the rail's **sixteen** rows, dumped: `clearSelection, selectAll, setSelectionMode, setInteractionGranularity, setActiveExample, copy, cut, paste, formatDocument, lintDocument, undo, redo, commitCheckpoint, createAlternative, switchAlternative, checkoutCheckpoint`. `formatDocument` reads `mutated: false` (the boot document is already formatted), `setActiveExample` is `HostOnly` (`✒️writer/…/✏️editor/🦀️.rs:821`), and every real document operation (`textEdit`, `setText`, `setSnapshot`, `commitRename`) is minted by `writer_hidden_operation`, i.e. `in_palette: false` | the same declaration as `stdio`: **no document verb reaches this rail at all** |
| `lowpoly`, `shooting` | `edits [0,1,1,1]`, `applied [0,1,3,6]` / `[1,2,4,7]`, `undoLane null`, `redoLane null`, **reproduced three times at load averages 40, 61 and 82**. `addPrimitive` is declared `Artifact + Config + Transient` (`💠️lowpoly/…/✏️editor/🦀️.rs:1288`), `addShot` `Artifact + Config` (`🎥️shooting/…/✏️editor/🦀️.rs:570`) | the verb IS counted (0 → 1, which no other FAIL achieves) and the ledger grows on the undo AND the redo, but the count never returns to 0. Three reproductions at three different loads made it worth chasing, and it turned out to be ONE root, **found and fixed in §3.4b** — a regression of S10 §4.2 in every multi-lane row |

## 4. G17 — a locale switch now re-renders the WHOLE ledger (root-fixed, law GREEN, measured)

Coordinator scope addition; S10 §2.10 item 2 listed it unowned.

### 4.1 Two roots, both in the host, neither needing a guest rebuild

**(a) A chrome row's label is frozen at dispatch.** `noteShellCommand(commandId, label: string, …)`
takes an already-resolved string and every call site hands it `shellLabel("ui.shellCommand.…")`; the
guest stores it as `LocalizedLabel::data(label)`
(`🧰️framework/…/🔌️plugin/🦀️.rs:27286`), which projects that ONE string onto every axis. So
`historyEntryLabelText` re-resolving the row against a new locale can only hand the same text back —
exactly what S10 photographed. `HistoryEntry.label`'s own docstring promises the opposite.

**Fixed** — chrome text is the shell's to own, so the shell resolves it at render from the row's
`actionId`: a closed nine-id map (`shell.dockMove`, `panelTab`, `panelToggle`, `windowActivate`,
`windowClose`, `windowMove`, `windowOpenInNewWindow`, `windowResize`, `windowSplit`) to its own
`ui.shellCommand.*` key, plus the live `osCommands` catalogue for an `os.*` row. Plugin rows are
untouched — they carry a real `LocalizedLabel`.

**(b) The shared `shellLabel` port moved one render too late.** With (a) alone the eight chrome rows
turned German and the `os.setLocale` row still read `"Set Locale"` although the DE bundle has
`"Sprache festlegen"` for it. `syncShellLabelLocale(uiLocale)` was called only from the appearance
`useEffect` — post-paint — while `osCommands` and every other `shellLabel`-building memo resolves
during the render in which `uiLocale` changed. Those memos therefore computed against the PREVIOUS
language and, because `uiLocale` never changes again, kept it for good.

**Fixed** — the port is moved during render, guarded, above the builder memos
(`🏛️ShellHost/🟦️.tsx:2094`), with the effect's call kept for the scope instance and
`documentElement.lang`; `shellLabelLocale()` (new, `🛠️ShellHelpers`) is the compare. This is the
discipline `initUiLocaleSync`'s own docstring already demands for boot, applied to the in-app switch.

### 4.2 Before and after, both measured on a live shell

| | rows in the History panel |
|---|---|
| **EN, before the switch** (both runs) | `["Activate Window","Activate Window","Toggle Panel","Switch Panel Tab"]` |
| **DE, S10 2026-09-21** (§2.10, the defect) | `[… ,"Switch Panel Tab","Set Locale","Panel-Tab wechseln"]` — old rows English |
| **DE, after (a) only**, 11:52 | `["Fenster aktivieren","Fenster aktivieren","Panel umschalten","Panel-Tab wechseln","Panel-Tab wechseln","Set Locale","Panel-Tab wechseln"]` |
| **DE, after (a)+(b)**, 12:0x | `["Fenster aktivieren","Fenster aktivieren","Panel umschalten","Panel-Tab wechseln","Panel-Tab wechseln","Sprache festlegen","Panel-Tab wechseln"]` |

`NOTES []` — zero pageerrors in both runs. Captures `🗑️generated/s11-locale-history.txt` and
`s11-locale-history2.txt` (`🐍️s10-locale-history.mjs`, unchanged).

### 4.3 The law — **3 passed**

`🧰️framework/…/🧱️elements/🛠️ShellHelpers/🧪️tests/🌐️chrome-history-locale/🟦️.ts` (new, registered in
the react renderer's vitest `elementSuite` list, which is the include list a co-located suite must be
named in or it never runs): every one of the nine chrome ids answers real text, every one answers a
DIFFERENT text once the shell is German (with two exact expectations,
`"Panel-Tab wechseln"` / `"Fenster aktivieren"`), and a non-chrome id answers `null` so the row keeps
its own label.

```
SEMIO_TEST_LEVEL=long bun ./📜️script.ts test --testNamePattern "chrome history rows"
Test Files  1 passed | 97 skipped (98)
     Tests  3 passed | 1952 skipped (1955)
```

Type-check: `@semio-tech/framework-os typecheck` reads **76 errors before and 76 after** this slice's
TS changes, the two in `🏛️ShellHost` only shifting line number — so nothing here added one
(`🗑️generated/s11-typecheck-os.txt`, `s11-typecheck-os2.txt`).

## 5. A hub-catalog document opened inside `s`

`🐍️s11-space-document.mjs` (new, permanent) drives the product's own journey with no shortcut:
sign in on hub `7641` → enter a studio → open the studio's SPACE INDEX
(`s.space.space@1/*#editor`, the surface that lists a space's artifacts) → read its rows → create one
if the space is empty → open a row and wait for the document's own window. Each phase is reported
separately so one failure cannot be read as another's.

### 5.1 Before the guest rebuild (measured 12:22, `🗑️generated/s11-space-document-before.txt`)

```
beacon      ready:s
sign-in     ok
studio      ["space-3::s-workflow","space-3::s-media-vfs","space-3::s-compiled-dag"]
space index ["space-4::framework.window.table"]
rows before 0
createArtifact → refused: dispatch-failed (user window=space-4::framework.window.table)
                 — presence local read requires a live exact local retirement owner
rows after  0
open        → space index rendered no artifact row to open
```

So **the whole journey up to the space index works** — sign-in, studio, and the index surface all
open inside `s` — and the last two steps are blocked by exactly one thing: §3.3's presence root. The
hub's own `s10-boot` root holds no documents (§0.1: 8.4 MiB, no published catalog), so the acceptance
clause's fallback — "if the catalog lacks documents, create one through the shell" — is the path, and
`createArtifact` is the verb that walks it. **This is the strongest possible statement of why §3.3 is
this slice's load-bearing fix.**

### 5.2 After the guest rebuild — the presence refusal is GONE, and the verb is fired EMPTY

The restage took the mutex at 13:23 and finished at 13:26 (`component rc=0`, `materialize rc=0`,
`activate rc=0 … 60 completed components (changed)`, `🗑️generated/s11-restage.txt`); peers rebuilt
the same two guests again at 14:06 / 14:23, both still newer than this slice's sources (11:02 /
11:13). Serve `6071` was restarted onto that tree at 15:53. Re-measured
(`🗑️generated/s11-space-document-after2.txt`):

```
sign-in ok · studio ["space-3::s-workflow","space-3::s-media-vfs","space-3::s-compiled-dag"]
space index ["space-4::framework.window.table"]
rail (24)  set-cell, createArtifact, deleteArtifact, renameArtifact, touchArtifact,
           requestDeleteArtifact, openArtifact, openArtifactWith, inviteMember, … undo, redo
staged controls []
name=absent  kind=absent  submit=absent
createArtifact → dispatched, NO REFUSAL          ← §3.3's presence root is cured at runtime
rows after (0)
```

**§3.3 is proven**: the sentence that refused `createArtifact`, `renameArtifact` and
`touchArtifact` before the rebuild is gone, and the verb now reaches the guest. What it reaches the
guest WITH is the next root, §5.3.

### 5.3 The next root — a verb whose only form is a DIALOG is fired bare from every other lane (fixed)

`createArtifact` needs two required arguments, and the space index declares them — on a
**`DialogDefinition`**, not on the action
(`🪐️space/…/✏️editor/🦀️.rs:445`, `.dialog(DialogDefinition::new("createArtifact", …).args(vec![…]))`).
The host decides whether to stage a form from `action.args` and from nothing else:

```ts
// 🛠️ShellHelpers/🟦️.tsx:4169
export function actionRequiresStagedForm(action: Pick<ActionDefinition, "args">): boolean {
  return (action.args ?? []).some((arg) => arg.presentation?.kind !== "hidden");
}
// …:4356 — false ⇒ the row fires immediately with no payload
if (!actionRequiresStagedForm(action)) {
  return { …, onClick: () => onExecute({ controllerId, action: action.id }) };
}
```

So the Actions rail (and the command palette, and the MCP lane) dispatch `createArtifact` with an
EMPTY payload, the guest creates nothing, and — because an empty create is not an error — there is
no refusal to see. That is exactly the measured `staged controls []` / `dispatched, no refusal` /
`rows after 0`.

**Fixed** in the app's own declaration: the two arguments are now declared on the ACTION as well as
on the dialog (`.action_args("createArtifact", vec![name, kindChoice])`), the shape 🔱️trinity's
editor already uses for four of its verbs. The dialog is untouched — it is the nicer human surface
and the rail is the other lane; both now carry the same declaration.

**Measured after the 16:43 rebuild** (`🗑️generated/s11-space-document-after{3,4,5}.txt`):

| | `staged controls` | fill | submit |
|---|---|---|---|
| before | `[]` | `name=absent kind=absent` | `absent` |
| **after** | **`["action.createArtifact.arg.name\|div\|treeitem","action.createArtifact.arg.kindChoice\|div\|treeitem"]`** | **`name=ok kind=ok`** | **`ok`** |

Two probe defects were found and fixed on the way, both now in `🐍️s11-space-document.mjs`:
the staged-arg ID sits on the rail's **tree row** (`div[role="treeitem"]`), not on the control inside
it, so a selector demanding `:is(input,textarea)` ON the id reads a fully rendered form as absent;
and the commit control is `…action.<verb>.execute`, never `submit` (the spelling
`🐍️s6-all-kinds-sweep.mjs` already proved).

### 5.4 And the LAST hop, named exactly: the host refuses the creation, silently

With the form filled and submitted the space index still lists nothing — its table renders only its
header (`Create Artifact ID Name Kind Subset Updated Updated By Presence Actions`) and zero rows. The
guest is not the refuser: `createArtifact`'s handler
(`🪐️space/…/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:22`) creates nothing itself — with empty
arguments it emits `Effect::OpenDialog`, and with real ones
`Effect::ReplayShellCommand { action_id: "os.create-space-artifact" }`, because "the host re-resolves
the kind, mints the identity, and owns the durable creation saga".

The host route has four fail-closed gates (`🏛️ShellHost/🟦️.tsx:6172`) and **every one of them
reports through `console.warn` alone**. Widening the probe's console filter to `[os-shell]` — its own
third defect, since none of those sentences contains a refusal word — named the gate on the first
run:

```
[os-shell] replayShellCommand: space artifact creation requires one mounted Space index
```

`origins` is the set of open document sessions belonging to the **shell's own base session** whose
scope is `S_SPACE_INDEX_DOCUMENT_ID`, and the route runs only when there is exactly one. A space
index reached the way this probe reaches it — SPAWNED as a program from the palette — is a different
instance from the shell's base session, so the set is empty and the gesture is dropped. **Two
findings, stated separately:**

1. **A product defect, independent of the journey:** a durable gesture refused by the host is
   invisible to the user. No fault surfaces, no ledger row appears, the rail row looks like it
   worked. Every one of these four gates deserves the same treatment the guest-side refusals get —
   this is why four measurements in a row read "dispatched, no refusal" while nothing happened.
2. **The journey's own last hop is not a spawned program.** The index has to be the one the SHELL
   mounted by navigating into the space, not one opened from the palette. That is the next slice's
   first move, and it needs no further source change from §5.3's chain.

**So the chain is proven to its last link**: presence owners (§3.3) → the verb reaches the guest;
action args (§5.3) → it reaches the guest with a payload; and the host's single-origin gate is what
now stands between that payload and a document. Nothing here is claimed as working that was not
measured.

## 6. Honest gaps

### 6.1 Every number in §2 was taken at load average 40–82, and the sweep is load-sensitive

Measured, not suspected: `demonstrator` and `gis` both scored `no Actions rail row after unfolding`
in one chunk and **PASSed** in the retry with a longer settle; `layout` scored `addPage` not offered
twice and PASSed on the third run. `writer`'s rail read 16 rows in one run and **0** in the next. The
machine carried 20 concurrent cargo processes and a peer fleet throughout (`uptime` captured with
each chunk). So a single FAIL from this sweep is evidence of nothing; only a FAIL reproduced at
different loads is, which is why §3.6b says how many times `lowpoly`/`shooting` were reproduced and
§3.4 does not claim a product defect. **Anyone re-running this should take the union of two passes,
not one run.**

### 6.2 Two source fixes are unproven at runtime, and one framework fix was not attempted

§3.2 (`playbook-module-procedural`) and §3.3 (`space`) are written and are behind the one wasm
rebuild. `cargo check -p semio-s-plugin-playbook-procedural` could not be run when they landed:
`semio-framework-plugin` was **red from a peer's in-flight edit** (`E0277`/`E0027` on `Emit`'s
`tasks` field, gated `#[cfg(test)]` at the time — the peer has since removed the gate, so the tree
should build again, but this slice never captured a green check of its own for these two crates).
§3.3b's framework-level cure for the remaining 91 exposed editors is named with its count and its
file:line and deliberately **not attempted**. §3.4b's `applied` fix compiles and keeps S10's own
law green, but its runtime half needs 💠️lowpoly and 🎥️shooting rebuilt, and its own law could not be
written in the contract suite (two attempts, both run and both reverted — §3.4b says why).

### 6.3 The mutex — and why step 2 of this slice's brief did not need it

The restage the brief queued this slice for had already landed at 04:54 (§1), so no hold was taken
for it. The hold that IS queued (`20260922110100-s11`) is for §3.2's and §3.3's own guests, and at
13:05 it was still second behind C8, whose holder (`48162`) has run a
`trusted-catalog-bootstrap --packages stdio,gis` since 11:32 — **alive, with a live child (66938)**,
so rule 27(b)'s deadlock test does not fire and nothing of C8's was touched. `tc3e`, `play` and `ca1`
queue behind. Nothing of any peer's was killed and the lock was never taken out of turn.

### 6.4 A peer swept `🗑️generated/` at ~16:30 and every capture of this slice is gone

Measured: the folder held 5 files afterwards, none of them this slice's, while
`📓️s11-…md`, `🐍️s11-space-document.mjs`, `🐍️s6-all-kinds-sweep.mjs` and `📜️s11-restage.sh` (all in
the ticket ROOT) survived. Every number quoted in this report was transcribed into it as it was
measured, which is the only reason they survive — the preamble's "write the report early" rule,
earning itself. The capture FILES named throughout are therefore no longer on disk; each is
reproducible by re-running the probe line beside it.

### 6.5 What was NOT touched

`✏️s/🔌️plugins/🌊️flow/**` was read only (peer session's topic) — §3.5's regression is named, not
fixed. No `🌎️hub` Rust was edited; the only hub finding is §0.1's, which is a data-root fact rather
than a code defect. No `git commit`/`stash`/`checkout` was run, `📌️important.md` and `🎫️ticket.json`
were not touched, and no `🗑️generated` file this slice did not create was removed. Hub `7641` was
left running on its inherited pids; only serve `6071` was restarted, by this slice, at 10:59.

## 8. 🎬️sequence — the play session's proposed `StepParams` cold boundary, applied

Coordinator hand-off (2026-09-22 15:57): the play session's
`…/26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/🗑️generated/xcut-dict/proposed-sequence.diff.md`
attributes the five remaining `final Dictionary ownership must be explicitly retired or owned by a
cold boundary` panics in `semio-s-artifact-sequence-sequence` (200 ok / 7 failed) to `StepParams`
declaring `ColdRetire` but **no `Drop` boundary**, unlike the sibling `imperative_engine::Step`. One
of the five backtraces reaches the drop through an `Arc<dyn Any>` inside the framework's own
child-owner machinery, where no call site exists to fix — which is why the boundary has to live on
the type.

Applied by hand after reading the current files (not patched blind):

| file | change |
|---|---|
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs` | new `impl Drop for StepParams` retiring `std::mem::take(&mut self.0)`; `ColdRetire::retire_cold` becomes `drop(self)`; `insert` takes `mut self` and `std::mem::take`s `.0` |
| `…/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | the five `.0` moves out of an owned `StepParams` become `std::mem::take(&mut …)` on `mut` bindings (lines 1532, 1597, 2137, 2142, 2143) |
| the same file | new `replace_scope_cold(&mut Dictionary, Dictionary)` (the helper `imperative_engine` already carries) and **all four** `self.scope = …` rebinds routed through it, including `SequenceRunState::advance`'s, which runs on every executed step of a `run` verb |

`🎬️sequence` is now the third manifest in `📜️s11-restage.sh`, so the served guest carries it.

**Compile: GREEN.** The first transcription did not build — the helper landed between
`#[derive(Default)]` and `struct SequenceRunState` (`E0774`, then `E0599`), and two of the rebinds
borrowed `self.scope` immutably inside a call that already borrowed it mutably (`E0502` ×2). All
three were repaired (helper moved above the derive, `next_scope` computed first):
`cargo check -p semio-s-artifact-sequence-sequence` → `Finished dev … in 1m 04s`, rc=0.

**Test: the diff does exactly what the play session predicted — MEASURED.**

```
cargo test -p semio-s-artifact-sequence-sequence      (private target dir, 20:58)
test result: FAILED. 205 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
grep -c "Dictionary ownership"  →  0
```

| | passed | failed | `final Dictionary ownership` panics |
|---|---|---|---|
| play session, before | 200 | 7 | **5** |
| **this slice, after** | **205** | **2** | **0** |

+5 / −5, and the five ownership panics are gone to ZERO. The two that remain are a different and
unrelated class — `import_media_steps_in_inserts_a_new_step_from_an_object_payload` and
`…wraps_a_bare_scalar_payload`, both failing with
`interactive-job.missing-reserved-builder: media port 'steps:in' is registered but has no concrete
resumable importer`, which this diff never claimed to touch.

**And the guest carries it**: `🎬️sequence` was built and materialized in the 17:50 restage
(`component rc=0`, `materialize rc=0`, `activate … (changed)`), and the sweep re-measured
`sequence addStep` **PASS `edits [0,1,0,1]`**, `lastLedgerRow framework.history.entry.4:Add Step↶`,
0 refusals — so the `Drop` boundary costs the shipped round trip nothing.

## 7. Files changed

**Product source (7 files, 7 root fixes):**

| file | change | § |
|---|---|---|
| `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` | `ModuleApp` declares its three `NoPresence` presence retirement owners AND `build_document_store_owners` — the two roots behind S10's tool factory. **This is the change that makes the kind PASS** | §3.2 |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/…/✏️editor/🦀️.rs` | `SpaceIndexEditor` declares the same three presence owners, PLUS `build_document_store_owners` (the second root behind them) and `.action_args("createArtifact", …)` (§5.3) — three fixes in one editor, each measured before the next was visible | §3.3, §5.3 |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs` + `…/✏️editor/🦀️.rs` | the play session's proposed `StepParams` cold boundary, applied by hand: `impl Drop`, five `std::mem::take` rewrites, `replace_scope_cold` + all four scope rebinds | §8 |
| `🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | new `shellChromeCommandLabel` (the closed nine-id chrome → `ui.shellCommand.*` map) and `shellLabelLocale` (the render-time compare) | §4.1 |
| `🧰️framework/…/🧱️elements/🏛️ShellHost/🟦️.tsx` | the History panel resolves a row's text through the new `historyRowLabelText` instead of trusting the frozen string; the shared `shellLabel` port is moved during render, above the builder memos, instead of only in the post-paint effect | §4.1 |
| `🧰️framework/…/🔌️plugin/🦀️.rs:24944` | `build_history_view`'s `applied` follows the PARENT lane for a row that published a parent document edit, and the other two lanes only for a row that did not — S10 §4.2's `\|\|` left every multi-lane row applied for ever after its undo | §3.4b |

**Laws (1 file + 1 registration):**

| file | laws |
|---|---|
| `🧰️framework/…/🛠️ShellHelpers/🧪️tests/🌐️chrome-history-locale/🟦️.ts` (new) + its `elementSuite` row in `…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | three: every chrome id answers real text; every one answers DIFFERENTLY in German; a non-chrome id answers `null` — **3 passed** |

**Ticket folder (permanent probes and scripts):**
`🐍️s6-all-kinds-sweep.mjs` (shared — the 📕️norm fixture argument, the 🪐️space app + verb + args pin,
and `railRowIds` on every row, which is what made §3.4 and §3.6a diagnosable at all),
`🐍️s11-space-document.mjs` (new — and three probe defects fixed in it: the staged-arg id sits on the
rail's tree ROW not on the control, the commit control is `…execute` not `submit`, and the host's own
`[os-shell]` refusals match no refusal word so they were invisible), `📜️s11-restage.sh` (new).
`🐍️s6-all-kinds-sweep.mjs` additionally gained the `renameArtifact` verb pin for 🪐️space, the
three window-KIT ids excluded from the scan, and a rail unfold that WAITS for rows instead of
sleeping 1 200 ms — the single largest source of false FAILs in this sweep (§6.1).
Captures: `🗑️generated/s11-*` and `🗑️generated/s6-sweep-s11{a,b,c,d,e,f,g,h,i,j}.txt`.
