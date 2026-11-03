"""Helpers of the rc-threading tool: idempotent text edits that clone the root `annex` entry as `reliabilityClass`.

Every edit asserts the number of matches it expects, so a layout that differs from the expectation stops the run instead of corrupting a file.
"""
import difflib, os, re

NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = NORM + "/🗿️artifacts"
SUBSET = "/🏅️standards/🔖️1/🪆️subsets/✳️any"
RC = ("RC1", "RC2", "RC3")
DRY = False
CHANGED = []
CACHE = {}
ORIG = {}


def rd(path):
    if path in CACHE:
        return CACHE[path]
    return open(path, encoding="utf-8").read()


def wr(path, text):
    old = open(path, encoding="utf-8").read() if os.path.exists(path) else ""
    if old == text:
        return
    ORIG.setdefault(path, old)
    CACHE[path] = text
    if path not in CHANGED:
        CHANGED.append(path)
    assert os.path.exists(path) or len(path.replace('/home/deck/semio/', '').encode()) < 256, path
    if not DRY:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(text)


def show(path):
    old = open(path, encoding="utf-8").read() if os.path.exists(path) else ""
    for line in difflib.unified_diff(old.splitlines(), CACHE[path].splitlines(), lineterm="", n=0):
        print(line[:260])


def done(marker, text):
    return marker in text


def sub_once(path, pattern, repl, flags=0, count=1, marker=None):
    text = rd(path)
    if marker is not None and marker in text:
        return False
    found = len(re.findall(pattern, text, flags))
    assert found == count, (path[-70:], pattern[:80], "expected", count, "found", found)
    wr(path, re.sub(pattern, repl, text, flags=flags))
    return True


def clone_lines(path, pattern, convert, count=None, marker=None, flags=re.M):
    """Insert after every match of `pattern` (a full-line regex ending in newline) the converted copy of the match."""
    text = rd(path)
    if marker is not None and marker in text:
        return False
    matches = list(re.finditer(pattern, text, flags))
    assert matches and (count is None or len(matches) == count), (path[-70:], pattern[:80], "expected", count, "found", len(matches))
    out, last = [], 0
    for m in matches:
        out.append(text[last:m.end()])
        out.append(convert(m.group(0)))
        last = m.end()
    out.append(text[last:])
    wr(path, "".join(out))
    return True


def rs_convert(s):
    s = re.sub(r"semio_s_plugin_norm_compliance::AnnexChoice", "semio_s_plugin_norm_compliance::ReliabilityClass", s)
    s = re.sub(r"AnnexChoice::(De|En)\b", "ReliabilityClass::RC2", s)
    s = re.sub(r"\bAnnexChoice\b", "ReliabilityClass", s)
    s = re.sub(r"\bannex\b", "reliability_class", s)
    return s


def balanced(text, start, open_ch="{", close_ch="}"):
    """Index just after the bracket group that opens at the first `open_ch` at or after `start`."""
    i = text.index(open_ch, start)
    depth = 0
    in_str = False
    j = i
    while j < len(text):
        c = text[j]
        if in_str:
            if c == "\\":
                j += 1
            elif c == '"':
                in_str = False
        elif c == '"':
            in_str = True
        elif c == open_ch:
            depth += 1
        elif c == close_ch:
            depth -= 1
            if depth == 0:
                return j + 1
        j += 1
    raise AssertionError("unbalanced")


def json_clone_property(path, expected, struct_anchor=None, converter=None):
    """Clone the `"annex": {...}` property blocks (pretty or compact JSON) as `"reliabilityClass"`."""
    text = rd(path)
    if '"reliabilityClass": {' in text or '"reliabilityClass":{' in text:
        return False
    out, last, n = [], 0, 0
    for m in re.finditer(r'(?P<sep>\n[ \t]*|)"annex":\s*\{', text):
        if struct_anchor and struct_anchor not in text[max(0, m.start() - 4000):m.start()] and n == 0 and False:
            continue
        end = balanced(text, m.start(), "{", "}")
        block = text[m.start() + len(m.group("sep")):end]
        out.append(text[last:end])
        clone = converter(block) if converter else block
        out.append("," + m.group("sep") + clone)
        last = end
        n += 1
    out.append(text[last:])
    assert n == expected, (path[-70:], "annex property blocks", n, "expected", expected)
    wr(path, "".join(out))
    return True


def json_rc_enum(block):
    block = block.replace('"annex"', '"reliabilityClass"').replace("properties/annex", "properties/reliabilityClass")
    block = re.sub(r'"En"(\s*,\s*)"De"', lambda m: '"RC1"' + m.group(1) + '"RC2"' + m.group(1) + '"RC3"', block)
    block = re.sub(r'"\$ref":\s*"#/\$defs/AnnexChoice"', '"type": "string", "enum": ["RC1", "RC2", "RC3"]', block)
    return block


