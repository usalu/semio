# MCP Spec & Repo Implementation Audit

**Spec Version**: 2026-07-28 (latest)  
**SDK Version**: 1.30.0 (LATEST_PROTOCOL_VERSION: 2025-11-25)  
**Repo Base**: /Users/ueli/Documents/semio

---

## A. Protocol Specification (2026-07-28)

### Initialize & Versioning
- **No handshake**: MCP 2026-07-28 is **stateless**. `initialize`/`notifications/initialized` removed.
- **Per-request metadata** in `_meta` fields:
  - `io.modelcontextprotocol/protocolVersion`: required on every request (e.g., `"2026-07-28"`)
  - `io.modelcontextprotocol/clientCapabilities`: required object
  - `io.modelcontextprotocol/clientInfo`: optional (name, version)
  - `io.modelcontextprotocol/serverInfo`: server may include in results
- **Version mismatch**: server returns `UnsupportedProtocolVersionError` (-32022) with `supported` array
- **New**: `server/discover` RPC for clients to probe version support before issuing other requests

### Tools Protocol

**tools/list** request/response:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list",
  "params": {
    "cursor": "optional-pagination-cursor",
    "_meta": { /* required */ }
  }
}
→ {
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "resultType": "complete",
    "tools": [
      {
        "name": "tool-name",
        "title": "Human Readable",
        "description": "...",
        "inputSchema": { "type": "object", ... },
        "outputSchema": { ... },
        "annotations": { /* optional */ },
        "icons": [ /* optional */ ]
      }
    ],
    "nextCursor": "pagination-cursor",
    "ttlMs": 300000,
    "cacheScope": "public",
    "_meta": { /* optional serverInfo */ }
  }
}
```

**tools/call** request/response:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "tool-name",
    "arguments": { /* any JSON */ },
    "inputResponses": { /* optional MRTR */ },
    "requestState": "opaque-string",
    "_meta": { /* required */ }
  }
}
→ {
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "resultType": "complete" | "input_required",
    "content": [
      {
        "type": "text" | "image" | "audio" | "resource_link" | "resource",
        "text": "...",
        "data": "base64",
        "mimeType": "...",
        "uri": "...",
        "annotations": { /* optional */ }
      }
    ],
    "structuredContent": { /* any JSON */ },
    "isError": false,
    "inputRequests": { /* for input_required */ },
    "requestState": "opaque-string",
    "_meta": { /* optional serverInfo */ }
  }
}
```

**Tool Names**:
- SHOULD be 1–128 chars, case-sensitive
- Allowed: A-Z a-z 0-9 `_` `-` `.`
- **NOT** spaces, commas, or special chars

**CallToolResult Fields**:
- `resultType` (required): `"complete"` or `"input_required"`
- `content` (array of ContentBlock): unstructured
  - `type` in {`text`, `image`, `audio`, `resource_link`, `resource`}
- `structuredContent` (any JSON): optional, per outputSchema
- `isError` (boolean): `true` signals tool execution failure (actionable); protocol errors (-32602 for unknown tool) are JSON-RPC errors, not tool results

**Errors**:
1. **Protocol errors** (JSON-RPC error response): unknown tool, malformed request, server errors → `-32602` (Invalid Params) or `-32603` (Internal Error)
2. **Tool execution errors** (inside successful result with `isError: true`): API failures, validation errors, business logic → include in `content` as text for LLM self-correction

### Resources Protocol

