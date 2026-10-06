# Canonical Native Typed Input Law

The unchanged original full renderer run (session 35764, actual exit 1) exposed an old test that passed a JSON object into the native whole-buffer-only retained cursor. The production API was already explicit about intrinsic Uint8Array ownership and transfer; no compatibility path was added.

The existing language-neutral input row now carries its explicit 159-byte wire vector. Its schema admits only nonempty pairs of lowercase hexadecimal digits. The original input facts and expected normalized component remain unchanged. Independent Ajv validates the whole fixture; an actual Bun runtime observation confirms complete buffer detachment, the original normalized component, payload retirement and cursor retirement, with a `[DEBUG]` log. The registered four-law owning suite still needs to run.

Three exact source pairs and three complete production contexts are retained in `🗑️generated/canonical-native-typed-input/source-1.json`; the guarded publication/inverses are retained in `publication-1.jsonl`. The test continues to exercise native decoding and the same original result assertion. Every other fixture case and test remains unchanged. No Rust or whole renderer success follows from these bounded observations.


Actual Whole Typed Cursor Law — 2026-10-05

The original complete General typed cursor module passes all four laws using the registered Nx renderer test-long route, with no name filter. The language-neutral fixture now supplies its exact 159-byte native input; original normalization expectations remain unchanged. The actual input detachment and payload/cursor retirement checks pass. Ajv validation and the bounded public-API semantic observation remain separate; this establishes no Rust result.

Actual terminal evidence and full raw logs, authored full pairs, publication journals, and post-terminal current bodies are retained in `🗑️generated/canonical-native-typed-input/owning-observation-1.json`. The current snapshot does not establish atomic source identity during execution. Current authored-pair matches: 3/3.
