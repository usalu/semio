# Directory Event-Page Canonical Clients

## Outcome

The TypeScript and Rust directory clients now fetch `GET /directory/event-page/v1?after=<frontier>` as bounded raw text/bytes, validate the existing strict canonical `DirectoryEventPageV1` contract, bind the response to the requested frontier, and retain only the original canonical JSON plus the authenticated page header. Neither client parses with a generic JSON response helper and reserializes it.

Global streams now also expose an explicit acknowledged-frontier mode. In this mode observed events and heartbeat heads are wakeups only: reconnects stay at the last Home-committed cursor until the page controller explicitly acknowledges a monotonic wire-safe `through` value. Existing global and document-scoped streams retain their explicit observed-frontier behavior.

The Rust client also owns a transport-neutral `DirectoryEventPageBootstrapV1` state machine. It retains at most one pending canonical page, advances only on an exact epoch/receipt/binding/generation/through acknowledgement, retries a rejection from the unchanged acknowledged cursor, coalesces live wakeups, and cannot be reused after close.

## Owned Source

- `🧰️framework/🛍️products/💻️os/🟦️.ts`
  - `CanonicalDirectoryEventPageV1`
  - `DirectoryAcknowledgedStream`
  - `DirectoryClient.eventPage`
  - `DirectoryClient.streamAcknowledged`
  - targeted inline Vitest laws
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
  - `CanonicalDirectoryEventPageV1`
  - `DirectoryClient::event_page`
  - `DirectoryClient::stream_acknowledged`
  - `DirectoryStream::acknowledge`
  - `DirectoryEventPageAckV1`
  - `DirectoryEventPageBootstrapV1`
  - exact native law
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`
  - independent AJV/Node SHA-256/canonical-byte oracle
  - exact Cargo-law runner
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc`

## TDD Evidence

The source oracle was first run after demanding `eventPage` and failed because the method did not exist. The acknowledged-frontier increment was likewise run first and failed with `TypeScript acknowledged directory frontier is missing`.

Current green evidence:

```text
bun ./📜️script.ts directory-event-page-client-check
directory-event-page-client-check: checks=11 clean
```

```text
NX_ISOLATE_PLUGINS=false NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false \
bun nx run @semio-tech/framework-os-kernel:directory-event-page-client-check --skip-nx-cache
NX Successfully ran target directory-event-page-client-check
```

```text
bun ./📜️script.ts test --testNamePattern 'DirectoryClient event page|reconnects an acknowledged stream'
Test Files 1 passed | 2 skipped (3)
Tests 2 passed | 240 skipped (242)
```

The neutral oracle validates the language-neutral fixture with AJV, checks exact `JSON.stringify` canonicality, recomputes the receipt with Node SHA-256, rejects oversize/trailing/frontier/receipt substitutions, and source-checks both language implementations without importing repository modules. This keeps the client proof independent of the concurrent unrelated Stdio taxonomy failure.

The exact native law `os_directory::client::tests::directory_event_page_preserves_canonical_bytes_bounds_and_cancels_before_io` reached discovery and passed its one selected assertion. The executable contains the final acknowledged-frontier and Rust bootstrap-owner law bytes, including forged-ACK rejection, page-to-page progression, live handoff, wakeup coalescing, same-cursor rejection retry, and terminal close.

```text
receipt: 🗑️generated/directory-event-page-client-exact/exact-cargo-laws-pofDn5/00
selected: 1
passed: 1
failed: 0
executable SHA-256: 74c30569e8740a6dbd2c14ea3c5b3acf977d52b0eac2b39587862359a762b945
```

## Remaining Boundary

This packet establishes canonical client transport and an ACK-controlled reconnect cursor. It does not claim the hub route, browser-worker fetch/ACK state machine, retained Home invocation acknowledgement, ShellHost/WGPU wiring, or a two-process gap/reconnect journey; those remain separate packets.
