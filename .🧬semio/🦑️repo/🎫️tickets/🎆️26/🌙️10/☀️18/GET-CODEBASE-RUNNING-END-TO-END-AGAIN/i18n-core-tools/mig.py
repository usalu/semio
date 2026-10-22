#!/usr/bin/env python3
"""The migration engine: finds the console and Error sites of a source by a needle, derives the English catalog text from the
source literal and rewrites the site to a catalog call. German texts come from the specs (hand-written)."""
import re, importlib.util, os
from lib import *

_spec = importlib.util.spec_from_file_location("chk", os.path.join(HERE, "staged/files", L, "⚖️laws/🗣️language-catalogs/🧪️tests/🗣️corpus/🐍️checker.py"))
CHK = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(CHK)

CALLS = re.compile(r"console\.(log|error|warn|info)\(|(?<![A-Za-z0-9_$])(?:process\.)?std(out|err)\.write\(|(?<![A-Za-z0-9_$.])(?:new\s+)?Error\(")
NOISE = {"length", "String", "Number", "toFixed", "join", "map", "filter", "slice", "toString", "join", "this", "root", "const", "ticketDir", "report", "args", "options", "context"}


def sites(text):
    """Every console/Error call of the text: dict(start, end (index after ')'), kind, level, args=[(s,e)], first=source of the first argument)."""
    out = []
    pos = 0
    for match in CALLS.finditer(text):
        if match.start() < pos:
            continue
        open_index = match.end() - 1
        close = scan_call(text, open_index)
        spans = split_args(text, open_index, close)
        first = text[spans[0][0]:spans[0][1]].strip()
        head = match.group(0)
        kind = "console" if head.startswith("console") else "stream" if "write(" in head else "error"
        level = {"log": "info", "info": "info", "error": "error", "warn": "warn"}.get(match.group(1)) if kind == "console" else ("info" if match.group(2) == "out" else "error") if kind == "stream" else None
        out.append({"start": match.start(), "end": close + 1, "kind": kind, "level": level, "spans": spans, "first": first, "text": text[match.start():close + 1]})
        pos = close + 1
    return out


def is_literal(source):
    if not source or source[0] not in "\"'`":
        return False
    try:
        return scan_string(source, 0) == len(source)
    except Refusal:
        return False


def camel(word):
    if "_" in word or word.isupper():
        parts = [p.lower() for p in word.split("_") if p]
        return parts[0] + "".join(p[:1].upper() + p[1:] for p in parts[1:])
    return word[:1].lower() + word[1:]


def autoname(expr, used):
    words = [w for w in re.findall(r"[A-Za-z_][A-Za-z0-9_]*", expr) if w not in NOISE and w not in ("await", "new", "undefined", "null", "true", "false") and not re.fullmatch(r"[0-9]+", w)]
    if not words:
        words = ["value"]
    name = camel(words[-1])
    base, n = name, 2
    while name in used:
        name = base + str(n)
        n += 1
    used.add(name)
    return name


def derive(first, names=None):
    """(EN template, [(name, expr)]) of a literal source; `names` overrides the automatic placeholder names in order."""
    parts = split_template(first)
    template, exprs, used = [], [], set(names or [])
    index = 0
    auto_used = set(names or [])
    for kind, value in parts:
        if kind == "t":
            template.append(escape_braces(value))
        else:
            name = names[index] if names and index < len(names) else autoname(value, auto_used)
            index += 1
            template.append("{" + name + "}")
            exprs.append((name, value))
    return "".join(template), exprs


def slug(template, words=5, scope=""):
    skip = {"verify", "commit", "micro", "nx"} | set(scope.split("-"))
    template = re.sub(r"\([^)]*\)", " ", template)
    tag = re.match(r"\s*\[([^\]]*)\]", template)
    tag_words = [w.lower() for w in re.findall(r"[A-Za-z][A-Za-z0-9]*", tag.group(1))] if tag else []
    body = re.sub(r"\{[A-Za-z0-9_]+\}", " ", template[tag.end():] if tag else template)
    body_words = [w.lower() for w in re.findall(r"[A-Za-z][A-Za-z0-9]*", body)]
    return "-".join([w for w in tag_words + body_words if w not in skip][:words]) or "line"


def values_source(exprs):
    if not exprs:
        return ""
    return "{" + ",".join(name if name == value.strip() else name + ":" + value.strip() for name, value in exprs) + "}"


def skeleton(first):
    """The static text of a literal with `{}` for every `${}` (real characters; the spec notation escapes them)."""
    return "".join(v if k == "t" else "{}" for k, v in split_template(first))


def enc(text):
    return text.replace("\\", "\\\\").replace("\n", "\\n")


def dec(text):
    out, i = [], 0
    while i < len(text):
        if text[i] == "\\" and i + 1 < len(text):
            out.append("\n" if text[i + 1] == "n" else text[i + 1])
            i += 2
        else:
            out.append(text[i])
            i += 1
    return "".join(out)


def guess_code(text):
    low = text.lower()
    if "usage" in low or low.startswith("expected "):
        return "usage"
    if "breach" in low or "finding" in low or "violation" in low or "disagree" in low:
        return "breach"
    if "missing" in low or "required" in low or "absent" in low:
        return "missing"
    if "unknown" in low or "must" in low or "needs" in low or "accepts" in low or "expected" in low:
        return "invalid"
    return "failed"