**resources/list** request/response:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "resources/list",
  "params": { "cursor": "...", "_meta": { /* required */ } }
}
→ {
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "resultType": "complete",
    "resources": [
      {
        "uri": "file:///path/to/file",
        "name": "display-name",
        "title": "Human Readable",
        "description": "...",
        "mimeType": "text/plain",
        "size": 1024,
        "icons": [ /* optional */ ],
        "annotations": { /* optional */ }
      }
    ],
    "nextCursor": "...",
    "ttlMs": 300000,
    "cacheScope": "public"
  }
}
```

**resources/templates/list** request/response:
```json
{
  "method": "resources/templates/list",
  "params": { "cursor": "...", "_meta": { /* required */ } }
}
→ {
  "result": {
    "resultType": "complete",
    "resourceTemplates": [
      {
        "uriTemplate": "file:///{path}",
        "name": "Project Files",
        "title": "📁 Project Files",
        "description": "...",
        "mimeType": "application/octet-stream",
        "icons": [ /* optional */ ],
        "annotations": { /* optional */ }
      }
    ],
    "nextCursor": "...",
    "ttlMs": 300000,
    "cacheScope": "public"
  }
}
```

**resources/read** request/response:
```json
{
  "method": "resources/read",
  "params": { "uri": "file:///...", "_meta": { /* required */ } }
}
→ {
  "result": {
    "resultType": "complete" | "input_required",
    "contents": [
      {
        "uri": "...",
        "mimeType": "...",
        "text": "...",
        "blob": "base64-data",
        "annotations": { /* optional */ }
      }
    ],
    "ttlMs": 60000,
    "cacheScope": "private",
    "inputRequests": { /* for input_required */ },
    "requestState": "opaque"
  }
}
```

**URI Templates**: RFC 6570 level (simple, reserved, label expansion; server-side `{id}` auto-completion via completion API)

**Resource Content**:
- Text: `{ "uri": "...", "mimeType": "...", "text": "..." }`
- Binary: `{ "uri": "...", "mimeType": "...", "blob": "base64" }`

**Subscriptions** (deprecated resources/subscribe; replaced by subscriptions/listen):
- Client sends `subscriptions/listen` with `resourceSubscriptions` filter
- Server delivers `notifications/resources/updated` on long-lived SSE stream with `io.modelcontextprotocol/subscriptionId` in `_meta`

**List Changed Notification**:
```json
{ "jsonrpc": "2.0", "method": "notifications/resources/list_changed" }
```

**Error Handling**:
- Resource not found: `-32602` (Invalid Params); clients SHOULD accept `-32002` for backwards compat with 2025-11-25

### Prompts Protocol

**prompts/list** request/response:
```json
{
  "method": "prompts/list",
  "params": { "cursor": "...", "_meta": { /* required */ } }
}
→ {
  "result": {
    "resultType": "complete",
    "prompts": [
      {
        "name": "code_review",
        "title": "Request Code Review",
        "description": "...",
        "arguments": [
          {
            "name": "code",
            "description": "...",
            "required": true
          }
        ],
        "icons": [ /* optional */ ]
      }
    ],
    "nextCursor": "...",
    "ttlMs": 600000,
    "cacheScope": "public"
  }
}
```

**prompts/get** request/response:
```json
{
  "method": "prompts/get",
  "params": {
    "name": "code_review",
    "arguments": { "code": "..." },
    "inputResponses": { /* optional MRTR */ },
    "requestState": "opaque",
    "_meta": { /* required */ }
  }
}
→ {
  "result": {
    "resultType": "complete" | "input_required",
    "description": "...",
    "messages": [
      {
        "role": "user" | "assistant",
        "content": {
          "type": "text" | "image" | "audio" | "resource_link" | "resource",
          "text": "...",
          "data": "base64",
          "mimeType": "...",
          "uri": "...",
          "annotations": { /* optional */ }
        }
      }
    ],
    "inputRequests": { /* for input_required */ },
    "requestState": "opaque"
  }
}
```

### Elicitation (Multi Round-Trip Requests - MRTR)

**elicitation/create** (embedded inside InputRequiredResult.inputRequests):
```json
{
  "method": "elicitation/create",
  "params": {
    "mode": "form" | "url",
    "message": "Please provide...",
    "requestedSchema": {
      "type": "object",
      "properties": { /* JSON Schema 2020-12, restricted */ },
      "required": [ /* required properties */ ]
    }
  }
}
```

**Restricted requestedSchema**:
- Top-level MUST be `type: "object"`
- MAY include `properties`, `required`, standard JSON Schema keywords
- No composition keywords (`oneOf`, `anyOf`, `allOf`, `not`)
- No `$ref`
- No `$defs` or external references

**Three Response Actions** (in inputResponses):
1. **`"accept"`**: `{ "action": "accept", "content": { /* matches requestedSchema */ } }`
2. **`"decline"`**: `{ "action": "decline" }`
3. **`"cancel"`**: `{ "action": "cancel" }`

### Streamable HTTP Transport

**Endpoint Contract**:
- Single POST endpoint (e.g., `https://example.com/mcp`)
- Each request is its own HTTP POST
- Server responds with either:
  - `Content-Type: application/json` (single JSON object result)
  - `Content-Type: text/event-stream` (SSE stream with notifications + final response)

