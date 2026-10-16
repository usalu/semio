import os, re, subprocess, sys
root = "/Users/ueli/Documents/semio"
os.chdir(root)
files = subprocess.run(["git", "-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-co", "--exclude-standard", "-z", "--", *sys.argv[1:]], capture_output=True).stdout.decode().split("\0")
tok = re.compile(r'//[^\n]*|/\*.*?\*/|r(#*)"(?:.|\n)*?"\1|"(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)\'|#\[path\s*=\s*"([^"]+)"\s*\]|\bmod\s+([A-Za-z_][A-Za-z0-9_]*)\s*(\{|;)|(\{)|(\})', re.S)
bad = checked = 0
for f in files:
    if not f.endswith(".rs"):
        continue
    try:
        text = open(f, encoding="utf8").read()
    except Exception:
        continue
    if "#[path" not in text:
        continue
    d = os.path.dirname(f)
    stack = []
    pending = None
    for m in tok.finditer(text):
        if m.group(2) is not None:
            pending = m.group(2)
        elif m.group(3) is not None:
            name, end = m.group(3), m.group(4)
            if end == "{":
                stack.append(pending if pending is not None else name)
                pending = None
            else:
                if pending is not None:
                    mods = [s for s in stack if s is not None]
                    if mods:
                        checked += 1
                        naive = os.path.join(d, *mods, pending)
                        if not os.path.exists(naive):
                            bad += 1
                            print(f"{f}: mod {name} path {pending!r} inside {'/'.join(mods)}")
                pending = None
        elif m.group(5) is not None:
            stack.append(None)
        elif m.group(6) is not None:
            if stack:
                stack.pop()
print("checked", checked, "bad", bad)
