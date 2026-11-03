import pathlib
ANY = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any")
S = ANY / "🧬️schema"
M = S / "🧬️mutations"
cache = {}
def text(p):
    p = pathlib.Path(p)
    if p not in cache: cache[p] = p.read_text(encoding="utf-8")
    return cache[p]
def sub(p, old, new, count=1):
    p = pathlib.Path(p); t = text(p); n = t.count(old)
    assert n == count, f"{p.relative_to(ANY)}: expected {count} of {old[:90]!r}, found {n}"
    cache[p] = t.replace(old, new)
def flush():
    for p, t in cache.items(): p.write_text(t, encoding="utf-8")
    print("wrote", len(cache), "files")
