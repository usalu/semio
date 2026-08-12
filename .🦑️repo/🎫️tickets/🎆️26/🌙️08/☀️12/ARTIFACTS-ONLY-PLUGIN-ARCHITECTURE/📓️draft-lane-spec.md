# Draft lane facet spec

Status: **rulings settled with SEMANTIC-MUTATIONS-OVERHAUL (SMO) 2026-08-12; awaiting SMO review of this document before any app is touched.**
Owner: APA (`26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`, #2549). Reviewer: SMO (#2545).

## Why this exists

`ArtifactApp` declares four lanes — document, config, draft, presence. The draft lane is the sanctioned home for **ephemeral local-only** state: work-in-progress that must not enter a checkpoint. Today it is dead. All 54 apps write

```rust
type Draft = NoDraft;
type DraftMutation = NoDraftMutation;
```

and keep their real session state in `thread_local!` instead — lowpoly's paint-stroke buffers, gumball transform session and texture cache; puzzle 3d and 5d holding the *entire app* in TLS; draw's session; cad's preview counter. That state is app-owned ambient memory that no store governs, no history records, no undo reaches and no collaborator sees. lowpoly even mutates it from `render()`, which is declared pure.

This is the sharpest violation of the ticket's thesis. `ArtifactApp::handle` is documented as "a total, side-effect-free function… No `&mut self`" — and the TLS scratch is precisely the `&mut self` the signature refuses, smuggled through thread-local storage. APA makes the lane real so that *every* change an app makes is a mutation dispatched into a store, with no exceptions and no side door.

## Lane semantics

| | document | config | **draft** | presence |
|---|---|---|---|---|
| persisted | shared | local | **no** | shared |
| enters a Checkpoint | yes | no (own undo stack) | **never** | no |
| survives instance restart | yes | yes | **no** | no |
| visible to collaborators | yes | no | **no** | yes |
| undo/redo | via VCS | own stack | **own stack, dropped at commit** | no |

A draft mutation is dispatched through `DraftStore` by `VcsArtifactApp::dispatch_emit` exactly like a document mutation, and is pruned by `ArtifactCommand::PruneDrafts`. The lane differs in **persistence and history**, not in **vocabulary or authorship**.

## Ruling 1 — directory shape (SMO, confirmed)

Symmetric with the two existing app-schema siblings:

```
🎛️apps/<app>/📝️draft/🧬️schema/
├── 📸️snapshot/          (+ 🦀️ 🟦️ 🔣️ 🔗️ 🛰️ leaves, per schemaFormats)
├── 🔺️diff/
└── 🧬️mutations/<emoji><slug>/
    ├── 🦠️mutation/      real `impl MutationKind<DraftSnapshot, DraftMutation>`
    ├── 🔺️diff/          real `pub fn diff(payload, base) -> DraftDiff`, sparse, built directly
    └── ↩️inverse/        real `pub fn inverse(payload, base) -> Vec<DraftMutation>`
```

This mirrors `POLICY_APP_CONFIG_DIR = "🎚️config"` (`📜️script.ts:6888`) and `POLICY_APP_PRESENCE_DIR = "👥️presence"` (:6892); the rule at :7043 already requires every app-schema owner to expose `🎚️config/🧬️schema` and `👥️presence/🧬️schema`, so `📝️draft/🧬️schema` is the third sibling the taxonomy already implies. `📝️draft` must be added to `appChildDirs` in `🔣️taxonomy.json` (APA holds that file ahead of UCAS W6; this addition is purely additive and is **not** the deferred `pluginChildDirs` flip).

Emoji uniqueness is scoped *within* one `🧬️mutations` tree, not across app-child dirs, so `📝️` at this level collides with nothing.

## Ruling 2 — every SMO policy rule applies, automatically (SMO, deliberate)

`policyFindAllMutationsDirs` (`📜️script.ts:5500`) walks all of `✏️s` for any directory named `🧬️mutations` and does **not** exclude `🎛️apps`. Today zero of the 107 facets sit under `🎛️apps`. **The moment a draft facet lands, every SMO rule picks it up** — banned-vocabulary scan, dispatch coverage, emoji uniqueness, TS-mirror presence. SMO has chosen not to exclude them: drafts are held to the same bar.

Consequences APA accepts:
- SMO's facet count goes 112 → ~127; SMO absorbs that.
- **APA's draft facets gate SMO's ticket exit criteria.** They must pass the four mechanical gates from the outset — there is no "clean it up later".
- If a draft facet cannot be authored conformingly, **leave its dispatch enum empty with no triad dirs and report it**. Never invent vocabulary to fill a gap. (Same standing arrangement SMO has with UCAS.)

### The four mechanical gates (binding)

1. **Triad dirs ↔ dispatch-enum variants, 1:1 in both directions.** No orphan dir, no variant without a dir.
2. **Unique emoji per directory within the facet.**
3. **Real leaves, not shims.** `🦠️mutation` contains an actual `impl MutationKind<`; `🔺️diff` builds the sparse diff directly from `(payload, base)` — never apply-then-capture, never a snapshot clone; `↩️inverse` reconstructs from `base`.
4. **Non-stub `🟦️component.ts` beside every triad `🦀️component.rs`**, and real glue `#[path]` mounts — never inline `#[path = "."]` self-wiring in the dispatch file.

Plus: docstrings start with a unique fitting emoji; no comments inside definitions; and **never write the three banned mutation identifiers anywhere under `✏️s/`, including in prose or docstrings** — the policy greps raw content.

## Ruling 3 — verbs (SMO, with two corrections to APA's instinct)

Only the closed `APPROVED_VERBS` table. Two corrections APA had wrong:

**Stroke lifecycle.** `create-stroke` to begin; **`insert-stroke-point{index}`** to extend, under the ordered-index law (inserted indices are FINAL-state). Do **not** mint `extend` — it is a synonym of `insert`/`add` and the closed table forbids synonyms. Commit-to-document is an ordinary *document* mutation; the draft side then clears with `delete-stroke`.

SMO notes that `📓️taxonomy.md`'s "Domain verbs" section explicitly pre-blesses **`paint-stroke` (lowpoly)** as its worked example of a legitimate domain verb. It is available *if* a stroke is genuinely one indivisible gesture rather than a point-by-point accumulation — with its own emoji + kebab slug, a real inverse partner verb in the same dispatch enum, and handcrafted diff + inverse. **APA's default is the core decomposition** (`create-stroke` + `insert-stroke-point`); `paint-stroke` is used only where the point-by-point structure genuinely is not observable in the draft snapshot. Decide per app, in the app's report, with the reason stated.

**Gumball / gizmo session.** `bind`/`unbind` for attach/detach is correct (derivation rule 4: a gizmo session attaching to a target is a parameterization, not an edge). For the drag itself **do not reach for `update`** — the table already has `move` (absolute reposition), `drag` (relative offset, inverse = negated offset), `rotate`, `scale`. A gumball *is* move/rotate/scale; apply them to the pending transform. `update-<facet>` is reserved for an inseparable ≥2-field facet never meaningfully set one field at a time. Cancel → `unbind-*`.

> The `update` trap has now caught two sessions in one afternoon (UCAS reached for `update-link-pin`, measured the type, found one field moving, and switched to `change-link-pin`). **Measure before choosing**: count the fields that actually move. An enum-with-payload is still *one* field taking one cohesive value.

## Ruling 4 — inverses required, no lane exemption (SMO)

`MutationKind::inverse` already has a zero-cost escape hatch: **return `Vec::new()`**. That is the sanctioned answer for "nothing to undo" and it is what replaced the banned sentinel type throughout the overhaul. So a draft mutation with nothing to restore is one line, not a special case.

Exempting the lane was rejected because it buys almost nothing and costs a lot: it forks the mechanism (`MutationKind` and the derive would need a lane-conditional shape, defeating the single authorship unit); it drops draft diffs out of the law harness (`assert_mutation_inverse_law`, `assert_mutation_diff_absorb_law`, `assert_diff_algebra_between_law`), which is where correctness actually comes from — an ephemeral lane that silently mis-absorbs is still a bug; and "undo my last paint stroke before I commit" is a real gesture users expect, which an exemption would make inexpressible rather than merely unimplemented.

**Rule:** author a real inverse where the mutation is invertible from `base`; return `Vec::new()` where it genuinely is not.

## Per-app inventory

To be populated from `📓️w0-c-purity.md` (W0 scout C), which censuses every `thread_local!` / interior-mutability site in the plugin tree, classifies sanctioned write-once tables against real mutable state, and sizes each migration. Each row becomes one W3 work packet:

| app | state held today | where | proposed `Draft` snapshot fields | proposed mutations (verb-slug) | size |
|---|---|---|---|---|---|
| _pending W0-c_ | | | | | |

Known from initial exploration, to be confirmed and sized by W0-c:
- **lowpoly** `🎛️apps/💠️lowpoly/🦀️component.rs:48-52` — paint-stroke buffers, gumball transform session, texture cache; `render()` mutates the TLS at :344-350. The texture cache is a *derived* value, not state: it belongs in an `Inference`, not the draft lane. Split the three concerns rather than lifting the struct wholesale.
- **puzzle 🧊️3d** `:1876` and **🖐️5d** `:1295` — the entire app in TLS. These need the most design; likely the largest packets.
- **draw** `:164` — `thread_local!` declared *inside* `handle()`.
- **cad** `:947` — a `u64` preview tick counter. Smallest; a good first exemplar.

## Sequencing

1. SMO reviews this document. ← **we are here**
2. W0-c lands; the per-app inventory table is populated and each app's verb set is drafted and sent to SMO for verb review *before* authoring.
3. `📝️draft` added to `appChildDirs` in `🔣️taxonomy.json` (additive, safe, independent of the deferred `pluginChildDirs` flip).
4. `cad` first as the smallest exemplar, reviewed end-to-end against the four gates, then the rest fan out.
5. No app is touched before its plugin is released by **both** SMO (lane complete) and UCAS (`📓️wave4-reports/<plugin>-report.md` exists). lowpoly, puzzle, draw and cad are all currently held by SMO.
