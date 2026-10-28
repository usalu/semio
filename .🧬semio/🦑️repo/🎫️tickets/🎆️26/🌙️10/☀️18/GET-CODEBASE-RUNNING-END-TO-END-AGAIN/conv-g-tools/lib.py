import re, unicodedata
FE0F = "️"
MARK = re.compile(r"^(?:[\U0001F000-\U0001FAFF←-⇿⌀-⏿①-➿⤀-⥿⬀-⯿©®‼⁉™ℹ〰〽㊗㊙■-◿☀-⛿✀-➿√⊕]|[0-9#*]️?⃣)")
def first_grapheme(s):
    m = re.match(r"[0-9#*]️?⃣", s)
    if m: return m.group(0)
    i = 1
    n = len(s)
    while i < n:
        c = s[i]
        if c == FE0F or c == "‍" or 0x1F3FB <= ord(c) <= 0x1F3FF or c == "⃣":
            i += 1
            if c == "‍" and i < n: i += 1
            continue
        break
    return s[:i]
def openers(path, text):
    lines = text.split("\n"); out = []
    def add(i, c): out.append((i + 1, c.strip()))
    if path.endswith(".rs"):
        run = None; opened = False
        for i, raw in enumerate(lines):
            l = raw.lstrip()
            if not re.match(r"^//[/!](?!/)", l): run = None; continue
            if run != l[:3]: opened = False
            run = l[:3]
            if opened or not l[3:].strip(): continue
            opened = True; add(i, l[3:])
    if path.endswith(".py"):
        return out
    op = False
    for i, raw in enumerate(lines):
        if not op:
            s = raw.find("/**")
            if s < 0 or raw[s+3:s+4] in ("/", "*") or re.search(r"//|[\"'`]", raw[:s]): continue
            rest = raw[s+3:]; ca = rest.find("*/")
            content = rest[:ca] if ca >= 0 else rest
            if content.strip(): add(i, content); continue
            op = ca < 0; continue
        ca = raw.find("*/")
        content = re.sub(r"^\s*\*?", "", raw[:ca] if ca >= 0 else raw)
        if content.strip(): op = False; add(i, content); continue
        if ca >= 0: op = False
    return sorted(out)
def reuse(path, text):
    d = {}
    for ln, c in openers(path, text):
        if not MARK.match(c): continue
        e = first_grapheme(c).replace(FE0F, "")
        d.setdefault(e, []).append(ln)
    return {e: l for e, l in d.items() if len(l) > 1}
