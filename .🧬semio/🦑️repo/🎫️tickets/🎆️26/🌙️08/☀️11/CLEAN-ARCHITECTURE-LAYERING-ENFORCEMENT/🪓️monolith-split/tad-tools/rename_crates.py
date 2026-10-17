"""🏷️ Coordinated crate rename over the trinity artifacts, the dev crate, the build-owner rows and the lock files.
usage: rename_crates.py [--apply] old=new [old=new ...]   (names are the hyphenated Cargo names; the underscore form is renamed too)
The renames are applied simultaneously through placeholders, so `x-mut-any-2=x-mut-any` and `x-mut-any=x-model` can be given together.
Lock files are re-sorted like cargo does (name, version, source); the tool refuses a lock whose untouched text does not already re-sort to itself."""
import os, re, sys

ROOT = "/Users/ueli/Documents/semio"
DIRS = [
    "✏️s/🔌️plugins/🔱️trinity",
    "✏️s/🧑‍💻dev/🔱️trinity",
]
FILES = ["📋️build-owners.json"]
SKIP_DIRS = {"node_modules", "target", ".git", "dist"}
TEXT_EXT = (".rs", ".toml", ".ts", ".json", ".md", ".lock", ".jsonc", ".txt", ".yml", ".yaml")


def walk():
    out = []
    for d in DIRS:
        for base, dirs, files in os.walk(os.path.join(ROOT, d)):
            dirs[:] = [x for x in dirs if x not in SKIP_DIRS]
            for f in files:
                if f.endswith(TEXT_EXT) and f != "AGENTS.md":
                    out.append(os.path.join(base, f))
    for f in FILES:
        out.append(os.path.join(ROOT, f))
    return out


def lock_key(block):
    name = re.search(r'^name = "([^"]+)"', block, re.M).group(1)
    version = re.search(r'^version = "([^"]+)"', block, re.M).group(1)
    source = re.search(r'^source = "([^"]+)"', block, re.M)
    vkey = tuple(int(x) if x.isdigit() else x for x in re.split(r"[.+-]", version))
    return (name, [(0, v) if isinstance(v, int) else (1, v) for v in vkey], source.group(1) if source else "")


def dep_key(entry):
    parts = entry.split(" ", 2)
    name = parts[0]
    version = parts[1] if len(parts) > 1 else ""
    vkey = [(0, int(x)) if x.isdigit() else (1, x) for x in re.split(r"[.+-]", version)] if version else []
    return (name, vkey, parts[2] if len(parts) > 2 else "")


def resort_lock(text):
    head, sep, rest = text.partition("[[package]]\n")
    if not sep:
        return text
    chunks = ("[[package]]\n" + rest).split("\n[[package]]\n")
    chunks[0] = chunks[0][len("[[package]]\n"):]
    tail = ""
    last = chunks[-1]
    m = re.search(r"\n\n(?=\[(?!\[package\]))", last)
    if m:
        tail = last[m.start():]
        chunks[-1] = last[:m.start()]
    blocks = []
    for c in chunks:
        c = c.rstrip("\n")

        def fix_deps(match):
            items = re.findall(r'^ "([^"]+)",$', match.group(2), re.M)
            items.sort(key=dep_key)
            return match.group(1) + "".join(f' "{i}",\n' for i in items) + match.group(3)

        c = re.sub(r"(^dependencies = \[\n)((?: \"[^\"]+\",\n)+)(\]$)", fix_deps, c, flags=re.M)
        blocks.append(c)
    blocks.sort(key=lock_key)
    return head + "".join("[[package]]\n" + b + "\n\n" for b in blocks).rstrip("\n") + "\n" + tail.rstrip("\n") + ("\n" if tail else "")


def main():
    args = sys.argv[1:]
    apply = "--apply" in args
    pairs = [a.split("=", 1) for a in args if "=" in a]
    assert pairs, "no renames"
    stage = []
    for i, (old, new) in enumerate(pairs):
        for form in (lambda s: s, lambda s: s.replace("-", "_")):
            stage.append((re.compile(r"(?<![A-Za-z0-9_-])" + re.escape(form(old)) + r"(?![A-Za-z0-9_-])"), f"\x00{i}\x00" + ("U" if form(old) != old else "H"), form(new)))
    changed = []
    for path in walk():
        try:
            text = open(path, encoding="utf-8").read()
        except Exception:
            continue
        low = text
        if not any(old.replace("-", "_") in text or old in text for old, _ in pairs):
            continue
        is_lock = path.endswith("Cargo.lock")
        if is_lock and resort_lock(text) != text:
            print(f"REFUSE {path}: lock does not re-sort to itself", file=sys.stderr)
            sys.exit(2)
        count = 0
        for rx, ph, _ in stage:
            low, k = rx.subn(ph, low)
            count += k
        for rx, ph, new in stage:
            low = low.replace(ph, new)
        if count == 0:
            continue
        if is_lock:
            low = resort_lock(low)
        changed.append((path, count))
        if apply:
            real = os.path.realpath(path)
            assert any(real.startswith(os.path.realpath(os.path.join(ROOT, d)) + os.sep) for d in DIRS) or real == os.path.realpath(os.path.join(ROOT, "📋️build-owners.json")), real
            open(path, "w", encoding="utf-8").write(low)
    for p, k in changed:
        print(f"{k:4d} {os.path.relpath(p, ROOT)}")
    print(f"{'APPLIED' if apply else 'DRY RUN'}: {len(changed)} files, {sum(k for _, k in changed)} replacements")


if __name__ == "__main__":
    main()
