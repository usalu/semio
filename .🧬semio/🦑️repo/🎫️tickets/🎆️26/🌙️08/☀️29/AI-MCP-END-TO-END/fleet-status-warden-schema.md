# `repo fleet status`: warden action `report` was not in the fleet report schema

Cause: the host warden emits alarms with `action` in `suspend|resume|cancel|report` (warden schema, `report` = a unit above its ceiling is only reported on a calm host). The fleet-status schema admitted only `suspend|resume|cancel` for `WorkerMemory.lastAlarm.action`, so whenever the newest alarm was a `report`, `memory.warden` failed the closed report schema (`/memory/warden must match one anyOf branch`). It appears and disappears with the newest alarm.

Fix (schema-first): `lastAlarm.action` enum gains `report` in `🚢️fleet-status/🧬️schema/🔣️.json`; `FleetWorkerMemoryV1.lastAlarm.action` type in `🚢️fleet-status/🟦️.ts`; the summarizer passes the action through unchanged. Python oracle generator (`🧪️tests/🐍️.py`) and the TS `alarmOf` helper emit `report`; golden corpus case `warden-reported-alarm` (oracle-computed, local warden whose last alarm is a `report`) appended to `🧪️tests/🧫️fixtures/🔣️.json`.

Tests: `repo test-file …/🚢️fleet-status/🧪️tests/🟦️.ts`: 28 pass, 0 fail. `repo fleet status` prints the report again.
