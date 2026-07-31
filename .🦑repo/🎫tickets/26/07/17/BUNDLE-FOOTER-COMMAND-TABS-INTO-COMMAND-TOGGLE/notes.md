# Bundle Footer Command Tabs Into Command Toggle

## Change

Wrapped every bottom-middle command-category leaf under one expandable `framework.category.command` branch (mirrors Display on bottom-left).

When the panel is folded (`rootRowOnly`), chrome shows a single **Command** toggle instead of Appearance / Layout / Language / … inlined along the footer. Expanding Command reveals the category tabs; palette redirects and category-switch collapse use the two-segment path `[framework.category.command, command.category.<id>]`.

## i18n

- `ui.panelToggle.command` → EN "Command" / DE "Befehl"
