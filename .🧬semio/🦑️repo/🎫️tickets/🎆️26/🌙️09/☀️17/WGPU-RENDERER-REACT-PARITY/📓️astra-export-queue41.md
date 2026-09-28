# Export Queue Parity 41

The WGPU host previously refused an entire IconRenderExport effect above 64 items, although React visits every item. It also bulk-extended a single pending item queue and rejected requests while cancellation was retiring resources.

The new host retains each incoming vector as an owned iterator in a linked queue. Admission transfers ownership without copying or visiting its items. Each pump takes at most one queued item or releases one exhausted group. Cancellation marks the active and admitted scope tokens; requests arriving afterward receive a fresh scope and wait until prior owners have retired. No queued payload is bulk-dropped on capacity refusal. The existing frame budget remains 32 steps or two milliseconds.

The neutral fixture now covers one 96-item effect, two effects totaling 112 items, cancellation of 96 queued items, and a new 73-item effect arriving while the previous 96 items drain. The native integration law routes actual host effects, checks admission counts, per-pump item bounds, terminal ownership, failure totals, cancellation totals and fresh-scope processing. The independent TypeScript oracle uses Ajv request validation, AbortController scopes and Promise.allSettled over the same cases.

Native tests were written before the source change, but no native red run was executed because the current renderer build is still waiting on the shared Cargo artifact lock. The scoped Nx TypeScript oracle completed with 6/6 passing tests and exit 0; its receipt is `🗑️generated/astra-runtime/gate34/icon-batch41-oracle.log`. Scoped rustfmt parsing also exited 0 without applying formatting. This is not a native passing receipt or a completed physical export acceptance.
