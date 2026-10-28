"""🇩🇪 Shared readers of lane germanqa: the German segments of a tree (JSON bilingual leaves and tables, `EN: … DE: …` descriptions, German markdown sections, German string literals of sources)."""
import json, os, re, subprocess

ROOTS = ["🧰️framework/🛍️products/🦑️repo", "🧰️framework/🔨️modules/🏃️process"]
CODE_EXT = (".ts", ".mts", ".mjs", ".rs", ".py", ".go")
UMLAUT = re.compile(r"[äöüÄÖÜß]")
FUNCTION_WORDS = re.compile(r"\b(?:und|nicht|wird|werden|oder|keine?|noch|für|ist|der|die|das|dem|den|mit|von|auf|bei|nur|eine|einen|erforderlich|fehlt|Befehl|Lauf|wurde)\b")


TRIGGERS = re.compile(r"(?i)Aufgabe|Closure-|Hülle|Beleg|Abbild|Besitzer|Inhaber|Benutzer|angeheftet|Fehlgriff|Einsammler|Maschine|Plattenbudget|Bahn|Policy-|Sitzungsplatz|Deadlock-|Kompiliereinheit|Mikro-Commit|Quota-|Inkrementell|Std\.|Min\.")


def german_like(text, translit_tokens):
    if UMLAUT.search(text) or TRIGGERS.search(text): return True
    if len(FUNCTION_WORDS.findall(text)) >= 2: return True
    return any(t in text for t in translit_tokens)
LITERAL = re.compile(r'"(?:[^"\\\n]|\\.)*"|`(?:[^`\\]|\\.)*`|\'(?:[^\'\\\n]|\\.)*\'')
DE_HEADING = re.compile(r"^#+ .*(?:Deutsch|🇩🇪)")
MARKERS = re.compile(r"(?:(?<=\s)|^)(EN|DE): ")


def files_of(root, exts):
    out = subprocess.run(["git", "-C", root, "ls-files", "-co", "--exclude-standard", "-z", "--"] + ROOTS, capture_output=True, check=True).stdout.decode().split("\0")
    return sorted(f for f in out if f.endswith(exts) and os.path.exists(os.path.join(root, f)))


def _flat(table, base, into):
    for k, v in table.items():
        if isinstance(v, str): into[f"{base}/{k}"] = v
        elif isinstance(v, dict): _flat(v, f"{base}/{k}", into)
        elif isinstance(v, list):
            for i, x in enumerate(v):
                if isinstance(x, str): into[f"{base}/{k}/{i}"] = x


def pairs_of(doc):
    """yield (path, en, de) for every bilingual leaf and table entry of a parsed JSON document; also (path, None, None) markers are not produced."""
    out = []

    def walk(v, path):
        if isinstance(v, list):
            for i, x in enumerate(v): walk(x, f"{path}/{i}")
            return
        if not isinstance(v, dict): return
        keys = set(v)
        if keys > {"en", "de"} and isinstance(v["en"], str) and isinstance(v["de"], str):
            out.append((path, v["en"], v["de"]))
        if keys == {"en", "de"}:
            if isinstance(v["en"], str) and isinstance(v["de"], str):
                out.append((path, v["en"], v["de"])); return
            if isinstance(v["en"], dict) and isinstance(v["de"], dict):
                e, d = {}, {}
                _flat(v["en"], "", e); _flat(v["de"], "", d)
                for k in sorted(set(e) | set(d)): out.append((path + k, e.get(k), d.get(k)))
                return
            if isinstance(v["en"], list) and isinstance(v["de"], list):
                for i in range(max(len(v["en"]), len(v["de"]))):
                    out.append((f"{path}/{i}", v["en"][i] if i < len(v["en"]) else None, v["de"][i] if i < len(v["de"]) else None))
                return
        for k, x in v.items(): walk(x, f"{path}/{k}")

    walk(doc, "")
    return out


