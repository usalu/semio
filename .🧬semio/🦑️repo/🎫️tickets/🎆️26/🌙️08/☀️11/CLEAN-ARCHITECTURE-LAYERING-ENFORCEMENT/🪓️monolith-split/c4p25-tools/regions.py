"""🧭 c4p9 region lister: top-level `//#region` spans (start, end, name, lines) of a file, read only. usage: regions.py <file>"""
import re, sys
ls = open(sys.argv[1], encoding="utf-8").read().split("\n")
stack = []
for i, l in enumerate(ls, 1):
    m = re.match(r"^\s*//\s?#region\s*(.*)$", l)
    if m:
        stack.append((i, m.group(1).strip()))
        continue
    m = re.match(r"^\s*//\s?#endregion\s*(.*)$", l)
    if m and stack:
        s, n = stack.pop()
        print(f"{'  ' * len(stack)}{s:5}-{i:5} {i - s + 1:5} {n}")
if stack:
    print("UNCLOSED", stack)
