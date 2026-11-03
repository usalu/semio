"""Shared helpers of the leaf-kinds generator: naming, staged writes, schema reading."""
import atexit, json, os, re, sys

SEMIO = "/home/deck/semio"
NORM = SEMIO + "/✏️s/🔌️plugins/📕️norm"
TICKET = SEMIO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
STAGE = TICKET + "/🗑️generated/leaf-kinds/stage"
TAXONOMY = NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
OWNERS = SEMIO + "/📋️build-owners.json"
VS = "️"


def camel(s):
    p = s.split("_")
    return p[0] + "".join(x[:1].upper() + x[1:] for x in p[1:])


def pascal(s):
    return "".join(x[:1].upper() + x[1:] for x in s.replace("-", "_").split("_"))


def snake(c):
    return re.sub(r"([A-Z])", lambda m: "_" + m.group(1).lower(), c)


def kebab(s):
    return s.replace("_", "-")


def emoji_dir(emoji, name):
    return (emoji if emoji.endswith(VS) or len(emoji) > 1 and emoji[-1] == VS else emoji + VS) + name


def rd(path):
    return open(path, encoding="utf-8").read()


def dumps(obj):
    return json.dumps(obj, indent=2, ensure_ascii=False) + "\n"


def merge_json(existing, staged):
    """Ordered union: the existing keys keep their order and values, keys only the generator knows are appended, lists keep their items and gain the staged ones that are missing."""
    if isinstance(existing, dict) and isinstance(staged, dict):
        out = {k: merge_json(v, staged[k]) if k in staged else v for k, v in existing.items()}
        out.update({k: v for k, v in staged.items() if k not in existing})
        return out
    if isinstance(existing, list) and isinstance(staged, list):
        return existing + [item for item in staged if item not in existing]
    return existing


def merge_lines(existing, staged):
    """Line union: lines only the generator produced are inserted where they belong, every line of the existing file stays (hand-written text is never replaced or removed)."""
    import difflib
    a, b = existing.split("\n"), staged.split("\n")
    known, out = set(a), []
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        out += [line for line in b[j1:j2] if line.strip() and (len(line) < 40 or line not in known)] if tag == "insert" else a[i1:i2] + ([line for line in b[j1:j2] if line not in known and not line.startswith("//!")] if tag == "replace" else [])
    return "\n".join(out)


def merge_def_lines(existing, staged):
    """JSON documents that keep one definition per line (spaced or compact): definitions only the generator knows are appended as lines in the same style, every existing line stays verbatim and wins."""
    import re
    compact = re.search(r'^    "[^"]+":\S', existing, re.M) is not None
    sep = ":" if compact else ": "
    known = set(re.findall(r'^    ("[^"]+"):', existing, re.M))
    fresh = [(k, v) for k, v in json.loads(staged).get("$defs", {}).items() if json.dumps(k) not in known]
    if not fresh:
        return existing
    lines = existing.split("\n")
    last = max(i for i, line in enumerate(lines) if re.match(r'^    "[^"]+":', line))
    lines[last] = lines[last].rstrip(",") + ","
    enc = (lambda v: json.dumps(v, separators=(",", ":"), ensure_ascii=False)) if compact else (lambda v: json.dumps(v, ensure_ascii=False))
    lines[last + 1:last + 1] = [("    %s%s%s," % (json.dumps(k, ensure_ascii=False), sep, enc(v))) for k, v in fresh]
    lines[last + len(fresh)] = lines[last + len(fresh)].rstrip(",")
    return "\n".join(lines)


def merge_text(path, existing, staged):
    if existing == staged:
        return existing
    if path.endswith(".json"):
        try:
            current = json.loads(existing)
            merged = merge_json(current, json.loads(staged))
        except ValueError:
            return merge_lines(existing, staged)
        if merged == current:
            return existing
        if dumps(current) == existing:
            return dumps(merged)
        if '\n    "' in existing and "$defs" in current:
            return merge_def_lines(existing, staged)
    return merge_lines(existing, staged)


