# Draw Browser Attestation

The ticket-side `attest-draw-journeys-26.ts` requires the exact newly built component SHA-256 as an explicit argument before any activation. It reconciles the model, committed semantic pair and browser publication using `verify-published-draw-receipt.ts`, then runs the existing registered support, prepare and activate owners through `draw-native-dev.ts`. It calls the registered browser acceptance owner independently for English and German. Before and after each journey it repeats model reconciliation, checks the HTTP receipt against the pinned component and descriptor, and invokes `verify-served-draw-cores-25.py` to hash the actual HTTP bytes against all three published core files. The four checkpoints must agree on all core names, lengths, URLs and hashes. Generated JSON records the pin, checkpoints, journeys and failure state atomically.

After the component stage is released, invoke from the repository root with the exact newly built SHA substituted:

```sh
repo tools exec --light bun -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/attest-draw-journeys-26.ts' final26 '<NEW_COMPONENT_SHA256>' 'http://127.0.0.1:6064/' run
```

The proof is `🗑️generated/attestation-final26.json`; per-journey artifacts are under `🗑️generated/journey-final26-en` and `journey-final26-de`. The wrapper fails closed when the model or receipt does not match the supplied pin, any stage fails, the served receipt is stale, a core differs, there are not exactly three cores, or a checkpoint changes. It does not itself build the component.

Read-only `check-only` passed for the previously published `fcc8e11c2267be890c236bf25f06d9e378b87b4becc78bebb74d95a6bcfb10e2` receipt; this is not final-component acceptance. A zero SHA in `check-only` failed before activation with a pin mismatch and recorded a failed proof. No activation, publication or browser journey was run as part of preparing this wrapper.
