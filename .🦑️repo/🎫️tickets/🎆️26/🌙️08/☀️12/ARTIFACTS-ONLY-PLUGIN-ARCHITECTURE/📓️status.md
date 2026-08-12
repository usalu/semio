# Status

## FINAL STATE (2026-08-13 ~00:15)

### The mechanism

`ArtifactDeclaration` is landed. **Registration is data the framework walks in a fixed deterministic order**, replacing 33 hand-written callbacks with implicit ordering. `build()` validates ownership — *produce-or-consume*, so export entries writing a foreign dialect are accepted while a plugin registering IO for a kind it does not own is a **hard error**. A strict `s.<plugin>.<artifact>` check activates automatically once kind ids become canonical.

**That check is the ticket's deliverable.** The lowpoly violation is now unrepresentable, not merely removed.

Builder surface: `schema · inferences · composers · formats · subset_validators · languages · document_codec::<A> · document_codec_bare::<S,M> · migrations · composition::<S> · capability`. Composition slots take the *snapshot type*, never a hand-written list — there is no public slice setter, so a divergent list is unwritable.

### `.setup()` — 33 → 11

| | count | |
|---|---:|---|
| plugins with **no** `.setup()` at all | **22** | |
| peer-held, cannot be touched | 8 | writer flow vcs animate sequence architect process reasoning |
| **APA-held residue, each documented** | **3** | procedural puzzle space |

The hook **cannot** be deleted while 8 peer-held plugins call it. It was found to already accumulate into a `Vec<fn()>` — the agent *verified* this rather than authoring it and said so, correcting an earlier report of a silent-overwrite footgun.

**The three residues are honest, not incomplete:** `🌀️procedural` keeps 4 named survivors (2 app-schema now closed elsewhere, a DWG mesh bridge, a linked-flow extension installer) with no declaration field by design; `🧩️puzzle` keeps OS media-host bridges; `🪐️space` keeps a wasm-sandbox app-registry mirror. Each is named, justified, and reported rather than force-converted.

### Escape hatches — 35 sites, classified

