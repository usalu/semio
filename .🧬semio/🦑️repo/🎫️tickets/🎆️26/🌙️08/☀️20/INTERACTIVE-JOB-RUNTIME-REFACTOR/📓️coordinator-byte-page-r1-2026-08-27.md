# Coordinator Actor Byte Page R1 — 2026-08-27

## Outcome

Actual independent focused run: **3 passed, 95 skipped, 98 total**, one file passed and six skipped, exit 0. Start 20:02:36; Vitest duration 716 ms. This is neutral TypeScript storage proof, not native/WIT integration, returned-page authority, hostile-object validation, immutable-input ownership, heap-retirement proof, or an 8 ms certificate.

The coordinator read the complete production schema, fixture schema, fixture, and TypeScript implementation before this run. All four captured source hashes were identical before and after.

## Command and observed footer

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-actor:test --skip-nx-cache --args='--run -t ActorBytePage'
```

```text
Test Files  1 passed | 6 skipped (7)
     Tests  3 passed | 95 skipped (98)
  Start at  20:02:36
  Duration  716ms (transform 1.25s, setup 0ms, import 1.76s, tests 193ms, environment 24ms)
NX Successfully ran target test for project @semio-tech/framework-actor
```

The original tool output also contained repeated Bun NO_COLOR/FORCE_COLOR warnings; only the actual outcome footer is reproduced here.

## Verified coverage and limits

The executed tests compare all 512 little-endian words against Node Buffer at ten lengths (0, 1, 7, 8, 9, 63, 64, 65, 4095, 4096), validate the shared fixture and canonical decimal-u64 JSON using strict Ajv, and reject malformed selected fields and nonzero tail bytes. Additional/foreign property validation is test-only. Production reads only the fixed own data fields and allocates one payload of at most 4096 bytes.

The selected-field test retains an 8192-byte unknown wrapper and verifies that neither own-key enumeration nor the unknown getter runs. This does not prove unknown-wrapper retirement. Object.getOwnPropertyDescriptor can invoke a Proxy trap; mutable/shared typed-array views do not become immutable simply because their copy fits one page. Captured producer provenance and retained input/output ownership must be established by the transport boundary before these helpers are used interactively.

Peer owns the authored command conversion to {cursor, page}. Its separate reported full actor 98/98 is not substituted for this coordinator's focused result. Dag owns canonical schema, Rust, WIT and the distinct return authority. No command opcode, factory witness or eager input-page array is reused as return authority.

## Stable SHA-256

```text
08732c8b215162a04e546d4c935f842814aeeba07bc2ad664fb64f9e5c894611  🧰️framework/🔨️modules/🎭️actor/📄️page/🧬️schema.json
9458d1d6b94083f008f49a7c1c72c53764bb695d95f46d224a9d66fdd00fc692  🧰️framework/🔨️modules/🎭️actor/📄️page/🧪️schema.json
a6398f9680b44ffca75890d84db3216a36405dbe1a0952ad9952db5da514e62b  🧰️framework/🔨️modules/🎭️actor/📄️page/🧪️fixture.json
06aa8d36e8643c11dbe65e9a89eae0e48d44b450a5d3e19b2041345f6788f515  🧰️framework/🔨️modules/🎭️actor/📄️page/🟦️component.ts
```

No production source, generated output, cache or existing evidence was modified or deleted by this review.

