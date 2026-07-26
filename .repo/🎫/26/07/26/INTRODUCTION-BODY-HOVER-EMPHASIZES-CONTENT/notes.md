# Introduction Body Hover Emphasizes Content

## Intent
When the pointer is inside the introduction step window body, all window content (title chip, body copy/checklist/icons, step chip, close) should use the emphasized color — same token as shell hover emphasis.

## Change
- CSS in `ui/styling/js/ui.css` `ShellParentHover`:
  - `:has([data-slot="window-chrome-body"]:hover)` emphasizes cap/footer/close content
  - body `:hover` emphasizes `p`/`li`/`span`/`[data-icon]` except `[data-celebrated="true"]`
- Docstring on `UIIntroduction` notes the contract
- Vitest covers CSS contract + markup slots

## MCP
`CallMcpTool` `ticket_open` failed with "Cannot call tool before MCP process client is registered" after successful `mcp_auth`. Opened via `./mcp ticket open … --no-issue`.
