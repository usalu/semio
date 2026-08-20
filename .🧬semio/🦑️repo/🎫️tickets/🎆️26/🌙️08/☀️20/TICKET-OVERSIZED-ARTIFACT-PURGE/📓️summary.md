# Ticket Oversized Artifact Purge

## Policy

Inside every ticket slug folder under `.🧬semio/🦑️repo/🎫️tickets`:

- Files **> 5 MiB** are deleted
- Subdirectories **> 10 MiB** (sum of contained file sizes) are deleted recursively
- `🎫️ticket.json` and the ticket root are never deleted
- Gitignored artifacts are included (plain filesystem walk)

## Automatic enforcement

`FinishTicket` (used by `ticket_close` / MCP `ticket_close`) already calls `purgeOversizedTicketArtifacts` after saving the closed ticket.

## Bulk cleanup

New CLI command:

```bash
client ticket purge-artifacts --all
client ticket purge-artifacts 26/08/20/TICKET-OVERSIZED-ARTIFACT-PURGE
```

`PurgeAllOversizedTicketArtifacts` walks every slug directory under the dated ticket tree (including orphan folders without `ticket.json`).

## Verification

```
go test -run 'TestFinishTicketPurgesOversizedArtifacts|TestPurgeAllOversizedTicketArtifacts' -count=1 -v ./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/
```

Result: **PASS**

Bulk purge run:

```
Purged oversized artifacts in 3463 ticket folders
find …/🎫️tickets -type f -size +5M | wc -l  → 0
```

Down from **9295** oversized files before cleanup.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️component_test.go`