NUMERIC = re.compile(r"(?:\.length|\.size|\.toFixed\([0-9]*\)|\.join\([^()]*\))$|^[0-9.]+$|^Math\.[A-Za-z]+\([^()]*\)$|^`|^\"")


def parse_spec(path):
    """Blocks of a spec: [(rel, scope, [entry dict])]."""
    blocks, current, entry = [], None, None
    for line in open(path, encoding="utf8").read().split("\n"):
        if line.startswith("#") or (not line.strip() and entry is None):
            continue
        if line.startswith("== "):
            rel, scope = line[3:].split(" ¦ ")
            current = (L + "/" + rel.strip(), scope.strip(), [])
            blocks.append(current)
            entry = None
            continue
        if line.startswith("=> "):
            entry["de"] = dec(line[3:])
            current[2].append(entry)
            entry = None
            continue
        if line.startswith("en=> "):
            current[2][-1]["en"] = dec(line[5:])
            continue
        if line.startswith("fixed=> "):
            current[2][-1]["fixed"] = dict(pair.split("=", 1) for pair in line[8:].split("; "))
            continue
        if not line.strip():
            continue
        parts = line.split(" ¦ ")
        entry = {"skeleton": dec(parts[0]), "names": [n for n in parts[1].split(",") if n] if len(parts) > 1 else [], "code": parts[2].strip() if len(parts) > 2 else "", "key": parts[3].strip() if len(parts) > 3 else ""}
    return blocks


def holders(template):
    return sorted(set(re.findall(r"(?<!\{)\{([A-Za-z_][A-Za-z0-9_]*)\}(?!\})", template.replace("{{", "\0\0").replace("}}", "\1\1"))))


def wrap(expr):
    e = expr.strip()
    if NUMERIC.search(e):
        return e
    return "String(" + e + ")"


def migrate_file(tree, rel, scope, entries, catalog, say, refuse, used):
    """Rewrites the prose console/Error sites of one source that the entries describe; fills `catalog` (key -> {en, de}); returns (rewritten, left)."""
    text = tree.text(rel)
    by_skeleton = {e["skeleton"]: e for e in entries}
    edits, left, hit = [], 0, set()
    for s in sites(text):
        first = s["first"]
        if not is_literal(first):
            continue
        sk = skeleton(first)
        entry = by_skeleton.get(sk)
        if entry is None:
            plain = "".join(v if k == "t" else "XX" for k, v in split_template(first))
            if CHK.prose(plain) and not plain.startswith(("[DEBUG] ", "[DATA] ")):
                left += 1
            continue
        hit.add(sk)
        template, exprs = derive(first, entry["names"])
        if s["kind"] == "stream":
            if not template.endswith("\n"):
                raise Refusal("%s: stream write without a trailing newline: %r" % (rel, sk))
            template = template[:-1]
        if len(exprs) != len(entry["names"]):
            raise Refusal("%s: %d names for %d expressions in %r" % (rel, len(entry["names"]), len(exprs), sk))
        if len(set(entry["names"])) != len(entry["names"]):
            raise Refusal("%s: duplicate names %r" % (rel, entry["names"]))
        de = entry["de"]
        if s["kind"] == "stream" and de.endswith("\n"):
            de = de[:-1]
        if holders(template) != holders(de):
            raise Refusal("%s: placeholders differ between English %r and German %r" % (rel, holders(template), holders(de)))
        key = entry["key"] or (scope + "." + slug(template, scope=scope))
        base, n = key, 2
        while key in catalog and (catalog[key]["en"] != template or catalog[key]["de"] != de):
            key = "%s-%d" % (base, n)
            n += 1
        catalog[key] = {"en": template, "de": de}
        used.add(key)
        values = "{" + ",".join(name + ":" + wrap(expr) for name, expr in exprs) + "}" if exprs else ""
        if s["kind"] in ("console", "stream"):
            if len(s["spans"]) != 1 and s["kind"] == "stream":
                raise Refusal("%s: stream write with several arguments: %r" % (rel, s["text"][:80]))
            tail = text[s["spans"][1][0]:s["end"] - 1].strip() if len(s["spans"]) > 1 else ""
            method = re.match(r"console\.(\w+)\(", s["text"]).group(1) if tail else ""
            call = "%s(%s" % (say, js_string(key))
            level = s["level"]
            if values or level != "info":
                call += "," + (values or "{}")
            if level != "info":
                call += "," + js_string(level)
            new = call + ")"
            if tail:
                new = "(%s,console.%s(%s))" % (new, method, tail)
        else:
            code = entry["code"] or guess_code(sk)
            extra = text[s["spans"][1][0]:s["end"] - 1] if len(s["spans"]) > 1 else ""
            new = "%s(%s,%s%s%s)" % (refuse, js_string(code), js_string(key), ("," + (values or "{}")) if (values or extra) else "", ("," + extra.strip()) if extra else "")
        edits.append((s["start"], s["end"], new))
    missing = [e["skeleton"] for e in entries if e["skeleton"] not in hit]
    if missing:
        raise Refusal("%s: spec entries match no site (stale spec): %r" % (rel, missing[:3]))
    for start, end, new in sorted(edits, reverse=True):
        text = text[:start] + new + text[end:]
    tree.put(rel, text)
    return len(edits), left