def json_required_after_annex(path, expected):
    """Insert `"reliabilityClass"` after `"annex"` list items (array elements, not property keys)."""
    text = rd(path)
    pat = re.compile(r'"annex"(?P<sep>\s*,\s*)(?!\s*:)')
    n = len(pat.findall(text))
    if '"reliabilityClass"' + "," in text.replace(" ", "").replace("\n", "") and n == 0:
        return False
    assert n == expected, (path[-70:], "required annex items", n, expected)
    wr(path, pat.sub(lambda m: '"annex"' + m.group("sep") + '"reliabilityClass"' + m.group("sep"), text))
    return True


RC_UNION = '"RC1" | "RC2" | "RC3"'
RC_LITERAL = 'normWireLiteral("RC1", "RC2", "RC3")'


def graphql_clone(path, expected):
    def conv(s):
        s = s.replace("annex:", "reliabilityClass:")
        return re.sub(r"(reliabilityClass: )AnnexChoice", r"\1String", s)
    return clone_lines(path, r"^  annex: [^\n]*\n", conv, expected, marker="reliabilityClass:")


def proto_clone(path):
    text = rd(path)
    if "reliability_class" in text:
        return False
    m = re.search(r"^(?P<pre>  // @state artifact\n)?  (?P<opt>optional )?\w+ annex = (?P<n>\d+);\n", text, re.M)
    assert m, path[-60:]
    start = text.rfind("\nmessage ", 0, m.start()) + 1
    end = text.index("\n}", m.end() - 1)
    numbers = [int(x) for x in re.findall(r"= (\d+);", text[start:end + 2])]
    n = max(numbers) + 1
    pre = m.group("pre") or ""
    clone = f"{pre}  {m.group('opt') or ''}string reliability_class = {n};\n"
    wr(path, text[:m.end()] + clone + text[m.end():])
    return True


def ts_clone(path, diff, expected_iface=1, expected_parser=1, doc_emoji="🛡️"):
    text = rd(path)
    if "reliabilityClass" in text:
        return False
    iface = re.compile(r"^(?P<doc>  /\*\*[^\n]*\*/\n)?  annex(?P<q>\??): (?P<type>[^\n]*);\n", re.M)
    found = list(iface.finditer(text))
    assert len(found) == expected_iface, (path[-60:], "iface", len(found))
    out, last = [], 0
    for m in found:
        end = m.end()
        out.append(text[last:end])
        t = m.group("type")
        if t.startswith("string"):
            ty = f"({RC_UNION}) | null" if "null" in t else RC_UNION
        else:
            ty = f"({RC_UNION}) | null" if diff else RC_UNION
        doc = m.group("doc") or ""
        if doc:
            doc = re.sub(r"/\*\* \S+", "/** " + doc_emoji, doc, count=1)
        out.append(f"{doc}  reliabilityClass{m.group('q')}: {ty};\n")
        last = end
    out.append(text[last:])
    text = "".join(out)
    parsers = list(re.finditer(r"\bannex: normWire", text))
    assert len(parsers) == expected_parser, (path[-60:], "parser", len(parsers))
    out, last = [], 0
    for m in parsers:
        start = m.start()
        end = balanced(text, start, "(", ")")
        expr = text[start + len("annex: "):end]
        out.append(text[last:end])
        if "normWireString" in expr:
            conv = expr.replace("normWireString", RC_LITERAL)
        else:
            conv = re.sub(r'normWireLiteral\("En", "De"\)', RC_LITERAL, expr)
            conv = re.sub(r"normWireRef\(\(\) => \w*parse\w*AnnexChoice\)|\bparse\w*AnnexChoice\b", RC_LITERAL, conv)
            assert "RC1" in conv, (path[-60:], expr)
        out.append(", reliabilityClass: " + conv)
        last = end
    out.append(text[last:])
    wr(path, "".join(out))
    return True


def rs_struct_append(path, name, field_text, marker="pub reliability_class"):
    """Append `field_text` as the last field of `pub struct name`, so no positional record index of the codecs moves."""
    text = rd(path)
    m = re.search(r"pub struct " + re.escape(name) + r"\s*\{", text)
    assert m, (path[-60:], name)
    end = balanced(text, m.start(), "{", "}")
    if marker in text[m.start():end]:
        return False
    wr(path, text[:end - 1] + field_text + text[end - 1:])
    return True


