"""📋️ Prints the findings of a layout survey report: per case and screen whatever is wider than the page, scrolls
sideways, is cut or is a small target, and how the task rows are laid out. `python report_digest.py <label> [case]`."""
import io, json, sys

label = sys.argv[1] if len(sys.argv) > 1 else "survey"
only = sys.argv[2] if len(sys.argv) > 2 else ""
report = json.load(io.open(f"🗑️generated/{label}/report.json", encoding="utf-8"))
out = io.open(sys.stdout.fileno(), "w", encoding="utf-8", closefd=False)
for key, value in report.items():
    if not key.startswith(only):
        continue
    if isinstance(value, list):
        if value:
            out.write(f"{key}: {value}\n")
        continue
    findings = {name: value[name] for name in ("pageWiderBy", "outside", "sideways", "cut", "smallTargets", "rows") if value.get(name)}
    out.write(f"{key} [{value['viewport']}] {json.dumps(findings, ensure_ascii=False)}\n")
