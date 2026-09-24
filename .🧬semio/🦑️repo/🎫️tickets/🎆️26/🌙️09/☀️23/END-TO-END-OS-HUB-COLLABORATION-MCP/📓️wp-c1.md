# WP-C1 — Local-bootstrap AJV + Two-Client Document E2E

## Headline

G3 AJV `blockedBy` strictness restored without relaxing required; G1 language-agnostic two-client document scenario fixture with TS live e2e (HUB_E2E) and Rust fixture twin.

## Status (filling)

| item | state |
|---|---|
| G3 local-bootstrap schema | DONE |
| Fixture + schema | DONE |
| TS runner | DONE (+ PR1 join-replay) |
| Rust fixture twin | DONE (cargo filtered green) |
| Live HUB_E2E | QUEUED on hub fleet mutex |
| os-hub / os-hub-ts test-quick | PENDING after e2e |
| launch.json | DONE (two-client entries present) |

## Protocol coverage (wire-level C9 / PR1 / HT16 / HC1)

- HC1: trusted-catalog clone + note artifact creation
- PR1: auth → open-plan → socket-grants → WS → Hello; presence beat A→B; **join-replay** (A beats before B attaches)
- HT16: Welcome/Ack `document_id` is plain artifact id
- C9 wire: A Commands → B identical envelope; reconnect/resume; hub restart preserves frontier
- Explicit nonclaims: plugin-ui-mount, uiPatch.receipt, browser-actor

## Compile fix landed on the way

`trusted-catalog` called `codec_*_observed` on `Arc<OwnedRuntime>`. Methods with `impl FnMut` are not found through `Deref`, so rustc suggested the non-observed twin. Fixed with `.as_ref()` at both call sites (`pack_schema_hash_observed`, `genesis_observed`).

## Captures

- `.tmp-ticket/wp-c1/generated/e2e-two-client.txt` (live)
- `.tmp-ticket/wp-c1/cargo-test2.log` (fixture twin)