def measure(sql_text):
    import sqlite3
    db = sqlite3.connect(":memory:")
    db.executescript(sql_text)
    rows = db.execute("select name, sql from sqlite_schema where type='table' order by name").fetchall()
    widths = {n: len(db.execute(f"pragma table_info({n})").fetchall()) for n, _ in rows}
    return sum(len(n.encode()) + len(q.encode()) for n, q in rows), widths


def contract_update(sem_path, sql_path, text_cell_bytes=3, ts_test_paths=()):
    """Recompute `schemaBytes`, `tableWidths` and add the bytes of the new text cell to every case, replaying the DDL with python sqlite3."""
    import json
    old_sql = ORIG.get(sql_path)
    if old_sql is None:
        return False
    new_sql = rd(sql_path)
    old_b, old_w = measure(old_sql)
    new_b, new_w = measure(new_sql)
    sem = json.loads(rd(sem_path))
    if sem["tableWidths"] != old_w:
        print("[contract] stale tableWidths", {k: (sem["tableWidths"].get(k), v) for k, v in old_w.items() if sem["tableWidths"].get(k) != v}, sem_path[-60:])
    if sem["schemaBytes"] != old_b:
        print("[contract] stale schemaBytes", sem["schemaBytes"], "replayed", old_b, sem_path[-60:])
    stored_b, stored_w = sem["schemaBytes"], sem["tableWidths"]
    sem["schemaBytes"], sem["tableWidths"] = new_b, new_w
    for case in sem["cases"]:
        case["valueBytes"] += text_cell_bytes
    wr(sem_path, json.dumps(sem, indent=2, ensure_ascii=False) + "\n")
    for t in ts_test_paths:
        text = rd(t)
        text = re.sub(r'(\["schemaBytes"\]\)\.toEqual\()\d+', lambda m: m.group(1) + str(new_b), text)
        text = re.sub(r'(\["tableWidths"\]\)\.toEqual\()\{[^}]*\}', lambda m: m.group(1) + json.dumps(new_w, separators=(",", ":")), text)
        wr(t, text)
    return (old_b, new_b)


def top_level_key(text, key):
    """(start, value_start, value_end) of `"key":` at depth 1 of a JSON object text, scalar values only; None when absent."""
    depth, i, n = 0, 0, len(text)
    while i < n:
        c = text[i]
        if c == '"':
            j = i + 1
            while text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            if depth == 1 and text[j + 1:j + 2] == ":" and text[i + 1:j] == key:
                k = j + 2
                while text[k] in " \t\n":
                    k += 1
                if text[k] == '"':
                    e = k + 1
                    while text[e] != '"':
                        e += 2 if text[e] == "\\" else 1
                    e += 1
                else:
                    e = k
                    while text[e] not in ",}\n ":
                        e += 1
                return i, k, e
            i = j
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
        i += 1
    return None


def snapshot_json_insert(text, is_diff, value="RC2", anchor="annex"):
    """Insert `"reliabilityClass"` after the depth-1 `anchor` value of a snapshot or diff JSON text; None when no insertion applies."""
    if top_level_key(text, "reliabilityClass"):
        return None
    hit = top_level_key(text, anchor)
    if not hit:
        return None
    start, vs, ve = hit
    after_colon = text[start:vs]
    m = re.match(r",(\s*)", text[ve:])
    ws = m.group(1) if m else "\n"
    key_part = '"reliabilityClass":' + after_colon.split(":", 1)[1] + ("null" if is_diff else f'"{value}"')
    return text[:ve] + "," + ws + key_part + text[ve:]


def fixture_sweep(root, required_keys, diff_keys, skip=lambda p: False, value="RC2"):
    """Walk every JSON under `root`; snapshots (all required keys at depth 1) get the value, diffs (path has a diff directory and the anchor) get null."""
    import json
    changed = 0
    for dirpath, _, names in os.walk(root):
        for name in names:
            if not name.endswith(".json"):
                continue
            path = os.path.join(dirpath, name)
            if skip(path):
                continue
            text = open(path, encoding="utf-8").read()
            if '"annex"' not in text or '"reliabilityClass"' in text:
                continue
            try:
                data = json.loads(text)
            except ValueError:
                continue
            if not isinstance(data, dict):
                continue
            keys = set(data)
            is_diff = "🔺️diff" in path
            if not is_diff and required_keys <= keys:
                new = snapshot_json_insert(text, False, value)
            elif is_diff and keys <= diff_keys and "annex" in keys:
                new = snapshot_json_insert(text, True)
            else:
                continue
            if new:
                wr(path, new)
                changed += 1
    return changed


