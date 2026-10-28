# keys5-g: independent Python reading of what `mutationSchemaDocumentIndexProjection` reads (dsl-derive-rs:generate-schema-index payload).
# Streams a directory walk (no tree in memory); prints one repository-relative JSON path per line: every file whose bytes the payload parses.
import os, sys, unicodedata
EXCLUDED = {"target", "node_modules", "dist", "🗑️generated", "📦️packages"}
COLLECTIONS = {"🧫️fixtures", "🧪️fixtures", "🧫️examples", "🧪️examples", "🧪️tests"}
def example(segs): return any(s in COLLECTIONS and (i == 0 or segs[i - 1] != "🔨️modules") for i, s in enumerate(segs))
def nfc(s): return unicodedata.normalize("NFC", s)
def entries(d):
    try: return sorted(os.scandir(d), key=lambda e: e.name.encode("utf-8"))
    except OSError: return []
def roots(repo):
    out = []
    def walk(d, segs):
        for e in entries(d):
            if not e.is_dir(follow_symlinks=False): continue
            name = nfc(e.name); nxt = segs + [name]
            if name.startswith(".") or name in EXCLUDED or example(nxt): continue
            if name in ("🔌️plugins", "🔨️modules"):
                for c in entries(e.path):
                    if not c.is_dir(follow_symlinks=False): continue
                    cn = nfc(c.name); cs = nxt + [cn]
                    if cn.startswith(".") or cn in EXCLUDED or example(cs): continue
                    out.append("/".join(cs))
            if name == "🧬️schema" and any(c.is_dir(follow_symlinks=False) and nfc(c.name) == "🧬️mutations" for c in entries(e.path)):
                segments = "/".join(nxt).split("/")
                def under(p):
                    idx = [i for i in range(len(segments)) if i > 0 and segments[i - 1] == p and segments[i]]
                    return "/".join(segments[: idx[-1] + 1]) if idx else None
                owner = "/".join(segments[:-1])
                o = under("🔌️plugins") or under("🔨️modules") or (owner or None)
                if o and not example(o.split("/")): out.append(o)
            walk(e.path, nxt)
    walk(repo, [])
    return sorted(set(out))
def documents(repo, root):
    segs = [s for s in os.path.join(repo, root).split(os.sep) if s]
    if example(segs): return
    def walk(d, ds, schema):
        for e in entries(d):
            if e.is_symlink(): continue
            name = nfc(e.name); nxt = ds + [name]
            if e.is_dir(follow_symlinks=False):
                if name.startswith(".") or name in EXCLUDED or example(nxt): continue
                yield from walk(e.path, nxt, schema or name == "🧬️schema")
            elif schema and e.is_file(follow_symlinks=False) and name.endswith(".json"):
                yield os.path.relpath(e.path, repo)
    yield from walk(os.path.join(repo, root), segs, "🧬️schema" in segs)
repo = sys.argv[1]
seen = set()
for r in roots(repo):
    for p in documents(repo, r):
        if p not in seen: seen.add(p); print(p)
print(os.path.relpath(os.path.join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧬️schema/🔣️.json"), repo))