**Required Headers** (sent by client on every POST):
- `MCP-Protocol-Version`: must match request body `_meta.io.modelcontextprotocol/protocolVersion` (e.g., `"2026-07-28"`)
- `Mcp-Method`: e.g., `"tools/call"`, `"resources/read"`
- `Mcp-Name`: e.g., tool name, resource URI (Base64-encoded if non-ASCII: `=?base64?{Value}?=`)
- `Accept`: MUST list both `application/json` and `text/event-stream`

**Custom Headers from Tool Parameters** (Streamable HTTP feature):
- Tool `inputSchema` properties can be marked with `x-mcp-header: "HeaderName"`
- Client mirrors parameter values into `Mcp-Param-HeaderName` headers
- Constraints: primitive types only (string, integer, boolean; NOT `number`), no control chars, case-insensitive unique, statically reachable (no `items`, `oneOf`, `$ref` in path)

**SSE Stream Behavior**:
- No resumability via `Last-Event-ID`; broken stream requires new request with new ID
- Server MAY send notifications (e.g., `notifications/progress`) before final response
- Server MUST NOT send independent JSON-RPC *requests* on SSE (no sampling/elicitation as separate requests; embedded in MRTR `InputRequiredResult` instead)
- Long-lived `subscriptions/listen` response opens separate SSE for opted-in change notifications

**Security**:
- Validate `Origin` header; reject with 403 if invalid (DNS rebinding protection)
- Bind to localhost (127.0.0.1) when running locally
- Implement proper authentication

**Notifications**:
- `notifications/subscriptions/acknowledged`: server acknowledges `subscriptions/listen` request
- `notifications/tools/list_changed`: tool list changed
- `notifications/resources/list_changed`: resource list changed
- `notifications/resources/updated`: specific resource URI changed (on listen stream)
- `notifications/prompts/list_changed`: prompt list changed
- `notifications/progress`: request-scoped progress (on request response stream only, not listen stream)
- `notifications/message`: logging/info messages

### Authorization (OAuth 2.1 for HTTP)

**Resource Server Requirements**:
- Implement OAuth 2.1 per draft-ietf-oauth-v2-1-13
- Implement RFC 9728 (OAuth 2.0 Protected Resource Metadata) at `.well-known/oauth-protected-resource`
- Validate access tokens per RFC 8707 (Resource Indicators)
- Validate `audience` claim matches resource URI
- Return 401 with `WWW-Authenticate: Bearer scope="..."` if scope insufficient

**Client Requirements**:
- Support both RFC 8414 (OAuth Server Metadata) and OpenID Connect Discovery 1.0
- Use Client ID Metadata Documents (HTTPS URL as client_id) per draft-ietf-oauth-client-id-metadata-document-00
- Implement PKCE (RFC 7636)
- Include `resource` parameter (RFC 8707) in auth + token requests (canonical server URI)
- Validate issuer (`iss` parameter) per RFC 9207 before redeeming code
- Use `Authorization: Bearer <token>` header on all resource requests; never in query string
- Refresh tokens if expired

