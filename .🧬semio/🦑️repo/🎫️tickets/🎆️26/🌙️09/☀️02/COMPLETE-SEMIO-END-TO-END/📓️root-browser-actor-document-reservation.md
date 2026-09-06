# Private Document Browser Actor Reservation

## Scope And Current Outcome

The new module-private Backbone reservation is prepared, but is not yet invoked by the production actor-loading path. Successful production socket-grant exchange now privately binds the verified lease to the authenticated actor identity and grant/plan deadlines. Renderer-unavailable remains the reported state; no browser actor body route, fetch, load, invocation, WASI bridge or real GIS renderer is claimed.

The private helper accepts only a current ArtifactState. It requires the exact live document lease, matching Hub origin/document/space/schema, an exchanged unexpired private grant and an empty per-document reservation slot. It derives actor hash and length from the lease, actor identity from the parsed grant, and an increasing u64 generation from the private owner. The slot is claimed before asynchronous child boot. The owner rechecks lease/state/slot/deadline after boot. Document close, lease drop, plan replacement, failure and plan expiry terminate/release the child. Lease retirement notifies the child before wiping component/descriptor bytes.

The lower child still owns the actual static Worker, private MessagePort, admission capacity, deadlines and transfer protocol. Reservation does not transfer broker proof, grant secret or Hub URL to the child.

## Executed Evidence

- Session 95687: expected TDD RED, the new positive closed-actor reservation row failed before the helper existed; three prior lease/renderer tests passed.
- Session 60192: registered `os-hub:browser-actor-document-reservation-check` GREEN. AJV validates the neutral fixture. Four selected Vitest tests passed (273 skipped), including 12 reservation rows and four retirement/isolation laws. No actor load is attempted.
- The reservation suite uses real MessageChannels with a mocked Worker factory. It qualifies document ownership logic, not browser process execution or authenticated Hub integration.
- Separate prior session 54763: real Chromium lower child containment GREEN for 36 checks and focused strict TypeScript. That evidence must not be conflated with the mocked document reservation gate.
- Scoped `git diff --check` GREEN after implementation.

## Remaining Work

Strengthen cross-boundary coverage with an actual actor-body acquisition/activation owner and real catalog-produced GIS component; implement the typed Semio/WASI host bridge, document Store owner, durable mutation/undo and cold authenticated MCP Map acceptance. The end-to-end goal remains active.

## Expanded Qualification

Session16883 was expected TDD RED: a changed requested surface still reserved a child. The helper now rechecks requested surface and any installed target against the exact lease. Session58528 GREEN: AJV1, sixteen reservation rows, seven lifecycle laws and all four selected Vitest tests (273 skipped). The added laws observe grant expiry during boot, actual plan-expiry retirement, authenticated actor identity, private/single-use admission, origin/schema/surface denials, real attempted-fetch count0, load count0 and final reserved capacity0/0. Worker remains mocked.

Root real materialization session9773 is active in the separate `🗑️generated/trusted-gis-real-activation` artifact directory. It runs the registered Stdio+GIS native generation/candidate gate with its own hub-target. This is not a real child activation result yet.

## Child-Local WASI Prerequisite

- Session89577: expected TDD RED when the real Chromium fixture attempted the absent child-local WASI clock.
- Session53435: GREEN40, with18 base laws +13 hostile wire laws +9 owner faults, strict production TypeScript and AJV. The four new laws cover monotonic child-local clock, observed synchronous stdout/stderr before caller byte mutation, output max+1 rejection and WASI exit terminating the child and settling the caller.
- The child now provides only a local Preview2 host port to its generated activation: performance-based monotonic nanoseconds, bounded synchronous console output (64KiB total/128 calls) and exit-to-retirement. Semio host effects, document/network/storage access, pure log/trace and effect completions remain denied. The generated first-party WASI runtime still owns its own Preview2 resources and JSPI checks.
- Limits are now also explicit in the neutral schema/TypeScript contract; follow-up91744 is GREEN40 on that exact source. This is port qualification with a fixture, not genuine GIS describe/renderer qualification. Real materialization9773 remains BUILD active in its private target.

## Native Receipt Correction

Session9773 ended BUILD RED before materialization. Retry27152 is still compiling in the same root-owned cache with bounded exact JSON output capture. Neither run establishes candidate or GIS activation success.
