# Warm gateway discovery contract

During staging the gateway answers `tools/list`, `resources/list`, `resources/templates/list` and `prompts/list` with empty lists (schema const `WarmDiscovery`) instead of queueing them. Other requests (`tools/call`) stay queued; `failed` refuses everything with -32603; modern (`server/discover` first) clients get no warm replies. On native readiness the three `list_changed` notifications are emitted.

Changed: gateway fixture `🧪️tests/🔣️.json` (rule, four discovery reply rows, initialize instruction text, journeys `staged-successfully` and `staging-failed`), gateway test `🧪️tests/🟦️.ts` (SDK list-result schemas, stdio splice test, early-peer-exit uses `tools/call`), `🌉️mcp/README.md` "Run it" paragraph.

Verified with `repo test-file` on the gateway test: 25 pass, 0 fail.
