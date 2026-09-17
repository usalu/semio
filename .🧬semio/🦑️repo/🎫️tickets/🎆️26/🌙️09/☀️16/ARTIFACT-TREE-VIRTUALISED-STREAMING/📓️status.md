# 📓️ Status — Artifact Tree Virtualised Streaming (2026-09-16)

## Goal
No `+N` continuation rows anywhere. Every tree section/group shows its complete entry set: children materialise lazily on expand, only the viewport's rows are materialised, more stream in from the guest as the user scrolls. One framework mechanism for all apps.

## Phase 1 — audit (Sonnet fleet)
- 📓️audit-host-tree-pipeline.md — React side: InterpretedUiNode, UiDocumentStore, Tree element, scroll container, expansion intents, action → guest → re-render round trip.
- 📓️audit-surface-budgets.md — SurfaceReconciler / FlatPresentedNode cost, 8 MiB cap, UiValue arena, UI_BUILT_CHILDREN_MAX.
- 📓️audit-app-panels-a.md / 📓️audit-app-panels-b.md — every plugin panel tree builder and its paging shape.
- 📓️audit-paging-plumbing-and-schema.md — setPanelPage state, action registration, Tree contract schema pipeline (Rust → JSON → TS), wgpu target.
- 📓️audit-paging-tests.md — every test/fixture that pins +N / paging behaviour.

## Phase 1 — done 2026-09-16 (all six audits landed).

## Phase 2 — design
- 📓️design-virtualised-tree.md v1 final (coordinator). Channel: ViewModel.tree_windows; contract TreeWindow + granularity; SDK TreeWindows/tree_window_section/tree_window_item; host observer + host-owned expansion; UI_BUILT_CHILDREN_MAX 32→128, UI_VALUE_PAGE_ROWS 31→128.

## Phase 3 — implementation (Opus fleet)
- Wave 1 launched: P1 contract+ViewModel (📓️p1-contract.md), P3 SDK (📓️p3-sdk.md), P4a Tree element (📓️p4a-tree-element.md), P4b Interpreter+ShellHost (📓️p4b-host-wiring.md), P5 wgpu+reconcile law (📓️p5-wgpu-and-reconcile-law.md).
- Wave 2 launched concurrently (code first, cargo gated on 📓️p1-contract.md + 📓️p3-sdk.md): A1 puzzle3d, A2 puzzle2d/5d, A3 cad, A4 fem, A5 energy, A6 procedural+process3d+block, A7 flow/dag/vcs/note/sequence/writer/animate/imperative/remodel, A8 remaining plugins (shared brief 📓️wave2-app-brief.md; reports 📓️a1…📓️a8).
- Wave 3 browser verification after wave 2.

## 2026-09-17 progress
- Rate limit cut the whole fleet at ~01:00; all 13 agents resumed via SendMessage with their transcripts intact.
- Done + reported: P1 (📓️p1-contract.md), P3 (📓️p3-sdk.md), P4a, P4b, P5 (measured FlatPresentedNode = 6 520 B; one reconcile law re-derived), A1 puzzle3d, A2 puzzle2d/5d, A3 cad, A4 fem, A5 energy, A7 nine list/graph apps.
- 📓️verify-retirement-laws-preexisting.md: the two remaining ui-runtime/ui-contract retirement failures fail identically at the pre-change commit f7fef5746d → pre-existing.
- 📓️w3-browser-verification.md (cad 6020 own serve, fem3d 6087 peer serve): no `.more`/`+N`, spacer arithmetic exact, collapse/expand + expansion-across-refresh green, picks green; streaming confirmed on fem3d House (nodes 16 of 63). Two defects: (1) observer bound to the unbounded inner viewport div → scroll never re-requests (F1); (2) fem3d House body hit 129 > 128 nodes → SDK body-wide node ledger (F2).
- In flight: A6, A8, F1, F2. Own fem3d serve on :6187 for the re-probe.

## 2026-09-17 ~10:00 — wave 4 (new coordinator session; previous fleet died with its session)
- State found: F1 selector edit + F2 SDK node ledger on disk but unverified/unreported; A6 (procedural/process3d/block) tests rc=101, no report; A8 stale sweep: 5 crates BUILD-ERR, 4 with failing window laws, 5 with no laws.
- Brief: 📓️wave4-resume-brief.md. Launched — Opus: F1 (host scroll streaming), F2 (SDK body node ledger), A6a (procedural), A6b (process3d+block), A8a (forms/layout/reasoning/shooting), A8b (architect/lowpoly/space/trinity), A8c (rest), R1 (puzzle+cad re-verify), R2 (fem/energy/A7 re-verify). Sonnet: S1 (plugin residue audit), S2 (framework+TS residue/conformance audit), S3 (streaming-loop correctness review).
- Next: wave 5 fixes from S1–S3, then browser re-probe (step e scroll streaming) on fem3d House, cad, puzzle3d, process3d + one list app.

## 2026-09-17 ~15:00
- Audits done: 📓️s1-audit-plugin-residue.md (31/32 plugins windowed; norm HIGH → A8c, space parameters panel MEDIUM → A8b), 📓️s2-audit-framework-ts-residue.md (framework/TS conform, zero residue; SpaceAdministration cursor paging = network list, out of scope), 📓️s3-review-streaming-loop.md (2 HIGH: host/guest budget constants don't compose; uniform row pitch breaks with nested open containers; MEDIUM: off-screen open containers re-request; key collisions).
- Coordinator decisions from S3: one `TREE_WINDOW_BODY_NODE_BUDGET` (=111) in ui-contract + TS parity law, cost `1 + rows` per container charged once; real row geometry via `data-tree-window-row`; off-screen containers sent as `rows: 0` at held offset, guest order-independent; duplicate node_key per body = loud SDK error + host console.error.
- F1 DONE (📓️f1-host-scroll-streaming.md): all of the above on the host; Tree 31/31, Interpreter 112/112, ShellHelpers 21/21.
- Usage limit killed F2, A6a, A6b, A8a, A8b, A8c, R1, R2 at ~11:20; all resumed by SendMessage at 14:56.
- Next: wave 5 browser re-probe (step e per F1 §7) after F2 lands and guests are restaged.

## 2026-09-17 ~21:10
- DONE + reported: A6b (📓️a6b-process3d-block.md, block inspectors windowed, 20 laws), R2 (📓️r2-…; flow Generations body windowed; 46/47 laws), F2 first pass (📓️f2-…; budget in ui-contract, order-independent reservations, parity law, 26/26 panel-kit, House laws green).
- Coordinator decision (F2 §10): window identity = container PATH (`TREE_WINDOW_PATH_SEPARATOR = U+001F`; enclosing windowed container keys + own key); node keys / pick ids untouched; only duplicate sibling paths refused. SDK (F2) + host (F1) implementing; stdio-json base + writer get unique sibling keys.
- Second usage-limit cut at ~16:10 (reset 19:50) killed F1, F2, A6a, A8a, A8b, A8c, R1 mid-run; all resumed 21:08 with "checkpoint report first" instruction.
- Known peer breakage: fem3d demo swapped to concrete-forest at 12:02 (commit 0b460ed19f) → 42 stale fem3d tests (window laws → F2; rest fem owner).
