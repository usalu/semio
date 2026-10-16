# RESULT-W4-OS-FLOW: carve of `semio-framework-os-flow` and the two sibling artifact crates

Executor: Sonnet 5.5. Tools, scripts and logs: `osflow-tools/` and `🗑️generated/osflow-*.log` next to this file.

## 1. Outcome

| Target | Result |
|---|---|
| `semio-framework-os-flow` (37 files, 1,328,784 bytes, 19,306 lines) | Carved into 18 producer crates plus a thin facade. Every crate is within 3500 lines / 28 files / 230000 bytes for both its lib and its test closure. |
| `semio-framework-artifact-workflow-workflow` (34 files, 224,869 bytes, 3,051 lines) | Carved into graph / snapshot / schema / schema-laws plus a thin facade. All within target. |
| `semio-framework-artifact-flow-flow` (39 files, 321,804 bytes, 4,971 lines) | **Not carved.** A type-level cycle that cannot be cut without redesign blocks it (section 7). Left unchanged. |

Not achieved: a real `cargo check` / `cargo test` (section 6). All equivalence evidence is from the full-resolution rustc differential, which does not reach the privacy phase.

## 2. Census

Before (original single crate `semio-framework-os-flow`): 37 files, 1,328,784 bytes, 19,306 lines.
After: the facade is 1 file, 7,815 bytes, 107 lines. Per-crate closures (files / bytes / active lines), from `closure.ts` via `osflow-tools/censusall.py`; log `🗑️generated/osflow-census-final.log`:

| Crate | Lib | Test closure |
|---|---|---|
| geometry | 1 / 10,242 / 127 | 4 / 27,626 / 228 |
| mesh | 1 / 19,112 / 309 | 1 / 19,112 / 309 |
| drawing | 1 / 48,854 / 1,102 | 4 / 66,722 / 1,542 |
| operators | 6 / 77,211 / 1,435 | 10 / 108,445 / 1,786 |
| vcs | 3 / 93,202 / 1,976 | 6 / 103,001 / 2,092 |
| vcs-laws (test only) | 1 / 1,125 / 18 | 2 / 134,901 / 2,030 |
| extensions | 5 / 206,704 / 1,537 | 6 / 207,086 / 1,621 |
| extensions-laws (test only) | 1 / 1,220 / 19 | 2 / 61,409 / 397 |
| host-source | 4 / 61,908 / 305 | 10 / 85,932 / 350 |
| host-core | 10 / 198,567 / 3,406 | 10 / 198,567 / 3,416 |
| host-read | 3 / 46,119 / 301 | 5 / 55,471 / 330 |
| host-session | 13 / 216,687 / 2,571 | 13 / 216,687 / 2,571 |
| host-laws (test only) | 1 / 3,016 / 41 | 12 / 138,809 / 864 |
| host (composition) | 1 / 2,385 / 36 | 8 / 192,780 / 3,090 |
| wasm-domain | 6 / 80,355 / 1,573 | 6 / 80,355 / 1,574 |
| wasm-actions-a | 2 / 110,814 / 2,028 | 2 / 110,814 / 2,172 |
| wasm-actions-b | 1 / 97,473 / 1,875 | 1 / 97,473 / 2,019 |
| wasm (top) | 6 / 57,012 / 945 | 19 / 179,885 / 2,766 |

The wasm32 cfg gives the same numbers for these crates (checked with `TARGET_CFG`).

Workflow (log in the same folder, commands in section 6):

| Crate | Lib | Test closure |
|---|---|---|
| graph | 1 / 85,333 / 1,187 | same |
| snapshot | 11 / 56,429 / 388 | same |
| schema | 24 / 82,321 / 1,450 | 25 / 83,518 / 1,487 |
| schema-laws (test only) | 1 / 3,947 / 39 | 26 / 79,837 / 850 |
| facade | 1 / 1,685 / 22 | same |

## 3. Crate graph

Flow (arrows point to dependencies, downward only):

