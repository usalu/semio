"""🔎 Audit referenced globals in changed section scene modules against actual imports."""

from pathlib import Path
import builtins
import importlib
import re
import symtable
import sys

TICKET = Path(__file__).resolve().parent
WORKSPACE = next(parent for parent in TICKET.parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(WORKSPACE))
ENERGY = WORKSPACE / "🎓️teaching/🏛️architecture/⚡️energy"
text = (TICKET / "section-consumers.md").read_text(encoding="utf-8")
paths = [ENERGY / name for name in re.findall(r"^- `([^`]+/🐍️\.py)`", text, re.MULTILINE)]
paths.append(ENERGY / "🏗️authoring/🎨️visuals/🐍️.py")
findings = []
for path in paths:
    name = ".".join(path.relative_to(WORKSPACE).with_suffix("").parts)
    module = importlib.import_module(name)
    known = set(vars(module)) | set(vars(builtins))
    pending = list(symtable.symtable(path.read_text(encoding="utf-8"), str(path), "exec").get_children())
    while pending:
        scope = pending.pop()
        pending.extend(scope.get_children())
        for symbol in scope.get_symbols():
            if symbol.is_global() and symbol.is_referenced() and symbol.get_name() not in known:
                findings.append((path.relative_to(ENERGY).as_posix(), scope.get_name(), scope.get_lineno(), symbol.get_name()))
lines = ["# Changed Section Source Global Audit", "", f"Read-only symbol-table inspection covered {len(paths)} changed scene/shared modules against their actual imported globals and Python builtins.", "",
         "This catches removed drawing constants still needed by unrelated explanatory helpers. It is an additional bounded source audit, not a general claim of static correctness for the repository.", ""]
if findings:
    lines += ["| Source | Scope | Scope Line | Unresolved Global |", "|---|---|---:|---|"]
    lines += [f"| {path} | {scope} | {line} | {name} |" for path, scope, line, name in sorted(set(findings))]
else:
    lines.append("No unresolved referenced global was found in the changed modules.")
(TICKET / "sections-globals.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
print("[DEBUG] Checked " + str(len(paths)) + " modules; unresolved global references: " + str(len(findings)))
for row in sorted(set(findings)):
    print(row)
