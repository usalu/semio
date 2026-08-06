# Wave 1a complete

## Store seal
- DocumentCommand gains PruneDrafts (+ IngestRemote with serde helper from parallel agent)
- CommandReceipt returned from dispatch/reset/ingest_remote
- DocumentEnvelopeView + DraftStore alias
- set_state/set_envelope are pub(crate); public reload via reset()
- Call sites in sync/plugin/host updated

## VCS ids
- ID_COUNTER removed
- content_addressed_entity_id / edit_scoped_id added
- store mint_* helpers for edit/operation ids

## Engine
- ⚙️engine module with EngineCache, EngineHandle, EngineHost
- wired in glue.rs as os_engine
- ArtifactKind::Engine added

## Gate
- cargo check -p semio-framework-os-kernel --lib GREEN
