# Semantic Command Names

## Why

Mutations live one-operation-per-folder with a verb-noun slug (`🌱create-step`, `🏷️rename-writer`). App commands were grouped under noun buckets:

`✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/🎮️commands/✍️text`

That folder held `text_edit`, `set_text`, `open_document`, `format_document`, … — the folder name is a domain noun, not a command. Lowpoly `➕️add-primitive` is the target: one command, semantic kebab slug, payload at file top level (`module_inception`).

## Target

- One `🎮️commands/<emoji><verb-noun>/🦀️component.rs` per command.
- Glue `pub mod` matches the slug (`text_edit`, `set_camera`, `open_document`).
- Flatten inner `pub mod foo { … }` to the file root.
- Keep struct names, `#[dsl(keyword)]`, and `app_commands!` action ids (wire format) unless they are already the inner module name.
- Shared helpers used by several commands from one old file are copied into each consumer.

## Inventory (pre-refactor)

- 344 command folders
- 170 multi-command grouped folders (split)
- 77 single inner `pub mod` (flatten + rename if the folder is a noun)
- 97 already flat (rename remaining nouns; skip empty leftovers)

No inner-mod name collisions within a single app.

## Associated goal

`AI-OPTIMIZED-REPO` — same family as `SEMANTIC-MUTATIONS-OVERHAUL` (26/08/12). That ticket is mutations-only; this one is app commands.
