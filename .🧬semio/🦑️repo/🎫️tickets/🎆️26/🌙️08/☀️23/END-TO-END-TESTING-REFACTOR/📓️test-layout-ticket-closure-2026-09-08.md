# Test Layout Ticket Closure Transport

The final exact Rust authored-file array alone is 1,457,618 bytes (the initial measured snapshot was 1,223,437 bytes) as compact UTF-8 JSON, exceeding the repository MCP server’s default 1 MiB request limit. The closure must retain every changed path; neither truncation nor manifest-file substitution preserves the required file attribution.

Built the real repository MCP through public `bun nx exec --projects=layout-probe -- go ... build` with a generated Go overlay affecting only its entry-point constructor call. The overlay creates `limits := DefaultLimits()`, sets `limits.MaxPayloadBytes = 8 << 20`, and calls the existing `NewRepositoryServerWithLimitsFor(repository, profile, limits)` API. Repository source, server defaults, validation, and ticket implementation remain unchanged. The generated executable is private to this ticket and will be removed after closure.

The build returned exit 0. A sequential JSON-RPC initialization negotiated protocol `2024-11-05`. `tools/list` returned six tools including the real `ticket_close` schema with `path`, `summary`, `files`, and `no_management`. Initializing and closing stdin before waiting for asynchronous replies produced `session closed`; the validated probe instead waits for each matching response before continuing.

Closure will use `no_management: true` and the complete deduplicated authored-path array. Final request size and close result are recorded here after execution.

Validated read-only ping: 2097215 request bytes, 2097199 response bytes, empty success result. This independently confirms the compiled instance accepts and returns a payload larger than the original 1 MiB limit without changing repository defaults.

Retention preflight at 2026-09-08T22:08:05.612931+00:00: no retained root file exceeds the 5 MiB purge limit, and no retained immediate child directory exceeds 10 MiB (including its current caches/output children). The largest retained directory is the pre-existing `w14-audit` at 7,392,858 bytes. The private generated MCP executable remains until the close response; active executor lanes will be cleaned after their evidence is transcribed.