- geometry, mesh, drawing: leaves.
- operators (registry, catalogue, bridge, extension schema): leaf.
- vcs (forms, retained VCS, `history_identity!` macro_export): leaf.
- extensions -> operators.
- host-source: leaf.
- host-core -> host-source, operators, extensions, geometry, vcs.
- host-read -> host-core, host-source, operators.
- host-session -> host-core, host-source, host-read, operators, geometry.
- host -> host-core, host-read, host-session, operators, drawing, vcs.
- wasm-domain -> host-core, vcs, geometry.
- wasm-actions-a -> wasm-domain, host-core. wasm-actions-b -> wasm-domain, host-core, geometry, drawing, operators.
- wasm -> wasm-domain, wasm-actions-a, wasm-actions-b, host-core, vcs, drawing (dev: host).
- laws crates (test only): vcs-laws -> vcs; extensions-laws -> extensions; host-laws -> host-core, host-read, host-session, host-source, operators, drawing.
- `semio-framework-os-flow` facade -> all producers except laws crates and host-source (explicit `pub use` lists derived from the pre-split public names; `extern crate self as flow_extension_sdk` kept). It carries no dev-dependencies and no tests.

Constraints that fixed this shape:

- Orphan rule: foreign-trait impls stay with their type. `FlowDomain` trait, bridge, dispatcher and `impl FlowDomain for FlowDomainAdapter` sit in the top wasm crate, which breaks the adapter/actions cycle.
- `FlowSourceBaseline` is a genuine inversion trait (host-read defines it, host-session implements it), so `host-read` sits below `host-session`.
- Catalogue cycle with the host: `merge_catalogue_sections` moved into host-core, `flow_backed_node_graph_extras` into host-session.

Workflow: graph (model, media contracts, `WorkflowInput`) <- snapshot (`WorkflowSnapshot`, bindings, io snapshot codecs) <- schema (mutations, diff, apply, io mutation codecs); schema-laws holds all tests; the facade re-exports graph, snapshot and schema.

## 4. God-object rule (coordinator rule in the brief)

Not triggered. The largest inherent impl, `FlowHost` (152 KB, 2,601 lines), fits in `host-core` (198,567 bytes, 3,406 lines), so no component decomposition was needed. For the decomposition that would be needed for flow-flow see section 7.

## 5. Files changed, consumers re-pointed

Created (flow): 18 crate nodes with `Cargo.toml` + `📋️project.json`, new roots for `🗃️operators`, `🖥️host/🧱️core`, `🖥️host/🎞️session`, `🖥️host/⚖️laws`, `🌿️vcs/⚖️laws`, `🧩️extensions/🕸️wasm/⚖️laws`; the former 5.4k-line host module and 5.8k-line wasm module split into child modules (stage 1, verified at 0 diff); facade root `📦️packages/🦀️rust/🦀️.rs` and `Cargo.toml` rewritten.
Created (workflow): nodes `🕸️graph`, `📸️snapshot`, `🧬️schema`, `🧬️schema/⚖️laws` (crate names `semio-framework-artifact-workflow-workflow-{graph,snapshot,schema,schema-laws}`, projects `@semio-tech/framework-workflow-workflow-{id}-rs`); facade root and `Cargo.toml` rewritten; glue `🚪️io/💾️binary/🦀️.rs` and `🚪️io/📝️text/🦀️.rs` lost their `mutations` mounts (the mutation codec leaves are mounted by the schema crate); the 20 mutation-leaf `mod tests` mounts moved into schema-laws.
Registration: root `Cargo.toml` members (+22), `📋️build-owners.json` (+22 rows), six launch routes per crate in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, `[profile.wasm-prototype.package.<crate>] opt-level = 2` for the 15 flow product crates in the 144 manifests that already carry the `semio-framework-os-flow` entry (8 ms interactive contract), and the same 15 names in `🧑‍💻dev/🧪️tests/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json` (`iterateHotPackages`).
Test fixtures: `include_str!` of fixtures and sources replaced by runtime reads (`read_node` / `read_source` helpers) so test closures stay small; the wasm top root gained `read_source`/`production_source`.
Consumers re-pointed:

