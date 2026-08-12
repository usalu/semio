# Status

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
