"""Copy emoji-named source files to ASCII paths for inspection."""
import os
import shutil

teaching = "/home/deck/semio"
for name in os.listdir(teaching):
    if "teaching" in name or name.endswith("teaching"):
        teaching = os.path.join("/home/deck/semio", name)
print("TEACHING", teaching)

out = "/tmp/staffage-read"
os.makedirs(out, exist_ok=True)
roots = []
for dirpath, dirs, files in os.walk(teaching):
    dirs[:] = [d for d in dirs if d not in ("node_modules", ".git")]
    base = os.path.basename(dirpath)
    if base.endswith("catalog") and "check" in "".join(dirs):
        roots.append(dirpath)
    if "representation" in base and any(x.endswith(".ts") for x in files) and "modules" in dirpath:
        print("REP", dirpath, dirs)
print("ROOTS", roots)
for parent in roots:
    for root, ds, fs in os.walk(parent):
        for f in fs:
            if f.endswith(".ts"):
                src = os.path.join(root, f)
                rel = os.path.relpath(src, parent).replace(os.sep, "__")
                dst = os.path.join(out, "catalog__" + rel)
                shutil.copy(src, dst)
                print("COPIED", dst, os.path.getsize(dst))
