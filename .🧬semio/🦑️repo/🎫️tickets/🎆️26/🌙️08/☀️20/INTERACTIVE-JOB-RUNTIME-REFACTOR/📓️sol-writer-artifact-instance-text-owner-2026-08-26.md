# Writer Artifact-Instance Text Owner

## Scope

This packet retires the Writer `WRITER_SCRATCH` row without an exemption or a process-wide wrapper. The prior `OnceLock<Mutex<HashMap<String, Arc<str>>>>` made a child ID ambient authority: two app instances, a stale snapshot, and a freshly reused identity all addressed the same process payload.

Writer mutation, inverse, import, export, fixture, inference, and render paths need the text while operating on a pure `WriterSnapshot`; those paths have no `ArtifactApp` or retained-operation context. The narrow ownership seam is therefore the `ArtifactChild<S>` embedded by that artifact instance. `ArtifactChild<S>` now optionally carries an immutable `Arc<str>` local-text owner. Clone retains the same owner; serde and DSL materialization omit it; a separately materialized handle owns a separate value even when hostile input reuses the durable child ID.

## Production Changes

- `ArtifactChild<S>` gained private, serialization-skipped local text plus bounded behavior methods: `with_local_text`, `set_local_text`, `local_text`, and `local_text_owner`.
- Clone retains the immutable owner. Durable identity, equality, debug, DSL shape, pack shape, and wire JSON remain defined only by `child_id` and `target`.
- Writer deleted `writer_scratch`, `WRITER_SCRATCH`, `cache_writer_document_text`, and `document_child_handle_and_cache`.
- Writer constructors and mutation diffs now use `document_child_handle_with_text`; decoded committed fixtures materialize text directly on their exact child handle with `attach_writer_document_text`.
- `writer_text_owner` now retains the snapshot's own immutable owner, so retained jobs copy no text bytes and cannot observe another instance.

## Schema-First Law

Language-neutral inputs:

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🧪️fixtures/writer-child-local-text.schema.json`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🧪️fixtures/writer-child-local-text-law.json`

The fixture is capped at four cases and exercises:

1. clone identity (`Arc::ptr_eq`, exact strong-owner count),
2. two independently materialized instances with a deliberately colliding child ID,
3. stale A / reused identity B / stale A readback (ABA), and
4. third-party serde wire omission plus unresolved decode.

The Rust fixture consumer lives in Writer's artifact component test. Ajv 2020 is the independent schema oracle.

## Fresh Evidence

### Ajv 2020 oracle

Command:

```text
bun -e '<Ajv2020 compile and validate>' _ ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🧪️fixtures/writer-child-local-text.schema.json ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🧪️fixtures/writer-child-local-text-law.json
```

Result: exit `0`, `{"oracle":"ajv-2020","valid":true,"cases":4,"maximumCases":4}`.

Log: `🧪️sol-writer-child-local-text-ajv-2026-08-26.txt`.

### Static formatting and source integrity

Command: `rustfmt --edition 2021 --check` over the changed store and Writer Rust sources.

Result: exit `0`. `git diff --check` also exits `0`, and exact source search finds no `WRITER_SCRATCH`, `writer_scratch`, or prior global type.

Log: `🧪️sol-writer-instance-owner-rustfmt-check-2026-08-26.txt`.

### Authoritative tool-job verifier

Command:

```text
bun ./📜️script.ts verify interactivity tool-jobs --self-test
```

Result: exit `0`, `self-tests=468 clean`.

Command:

```text
bun ./📜️script.ts verify interactivity tool-jobs --format json --output .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📊️sol-writer-instance-owner-tool-jobs-2026-08-26.json
```

Result: expected aggregate exit `1` for unrelated live inventory. The report has `globalPayloadStores=32` (prior accepted packet checkpoint `33`), `Writer` global rows `0`, exemptions unchanged at `3`, `boundedRows=217`, `remainingCommands=719`, and `selfTests=468`. Aggregate failures remain 32 other global rows, 53 scan-then-monolith routes, 35 app-owned imports, and 719 live registrations.

Evidence:

- `🧪️sol-writer-instance-owner-tool-jobs-self-test-2026-08-26.txt`
- `📊️sol-writer-instance-owner-tool-jobs-2026-08-26.json`
- `🧪️sol-writer-instance-owner-tool-jobs-stderr-2026-08-26.txt`

## Pending Runtime Gate

No Cargo or Nx command was started while the Framework executor owned the shared compiler lane. The exact queued runtime command is a focused `semio-s-plugin-writer` library test selecting `child_local_text_fixture_proves_bounded_identity_isolation_aba_and_wire_omission`, followed by the Writer native library check. Runtime acceptance is not claimed until those commands execute.
