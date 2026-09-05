# Pack, Intrinsic Size, and Graph Hand Review

The exact scopes are framework `🎒️pack`, `📏️intrinsic-size`, and `🕸️graph`; none contains scoped AGENTS instructions. All authored and physically present generated entries are included in review, not just tracked files.

## Authored Decisions

- `🎒️pack/🧪️testkit` → `🎒️pack/🧨️testkit`: the source implements truncation and bit-flip corruption fuzzers. The explosion identity distinguishes deliberate corruption testing from sibling `🧪️tests`, which contains the neutral fixture corpus. The package's exact module mount was updated; its public `testkit` module name is unchanged. The retained source SHA-256 before and after is `a0ffb99d55faa35342421d458a8862209289fc6c30e6d0b6b4506353a43c86b2`.
- `🕸️graph/🛂️manifest/🦀️generated-value-bridge.rs` → `🌉️generated-value-bridge.rs`: despite its stem, this is explicitly handwritten ToValue/FromValue conversion glue for generated manifest enums. The bridge identity distinguishes it from the sibling Rust manifest implementation. Its one exact include was updated; the before/after SHA-256 is `20b51afe1e08ccd41e4a0301d04e228d4432288e28551a8841069d3453a16ee9`.
- The empty, unreferenced `🎒️pack/🧾️json` directory was removed with an exact `rmdir`. No file was removed; the directory is trivially recreatable. The actual JSON implementation remains intact under `🔤️json`.

Pack's remaining folders distinguish its binary format, asynchronous scheduling, HTTP range transport, native I/O, JSON codec, package glue, and neutral fixtures. The handpicked corruption-test directory was added to the exact semantic member registry.

Intrinsic-size has seven physical entries / six governed and no naming or role violations. Its literal Cargo manifest and existing single Rust/JSON leaves are retained. Its source-link target for image preview measurement still exists and contains the named function.

Native verification through Nx with an isolated ticket Cargo target passed 90 pack tests and nine intrinsic-size tests. The existing neutral corpora and independent serde_json, image, and usvg comparisons ran. No test assertions or API behavior were changed.

## Generated Graph Authority

The initial graph output root had 22 files and 19 sibling-duplicate emoji findings. These ignored files were included, not exempted. The existing generator derived every Rust/TypeScript filename from its format marker. It now consumes `🛂️manifest/📇️outputs.json`, governed by `🧬️outputs.schema.json`, with exact handpicked current output identities and a bijection to the admitted manifest IDs. Missing/extra/duplicate IDs, duplicate paths, unpaired language owners, unsafe paths, and emoji breaches are rejected rather than assigned a fallback name.

Each of the following manually reviewed owners contains the explicitly selected language leaves `🦀️.rs` and `🟦️.ts`:

| Manifest identity | Directory | Content basis |
| --- | --- | --- |
| drawing-layers | 🖌️drawing-layers | Shape, path, text, image, group, boolean, and trace layers |
| flow-dag | 🌊️flow-dag | Computation, controls, preview, action, and app-instance nodes |
| nakagin | 🏢️nakagin | Nakagin Capsule Tower pieces, connectors, and building relationships |
| puzzle2d-default | ◻️puzzle2d-default | Planar puzzle ports and links |
| puzzle3d-default | 🧊️puzzle3d-default | Spatial vortex, cable, and attraction relationships |
| puzzle5d-default | 🖐️puzzle5d-default | Combined port/vortex and link/cable/attraction catalog |
| rewrite-lhs | 🫲️rewrite-lhs | Left-hand-side match/where rewrite patterns |
| wires | 🧠️wires | Mindmap ownership, identity, references, and possession relations |
| writer-languages | 🗣️writer-languages | Jack, Wire, plaintext, and Markdown languages |

The root's shared files are `🦀️registry.rs`, `🟦️.ts`, `🔠️types.ts`, and `🔣️manifest.schema.json`. Their sibling emojis are distinct. Registry attributes, TypeScript exports/imports, nested type imports, preview nodes, freshness membership, and normal writer parent-directory handling all consume the declared paths. Cargo watches the two authority files. Existing graph test commands now run the neutral output tests before their native tests; no separate permanent script was introduced.

The neutral fixture deliberately maps unrelated IDs to different handpicked owner stems, proving paths are metadata rather than inferred from an ID. Ajv independently validates structural examples. The initial test invocation mistakenly supplied a Bun filter rather than an explicit `./` path and did not run tests; that routing error is retained, not described as a behavioral red test. The actual nested writer test then failed with ENOENT before implementation, and subsequently both tests passed with 27 assertions, including fail-before-write symlink rejection and exact stale nested removal. Writer/preview/check ownership was handed to the concurrent backend agent for review after these changes; subsequent edits in those regions must be coordinated through the parent.

## Provenance and Verification

A concurrent process invoked the normal graph producer at 17:23 before this agent's planned raw backup/preview. The original raw 22-file backup is unavailable. The attempted 17:24 copy was therefore explicitly renamed `graph-output-after-observed`; it is not misrepresented as a before image. No generated payload was hand-edited by this agent.

All 22 original SHA-256 values were recorded before that invocation. A read-only proof with an exact hand-listed old/new mapping matched every original hash: nine Rust manifest modules, the shared types, and the JSON schema are byte-identical; nine TypeScript modules differ only by their known type-import path; the Rust registry differs only by its nine path attributes; the TypeScript index differs only by its exact imports/exports. The prior bytes were reconstructed only in memory for hashing, never written as sources or compatibility aliases. The proof is retained in `🗑️generated/metabolism-glb/graph-preimage-proof.json`.

The normal graph freshness target passed for nine manifests. Its preview has 32 exact nodes: one root, nine semantic directories, and 22 files; no stale removals. Final physical audit: pack 21 entries / 20 governed, intrinsic-size 7 / 6, graph 55 / 53. Every tree has zero naming and unresolved-directory-role findings, including all generated output entries.

Native graph verification passed 184 tests, including current manifest lookup, Nakagin/Flow loading, value conversion, algorithms, and graph vocabulary checks. The final Bun suite passed three tests / 60 assertions, including actual TypeScript registry imports for all nine IDs, current nested type-import paths, and unknown-ID rejection. An earlier standalone inline import diagnostic terminated with status 143 and no output; its cause is unproven and it was not counted as a test result. The permanent successful registry test provides the runtime evidence instead. Scoped whitespace checks passed.
