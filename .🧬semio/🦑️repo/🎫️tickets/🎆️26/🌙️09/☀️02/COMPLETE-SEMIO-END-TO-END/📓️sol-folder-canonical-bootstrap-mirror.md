# Folder Canonical Bootstrap Mirror

## Outcome

The folder bootstrap mirror now has a Vite-process-owned durable epoch and opaque capability instead of a post-await client assertion around the generic SQLite upsert. The implementation preserves the existing pure-folder `GET|PUT /semio-backbone` behavior and requires an explicit exact-owner retirement before a mixed Hub/folder client can resume generic folder writes.

This slice is limited to the local Vite/Bun folder mirror. It does not authorize or publish a Hub checkpoint and it does not qualify the still-pending full Hub/Checkpoint native process lane.

## Contract

The folder-only sibling route `/semio-backbone/canonical-bootstrap/{reserve,stage,publish,retire}` is keyed by the existing absolute folder URI and document id.

- `reserve` transactionally increments a retained per-document epoch, mints a 32-byte server capability, retains the exact artifact schema, descriptor digest, aggregate SHA-256 and full baseline frontier, and makes any prior pair invisible.
- `stage` requires the exact epoch/capability in private headers, accepts only `application/octet-stream`, enforces the 64 MiB pair ceiling, decodes the document bundle and recomputes SHA-256 over `pack || spr` before retaining a non-current stage.
- `publish` atomically transitions only the exact current reserved owner whose stage digest matches the retained expectation.
- `retire` atomically hides and deletes only the exact current owner's stage. A stale owner cannot retire its successor.
- A published GET verifies the retained aggregate against both the staged digest and current bytes before returning the pair. Reserved and retired owners suppress fallback to an older generic row.
- After exact retirement, a generic PUT records a `generic` state without deleting the retained epoch. A later reserve remains monotonic and suppresses that generic row until explicitly retired.
- Canonical route media types are exact; reserve accepts `application/json`, stage accepts `application/octet-stream`, and bodyless publish/retire accept no content type.
- Folder path admission uses the platform `node:path` absolute-path predicate, preserving native POSIX and Windows folder bindings without accepting relative destinations.

The browser worker retains the server capability only in its private bootstrap owner. It reserves before chunk transfer, asserts ownership before and after stage/publish, retires the exact epoch on stale stage or stale publish completion, and retires before folder fallback, welcome replacement, rebootstrap, lease drop or close can expose a successor state.

## Schema and executable corpus

The language-neutral corpus is under `🧫️fixtures/📇️folder/📣️canonical-bootstrap-folder-mirror-v1`. Draft 2020-12 AJV validates the corpus. The native Bun process law uses real `bun:sqlite`; WebCrypto independently validates both SHA-256 rows.

The process law proves:

- exact A publication and read;
- B reservation hides already-published A, while stale A stage, publish and retire all conflict;
- B-before-A-publication makes late A publication conflict and never exposes A;
- exact B publication/read in both sequences;
- wrong capability, wrong epoch, aggregate mismatch, malformed framing and max-plus-one denial;
- pure-folder generic PUT/GET preservation;
- explicit published-to-retired-to-generic transition; and
- monotonic server epoch retention across the generic transition.

The worker Vitest law separately invalidates the local client owner during stage and during publish. Both cases require one exact retirement, no installed canonical pair/frontier and no retained mirror owner.

## Registered commands and evidence

Source:

```text
bun ./📜️script.ts nx run @semio-tech/framework-os-dev:canonical-bootstrap-folder-mirror-check --skip-nx-cache
canonical-bootstrap-folder-mirror: phase=source ajv=1 markers=8
exit 0
```

Native Bun/SQLite process:

```text
SEMIO_TEST_ARTIFACT_DIR=$TICKET/🗑️generated/canonical-bootstrap-folder-mirror bun ./📜️script.ts nx run @semio-tech/framework-os-dev:canonical-bootstrap-folder-mirror-process-check --skip-nx-cache
canonical-bootstrap-folder-mirror: phase=process ajv=1 sqlite=1 sha256=2 stale=7 hostile=8 mixed=1 monotonic-epoch=1
exit 0
```

Worker race regression:

```text
bun ./📜️script.ts nx run @semio-tech/framework-os:test-quick --skip-nx-cache --testNamePattern='artifact bootstrap atomic restore'
Test Files 1 passed | 2 skipped
Tests 8 passed | 274 skipped
exit 0
```

Plugin registry generation and immediate freshness:

```text
bun nx run @semio-tech/plugin-registry:generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 45 framework packages)
.vscode/launch.json regenerated
exit 0

bun nx run @semio-tech/plugin-registry:check-generated
plugin registry generated catalog and launch bytes are fresh.
exit 0
```

The generated launch entries are at `.vscode/launch.json:6819` (source) and `:6827` (process); the ticket-owned process artifact root is at `:6833`.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🔌️vite-plugins.ts`
- `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/📇️folder/📣️canonical-bootstrap-folder-mirror-v1/🧬️.schema.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/📇️folder/📣️canonical-bootstrap-folder-mirror-v1/🔣️.json`
- `.vscode/🧩️launch.seed.jsonc`
- generated `.vscode/launch.json`

## Remaining qualification

- No Chromium journey was run for this local mirror slice.
- The authenticated Hub checkpoint publication native/process targets remain unqualified while root's full Hub/Stdio native build owns the shared expensive compiler lane.
