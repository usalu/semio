# Nested Interaction Call

**Status:** In progress  
**Repo MCP:** unavailable in this session.

## Goal

Interactions can call other interactions via `interaction.call` effect — child state machine runs while host is paused until complete or abort.

## Scope

- `EffectSpec` + `applyTransition` + `StateEngineSendResult.childCall`
- `InteractionRuntime` child stack, snapshot `nested` metadata
- `spatial.shape` `pick.face` asset
- Typology construct codegen surface mode uses `pick.face`