- **20 inside artifact trees** (puzzle 18, process 1, procedural 1) — the *compliant interim shape*, which converts when the remaining declaration gaps close.
- **15 in app/pane files** — the real violations: `🎪️demonstrator` 12 (the `2d.map`/`3d.cad` registrations **deliberately preserved** per UCAS's composition ruling — deleting them removes capability), `🪐️space` 2 (app-registry mirror, not domain IO), `🧩️puzzle` 1.

### The design premise that was wrong

APA asserted app-schema was the *last* legitimate `.setup()` use. Conversion found **four** categories. Two were closed (`app_schema()`, `document_codec_bare`). One dissolved on inspection — puzzle's "distinct gap" was category-1 under a stale comment. One was **deliberately left unclosed**: the agent declined to add a declaration field for OS media-host bridges, on the grounds that doing so would *legitimise the exact process-global registry mechanism this ticket documents as a bug*. That refusal is the correct call and the best single judgement any agent made here.

### Verified

Nine crates at 0 errors under `RUSTC_WRAPPER=""` + `--all-targets` with `Finished` + exit 0, plus `📕️norm` green after retry. **All such results are timestamps, not properties** — stdio regressed three times this evening, and every plugin depends on it.

Coordinator: Opus 5 session. Executors: Sonnet 5 agents via `Workflow`. Plan authored by a Fable session at `/Users/ueli/.claude/plans/the-new-architecture-is-prancy-pearl.md`.
**Only the coordinator edits this file.** Agents append to their own report files.

## W0 — Recon: DONE

Five read-only Sonnet scouts + a synthesizer. Authoritative output `📓️w0-census.md` (per-plugin work packets, framework work packets, agent checklist); backing detail in `📓️w0-{a,b,c,d,e}-*.md`. Verdict: W1 clear framework-side pending per-file negotiation; W3 gated per plugin on peer release.

### Findings that changed the plan

1. **The taxonomy gate is a runtime `assert!`, not a lint** (`🔌️plugin/🦀️component.rs:2226-2235`): it reads `pluginChildDirs` dynamically and requires `<child>/🦀️component.rs` on every plugin. Deleting a facet dir from one plugin panics the gate while the list still names it — so **the flip is a precondition for the cleanup, not a consequence**. This reverses the ordering originally agreed with UCAS, who independently verified and withdrew their hazard.
2. **The flip target was wrong in the plan.** `["🎛️apps","🗿️artifacts"]` would have demanded `🗿️artifacts/🦀️component.rs`, which exists in **0 of 33** plugins (`artifactsDirName` governs that dir separately) — it would have panicked the gate repo-wide on first run. Correct target: `["🎛️apps"]`.
3. **`🎪️demonstrator` is a cross-plugin god-pane** registering IO for four kinds it does not own. For `3d.process`/`3d.procedural` both the owner *and* demonstrator register into the same process-global `HashMap`, so **plugin load order silently decides whose DWG-import closure answers** — a live, non-deterministic correctness bug, not merely a layering complaint. For `2d.map`/`3d.cad` demonstrator is the *sole* registrant, so those owners have no IO at all.
4. **`3d.mesh` is architecturally ownerless** — lowpoly declares and solely registers it while remodel, gis and procedural consume it. UCAS resolved it as a **deletion, not a relocation**: stdio's `🧿️semio` `mesh` subset already exists with its own `🚪️io` tree. UCAS also supplied the half APA's census missed — **gis's `🏔️gisterrain` declares `3d.mesh` too**.
5. **`MeshExporter`/`MeshImporter` have exactly 6 implementors, all framework-owned, in one file.** No plugin implements them, so the deletion is clean.

## Landed

| change | verification |
|---|---|
| `🔣️taxonomy.json` `pluginChildDirs` → `["🎛️apps"]` | both tests touching it pass; `bun ./📜️script.ts policy` reports zero plugin-root-shape breaches |
| `🧪️index.test.ts` `pluginChildDirs` literal | pass |
| `🧪️index.test.ts` `artifactComponentDirs` literal (stale expectation from a closed ticket, provenance confirmed at flag 490 with UCAS) | suite 132 pass/22 fail → **134 pass/20 fail** |

Baselines recorded in `📓️baselines.md` so the remaining 20 pre-existing failures are not misattributed to APA.

## W2 — Policy seal: LANDED (report mode)

One new region `//#region 🔧️PolicyRuleArtifactsOnlyPluginArchitecture` in repo-root `📜️script.ts`, after `//#endregion 🔧️PolicyRuleTaxonomy`. **1727 breaches, every one `priority: "medium"`** — nothing gates. `📜️script.ts` write announced as stopped on all four peer channels; the file is released to UCAS-W6.

| rule | breaches | notable |
|---|---|---|
| `PolicyRulePluginClosedShape` | 113 | every legacy facet dir + the genuine extra dirs, each carrying its proposed destination |
| `PolicyRulePluginPurity` | 115 | 36 `RefCell`, 19 `Mutex`, 11 `Atomic*`, 6 `thread_local!`, 35 filesystem, 4 env/process, 2 TS `fetch` |
| `PolicyRuleDeclarativeRegistration` | 1334 | split into 582 real violations vs 721 engine-backlog vs 31 `.setup(` callbacks |
| `PolicyRulePluginDependencyAllowlist` | — | see report |
| `PolicyRuleEffectCapabilityParity` | — | fires broadly by design: only one plugin declares any capability today |

Two mechanism findings worth keeping:
- **The two gate registration sites are not equivalent.** `dissolveBreaches` filters `priority === "high"` before throwing; the earlier `osBreaches` block throws on *any* breach. A report-mode rule wired into the second blocks the gate for all five sessions on first run. The agent correctly deviated from "register at all the same sites" for this reason and said so.
- **Bun tokenizer trap:** a `/** … */` doc comment containing `**/📦️packages` terminates at the embedded `*/` (`error: Unexpected 📦`). Relevant to every session documenting glob paths.

Calibration for anyone reading the gate: **22188 pre-existing high-priority breaches across 27 rules**, 19601 of them handcrafted-grammar/spec-distinctness. None new.

## W1c — `.setup()` eliminated from 19 plugins; and the design premise was WRONG

`ArtifactApp::app_schema()` landed (`🔌️plugin/🦀️component.rs:5118`), so an app's config/presence schema is now answered by the app type itself rather than by a setup callback. **19 of 33 plugins now have no `.setup()` call at all**: `➗️mathematical 🌍️gis 🎥️shooting 🎪️demonstrator 🏗️fem 💠️lowpoly 📋️forms 📏️layout 📐️cad 📖️playbook 📜️imperative 📸️remodel 🔱️trinity 🕸️dag 🖍️draw 🖨️raster 🗄️stdio 🗒️note 🪵️sourcing`.

### The correction: app-schema was NOT the last holdout

The W1c design rested on a premise this ticket asserted and did not verify — that `register_app_schema` was the *only* remaining legitimate use of `.setup()`. **The conversion found at least four distinct categories**, each discovered by an agent hitting it rather than by design review:

1. **App-scope config/presence schema** — the known one. Closed by `app_schema()`.
2. **OS media-host bridges** (`🧩️puzzle`) — no `ArtifactDeclaration` field covers them.
3. **A bare document codec with no `ArtifactApp` to bind to** (`🔋️energy`) — `.document_codec::<A: ArtifactApp>()` requires an app; a library plugin with zero apps registers a `Snapshot`/`Mutation` codec directly.
4. **An app whose document codec has no artifact node** (`🪐️space`) — `SpaceApp` owns no `🗿️artifacts` node in its own plugin, so its codec cannot live in any artifact declaration.

So `.setup()` cannot be deleted by closing category 1 alone. **The honest status is: the mechanism works and is proven across 19 plugins; the remaining 14 split into 8 peer-held and 6 with genuine, now-documented coverage gaps** (`🌀️procedural 📕️norm 🔋️energy 🧩️puzzle 🧱️block 🪐️space`).

This is the ticket's clearest case of *the plan being corrected by contact with the code*. Categories 2–4 were invisible from the design and are exactly the kind of thing a mechanical conversion surfaces and a review does not.

### Also fixed, unprompted, by conversion agents

`🪵️sourcing` reached **exit 0** after its agent found and repaired two genuine pre-existing in-plugin bugs the conversion exposed — a `JsonValue`/`serde_json::Value` stdio gap (fixed with the *already-verified* note/lowpoly text-bridge pattern rather than an invented one) and an `E0252` self-collision in its mutations enum. The pattern propagating correctly between agents, without my involvement, is the strongest evidence that documenting a verified fix beats describing a correct one.

## W1b — declarative registration across the tree: 24 of 33 plugins

Every plugin available to APA now registers its artifacts as **data** (`.artifact(declaration())`) rather than through an imperative `.setup(fn)` callback. Nine remain, all held by peer sessions: `✒️writer`, `🌊️flow`, `🌿️vcs`, `🎞️animate`, `🎬️sequence`, `🏛️architect`, `🏭️process`, `💡️reasoning`, plus `🗄️stdio`.

`💠️lowpoly` — the plugin this ticket was opened against — is converted **and compiler-verified at 0 errors** under `RUSTC_WRAPPER=""` + `--all-targets`.

### Two findings the conversion surfaced that the design had missed

**1. `PluginBuilder::setup` silently drops earlier callbacks.** It stores a single `fn()`, so `.setup(a).setup(b)` keeps only `b` — no compile error, no warning. The `🧩️puzzle` agent wrote four `.setup()` calls in its draft, **caught the bug in its own work before landing it**, and collapsed them into one. Nothing in the tree is broken by it today, but the next author to reach for a second `.setup()` loses three registrations silently. `.artifact()` is already repeatable and accumulating, so the two behave inconsistently for no reason a caller could guess. Handed to the W1c agent: if `.setup()` survives at all, double-set must become impossible rather than merely unlikely.

**2. `ArtifactDeclaration` has no field for OS media-host bridges.** `🧩️puzzle` had to keep a `.setup()` call purely for them — a *second* legitimate escape-hatch category beyond the app-schema one W1c is closing. So closing app-schema may not be sufficient to delete the hook. The real remaining work is the complete classification of surviving `.setup()` calls by what each registers, which is now W1c's first task.

**3. Two real pre-existing bugs fixed as a side effect**, both found by conversion rather than by search: puzzle3d's and puzzle5d's own DSL grammars were dead code — defined and never called — now wired through `.languages(pilot_languages())`. And `📓️iso16757`'s pilot languages point at `en1999`'s grammar paths, a copy-paste bug that was **carried forward unfixed and flagged** rather than quietly corrected, since it is out of this ticket's scope.

### Judgement calls the agents made correctly

- **`📕️norm`'s four root dirs were deliberately NOT split.** `🎚️config`/`👥️presence`/`📄️artifact`/`🖥️app-surface` are genuinely shared across all 15 artifacts and apps — verified by reading each rather than inferred — so per this ticket's own instruction they were filed as a question, not force-split. A wrong split across 15 apps is expensive to undo.
- **`🎪️demonstrator`'s `🎪️panes` restructure was deferred with a written plan** rather than half-done: six thin wiring shims gain little from the move, and the 96-mount `glue.rs` blast radius was judged too risky to rush.
- **`✒️writer`, `🌊️flow`, `🌿️vcs`, `🎬️sequence` refused outright.** All four are *explicitly* listed HELD in SMO's ledger — a positive entry, not an absence — so the "absence means free" clause correctly did not apply.

### Relocation: COMPLETE, and it validated the stop-and-redispatch

Final invariants, measured:

```
declaration() in ⚙️engine ................. 0
declaration() at artifact root ........... 45
pub fn pilot_languages ................... 0   ← nothing widened
real engine::declaration code hits ....... 0   (2 remaining hits are doc comments)
```

**The decision to stop the first pass five minutes in was vindicated by the second.** Two agents (`➗️mathematical`, `🌍️gis`) found `pilot_languages()` *already stranded as `pub`* in `⚙️engine` — the v1 pass had begun widening before it was stopped — and reverted it. Had it run to completion, 45 public functions would have landed, each looking locally necessary in its own diff.

**A subtler defect the agents caught that the design missed entirely.** Several `declaration()` bodies called `io_registry::entries()` **unqualified**, resolving to a sibling module in the engine file. Moved to the artifact root and left bare, that call would have **silently rebound to a differently-typed `io_registry` wrapper that exists at the artifact root** — not a compile error, a *different function*. Agents in g1, g3, g4 and g5 independently spotted it and qualified the path (`crate::artifacts::<x>::standards::v1::engine::io_registry::entries()`).

This is the strongest argument in the ticket for per-site verification over pattern substitution: the transform was uniform in shape and non-uniform in meaning, and the failure mode was silent rebinding rather than a broken build.

**Compiler-verified green at 0 errors** (`RUSTC_WRAPPER=""`, `--all-targets`, `Finished` + exit 0): `💠️lowpoly`, `🖨️raster`, `📸️remodel`, `🔱️trinity`, `🏗️fem`, `📐️cad`, `🪵️sourcing`, `🧩️puzzle`, `🗒️note`.

### The `⚙️engine` relocation, and why it was needed mid-flight

While the conversion batches ran, another session **removed `⚙️engine` from the taxonomy** — gone from `artifactComponentDirs`, `artifactChildDirs`, `subsetComponentDirs`, `subsetChildDirs`; both mandating policy rules deleted; a forbidding rule now censusing all 95 remaining dirs. APA's agents were writing `declaration()` into exactly that directory class.

Rather than kill 27 in-flight agents (a half-converted plugin is far worse than a file in a known wrong place), the batches were allowed to finish and a **relocation pass** moves `declaration()` to the artifact root, where `artifact_kind()` and identity already live. That is also simply the right home: `declaration()` returns *data describing the artifact*, which is not engine behaviour under any definition.

**A near-miss inside that pass, worth keeping.** The first dispatch told agents to qualify the one local call (`pilot_languages()`) and make it `pub`. A peer flagged that this would add **45 newly-public functions** — an API-surface change smuggled in by a mechanical refactor, invisible in review because each individual `pub` looks obviously necessary, and surfacing later as a lint failure nobody can attribute. Verified before acting: 45 definitions, **0 public**, and `declaration()` is the **only** caller. The pass was stopped five minutes in and re-dispatched with *move both functions, widen nothing*.

The verification of that claim nearly misfired too: a first count reported **162 other callers**, which would have killed the fix. The regex was matching inside `register_pilot_languages()` — a different function containing the substring. True count: **zero**; all nine apparent hits are doc comments. Third variant of the same error in one hour, across three sessions. Hence the rule now in `📌️important.md`: **grep to find, enumerate to count.**

## W1 — the mechanism: LANDED

UCAS explicitly released `🔌️plugin/🦀️component.rs` after a handshake; APA took it, landed the mechanism, and released it back. Their composition runtime was untouched throughout.

**`ArtifactDeclaration`** — `🔌️plugin/🦀️component.rs:930-1241`. A consuming typestate builder (`NeedsSchema → DeclarationReady`) with module-private fields. Registration becomes **data the framework walks in a fixed deterministic order**, replacing 33 hand-written callbacks whose ordering was implicit.

**`.composition::<Snapshot>()` is the only slot setter.** No public slice setter exists, so a hand-written slot list that diverges from `ArtifactCompositionFields` is *unwritable* rather than discouraged — UCAS's review correction, and the standard the rest of the design is held to.

**Ownership validation — improved on the design during implementation.** The spec said "every composer entry's `artifact_kind` must equal `decl.kind`". That would have wrongly rejected **export** entries, which legitimately write a foreign dialect. The landed check is **produce-or-consume**: the declared kind must appear on one side or the other. A strict `s.<plugin>.<artifact>` segment check then activates *automatically* once a kind string is canonical — verified by tracing real on-disk dialects rather than assumed, since today's kinds (note's `"s.note"`) are pre-migration. When UCAS's W4 renames kinds, the strict check switches itself on per plugin with no further edit.