def description_pairs(doc):
    """(path, en, de) for strings written as `EN: … DE: …` (several markers allowed)."""
    out = []

    def walk(v, path):
        if isinstance(v, dict):
            for k, x in v.items(): walk(x, f"{path}/{k}")
        elif isinstance(v, list):
            for i, x in enumerate(v): walk(x, f"{path}/{i}")
        elif isinstance(v, str) and re.search(r"(?:^|\s)DE: ", v) and re.search(r"(?:^|\s)EN: ", v):
            segs = split_markers(v)
            out.append((path, " ".join(t for k, t in segs if k == "EN"), " ".join(t for k, t in segs if k == "DE")))

    walk(doc, "")
    return out


def split_markers(text):
    """[(kind, text)] of an `EN: … DE: …` string; text before the first marker has kind None."""
    parts, last, kind = [], 0, None
    for m in MARKERS.finditer(text):
        if m.start() > last or kind is not None: parts.append((kind, text[last:m.start()]))
        kind, last = m.group(1), m.end()
    parts.append((kind, text[last:]))
    return [(k, t) for k, t in parts if t != "" or k is not None]


def language_leaves(doc):
    """(path, text) of every string below a JSON object that carries the sibling `language: "de"` (rendered expectations of fixtures)."""
    out = []

    def strings(v, path):
        if isinstance(v, str): out.append((path, v))
        elif isinstance(v, dict):
            for k, x in v.items(): strings(x, f"{path}/{k}")
        elif isinstance(v, list):
            for i, x in enumerate(v): strings(x, f"{path}/{i}")

    def walk(v, path):
        if isinstance(v, dict):
            env = v.get("environment") if isinstance(v.get("environment"), dict) else v.get("env") if isinstance(v.get("env"), dict) else {}
            if v.get("language") == "de" or v.get("lang") == "de" or any(str(env.get(k, "")).startswith("de") for k in ("SEMIO_LANGUAGE", "LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG")):
                for k, x in v.items():
                    if k not in ("language", "lang", "row", "environment", "env", "name"): strings(x, f"{path}/{k}")
            for k, x in v.items(): walk(x, f"{path}/{k}")
        elif isinstance(v, list):
            for i, x in enumerate(v): walk(x, f"{path}/{i}")

    walk(doc, "")
    return out


def md_german_lines(text):
    """[(index, line)] of the German part of a markdown document outside fenced blocks (from the first heading naming Deutsch)."""
    lines = text.split("\n")
    start = next((i for i, l in enumerate(lines) if DE_HEADING.match(l)), None)
    if start is None: return []
    fenced, out = False, []
    for i in range(start, len(lines)):
        if lines[i].lstrip().startswith("```"): fenced = not fenced; continue
        if not fenced: out.append((i, lines[i]))
    return out


def code_literals(text, translit_tokens):
    """[(start, end, content)] of German-looking string literals of a source file."""
    out = []
    for m in LITERAL.finditer(text):
        s = m.group(0)[1:-1]
        if len(s) >= 4 and german_like(s, translit_tokens): out.append((m.start() + 1, m.end() - 1, s))
    return out


_EN_STOP = re.compile(r"\b(?:the|of|and|is|are|to|for|with|not|that|this|from|when|which|must|may|by|an|in|on|it|its|at|as|be)\b")
_DE_STOP = re.compile(r"\b(?:der|die|das|und|nicht|ist|sind|wird|werden|mit|von|auf|für|fuer|zu|im|bei|oder|ein|eine|einen|dem|den|des|aus|nur|noch|jede[rsnm]?|ohne|über|ueber|wenn|dass|als|durch|nach|vor|bis|auch|kein|keine)\b")


def slash_split(text, translit_tokens):
    """(english, german) of a string written `English / Deutsch`, else None. The split is the ` / ` that maximises English-ness left plus German-ness right."""
    best, score = None, 0
    for m in re.finditer(r" / ", text):
        left, right = text[:m.start()], text[m.end():]
        le, ld, re_, rd = len(_EN_STOP.findall(left)), len(_DE_STOP.findall(left)), len(_EN_STOP.findall(right)), len(_DE_STOP.findall(right))
        strong = le >= 3 and rd >= 3 and le > ld and rd > re_
        short = len(text) < 200 and text.count(" / ") == 1 and ld == 0 and re_ == 0 and german_like(right, translit_tokens) and not german_like(left, translit_tokens)
        value = (le - ld) + (rd - re_) + (1 if short else 0)
        if (strong or short) and value > score: best, score = (left, right), value
    return best
