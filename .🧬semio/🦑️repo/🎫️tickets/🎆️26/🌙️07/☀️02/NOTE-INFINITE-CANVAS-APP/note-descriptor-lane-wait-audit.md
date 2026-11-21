# Descriptor Emitter Lane Wait Audit

`🗑️generated/descriptor-emitter-capabilities-3.log` begins with the owned build entering `build`, then `[repo cargo] all 2 live lanes are in use (maximum 32); waiting for a free lane`, followed only by the outer heartbeat. The absence of Cargo output or a reported repo lease waiter does not imply the build is executing Cargo or stuck in source admission.

The corresponding pre-admission path is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🎯️product/🟦️.ts::acquireProductLaneV1`, lines 10–35. It derives a stable product lane from the product key, takes a catalog claim, and attempts the exclusive `lane-<id>` claim. If that particular claim fails, line 21 emits `poolBusy` with the count of claimed lanes and the configured maximum, then lines 34–35 poll. The message does not mean the entire 32-lane pool is full. The same function releases its claim when the current owner releases its lane. These internal claims are distinct from the resource leases listed by `repo leases status`.

The supported path established by the code is continued waiting for normal product-lane release; no safe independent bypass or intervention was established. No peer process was inspected, stopped, or altered, and no build, deletion, or code change was performed.