**`genesis()` replaced `ArtifactApp::seed(&mut ArtifactStore)`** — the last place an app touched a store directly.

**Exemplar:** `🗒️note` converted end-to-end, `cargo check -p semio-s-plugin-note --all-targets` → **0 errors**.

### What was deliberately NOT done, and why

- **`.setup()` still exists.** 31 live call sites; removing it now breaks every plugin simultaneously. `.artifact()` and `.setup()` coexist until the other 32 plugins migrate. **The honest claim is "the mechanism is landed and proven on one plugin", not "registration is declarative now".**
- **`register_mesh_exporter`/`register_app_io` were not removed from that file** — measured zero definitions there, contrary to an earlier peer note. The family in `💻️os/🦀️component.rs` and `💻️os/🖥️host/` remains APA's to delete once call sites clear.

## A gap in `PolicyRulePluginPurity`, found by a peer

DKM found `📐️cad` holding `static HOST: OnceLock<BrepEngineHost>` (`🗿️artifacts/📐️cad/…/⚙️engine/🦀️component.rs:91-93`) and `🏭️process` holding `host: BrepEngineHost` as a struct field (`…/🧊️process3d/…/⚙️engine/🦀️component.rs:403,415`). **The purity rule does not catch either**, and by its own criterion it is correct not to: it exempts bare `OnceLock` as write-once-by-type, because every artifact's `io_registry` uses `static ENTRIES: OnceLock<Vec<ComposerEntry>>` and flagging those would drown the signal.

