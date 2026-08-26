# Writer Set-Locale Bounded Route

Date: 2026-08-26

## Scope

This packet admits only Writer `setLocale`. Writer `openDocument` remains fail-closed because it still performs content classification, snapshot construction, pack encoding, and SPR encoding as one monolithic command. This packet does not widen the truth claim for Writer's previously admitted routes.

Writer's app-wide localization contract is still red: `WriterConfig::default` currently hard-codes `en-US`. The bounded mutation route does not satisfy the final no-default-language requirement by itself.

## Production shape

- `setLocale` is an exact `WriterCommand` variant routed through the owner-local `WriterCommandJobFactory`.
- Typed admission rejects locale payloads above 64 bytes before publication and preserves command, snapshot, config, retained input, and cursor owners on rejection.
- One accepted step emits only `WriterConfigMutation::SetLocale`; no document, pack, snapshot, or effect construction occurs.
- The manifest declares `InteractiveJobClassification::Migrated` only after the exact factory proof and implementation were joined.

## Language-neutral and differential evidence

The existing schema-first fixture `📚️examples/🎬️demo-session/🧵️interactive-job-migration.json` now includes the exact `setLocale`/`locale` identity and 64/65-byte maximum/maximum-plus-one admission cases. The production Rust test consumes this fixture. The test-only serde path encodes and decodes the same typed `WriterCommand::SetLocale` value and compares it with the owned binary protocol, without adding a runtime dependency.

## Current gates

- `bun ./📜️script.ts verify interactivity tool-jobs --format json --output .../🧪️coordinator-writer-set-locale-tool-jobs.json`: expected global exit 1; Writer accepted rows include `setLocale`, and Writer remaining rows contain only `openDocument`.
- The global output remains red for unrelated Draw, exact declaration/proof, global payload, import, and remaining-command blockers.
- Focused native Writer tests are still compiling in the isolated ticket target at this checkpoint; no runtime pass is claimed until their exact results are appended.
