# Presentation Metadata Ownership Audit

The generic framework manifest currently chooses icons by matching app-specific action IDs and example names. This is an additional ownership violation beyond locale and active utility config.

`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` contains `catalog_action_icon_id` branches for `recomputeRewrite`, `exportRegistersCsv`, `exportVideoFromDeck`, `paintStroke`, `incrementViaCommand`, and other domain commands. `catalog_command_icon_id` recognizes `animate.resetGrid` and OS command IDs. `catalog_example_icon_id` recognizes Nakagin, forest, concrete, and hex example names. Those names belong to their declaring app or product, not a generic manifest model.

`ActionDefinition::new` and `CommandDefinition::new` already accept an explicit icon, and `App::action_with` accepts a complete declaration. The final design should keep neutral defaults based on `ActionKind` in the shared schema, with explicit presentation metadata at action/command/example declarations. Framework interaction/history actions can declare their icons in their owning framework subsystem. OS-specific command icons belong to the OS declaration. Existing visible icon choices should be retained through declarations where used. Dead helper APIs should be removed rather than kept as compatibility layers.

The mode and example helpers require an exact usage audit before removal. One SDK builder contract test calls the mode helper; the same test already declares its real mode icon explicitly. No implementation change to presentation metadata has been made yet. This is a bounded remaining execution/audit item for this ticket.