**The rule measures the wrong property here.** `OnceLock<Vec<ComposerEntry>>` is a plugin caching its own immutable data. `OnceLock<BrepEngineHost>` is a plugin holding a **handle to host-owned engine state** for the process lifetime. Identical mutability; entirely different violation — not ambient *mutability* but ambient *reach*. The `OnceLock` makes the handle unforgeable after init and does nothing about a plugin having one at all.

**Fix: a distinct check** — a plugin may not hold a host/engine handle in a static, whatever the wrapper — rather than widening the mutability rule, which would only manufacture false positives against the sanctioned tables. Deleting the handle model itself is cross-session (process is SMO-held, cad is APA-held, the trait reaches `💻️os/🖥️host`) and APA owns sequencing it.

## 💠️lowpoly — the named violation: RESOLVED

The ticket was opened against `✏️s/🔌️plugins/💠️lowpoly/🔧️setup/🦀️component.rs`. Verified on disk:

```
$ ls ✏️s/🔌️plugins/💠️lowpoly/
AGENTS.md   🎛️apps   📦️packages   🗿️artifacts   🦀️component.rs        ← the target shape, exactly

$ ls ✏️s/🔌️plugins/💠️lowpoly/🔧️setup/
ls: No such file or directory                                          ← the setup facet is gone

$ grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_\|semio_framework_os::" ✏️s/🔌️plugins/💠️lowpoly/
…/⚙️engine/🦀️component.rs:82:/// `register_mesh_exporter`/… calls are not carried over   ← 1 hit, a docstring
```