**Bearer Token on Loopback**:
- Simple bearer token (no OAuth handshake) is NOT spec-compliant for HTTP transport
- Spec mandates full OAuth 2.1 flow for HTTP
- STDIO transport should retrieve credentials from environment, bypassing authorization spec

### JSON Schema Dialect

**Default**: JSON Schema 2020-12 (when `$schema` field absent)  
**Explicit**: Schemas MAY include `$schema` field to specify different dialect  
**Support**: Implementations MUST support 2020-12, SHOULD document others  
**Composition Keywords** (`anyOf`, `oneOf`, `allOf`, `if`/`then`/`else`, `$defs`): Implementations SHOULD apply resource bounds (max depth, subschema count, per-validation budget) to prevent DoS  
**`$ref` Resolution**: Implementations MUST NOT auto-dereference network URIs; opt-in fetch only with allowlist, timeouts, size limits

---

## B. Repository Implementation Facts

### 1. TypeScript SDK (node_modules/@modelcontextprotocol/sdk)

**Version**: 1.30.0

**Protocol Version Constants** (from dist/esm/types.js):
```typescript
export const LATEST_PROTOCOL_VERSION = '2025-11-25';  // Note: SDK lags spec (spec is 2026-07-28)
export const DEFAULT_NEGOTIATED_PROTOCOL_VERSION = '2025-03-26';
export const SUPPORTED_PROTOCOL_VERSIONS = [
  '2025-11-25',  // latest in SDK
  '2025-06-18',
  '2025-03-26',
  '2024-11-05',
  '2024-10-07'
];
```

**Implication**: The Rust gateway must support LATEST_PROTOCOL_VERSION 2025-11-25 at minimum for the conformance test client to negotiate. The spec (2026-07-28) is ahead of the SDK.

### 2. Go MCP Server (🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go)

