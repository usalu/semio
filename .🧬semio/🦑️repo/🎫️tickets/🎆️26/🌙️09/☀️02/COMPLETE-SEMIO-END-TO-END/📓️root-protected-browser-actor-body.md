# Protected Browser Actor Body Delivery

## Implementation

The execution-target asset vocabulary now has four closed members: manifest, component, descriptor and browser-actor. The new Hub POST handler delegates to the existing document selection path: authenticated session/share, exact scope/open intent, durable descriptor, subject role, current trusted catalog selection, actor/descriptor/component binding, final subject and directory revision fences. It returns only the current selection's retained actor Arc bytes. None or missing actor bytes fail closed. No package, digest, generation, path, receipt or arbitrary URL selector is accepted.

The local browser relay admits the same closed member, uses the actor's schema-owned64MiB maximum, the existing two-request admission and9-second relay deadline, private proof ratchet and bounded cancellation. Backbone's private asset union/whitelist now recognizes the path, but no production actor-body request or child load has yet been wired.

The native HTTP law now checks exact actor bytes, a matching actor SHA in the synthetic server fixture, actor None denial, manifest/plan identity and redaction, plus anonymous, foreign-scope/surface, query/unknown selector and oversized-request denials for all four assets. This is authored, not yet natively executed.

## Executed Evidence

- Session19367: expected TDD RED; the neutral schema admitted browser-actor while production relay policy rejected it.
- Session59100: `os-hub:execution-target-relay-check` GREEN,21 route vectors,8 response vectors,38 checks, real relay requests including bytes above the generic cap and actor64MiB+1 denial, two-request capacity and cancellation, existing browser proof-ratchet regressions.
- Session20781 adds explicit schema-owned actor64MiB limit and an exact64MiB admitted response; GREEN39 (21 routes,9 responses), including exact64MiB acceptance; no boundary is inferred from only an above-generic-cap sample.
- Rustfmt and scoped diff-check GREEN.
- Home is assigned the registered current Hub catalog5+HTTP1 native gate after its MCP GREEN2 receipt. No HTTP pass is inferred from relay tests.
- Root genuine Stdio+GIS generation gate9773 remains active in the distinct `🗑️generated/trusted-gis-real-activation/hub-target`. Its in-memory script snapshot predates this body-route addition; it does not qualify the new HTTP path.

## Remaining Boundary

Implement private reservation-owned actor acquisition, per-await authority/expiry checks, exclusive body handoff, and a real GIS describe call in Chromium against the freshly materialized descriptor. Keep renderer-unavailable until actual renderer behavior is observed. Typed host effects, document Store/WAL ownership, durable approval/undo, cold Map MCP and second-collaborator observation remain separate mandatory acceptance.

## Native Receipt Correction

Session9773 ended BUILD RED before materialization. Retry27152 is still compiling in the same root-owned cache with bounded exact JSON output capture. Neither run establishes candidate or GIS activation success.
