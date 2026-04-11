from pathlib import Path

p = Path(__file__).resolve().parent / "main.py"
lines = p.read_text(encoding="utf-8").splitlines(keepends=True)

idx = next(i for i, ln in enumerate(lines) if ln.strip() == "return _mcp_sessions[sid]")
end_sess = next(
    i
    for i, ln in enumerate(lines)
    if i > idx and ln.startswith("# #endregion") and "Session" in ln
)

GATHER = """

def _gather_workspace_payload(sess: Session, panel: str) -> dict[str, typing.Any]:
    \"\"\"Shallow snapshot of coda workspace for MCP Apps (mirrors desktop views, not full on-disk dumps).\"\"\"
    config = _get_coda_config()
    project = _get_project_config()
    status = sess.get_status()
    root = sess.project_root or _get_project_root()
    measures = config.get(\"measures\", [])
    property_kinds = config.get(\"property_kinds\", {})
    correlation = config.get(\"correlation\", {})
    properties = [
        _normalize_property_definition(p) if isinstance(p, dict) else p
        for p in config.get(\"properties\", [])
    ]
    frameworks = [_normalize_target_definition(t) for t in config.get(\"targets\", [])]
    platforms: list[typing.Any] = []
    if root:
        platforms_dir = root / \".coda\" / \"platforms\"
        if platforms_dir.is_dir():
            for pf in platforms_dir.iterdir():
                if pf.is_file() and pf.suffix == \".json\":
                    try:
                        platforms.append(json.loads(pf.read_text(encoding=\"utf-8\")))
                    except Exception:
                        continue
    if not platforms:
        platforms = list(config.get(\"platforms\", []) or [])

    run_summary: dict[str, typing.Any] | None = None
    iteration_summary: dict[str, typing.Any] | None = None
    report_summary: dict[str, typing.Any] | None = None
    breachs_shallow: list[typing.Any] = []
    translations: dict[str, typing.Any] = {}

    if root:
        run_dir = sess.run_dir or _get_latest_run(root)
        if run_dir and run_dir.is_dir():
            run_json = run_dir / \"run.json\"
            run_summary = (
                json.loads(run_json.read_text(encoding=\"utf-8\"))
                if run_json.is_file()
                else {\"id\": run_dir.name}
            )
        iter_dir = sess.iteration_dir or (
            _get_latest_iteration(run_dir) if run_dir else None
        )
        if iter_dir and iter_dir.is_dir():
            iter_json = iter_dir / \"iteration.json\"
            iteration_summary = (
                json.loads(iter_json.read_text(encoding=\"utf-8\"))
                if iter_json.is_file()
                else {\"index\": iter_dir.name}
            )
            agg = iter_dir / \"targets\" / \"report.json\"
            if agg.is_file():
                report_full = json.loads(agg.read_text(encoding=\"utf-8\"))
                breachs = report_full.get(\"breachs\") or []
                validations = report_full.get(\"validations\")
                report_summary = {
                    \"summary_keys\": list(report_full.keys())[:24],
                    \"breachs_count\": len(breachs) if isinstance(breachs, list) else 0,
                    \"validations_count\": len(validations)
                    if isinstance(validations, list)
                    else 0,
                }
                if isinstance(breachs, list):
                    breachs_shallow = breachs[:80]
            targets_dir = iter_dir / \"targets\"
            if targets_dir.is_dir():
                for tdir in targets_dir.iterdir():
                    if not tdir.is_dir():
                        continue
                    tr = tdir / \"translation.json\"
                    translations[tdir.name] = {
                        \"has_translation\": tr.is_file(),
                        \"path\": str(tr) if tr.is_file() else None,
                    }

    return {
        \"kind\": \"coda-workspace\",
        \"panel\": panel,
        \"session\": status,
        \"project\": project,
        \"measures\": measures,
        \"property_kinds\": property_kinds,
        \"correlation\": correlation,
        \"properties\": properties,
        \"frameworks\": frameworks,
        \"platforms\": platforms,
        \"run\": run_summary,
        \"iteration\": iteration_summary,
        \"report\": report_summary,
        \"breachs_shallow\": breachs_shallow,
        \"translations\": translations,
    }


"""

RES = """

@mcp.resource(
    _WORKSPACE_APP_URI,
    name=\"coda workspace\",
    description=\"Interactive coda ACC workspace using elements/ui primitives (dashboard, config, runs, report).\",
    mime_type=\"text/html;profile=mcp-app\",
    meta=_mcp_app_html_resource_meta(),
)
def coda_workspace_viewer_resource() -> str:
    \"\"\"Serve the MCP App HTML shell built from coda/assistant mcp-app (elements/ui).\"\"\"
    return _build_mcp_app_html(panel=\"dashboard\")


"""

text = p.read_text(encoding="utf-8")
if "def _gather_workspace_payload" not in text:
    lines.insert(end_sess, GATHER)
    text = "".join(lines)
    p.write_text(text, encoding="utf-8")
    lines = text.splitlines(keepends=True)

text = p.read_text(encoding="utf-8")
if "def coda_workspace_viewer_resource" not in text:
    lines = text.splitlines(keepends=True)
    mi = next(i for i, ln in enumerate(lines) if '@mcp.resource("coda://measures")' in ln)
    lines.insert(mi, RES)
    p.write_text("".join(lines), encoding="utf-8")

print("done")
