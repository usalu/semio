# Component staging writer order

The Energy release run wKfdy4s requested an exclusive artifact capture at rank 6 while the shared repository compiler writer was held at rank 7. The original Energy output reported it at line 4267 shortly before the Cargo pass reported a successful link. The completed BIM run wVg9WoJ reported the same acquisition at line 3140 of the BIM output. BIM also reported an independent Cargo configuration-view request at rank 1 while the registry reader was held at rank 2. The configuration request occurs before linking and is outside the component-staging path.

The repository plugin driver keeps the compiler writer around compiler capacity and streaming stage work. The native driver originally called general artifact publication from inside that scope. General publication acquires two independent writers, for source capture and destination replacement, both at rank 6. Waiting for Cargo exit still leaves streaming staging inside the enclosing writer scope.

The first correction moved both transactions before the compiler scope. The subsequent Energy run wEUVfRw showed that this made the compiler request the rank-3 host-capacity gate while holding a rank-6 artifact guard. The actual host-capacity gate retains physical capacity custody. The lock policy requires waiting for the compiler writer before capacity and forbids waiting in the reverse direction.

The native driver now runs each linked component's publication as an independently joined sibling of compilation. The Cargo branch holds the shared compiler writer before waiting for capacity. The staging branch takes the two exclusive transactions for its exact generation-private destination, then takes an immediate shared image writer through the live compiler workspace reservation. The stage has no CPU-capacity custody and finishes while the original compiler writer remains active. Its exclusive transactions release after the stage writer returns and before provenance or downstream consumers proceed. The same artifact snapshot, ownership marker, physical identity, commit and rollback checks still run. The policy and general shared publication route retain their existing behavior.

The active Fleet One Machine Shared Hot Cache and Remote Execution ticket also covers compiler and stage storage, but its records did not show a change to this local staging acquisition.

## Verification

- The actual streaming driver passed using stub Cargo for linked WASM and the genuine shared compiler-writer scope. It verified the real host-capacity lease was held by the compiler sibling as all four components from two profiles staged. The compiler branch had no artifact guards and the stage branch held both exclusive artifact guards before independently taking its image writer. No artifact-capture or host-capacity ordering diagnostic was emitted. The test ran Cargo metadata and fixture lockfile preparation but compiled no release modules.
- The source/publication suite passed all 23 tests. A language-neutral Wasm vector uses an independent SQLite connection to check that both exclusive transactions are held before the compiler writer and released after the stage writer callback returns. SQLite independently checks the exact staged bytes.
- The five independent CPU-last compiler-writer laws passed with the pinned Python oracle.

## Original stage proof after image custody

Procedural w51P2HP linked all 1,207 units and Cargo exited 0. The older staging path requested a destination proof using a control that differed from the original proof holder. Its first refusal was the live-writer check in the destination publication entry, before source capture or downstream receipt creation.

The streaming regression also exposed this failure when the stage received the image writer's refined control. The current stage opens both artifact writers using the original I/O stage control and invokes publication through a zero-argument callback closing over that same object. The image reservation's writer and borrowing controls remain internal to its own scope. Production supplies the same workspace stage writer that the native test uses. The real fixture passed with 42 assertions across four streamed components and reported zero ordering warnings. The observed Procedural process had loaded the earlier implementation. A fresh release process is needed to exercise the corrected route with a full component.
