"""📋 Prints the generator inventory (markdown table): script, purpose (first docstring line), writes to, status (guarded | native-additive | safe_run)."""
import ast, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SC = os.path.dirname(HERE)
guarded = set(open(HERE + "/guarded.txt").read().split())
rows = []
for root, dirs, files in os.walk(SC):
    dirs[:] = sorted(d for d in dirs if d not in ("__pycache__", "tools-hardening"))
    for f in sorted(files):
        if not f.endswith((".py", ".sh")):
            continue
        rel = os.path.relpath(os.path.join(root, f), SC)
        src = open(os.path.join(root, f), encoding="utf-8", errors="replace").read()
        doc = ""
        if f.endswith(".py"):
            try:
                doc = (ast.get_docstring(ast.parse(src)) or "").strip().split("\n")[0]
            except SyntaxError:
                pass
        writes = bool(re.search(r"write_text|\.write\(|open\([^)]*[\"'][wa]|F\.write|shutil|os\.(rename|remove)", src))
        native = "from common import" in src and "Files(" in src or "KFiles(" in src
        status = "guarded (TH_FORCE=1)" if rel in guarded else "native additive + --check (Files)" if native else "safe_run" if writes else "read-only"
        rows.append((rel, doc[:110].replace("|", "/"), status))
print("| script | purpose | status |\n|---|---|---|")
for r in rows:
    print("| `%s` | %s | %s |" % r)
