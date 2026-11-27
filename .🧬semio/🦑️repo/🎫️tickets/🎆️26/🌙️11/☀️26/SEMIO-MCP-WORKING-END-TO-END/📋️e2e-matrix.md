# 📋️ Semio MCP End-to-End Matrix

Server: `.mcp.json` → `semio` (`repo dev mcp stdio os --folder .🧬semio/🔗space/os-mcp ...`). Probe: [🐍️probe.py](🐍️probe.py).

## Initial run (2026-11-26, live session)

| Area | Tool / resource | Result |
| --- | --- | --- |
| Boot | initialize, tools/list, list_changed | ✅ (dependencies ~25 s, native publication build, then ready) |
| Discovery | context_resolve, capabilities_search, capabilities_describe | ✅ |
| Inference discovery | inference_list | ⚠️ rows never carry `payload` contract, even when narrowed |
| Artifacts | artifact_open, artifact_validate, artifact_snapshot, resource `semio://artifact/{id}` | ✅ (probe kind) |
| Artifacts | artifact_create `s.note.note` | ❌ `BUDGET_EXCEEDED: guest poll reached its wall budget before it settled` |
| Artifacts | artifact_create `s.stdio.txt`, `s.space.space`, `s.draw.drawing` | ❌ `PLUGIN_UNAVAILABLE ... no component model yet` forever, no build runs |
| Artifacts | artifact_export (probe kind) | ✅ typed `PRECONDITION_FAILED` (no plugin owns the kind) |
| Actions | action_prepare `framework.*` | ❌ `owned by framework, not a plugin — no artifact channel routes to it` |
| Inference | inference_run `s.gis.gismap.inference` | ❌ blocked on missing component model |
| Inference | inference_get | ✅ typed `NOT_FOUND` for undeclared service |
| Jobs | job_get | ✅ |
| UI | ui_focus, ui resources | ⏳ typed `PLUGIN_UNAVAILABLE` without a shell — shell attach under test |
| Conversation | conversation_reply | ✅ (`shells: 0`) |
| Resources | resources/list | ✅ |

| Prompts | prompts/list, prompts/get ×5 (en, de) | ✅ |
| Resources | resources/templates/list, semio://capability, semio://workspace/artifacts, ping | ✅ |

## Fixes landed (source)

- Component models: launcher now reports failed model builds truthfully (`model-build-failed` + reason) and retries; stray hardlinked `lb` copy that broke the Cargo single-link source check removed.
- Framework capabilities (`framework.*`): route from `target.artifactRef`'s binding (headless + shell lane + guest address); catalog text says a target is required.
- Inference payload contracts: `artifact-pack-bytes` binding + `pack_bound` contracts for 67 glTF leaves, gis gismap, cad aec-building; conformance test that every declared inference publishes a contract. Descriptor regeneration (`repo run workspace:components -- describe stdio gis cad-extension-aec-building`) still pending on host memory.
- Resident build: `BUILD_SOURCE_CONFLICT` on hot global closures now compares the fact's answer, not its digest, so concurrent edits in the live worktree no longer fail long builds.
- `StallWait` (archive wait law) restored/defined and re-exported; law test passes.
- `semio-plugin-instance`: `semio-framework-trace` promoted from dev- to normal dependency (used by `[DEBUG]` heap marks in ingress).
- Shell rendezvous: stale-offer tests added on the shell side.

## Blockers observed (environment)

- 16 GB host: graph slice workers (3–6 GB) repeatedly suspended/cancelled by the memory warden; graph reads serialize all builds.
- Published `semio-os-mcp` is still the 2026-11-25 18:06 binary: every rebuild so far failed on concurrent in-flight refactors of other agents (`StallWait`, `semio_framework_trace`, job-context crate split, build-owner identity collision mid-write).

## Work streams

- [🔎️artifact-create-budget.md](🔎️artifact-create-budget.md) — guest poll budget fault.
- [🔎️component-model-build.md](🔎️component-model-build.md) — launcher never builds missing component models.
- [🔎️framework-capability-routing.md](🔎️framework-capability-routing.md) — framework capabilities + action/transaction/history flow.
- [🔎️inference-payload-contracts.md](🔎️inference-payload-contracts.md) — missing payload contracts.
- [🔎️shell-bridge.md](🔎️shell-bridge.md) — live shell attach and UI tools.
- [🔎️mcp-test-suites.md](🔎️mcp-test-suites.md) — existing test suites.
