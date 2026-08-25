# Phase 9/10 Current Closeout Scout — 2026-08-25

## Verdict

**RED; neither Phase 9, Phase 10, nor the master ticket is eligible to close.** This was a
read-only scout: no source/configuration/lockfile change, no Cargo/Nx/Wasm/browser command, and no
dependency verifier execution occurred.

The repository has three distinct dependency snapshots. They must not be substituted for one
another:

| Snapshot | Rust | JavaScript | All ecosystems | Meaning |
| --- | ---: | ---: | ---: | --- |
| Accepted isolated checkpoint | 63 | 66 | 129 (two-ecosystem boundary) | Last accepted direct Rust/JS evidence. |
| Last executed shared-tree gate (2026-08-24) | 84 | 70 | 229 current against 216 baseline | Last exact live verifier result; it was RED with 13 new identities. |
| Current committed `🔒️dependencies.json` inventory | 85 | 70 | 232 | Static count of committed baseline entries, not a fresh execution result. It contains 60 Go and 17 Python rows as well. |

The final row was obtained by counting its `ecosystem` fields. The 85/70 committed baseline differs
from the last live 84/70 scan, so it cannot prove the live gate clean. A serialized owner must rerun
the permanent verifier after shared manifest work becomes quiescent.

## Accepted Boundary and Current Divergence

The accepted isolated boundary remains **63 Rust + 66 JavaScript = 129** direct identities, per
`📓️coordinator-dependency-boundary-2026-08-22.md`. It was always a shrink-only ratchet, not either
phase's literal-zero exit gate.

The last live shared-tree gate is documented by
`📓️coordinator-live-dependency-gate-divergence-2026-08-24.md`:

- 84 Rust + 70 JavaScript = 154 direct identities;
- 216 baseline versus 229 current across all ecosystems; and
- 13 rejected Rust identities from the stdio oracle packet: `calamine`, `comrak`, `dxf`,
  `html5ever`, `json`, `las`, `markup5ever_rcdom`, `mp4`, `ply-rs`, `quick-xml`, `riff`,
  `rust_xlsxwriter`, and `ruststep`.

The present stdio oracle manifest also declares `id3`, so no agent may infer that the previous
13-row finding fully describes the current source without rerunning the verifier. Its optional
oracle set is an external test boundary, not an automatic zero-dependency exemption. In particular,
the current manifest shape still makes `markup5ever_rcdom` and `rust_xlsxwriter` production-runtime
rows unless the permanent policy intentionally reclassifies them through the registered oracle
mechanism.

## Attribution-Safe Shared-Tree Census

Current working-tree deltas fall into these mutually exclusive buckets:

| Bucket | Evidence | Disposition |
| --- | --- | --- |
| Active interactivity source packets | Actor replay, renderer host/canvas/glue, surface Graph/Map, Flow host/VCS, plugin shard, Editor, and `📜️script.ts` P2c/P3mn static gates. | In scope for phases 2/3/5/8; not P9/P10 dependency removal. Preserve until their owners finish. |
| Internal-only manifest additions | `surface` and OS `flow` each add `semio-framework-job = { workspace = true }`. | First-party path/workspace dependency; it cannot increase the external freeze identity count. |
| Concurrent stdio/oracle work | 29 modified stdio artifact/test files plus `Cargo.lock`; live source has the large optional external oracle set. | Peer work. Do not remove, reset, restage, unstage, or baseline it from this refactor. It is nevertheless a real final dependency-gate input. |
| Unrelated end-to-end work | New `END-TO-END-TESTING-REFACTOR/w13-carrier-probe/` ticket content. | Preserve; no attribution to this plan. |

No current unstaged or staged `package.json`, `📋️project.json`, `bun.lock`, or `.vscode/launch.json`
delta was found. The only changed manifests in the current worktree are the two internal Rust
workspace additions above; `Cargo.lock` is modified without a package-name delta in its visible
diff. This does not replace the required final dependency scan.

## Forbidden and Remaining Third-Party Boundaries

The attached plan still requires removal of every third-party runtime, build, test, documentation,
code-generation, release, and browser-driving dependency under its declared scope. `compose/` stays
explicitly out of scope.

Repository instructions supersede the plan only for the required **Bun package-manager and Nx task
runner boundary**:

