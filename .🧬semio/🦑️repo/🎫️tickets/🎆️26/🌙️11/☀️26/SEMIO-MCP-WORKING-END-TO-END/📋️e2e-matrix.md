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

## Work streams

- [🔎️artifact-create-budget.md](🔎️artifact-create-budget.md) — guest poll budget fault.
- [🔎️component-model-build.md](🔎️component-model-build.md) — launcher never builds missing component models.
- [🔎️framework-capability-routing.md](🔎️framework-capability-routing.md) — framework capabilities + action/transaction/history flow.
- [🔎️inference-payload-contracts.md](🔎️inference-payload-contracts.md) — missing payload contracts.
- [🔎️shell-bridge.md](🔎️shell-bridge.md) — live shell attach and UI tools.
- [🔎️mcp-test-suites.md](🔎️mcp-test-suites.md) — existing test suites.
