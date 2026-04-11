from pathlib import Path

p = Path(__file__).resolve().parent / "main.py"
t = p.read_text(encoding="utf-8")

if "\nimport logging\n" not in t:
    t = t.replace("import json\n", "import json\nimport logging\n", 1)

TOOL = """

@mcp.tool(meta=_WORKSPACE_APP_META)
def show_coda_workspace(ctx: Context, panel: str = \"dashboard\") -> CallToolResult:
    \"\"\"Show the coda workspace in an MCP App. Panels: dashboard, config, runs, report, translations, actions, events.\"\"\"
    p = (panel or \"dashboard\").strip().lower()
    allowed = {
        \"dashboard\",
        \"config\",
        \"runs\",
        \"report\",
        \"translations\",
        \"actions\",
        \"events\",
    }
    if p not in allowed:
        p = \"dashboard\"
    try:
        return _as_mcp_app_tool_result(
            _gather_workspace_payload(_get_mcp_session(ctx), p)
        )
    except Exception as e:
        return _as_mcp_app_tool_result({\"error\": str(e)}, is_error=True)


"""

TOOLS_END = "# #endregion \U0001f941Tools\n"
if "def show_coda_workspace" not in t:
    if TOOLS_END not in t:
        raise SystemExit("Tools endregion not found")
    t = t.replace(TOOLS_END, TOOL + TOOLS_END, 1)

HTTP = """

# #region CodaHttp
# Composite HTTP server: MCP streamable-http at /mcp plus MCP App static payload routes.


@contextlib.asynccontextmanager
async def _coda_http_lifespan(app):
    \"\"\"Run the FastMCP session manager for the combined Starlette app.\"\"\"
    async with mcp.session_manager.run():
        yield


mcp.settings.streamable_http_path = \"/\"


async def _http_app_payload(request):
    \"\"\"Return a stored MCP App JSON payload by token.\"\"\"
    token = request.path_params[\"token\"]
    payload = _mcp_app_payloads.get(token)
    if payload is None:
        return JSONResponse({\"error\": \"Payload not found\"}, status_code=404)
    return JSONResponse(payload, headers={\"Access-Control-Allow-Origin\": \"*\"})


async def _http_app_workspace(request):
    \"\"\"Serve the inlined MCP App HTML; optional ?panel= query mirrors desktop pages.\"\"\"
    panel = request.query_params.get(\"panel\") or \"dashboard\"
    return HTMLResponse(
        _build_mcp_app_html(panel=panel),
        headers={\"Content-Security-Policy\": _mcp_app_csp_value()},
    )


_coda_http_app = Starlette(
    lifespan=_coda_http_lifespan,
    routes=[
        Route(\"/app/payload/{token}\", _http_app_payload),
        Route(\"/app/workspace\", _http_app_workspace),
        Route(\"/app/mcp-app\", _http_app_workspace),
    ],
)
_coda_http_app.mount(\"/mcp\", mcp.streamable_http_app())


# #endregion CodaHttp

"""

SIDE_MAIN = "# #endregion \U0001f9f1Sidecar\n\n# #region \U0001f43cMain\n"
if "_coda_http_app" not in t:
    if SIDE_MAIN not in t:
        raise SystemExit("Sidecar/Main boundary not found")
    t = t.replace(
        SIDE_MAIN,
        "# #endregion \U0001f9f1Sidecar\n" + HTTP + "\n# #region \U0001f43cMain\n",
        1,
    )

if "global _SIDECAR_MODE" not in t:
    lines = t.splitlines(keepends=True)
    for i, ln in enumerate(lines):
        if ln.startswith("def _run_sidecar()"):
            body = i + 1
            while body < len(lines) and '"""' not in lines[body]:
                body += 1
            body += 1
            while body < len(lines) and '"""' not in lines[body]:
                body += 1
            ins = body + 1
            if ins < len(lines) and "global _SIDECAR_MODE" not in lines[ins]:
                lines.insert(ins, "    global _SIDECAR_MODE\n")
                lines.insert(ins + 1, "    _SIDECAR_MODE = True\n")
            break
    t = "".join(lines)

p.write_text(t, encoding="utf-8")
print("patch2 ok")
