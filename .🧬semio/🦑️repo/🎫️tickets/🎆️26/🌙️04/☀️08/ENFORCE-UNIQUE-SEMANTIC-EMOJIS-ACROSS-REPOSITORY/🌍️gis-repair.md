# GIS Emoji Repair

## Scope

Hand-reviewed every strict path-statute finding under `✏️s/🔌️plugins/🌍️gis` and assigned semantic, sibling-unique identities. The repair covered configuration/options pairs, artifact carrier formats, fixture transitions, the Gismap mutation runner, mutation payload schemas, presentation normalization, and oracle ownership.

## Result

- Strict audit: 637 files, 491 directories, 1,120 governed entries.
- Findings: zero missing, generic, presentation, spacing, duplicate, multiple-emoji, reserved-name, and oracle breaches.
- Central taxonomy validation: zero problems.
- TypeScript example/runtime tests: 4 passed, 0 failed.
- Rust library test reached the shared `semio-s-plugin-stdio` dependency and failed there; no GIS path diagnostic was reported. This is retained as an honest shared-dependency blocker, not reported as a GIS pass.

## Handpicked identities

- Window configuration: `⚙️config`, paired with retained `🎚️options`.
- OBJ carrier: `🗿️obj`; DXF carrier: `📐️dxf`.
- Applied camera/locale fixtures: `📷️set-camera-applied`, `🌐️set-locale-applied`, with `⬅️before` and `➡️after`.
- Gismap mutation runner: `🦠️mutate-gismap-1`.
- Mutation payload schema: `🧬️.schema.json` where it has no same-level collision.
- Oracle owner: `🔮️oracle`.

