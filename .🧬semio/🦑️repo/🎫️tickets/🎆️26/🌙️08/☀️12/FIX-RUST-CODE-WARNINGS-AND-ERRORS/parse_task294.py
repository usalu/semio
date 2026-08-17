import sys

log_path = "/Users/ueli/.gemini/antigravity/brain/6a14394e-3d7b-4b89-a759-04b092bb88c3/.system_generated/tasks/task-294.log"

with open(log_path, "r", encoding="utf-8", errors="ignore") as f:
    lines = f.readlines()

errors = []
current_item = []

for line in lines:
    if line.startswith("error[E") or line.startswith("error:"):
        if current_item:
            errors.append("".join(current_item))
        current_item = [line]
    elif current_item:
        if line.startswith("     Checking ") or line.startswith("    Compiling ") or line.startswith("warning:"):
            errors.append("".join(current_item))
            current_item = []
        else:
            current_item.append(line)

if current_item:
    errors.append("".join(current_item))

print(f"Total Errors in Task 294: {len(errors)}")
print("\n--- FIRST 15 ERRORS ---")
for i, e in enumerate(errors[:15]):
    print(f"=== Error {i+1} ===")
    print(e[:600])
