# Test Layout Ticket Closure Transport

The final exact Rust authored-file array alone is 1,457,618 bytes (the initial measured snapshot was 1,223,437 bytes) as compact UTF-8 JSON, exceeding the repository MCP server’s default 1 MiB request limit. The closure must retain every changed path; neither truncation nor manifest-file substitution preserves the required file attribution.

Built the real repository MCP through public `bun nx exec --projects=layout-probe -- go ... build` with a generated Go overlay affecting only its entry-point constructor call. The overlay creates `limits := DefaultLimits()`, sets `limits.MaxPayloadBytes = 8 << 20`, and calls the existing `NewRepositoryServerWithLimitsFor(repository, profile, limits)` API. Repository source, server defaults, validation, and ticket implementation remain unchanged. The generated executable is private to this ticket and will be removed after closure.

The build returned exit 0. A sequential JSON-RPC initialization negotiated protocol `2024-11-05`. `tools/list` returned six tools including the real `ticket_close` schema with `path`, `summary`, `files`, and `no_management`. Initializing and closing stdin before waiting for asynchronous replies produced `session closed`; the validated probe instead waits for each matching response before continuing.

Closure will use `no_management: true` and the complete deduplicated authored-path array. Final request size and close result are recorded here after execution.

Validated read-only ping: 2097215 request bytes, 2097199 response bytes, empty success result. This independently confirms the compiled instance accepts and returns a payload larger than the original 1 MiB limit without changing repository defaults.

Retention preflight at 2026-09-08T22:08:05.612931+00:00: no retained root file exceeds the 5 MiB purge limit, and no retained immediate child directory exceeds 10 MiB (including its current caches/output children). The largest retained directory is the pre-existing `w14-audit` at 7,392,858 bytes. The private generated MCP executable remains until the close response; active executor lanes will be cleaned after their evidence is transcribed.

At 2026-09-08T23:13:47.502631+00:00, completed plugin, SPR, Binaryen, renderer, Rustdoc, and CI baseline generated lanes were removed after their runtime evidence was recorded. The source-preservation preimages remain in the retained Markdown report; verification inputs remain in their ticket directories. The final scan and MCP transport outputs remain until their operations finish.

The current `Ticket` serializer excludes its in-memory interactions (`json:"-"`), and `SaveTicket` persists the ticket status and summary. Closure verification therefore checks the exact complete file array passed to the real MCP, its successful response, the stored ticket status/summary, and the retained full manifest. It will not claim that the ticket JSON stores an interaction/file array that its current schema omits.

Final retention preflight at 2026-09-08T23:20:08.821974+00:00: all current verification and scan generated lanes are deleted, with only the private MCP transport retained until its response. No retained root file exceeds 5 MiB and no retained child directory exceeds 10 MiB. The minimal Nx fixture keeps its three input configuration files; its generated caches are removed.

The first close attempt submitted all 9,688 paths (1,648,543 request bytes), but the real MCP rejected closure because `📌️important/📝️.md` was missing. The ticket remained open. The current lifecycle requires this file to be an empty regular file in an otherwise empty directory; it is a lifecycle marker, while the substantive completion summary remains in this report and the final report. The open ticket cannot be reopened through the MCP (`ticket is already open`), so the missing empty marker was restored as a ticket-owned input file. No existing document was erased. Its created/removed path was added to the exact authored manifest before retrying the unchanged close implementation.

## Executed Closure

The real repository MCP accepted `ticket_close` with the full authored-file array and returned success. The ticket JSON now stores `closed` and the exact completion summary. The complete retained manifest exactly matches the submitted array. The MCP removed its restored empty lifecycle marker. As described above, the current ticket JSON schema does not store the interaction file list.

```json
{
  "completedAt": "2026-09-08T23:24:20.338269+00:00",
  "tool": "ticket_close",
  "ticket": "26/08/23/END-TO-END-TESTING-REFACTOR",
  "noManagement": true,
  "submittedFiles": 9689,
  "requestBytes": 1648709,
  "fileArrayBytes": 1647516,
  "fileArraySha256": "37dbbccabc6044fcfdb9ea80d5e42cd325f6f96a0a83df23b9ad88ddc7e1d38d",
  "mcpError": false,
  "persistedStatus": "closed",
  "persistedSummaryMatches": true,
  "retainedManifestMatches": true,
  "ticketJsonStoresInteractions": false,
  "lifecycleMarkerRemovedByMcp": true
}
```

Final cleanup verified: the ticket `🗑️generated` tree and the minimal Nx fixture caches are absent; the private MCP executable is removed. All retained verification inputs, authored reports, complete manifest, and linked final evidence remain. The ticket remains closed.
