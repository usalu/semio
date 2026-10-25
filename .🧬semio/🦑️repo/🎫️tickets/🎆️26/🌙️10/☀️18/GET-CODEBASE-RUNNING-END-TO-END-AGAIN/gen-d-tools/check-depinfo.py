"""Independent oracle: every workspace file rustc read for a cargo invocation is inside the declared selection.
usage: python3 check-depinfo.py <cargo --message-format=json log> <selected.json (files)> <derived.json (patterns)> [own output prefix...]
The compiler-artifact rows name each unit's deps/<name>-<hash> output; its .d dep-info file lists every source rustc read (modules, include_str!, include_bytes!, build scripts).
Reads under the own output prefixes are written, not read (the seeded include of the mirror), reads outside the workspace (registry, toolchain) are the lock/toolchain identity, not files of the key."""
import fnmatch, json, os, re, sys
root = "/Users/ueli/Documents/semio/"
log, selected, derived = sys.argv[1:4]; own = sys.argv[4:]
files = set(json.load(open(selected, encoding="utf-8"))["files"])
patterns = [p.replace("{workspaceRoot}/", "") for p in json.load(open(derived, encoding="utf-8"))["patterns"] if not p.startswith("!")]
units = []
for line in open(log, encoding="utf-8", errors="replace"):
    line = line.strip()
    if not line.startswith("{"): continue
    try: row = json.loads(line)
    except Exception: continue
    if row.get("reason") != "compiler-artifact": continue
    for name in row.get("filenames", []): units.append(name)
reads = {}
for name in units:
    directory, base = os.path.split(name)
    stem = re.sub(r"^lib", "", base); stem = re.sub(r"\.(rlib|rmeta|dylib|so|dll|a|exe|wasm)$", "", stem)
    for candidate in {stem + ".d", base + ".d", re.sub(r"\.(rlib|rmeta)$", ".d", base)}:
        path = os.path.join(directory, candidate)
        if os.path.exists(path):
            first = open(path, encoding="utf-8", errors="replace").readline()
            if ": " in first:
                for dep in first.split(": ", 1)[1].split():
                    full = dep if os.path.isabs(dep) else os.path.join(root, dep)
                    if full.startswith(root): reads.setdefault(os.path.relpath(full, root), set()).add(base)
            break
outside, written, covered = [], [], 0
def matches(path): return any(fnmatch.fnmatch(path, p.replace("**/", "*/").replace("**", "*")) or path == p for p in patterns)
for path in sorted(reads):
    if any(path == prefix or path.startswith(prefix.rstrip("/") + "/") for prefix in own): written.append(path)
    elif path in files or matches(path): covered += 1
    else: outside.append(path)
print("units", len(units), "workspace files read", len(reads), "covered", covered, "own outputs", len(written), "OUTSIDE THE SELECTION", len(outside))
for path in outside[:40]: print("  outside:", path)
sys.exit(1 if outside else 0)