def dsl_asset_insert(path, token="reliability-class=rc2"):
    text = rd(path)
    if "reliability-class=" in text:
        return False
    lines = text.split("\n")
    for i, line in enumerate(lines[:6]):
        if line.startswith("annex="):
            head, _, rest = line.partition(" ")
            lines[i] = head + " " + token + (" " + rest if rest else "")
            wr(path, "\n".join(lines))
            return True
    raise AssertionError(("no annex line", path[-70:]))


def json_reorder_last(path):
    """Move `reliabilityClass` to the end of the top-level `properties` and `required`, matching the Rust field order."""
    import json
    text = rd(path)
    d = json.loads(text)
    props = d.get("properties")
    if not props or "reliabilityClass" not in props:
        return False
    changed = list(props)[-1] != "reliabilityClass"
    props["reliabilityClass"] = props.pop("reliabilityClass")
    rc = props["reliabilityClass"]
    if rc.get("type") == "string" and "enum" not in rc:
        rc["enum"] = list(RC)
        changed = True
    req = d.get("required")
    if req and "reliabilityClass" in req and req[-1] != "reliabilityClass":
        req.remove("reliabilityClass")
        req.append("reliabilityClass")
        changed = True
    if not changed:
        return False
    import sys as _s
    _s.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts")
    from schema_json import dump_schema
    out = dump_schema(d)
    wr(path, out)
    return True



def _expr_end(text, k):
    depth = 0
    j = k
    while True:
        c = text[j]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                return j
            depth -= 1
        elif c == "," and depth == 0:
            return j
        j += 1


def twin_last_ts(path):
    """Move the `reliabilityClass` member of every interface and of the wire parser to the end."""
    text = rd(path)
    out = text
    for m in list(re.finditer(r"^(?:  /\*\*[^\n]*\*/\n)?  reliabilityClass\??: [^\n]*;\n", out, re.M)):
        pass
    m = re.search(r"^(?:  /\*\*[^\n]*\*/\n)?  reliabilityClass\??: [^\n]*;\n", out, re.M)
    if m:
        end = out.index("\n}", m.end() - 1)
        if out[m.end():end + 1].strip():
            block = m.group(0)
            out = out[:m.start()] + out[m.end():]
            end = out.index("\n}", m.start() - 1)
            out = out[:end + 1] + block + out[end + 1:]
    p = re.search(r", reliabilityClass: ", out)
    if p:
        k = p.end()
        j = _expr_end(out, k)
        expr = out[k:j]
        ob = out.rindex("normWireObject", 0, p.start())
        close = balanced(out, ob, "{", "}") - 1
        if out[j:close].strip():
            out = out[:p.start()] + out[j:]
            close = balanced(out, ob, "{", "}") - 1
            out = out[:close].rstrip() + ", reliabilityClass: " + expr + " " + out[close:]
    if out == text:
        return False
    wr(path, out)
    return True


def twin_last_graphql(path):
    text = rd(path)
    m = re.search(r"^  reliabilityClass: [^\n]*\n", text, re.M)
    if m:
        end = text.index("\n}", m.end() - 1)
        if not text[m.end():end + 1].strip():
            return False
        out = text[:m.start()] + text[m.end():]
        end = out.index("\n}", m.start() - 1)
        wr(path, out[:end + 1] + m.group(0) + out[end + 1:])
        return True
    m = re.search(r"reliabilityClass: String!? @state\(class: ARTIFACT\) ", text)
    if not m:
        return False
    start = text.rindex("type ", 0, m.start())
    end = balanced(text, start, "{", "}") - 1
    if not text[m.end():end].strip():
        return False
    out = text[:m.start()] + text[m.end():]
    end = balanced(out, start, "{", "}") - 1
    wr(path, out[:end] + m.group(0) + out[end:])
    return True


def twin_last_proto(path):
    text = rd(path)
    m = re.search(r"(?:  // @state artifact\n)?  (?:optional )?string reliability_class = \d+;\n", text)
    if m:
        end = text.index("\n}", m.end() - 1)
        if not text[m.end():end + 1].strip():
            return False
        out = text[:m.start()] + text[m.end():]
        end = out.index("\n}", m.start() - 1)
        wr(path, out[:end + 1] + m.group(0) + out[end + 1:])
        return True
    m = re.search(r" (?:optional )?string reliability_class = \d+;", text)
    if not m:
        return False
    start = text.rindex("message ", 0, m.start())
    end = balanced(text, start, "{", "}") - 1
    if not text[m.end():end].strip():
        return False
    out = text[:m.start()] + text[m.end():]
    end = balanced(out, start, "{", "}") - 1
    wr(path, out[:end].rstrip() + m.group(0) + " " + out[end:])
    return True
