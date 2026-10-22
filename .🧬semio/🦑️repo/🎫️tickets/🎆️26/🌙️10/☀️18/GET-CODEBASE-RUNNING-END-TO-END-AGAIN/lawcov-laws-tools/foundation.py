import unicodedata

ALIASES = {"protocol": "spr"}
LEAF = "🦀.rs"


def strip_emoji(s):
    return re.sub(r"[^\x00-\x7f]", "", s)


def app_id(path):
    seg = path.split("/")
    if "🎛apps" in seg:
        i = seg.index("🎛apps")
        if len(seg) > i + 1:
            return strip_emoji(seg[i + 1])
    return ""


def component(seg):
    a = strip_emoji(seg)
    return ALIASES.get(a, a)


def file_suffix(tail, default):
    if len(tail) == 1 and tail[0] == default:
        return ""
    return "#" + "-".join(strip_emoji(re.sub(r"\.rs$", "", s)) for s in tail)


def normalize_rel(rel):
    norm = rel[2:] if rel.startswith(".") else rel
    seg = norm.split("/")
    if "⚡implementations" in seg and seg.index("⚡implementations") > 1:
        ii = seg.index("⚡implementations")
        module = seg[ii - 1]
        suffix = file_suffix(seg[ii + 2:], LEAF)
        chain = [s for s in seg[:ii - 1] if s not in ("🔨modules", "🛂manifest")]
        if "🔌plugins" in chain and len(chain) > chain.index("🔌plugins") + 1:
            pi = chain.index("🔌plugins")
            plugin = strip_emoji(chain[pi + 1])
            app = strip_emoji(chain[chain.index("🎛apps") + 1]) if "🎛apps" in chain and len(chain) > chain.index("🎛apps") + 1 else None
            comp = component(module)
            return ("%s/%s/%s" % (plugin, app, comp) if app and app != plugin else "%s/%s" % (plugin, comp)) + suffix
        owner = strip_emoji(chain[-1] if chain else "")
        if owner:
            return "%s/%s%s" % (owner, component(module), suffix)
    if "🗿artifacts" in seg:
        ai = seg.index("🗿artifacts")
        if ai > 0 and len(seg) > ai + 2:
            plugin = strip_emoji(seg[ai - 1])
            art = strip_emoji(seg[ai + 1])
            comp = component(seg[ai + 2])
            suffix = file_suffix(seg[ai + 3:], LEAF)
            return ("%s/%s/%s" % (plugin, art, comp) if art and art != plugin else "%s/%s" % (plugin, comp)) + suffix
    return norm


def region_events(lines):
    out = []
    for i, line in enumerate(lines):
        for kind, rx in (("open", r"^(\s*)//(\s*)#region(?:\s+(.*))?\s*$"), ("close", r"^(\s*)//(\s*)#endregion(?:\s+(.*))?\s*$")):
            m = re.match(rx, line)
            if m:
                out.append({"kind": kind, "line": i + 1, "spaceAfterSlashes": len(m.group(2) or "") > 0, "label": (m.group(3) or "").strip()})
                break
    return out


def pair_spans(events):
    stack, out = [], []
    for ev in events:
        if ev["kind"] == "open":
            stack.append(ev)
            continue
        if not stack:
            continue
        op = stack.pop()
        out.append({"label": op["label"], "closeLabel": ev["label"], "startLine": op["line"], "endLine": ev["line"]})
    return out


def mod_spans(lines):
    stack, out, depth = [], [], 0
    for i, raw in enumerate(lines):
        co = code_only(raw)
        m = re.match(r"^\s*pub mod (\w+)\b.*\{\s*$", raw, A)
        if m:
            stack.append((m.group(1), i + 1, depth))
        depth += co.count("{") - co.count("}")
        while stack and depth <= stack[-1][2]:
            name, start, _ = stack.pop()
            out.append({"name": name, "startLine": start, "endLine": i + 1})
    return out


def mod_at(spans, line):
    inside = sorted((s for s in spans if s["startLine"] <= line <= s["endLine"]), key=lambda s: s["endLine"] - s["startLine"])
    return inside[0]["name"] if inside else ""


def label_name(label):
    i = 0
    while i < len(label) and not unicodedata.category(label[i]).startswith("L"):
        i += 1
    return label[i:].strip()


def extract_body(content, start):
    masked = "\n".join(mask(l) for l in content.split("\n"))
    op = masked.find("{", start)
    if op == -1:
        return ""
    depth = 0
    for i in range(op, len(masked)):
        if masked[i] == "{":
            depth += 1
        elif masked[i] == "}":
            depth -= 1
            if depth == 0:
                return content[op:i + 1]
    return content[op:]


def pascal(i):
    return "".join(p[:1].upper() + p[1:] for p in re.split(r"[-_]+", i) if p) + "App"


