#!/usr/bin/env python3
"""🇩🇪️ Builds the checked-in EN→DE mutation-label table `🐍️u3-de-glossary.json`.

Harvests every English `MutationKind::label()` template in the tree, translates it with an
authored term dictionary plus German UI grammar rules (infinitive-final imperative), and writes
the exhaustive table. A content word that is neither in `TERMS` nor declared locale-invariant in
`INVARIANT` aborts the build: there is no English pass-through for prose.
"""
import json, os, re, sys, collections

HERE = os.path.dirname(os.path.abspath(__file__))
def find_repo(start):
    """🌳️ Walks up to the repository root (the directory owning `🧰️framework`)."""
    node = start
    while node != os.path.dirname(node):
        if os.path.isdir(os.path.join(node, "🧰️framework")):
            return node
        node = os.path.dirname(node)
    raise SystemExit("repository root not found")

REPO = find_repo(HERE)
ROOTS = ["✏️s", "🧰️framework"]
TRAIT_ANCHORS = ("MutationKind", "CompositeMutationKind", "SemanticMutation")

# 🔤️ Tokens that are identical in both languages by declaration: SI units, physical symbols,
# file-format and standard names, proper nouns, single-letter variables.
INVARIANT = set("""
m mm cm km kg g t s ms h d n v k f r w j c a b e i l p q u x y z kn knm nm mpa gpa kpa pa
hz w2 m2 m3 m4 wm wmk ppm rgb rgba srgb cmyk dpi ppi px pt em rem lod uv url uri uuid id ids
pdf svg png jpg jpeg jpg2000 gif tiff webp obj gltf glb stl step ifc dxf dwg csv json xml yaml
html css js ts docx xlsx pptx vml zip lzw rle ascii utf utf8 crc md5 sha
en iso din vdi ashrae en1990 en1991 en1992 en1993 en1994 en1995 en1996 en1997 en1998 en1999
gcp pv wfc fem cad gis bim brep nurbs csg dag lbs cop eer seer cfd hvac vav cav ahu
ok api sdk cli gpu cpu ram io ui ux http https ws wss tcp udp dns tls ssl jwt
x1 x2 y1 y2 z1 z2 u1 u2 v1 v2 i j
shw ed v_ed kwh m_ed h_ed v_rd min mu vlr ef yk n_ed h_rd deg eff rho rd m_pl int
javascript re ext geo ifd var ach poisson's nu dc vert uk q_b bb d_ed d_rd l_cr chi f_k f_vk
e_cm f_ck beta_w c_d h_ef t_ef w_el ck theta_c p_rd d_f phi hd lambda ct crit g_k psi a_gr q_p
r_k gamma_el e_d rh a_ed alpha_s q_s k_sigma n_rd c_s e_s f_y h_sc f_u mime i_t m_rd udl
lhs rhs vcs qc rotatenode scalenode eval acro sfm jfif fmt ftyp dst icc idx mtllib sof usemtl
""".split())

# 🧭️ English verb head → German infinitive, with the preposition a trailing `to <target>` takes.
VERBS = {
  "add": ("hinzufügen", "zu"), "append": ("anhängen", "an"), "apply": ("anwenden", "auf"),
  "attach": ("anhängen", "an"), "bind": ("binden", "an"), "change": ("ändern", "auf"),
  "clear": ("leeren", None), "commit": ("festschreiben", None), "connect": ("verbinden", "mit"),
  "copy": ("kopieren", "nach"), "create": ("erstellen", None), "cut": ("ausschneiden", None),
  "declare": ("deklarieren", None), "delete": ("löschen", None), "demote": ("herabstufen", None),
  "deselect": ("abwählen", None), "detach": ("ablösen", "von"), "disable": ("deaktivieren", None),
  "disconnect": ("trennen", "von"), "drag": ("ziehen", "nach"), "duplicate": ("duplizieren", None),
  "edit": ("bearbeiten", None), "embed": ("einbetten", "in"), "enable": ("aktivieren", None),
  "insert": ("einfügen", "in"), "mask": ("maskieren", None), "merge": ("zusammenführen", "mit"),
  "move": ("verschieben", "nach"), "paint": ("malen", None), "paste": ("einfügen", "in"),
  "pin": ("anheften", "an"), "promote": ("hochstufen", None), "remove": ("entfernen", "aus"),
  "rename": ("umbenennen", "in"), "reorder": ("umordnen", None), "replace": ("ersetzen", "durch"),
  "reset": ("zurücksetzen", "auf"), "resize": ("skalieren", "auf"), "restore": ("wiederherstellen", None),
  "rotate": ("drehen", "um"), "scale": ("skalieren", "auf"), "select": ("auswählen", None),
  "set": ("setzen", "auf"), "sign": ("signieren", None), "split": ("teilen", "an"),
  "stamp": ("stempeln", None), "start": ("starten", None), "toggle": ("umschalten", None),
  "unbind": ("lösen", "von"), "unmask": ("demaskieren", None), "unpin": ("abheften", "von"),
  "update": ("aktualisieren", "auf"), "upsert": ("einfügen oder aktualisieren", "in"),
}

