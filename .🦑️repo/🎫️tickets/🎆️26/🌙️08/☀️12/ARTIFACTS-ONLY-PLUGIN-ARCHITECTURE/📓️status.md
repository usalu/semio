# Status

Coordinator: Opus 5 session. Executors: Sonnet 5 agents via `Workflow`. Plan authored by a Fable session at `/Users/ueli/.claude/plans/the-new-architecture-is-prancy-pearl.md`.
**Only the coordinator edits this file.** Agents append to their own report files.

## W0 — Recon: IN PROGRESS

Five read-only Sonnet scouts censusing the four violation classes plus peer state; a synthesizer merges them into `📓️w0-census.md` (which becomes the authoritative dispatch list for every later wave). Nothing outside this ticket folder is written.

| scout | class | report |
|---|---|---|
| A | OS-host registry escape-hatch family + every call site + cross-ownership of `3d.mesh` | `📓️w0-a-escape-hatch.md` |
| B | plugin directory shape; facet dirs; extra dirs + their destinations; taxonomy hardcoding | `📓️w0-b-plugin-shape.md` |
| C | impurity: fs/env/process/net, mutable ambient state, the dead Draft lane, `seed` | `📓️w0-c-purity.md` |
| D | SDK re-export surface, Cargo dep matrix, WIT host gating, `HostEffect`, registration fns | `📓️w0-d-sdk-surface.md` |
| E | peer-ticket state, UCAS landed-state verification, per-plugin clearance | `📓️w0-e-peer-state.md` |

## Cross-session negotiation: SETTLED (2026-08-12)

Three sessions share this tree. Full protocol in `📌️important.md`; the outcomes that shaped the plan:

- **UCAS ceded registration consolidation to APA outright.** Their `declare_artifact!`/`plugin!` macro work is deleted from their plan; `ArtifactDeclaration` + `.artifact()` + `Registrar` + capability gating + sealing policy is APA's, whole. They also ceded the `MeshExporter`/`MeshImporter` deletion and left stdio's registration unconverted so APA can do it in one consistent pass.
- **APA has the first writer slot on repo-root `📜️script.ts`** (order: APA → UCAS-W6 → SMO), in **report mode only**, announced on both channels before and after. SMO deliberately holds its own policy work so as not to shift the gate under its 14 running agents; APA extends the same courtesy.
- **`🔣️taxonomy.json` is APA's ahead of UCAS W6**, but the `pluginChildDirs` flip is deferred to W5 — flipping before per-plugin cleanup reaches stdio would turn UCAS's W2 red on violations only APA can fix.
- **Draft lane design settled with SMO** — shape, verbs, and the inverse law. See `📓️draft-lane-spec.md`, out for SMO review. APA's draft facets will gate SMO's exit criteria, so they must be born conforming.
- **Plugin ordering: UCAS-W4 before APA, per plugin**, clearance signalled by `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave4-reports/<plugin>-report.md`. SMO additionally holds ~21 lanes and pings on release.

### Blocking reality

`🗄️stdio` is transiently red from UCAS's in-flight subset renames, and **every plugin depends on stdio**, so no plugin-side `cargo check` passes for anyone right now. APA therefore starts **framework-side and policy-side**, and holds all plugin work — including `🪐️space`, the one plugin cleared by both peers — until UCAS broadcasts "roster frozen".

`🔌️plugin/🦀️component.rs` is frozen to APA until UCAS signals C1 landed. The `register_mesh_exporter`/`register_app_io` copies living in that file come out then, as part of one coherent escape-hatch removal.

## Wave plan

| wave | content | gate |
|---|---|---|
| W0 | recon census | in progress |
| W1 | M1 `ArtifactDeclaration` + `.artifact()`, M2 `Registrar` seal, M5 capability gating, M4 `genesis()` | UCAS C1 unfreeze for `🔌️plugin/🦀️component.rs`; `🎠️kernel`/`🚪️io`/`🧬️schema` still UCAS-claimed — negotiate per file |
| W2 | five policy regions (report mode), dev plugin lint, dep-cruiser rule, `📝️draft` in `appChildDirs` | APA holds the `📜️script.ts` slot now; announce before/after |
| W3 | per-plugin migration | per plugin: UCAS wave4 report exists AND SMO lane released AND stdio green |
| W4 | escape-hatch family deletion (both `💻️os` copies + SDK trait set) | all call sites converted in W3 |
| W5 | flip policies to gate-blocking, flip `pluginChildDirs`, full verify, adversarial loop-until-dry, close | everything above green |

## Decisions taken

1. **The Draft-lane texture cache is an `Inference`, not draft state.** lowpoly's TLS holds three different things; only two are user gestures. A derived cache in the mutation lane would mean authoring vocabulary for something no user does. Flagged to SMO for objection.
2. **Core verb decomposition is the default over the pre-blessed `paint-stroke` domain verb.** Any use of `paint-stroke` must justify itself in that app's report by showing point-by-point structure is not observable in the draft snapshot.
3. **Policy rules land in report mode first, always.** A rule that gates before the tree is clean is a rule that blocks two other sessions' work for a violation they did not create.
