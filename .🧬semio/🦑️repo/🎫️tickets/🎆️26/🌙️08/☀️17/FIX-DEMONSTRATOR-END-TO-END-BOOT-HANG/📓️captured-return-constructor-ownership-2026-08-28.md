# Captured Return Constructor Ownership

## Exact Source Finding and Test Boundary

The existing `reserveInstanceReturn` constructs its response roster and state, then mints/freezes the public captured-return facade, and only afterward assigns `activation.returned` and `state.facade`. A finalizer throw therefore loses the exact parent registration and permits a second reservation. This is an existing local ownership gap, independent of the unreleased live binary inbox.

The canonical captured-return fixture/schema declare ten facade-finalization cases: before/after actual Object.freeze crossed with null, undefined, false, zero and an unread object holding8193 bytes. The test requires original parent installation before the callback, exact original facade and first raw fault afterward, no replacement reservation or worker dispatch, and no retirement claim. TypeScript AST independently checks the actual state inventory; Immer provides a separate state-transition oracle. Strict Ajv validates the shared neutral fixture.

The correction installs the existing state in the original activation before facade construction, installs the exact facade within its constructor before finalization, and retains the raw thrown value without reading getters. This adds one actual fault field to the captured state and must be counted before future metadata admission.

## Executed Red and Green

The exact focused command is `bun x nx run @semio-tech/framework-actor:test --skip-nx-cache -- --testNamePattern=CapturedReturnConstruction`, with Nx daemon/graph-cache/isolation disabled for the actual run.

Before implementation it failed0PASS/1FAIL/147skip148,1.13s,start00:55:50: original parent registration was false at the actual finalizer. After the narrow source repair it passed1/147skip148,2.66s,start00:56:30. That group executes all ten combinations and the source-inventory oracle. Logs: `🧪️captured-return-constructor-red-1.log` and `🧪️captured-return-constructor-green-1.log`.

Post-run hashes (not represented as pre/post stable): Shard `3b7bd406195eb3ba360d0b31f4bc14ba4cccf3f282fdd9e78219380a5af65b22`; captured-return fixture `8b7c2a037b1b0b8d545010653e033a7cc7ea1fc4c42b71aa29d8c88d212988c7`; fixture schema `d306da05c4b9a2b4a4bcbbbbd1601250247cc8dab4a4da0f2e54ef42542b9e3f`. Canonical launch registration400.97 was initially queued. Taxonomy subsequently reported completion, and this lane directly verified both authored seed and generated launch contain the exact `⚖️gate🎭️actor📤️construction` command at4_gate/400.97. Direct SHA256 readback matches seed `992b54f878c056aed66c8a3e23542d58be62e60c9af998fceb786e4f270d71bf` and launch `8334bed743b37d9b259c79ff3dd091ecbadf955519ff0e6ca0977fe75ee8d493`. Registration did not execute the test or publish any plugin/WGPU output; the executed RED/GREEN above remains its separate evidence.

This packet does not certify response-roster constructor allocation, pre-admission memory, all Shard construction, whole-result retirement, native InputAck, worker memory or live guest content. It introduces no alternate response layout, neutral API, compatibility path or quota increase. The original first fault remains held; no terminal witness or final refund follows from ordinary dispose.
