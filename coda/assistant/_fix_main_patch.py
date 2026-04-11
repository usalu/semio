"""One-off: rewrite mangled top of main.py. Delete after success."""
from __future__ import annotations

from pathlib import Path

p = Path(__file__).resolve().parent / "main.py"
lines = p.read_text(encoding="utf-8").splitlines(keepends=True)

# Find indices (0-based)
start = next(i for i, ln in enumerate(lines) if ln.startswith("mcp = FastMCP"))
end = next(i for i, ln in enumerate(lines) if "# #region" in ln and "Helpers" in ln)

new_block = '''mcp = FastMCP("coda", json_response=True)

_CODA_ROOT = Path(__file__).resolve().parent
_CODA_JSON_PATH = _CODA_ROOT / "coda.json"
_PROJECT_ENV = "CODA_PROJECT"
_PROPERTY_KIND_MEASURE_KINDS = {
    "number": ["increase", "decrease"],
    "object": ["add", "remove"],
    "array": ["append", "remove_item"],
    "level": ["raise", "lower"],
    "category": ["include", "exclude"],
}

# #endregion Imports

# #region CodaMcpAppRuntime
# MCP App HTTP helpers share the streamable-http port; stdio mode omits fetchUrl on tool payloads.

CODA_HTTP_PORT = int(os.environ.get("CODA_HTTP_PORT", "8080"))
_HTTP_PAYLOADS_ENABLED = False
_SIDECAR_MODE = False

_mcp_app_payloads: collections.OrderedDict[str, dict[str, typing.Any]] = collections.OrderedDict()
_MCP_APP_PAYLOADS_MAX_SIZE = 100

_WORKSPACE_APP_URI = "ui://coda/workspace"
_WORKSPACE_APP_META: dict[str, typing.Any] = {
    "ui": {"resourceUri": _WORKSPACE_APP_URI},
    "ui/resourceUri": _WORKSPACE_APP_URI,
}


def _emit_event(event: str, data: dict[str, typing.Any]) -> None:
    """Emit a sidecar event line when running under Electron stdio (ignored in MCP-only processes)."""
    if not _SIDECAR_MODE:
        return
    _write_stdout(
        {"id": None, "event": event, "data": data, "timestamp": time.time()}
    )


def _mcp_app_html_resource_meta() -> dict[str, typing.Any]:
    """Resource _meta for MCP App HTML: hosts apply _meta.ui.csp to the sandbox."""
    origins = [
        f"http://127.0.0.1:{CODA_HTTP_PORT}",
        f"http://localhost:{CODA_HTTP_PORT}",
        f"http://[::1]:{CODA_HTTP_PORT}",
        f"ws://127.0.0.1:{CODA_HTTP_PORT}",
        f"ws://localhost:{CODA_HTTP_PORT}",
    ]
    csp = {"connectDomains": origins, "resourceDomains": origins}
    return {"ui": {"csp": csp}, "ui/csp": csp}


def _build_mcp_app_html(*, panel: str = "dashboard") -> str:
    """Load the single-file MCP App HTML bundle; swap initial panel for the workspace shell."""
    app_html_path = Path(__file__).resolve().parent / "dist" / "mcp-app.html"
    if app_html_path.is_file():
        html = app_html_path.read_text(encoding="utf-8")
    else:
        return """<!doctype html><html><body><p>MCP App not built. Run: npm run build:mcp-app in coda/assistant</p></body></html>"""
    if "data-coda-panel=" in html:
        html = re.sub(
            r'data-coda-panel="[^"]*"',
            f'data-coda-panel="{panel}"',
            html,
            count=1,
        )
    return html


def _mcp_app_csp_value() -> str:
    """CSP header matching semio engine MCP apps (iframe-friendly, allows wasm for elements/ui)."""
    return (
        "default-src 'self' 'unsafe-inline'; "
        "script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' blob:; "
        "frame-ancestors *; connect-src * data: blob:; img-src * data: blob:; worker-src blob:;"
    )


def _as_mcp_app_tool_result(
    payload: dict[str, typing.Any], *, is_error: bool = False
) -> CallToolResult:
    """Return tools/call payload with optional fetchUrl when HTTP server is up."""
    token = uuid.uuid4().hex
    _mcp_app_payloads[token] = payload
    while len(_mcp_app_payloads) > _MCP_APP_PAYLOADS_MAX_SIZE:
        _mcp_app_payloads.popitem(last=False)
    fetch_url: str | None = None
    if _HTTP_PAYLOADS_ENABLED:
        fetch_url = f"http://127.0.0.1:{CODA_HTTP_PORT}/app/payload/{token}"
        payload = {**payload, "fetchUrl": fetch_url}
    text = json.dumps(payload)
    hint: dict[str, typing.Any] = {"panel": payload.get("panel"), "kind": payload.get("kind")}
    if fetch_url:
        hint["fetchUrl"] = fetch_url
    return CallToolResult(
        content=[
            TextContent(type="text", text=json.dumps(hint)),
            TextContent(type="text", text=text),
            EmbeddedResource(
                type="resource",
                resource=TextResourceContents(
                    uri="coda://mcp-app/tool-payload",
                    mimeType="application/json",
                    text=text,
                ),
            ),
        ],
        structuredContent=payload,
        isError=is_error,
    )


# #endregion CodaMcpAppRuntime

'''

out = lines[:start] + [new_block] + lines[end:]
p.write_text("".join(out), encoding="utf-8")
print("rewrote", start, "to", end, "lines")
