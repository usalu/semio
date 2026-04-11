# Temporary script; remove after use.
path = r"c:\git\semio\semio\py\main.py"
with open(path, "r", encoding="utf-8") as f:
    lines = f.readlines()
# Line 8993 (1-based) -> index 8992
i = 8992
if i < len(lines):
    lines[i] = '    """Validate removed/updated/added guids for one collection diff; heal trims invalid ops when ctx["heal"]."""\n'
# Fix broken else around 9047 (search)
for j, L in enumerate(lines):
    if "first = False else:" in L:
        lines[j] = "                            first = False\n"
        lines.insert(j + 1, "                        else:\n")
        break
with open(path, "w", encoding="utf-8", newline="") as f:
    f.writelines(lines)
print("ok")
