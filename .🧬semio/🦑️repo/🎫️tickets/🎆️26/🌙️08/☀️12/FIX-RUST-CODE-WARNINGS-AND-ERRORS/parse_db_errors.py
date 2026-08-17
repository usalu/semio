import re

log_path = ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/FIX-RUST-CODE-WARNINGS-AND-ERRORS/db_check.log"

with open(log_path, "r", encoding="utf-8", errors="ignore") as f:
    text = f.read()

# Summarize unique error messages and files
file_counts = {}
error_types = {}

for err in re.finditer(r"error\[E\d+\]:\s*(.*?)\n\s+-->\s*(.*?):(\d+):(\d+)", text):
    msg, path, line, col = err.groups()
    file_counts[path] = file_counts.get(path, 0) + 1
    err_short = msg[:60]
    error_types[err_short] = error_types.get(err_short, 0) + 1

print("\nFiles with errors:")
for path, count in sorted(file_counts.items(), key=lambda x: -x[1]):
    print(f"  {count:3d} errors in {path}")

print("\nError types summary:")
for err, count in sorted(error_types.items(), key=lambda x: -x[1]):
    print(f"  {count:3d} x {err}")
