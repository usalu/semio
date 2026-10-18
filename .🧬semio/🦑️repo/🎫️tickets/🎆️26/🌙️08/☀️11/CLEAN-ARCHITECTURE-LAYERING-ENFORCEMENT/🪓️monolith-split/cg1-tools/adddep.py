import os, re, sys
# ➕️ adddep.py <manifest> <section> <name> <target-pkg-dir> [features] : inserts `<name> = { path = <rel>[, features] }` into <section> of <manifest> right after the semio-framework-job line of that section (else at the section end); no-op when present.
R = "/Users/ueli/Documents/semio"
man, sec, name, target = sys.argv[1:5]
feats = sys.argv[5] if len(sys.argv) > 5 else ""
mp = os.path.join(R, man)
t = open(mp, encoding="utf8").read()
lines = t.split("\n")
rel = os.path.relpath(os.path.join(R, target), os.path.dirname(mp))
new = f'{name} = {{ path = "{rel}"{(", features = [" + feats + "]") if feats else ""} }}'
start = next((i for i, l in enumerate(lines) if l.strip() == f"[{sec}]"), None)
assert start is not None, ("section missing", man, sec)
end = next((i for i in range(start + 1, len(lines)) if lines[i].strip().startswith("[")), len(lines))
if any(re.match(rf"^{re.escape(name)}\s*=", l) for l in lines[start:end]):
    print("present", man[-50:]); sys.exit(0)
at = next((i for i in range(start + 1, end) if re.match(r"^semio-framework-job\s*=", lines[i])), None)
if at is None:
    at = end - 1
    while at > start and not lines[at].strip(): at -= 1
lines.insert(at + 1, new)
open(mp, "w", encoding="utf8").write("\n".join(lines))
print("added", man[-60:], "->", new[:150])
