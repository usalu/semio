---
name: Native Brep Kernel Workforce
overview: Finish the native B-Rep kernel in `semio-s-3d` and delete all six `brepkit-*` git dependencies, by resuming the open ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT` at phase 4 and restructuring the remaining work into 18 file-disjoint agent lanes across 7 waves, coordinated by a frozen trait contract, an integrator-owned glue file, and ticket-local ownership/integration-request/lane-status mechanisms.
todos:
  - id: wave0-scaffold
    content: "Wave 0 (integrator, composer-2.5): reopen ticket 26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT; create ownership/integration-requests/lane-status/module-contracts files; create 20 stub module files under 📐️brep/ and register them in 📦️glue.rs; add lint target to project.json + LintScript to script.ts; add launch.json entries; capture baseline test+clippy logs"
    status: completed
  - id: wave1-foundations
    content: "Wave 1 (6 parallel lanes): bvh, primitive, measure, tessellate, oracle extension + brepkit differential harness, intersect shared types + curve-curve"
    status: completed
  - id: wave2-intersect-io
    content: "Wave 2 (6 parallel lanes): intersect/curve-surface, intersect/surface-surface, sweep, sew, step IO, mesh IO (STL/OBJ/GLB/DWG)"
    status: completed
  - id: wave3-classify-imprint
    content: "Wave 3 (3 parallel lanes): classify (point-in-loop/solid), imprint (UV arrangement + face split), heal + defeature + convert-to-nurbs"
    status: completed
  - id: wave4-boolean
    content: "Wave 4 (1 lane, flagship): boolean pipeline imprint->split->classify->select->stitch, plus section/split/compound_cut and native mesh-boolean fallback; SDF differential + volume additivity + determinism gates"
    status: completed
  - id: wave5-offset-blend
    content: "Wave 5 (2 parallel lanes): offset (offset face/thicken/offset solid/shell/draft) and blend (rolling-ball fillet, variable fillet, chamfer, edge-targeted variants)"
    status: completed
  - id: wave6-flip
    content: "Wave 6 (the flip, sequential): rewrite 🧰️kernel/🦀️component.rs as native Brep delegating to modules; drop six brepkit deps from Cargo.toml and Cargo.lock; rename BrepkitKernel->Brep across 12 consumer files; rewrite benches; delete differential harness"
    status: in_progress
  - id: wave7-hardening
    content: "Wave 7 (3 parallel lanes): exhaustive-tier fuzz and adversarial scale sweeps; consumer + wasm verification (flow_extension_brep, TS vitest, cargo build --workspace); runtime end-to-end confirmation of procedural-3d and CAD with [DEBUG] logs and Playwright screenshots in the ticket folder"
    status: pending
  - id: close-ticket
    content: Confirm no brepkit reference remains outside ticket artifacts, run the full repo verify gate, then ticket_close with summary and full file list; record deferred VCS .brep document layer and missing 📐️brep/AGENTS.md as dev follow-ups
    status: pending
isProject: false
---

« �ံ TRUNCATED »  and 2 named `🧪test`-prefixed entries), `📋️project.json` (`lint` target), `📜️script.ts` (`LintScript`)
- Cannot start a lane whose upstream contract row in `📐️module-contracts.md` is still `DRAFT`
- Cannot delete brepkit deps before Wave 6

### Verification gate, run by every lane before reporting done

1. `bun nx run semio-s-3d:test-quick` — output verbatim to `🧪lane-<id>-test-quick-run-K.txt`
2. `bun nx run semio-s-3d:test-long` for algorithmic lanes
3. `bun nx run semio-s-3d:lint` — zero new warnings attributable to the lane's file
4. Append a row to `🚦️lane-status.md` with gate results and the differential/oracle evidence
5. Write `🧾lane-<id>-scope-note.txt` in the Implemented / Validated by / Deferred format used by `phase-0-scope-note.txt`

## Explicitly out of scope for this run

- The `.brep` VCS document layer (phases 12–13 of the original ticket) — stays open on the ticket as follow-on scope
- The TypeScript `brepjs` / OpenCascade npm dependency in `bun.lock` — separate technology, separate ticket
- `async-trait`, `pollster`, `rayon`, `base64`, `blake3` — not brepkit, and dropping `async-trait` would force a sync trait and ripple into all nine consumers
- `📐️brep/AGENTS.md` is missing while [✏️s/🔨️modules/🧊️3d/AGENTS.md](✏️s/🔨️modules/🧊️3d/AGENTS.md) links to it — agents must not edit `AGENTS.md`, so this is recorded as a dev follow-up in the ticket close summary