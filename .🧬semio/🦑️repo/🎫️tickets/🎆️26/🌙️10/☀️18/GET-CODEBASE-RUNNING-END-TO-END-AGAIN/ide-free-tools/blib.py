"""Shared builder helpers for ide-free-tools/build-plan.py (read only on the live tree; standard library only)."""
import hashlib, json, os, re

HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(C, *[".."] * 7))
M = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
L = M + "/📚️library"
PLAN = {"version": 1, "edits": [], "creates": [], "deletes": [], "moves": [], "rmdirs": []}
_work = {}
_ids = {}


def read(rel):
    if rel not in _work:
        _work[rel] = open(os.path.join(ROOT, rel), encoding="utf-8", newline="").read()
    return _work[rel]


def ident(prefix):
    _ids[prefix] = _ids.get(prefix, 0) + 1
    return f"{prefix}-{_ids[prefix]}"


def ED(group, path, old, new, counts=(1,), tag=None):
    text = read(path)
    if "\r\n" in text and "\r\n" not in old:
        old, new = old.replace("\n", "\r\n"), new.replace("\n", "\r\n")
    n = text.count(old)
    if n not in counts:
        raise SystemExit(f"anchor count {n} not in {counts} for {path}: {old[:90]!r}")
    _work[path] = text.replace(old, new)
    PLAN["edits"].append({"id": ident(tag or group), "group": group, "path": path, "old": old, "new": new, "counts": list(counts)})


def LINE(group, path, line, counts=(1,), tag=None):
    """Removes whole lines that equal `line` (without the newline)."""
    ED(group, path, line + "\n", "", counts, tag)


def RX(group, path, pattern, repl, counts=(1,), tag=None, flags=0):
    """Regex-derived edit: every distinct match becomes an exact-string edit so the apply engine stays anchored."""
    text = read(path)
    seen = []
    for m in re.finditer(pattern, text, flags):
        old = m.group(0)
        new = m.expand(repl) if isinstance(repl, str) else repl(m)
        if old not in [s[0] for s in seen]:
            seen.append((old, new))
    if not seen:
        raise SystemExit(f"regex found nothing in {path}: {pattern!r}")
    for old, new in seen:
        ED(group, path, old, new, counts if len(seen) == 1 else (text.count(old),), tag)


def CREATE(group, path, content, tag=None):
    PLAN["creates"].append({"id": ident(tag or group), "group": group, "path": path, "content": content})


def DELETE(group, path, tag=None):
    data = open(os.path.join(ROOT, path), "rb").read()
    PLAN["deletes"].append({"id": ident(tag or group), "group": group, "path": path, "sha256": hashlib.sha256(data).hexdigest()})


def MOVE(group, src, dst, tag=None):
    data = open(os.path.join(ROOT, src), "rb").read()
    after = hashlib.sha256(data).hexdigest()
    for w in PLAN.get("writes", []):
        if w["path"] == src:
            after = hashlib.sha256(open(os.path.join(HERE, "staged", "new", w["from"]), "rb").read()).hexdigest()
    PLAN["moves"].append({"id": ident(tag or group), "group": group, "src": src, "dst": dst, "sha256": hashlib.sha256(data).hexdigest(), "sha256After": after})


def RMDIR(group, path):
    PLAN["rmdirs"].append({"group": group, "path": path})


def jtext(doc):
    return json.dumps(doc, ensure_ascii=False, indent=2) + "\n"


def FUNC(group, path, name, new_code, tag=None, lang="go"):
    """Replaces one top-level function (from `func name(` to its closing `\\n}\\n`) by exact block, so the edit stays an anchored string."""
    text = read(path)
    start = text.find(f"\nfunc {name}(")
    if start < 0 or text.count(f"\nfunc {name}(") != 1:
        raise SystemExit(f"function {name} not unique in {path}")
    end = text.find("\n}\n", start) + 3
    ED(group, path, text[start + 1:end], new_code if new_code.endswith("\n") else new_code + "\n", (1,), tag)


PLAN["writes"] = []


def JEDIT(group, path, fn, tag=None):
    """Whole-file structural JSON edit: the file must round-trip byte-exactly through json (indent 2, no ASCII escaping, final newline)."""
    text = open(os.path.join(ROOT, path), encoding="utf-8", newline="").read()
    doc = json.loads(text)
    if json.dumps(doc, ensure_ascii=False, indent=2) + "\n" != text:
        raise SystemExit("not canonical JSON, cannot structurally edit: " + path)
    fn(doc)
    new = json.dumps(doc, ensure_ascii=False, indent=2) + "\n"
    if new == text:
        raise SystemExit("JEDIT changed nothing: " + path)
    ident_ = ident(tag or group)
    rel = "writes/" + ident_ + ".json"
    full = os.path.join(HERE, "staged", "new", rel)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    open(full, "w", encoding="utf-8", newline="").write(new)
    PLAN["writes"].append({"id": ident_, "group": group, "path": path, "sha256": hashlib.sha256(text.encode("utf-8")).hexdigest(), "from": rel})


def TEXTWRITE(group, path, new_text, tag=None):
    """Whole-file text write guarded by the sha256 of the current bytes (used by the agent-client codemod)."""
    raw = open(os.path.join(ROOT, path), "rb").read()
    ident_ = ident(tag or group)
    rel = "writes/" + ident_ + ".txt"
    full = os.path.join(HERE, "staged", "new", rel)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    open(full, "w", encoding="utf-8", newline="").write(new_text)
    PLAN["writes"].append({"id": ident_, "group": group, "path": path, "sha256": hashlib.sha256(raw).hexdigest(), "from": rel})