**Location**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go:46354`

**CreateMcpServer signature**:
```go
func CreateMcpServer(kind McpClientKind, toolTimeout time.Duration) *server.MCPServer
```

**Capabilities Declared**:
```go
s := server.NewMCPServer(
  McpServerName(kind),
  "1.0.0",
  server.WithToolCapabilities(true),      // tools/list, tools/call
  server.WithPromptCapabilities(true),    // prompts/list, prompts/get
)
// (resources implicitly enabled via s.AddResource)
```

**Prompts Registered** (4 total):
- `"enhance"` → handleEnhancePrompt
- `"refactor"` → handleRefactorPrompt
- `"test"` → handleTestPrompt
- `"comply"` → handleComplyPrompt

**Resources Registered** (16 total):
- Static: `"repo://"`, `"repo://bundles"`, `"repo://folders"`, `"repo://files"`, `"repo://tickets"`, `"repo://goals"`, `"repo://policies"`, `"repo://statutes"`, `"repo://contributors"`, `"repo://checkpoints"`
- Templates: `"repo://bundle/{id}"`, `"repo://folder/{id}"`, `"repo://file/{id}"`, `"repo://sections/{id}"`, `"repo://section/{id}"`, `"repo://definitions/{id}"`, `"repo://definition/{id}"`, `"repo://ticket/{id}"`, `"repo://goal/{id}"`, `"repo://policy/{id}"`, `"repo://statute/{id}"`, `"repo://contributor/{id}"`, `"repo://checkpoint/{id}"`

**Tools Registered** (4 total):
- `"ticket_open"`: opens a new ticket (requires emoji, title, prompt, goal; optional: plan_id/spec_id per client kind, client, llm, effort, draft, parent, issue)
- `"ticket_close"`: closes ticket (requires summary; optional: files, path, title, no_management)
- `"ticket_reopen"`: reopens ticket (optional: path, prompt, llm, effort, client, title, draft, no_management; optional plan_id/spec_id per client)
- `"section_move"`: rename section in file (requires file, old_name, new_name)
- `"file_integrate"`: integrate source file into target file section (requires source, target_section, target_file; optional: target_parent_section)
- `"section_extract"`: extract section to new file (requires source_file, source_section, target_file)

**Tool Registration API Pattern** (mark3labs/mcp-go):
```go
s.AddTool(
  mcp.NewTool("tool-name",
    mcp.WithDescription("Human description"),
    mcp.WithString("param1", mcp.Required(), mcp.Description("...")),
    mcp.WithArray("files", mcp.Description("..."), mcp.WithStringItems()),
    mcp.WithBoolean("flag", mcp.Description("...")),
  ),
  wrapMcpToolHandler(toolTimeout, actualHandlerFunc),
)
```

**Resource Registration API Pattern**:
```go
// Static resource:
s.AddResource(
  mcp.NewResource("repo://", description, mcp.WithMIMEType("text/plain")),
  handlerFunc,
)
// Resource template:
s.AddResourceTemplate(
  mcp.NewResourceTemplate("repo://bundle/{id}", description),
  handlerFunc,
)
```

**Prompt Registration API Pattern**:
```go
s.AddPrompt(
  mcp.NewPrompt("enhance",
    mcp.WithPromptDescription("..."),
    mcp.WithArgument("prompt", mcp.ArgumentDescription("..."), mcp.RequiredArgument()),
  ),
  handleEnhancePrompt,
)
```

### 3. Root script.ts (📜️script.ts)

**DevScript.runMcp** (lines 544–569):
```typescript
private runMcp(segments: string[]): void {
  const a = segments[0];
  if (a === "engine") {
    runCmd("bun", [join(this.root, "compose", "client", "bin", "engine", "📜️script.ts"), "dev", "mcp"], { cwd: this.root, ...daemonBudgetOpts() });
    return;
  }
  if (a === "neo4j") {
    this.runMcpNeo4j(segments.slice(1));
    return;
  }
  if (a === "stdio") {
    this.runMcpStdioRepo(segments.slice(1));
    return;
  }
  const mode = a === "repo" ? "repo" : "default";
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  if (mode === "repo") {
    runCmd("npx", ["--yes", "@modelcontextprotocol/inspector", "--config", ".cursor/mcp.json", "--server", "repo"], {
      cwd: this.root,
      env: { ...process.env, HOST: host },
      ...daemonBudgetOpts(),
    });
    return;
  }
  runCmd("npx", ["--yes", "@modelcontextprotocol/inspector"], { cwd: this.root, ...daemonBudgetOpts() });
}
```

**POLICY_MCP_CONFIG_PATHS** (line 4809):
```typescript
const POLICY_MCP_CONFIG_PATHS = [
  ".cursor/mcp.json",
  ".mcp.json",
  ".vscode/mcp.json",
  ".windsurf/mcp.json",
  ".kiro/settings/mcp.json",
  ".codex/config.toml"
] as const;
```

**Policy Constraint** (lines 4813–4862):
> Every repo MCP server **MUST** use the cross-platform Bun bootstrap to ensure platform-specific binaries are built on demand before stdio is handed to the native MCP server.

**Validation Function**:
```typescript
function policyMcpRepoServerUsesBootstrap(entry: PolicyMcpServerEntry): boolean {
  if ((entry.type ?? "stdio") !== "stdio") return false;
  const cmd = (entry.command ?? "").trim();
  const args = entry.args ?? [];
  return cmd === "bun" && args[0] === "./📜️script.ts" && args[1] === "dev" && args[2] === "mcp" && args[3] === "stdio" && Boolean(args[4]);
}
```

**Constraint Summary**:
- `type`: must be `"stdio"`
- `command`: must be `"bun"`
- `args`: must be `["./📜️script.ts", "dev", "mcp", "stdio", "<kind>"]`
  - The 5th element (args[4], `<kind>`) is client name; must be non-empty truthy string
- **Passing constraint**: ✅ `["./📜️script.ts", "dev", "mcp", "stdio", "semio"]`
- **Failing constraint**: ❌ `["go", "run", "./main.go"]` (not Bun bootstrap)

### 4. MCP Configuration Files (All Locations)

#### `.mcp.json` (generic/fallback config)
```json
{
  "mcpServers": {
    "repo": {
      "type": "stdio",
      "command": "bun",
      "args": ["./📜️script.ts", "dev", "mcp", "stdio", "client"]
    },
    "neo4j-*": { /* 6 neo4j servers */ }
  }
}
```

#### `.cursor/mcp.json` (Cursor IDE specific)
```json
{
  "mcpServers": {
    "repo": {
      "type": "stdio",
      "command": "bun",
      "args": ["./📜️script.ts", "dev", "mcp", "stdio", "cursor"]
    },
    "neo4j-*": { /* 6 neo4j servers */ }
  }
}
```

#### `.vscode/mcp.json` (Copilot/VS Code specific)
```json
{
  "inputs": [],
  "servers": {
    "repo": {
      "type": "stdio",
      "command": "bun",
      "args": ["./📜️script.ts", "dev", "mcp", "stdio", "copilot"]
    },
    "neo4j-*": { /* 6 neo4j servers */ }
  }
}
```

#### `.windsurf/mcp.json` (Windsurf IDE specific)
```json
{
  "mcpServers": {
    "repo": {
      "type": "stdio",
      "command": "bun",
      "args": ["./📜️script.ts", "dev", "mcp", "stdio", "client"],
      "disabled": false
    },
    "neo4j-*": { /* 6 neo4j servers, all with "disabled": false */ }
  }
}
```

#### `.codex/config.toml` (Codex IDE specific)
```toml
[mcp_servers.repo]
command = "bun"
args = ["./📜️script.ts", "dev", "mcp", "stdio", "codex"]
enabled = true
cwd = "."