def load_terms():
    with open(os.path.join(HERE, "🐍️u3-de-terms.json"), encoding="utf-8") as fh:
        raw = json.load(fh)
    return raw["adjectives"], raw["modifiers"], raw["nouns"], raw["phrases"]

ADJ, MOD, NOUN, PHRASE = load_terms()
MISSING = collections.Counter()

def translate_run(words):
    """🧩️ Translates one run of English content words into a German noun phrase.

    A whole-run phrase entry wins; otherwise adjectives stay separate and leading modifiers
    compound onto the head noun the way German technical vocabulary does.
    """
    words = [w for w in words if w is not None]
    key = " ".join(w.lower() for w in words)
    if key in PHRASE:
        return PHRASE[key]
    adjs, nouns = [], []
    for w in words:
        lo = w.lower()
        if lo in ADJ:
            adjs.append(ADJ[lo] if ADJ[lo].endswith("e") else ADJ[lo] + "e")
        elif lo in MOD:
            nouns.append(MOD[lo])
        elif lo in NOUN:
            nouns.append(NOUN[lo])
        elif lo in INVARIANT or re.fullmatch(r"[a-z]?[0-9][a-z0-9_,.]*", lo) or (w.isupper() and len(w) > 1):
            nouns.append(w)
        else:
            MISSING[lo] += 1
            nouns.append("«" + lo + "»")
    if not nouns:
        return " ".join(a[:-1] if a.endswith("e") and a[:-1].lower() in ADJ.values() else a for a in adjs)
    head = nouns[-1]
    if len(nouns) > 1:
        stem = "".join(linking(n[0].lower() + n[1:] if i else n) for i, n in enumerate(nouns[:-1]))
        head = stem + head[0].lower() + head[1:]
        head = head[0].upper() + head[1:]
    return (" ".join(adjs) + " " + head).strip() if adjs else head

FUGEN_S = ("ung", "heit", "keit", "ion", "tät", "schaft", "ling", "tum", "ität")

def linking(stem):
    """🔗️ German compound linking morpheme: `-ung`/`-heit`/`-ion`/… stems take a joining `s`."""
    if stem.endswith(FUGEN_S):
        return stem + "s"
    if len(stem) > 4 and stem.endswith("e"):
        return stem + "n"
    return stem


TOKEN = re.compile(r"""(\{[^{}]*\}|\\"|"|[A-Za-z][A-Za-z_']*|[^A-Za-z{]+)""")

def translate_phrase(text):
    """🧵️ Translates a mixed run of words, placeholders and punctuation, preserving hole order."""
    out, run = [], []
    def flush():
        if run:
            lead = " " if run[0] is None else ""
            tail = " " if run[-1] is None else ""
            out.append(lead + translate_run(run) + tail)
            run.clear()
    for tok in TOKEN.findall(text):
        if re.fullmatch(r"[A-Za-z][A-Za-z_']*", tok):
            run.append(tok)
        elif tok.isspace():
            run.append(None)
        else:
            flush()
            out.append(tok)
    flush()
    joined = "".join(out)
    return re.sub(r"[ ]{2,}", " ", joined).strip()

PREPS = {"to": "zu", "of": "von", "from": "aus", "in": "in", "on": "auf", "at": "an", "for": "für",
         "with": "mit", "by": "durch", "and": "und", "or": "oder", "into": "in", "onto": "auf",
         "the": "", "a": "", "an": "", "as": "als", "per": "pro", "between": "zwischen", "over": "über", "since": "seit",
         "via": "über", "before": "vor", "through": "durch", "without": "ohne", "out": "aus",
         "is": "", "has": "", "than": "als", "not": "nicht", "its": "", "their": ""}

