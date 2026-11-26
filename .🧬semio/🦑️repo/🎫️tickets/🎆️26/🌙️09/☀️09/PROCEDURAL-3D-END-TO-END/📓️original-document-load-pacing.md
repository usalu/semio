# Original document load pacing

The stock B4 requester reported 19,030 consecutive reactor crossings with document load as its only more-work source during initial example hydration. The archive status poll observes the retained load, besides advancing a merge that requests host polling. Initial document replacement progresses on the reactor’s existing live maintenance lane. Its per-app ladder can spend multiple funded units in one callback, while the reactor typically invoked the outer worker pump only once per crossing. A worker callback, outcome acknowledgement, and resume used successive pump calls even when no host input or output was involved.

After its normal first cleanup step, an ingress-free reactor turn can meanwhile continue that same maintenance pump while the live archive lane’s demand fits the current retained grant. Each extra call passes through the existing round-robin instance selector and its original worker outcome checkout, acknowledgement, and Source resume. Both existing displaced-owner pressure and the 32-pump ceiling remain in the same loop. The archive branch checks cancellation, context deadline and fuel, and a relative 8,000 μs execution slice. An absent or regressing clock closes that slice. The current grant is checked again before each extra call. Input-bearing turns still start with one ordinary maintenance call before driving incoming events and pages.

## Verification

- A focused native executable compiled the actual repository Rust predicate and ran eight language-neutral fixture cases, plus the original quote-error propagation check. The regression first failed against the missing archive slice.
- SQLite independently reproduced the same eight outcomes. A recursive model of 19,030 unchanged pump calls at 200 μs per call, with the repository’s 32-call cap, produced 595 reactor crossings. That model holds the per-call duration constant. Updated browser timing remains unmeasured.
- The repository’s canonical source closure measured the reactor turn at 2,781 native active lines and 2,806 WASI active lines. Each closure contains 17 inputs and 224,982 bytes.
- No full requester rebuild was started for this lane.
