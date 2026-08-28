# Coordinator Actor R16

Independent root execution on 2026-08-27 completed with exit 0: **106/106 tests, eight passing files**, start 20:47:21, duration 7.54 s. All five explicitly captured source/schema/fixture hashes matched before and after execution. The peer codec/Shard hold was released after this check.

## Scope

This verifies the current actor TypeScript suite including the canonical protocolFault refinement, fixed returned-result codec, existing lifecycle and issued-UI codecs and captured transport fixtures. It does not establish executed native preadmission, a mounted retained return owner, fresh component-Wasm behavior, final raw-output retirement or the 8 ms ceiling. New Kernel content framing and UI resident-pool work is outside this actor run.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-actor:test --skip-nx-cache
```

## Identical Before and After SHA-256

```text
836fe0351b67f1a86e953b5c41cb526fb67e1ef99090f377f43c714893751191  🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️component.ts
a1039d317bb150607bad46902d7891e61a49ea8ae24abd7236396460783b2033  🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema.json
3328697d8ed6e7e8c3d939c5213ea276d7075aaa029b75e620258869ade72fff  🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️schema.json
ed2c9f97b5abdb39963969d13684e68701ea705d03adc1f7823ac0c25c3aa1e7  🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️fixture.json
b1a16ae654a7dcbdfa08f5b8807b0f0a31b80388c8c0fedd0fad6d833e540855  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
```

## Observed Completion

The terminal tool output was truncated in its repeated color-warning preamble; the following complete completion footer was observed. This report does not claim to preserve the full raw log.

```text
Test Files 8 passed (8)
Tests 106 passed (106)
Start at 20:47:21
Duration 7.54s (transform 10.84s, setup 0ms, import 7.91s, tests 12.68s, environment 18ms)
NX Successfully ran target test for project @semio-tech/framework-actor
exit_code: 0
```

