# Admission Fix: `🏗️build/⚡️caching` ENOENT in `build:source-admission`

## Root cause
Transient, caused by the in-flight split of `🏗️build/🕸️graph/🟦️.ts` into `📁️workspace`, `🧭️discovery`, `🕰️host`, `📜️contract`, `📥️load`, `🔐️authority` (files written 01:17:59). Failed run was 01:03: the split modules were one directory deeper than the monolith, so relative `../../⚡️caching/...` strings resolved to `🏗️build/⚡️caching` (missing). Bench re-based them.

## Evidence
- Light harness `🧰️scripts/capture-harness.ts` (calls `captureNxAuthorizerSourcesV1` directly) now passes for entries: `🏗️build/📜️script.ts` (1834 files), `authorizers/📜️script.ts` (1743), the native directory runtime entry (58), `closure/🚪️command/🟦️.ts` (228). No reproduction.
- Registry `🔌️backends/📋️registry/🔣️.json` paths are intentionally relative to `🔌️backends/` (schema pattern, `buildBackendDriverUrlV1`, orchestration, tests); not a cause.

## Changes
- New test `🏗️build/🧪️tests/🧭️relative-paths/🟦️.ts`: every static import/export-from, dynamic `import("...")` (non-test files), `new URL(x, import.meta.url)` and registry path resolves to an existing file. Green.
- MCP: exposed `scheduler status|cancel|run` as `scheduler_status`, `scheduler_cancel`, `scheduler_run` in `🔌️mcp/🖼️assets/🏗️build-surface.json` (+ flagTypes `--depth`, `--peak-mb`) and golden tools in `🔌️mcp/🧪️tests/🧫️fixtures/🏗️build-surface.json`. Build-surface oracle 6/6 green; mcp capability/roundtrip/unit tests green.

## Gated dry run
See bottom section.