[mcp_servers.neo4j-*]
/* 6 neo4j servers */
```

**Client IDs in `repo` entry across configs**:
- `.mcp.json`: `"client"`
- `.cursor/mcp.json`: `"cursor"`
- `.vscode/mcp.json`: `"copilot"`
- `.windsurf/mcp.json`: `"client"` (note: different from cursor)
- `.codex/config.toml`: `"codex"`

---

## Implementation Checklist for the Rust Gateway

Every JSON-RPC 2.0 method and notification that must be implemented, with exact wire shape and REQUIRED vs OPTIONAL status.

### REQUIRED Methods (Core Protocol)

1. **`tools/list`** — REQUIRED
   - Request params: `{ "cursor"?: string, "_meta": { ... } }`
   - Response result: `{ "resultType": "complete", "tools": Tool[], "nextCursor"?: string, "ttlMs": number, "cacheScope": "public"|"private", "_meta"?: { ... } }`
   - Notifications: none
   - Supports MRTR: no

2. **`tools/call`** — REQUIRED
   - Request params: `{ "name": string, "arguments": object, "inputResponses"?: InputResponses, "requestState"?: string, "_meta": { ... } }`
   - Response result: `{ "resultType": "complete"|"input_required", "content": ContentBlock[], "structuredContent"?: any, "isError": boolean, "inputRequests"?: InputRequests, "requestState"?: string, "_meta"?: { ... } }`
   - Notifications: `notifications/progress` (optional, on response stream)
   - Supports MRTR: yes

3. **`resources/list`** — REQUIRED
   - Request params: `{ "cursor"?: string, "_meta": { ... } }`
   - Response result: `{ "resultType": "complete", "resources": Resource[], "nextCursor"?: string, "ttlMs": number, "cacheScope": "public"|"private", "_meta"?: { ... } }`
   - Notifications: none
   - Supports MRTR: no

4. **`resources/templates/list`** — REQUIRED
   - Request params: `{ "cursor"?: string, "_meta": { ... } }`
   - Response result: `{ "resultType": "complete", "resourceTemplates": ResourceTemplate[], "nextCursor"?: string, "ttlMs": number, "cacheScope": "public"|"private", "_meta"?: { ... } }`
   - Notifications: none
   - Supports MRTR: no

5. **`resources/read`** — REQUIRED
   - Request params: `{ "uri": string, "inputResponses"?: InputResponses, "requestState"?: string, "_meta": { ... } }`
   - Response result: `{ "resultType": "complete"|"input_required", "contents": ResourceContent[], "ttlMs": number, "cacheScope": "public"|"private", "inputRequests"?: InputRequests, "requestState"?: string, "_meta"?: { ... } }`
   - Notifications: `notifications/progress` (optional)
   - Supports MRTR: yes

6. **`prompts/list`** — REQUIRED
   - Request params: `{ "cursor"?: string, "_meta": { ... } }`
   - Response result: `{ "resultType": "complete", "prompts": Prompt[], "nextCursor"?: string, "ttlMs": number, "cacheScope": "public"|"private", "_meta"?: { ... } }`
   - Notifications: none
   - Supports MRTR: no

7. **`prompts/get`** — REQUIRED
   - Request params: `{ "name": string, "arguments"?: object, "inputResponses"?: InputResponses, "requestState"?: string, "_meta": { ... } }`
   - Response result: `{ "resultType": "complete"|"input_required", "description"?: string, "messages": PromptMessage[], "inputRequests"?: InputRequests, "requestState"?: string, "_meta"?: { ... } }`
   - Notifications: `notifications/progress` (optional)
   - Supports MRTR: yes

8. **`subscriptions/listen`** — REQUIRED (for server-initiated change notifications)
   - Request params: `{ "notifications": { "toolsListChanged"?: boolean, "promptsListChanged"?: boolean, "resourcesListChanged"?: boolean, "resourceSubscriptions"?: string[] }, "_meta": { ... } }`
   - Response result (initial): `{ "resultType": "complete", "_meta"?: { ... } }`
   - Then stream notifications: `{ "jsonrpc": "2.0", "method": "notifications/subscriptions/acknowledged", ... }`, followed by change notifications as they occur
   - Each notification carries `_meta.io.modelcontextprotocol/subscriptionId`
   - Notifications on stream: `notifications/tools/list_changed`, `notifications/prompts/list_changed`, `notifications/resources/list_changed`, `notifications/resources/updated`
   - Supports MRTR: no (but response is long-lived SSE stream)

9. **`server/discover`** — REQUIRED (new in 2026-07-28)
   - Request params: `{ "_meta": { ... } }`
   - Response result: `{ "resultType": "complete", "protocolVersion": string, "capabilities": ServerCapabilities, "serverInfo": Implementation, "_meta"?: { ... } }`
   - Returns supported protocol versions in `capabilities`, server identity in `serverInfo`
   - Notifications: none
   - Supports MRTR: no

### OPTIONAL Methods (Extensions)

10. **`sampling/createMessage`** — OPTIONAL (deprecated in 2026-07-28, but may be in request MRTR)
    - Appears only inside `InputRequiredResult.inputRequests` (MRTR pattern)
    - Request shape: `{ "method": "sampling/createMessage", "params": { "messages": Message[], "systemPrompt"?: string, "maxTokens": number, "modelPreferences"?: { "hints"?: { "name": string }[] }, "_meta": { ... } } }`
    - Response: `{ "role": "assistant", "content": ContentBlock, "model": string, "stopReason": string }`

11. **`roots/list`** — OPTIONAL (deprecated in 2026-07-28, but may be in request MRTR)
    - Appears only inside `InputRequiredResult.inputRequests` (MRTR pattern)
    - Request shape: `{ "method": "roots/list" }`
    - Response: `{ "roots": { "uri": string, "name"?: string }[] }`

### REQUIRED Notifications (Server → Client)

12. **`notifications/subscriptions/acknowledged`** — REQUIRED
    - Sent in response to `subscriptions/listen` request (on SSE stream)
    - Payload: `{ "jsonrpc": "2.0", "method": "notifications/subscriptions/acknowledged" }`

13. **`notifications/tools/list_changed`** — REQUIRED (when listChanged capability enabled)
    - Payload: `{ "jsonrpc": "2.0", "method": "notifications/tools/list_changed" }`
    - Sent when tool list changes; client should re-issue `tools/list`

14. **`notifications/resources/list_changed`** — REQUIRED (when listChanged capability enabled)
    - Payload: `{ "jsonrpc": "2.0", "method": "notifications/resources/list_changed" }`
    - Sent when resource list changes; client should re-issue `resources/list`

15. **`notifications/resources/updated`** — REQUIRED (when subscribe capability enabled)
    - Payload: `{ "jsonrpc": "2.0", "method": "notifications/resources/updated", "params": { "uri": string, "_meta": { "io.modelcontextprotocol/subscriptionId": string } } }`
    - Sent when specific resource URI content changes; client should re-issue `resources/read`

16. **`notifications/prompts/list_changed`** — REQUIRED (when listChanged capability enabled)
    - Payload: `{ "jsonrpc": "2.0", "method": "notifications/prompts/list_changed" }`
    - Sent when prompt list changes; client should re-issue `prompts/list`

17. **`notifications/progress`** — OPTIONAL (request-scoped)
    - Payload: `{ "jsonrpc": "2.0", "method": "notifications/progress", "params": { "progressToken"?: string, "progress": number, "total"?: number, "_meta": { "progressToken"?: string } } }`
    - Sent on response stream (SSE) before final response; relates to request with matching progressToken in request `_meta`

18. **`notifications/message`** — OPTIONAL (logging, deprecated in 2026-07-28)
    - Payload: `{ "jsonrpc": "2.0", "method": "notifications/message", "params": { "level": "debug"|"info"|"notice"|"warning"|"error", "logger"?: string, "text": string, "_meta"?: { ... } } }`
    - Only sent if client includes `io.modelcontextprotocol/logLevel` in request `_meta`

### Error Codes (JSON-RPC)

- `-32700`: Parse error
- `-32600`: Invalid Request
- `-32601`: Method not found
- `-32602`: Invalid params (includes resource not found, missing capability)
- `-32603`: Internal error
- `-32020`: HeaderMismatch (Streamable HTTP header/body mismatch)
- `-32021`: MissingRequiredClientCapability
- `-32022`: UnsupportedProtocolVersion

### Transport-Level (Streamable HTTP)

19. **HTTP POST to MCP endpoint** — REQUIRED
    - Headers: `MCP-Protocol-Version: {version}`, `Mcp-Method: {method}`, `Mcp-Name: {name/uri}` (when applicable), `Accept: application/json, text/event-stream`
    - Body: Single JSON-RPC request or notification
    - Response: `200 OK` with `application/json` (single response) or `Content-Type: text/event-stream` (SSE stream)
    - HTTP status codes: `200` (OK), `202` (Accepted for notifications), `400` (Bad Request with JSON-RPC error), `401` (Unauthorized), `403` (Forbidden), `404` (Not Found with JSON-RPC error)

20. **Origin validation** — REQUIRED
    - Server MUST validate `Origin` header on all incoming HTTP requests
    - Reject invalid origin with `403 Forbidden`
    - Prevents DNS rebinding attacks

21. **Localhost binding** — STRONGLY RECOMMENDED
    - Bind to `127.0.0.1` when running locally, not `0.0.0.0`

### Capabilities Declaration

Server MUST advertise:
```json
{
  "capabilities": {
    "tools": { "listChanged": true },
    "resources": { "listChanged": true, "subscribe": true },
    "prompts": { "listChanged": true }
  }
}
```

---

**End of Audit**  
All protocol versions, request/response shapes, field names, and constraints extracted from MCP 2026-07-28 specification.
