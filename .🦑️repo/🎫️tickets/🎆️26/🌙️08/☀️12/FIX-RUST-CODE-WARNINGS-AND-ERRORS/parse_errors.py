import re
import sys

log_path = "/Users/ueli/.gemini/antigravity/brain/6a14394e-3d7b-4b89-a759-04b092bb88c3/.system_generated/tasks/task-29.log"

with open(log_path, "r", encoding="utf-8", errors="ignore") as f:
    lines = f.readlines()

errors = []
warnings = []

current_item = []
current_type = None

for line in lines:
    if line.startswith("error[E") or line.startswith("error:"):
        if current_item and current_type:
            if current_type == "error":
                errors.append("".join(current_item))
            else:
                warnings.append("".join(current_item))
        current_item = [line]
        current_type = "error"
    elif line.startswith("warning:"):
        if current_item and current_type:
            if current_type == "error":
                errors.append("".join(current_item))
            else:
                warnings.append("".join(current_item))
        current_item = [line]
        current_type = "warning"
    elif current_item:
        if line.startswith("     Checking ") or line.startswith("    Compiling "):
            if current_type == "error":
                errors.append("".join(current_item))
            else:
                warnings.append("".join(current_item))
            current_item = []
            current_type = None
        else:
            current_item.append(line)

if current_item and current_type:
    if current_type == "error":
        errors.append("".join(current_item))
    else:
        warnings.append("".join(current_item))

print(f"Total Errors Found: {len(errors)}")
print(f"Total Warnings Found: {len(warnings)}")

# Output summary by error code / first line
error_summaries = {}
for e in errors:
    first_line = e.splitlines()[0] if e.splitlines() else e
    error_summaries[first_line] = error_summaries.get(first_line, 0) + 1

print("\n--- UNIQUE ERRORS ---")
for err_msg, count in sorted(error_summaries.items()):
    print(f"[{count}x] {err_msg}")

warning_summaries = {}
for w in warnings:
    first_line = w.splitlines()[0] if w.splitlines() else w
    warning_summaries[first_line] = warning_summaries.get(first_line, 0) + 1

print("\n--- TOP WARNINGS ---")
for warn_msg, count in sorted(warning_summaries.items(), key=lambda x: x[1], reverse=True)[:30]:
    print(f"[{count}x] {warn_msg}")
