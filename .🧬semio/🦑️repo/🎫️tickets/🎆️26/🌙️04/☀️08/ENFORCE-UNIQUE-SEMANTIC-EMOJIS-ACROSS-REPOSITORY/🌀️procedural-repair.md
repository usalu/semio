# Procedural Emoji Repair

## Scope

`✏️s/🔌️plugins/🌀️procedural`

## Handpicked identities

- Configuration/options sibling pairs: `⚙️config` and `🎚️options`.
- Oracles: `🔮️oracle`; WFC scheduling: `💼️job` and `🧵️parallel`.
- 2D formats: `🖊️dwg` and `📐️dxf`; 3D formats: `🧊️gltf` and `🗿️obj`.
- 3D examples: `📐️box-fillet-preview`, `🐚️box-shell-preview`, `🧹️face-sweep-extrude`, `🍄️hexagonal-mushroom-column`, `📦️rectangle-extrude-volume`, `🪢️rectangle-wire-preview`, `🧲️sphere-box-fuse`, and `🍩️sphere-cut-with-torus`.
- Assembly mutations: `📏️create-rule`, `🧩️create-slot`, `❌delete-rule`, `🕳️delete-slot`, and `🪶️remove-weight`.
- Mutation schemas use `🧬️.schema.json`, mutation data retains `🔣️.json`, and the two colliding mutation GraphQL files use `🕸️.graphql`.
- Camera fixture and law files received role-specific identities, including `⬅️before.json`, `➡️after.json`, `👑️...owner...`, `🧷️...retained...`, and `🔬️...third-party-oracle...`.

## Verification

- Strict path audit: 1,143 files, 945 directories, 2,080 governed entries; all eight finding categories are zero.
- Central taxonomy validation: `[]`.
- TypeScript package test: passed (`[DEBUG] procedural ts ok`).
- Rust quick test: running at the time this note was first written; its final result is appended below when available.

