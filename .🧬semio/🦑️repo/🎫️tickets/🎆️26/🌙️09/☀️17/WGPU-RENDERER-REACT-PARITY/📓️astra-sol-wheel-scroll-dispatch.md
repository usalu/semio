# Owned Scroll Dispatch

## Contract

The former `wheel-application-point/v1` fixture described a renderer accumulator with eight slots, same-point coalescing, opposite-sign cancellation, and saturated tail merging. Those behaviors erase browser event identity and contradict Canvas camera parity, where every admitted wheel applies its own `1.1` or `0.9` factor in order.

`framework.wgpu.wheel-application-point/v2` now defines one ordered owned dispatch for every admitted Scroll. Each dispatch preserves the source event's `x`, `y`, `deltaX`, vertical `delta`, and `shift`/`ctrl`/`meta`/`alt` snapshot. Pointer moves publish no Scroll and cannot relocate an existing dispatch.

The strict Draft 2020-12 schema is:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🖱️wheel-application-point/🔣️.json`

The neutral fixture retains the three travelling-pointer counterexamples and adds exact stationary-burst, opposite-sign, horizontal-delta, modifier-snapshot, and pointer-only cases:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🖱️wheel-application-point/🔣️.json`

## Independent browser oracle

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖱️wheel-application-point/🟦️.ts` mounts a real jsdom `canvas`, registers a browser `wheel` event listener, dispatches real `WheelEvent` and `MouseEvent` instances, and records a fresh owned value per wheel callback. It does not implement or import an accumulator.

The oracle validates the neutral JSON through Ajv 2020 in strict mode, requires dispatch count to equal source Scroll count, requires distinct owned application objects, compares every field, and reconstructs the superseded relocate-and-aggregate shape only to prove the travelling counterexamples differ.

## Verification

Both JSON files parse successfully with `python3 -m json.tool`.

The canonical no-Cargo Nx/Vitest attempt was:

```text
bun nx exec workspace -- bun x vitest run --config 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts 🧪️tests/🖱️wheel-application-point/🟦️.ts
```

Nx refused before Vitest because the current shared project graph contains a circular dependency:

```text
@semio-tech/framework-actor-rs
→ @semio-tech/framework-replication-rs
→ @semio-tech/value-derive-rs
→ @semio-tech/framework-os-kernel
→ @semio-tech/value-derive-rs
```

Receipt: `🗑️generated/astra-runtime/wheel-scroll-jsdom/run.log`. No test result is claimed from that infrastructure failure.

The canonical project-owned retry used `@semio-tech/framework-renderer-wgpu:test-browser-worker` with the fixture test path. It passed all 9 selected files and 120 tests in 3.77 seconds (Nx 13.2 seconds). Receipt: `🗑️generated/astra-runtime/wheel-scroll-jsdom/run-3.log`.
