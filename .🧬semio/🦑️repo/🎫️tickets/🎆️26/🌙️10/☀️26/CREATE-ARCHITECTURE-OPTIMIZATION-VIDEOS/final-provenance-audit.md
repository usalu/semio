# Final Provenance Audit

All six German speaker movies pass current provenance verification through the existing runtime interface. Settings are explicit German, speaker audience, high quality (-qh), default 1080p resolution and 30 fps. Each complete movie has its own matching content and presentation identity.

| Video | Certificate | Source Entries | Bytes |
|---|---|---:|---:|
| optimization_extrema | Pass | 38 | 17873589 |
| optimization_sampling | Pass | 38 | 21608735 |
| optimization_experiments | Pass | 38 | 25902815 |
| optimization_index | Pass | 38 | 28905895 |
| optimization_pareto | Pass | 38 | 21321371 |
| optimization_algorithms | Pass | 38 | 39721291 |

The six curriculum entries have distinct IDs, wrapper paths and output names. Each imported wrapper declares the correct VIDEO_ID. Each certificate matches current presentation_state and freshly recomputed segment_state, including chapter identities and settings. All nested SHA-256 source fingerprints pass state_inputs_current. No stale or incomplete provenance was accepted.

Read-only verification: no rendering, encoding, broad tests or source edits. Visual and decode/metadata audits are handled separately. provenance-audit.py is retained as the verification input.
