---
goal: HOOKS/AGENT-EVENTS
---

# Ticket

## Summary

Refactored all agent hook events to match the new YAML spec exactly.

## Changes

- Renamed `agent.starting` → `agent.started` (HookEvent constant, result type, dispatch, tests, help text, log filenames)
- Renamed `agent.tool.code.searching` → `agent.tool.searching` (HookEvent constant, result type, dispatch, tests)
- Created `HookResultAgentBase` struct with shared fields: session, timestamp, client, llm, transcript, message (ID), parent
- Updated all 12 agent result types to embed `HookResultAgentBase` instead of directly embedding `HookResultBase` + individual fields
- Updated `dispatchHook` with `agentBase()` helper that populates all shared fields from input extraction
- Added 3 extraction functions: `extractTranscriptFromInput`, `extractMessageIDFromInput`, `extractParentMessageIDFromInput`
- Fixed JSON tag on `MessageID` field to use `json:"message"` (matching spec) with Go depth-based shadowing of `HookResultBase.Message`
- Updated `TestHookCommandJSONOutput` to verify timestamp instead of shadowed dispatch message
- Updated `TestHookResultJSONFields` to set `MessageID` instead of `HookResultBase.Message`
- Updated `TestHookLogging` filename expectations from `_agent-starting.json` to `_agent-started.json`
- Updated all test struct literals for nested `HookResultAgentBase{HookResultBase{...}}` embedding

## Log

- All 14 hook events match the spec exactly
- All hook-related tests pass (77s)
- CLI binary builds successfully
- go vet clean (except pre-existing unreachable code at line 37259)

## Todos

- [x] Rename `agent.starting` → `agent.started` (HookEvent, result types, dispatch, tests)
- [x] Rename `agent.tool.code.searching` → `agent.tool.searching` (HookEvent, result types, dispatch, tests)
- [x] Add missing fields (`llm`, `transcript`, `message`, `parent`) to all agent event result types
- [x] Update `HookContext` with `LLM`, `Transcript`, `Message`, `Parent` fields
- [x] Update `dispatchHook` to populate new fields from input extraction
- [x] Add extraction functions for `transcript`, `message`, `parent` from input
- [x] Update all event mapping functions (resolve\*Event) for renamed events
- [x] Update `vsCodeEventFromHookEvent`, `formatVSCodeHookOutput` for renamed events
- [x] Update `hookCommand` help text
- [x] Update all tests in `main_test.go`
- [x] Build and run tests

## Plan

### Spec Analysis

The new spec defines these events:

**git.commit.starting**: message
**git.commit.ended**: sha, message

**agent.started** (was agent.starting): session, timestamp, client, llm, transcript, parent
**agent.ended**: session, timestamp, client, llm, transcript, message, parent
**agent.prompt.submitting**: session, timestamp, client, llm, message, parent, prompt
**agent.compacting**: session, timestamp, client, llm, transcript, message, parent, chat

**agent.tool.starting**: session, timestamp, client, llm, transcript, message, parent, name, input
**agent.tool.ended**: session, timestamp, client, llm, transcript, message, parent, name, input, response

**agent.tool.plan.updating**: session, timestamp, client, llm, transcript, message, parent, steps (name, status)
**agent.tool.searching** (was agent.tool.code.searching): session, timestamp, client, llm, transcript, message, parent, query, include, exclude

**agent.tool.code.editing**: session, timestamp, client, llm, transcript, message, parent, path, old, new, all
**agent.tool.code.edited**: session, timestamp, client, llm, transcript, message, parent, path, old, new

**agent.tool.terminal.starting**: session, timestamp, client, llm, transcript, message, parent, command
**agent.tool.terminal.ended**: session, timestamp, client, llm, transcript, message, parent, command, pid, terminated, stdout, stderr

### Key Renames

1. `agent.starting` → `agent.started`
2. `agent.tool.code.searching` → `agent.tool.searching`

### Missing Fields (to add to ALL agent event result types)

- `llm` (LLM model identifier)
- `transcript` (transcript file path)
- `message` (message ID)
- `parent` (parent message ID) - some types already have `parent` but need renaming context