- Root `📜️script.ts` (module-tree readers for the host: `🧱️core`, `🎞️session`).
- `🦑️repo/.../🟦️typescript/📜️script.ts` (law `slider_ghost_descriptor_requires_authored_label` now in `semio-framework-os-flow-host`).
- Flow facade `📜️script.ts` (`test-extension-registry` -> operators, `test-form-producer` -> vcs); `🫀️core` `TestScript` now runs all test-owning flow crates.
- `📺️renderer/.../🫳️node-graph-gestures/🟦️.ts` (reads `✏️host-authoring` + `🌳️host-evaluation-history` instead of the old host root).
- Four Flow TS readers fixed earlier in the wave.
- Workflow facade `📜️script.ts` (`packages:` list, sqlite tests -> `...-schema-laws`).
- No Rust consumer outside the family needed an edit: all 117 distinct `semio_framework_os_flow::<name>` paths used in the repo resolve through the facade's explicit exports; the only dropped export is workflow's `io` module (no consumer in the repo).

## 6. Verification (real output)

- **Full-resolution differential** (`rustc +nightly-2026-07-20 --emit=metadata`, union sandbox, external-crate globs neutralized), final run, logs `osflow-union-final-flow.log` and `osflow-union-final-workflow.log`:
  - flow: native, test, wasm32-unknown-unknown, +test, wasm32-wasip2, +test all show 0 only-split errors, except 1 benign line in the `test` config (`unresolved import semio_framework_trace_memory_testing`, a new external dev crate a peer added to geometry's tests).
  - Only-baseline lines are explained: the removed `flowSetCompilerFonts` export (peer's runtime-assets change, not resurrected), and the fixture/`CARGO_MANIFEST_DIR` reads now done at runtime.
  - workflow: native, test, wasm, wasm+test: TOTAL DIFF 0.
  - Baselines: `base-flow.pkl`, `base-workflow.pkl`, `base-flowflow.pkl` in the scratchpad (pre-split snapshot).
- **Widening**: `widen_run.py` (flow) and `widen_wf.py` (workflow) dry runs report nothing pending. Seeded sensitivity check: a seeded private fn, method and field used by a dependent crate were all reported.
- **Region markers**: none lost; flow gained the `🔖️Laws` pair (vcs-laws); workflow originally had none.
- **Syntax/paths/JSON**: the union compiles every owned file, so parse and `#[path]` resolution are proven. `📋️build-owners.json`, `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` parse (Bun JSONC); the fixture JSON parses.
- **TS readers**: `🌐️geometry/.../🎟️caller` passes under `bun test` (1 pass). Via direct calls: manifest-admission OK, retained-receipt OK, retained-grant OK, VCS closure OK. Evaluation-source read/work/seeds (`bun test`): 11 + 1 + 1 pass. Extensions SDK TS test: 22 pass / 12 fail; I checked the 2 failures that read the extensions root: both expect strings that are absent from HEAD and from the pre-split tree (stale, not caused by the split). Host-source custody fixture lists 40 fields but the struct has 41 (`history_store_factory`, peer addition, stale). The source-contract script fails at an assertion about the canvas crate (peer carve). The `close_owned_step` mutation of the interactivity audit never bound live source, before and after the split.
- **Real compile**: tried at the start and at the end.
  - `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-flow-geometry`: at start `Finite original maintenance authority required`; the `bun repo run <project>:check` route failed on `Canonical native dependency owner required` (the `◻️2d` engine owner row, peer) and later `Active graph readers retain required Nx pool headroom`.
  - After I ran the recipe's `... -- update --workspace --offline` (exit 0, root `Cargo.lock` refreshed, which also took in peers' new crates), the check was refused with `Rust crate Source boundary exceeded before compiler byte allocation` for `semio-framework-os-kernel`, a peer-owned dependency. No cargo check or test of my crates could run.
  - Commands: `🗑️generated/osflow-check-*.log`, `osflow-nx-*.log`, `osflow-lock-update.log`.

## 7. Flow-flow: why it was not carved, and the options

`FlowHostSnapshot` (document types and all their codecs: text 23 KB, binary, sqlite about 110 KB) and `FlowMutation` (+ leaves, diff, canonical) form a type-level cycle:

- D (document) refers to M (mutation): `ArtifactStoreSnapshot::native_snapshot_registration()` and `publish_native_snapshot()` return `bare_artifact_codec::<FlowHostSnapshot, FlowMutation>`; `FlowOwner::Mutation(FlowMutation)` and `FlowLayoutEntry` sit in the retained owner enum; `retained/fields` implements `RetireOwned` for `FlowMutation` and its leaves.
- M refers to D (every leaf's `MutationKind<FlowHostSnapshot, FlowMutation>` impl and `FlowDiff`).
- The orphan rule keeps every inherent or foreign-trait impl for `FlowHostSnapshot`/`Widget` in the crate defining the type, so the codecs cannot move away from the types. `native_snapshot_registration` is a foreign trait method without type parameters, so it cannot be rehomed to M.
- Sizes: D-only about 205 KB / 3,200 lines / 25 files and M-only about 71 KB / 1,600 lines / 26 files would fit if the cycle were cut; D+M together (about 275 KB, 4,600 lines) do not. The independent `retained/📑️copy` file (47 KB, 607 lines) would split off cleanly but alone leaves 274 KB / 4,364 lines.

Options (each needs a decision outside this carve):

1. Change os-kernel so the pair registration is declared by the producer (`ArtifactStorePublication`) instead of the snapshot trait (peer-owned crate).
2. Make `FlowOwner` open (erased owner) so it does not name `FlowMutation`, and move `FlowLayoutEntry` into the document; both are API changes for roughly a dozen retirement call sites.
3. Apply the god-object rule: decompose the snapshot into component structs (widgets, synapses, gui) with component codecs per crate. This is the structurally right long-term answer and the largest change.

## 8. Procedural plugin coupling (recorded, not changed)

The procedural plugin crates link `semio_framework_os_flow` natively (for example `sync_host_flow_extension_contributions_page`, `flow_extension_registry_generation`, `FlowEvalSession` in `generation2d/.../🧩️set-contributions`). The seven flow extensions enter through a process-wide registry fed by the paged `setContributions` command, bypassing the WIT extension world. The facade keeps these names, so the coupling is unchanged.

## 9. Residual risks

- **Privacy phase unverified.** Field and method privacy (E0616/E0624/E0603) is not reachable by the resolution-phase oracle. Widening is name-based; the first real `cargo check` of the new crates may still show a missed item.
- **Cargo.lock.** The root lock was refreshed offline once; the plugin workspace locks (wfc, procedural, hub, teaching and others) still list the old facade edges and need the same refresh when the launcher allows.
- **Manifest edits by peers.** `semio-framework-os-flow-geometry` carries a dev-dependency (`semio-framework-trace-memory-testing`) added by the TRACE-TESTKIT agent; `build_crates.py` does not know it, so do not rerun it without re-adding it. The same goes for any other peer lines.
- **Union tooling leak (fixed).** Earlier union runs followed `#[path]` mounts outside the sandbox and rewrote a shared trace test file (reported by the coordinator). The scan found no `crate::m_` or `pub(in crate::m_` residue in tracked or untracked sources. `union.py`, `unionf.py` and `sandbox.py` now refuse to write any file that is a symlink or whose realpath is outside the sandbox.
- **Test-only crates run separate targets.** Launch routes exist for them, and the `🫀️core` `TestScript` and the workflow facade's `packages:` list now cover them.
- **Opt-level profile.** The profile entries cover only product crates; if a new flow product crate is added later its entry must be added to the 144 manifests and the policy fixture.
