import os, zipfile, shutil
OUT = r"C:\git-semio-rescue-archive"
os.makedirs(OUT, exist_ok=True)
def zipdirs(name, roots, skip=("node_modules", ".git", "__pycache__")):
    zf = zipfile.ZipFile(os.path.join(OUT, name), "w", zipfile.ZIP_DEFLATED)
    n = 0
    for root in roots:
        base = os.path.dirname(root)
        for r, ds, fs in os.walk(root):
            ds[:] = [d for d in ds if d not in skip]
            for f in fs:
                p = os.path.join(r, f)
                try: zf.write(p, os.path.relpath(p, base)); n += 1
                except Exception as e: print("ERR", p.encode("ascii","replace"), e)
    zf.close(); print(name, n, os.path.getsize(os.path.join(OUT, name)))
zipdirs("temp.zip", [r"C:\git\semio\temp"])
zipdirs("launch-kits.zip", [r"C:\git\semio\staffage-launch-kit", r"C:\git\semio\teaching-morph-preview-launch-kit", r"C:\git\semio\puzzle-artifact-editors-windows-kit", r"C:\git\semio\holder"])
shutil.copy2(r"C:\git\semio\.claude\settings.local.json", os.path.join(OUT, "claude-settings.local.json"))
