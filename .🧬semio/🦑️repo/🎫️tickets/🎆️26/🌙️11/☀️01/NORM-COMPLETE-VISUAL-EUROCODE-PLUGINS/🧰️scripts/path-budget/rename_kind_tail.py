"""📏 Renames the noun/field tail of mutation kinds in every case form (kebab, snake, Pascal incl. camel/parse/print prefixes, SCREAMING), in file contents and in file/folder names, so over-budget fixture paths shrink at the source.

`python -I rename_kind_tail.py [--apply] OLD=NEW ...` where OLD and NEW are kebab tails without the verb (e.g. `ground-investigation-reading-measurement=ground-reading-measurement`). Dry run lists the files and the superstring tokens that would also change."""
import os, re, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
EXTRA = ["/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json"]
SKIP = {"target", "node_modules", ".git", "__pycache__"}


def forms(tail):
    parts = tail.split("-")
    return [tail, "_".join(parts), "".join(p[:1].upper() + p[1:] for p in parts), "_".join(parts).upper()]


def main():
    apply = "--apply" in sys.argv
    pairs = [a.split("=") for a in sys.argv[1:] if "=" in a]
    table = []
    for old, new in pairs:
        table += list(zip(forms(old), forms(new)))
    table.sort(key=lambda p: -len(p[0]))
    rx = re.compile("|".join(re.escape(o) for o, _ in table))
    mp = dict(table)
    files = changed = 0
    supers = {}
    paths = []
    for d, ds, fs in os.walk(ROOT):
        ds[:] = [x for x in ds if x not in SKIP]
        paths += [os.path.join(d, f) for f in fs]
    paths += [p for p in EXTRA if os.path.exists(p)]
    for p in paths:
        if os.path.islink(p):
            continue
        try:
            t = open(p, encoding="utf-8").read()
        except Exception:
            continue
        for m in rx.finditer(t):
            s, e = m.span()
            while s > 0 and re.match(r"[A-Za-z0-9_-]", t[s - 1]): s -= 1
            while e < len(t) and re.match(r"[A-Za-z0-9_-]", t[e]): e += 1
            supers[t[s:e]] = supers.get(t[s:e], 0) + 1
        n = rx.sub(lambda g: mp[g.group(0)], t)
        if n != t:
            changed += 1
            if apply:
                open(p, "w", encoding="utf-8").write(n)
    if not apply:
        for k, v in sorted(supers.items()):
            print(v, k)
    moved = 0
    for d, ds, fs in os.walk(ROOT, topdown=False):
        ds[:] = [x for x in ds if x not in SKIP]
        for x in fs + ds:
            if rx.search(x):
                new = rx.sub(lambda g: mp[g.group(0)], x)
                if apply:
                    os.rename(os.path.join(d, x), os.path.join(d, new))
                moved += 1
    print("apply" if apply else "dry", "files changed", changed, "renamed entries", moved)


main()