- retain Bun and Nx as visible mandated toolchain identities;
- do not hide or baseline-waive them as zero dependencies;
- report literal third-party totals and a separately labelled permitted-removals total; and
- never remove/bypass Bun or Nx while `AGENTS.md` requires them.

All other third-party identities remain actionable. Directly evidenced examples include
`@xyflow/react`, `dagre`, `i18next`, React, Storybook, Vitest, Playwright, Binaryen,
dependency-cruiser, esbuild, external Rust serialization/image/browser bindings, and the stdio
oracle references. The Phase 10 Diagram audit accepts the owned directed-layout behavior but leaves
`dagre`/`graphlib` in the UI React manifest/lock; removal needs a separate post-browser-gate packet.
It must not be bundled with current renderer/port work.

## Orchestration, Verifier, and Launch Gaps

`📜️script.ts` already owns these permanent verifier surfaces:

```text
bun ./📜️script.ts verify interactivity
bun ./📜️script.ts verify interactivity tool-jobs
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list rust
bun ./📜️script.ts verify dependencies list js
bun ./📜️script.ts verify dependencies parity js
```

Root `📋️project.json` uses the required `bun ./📜️script.ts …` shape, and root `package.json`
retains the required Nx path. These are correct boundaries, but `workspace` does not yet expose
dedicated Nx targets for the three interactivity/dependency gates, and `.vscode/launch.json` has no
registration for `verify interactivity`, `verify interactivity tool-jobs`, or `verify dependencies`.
Because developers operate from `launch.json`, that omission blocks normal executable access to the
final static gates.

The prior general `verify` command is not a substitute: it runs the broad gate and test path, while
the final matrix requires the named, separately reported static checks. The existing launch ordering
places gate entries in group `4_gate` around orders 410–411.42; add the permanent checks there in a
single packet after source work is quiescent.

## Bounded Closeout Packets

1. **P9/P10 reconciliation scout-and-gate owner (serialized, no source edits).** Wait for stdio
   oracle and all manifest owners to quiesce; run the five permanent dependency commands above plus
   `git diff --check`; publish literal/direct/all-ecosystem counts, new/removed names, kinds, and
   source owners. Do not run `write-baseline`. This packet decides whether the old 84/70 versus the
   committed 85/70 discrepancy is real and whether the gate remains RED.
2. **P10 launch/Nx exposure.** Add `workspace` Nx run-command targets that delegate only to
   `📜️script.ts verify …`, then register the three named gates in `.vscode/launch.json` in the
   `4_gate` order. Verify project/package delegation shapes statically, then execute through Nx only
   when the serialized matrix owner authorizes it. No Bun/Nx removal.
3. **P9 stdio-oracle policy packet (separate owner).** Establish whether every optional oracle is
   registered and classified test-only by the permanent collector, or delete/replace it under the
   literal zero-dependency program. This needs an explicit final policy decision; it must not be
   silently absorbed by the ratchet.
4. **P10 Dagre retirement packet (after browser/worker correctness is GREEN).** Prove no live
   Dagre/graphlib consumer, remove UI-manifest and Bun-lock rows through Bun reconciliation, then
   run focused Diagram, UI, renderer-browser-worker, parity, frozen lock, and dependency scans.
   Keep XYFlow and its shared transitive D3 edges out of the packet unless independently proven
   unreferenced.
5. **Remaining literal-zero replacement waves.** Split by owned interface and runtime class
   (Rust platform/render/storage/plugin boundaries; JS UI/platform/tooling boundaries; Go/Python
   production tool boundaries). Every packet requires an owned contract, language-agnostic behavior
   test, third-party differential result where applicable, source/manifest/lock absence evidence,
   and a fresh freeze count. Do not report Phase 9/10 complete merely because the ratchet has no new
   rows.

## Closure Blockers

- The live dependency gate was last observed RED, and no fresh final scan was run here.
- The committed inventory and last live scan disagree by at least one Rust identity.
- Third-party dependencies remain far above literal zero across Rust, JavaScript, Go, and Python.
- Bun/Nx are mandatory retained toolchain identities; literal zero for the complete repository is
  impossible without an explicit repository-instruction change.
- Current stdio/oracle and interactivity packets are still modifying the shared tree.
- The dedicated static gates are absent from the mandated developer launch surface.
- Cargo/native/Wasm/browser/replay/timing final-matrix stages remain deferred until source quiescence.

No ticket API was called and no ticket is claimed closed.
