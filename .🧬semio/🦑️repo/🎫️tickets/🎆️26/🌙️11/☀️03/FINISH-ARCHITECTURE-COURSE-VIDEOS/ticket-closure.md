# Ticket Closure Inspection

Do not close until all required work and verification are complete. The user forbids modifying Git commands and goal closure without explicit instruction. MCP ticket_close is preferred when bound to the correct workspace; CLI fallback is available if MCP remains in the wrong cwd.

CLI syntax: repo ticket close 2026/11/03/FINISH-ARCHITECTURE-COURSE-VIDEOS <summary> <file1> <file2> ... --no-management. Summary is one quoted positional argument and at least one file path is mandatory. Alternatively --summary accepts the summary and --files is a repeatable string-slice flag; the positional file list avoids comma parsing. --title is optional and should be omitted absent a need to change title. --all is bulk and must not be used. Hidden --no-management skips management-provider operations.

The CLI implementation sends GraphQL ticketClose with year/month/day/slug, summary, files and noManagement. The current service validates, moves any plan into the ticket, removes the important marker, sets summary/status, appends a close interaction including provided files, saves ticket JSON and emits an event. It skips issue closure when noManagement is set. An older finish path also skips issue diffs/comments/closure under noManagement and can purge oversized generated output; it reads author/checkpoint metadata. The inspected close paths contain no modifying Git operation, but no live close was executed and this source review cannot prove behavior of an independently configured provider/hook. Closure inherently changes local ticket records and may move a plan or remove the important marker.

## Deferred Closure Interface Recheck

The current MCP ticket_close schema accepts summary, optional path, files array, no_management and title; no workspace argument is exposed. Prefer a final structured call with the existing ticket ID, complete attributed file array and no_management=true after verification and cleanup. If the already observed server working-directory issue remains, the CLI help confirms `repo ticket close [path] [summary] [files...]`; use the reviewed no-management path. This is interface discovery only. The ticket and goal are still open.
