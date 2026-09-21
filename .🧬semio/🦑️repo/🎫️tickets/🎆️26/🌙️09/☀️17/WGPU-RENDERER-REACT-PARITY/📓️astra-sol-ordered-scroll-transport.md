# Ordered Scroll Transport

## Scope

The host receives one `DispatchEvent::Scroll` for each browser or native wheel callback, but `EventQueue` currently accumulates every pending Scroll into one replaceable `ScrollSample`. The accumulator adds deltas while replacing position, modifier state, and generation. `drain_page` also returns that combined sample before the bounded discrete page, so wheel multiplicity and order relative to key or pointer input are lost before application dispatch.

This packet keeps the existing fixed 256-event bound, four-event drain page, 4 KiB per-event owned-byte ceiling, and explicit `Overflow`. Scroll owns no variable bytes and must use that existing bounded admission. Pointer move remains replaceable, with its retained final sample ordered at its final ingress generation. Metrics remains replaceable frame state.

The renderer's `AppWheel` removal and direct per-event application are separately owned by the root executor. Host production remained unchanged through the fail-first registration and fresh WGPU activation.

## Neutral contract

The closed version-1 schema and fixture live under `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎡️ordered-scroll`.

The fixture proves:

- same-position Scroll events keep separate deltas and `ctrl`/`shift` snapshots;
- opposite-sign Scroll events remain two events rather than one zero sum;
- Scroll and key input retain ingress order while only the latest pointer move remains at its own final sequence position;
- a Scroll after 256 admitted discrete events returns `Overflow` without changing the admitted queue or generation.

The neutral oracle validates the fixture with Ajv and independently derives the retained queue sequence. It also demonstrates that a schema-valid merged output is rejected by the semantic oracle.

Receipt:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/ui-host-rs:test-source' --skip-nx-cache -- ordered-scroll
[DEBUG] ordered Scroll oracle: 4 sequences preserve physical wheels through fixed capacity 256
PASS, Nx 2.2 s, cache skipped
```

The UI-host source script now accepts the explicit `input-admission` or `ordered-scroll` scope while retaining the unscoped aggregate. This isolates the new contract from the pre-existing input-admission schema-reference failure without suppressing either oracle.

## Third-party DOM oracle

The existing WGPU browser-input-wire Vitest/JSDOM suite dispatches real `WheelEvent`, `MouseEvent`, and `KeyboardEvent` objects for the same fixture. It observes exact event count, order, coordinates, deltas, and modifier snapshots.

Receipt:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-wgpu:test-browser-worker' --skip-nx-cache -- '🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts' --silent=false --reporter=verbose
9 files PASS, 139 tests PASS, new ordered-wheel DOM law PASS
Vitest 4.43 s; Nx 15.7 s; cache skipped
```

The older browser-wire assertion that classifies wheel callbacks on a coalescing browser lane describes the page/worker wire boundary. The host queue contract here begins after decoding to `DispatchEvent`; renderer direct-dispatch work must update that older assertion only if its owned wire boundary changes.

## Native fail-first registration

The existing `enqueue-unit` module has four physical-event laws:

```text
same_point_scrolls_keep_event_local_deltas_and_modifiers
opposite_scrolls_remain_two_ordered_events
scroll_key_and_replaceable_pointer_keep_ingress_sequence
scroll_overflow_refuses_without_mutating_the_admitted_queue
```

Target: `@semio-tech/ui-host-rs:test`, crate `semio-framework-ui-host`.

Expected production failures before repair:

- the first two laws find one coalesced Scroll sample instead of two discrete events;
- the interleaving law finds Scroll outside the ordered page and ahead of discrete input;
- the overflow law accepts Scroll into the independent coalescer after the 256-event queue is full.

Root's fail-first host gate ran the four laws against unchanged production: 4 run, 0 passed, 4 failed, 78 outside the filter, 43 ms test time and 25.7 s Nx time. The receipt is `host-ordered-scroll-red2/run.log`.

The expanded fourth neutral sequence crosses the fixed four-item drain boundary. Its final pointer sample is generation eight, after six ordered events. The corresponding native law is:

```text
retained_pointer_waits_for_every_older_ordered_page
```

The law requires the first page to withhold that pointer while older ordered input remains queued. This prevents a cursor from generation-sorting one page locally and still dispatching the retained pointer before older events on the following page. Its distinct fail-first gate ran 1 test, passed 0, failed 1, left 82 outside the filter, and finished in 42 ms test time and 24.6 s Nx time. The failure reported the older ordered page leapfrog.

Rust parse and scoped diff checks pass.

## Production boundary after RED

The host repair removes Scroll from `CoalesceSlot`, admits each `DispatchEvent::Scroll` through the existing bounded discrete queue, and preserves its original generation. `drain_page` withholds the retained pointer while the next queued ordered event has an older generation. Once the pointer can enter a page without overtaking an older future page, `RuntimeDispatchCursor` merges it with that page's ordered events by generation.

`ScrollSample` and the obsolete `DrainedEvents.scroll` slot were removed from all Rust call sites. The winit drain receipt now names `ordered` count rather than a coalesced Scroll bit. The fixed capacities and owned-byte accounting did not change. No additional queue, unbounded allocation, delta merge, silent drop, or fallback ordering path was added.

Production source checks before the Cargo gate:

```text
rustfmt --edition 2021 --emit stdout: PASS for host enqueue/tests, renderer cursor/test, and winit host
scoped git diff --check: PASS
obsolete ScrollSample/DrainedEvents.scroll reference sweep: zero matches
```

Root owns the host and renderer Cargo confirmation; no post-repair native green is claimed yet.
