# Fan-out: semantic command folders

Ticket: `26/08/13/SEMANTIC-COMMAND-NAMES`. Writer is the exemplar and is **done** — do not touch `✒️writer`.

## Rule

Every `🎮️commands/<folder>/` must be **one command**, named like a mutation (verb-noun kebab), payload at file top level (`module_inception`). Noun buckets (`✍️text`, `🗂️selection`, `🗣️locale`, `🎥️camera`) are wrong.

Writer target (already applied):

- `✍️text` → `✍️text-edit`, `✍️set-text`, `✍️open-document`, `✍️format-document`, …
- Glue: `pub mod text_edit;` with `#[path = ".../✍️text-edit/🦀️component.rs"]`
- App imports: `commands::text_edit`, not `commands::text::text_edit`

## Per plugin

1. Split each `pub mod foo { struct … handle }` into `🎮️commands/<emoji><kebab(foo)>/🦀️component.rs` with the struct at file root.
2. Split grouped `pub fn` command files the same way.
3. Rename remaining noun folders that already hold a single command (`locale` → `set-locale` when the command is `set_locale`).
4. Update that plugin’s `📦️glue.rs` `#[path]` + `pub mod`.
5. Update imports **only inside that plugin**.
6. Copy shared helpers into each consumer file. Move tests with the command they exercise.
7. Keep struct names and `#[dsl(keyword)]` / `app_commands!` action ids.
8. Do not use git modifying commands. Do not touch other plugins. Scratch files only in this ticket folder.

## Plugin ownership

See agent assignments. Writer is excluded.
