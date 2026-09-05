# Directory Ordered Publication Packet

## Outcome

`DirectoryService` now keeps durable append and synchronous live broadcast inside the same single-writer guard lifetime. A later committed sequence therefore cannot overtake an earlier committed sequence on the live directory channel.

The repair covers the ordinary command pipeline, administrator space creation, artifact-authority events, reserved checkpoint publication, and invite redemption. Non-event presence/connection messages remain outside the event-sequence writer by design.

## Contract and implementation

- Language-neutral fixture and strict schema: `🌎️hub/📇️directory/🧫️fixtures/📣️ordered-append-broadcast-v1/`.
- Independent Bun/AJV and live-source oracle: `directory-ordered-publication-check`.
- `DirectoryService::append_and_publish_locked` awaits the event-store append and broadcasts the returned page while borrowing the live `HubClock` mutex guard.
- `DirectoryService::publish_persisted_locked` covers atomic reservation publication, whose store API already performs append and projection work together.
- A test-only one-shot post-append fence creates the formerly vulnerable schedule deterministically: writer one is durably committed but paused before broadcast while writer two attempts a role update.
- The native law requires no broadcast during the fence, durable and live sequence `[since + 1, since + 2]`, and a final member projection matching the second event.

## TDD and verification

| Check | Result |
|---|---|
| Intended source RED before production repair | RED: append and broadcast did not share one writer-guard lifetime |
| `bun ./🌎️hub/📦️packages/🦀️rust/📜️script.ts directory-ordered-publication-check` | GREEN, 7 checks |
| Rust 2021 parser via pinned nightly rustfmt | GREEN |
| Scoped diff hygiene | GREEN |
| Exact native concurrency law | In progress under a live exact-Cargo lease |
| Plugin-registry generated launch freshness | GREEN |

## Executable surfaces

- Nx source target: `os-hub:directory-ordered-publication-check`.
- Nx exact native target: `os-hub:directory-ordered-publication-native-check`.
- Both targets are registered in `.vscode/🧩️launch.seed.jsonc`; `.vscode/launch.json` was regenerated from that source.

