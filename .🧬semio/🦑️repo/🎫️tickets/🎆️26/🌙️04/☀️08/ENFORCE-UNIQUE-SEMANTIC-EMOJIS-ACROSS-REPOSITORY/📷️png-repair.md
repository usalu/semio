# PNG Handpicked Emoji Repair

Repaired `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any`. Options use `☑️`, oracle `🔮️`, regression tests `🛡️`, and pixel replacement `🔲️`. Mutation payload schema files use `🧬️.schema.json`, distinct from their sibling `🔣️.json` metadata.

The 15 primary fixture directories use their individual mutation identities: header `📐️`, palette `🎨️`, transparency `👁️`, gamma `🌗️`, chromaticities `🌈️`, rendering intent `🖌️`, dimensions `📏️`, timestamp `🕰️`, background `🖼️`, insert text `📥️`, remove text `🗑️`, edit text `✏️`, pixels `🔲️`, insert unknown chunk `📦️`, remove unknown chunk `📤️`. The independent Pillow fixtures use `📅️` timestamp, `➕️` insert, and `➖️` remove, keeping them distinct from sibling primary fixtures. Every before/after carrier uses `⬅️`/`➡️`.

Generators now carry authored fixture directory names separately from stable recipe IDs. Both the standalone Rust writer and Pillow writer emit the chosen carrier names. Exact schema imports, metadata, fixture manifests, test paths, and Stdio oracle mount were repaired.

Verification on 2026-09-05:

- Statute audit: 402 files, 280 directories, 675 governed entries; every finding category zero.
- Fixture verifier: 18 fixtures, zero problems, exit 0.
- All 119 JSON files parse; 18 manifests reference 36 files with exact byte lengths and SHA-256.
- Nx primary generator-manifests produced 15/15 bundles under this ticket's generated folder.
- Nx Pillow chunks generator and chunks-manifests completed successfully for all three chunk cases. Runtime reader comparisons confirmed the changes are observable.
- All 36 regenerated files match the committed fixture bytes exactly by SHA-256 and byte length.
- No stale options, schema, pixel-operation, or local-oracle coordinates remain in the tree; the plugin-level `🧪️oracle` host path is intentional.

The semantic follow-up changed the arbitrary octopus mutation-suite identity to `🔀️`, each purple-circle direct-behavior case to `🎯️`, and the Rathaus Ahlen plan fixture to `🏛️`. All fifteen mutation-catalog scenario directory fields now point at the actual `🎯️direct-behavior` directories.