All four defects are gone: no OS-host registry calls from a setup facet, no IO registered for `3d.mesh` (a kind lowpoly does not own), no duplication of the IO its own composer tree already declares, and no unguarded host-only calls in a crate that also builds to wasm. The surviving `3d.mesh` mentions are the `ArtifactKindSpec` declaration at `🗿️artifacts/💠️lowpoly/🦀️component.rs:265` — **deliberately left for UCAS**, who own duplicate-kind cleanup — and `kind_id` references inside the composer spec, which is the artifact-native form and correct.

## W3 — plugin migration: EVERY AVAILABLE PLUGIN IS DONE (24 of 33; the other 9 are peer-held)

**Fully closed** to exactly `🦀️component.rs` + `AGENTS.md`/`README.md` + `🎛️apps` + `🗿️artifacts` + `📦️packages` — **15**:
`🪐️space` · `🔋️energy` · `🖨️raster` · `🕸️dag` · `🧩️puzzle` · `🧱️block` · `📋️forms` · `💠️lowpoly` · `🌍️gis` · `➗️mathematical` · `🌀️procedural` · `🏗️fem` · `🎥️shooting` · `📏️layout` · `📸️remodel`

**Done except a sanctioned exception — 9.** Each leftover is either a directory containing its own `Cargo.toml` (inventory-only by hard rule, since relocating a crate is a workspace-topology change) or a root data file:
`📐️cad` · `📖️playbook` · `📜️imperative` · `🪵️sourcing` (`🧩️extensions` crates) · `🔱️trinity` (`🔨️modules` = jack shell/lsp crates) · `🖍️draw` (`🔄️fsm` crate) · `🗒️note` (`🛂️manifest.json`) · `🎪️demonstrator` (`🎪️panes`) · `📕️norm` (`🎚️config`/`👥️presence`/`📄️artifact`/`🖥️app-surface` — deliberately left; see below)