def protocol_merge(text, kinds_all):
    """Binary protocol text with every existing record and tag kept verbatim; kinds without a record are appended in order at the tags after the highest existing one."""
    records = re.findall(r"^record (\S+) tag=(\d+)$", text, re.M)
    known = {k for k, _ in records}
    nxt = max((int(t) for _, t in records), default=-1) + 1
    add = [k for k in kinds_all if k not in known]
    assert nxt + len(add) < 2**32, "varint tag overflow"
    head = text[: re.search(r"^record ", text, re.M).start()] if records else text.rstrip("\n") + "\n"
    body = text[len(head):].rstrip("\n")
    return head + (body + "\n" if body else "") + "".join("record %s tag=%d\nfield payload bytes\n" % (k, nxt + i) for i, k in enumerate(add))


class Files:
    """Staged writer, additive by construction: every write lands under STAGE mirroring the real tree; `apply` copies the stage onto the tree. Reads prefer staged content so passes compose. An existing file is never replaced: `apply` merges the staged additions into it (`merge_text`), so hand-written leaves, crate files, reordered twins and lanes the generator does not know survive. `--check` (any driver) prints the files an `--apply` would change and exits 1 when there are any, without touching the tree."""

    def __init__(self, stage, merge=True):
        self.stage = stage
        self.merge = merge
        self.written = set()
        self.merged = []
        self.changed = []
        if "--check" in sys.argv:
            atexit.register(self._check)

    def _s(self, path):
        return self.stage + path

    def exists(self, path):
        return os.path.exists(self._s(path)) or os.path.exists(path)

    def read(self, path):
        s = self._s(path)
        return rd(s) if os.path.exists(s) else rd(path)

    def write(self, path, text):
        assert len(path[len(SEMIO) + 1:].encode()) < 256, "path of 256 bytes or more: " + path
        s = self._s(path)
        os.makedirs(os.path.dirname(s), exist_ok=True)
        open(s, "w", encoding="utf-8").write(text)
        self.written.add(path)

    def pending(self):
        out = []
        for path in sorted(self.written):
            staged = rd(self._s(path))
            existing = rd(path) if os.path.exists(path) else None
            text = staged if existing is None else merge_text(path, existing, staged) if self.merge else staged
            if text != existing:
                out.append((path, text, existing is not None and text != staged))
        return out

    def apply(self):
        if "--check" in sys.argv:
            return 0
        self.changed, self.merged = [], []
        for path, text, merged in self.pending():
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(text)
            self.changed.append(path)
            if merged:
                self.merged.append(path)
        return len(self.changed)

    def _check(self):
        todo = self.pending()
        for path, _, _ in todo:
            print("would change", path[len(SEMIO) + 1:])
        sys.stdout.flush()
        os._exit(1 if todo else 0)


def rep(text, old, new, count=1):
    assert old in text, "anchor missing: " + old[:120]
    return text.replace(old, new, count)


def compact_json(d):
    c = lambda v: json.dumps(v, separators=(",", ":"), ensure_ascii=False)
    rows = []
    for k, v in d.items():
        if k == "$defs" and isinstance(v, dict):
            body = ",\n".join("    %s:%s" % (c(dk), c(dv)) for dk, dv in v.items())
            rows.append("  %s:{\n%s\n  }" % (c(k), body))
        else:
            rows.append("  %s:%s" % (c(k), c(v)))
    return "{\n" + ",\n".join(rows) + "\n}\n"


def mount_into(root, mods):
    """Append-only crate root: `mods` is a list of (dir, module); the mounts a root lacks are added before the end of its mount region, every existing line stays."""
    new = "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (d, m) for d, m in mods if "pub mod %s;" % m not in root)
    return root.replace("//#endregion 🪢️Mounts", new + "//#endregion 🪢️Mounts", 1) if new else root
