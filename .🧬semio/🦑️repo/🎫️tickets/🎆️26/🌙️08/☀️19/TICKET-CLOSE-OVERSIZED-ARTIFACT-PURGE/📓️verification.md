# Ticket Close Oversized Artifact Purge

## Summary

On `ticket_close`, `FinishTicket` now purges oversized scratch artifacts inside the closed ticket folder only:

- Files **> 5 MiB** are deleted (`os.Remove`)
- Subdirectories **> 10 MiB** (sum of contained file sizes) are deleted (`os.RemoveAll`)
- Applies to tracked and gitignored paths (plain filesystem walk)
- Never deletes the ticket root or `🎫️ticket.json`
- Failures warn via `writeWarningf` and do not fail the close

## Verification

```
go test -run TestFinishTicketPurgesOversizedArtifacts -count=1 -v
```

Result: **PASS** (0.05s)

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️component_test.go`

## MCP note

Repo MCP (`ticket_open` / `ticket_close`) was unavailable in this session; ticket folder created on disk manually.
