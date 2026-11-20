# Channel Preview and Hover Audit

The preview builds one instance per geometry-bearing output leaf in editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:348. Instance IDs are widget@channel#index and interaction IDs are widget@channel (lines 381 and 426). Enumeration is in 🧵️preview-eval/🧊️preview-geometry/🦀️.rs:138. Widget preview visibility gates emission (lines 369–374).

The graph outline port rows in editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:145 declare selection granularity but no hover binding. Node rows bind HoverPreview explicitly (lines 159–172). Add a port-row hover action using the qualified port ID, graph handle granularity, and pointer channel; test the target, channel-specific highlighting and hover clearing.
