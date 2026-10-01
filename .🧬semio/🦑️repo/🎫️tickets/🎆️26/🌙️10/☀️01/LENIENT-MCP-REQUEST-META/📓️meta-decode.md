# Lenient MCP Request Meta

`DecodeParams` still rejects unknown fields. `_meta` is the exception: MCP defines it as an open map, and Claude Code sends keys besides `progressToken` (for example `claudecode/toolUseId`).

`ListMeta` is now `map[string]json.RawMessage`. A map accepts every key even when the parent decoder uses `DisallowUnknownFields`. `ProgressToken()` still reads `progressToken`. `ListParams` carries `_meta` too, so list requests follow the same rule. `tools/call`, `resources/read`, and `prompts/get` already had the field.

Sibling fields outside `_meta` stay strict. A non-object `_meta` still fails.

`TestOpenRequestMetaAdmitsClientKeys` passes: a `tools/call` with `_meta: {"progressToken":1,"claudecode/toolUseId":"x"}` succeeds and the progress notification keeps the numeric token. The same object is accepted on `resources/read`, `prompts/get`, and the four list methods. The quick suite for `repo-mcp` passed.

The binary is rebuilt with `bun nx run repo-mcp:build`, which writes `.🧬semio/🦑️repo/⚡️cache/🗃️bin/repo`. A stdio session against that binary accepted `tools/call` with the same `_meta`, reported `"progressToken":1`, and reached `ticket_open` (`invalid ticket_open arguments` for empty arguments). An unknown sibling field still returns `invalid tool params`.
