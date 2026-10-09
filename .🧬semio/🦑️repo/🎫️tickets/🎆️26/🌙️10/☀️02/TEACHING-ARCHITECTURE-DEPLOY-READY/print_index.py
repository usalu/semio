import os

base = r"C:\git\semio"
teaching = next(path for path in os.listdir(base) if path.endswith("teaching"))
root = os.path.join(base, teaching)
html = None
for dirpath, dirs, files in os.walk(root):
    if "index.html" in files and "dist" in dirpath and "typescript" in dirpath and "pages" not in dirpath:
        html = os.path.join(dirpath, "index.html")
        break
print(html.encode("unicode_escape").decode() if html else "missing")
if html:
    print(open(html, encoding="utf-8").read())
