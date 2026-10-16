"""🚩️ Flags the classified references that still reach a moved name through its old framework path or an unproven glob."""
import json, sys, re
report = json.load(open(sys.argv[1]))
OK_ROOT = re.compile(r"(^|::)(semio_framework_tool_run|history_edit|tool_run|onboarding)$")
for f, hits in report.items():
    imported_ok = {h[1] for h in hits if h[2] == "use" and (h[3].startswith("semio_framework_tool_run") or re.search(r"(^|::)(history_edit|tool_run|onboarding)(::|$)", h[3]) or re.search(r"(semio_framework::|crate::)(history_edit|tool_run|onboarding)", h[4]))}
    flagged = []
    for line, name, kind, root, body in hits:
        if kind == "path":
            if not OK_ROOT.search(root):
                flagged.append((line, name, f"path {root}"))
        elif kind == "use":
            ok = root.startswith("semio_framework_tool_run") or re.search(r"(semio_framework::|crate::|^)(history_edit|tool_run|onboarding)\b", body)
            if not ok:
                flagged.append((line, name, f"use {root} :: {body[:90]}"))
        else:
            if name not in imported_ok:
                flagged.append((line, name, "bare"))
    if flagged:
        print(f)
        seen = {}
        for line, name, why in flagged:
            seen.setdefault(why, []).append(f"{name}@{line}")
        for why, names in seen.items():
            print("   ", why[:120], "->", ", ".join(names)[:400])
