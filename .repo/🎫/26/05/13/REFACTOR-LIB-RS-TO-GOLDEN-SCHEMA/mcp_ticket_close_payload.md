# Repo MCP `ticket_close` payload

From `mcps/project-0-compose-repo/tools/ticket_close.json`:

- **`summary`** (string, **required**): closing summary text.
- **`files`** (array of strings, optional): paths created/updated/removed during the ticket (repo-relative or as you store in `ticket.json`).
- **`path`** (string, optional): ticket id folder e.g. `26/05/13/TICKET-SLUG`; omit = latest open ticket.
- **`title`** (string, optional): updated ticket title.
- **`no_management`** (boolean, optional): skip GitHub issue update.

Probe: `ticket_close` with invalid `path` returned `ticket not found` (tool accepted `files` array without schema error).
