"""✅️ Structural checks of every new pfb file: path length, docstring emoji, region balance, mount uniqueness, rustfmt parse, line caps."""
import os, re, subprocess, sys, unicodedata, collections

ROOT = "/Users/ueli/Documents/semio"
TARGETS = {
    "sim": ("✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim", "🦀️.rs"),
    "visual-job": ("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job", "🦀️.rs"),
    "energy-test-rs": ("✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1", "🦀️.rs"),
    "energy-test-py": ("✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1", "🐍️.py"),
}
problems = []
def leading_emoji(text):
    first = text[0]
    return first if unicodedata.category(first) == "So" or ord(first) > 0x2000 else None
for name, (rel, filename) in TARGETS.items():
    base = os.path.join(ROOT, rel)
    root_file = os.path.join(base, filename)
    folders = sorted(d for d in os.listdir(base) if os.path.isfile(os.path.join(base, d, filename)))
    if name in ("sim", "visual-job"):
        folders = [d for d in folders if d not in ("♻️restore", "📡️wire", "🪜️retirement", "📦️packages", "🧪️tests")]
    files = [root_file] + [os.path.join(base, d, filename) for d in folders]
    emojis = collections.Counter()
    for d in folders:
        emojis[d[0]] += 1
    dup = [e for e, n in emojis.items() if n > 1]
    if dup:
        problems.append(f"{name}: duplicate sibling emoji {dup}")
    root_text = open(root_file, encoding="utf-8").read()
    if filename.endswith(".rs"):
        mounts = re.findall(r'#\[path = "([^"]+)"\]\s*mod (\w+);', root_text)
        mounted = [m[0] for m in mounts if not m[0].startswith("🧪️tests")]
        wanted = [d + "/" + filename for d in folders]
        if sorted(mounted) != sorted(wanted):
            problems.append(f"{name}: mounts {sorted(mounted)} != folders {sorted(wanted)}")
        names = [m[1] for m in mounts]
        if len(set(names)) != len(names):
            problems.append(f"{name}: duplicate mod names")
    else:
        loads = re.findall(r'_load\("[^"]+", "([^"]+)"\)', root_text)
        if sorted(loads) != sorted(folders):
            problems.append(f"{name}: loads {sorted(loads)} != folders {sorted(folders)}")
    for path in files:
        text = open(path, encoding="utf-8").read()
        lines = text.split("\n")
        relp = os.path.relpath(path, ROOT)
        if len(relp.encode("utf-8")) > 240:
            problems.append(f"path too long ({len(relp.encode('utf-8'))}): {relp}")
        if path != root_file:
            head = lines[0]
            body = head[4:] if head.startswith("//! ") else head[3:] if head.startswith('"""') else None
            if body is None or leading_emoji(body) is None:
                problems.append(f"{relp}: first line is not an emoji docstring: {head[:40]}")
            elif body[0] != os.path.basename(os.path.dirname(path))[0]:
                problems.append(f"{relp}: docstring emoji {body[0]} differs from the folder emoji")
        opens = [l.strip() for l in lines if re.match(r"\s*(//#|# )region ", l)]
        closes = [l.strip() for l in lines if re.match(r"\s*(//#|# )endregion ", l)]
        stack = []
        for l in lines:
            m = re.match(r"\s*(?://#|#) region (.*)", l)
            if m: stack.append(m.group(1).strip())
            m = re.match(r"\s*(?://#|#) endregion (.*)", l)
            if m:
                if not stack or stack.pop() != m.group(1).strip():
                    problems.append(f"{relp}: region mismatch at {l.strip()}")
        if stack:
            problems.append(f"{relp}: unclosed regions {stack}")
        count = len(lines)
        if count > 1500:
            problems.append(f"{relp}: {count} lines")
        if path.endswith(".rs"):
            r = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout", path], capture_output=True, text=True)
            if r.returncode != 0:
                problems.append(f"{relp}: rustfmt {r.stderr[:100]}")
    sizes = {os.path.relpath(p, base): len(open(p, encoding='utf-8').read().split('\n')) for p in files}
    print(name, "files", len(files), "max lines", max(sizes.values()), sizes)
print("PROBLEMS:", problems if problems else "none")
sys.exit(1 if problems else 0)
