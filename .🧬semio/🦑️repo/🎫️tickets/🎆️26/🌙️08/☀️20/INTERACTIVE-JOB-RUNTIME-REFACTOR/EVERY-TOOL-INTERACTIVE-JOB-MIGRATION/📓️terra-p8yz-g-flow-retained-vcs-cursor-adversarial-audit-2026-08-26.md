# P8yz-g Flow Retained VCS Adversarial Cursor Audit

Date: 2026-08-26  
Auditor: Terra (independent read-only audit)  
Verdict: **RED — blocked at the source/static boundary**

## Scope and evidence

Read the master all-app acceptance contract, the claimed implementation report, the live Flow retained VCS production region, all three Flow-local fixtures, and the adjacent retained tests. This audit intentionally inspected the complete live `🌊️RetainedVcs` region rather than trusting the report's narrower `🌊️RetainedActionCursor` substring.

Files audited:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures/📒️lifecycle.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures/🗂️owners.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures/🔮️oracle.json`

No production source was changed. Cargo, Nx, Wasm, native/browser, and timing gates were not run because parallel Rust work remains active.

## Blocking defects

### RED-1 — the live retained region still contains whole recursive scans and a whole-fixture digest

The implementation report’s static predicate only isolates lines 1856–2509, but the production retained region is lines 959–2649. It omits the active constructor, feature admission, close, and digest paths that the public `FlowRetainedVcs` invokes.

- `FlowVcsDocument::new` unconditionally calls `flow_vcs_fixture_digest(&fixture)` at `component.rs:1349-1353`.
- `flow_vcs_fixture_digest` performs nested whole-document/byte iteration at `component.rs:2625-2646`: every schema byte, widget, synapse field, and layout id is walked synchronously. This directly contradicts the report’s claim that the committed digest is scalar metadata and no whole-fixture digest walker remains.
- `begin_replace_document` calls `flow_vcs_fixture_census(source.get()?)` before transfer at `component.rs:1543-1547`; the census recursively walks every widget, synapse, layout entry, dictionary, tree, node, preview, and nested value at `component.rs:2518-2623`. Ordinary add/patch admission also recursively walks supplied widget content.
- The same retained region contains an operation-slot `.position` scan (`component.rs:1797-1803`), `rediscover` `.iter().find` (`component.rs:1645-1647`), resource/surface aggregate scans (`component.rs:1139-1156`, `1407-1409`), and seventeen `for` spellings overall.

Those scans are not cursor state and have no fuel/deadline/cancellation boundary. Consequently the retained feature has neither a fixed one-grant admission-census unit nor the required prohibition of hidden iteration/position scanning. Limiting the report’s predicate to the action helper is a scope evasion, not evidence for the live API.

### RED-2 — cancellation/fault during redo retirement drops history ownership

New-branch publication first enters `RetireRedo` when redo is nonempty (`component.rs:1902-1905`). Each poll transfers one redo owner to `retired_actions` and increments `redo_retired` (`1881-1885`). That state needs rollback.

`cancel` and `fault` enter `Rollback` only when the cursor was mutated, owns the edit lock, or has history mode 1/2 (`1611-1638`). A normal new-branch cursor has history mode 0; while in `RetireRedo` it need not have mutated or acquired the edit owner. Therefore cancellation/fault after one or more `1882-1884` transfers leaves the cursor outside rollback. `close_operation_step` only restores retired redo actions inside the rollback branch (`1660-1666`), then otherwise retires the operation’s action (`1697-1709`). The displaced redo owners remain in `retired_actions` and are subsequently destroyed by `close_retired_step`.

This is an exact partial-state handback failure: cancel/fault after redo retirement loses redo history. It falsifies the report’s claimed cancellation restoration and violates the acceptance contract’s cancellation before/after every ownership transfer.

### RED-3 — deadline/interruption controls and close are not fail-closed

`poll` validates the full grant envelope, but `cancel`, `fault`, and `close_operation_step` test only some of controls/fuel/items (`1611-1613`, `1626-1628`, `1649-1651`). `close_retired_step` does the same (`1720-1725`). None rejects `interrupted`, an expired deadline, or a grant whose deadline window exceeds the eight-millisecond rule. These calls can mutate stage, rollback state, pages, retained owners, and credits under an interrupted or expired grant.

This directly conflicts with the stated zero-fuel/deadline/interrupted-close law. The existing test only sends expired/interrupted grants to `poll` (`2982-3000`), so it does not cover the broken mutating controls or close paths.

### RED-4 — one-grant publication combines distinct ownership transfers and publication

`publish_cursor` takes the action into undo/redo history (`1940-1946`), then may detach a surface into the retired-surface ledger (`1952-1954`), writes revision/generation/owner metadata, computes aggregate counts/digest, creates the page, stores it, and publishes `PageReady` (`1947-1977`) in one poll. This contains at least an inverse-owner transfer, optional surface-owner transfer, and output publication with no intervening cancellation point.

The acceptance contract requires one semantic unit per grant and cancellation immediately before/after every ownership transfer. Treating the entire block as one generic “publish” helper does not satisfy that requirement, particularly where two independent retained owner transfers occur.

### RED-5 — fixtures/laws do not specify or probe the discovered hostile states

The JSON fixtures parse and have the reported headline inventory (13 features, 24 laws, 8 listed transfer cancellations, 19 fingerprint fields, 62 owner names, and private `FlowSemanticOracle`). However, `📒️lifecycle.json` has no law for redo-retirement restoration, deadline/interruption of close/control, or incremental admission census. Its eight cancellation labels are lifecycle milestones, not the actual transfer boundaries in retained code.

The executable cancellation test only runs checkpoint operations (`component.rs:3068-3106`) and cannot reach redo retirement, a widget/synapse removal inverse, layout replacement, document-version retirement, or nested-census admission. The oracle test covers only the small layout result and has not been executed under the permitted gate set. It does not prove independent owned-oracle parity for the retained feature matrix.

## Non-blocking GREEN observations

- The dedicated action-cursor substring is 654 lines and has no legacy `apply_action`, `flow_vcs_apply_action`, `Vec::insert`, `Vec::remove`, `mem::replace`, or `serde_json` token.
- The live retained region uses fixed `[Option<T>; N]` owner ledgers for undo, redo, retired actions, retired surfaces, and document versions; it has no retained `Vec<FlowVcsAction>`, `Vec<FlowSurfaceOwner>`, `Vec::with_capacity`, or `.clone()` spelling.
- The cursor does retain scan/origin/current/target/history/redo state and uses adjacent swaps plus typed item moves for the visible collection mutation phases.
- `rustfmt --edition 2021 --check` on the VCS source passed. `git diff --check` on the VCS source and three fixtures passed. A standalone Bun parser/statics gate parsed all fixture JSON successfully.

## Raw census reconciliation

The live raw command below returns **eight** paths, not the stale historical nine:

```sh
rg -l --glob '*.rs' 'reject_whole_buffer_artifact_envelope_ingress' | sort
```

The exact eight are the shared Store guard, framework directed DAG, Shooting, FEM 2D, FEM 3D, CAD, Puzzle 5D, and Puzzle 3D. Flow is absent. This agrees with the live source census and should supersede any report phrasing that suggests a currently observable ninth raw Flow caller.

## Deferred gates

A4 Flow consumer/runtime adoption remains deferred: no evidence here proves the Wasm dispatcher consumes this retained VCS API, or that native/Wasm ledgers, timing, browser responsiveness, launch registration, localization/accessibility, or all-app matrix behavior pass. Those gates remain required by the master contract, but RED-1 through RED-5 already block P8yz-g source/static acceptance.

## Required remediation before re-audit

1. Move all source census/digest/slot discovery work into explicit bounded cursor phases (or replace it with schema-fixed scalar metadata); do not hide it outside a smaller static-scan region.
2. Make redo retirement cancellation/fault always enter a restoration cursor, including history mode 0 with `redo_retired > 0`; add exact fingerprint laws at each retired-owner count.
3. Apply the same full grant validation to every mutating control and close step, and add zero-fuel, interruption, expired, and over-window assertions for each.
4. Split history transfer, surface transfer, revision publication, and page publication into separately cancellable retained cursor phases.
5. Extend the language-neutral fixture and executable owned/oracle law matrix to cover every actual transfer boundary and all partial states above.
