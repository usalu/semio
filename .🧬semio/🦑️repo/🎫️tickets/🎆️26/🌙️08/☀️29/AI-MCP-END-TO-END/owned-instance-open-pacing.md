# Owned Startup Pacing

The native SDK call against candidate Note `794785f1e15990754b2cbf2a3e91b7c8567b3f5a75c2818944d83490fe9665f8` reached `waiting for capture` after 212.564 s. The last passive gateway counters were Poll session stage 0, header extent 144 and 1,023,980,902 cumulative VM instructions. Stage 0 is shared by several receipt/dispatch paths. That message identifies the startup phase, and cannot identify the exact in-flight guest export, completed turn count, or per-turn elapsed clock observations by itself. It averages 4.82 million recorded VM instructions per SDK elapsed second. The earlier standalone scalar replay of the original matched Open Input alone measured roughly 33.9 million instructions per second. Its receiver, accounting and timed region differ from the SDK stopwatch.

A second policy replay ended in the same startup phase at 212.472 s, with 992,741,956 cumulative VM instructions and a frozen full Poll header in stage 10, `ClaimOrigin`, row 0 of 32. The wall limit can therefore expire while crossing real guest output escrow. The paced-opening experiment addresses a structural multiplier and leaves receipt capture and settlement costs intact.

The clock used for the reactor's 8 ms quiet cutoff is real elapsed time. `with_turn_execution` measures guest future polls via `default_now_us` and WASI `Instant`. The owned runtime answers WASI monotonic time using host process `Instant::elapsed`. A sliced VM remains within the same guest `Future::poll` across native scalar bookkeeping and scheduling, so that timing includes interpreter and host suspension overhead between guest clock reads. `original_job_clock` separately enters through authenticated `operation-clock-us`; Source `should_yield` observes that budget and deadline. WASI monotonic clock and Source operation clock are distinct imports and both are real in the deployed host.

## Candidate Clock Sensitivity

`note-poll-bun.ts --tuple-limits --lifecycle-turns --monotonic-scale=N` was run against the exact same candidate bytes. The diagnostic multiplies only the WASI monotonic replies. The Source operation clock, fixture wire, fixed loans, escrow handbacks and cancellation rules are unchanged. The normal mode uses scale 1. This is a controlled pacing experiment, with no claim that the native host uses any listed multiplier.

| Diagnostic clock scale | Captured turn | ACK turn | Total kernel turns |
| --- | ---: | ---: | ---: |
| 1 | 3 | 4 | 5 |
| 8 | 3 | 4 | 5 |
| 32 | 7 | 8 | 9 |
| 64 | 17 | 18 | 19 |
| 128 | 36 | 37 | 38 |
| 256 | — | — | Guest returned a failure header before a valid lifecycle output |

The successful sessions settle the actual matching Captured receipt and a successful MoreWork ACK with no pending input. Faster virtual elapsed time alone reproduces nearly the original 42-turn pacing. The candidate's empty Input specialization separately reduced the isolated scalar input admission from 107,438,390 to 368,119 instructions, so sustained per-turn cost can still accrue in Poll output encoding, rotation, escrow and close. The standalone lifecycle takes 2,913 guest epochs even when startup needs five kernel turns.

## Scope and Boundaries

The MCP `GuestPollSession` uses the 19-argument Source-bound `semio_owned_poll_v1` export. The WIT bridge has its own interactive adapter through `poll_kernel_output`. Browser calls use that WIT bridge. Native renderer code has an owned worker-pool runtime, and its general `execute_actor_turn` path uses a separate operation adapter with the pointer/length export invocation. No dynamic deadline, clock binding, request identity, numerical capability grant or Ordinary event purpose identifies headless startup. The egress action calls now select an internal pacing policy at the Rust entry boundary while retaining the existing scalar and WIT wire contracts.

The standalone neutral tuple requests 48 million items, 3 billion copy bytes, 2 GiB capacity and release, depth 64, 1 billion total guest work and one million per turn. The authored MCP editor request instead starts at 1,048,576 items, 256 MiB copy, 128 MiB capacity and release, depth 64, and 100 million work units. The effective native tuple intersects that request with the host policy and authentic scoped loans. The selected Source deadline also incorporates issuance and the current operation. The standalone diagnostic's broad fixed scopes make it suitable for a within-engine clock experiment. A direct native phase sample is needed to associate the live SDK result with specific subloans or clock readings.

The original WIT/native interactive path retains 8 ms pacing. A Source-bound owned Poll can take a second opening step after that soft cutoff if the same opening remains pending, an original progress/work observer advanced, and the caller is neither cancelled nor yielding. It still requires a usable real executing clock below the strict lifecycle ceiling, which `finish_turn` independently verifies. The 64-step gate, real Source deadline, actual retained/work authority and host VM instruction pacing remain. The helper selects only the same opening, leaves publication after the loop, and publishes at most its one original Captured receipt. Other turn phases use their existing quiet pacing.

The shared neutral fixtures cover both execution paths, paid progress, a parked action, one capture, cancellation, Source yield, a lost clock and the real lifecycle ceiling. No guest rebuild was run for this adjustment while the shared build target was held.