**Blocked, all peer-held — 9:** `🏛️architect`, `🎞️animate`, `🏭️process`, `💡️reasoning` (SMO lanes in flight); `✒️writer`, `🌿️vcs`, `🌊️flow`, `🎬️sequence` (SMO between waves); `🗄️stdio` (UCAS, roster not frozen).

`📕️norm`'s four dirs were left on purpose. `🎚️config` and `👥️presence` are app-schema facets that belong under an app tree — but norm has **15 apps**, and the agent could not establish from the code whether they are shared by all fifteen or belong to one. It filed the question rather than guessing, which is right: a wrong split across 15 apps is expensive to undo and cheap to avoid.

### Techniques that emerged, now standard

1. **`🔱️trinity`: relocate with zero call-site edits.** Keep the crate-root module names in `📦️glue.rs` stable and repoint only the `#[path]` targets. Less work and far less risk than rewriting imports. `🌍️gis`, `📐️cad` and `🏗️fem` all followed it.
2. **`💠️lowpoly`: check the composer tree before relocating a registration.** 7 of its 15 calls were pure duplicates of an existing `LowpolyComposerComposition` entry — the capability was already declared artifact-natively, so they were *deleted*, not moved. Prefer deletion wherever the composer tree already covers it.
3. **`🏗️fem`: a relocation is not finished until the mounts are repointed.** A concurrent session had already git-renamed all 8 compute files into the artifact engines but left `📦️glue.rs` pointing at the old paths — 8 dangling `#[path]` mounts that would have failed the build. The agent found and repaired them. This is exactly the failure class that structural verification catches and `cargo check` (in a red tree) would not have.



**Fully closed** to exactly `🦀️component.rs` + `AGENTS.md`/`README.md` + `🎛️apps` + `🗿️artifacts` + `📦️packages` (9):
`🪐️space` · `🔋️energy` · `🖨️raster` · `🕸️dag` · `🧩️puzzle` · `🧱️block` · `📋️forms` · `💠️lowpoly` · `🌍️gis`

**Done except a sanctioned exception** (8) — the leftover is either a crate-bearing directory (inventory-only by rule) or a root data file:
`📐️cad` (`🧩️extensions`) · `🔱️trinity` (`🔨️modules` = jack shell/lsp crates) · `🖍️draw` (`🔄️fsm` crate) · `🗒️note` (`🛂️manifest.json`) · `🎪️demonstrator` (`🎪️panes`) · `📖️playbook` · `📜️imperative` · `🪵️sourcing` (all `🧩️extensions`)

**In flight** (7): `➗️mathematical` · `🌀️procedural` · `🏗️fem` · `🎥️shooting` · `📏️layout` · `📕️norm` · `📸️remodel`

**Held by peers** (9): `🏛️architect`, `🎞️animate`, `🏭️process`, `💡️reasoning` (SMO lanes in flight); `✒️writer`, `🌿️vcs`, `🌊️flow`, `🎬️sequence` (SMO between waves); `🗄️stdio` (UCAS, roster not frozen).

### Technique that emerged, now the standard for relocations

`🔱️trinity` moved four compute dirs into an artifact engine **with zero call-site edits anywhere in the crate**, by keeping the crate-root module names in `📦️glue.rs` stable and repointing only the `#[path]` targets. That is both less work and far less risk than rewriting imports, and it is now written into every packet. `🌍️gis` and `📐️cad` followed it.

