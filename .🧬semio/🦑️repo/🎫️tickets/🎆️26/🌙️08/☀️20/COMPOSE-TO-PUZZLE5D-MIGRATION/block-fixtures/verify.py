#!/usr/bin/env python3
# ✅️ Structural verification: every glue #[path] resolves, every include_str! target exists,
# every emitted JSON reparses, and rustfmt parses every emitted test file plus glue.rs.
import io, os, re, subprocess, sys, json

REPO = "/Users/ueli/Documents/semio"
GLUE = os.path.join(REPO, "✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs")
GLUE_DIR = os.path.dirname(GLUE)
errors = []

text = io.open(GLUE, encoding="utf-8").read()
paths = re.findall(r'#\[path = "([^"]+)"\]', text)
missing = [p for p in paths if p != "." and not os.path.exists(os.path.normpath(os.path.join(GLUE_DIR, p)))]
for p in missing:
    errors.append(f"glue #[path] dangling: {p}")
print(f"glue #[path] entries: {len(paths)} · dangling: {len(missing)}")

roots = [os.path.join(REPO, f"✏️s/🔌️plugins/🧱️block/🗿️artifacts/{a}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations") for a in ("🖐️5d", "🧊️3d", "◻2d")]
test_files, json_files = [], []
for root in roots:
    if not os.path.isdir(root):
        continue
    for leaf in sorted(os.listdir(root)):
        tests = os.path.join(root, leaf, "🧪️tests")
        if not os.path.isdir(tests):
            continue
        for case in sorted(os.listdir(tests)):
            case_dir = os.path.join(tests, case)
            if not os.path.isdir(case_dir):
                continue
            rs = os.path.join(case_dir, "🦀️component.rs")
            test_files.append(rs)
            for target in re.findall(r'include_str!\("([^"]+)"\)', io.open(rs, encoding="utf-8").read()):
                resolved = os.path.normpath(os.path.join(case_dir, target))
                if not os.path.exists(resolved):
                    errors.append(f"include_str! dangling: {rs} -> {target}")
            for dirpath, _, names in os.walk(case_dir):
                for name in names:
                    if name.endswith(".json"):
                        json_files.append(os.path.join(dirpath, name))

print(f"test files: {len(test_files)} · json files: {len(json_files)}")
for path in json_files:
    try:
        json.load(io.open(path, encoding="utf-8"))
    except Exception as exc:
        errors.append(f"invalid JSON: {path}: {exc}")

bad = []
for path in test_files + [GLUE]:
    result = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout", path], capture_output=True)
    if result.returncode != 0:
        bad.append((path, result.stderr.decode()[:300]))
print(f"rustfmt parsed: {len(test_files) + 1 - len(bad)}/{len(test_files) + 1}")
for path, err in bad[:5]:
    errors.append(f"rustfmt failed: {path}: {err}")

for e in errors[:20]:
    print("❌️", e)
print("✅️ structural verification clean" if not errors else f"❌️ {len(errors)} problem(s)")
sys.exit(1 if errors else 0)
