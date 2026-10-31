# Ticket Close Advance Audit

Read-only source/schema/client review. No ticket_close, goal mutation, management message, Git mutation or worktree operation was invoked. Completion gates remain prerequisites.

## Current MCP Route and Schema

MCP schema is summary(required), files, explicit path, optional title and no_management. Omitting path selects latest open ticket: always send26/10/28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS. Omit title to avoid separate retitle effect; no bulk/all. Schema: ⌨️cli/📦️packages/🐹️go/🏭️mcp-factory/🐹️.go212-220. Handler: ⌨️cli/📦️packages/🐹️go/🛎️mcp-tool-repo/🐹️.go156-160. Context: 🔗️graphql/📦️packages/🐹️go/💗️tickets/🐹️.go58-77.

Canonical service.Close17-75 in 🎫️tickets/📦️packages/🐹️go/🔒️service-close/🐹️.go requires open status, normalizes files, inspects/removes important bundle, optionally moves plan into ticket, sets closed status/summary/interaction, saves and emits close-ended. No Git commit/stash/checkout/reset/worktree operation occurs. Context metadata uses author/checkpoint read helpers. Service.Save143-152 in 🛎️service/🐹️.go writes only selected-layout ticket document. no_management=true skips external management issue/comment integration at service46-48; it does not bypass local close, important requirements, plan relocation or input validation. No goal close/reopen/open occurs.

Important bundle must be regular, exactly empty, and its directory contain only that file (🎫️tickets/📦️packages/🐹️go/💾️important/🐹️.go54-83). Preserve notes elsewhere first; do not delete the required bundle before close. Plan relocation applies only when ticket.Plan.Source is set (🗺️plan-source/🐹️.go124-147). Inspect actual ticket metadata before close.

## Generated Cleanup Distinction

Current canonical MCP service.Close does NOT purge generated output. Separate older FinishTicket DOES invoke PurgeTicketGeneratedOutput(PurgeOversized,Apply=true) after save (🎫️tickets/📦️packages/🐹️go/🔓️finish-reopen/🐹️.go563-565). Policy selects generated files>5MiB/folders>10MiB; protects tracked/linked/outside/recent-writer content (same file28-47,292-368). It is not a complete cleanup guarantee. Installed aa03 cache image had successful discovery, but this source review does not bind its executable to the current checked-in route. Do not rely on automatic cleanup or its absence to preserve unexported evidence. Export proof first.

## Client Outcome Marker

repo-mcp-client.py26-46 exclusively creates generated/repo-ticket-close-attempt.json BEFORE dispatch, blocks existing attempts, records unknown failure type/message on BaseException and completed only after non-isError reply. Single monotonic45s reply deadline85; owned graceful/terminate/kill shutdown10-24. Explicit executable/cwd60 avoids wrong root. Actual close response is saved109-111. No mutation was dispatched during this audit.

## Safe Final Ordering

1. Complete all tests/native/image/profile/query gates. Export exact maps/runtime/outcomes/image inventory/review to permanent ticket Markdown. Preserve scripts/config/cases/reports; preserve needed binary/image evidence in an authorized durable artifact before generated deletion.
2. Stop/confirm owned writers and account for shared-ticket external active work. Resolve/check canonical generated path inside this exact ticket. Delete only authorized generated output after export. Preserve required important bundle and production inputs.
3. Prepare exact structured request plus permanent Markdown intent: exact ticketID, summary, complete production/test and retained ticket file inventory, no_management=true. Recheck current service schema and ticket state; no implicit latest, title, bulk/all or goal calls.
4. Client recreates generated for stderr/exclusiveattempt/result. Dispatch ONCE only after all gates pass. Keep attempt marker until outcome settled; no concurrent generated cleanup during RPC.
5. After dispatched timeout/protocol/tool/process failure, outcome is UNKNOWN. Preserve request/marker and inspect actual ticket status/last close interaction read-only. Never remove marker merely to retry. A documented intentional retry requires verified still-open state; already-closed means no retry.
6. After successful reply AND verified closed state, export reply/attempt/state evidence into permanent Markdown. Only then remove generated close logs/result/exclusivemarker and owned generated directory. This meets final cleanup without erasing uncertainty evidence.

This is advance ordering advice. Actual cutaway blocker and remaining gates prohibit closing now.

## Exact Source Links

- [MCP schema](C:/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🏭️mcp-factory/🐹️.go:212)
- [MCP handler](C:/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🛎️mcp-tool-repo/🐹️.go:156)
- [Canonical close](C:/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🐹️go/🔒️service-close/🐹️.go:17)
- [Context route](C:/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🐹️go/💗️tickets/🐹️.go:58)
- [Older purge call](C:/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🐹️go/🔓️finish-reopen/🐹️.go:563)
