import os
import re
import sys

ROOT = sys.argv[1]
APPLY = len(sys.argv) > 2 and sys.argv[2] == "apply"
PATTERN = re.compile(r"^(\s*#\[derive\()([^\n]*)(\)\]\s*)$")
TOKEN = "semio_framework_value::RetireOwned"
changed_files = 0
changed_lines = 0
for base, dirs, files in os.walk(ROOT):
    dirs[:] = [d for d in dirs if not d.startswith("\U0001F9EA") and not d.startswith("\U0001F52E") and d != "target"]
    for name in files:
        if not name.endswith(".rs"):
            continue
        path = os.path.join(base, name)
        with open(path, encoding="utf-8", newline="") as handle:
            text = handle.read()
        out = []
        touched = 0
        for line in text.split("\n"):
            carriage = line.endswith("\r")
            body = line[:-1] if carriage else line
            match = PATTERN.match(body)
            if match and "ToValue" in match.group(2) and "FromValue" in match.group(2) and "RetireOwned" not in match.group(2):
                body = match.group(1) + TOKEN + ", " + match.group(2) + match.group(3)
                touched += 1
            out.append(body + ("\r" if carriage else ""))
        if touched:
            changed_files += 1
            changed_lines += touched
            if APPLY:
                with open(path, "w", encoding="utf-8", newline="") as handle:
                    handle.write("\n".join(out))
print("files", changed_files, "lines", changed_lines, "applied" if APPLY else "dry-run")