def translate_body(text):
    """🧵️ Word-run translation that also rewrites English prepositions and drops articles."""
    out, run = [], []
    def flush():
        if run:
            lead = " " if run[0] is None else ""
            tail = " " if run[-1] is None else ""
            out.append(lead + translate_run(run) + tail)
            run.clear()
    for tok in TOKEN.findall(text):
        if re.fullmatch(r"[A-Za-z][A-Za-z_']*", tok) and tok.lower() in PREPS:
            flush()
            german = PREPS[tok.lower()]
            out.append(" " + german + " " if german else " ")
        elif re.fullmatch(r"[A-Za-z][A-Za-z_']*", tok):
            run.append(tok)
        elif tok.isspace():
            run.append(None)
        else:
            flush()
            out.append(tok)
    flush()
    return re.sub(r"[ ]{2,}", " ", "".join(out)).strip()

def translate_template(raw):
    """🇩🇪️ English `label()` template → German template with the identical placeholder order."""
    text = raw
    kebab = re.fullmatch(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+", text)
    if kebab:
        text = text.replace("-", " ")
    words = text.split()
    head = words[0].lower() if words else ""
    if head in VERBS and len(words) > 1:
        verb, prep = VERBS[head]
        rest = text[len(words[0]):].strip()
        target = None
        match = re.search(r"\s+to\s+", rest)
        if match and prep:
            target, rest = rest[match.end():], rest[:match.start()]
        german = translate_body(rest)
        if target is not None:
            german = german + " " + prep + " " + translate_body(target)
        return (german + " " + verb).strip()
    if head in VERBS:
        return VERBS[head][0][0].upper() + VERBS[head][0][1:]
    return translate_body(text)

def harvest(repo):
    """🔍️ Every anchored `MutationKind::label()` string literal in the tree, with its use count."""
    counts = collections.Counter()
    for root in ROOTS:
        for dirpath, _, names in os.walk(os.path.join(repo, root)):
            for name in names:
                if name != "🦀️.rs":
                    continue
                path = os.path.join(dirpath, name)
                try:
                    source = open(path, encoding="utf-8").read()
                except (OSError, UnicodeDecodeError):
                    continue
                if "fn label(&self) -> String" not in source:
                    continue
                for body in anchored_bodies(source):
                    for literal in re.findall(r'"((?:[^"\\]|\\.)*)"', body):
                        counts[literal] += 1
    return counts

def anchored_bodies(source):
    """🎯️ Bodies of `fn label` declared inside an `impl <mutation trait> for` region only."""
    lines = source.split("\n")
    current = None
    for index, line in enumerate(lines):
        if re.match(r"^\s*impl\b", line):
            current = line.strip()
        if "fn label(&self) -> String" not in line:
            continue
        if current is None or " for " not in current or not any(t in current for t in TRAIT_ANCHORS):
            continue
        depth = line.count("{") - line.count("}")
        if depth == 0:
            yield line.split("{", 1)[1].rsplit("}", 1)[0]
            continue
        collected, cursor = [], index
        while depth > 0:
            cursor += 1
            collected.append(lines[cursor])
            depth += lines[cursor].count("{") - lines[cursor].count("}")
        yield "\n".join(collected[:-1])

def main():
    counts = harvest(REPO)
    table, holes = {}, []
    for literal in sorted(counts):
        german = translate_template(literal)
        if re.findall(r"\{[^{}]*\}", literal) != re.findall(r"\{[^{}]*\}", german):
            holes.append((literal, german))
        table[literal] = german
    if MISSING:
        print(f"untranslated content words: {len(MISSING)}", file=sys.stderr)
        print(" ".join(w for w, _ in MISSING.most_common()), file=sys.stderr)
    if holes:
        print(f"placeholder-order breaks: {len(holes)}", file=sys.stderr)
        for en, de in holes[:20]:
            print(f"  {en!r} -> {de!r}", file=sys.stderr)
    out = os.path.join(HERE, "🐍️u3-de-glossary.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(table, fh, ensure_ascii=False, indent=1, sort_keys=True)
        fh.write("\n")
    print(f"templates: {len(table)}  sites: {sum(counts.values())}  missing-words: {len(MISSING)}  hole-breaks: {len(holes)}")
    return 1 if (MISSING or holes) else 0

if __name__ == "__main__":
    sys.exit(main())
