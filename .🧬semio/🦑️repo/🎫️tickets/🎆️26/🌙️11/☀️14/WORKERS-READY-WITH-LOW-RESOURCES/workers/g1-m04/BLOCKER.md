# g1-m04: What Blocks This Worker (for the coordinator; state 2026-11-14 23:45 UTC)

Rounds 3, 5, 7 and 8: 36 tries, each `native_rc=75`, each "the fleet was asked for 516 (517) and delivered 0". No memory problem, no hard stop, nothing else fails, because `workspace:setup` never gets past the graph step. The fixes of rounds 6 to 10 concern macOS; none changes this.

Probable cause, read in the code and not changed: `📚️library/🏗️build/🪪️graph-state/🙋️wanted/🟦️.ts`. A request is stored per requester **and platform** (`wantedStateV1`), and a serving machine reads the other machines' requests with **its own** platform (`claimWantedV1` → `readWantedV1({…, platform: input.platform})`, called in `🛠️server/🟦️.ts`) and only for its own `graphKey`. g1-m04 is the Linux worker (SteamOS, 16 GB); the group server and the other workers are macOS. Nobody reads the slot g1-m04 writes to. The sidecar here logs `graph-server: alive, 0 machines seen, 0 requests held, read none`.

On its own the machine cannot do it either: `516 stale projects, 0 taken from the fleet, 516 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), …)`, `this machine can infer 0 (0 per wave)`.

What would unblock it (the coordinator's choice): the group server serving requests of other platforms (if project records do not depend on the platform, or by inferring for the requested platform), or a Linux machine with enough memory in the group, or a bounded local inference for a 16 GB Linux worker.

Evidence: `round-7.md` (addendum), `round-8.md`, `round-*.service.txt`.
