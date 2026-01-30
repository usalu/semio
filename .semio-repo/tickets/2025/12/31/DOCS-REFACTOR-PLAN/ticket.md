# Ticket

## Todos
# Previously

- Initial analysis compared `AGENTS.md` and other docs with current implementation.
- Found multiple mismatches: missing `hooks/` folder, `scripts/log.ts` references, `repo` CLI vs scripts, `tool` command mismatch, terminology inconsistencies (Port vs port), and several undocumented commands and files.

# Plan

## Executive Summary

The documentation requires fixes across several areas:

- Remove or update outdated references (scripts/log.ts, tool command, hook file names).
- Add documentation for implemented repo commands and features (update-metabolism, definition tree, caching).
- Standardize terminology (Port → port, AppStore naming).
- Create or restore missing tooling files referenced by docs (`hooks/*.tsx`) or update docs to reflect current implementation.

## Phase 1: Critical Infrastructure Fixes

1. Hooks folder missing (BLOCKER)
   - Create `hooks/` folder or update `preflight.ts` and documentation to reflect current hook locations.
   - Ensure presence of `hooks/code.tsx`, `hooks/i18n.tsx`, `hooks/prettier.tsx`, `hooks/ruff.tsx`, `hooks/typescript.tsx`, `hooks/eslint.tsx` or update docs to use existing scripts.

2. Scripts folder references
   - Remove/replace references to `scripts/log.ts` and other `scripts/*.ts` calls in `AGENTS.md` with the `repo` CLI equivalents implemented in `go/repo`.

## Phase 2: Repo CLI Documentation Alignment

1. Remove non-existent `tool` command from docs and reconcile with `go/mcp`:
   - Either implement `repo tool` in `go/repo` or remove `tool_run` from MCP and the `tool <name>` row in `AGENTS.md`.

2. Document implemented commands in `go/repo/main.go`:
   - Add `update-metabolism` to the CLI table.
   - Add `definition tree <file>` to the CLI table and explain its behavior.

3. Ticket status terminology
   - Normalize the ticket status to `finished` in code or update `AGENTS.md` to accept `closed`. Prefer changing `go/repo` `TicketClosed` to `finished` for consistency.

4. Ticket iteration file flags
   - Either update `go/repo` to accept `--file`, `--file-created`, `--file-removed` flags, or remove these examples from `AGENTS.md`.

## Phase 3: Terminology Standardization

1. Port vs Port
   - Update `AGENTS.md` to use `port` for user-facing documentation while documenting the code-level `Port`/`PortId` types and mapping.

2. Store class names
   - Clarify that `PlainAppStore` / `PlainKitDiffAppStore` are the concrete class names; update the Store Architecture section accordingly.

## Phase 4: File Reference Cleanup

1. Remove references to non-existent files that were merged (e.g., `xstate-hooks.ts`, `machines.ts`) and point readers to `Sketchpad.tsx`.
2. Update hook extension examples from `.ts` to `.tsx` where `preflight.ts` expects `.tsx`.

## Phase 5: Missing Documentation

Add missing documentation entries for:

- `update-metabolism` CLI command
- `definition tree` CLI command
- file caching system (`.semio-repo/cache`)
- Kit diagram relationships implemented in `Kit.tsx`
- `LoadingKit` interface in `Home.tsx`
- `MobileDevice` / device type in `shared.ts`

## Phase 6: AGENTS.md Section-by-Section Updates

- Update CI/CD and Hook Workflow sections to match actual hook file names and locations.
- Update Repo CLI commands table: remove `tool` row, add `update-metabolism`, `definition tree`.
- Update MCP tools list to match `go/mcp` and `go/repo` capabilities.
- Update Ticket System examples to use `repo` CLI and remove programmatic TypeScript usage references.

## Implementation Order (Priority)

1. P0: Create hooks or update docs to remove missing references.
2. P0: Fix ticket status mismatch in `go/repo` or docs.
3. P1: Replace `scripts/log.ts` references with `repo` CLI.
4. P1: Remove/resolve `tool` command documentation.
5. P2: Standardize Port/Port terminology.
6. P2: Document implemented commands in `AGENTS.md`.
7. P3: Misc cleanup and validation.

## Validation Checklist

- `npm run preflight` runs without errors (or docs updated to reflect `preflight.ts` expectations).
- All documented commands work: `repo analyze`, `repo fix`, `repo ticket open`, etc.
- No references to `scripts/log.ts` remain.
- No references to non-existent files remain in docs.
- Ticket status uses consistent `finished` terminology.
- `hooks/` folder exists or docs no longer reference missing hooks.

# Changes

- Created this ticket and restored the formatted refactor plan.

## Changes

## Log

## Summary
# Summary

"Restore formatted refactor plan for outdated documentation"