`💠️lowpoly` established the other half: **check the artifact's `🚪️io` composer tree before relocating a registration call.** Seven of its fifteen calls turned out to be pure duplicates of an existing `LowpolyComposerComposition` entry — the capability was already declared artifact-natively, so the calls were *deleted*, not moved. Prefer deletion over relocation wherever the composer tree already covers it.

## Adopted: dead TS export paths (detector only)

A peer session (IIF #2546) found that **517 of 567 relative export paths (91%)** in the plugin `📦️packages/🟦️typescript/📦️index.ts` barrels point at files that do not exist — pre-standards paths against a tree that migrated to `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/`. Worst: `📕️norm` **180/180**, then `🧱️block` and `🧩️puzzle` 36/36, `🔱️trinity`/`🌍️gis`/`🏗️fem` 24/24. Independently reproduced here at exactly 517/567.

**Nothing enforced it — there was no policy on `📦️index.ts` at all.** That absence is the kind of gap this ticket exists to close, so APA took the *detector*; the peer left it unowned and the **fix remains unowned**, stated explicitly rather than quietly absorbed.

Landed as `PluginIndexExportPathLintScript` in the **dev-side** `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (a file APA already owns), **not** repo-root `📜️script.ts` — which sidesteps the single-writer queue entirely rather than contending for a second slot. It is reachable only via its own standalone `index-lint` target, never folded into `plugin lint` or `VerifyScript`, and `run()` cannot throw. So 517 findings are visible without blocking five other sessions on debt none of them created.

Note: the agent hit the **same Bun comment-termination trap** already documented in this ticket — a docstring containing `plugins/*/📦️packages` closed the `/** … */` block early at the embedded `*/`. Documented once, cost recovered twice.

## Measured effect of W3 (policy run, 2026-08-12 18:29)

The five APA rules are the instrument, so their counts are the evidence. Before = the run taken when the rules landed; now = after 24 plugin packets.

| rule kind | before | now | delta |
|---|---:|---:|---:|
| `plugin-closed-shape` | 104 | **41** | **−63** |
| `plugin-registration-violation` | 582 | **525** | **−57** |
| `plugin-dependency-os-host` | 13 | **10** | **−3** |
| `plugin-registration-engine-backlog` | 721 | 722 | +1 |
| `plugin-registration-setup-callback` | 31 | 31 | 0 |
| `plugin-dependency-allowlist` | 105 | 105 | 0 |
| `effect-capability-parity` | 47 | 47 | 0 |
| purity (all sub-kinds) | 115 | 115 | 0 |
| high-priority among APA rules | 0 | **0** | — |

**The zeros are expected, not failures**, and each names its blocking wave:
- **`setup-callback` (31) cannot fall in W3.** Deleting the `🔧️setup/` *facet directory* is W3's job and is nearly done; removing the builder's `.setup(fn)` *hook* requires `ArtifactDeclaration` to exist to replace it — that is W1, blocked on the SDK file.
- **`purity` (115) is inventory-only by design this wave.** Draft-lane facets cannot be authored until per-app verb sets clear SMO review.
- **`effect-capability-parity` (47)** needs the capability gating in W1/M5.
- **`engine-backlog` (722)** is the *compliant interim* shape (registration called from an artifact engine); it converts to declarations in W1, not before.

So W3 moved precisely the two dimensions it targets — plugin shape and escape-hatch call sites — and left the rest for the waves that own them.

> ⚠️ **A near-miss worth recording.** The first attempt to read these numbers reported **0 total**, which would have been a spectacular false success. The breach cache's top-level key is `breachs`, not `breaches`; querying the wrong key returns an empty list, not an error. The result was implausible on its face — 16 plugins still carried facet dirs — which is the only reason it was caught. **A measurement that says you are finished deserves more scrutiny than one that says you are not.**

## Disk exhaustion (resolved) and the cargo blackout

The volume hit **100%, 933Mi free** — root `target/` alone was 428G of stale cache orphaned by the per-ticket `CARGO_TARGET_DIR` policy. Every cargo result repo-wide was untrustworthy during that window, in all six sessions.

Escalated to the user rather than deleted unilaterally (shared state, everyone pays a cold rebuild). **The user chose to delete the per-ticket `🎯️target*` dirs and leave root `target/` alone** — the smaller win, with the trade-off disclosed. Executed after clearing with every reachable session; SMO and DKM both confirmed no build in flight. Volume now **84% used, 141Gi free**.

Worth preserving for whoever revisits this: DKM measured root `target/` at 428G, mtime 17.5h old, with **zero files modified in the preceding two hours** — the signal that distinguishes "stale" from "idle but live". Recorded in their `📓️wave1-mechanism-report.md`.

## In flight

- **W2 — policy + lint seal.** Five report-mode policy regions in repo-root `📜️script.ts` (single writer; both peers notified before the write started, per the agreed announce protocol), plus the dev-side plugin capability lint, the unwired layering lint, and `.dependency-cruiser.cjs`. All at `medium` priority / `warn` severity so **nothing gates** — two other sessions have ~14 agents running against this gate. Adversarial verifier follows.
- **W3 batch 1 — the seven peer-released plugins** (`🪐️space`, `🔋️energy`, `🖨️raster`, `🕸️dag`, `🪵️sourcing`, `🗒️note`, `🧩️puzzle`). Narrow scope: delete dead facet dirs, relocate plugin-root strays into artifact engines, convert same-plugin escape-hatch call sites, **inventory only** the Draft-lane debt. Adversarial verifier follows.

## Blocked, and on what

| work | blocked on |
|---|---|
| W1 `ArtifactDeclaration` / `.artifact()` / curated re-exports / `genesis()` | UCAS signalling C1 landed, unfreezing `🔌️plugin/🦀️component.rs` |
| W1 `Registrar` seal | per-file negotiation with UCAS for `🎠️kernel` / `🚪️io` / `🧬️schema` |
| W1 capability gating | UCAS's `🔌️plugin/🖥️host/` (their `IoRouter` file — *not* APA's `💻️os/🖥️host/`) |
| W3 remaining 26 plugins | per-plugin release from SMO; UCAS W4 report where applicable |
| W3 `🗄️stdio` registration conversion | UCAS broadcasting "roster frozen" (5 new subsets not yet started) |
| W3 demonstrator `2d.map`/`3d.cad` deregistration | **deliberately sequenced after UCAS W4 reaches gis and cad** — writing bespoke IO now would be deleted by their composition work |
| Draft-lane authoring | per-app verb sets reviewed by SMO; `cad` first as smallest exemplar |
| W4 escape-hatch family deletion | all call sites converted |

## Cross-session negotiation: settled

Full protocol in `📌️important.md`. Outcomes that shaped the work:

- **UCAS ceded registration consolidation to APA outright** — their `declare_artifact!`/`plugin!` macro plan is deleted; `ArtifactDeclaration` + `Registrar` + capability gating + sealing policy is APA's, whole. They also ceded the `MeshExporter`/`MeshImporter` deletion and left stdio's registration unconverted so APA can do it in one pass.
- **APA holds the first writer slot on `📜️script.ts`** (APA → UCAS-W6 → SMO), announce before and after.
- **Draft lane settled with SMO** — shape, verbs, inverse law, plus three further obligations (`DiffAlgebra`, full text+binary spec set, `PruneDrafts` never becomes vocabulary). SMO broke a deadlock by ruling draft facets **out of their close criteria**, enforced by green-gating policy instead.
- **SMO published a live per-plugin release predicate** rather than leaving APA to infer freedom from report files.

### A protocol lesson worth keeping

UCAS's per-plugin clearance oracle ("a `wave4-report` exists ⇒ the plugin is free") was unreliable in **both** directions — demonstrator had a report but a live lane, while energy had no report but was explicitly released. The defect is not "infer instead of ask": it is that **report-existence encodes "did wave N finish for plugin P" while every consumer needs "is P free"**, and those coincide only for a ticket that touches each plugin once. The fix adopted by all three sessions is to publish *the predicate the consumer needs*. APA reports therefore carry an explicit `apa-status: complete | partial` line.

## Decisions taken

1. **The Draft-lane texture cache is an `Inference`, not draft state** — a value recomputed from the projection is derived, not authored; modelling it as a mutation would mint vocabulary for something no user gestures at, and would enshrine the caching bug where `render()` mutates it. SMO endorsed.
2. **Core verb decomposition beats the pre-blessed `paint-stroke` domain verb** unless point-by-point structure is genuinely unobservable in the draft snapshot, justified per app. SMO endorsed as better than their own pre-blessing.
3. **Policy rules land in report mode first, always.** A rule that gates before the tree is clean blocks two other sessions for violations they did not create.
4. **`3d.mesh` is a deletion, not a new artifact** — accepted from UCAS; avoided building a duplicate of an existing subset.
5. **No bespoke IO for `2d.map`/`3d.cad`** — accepted from UCAS; their owners get it via composition.
