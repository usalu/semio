# Browser Directory Event-Page Bootstrap Worker

## Outcome

The browser backbone worker now owns a bounded directory bootstrap protocol instead of requiring the UI thread to treat socket observations as projection authority. The protocol serializes one page fetch, one exact retained-Home acknowledgement, the next page, and finally an acknowledged-cursor live socket.

The worker retains only one canonical page at a time. An ACK must match the page receipt, session binding, authorization generation, through frontier, and bootstrap epoch. A rejected Home action releases that page and retries the same acknowledged cursor with bounded jitter. Cancellation clears the pending fetch, timer, page, and socket. Events and heartbeat heads wake a new page fetch but do not advance the reconnect cursor; `rebootstrap-required` resets the fetch cursor to zero. Non-projection presence/connection messages remain deliverable.

The event-page broker boundary admits only exact `GET /_semio/hub/directory/event-page/v1?after=<canonical-safe-decimal>` requests. Bootstrap requests are explicitly kept on the TypeScript worker owner even when the Rust WASM worker is available.

## Schema and Protocol

- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧬️event-page-bootstrap-v1.schema.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️event-page-bootstrap-v1.json`
- `DirectoryEventPageAckV1` and exact `BackboneWorkerRequest`/`BackboneWorkerResponse` variants in `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `DirectoryEventPageBootstrapV1` and its fetch/retry/live owner in `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts`

No message contains a bearer, socket grant, raw session id, or invite secret.

## TDD and Verification

The independent bootstrap gate was run before implementation and failed exactly with `browser worker bootstrap owner missing`.

Current evidence:

```text
bun ./📜️script.ts directory-event-page-bootstrap-check
directory-event-page-bootstrap-check: checks=11 clean
```

```text
bun nx run @semio-tech/framework-os-kernel:directory-event-page-bootstrap-check --skip-nx-cache
NX Successfully ran target directory-event-page-bootstrap-check
```

The independent AJV/plain-state-machine oracle proves page-two-before-ACK denial, forged receipt denial, stale epoch denial, exact cursor advancement, and wakeups that cannot alter the committed socket cursor.

```text
bun ./📜️script.ts test quick
Test Files 3 passed (3)
Tests 245 passed (245)
```

The Vitest laws cover the same state transitions with the production class, request/response codec round-trips, secret absence, canonical-safe-decimal allowlisting, and TS-worker ownership. JSON parsing and owned diff hygiene are green. Plugin-registry generation and `check-generated` are green after adding launch order `411.052`.

## Permanent Gate

- `@semio-tech/framework-os-kernel:directory-event-page-bootstrap-check`
- `⚖️gate🧭️directory-event-page-bootstrap-worker`

## Honest Boundary

This packet does not claim that ShellHost currently invokes the retained Home action or sends the ACK, that the WGPU shell has the twin state machine, or that the hub route is live. Until those packets land, the existing shell startup still uses its legacy `directory-open` request. The worker protocol is ready for that consumer migration without claiming the end-to-end journey early.
