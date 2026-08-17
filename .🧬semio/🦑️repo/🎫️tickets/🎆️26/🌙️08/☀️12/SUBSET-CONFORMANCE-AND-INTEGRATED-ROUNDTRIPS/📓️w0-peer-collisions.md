# W0 — Peer-Ticket Collision Matrix

Audited: 2026-08-12. Sources: open `🎫️ticket.json` + `📓️status.md` (where present) for all Aug-12 tickets under `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/`.

## Executive summary

Eight concurrent tickets touch the same hot files this ticket needs for W1–W6. None are closed except `DERIVE-ARTIFACT-ANALYZERS-COMPOSERS-AND-BUILDERS` (#2547, closed) and `FIX-MISSING-EPOCH-DEADLINE-IN-WASM-PLUGIN-RUNTIME` (closed). The subset-conformance ticket must treat every peer's **live release predicate** as authoritative — not report existence, not git status, not workspace compile color.

Recommended coordination model for this ticket:

1. **Acquire every hot file in `📓️freeze-ledger.md` before edit**; announce on peer channels when crossing ticket boundaries.
2. **Never edit during another ticket's declared writer slot** on serialized files (`📜️script.ts`, `🔣️taxonomy.json`).
3. **Gate W3+ subset body work per plugin** on SMO + UCAS + APA release predicates for that plugin.
4. **Consume inference APIs only** — do not add spine traits, taxonomy flips, or cache engine code (IIF + DKM own those).

---

## Peer ticket roster (Aug 12)

| Ticket | Status | Goal | Primary hot surfaces |
|---|---|---|---|
| `ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` (#2549) | open | `🎯r2602🎯runningsketchpad` | `🔣️taxonomy.json`, repo-root `📜️script.ts`, per-plugin roots, `🔌️plugin/🦀️component.rs` (post-C1), plugin facet dirs |
| `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (#2548) | open | `🎯aioptimizedrepo` | `🔌️plugin/🦀️component.rs`, `🏪️store`, `📡️spr`, `🧬️schema`, `🚪️io`, `🗄️stdio/**`, per-plugin `📦️glue.rs` |
| `SEMANTIC-MUTATIONS-OVERHAUL` (#2545) | open | `🎯aioptimizedrepo` | all `🧬️mutations/**`, app `🦀️component.rs`, `📦️glue.rs`, repo-root `📜️script.ts` (last in queue) |
| `INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING` (#2546) | open | `🎯aioptimizedrepo` | `💡️inference` OS module, `📡️spr` inference traits, `🔣️taxonomy.json` (P3 flip), `📜️script.ts` (queue pos 4) |
| `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` (#2550) | open | `🎯aioptimizedrepo` | brep/mesh/drawing kernels, `⚙️engine`, `🌊️flow/🌿️vcs`, `🪐️space`, `📜️script.ts` (queue pos 5) |
| `DERIVE-ARTIFACT-ANALYZERS-COMPOSERS-AND-BUILDERS` (#2547) | **closed** | `🎯r2603` | already landed — builders/analyzers/composers derived |
| `DASHBOARD-TUI-WORKFORCE` | open | repoclient/cli | `⌨️cli` daemon/PTY/workflow — largely orthogonal |
| `FIX-STDIO-DWG-AC1018-AND-AC1024-SCHEMA-ID-COLLISION` | open | `🎯aioptimizedrepo` | `🖊️dwg/ac1018` engine + snapshot schema ids |
| `FIX-MISSING-EPOCH-DEADLINE-IN-WASM-PLUGIN-RUNTIME` | **closed** | `🎯r2603` | wasm runtime only |
| `SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS` (this) | open (W0) | `🎯r2602🎯runningsketchpad` | subset bodies, examples, roundtrip harness, policies |

---

## Hot-file ownership matrix

### `🔣️taxonomy.json`

| Owner / sequence | What they change | Release predicate for this ticket |
|---|---|---|
| **APA** — first writer | Already flipped `pluginChildDirs` → `["🎛️apps"]`; adds `📝️draft` to `appChildDirs` | Do not revert `pluginChildDirs`; wait for APA before adding subset archetype keys (`owning`/`derived`, fidelity classes) planned in W1 |
| **UCAS W6** — after APA | Composition-related taxonomy ratchet | Wait for UCAS W6 complete before subset-archetype edits that overlap composition vocabulary |
| **IIF P3** — after SMO | `schemaChildDirs += 💡️inferences` atomic flip | **Hard block:** flipping before fan-out panics runtime gate at `🔌️plugin/🦀️component.rs:2226-2235` |
| **DKM W6** — last | Engine-rep policy allowlists | Do not interleave engine policy with subset conformance policy |
| **This ticket W1** | `subsetChildDirs` extensions, owning/derived archetypes, fidelity enum | Acquire freeze; announce to APA + UCAS + IIF |

**Freeze protocol:** announce on all four peer channels immediately before and after any `schemaChildDirs` or `pluginChildDirs` edit (IIF widened P3 protocol).

### Repo-root `📜️script.ts`

**Writer queue (confirmed five-deep):** APA → UCAS-W6 → SMO → IIF → DKM → **this ticket W2**.

| Peer | Region / rules | Mode today |
|---|---|---|
| APA | `//#region 🔧️PolicyRuleArtifactsOnlyPluginArchitecture` (~1727 breaches, medium) | report-only |
| UCAS W6 | composition/taxonomy ratchet (planned) | report-only first |
| SMO | `🔧️PolicyRuleMutationArtifactEngines` ~5280–6050 + allowlists | held until fan-out settles |
| IIF | inference policy cluster (P3) | not started |
| DKM | engine-rep escape/consumption rules | W6 |
| **This ticket** | subset conformance, example verification, glue generator, roundtrip policies | W2, medium first |

**Critical gate mechanics (APA + IIF verified):**

- `dissolveBreaches` throws only on `priority === "high"` — safe for report-mode rules.
- Earlier `osBreaches` block throws on **any** breach — do not register new rules there.
- Bun doc-comment trap: never embed literal `**/` inside `/** … */` (e.g. glob prose).

**Freeze protocol:** single writer; announce before/after; verify prior writes with `git log --oneline -5 -- 📜️script.ts`, not announcements alone.

### `🔌️plugin/🦀️component.rs` (framework plugin runtime)

| Phase | Holder | This ticket may… |
|---|---|---|
| Now | UCAS C1 in flight | **Wait** for C1 landed + APA unfreeze signal before `subset!` macro |
| After C1 | APA owns Registrar / declarative registration | Add `subset!` beside existing derive; do not touch Registrar seal |
| Shared | UCAS composition runtime regions | Edit only non-overlapping regions; ping before `🏪️store`-adjacent changes |

### `🏪️store/🦀️component.rs`

| Holder | Regions | Collision |
|---|---|---|
| UCAS | `🔖️Composition`, `🔖️CompositionCoordinator`, `group_id` bridge | Different regions from roundtrip harness — ping before entry |
| SMO final ratchet | `SemanticMutation` bounds on `ArtifactStore` | Last; gated on all facets migrated |
| **This ticket W1** | integrated Rust roundtrip stages in existing `test_support` | Acquire freeze; reciprocate UCAS ping protocol |

### `📡️spr/🎮️command/🦀️component.rs` + `🚪️io/🦀️component.rs` + `🧬️schema/🦀️component.rs`

| Holder | Owns | This ticket consumes |
|---|---|---|
| IIF | `Inference`, `InferredField`, `ArtifactInferenceDescriptor`, `StateClass::Inferred` | fidelity + inference **metadata** hooks only — no second cache |
| UCAS | `ArtifactRef`, composition schema emission, `MutationMeta.group_id` | IO fidelity declarations, dialect registry enumeration |
| DKM | deletes imperative kernel bridges | do not preserve setter-shaped IO registration |

`🚪️io/🦀️component.rs` is **dual-mounted** (`semio-framework` + `semio-framework-os-kernel` via `#[path]`) — one edit, two crates.

### `📡️spr` command surface (SPR)

UCAS owns `MutationMeta.group_id` threading; SMO owns `semantic_kind`/`label` final population. This ticket's roundtrip law uses existing command/envelope paths — **do not add new SPR op variants** without SMO verb review.

### Per-plugin `📦️glue.rs`

| Peer | Rule |
|---|---|
| SMO | Active on ~15+ plugins; Wave R glue repair in flight | Do not edit plugin glue until plugin released |
| UCAS | stdio glue after roster freeze | Regenerate only after subset migration batch for that plugin |
| APA | Converts registration in released plugins | Serialize glue regen after APA W3 batch for that plugin |
| **This ticket** | `generate plugin-glue` dry run in W1; serialized regen per plugin in W4 | One writer per plugin glue at a time |

### Animate / process mutation trees

| Tree | SMO state | APA state | This ticket |
|---|---|---|---|
| `🎞️animate/🗿️artifacts/🎬️present/**/🧬️mutations/**` | Wave R2b **running** (6 leaves) | not in released batch | **Hold** — extend existing test regions only after SMO lane completes |
| `🏭️process/🗿️artifacts/🧊️process3d/**/🧬️mutations/**` | Wave R2b **running** (2 leaves) | held | **Hold** |
| Both plugins | SMO held list includes animate + process | APA held | No subset-owned migration until both release |

SMO milestone (2026-08-12): `cargo check --workspace` → 0 errors; all **54 non-stdio** facets on `#[derive(Mutations)]`. Stdio 53 facets deferred behind UCAS roster freeze.

### DWG standards (`🖊️dwg`)

| Ticket | Scope | Collision |
|---|---|---|
| `FIX-STDIO-DWG-AC1018-AND-AC1024-SCHEMA-ID-COLLISION` | Assign `ac1018` → `"stdio.dwg.ac1018"`, keep `ac1024` on `"stdio.dwg"` | **Do not** touch `register_document_codec` ids until that ticket lands or explicitly releases |
| DKM + IIF | `✳️brep`/`✳️drawing` inference reassigned to DKM; not IIF P2 | Subset roundtrips on DWG wait for schema-id fix + DKM drawing dissolution |
| UCAS | stdio roster owner | DWG subsets remain `✳️any` per standard (no named profile subsets) |

Current DWG shape: two standards (`ac1018`, `ac1024`), each with `✳️any` only — no derived profile subsets like PDF/SVG.

---

## Per-peer release predicates (actionable)

### APA (#2549)

**Released now (SMO predicate):** `🪐️space`, `🔋️energy`, `🖨️raster`, `🕸️dag`, `🪵️sourcing`, `🗒️note`, `🧩️puzzle` (partial — verify cargo before editing).

**Blocked on UCAS C1:** framework `ArtifactDeclaration`, Registrar, capability gating.

**Blocked on SMO:** remaining 26 plugins until `apa-status: complete` in APA W3 reports (not merely wave4-report existence — APA corrected unreliable oracle).

**stdio registration conversion:** blocked until UCAS broadcasts **"roster frozen"** (not merely compiles).

**Landed already:** `pluginChildDirs` → `["🎛️apps"]`; APA W2 policy regions in report mode.

### UCAS (#2548)

**Claims:** `🗄️stdio/**`, framework composition spine, kernel store/spr/vcs regions.

**Release signals:**

- **"Roster frozen"** — required before SMO stdio mutation lane and APA stdio registration pass.
- **`📓️wave4-reports/<plugin>-report.md`** — APA uses as clearance oracle (UCAS before APA per plugin).
- **C1 landed** — unfreezes `🔌️plugin/🦀️component.rs` for APA mechanism work.

**W2-prep done:** `✳️object`→`✳️value`, `✳️workflow`→`✳️flow` (148 files). stdio baseline: **2021 passed / 5 failed / 3 skipped** (5 failures owned by IIF).

**Held by SMO for mutations:** all non-stdio plugins in SMO lanes; stdio held by UCAS ordering agreement.

### SMO (#2545)

**Released to APA:** `space`, `energy`.

**Held (in flight):** architect, shooting, demonstrator, lowpoly, animate, process, reasoning, layout, gis, mathematical, note, block, puzzle, norm, trinity, dag, raster, sourcing, remodel, imperative, playbook.

**stdio:** lane written, **deliberately unlaunched** until UCAS roster frozen. Facet count will grow 107 → 112 after 5 new semio subsets.

**`📜️script.ts`:** SMO takes **last** in queue; Wave R3 held until fan-out settles.

### IIF (#2546)

**Blocked:** stdio P2 on UCAS roster; trinity on APA relocation; puzzle verifying.

**Owns spine:** all inference traits, cache engine, taxonomy flip (P3), five stdio test failures.

**Reassigned to DKM:** `🧿️semio ✳️brep` / `✳️drawing` / `✳️mesh` inference facets.

**This ticket:** consume `ArtifactInferrer`, `InferredField`, `infer_field` — never duplicate cache model.

### DKM (#2550)

**W1 mechanism dispatched;** W2 platform exemplar dispatched.

**Handshake:** SMO handed `🌊️flow/🌿️vcs` bridge; owns `🪐️space` module CRUD elimination separately from flow.

**stdio handoff pending:** UCAS must confirm write access to `✳️brep`/`✳️drawing`/`✳️mesh` subset dirs.

**`📜️script.ts`:** queue position 5.

### DASHBOARD-TUI-WORKFORCE

W0–W5 largely done; verify partial. Touches CLI/daemon/workflow — **no direct collision** with subset bodies except shared `launch.json` registration order (coordinate before adding subset conformance launch entries).

### FIX-STDIO-DWG (open, no status.md)

Collision on `ac1018`/`ac1024` schema id strings and coupled snapshot/engine literals. Treat as **exclusive** until closed or released.

---

## Freeze-protocol recommendations for this ticket

| Hot file | Acquire from | Release when | Announce |
|---|---|---|---|
| `🔣️taxonomy.json` | coordinator | W1 taxonomy task completes | APA + UCAS + IIF channels |
| `📜️script.ts` | coordinator | W2 policy task completes | wait for DKM slot after IIF; announce before/after |
| `🔌️plugin/🦀️component.rs` | coordinator | W1 macro merged | UCAS (C1) + APA |
| `🏪️store/🦀️component.rs` | coordinator | W1 harness merged | UCAS |
| `📡️spr/🎮️command`, `🚪️io`, `🧬️schema` | per W1 subtask | subtask done | IIF for inference metadata touches |
| `✏️s/🔌️plugins/🗄️stdio/**` | per subset worker | subset reference passes | UCAS roster frozen + IIF stdio carve-outs |
| `✏️s/🔌️plugins/<P>/📦️packages/🦀️rust/📦️glue.rs` | per plugin | glue regen done | SMO + APA + UCAS per plugin |
| `🎞️animate/**`, `🏭️process/**` mutation trees | — | **do not acquire until SMO releases** | SMO channel |
| `🖊️dwg/**` | — | **do not acquire until FIX-STDIO-DWG releases** | that ticket |

**Observations with shelf life:** any file with a live peer agent may change within minutes. Timestamp observations; re-read immediately before edit. Never use `git checkout` to undo (SMO block lane incident).

**Auto-commit note:** repo periodically auto-commits (`🐙️ueli…🚩️<n>`). `git status` clean ≠ no recent edits. Use `git log --oneline -5 -- <path>` + mtime.

---

## Collision severity by wave (this ticket)

| Wave | Highest collision risk | Mitigation |
|---|---|---|
| W0 | none (read-only) | this document |
| W1 | store, plugin macro, taxonomy | strict freeze ledger; wait UCAS C1 for plugin file |
| W2 | `📜️script.ts` queue | slot 6; medium severity only |
| W3 | stdio derived profiles + semio refs | UCAS roster frozen; IIF 5 stdio failures fixed or baselined |
| W4 | all plugin glues + SMO-held plugins | per-plugin release predicate |
| W5 | deletions | all 138 subsets green first |
| W6 | policy promotion to high | all peers report-mode rules stable |

---

## G0 gate checklist (peer inputs)

Before W1 mechanisms:

- [ ] UCAS "roster frozen" broadcast received
- [ ] IIF stdio failure baseline acknowledged (5 tests)
- [ ] FIX-STDIO-DWG disposition known (block vs release)
- [ ] Freeze ledger entries acquired with peer ack for framework files
- [ ] SMO release map refreshed for W3 reference plugins (cad, norm en1990, csv, tiff, docx, semio mesh, xml valid)
