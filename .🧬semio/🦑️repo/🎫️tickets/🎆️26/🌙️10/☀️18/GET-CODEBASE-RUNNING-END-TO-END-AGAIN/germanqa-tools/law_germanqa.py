#!/usr/bin/env python3
"""⚖️ Law of lane germanqa (independent of rules.py): the German texts of the repo tool follow one glossary, mirror their English fields and keep real umlauts.
usage: python3 -I law_germanqa.py <root> [--files-from manifest.json] [--limit N] [--json out.json] | --selftest
Checks over the bilingual JSON catalogs, `EN: … DE: …` schema descriptions, the German markdown sections and the German string literals of the sources:
  parity       EN and DE hold the same {fields}, `code spans` and numbers, both sides exist, DE is not empty and not an untranslated English sentence
               (oracle: string.Formatter().parse for the fields)
  translit     no ASCII fallback of an umlaut or ß (explicit map derived from the macOS German spell checker + any token whose umlaut twin occurs in the tree)
  glossary     one German term per concept (forbidden variants, EN-aligned where the concept is visible in the English field)
  grammar      Lease/Task/Crate gender, du instead of Sie, Agenten, Nutzer, lowercase verb in titles, `n %` / unit spellings"""
import json, os, re, string, subprocess, sys

ROOTS = ["🧰️framework/🛍️products/🦑️repo", "🧰️framework/🔨️modules/🏃️process"]
HERE = os.path.dirname(os.path.abspath(__file__))
CODE_EXT = (".ts", ".mts", ".mjs", ".rs", ".py", ".go")
SKIP = set()
TRANSLIT_ONLY = {"🧰️framework/🛍️products/🦑️repo/README.md"}
TASK_EXEMPT = ("🔨️modules/🎛️dashboard/", "💻️client/🛠️developer-tools/", "📋️project.json")
UMLAUT = re.compile(r"[äöüÄÖÜß]")
FUNCTION = re.compile(r"\b(?:und|nicht|wird|werden|oder|keine?|noch|für|ist|der|die|das|dem|den|mit|von|auf|bei|nur|eine|einen|erforderlich|fehlt|Befehl|Lauf|wurde)\b")
LITERAL = re.compile(r'"(?:[^"\\\n]|\\.)*"|`(?:[^`\\]|\\.)*`|\'(?:[^\'\\\n]|\\.)*\'')
HEADING = re.compile(r"^#+ .*(?:Deutsch|🇩🇪)")
MARKER = re.compile(r"(?:(?<=\s)|^)(EN|DE): ")
PLACEHOLDER = re.compile(r"\$?\{([A-Za-z_][A-Za-z0-9_.]*)\}")
SPAN = re.compile(r"`[^`\n]*`")
ENGLISH_WORDS = re.compile(r"\b(?:the|of|and|is|to|for|with|not|are|was|must|may|that|this|from|when|which|into|its|has|have|until|before|after)\b")
STEMS = re.compile(r"(?:^|[^A-Za-z])(?:fuer|ueber|koennen|muessen|duerfen|laeuft|laeufe|haelt|traegt|waechst|zaehlt|hoechstens|zurueck|gegenueber|waehrend|ausfuehr\w*|pruef\w*|schluessel\w*|veroeffent\w*|unabhaeng\w*|Abhaeng\w*|geaender\w*|Aender\w*|Vordertuer|Tuer|befoerder\w*|Befoerder\w*|frueher\w*|aelter\w*|Huelle|Identitaet|Domaene\w*|ausserhalb|heisst|massgeblich|Groesse|temporaer\w*|regulaer\w*)\b")

# (rule id, regex over the German text, hint)  – checked on every German segment
DE_FORBIDDEN = [
    ("closure", r"\bClosures?\b|Closure-|-Closures?\b|Hüllen-Inventar|Historien-Hülle|Zero-Touch-Hüllen|Eingabehüllen|Hüllenverletzung|Executor-Hülle|Hülle der Historie|Hülle jedes Ziels|Projekte der Hülle|\d Hüllen\b|^Hüllen$", "closure -> Abschluss"),
    ("receipt", r"Generationsbeleg|Bereitschaftsbeleg|Erzeuger-Beleg|Belegeintrag|\bBeleg(?:e|en|s)?\b|Testbelege|Belegregeln", "receipt -> Quittung, evidence -> Nachweis, claim -> Anspruch"),
    ("pin", r"angeheftet|Anheftung|[Ff]est eingebunden|[Ff]est gehalten", "pin -> gepinnt/Pin"),
    ("rechner", r"Maschinen-Budget|Maschinendatei|Maschinen-Zulassung|Maschinen-Platzbudget|Maschinenstandard|Budget der Maschine|dieser Maschine|Host-Kapazität|Host-Speicher|Host-Zähler|Dem Host fehlen|Host-Kapazitätsfreigabe|ganze Maschine|ganze Host|beim Host werden|Host-Volume", "machine/host -> Rechner"),
    ("retire", r"[Aa]usmuster|ausgemustert", "retire -> stilllegen"),
    ("snapshot", r"Snapshots?\b|-Snapshots?\b", "snapshot -> Schnappschuss"),
    ("lane", r"\bBahn(?:en)?\b|-Bahn\b|bahnen\b|Bahn-", "lane -> Lane"),
    ("slot", r"Sitzungsplätze|Kernplätze|Kapazitätsplatz|Daemon-Plätze|Rechenplätze|Sitzungsplatz|Platzdatei|Platzansprüche|gezählten Platz", "slot -> Slot"),
    ("owner", r"\bBesitzer\b|Besitzern|Nur-Besitzer|Go-Testbesitzer", "owner -> Eigentümer"),
    ("holder", r"Inhaber", "holder -> Halter"),
    ("image", r"Abbild", "image -> Image"),
    ("policy", r"\bPolicy\b|[A-Za-zäöü]-Policy\b", "policy -> Richtlinie"),
    ("policy-compound", r"Richtlinie-[A-ZÄÖÜa-zäöü]", "compound: Richtlinien-"),
    ("governor", r"Kapazitätsregelung|schaltet die Steuerung|Arbeitsprozess", "capacity governor -> Kapazitätswächter; worker -> Worker(-Prozess)"),
    ("dashboard-words", r"Dateioperation|Dateiarbeiter|[Bb]ereite Adresse|Ihre Version", "Dateivorgang, Datei-Worker, Bereitschaftsadresse, du"),
    ("action", r"\bActions?\b|[A-Za-z]-Actions?\b|\bAction-", "action -> Aktion"),
    ("collector", r"(?<!Garbage-)Collector|Einsammler|Müllsammler", "collector -> Sammler"),
    ("worktree", r"Worktree", "worktree -> Arbeitsbaum"),
    ("namespace", r"Namespaces?\b", "namespace -> Namensraum"),
    ("deadlock", r"Deadlock", "deadlock -> Verklemmung"),
    ("compile-unit", r"Kompiliereinheit|Compilereinheit", "compile units -> Compile-Einheiten"),
    ("unit-cache", r"Einheiten-Cache", "unit cache -> Unit-Cache"),
    ("user", r"Benutzer|Nutzerin", "user -> Nutzer"),
    ("claim", r"\bClaims?\b", "claim -> Anspruch"),
    ("quota", r"Quota\b|Quota-", "quota -> Quote"),
    ("id", r"\bIds?\b", "id -> ID"),
    ("dependency", r"Dependency", "dependency -> Abhängigkeit"),
    ("micro-commit", r"Mikro-Commit", "micro-commit"),
    ("staged", r"gestaged|gestagt|\bstagen\b", "staged -> vorgemerkt"),
    ("miss", r"Fehlgriff", "cache miss -> Fehltreffer"),
    ("swap", r"\bSwap\b", "swap -> Auslagerung"),
    ("scope", r"Geltungsbereich", "scope -> Scope"),
    ("gc", r"Garbage Collection", "garbage collection -> Einsammeln"),
    ("disk", r"Plattenplatz|Plattenbudget|auf der Platte|Speicher auf Platte", "disk -> Datenträger"),
    ("incremental", r"Inkrementell-", "incremental"),
    ("ledger", r"Cache-Verzeichnis deklarieren|Bereiche des Verzeichnisses|Größenverzeichnis", "cache ledger -> Cache-Bilanz"),
    ("agents", r"\bAgents\b", "Agent declines weak: Agenten"),
    ("crate", r"Das Crate|dem gesperrten Crate", "Crate is feminine"),
    ("scratch", r"Scratch-Abbild|Reclaim-Stempel|sandboxed", "scratch/reclaim"),
    ("worker-builder", r"Bauers?\b", "builder"),
    ("formal-sie", r"\b[A-ZÄÖÜ][a-zäöüß]+en Sie\b|\bIhnen\b", "du, not Sie"),
    ("lease-gender", r"\b(?:ein|das|dem|jedes|dieses|kein|einem)\s+(?:(?:gehaltene|erwartete|exklusive|andere)\s+(?:und\s+erwartete\s+)?)?(?:[A-Za-zäöü]+-)?Lease\b(?!-)|\bsein Lease\b(?!-)", "Lease is feminine"),
    ("task-gender", r"\b(?:ein|einen|einem|den|dem|des|jeden|jedem|dieses|diesen|diesem)\s+(?:cachebare[rn]?\s+)?(?:[A-Za-zäöü]+-)?Task\b(?!-)", "Task is feminine"),
    ("verb-gender", r"Der Micro-Commit-Verb|\beines Tasks\b", "Verb is neuter; Task is feminine"),
    ("percent", r"[\d\}]%", "n %"),
    ("unit", r"\bMin\.|\bStd\.|(?<=[\d\}]) T\.|\}s\b", "min / h / d / {n} s"),
]
EN_ALIGNED = [
    ("closure", r"\bclosures?\b", r"Hülle|Hüllen\b|Closure"),
    ("receipt", r"\breceipts?\b", r"Beleg"),
]
PARITY_EXEMPT = ("🗣️language/📖️catalog/🧪️tests/",)
TITLE_CAPITAL_VERB = re.compile(r"(?<=[a-zäöüß\-]) (?:Prüfen|Testen|Bauen|Erzeugen|Vorschauen|Installieren|Zurücksetzen|Einrichten|Ausführen|Entfernen|Starten|Auswählen|Anzeigen|Freigeben)$")


def tracked(root):
    r = subprocess.run(["git", "-C", root, "ls-files", "-co", "--exclude-standard", "-z", "--"] + ROOTS, capture_output=True)
    if r.returncode == 0 and r.stdout:
        names = r.stdout.decode().split("\0")
    else:
        names, skip = [], {"node_modules", ".git", "target", "dist", ".cache", "🗑️generated", ".nx"}
        for base in ROOTS:
            for d, dirs, files in os.walk(os.path.join(root, base)):
                dirs[:] = [x for x in dirs if x not in skip]
                names += [os.path.relpath(os.path.join(d, f), root) for f in files]
    return sorted(f for f in names if f.endswith((".json", ".md") + CODE_EXT) and f not in SKIP and os.path.exists(os.path.join(root, f)))


def flat(table, base, into):
    for k, v in table.items():
        if isinstance(v, str): into[f"{base}/{k}"] = v
        elif isinstance(v, dict): flat(v, f"{base}/{k}", into)
        elif isinstance(v, list):
            for i, x in enumerate(v):
                if isinstance(x, str): into[f"{base}/{k}/{i}"] = x


def pairs(doc):
    out = []

    def walk(v, path):
        if isinstance(v, list):
            for i, x in enumerate(v): walk(x, f"{path}/{i}")
        elif isinstance(v, dict):
            if set(v) > {"en", "de"} and isinstance(v["en"], str) and isinstance(v["de"], str): out.append((path, v["en"], v["de"]))
            if set(v) == {"en", "de"}:
                e, d = v["en"], v["de"]
                if isinstance(e, str) and isinstance(d, str): out.append((path, e, d)); return
                if isinstance(e, dict) and isinstance(d, dict):
                    fe, fd = {}, {}
                    flat(e, "", fe); flat(d, "", fd)
                    for k in sorted(set(fe) | set(fd)): out.append((path + k, fe.get(k), fd.get(k)))
                    return
                if isinstance(e, list) and isinstance(d, list):
                    for i in range(max(len(e), len(d))): out.append((f"{path}/{i}", e[i] if i < len(e) else None, d[i] if i < len(d) else None))
                    return
            for k, x in v.items(): walk(x, f"{path}/{k}")

    walk(doc, "")
    return out


def language_leaves(doc):
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


def slash_halves(doc, tokens):
    """(path, english, german) for strings written `English / Deutsch` (split = the ` / ` that maximises English left + German right)."""
    out = []
    en_stop = re.compile(r"\b(?:the|of|and|is|are|to|for|with|not|that|this|from|when|which|must|may|by|an|in|on|it|its|at|as|be)\b")
    de_stop = re.compile(r"\b(?:der|die|das|und|nicht|ist|sind|wird|werden|mit|von|auf|für|fuer|zu|im|bei|oder|ein|eine|einen|dem|den|des|aus|nur|noch|jede[rsnm]?|ohne|über|ueber|wenn|dass|als|durch|nach|vor|bis|auch|kein|keine)\b")

    def walk(v, path):
        if isinstance(v, dict):
            for k, x in v.items(): walk(x, f"{path}/{k}")
        elif isinstance(v, list):
            for i, x in enumerate(v): walk(x, f"{path}/{i}")
        elif isinstance(v, str) and " / " in v and not re.search(r"(?:^|\s)DE: ", v):
            best, score = None, 0
            for m in re.finditer(r" / ", v):
                left, right = v[:m.start()], v[m.end():]
                le, ld, re_, rd = len(en_stop.findall(left)), len(de_stop.findall(left)), len(en_stop.findall(right)), len(de_stop.findall(right))
                strong = le >= 3 and rd >= 3 and le > ld and rd > re_
                short = len(v) < 200 and v.count(" / ") == 1 and ld == 0 and re_ == 0 and german_like(right, tokens) and not german_like(left, tokens)
                value = (le - ld) + (rd - re_) + (1 if short else 0)
                if (strong or short) and value > score: best, score = (left, right), value
            if best: out.append((path, best[0], best[1]))

    walk(doc, "")
    return out


def descriptions(doc):
    out = []

    def walk(v, path):
        if isinstance(v, dict):
            for k, x in v.items(): walk(x, f"{path}/{k}")
        elif isinstance(v, list):
            for i, x in enumerate(v): walk(x, f"{path}/{i}")
        elif isinstance(v, str) and re.search(r"(?:^|\s)DE: ", v) and re.search(r"(?:^|\s)EN: ", v):
            en, de, kind, last = [], [], None, 0
            for m in MARKER.finditer(v):
                (en if kind == "EN" else de if kind == "DE" else []).append(v[last:m.start()]); kind, last = m.group(1), m.end()
            (en if kind == "EN" else de).append(v[last:])
            out.append((path, " ".join(en), " ".join(de)))

    walk(doc, "")
    return out


def fields_regex(text):
    return sorted(m.group(1) for m in PLACEHOLDER.finditer(text))


def fields_oracle(text):
    try: return sorted(f.split(".")[0].split("[")[0] for _l, f, _s, _c in string.Formatter().parse(text) if f)
    except ValueError: return None


def numbers(text):
    t = SPAN.sub(" ", PLACEHOLDER.sub(" ", text))
    return sorted(re.findall(r"(?<![\w.])\d+(?![\w])", t))


TRIGGERS = re.compile(r"(?i)Aufgabe|Closure-|Hülle|Beleg|Abbild|Besitzer|Inhaber|Benutzer|angeheftet|Fehlgriff|Einsammler|Maschine|Plattenbudget|Bahn|Policy-|Sitzungsplatz|Deadlock-|Kompiliereinheit|Mikro-Commit|Quota-|Inkrementell|Std\.|Min\.")


def german_like(text, tokens):
    return bool(UMLAUT.search(text)) or bool(TRIGGERS.search(text)) or len(FUNCTION.findall(text)) >= 2 or any(t in text for t in tokens)


def load_map():
    p = os.path.join(HERE, "translit_map.json")
    return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else {}


NOEN = "\0no-english-side"


def segments(root, only=None, overlay=None):
    """yield (rel, path, en, de) – en is NOEN for German-only segments (markdown lines, source literals)."""
    tmap = load_map(); tokens = [k for k in tmap if len(k) >= 5]
    for rel in tracked(root):
        if only is not None and rel not in only: continue
        text = overlay[rel].decode("utf-8") if overlay and rel in overlay else open(os.path.join(root, rel), encoding="utf-8", errors="replace").read()
        if rel.endswith(".json"):
            try: doc = json.loads(text)
            except Exception: continue
            for p, e, d in pairs(doc): yield rel, p, e, d
            for p, d in language_leaves(doc): yield rel, p + "#language-de", NOEN, d
            for p, e, d in descriptions(doc): yield rel, p + "#desc", e, d
            for p, e, d in slash_halves(doc, tokens): yield rel, p + "#slash", e, d
        elif rel.endswith(".md"):
            lines = text.split("\n"); start = next((i for i, l in enumerate(lines) if HEADING.match(l)), None)
            if start is None: continue
            fenced = False
            for i in range(start, len(lines)):
                if lines[i].lstrip().startswith("```"): fenced = not fenced; continue
                if not fenced: yield rel, f"L{i + 1}", NOEN, SPAN.sub(" ", lines[i])
        else:
            for m in LITERAL.finditer(text):
                s = m.group(0)[1:-1]
                if len(s) >= 4 and german_like(s, tokens): yield rel, f"@{m.start()}", NOEN, s


def check(root, only=None, selftest_segments=None, overlay=None):
    tmap = load_map()
    segs = list(selftest_segments) if selftest_segments is not None else list(segments(root, only, overlay))
    corpus = set()
    for _r, _p, _e, d in segs:
        if d:
            for w in re.findall(r"[A-Za-zÄÖÜäöüß]+", d):
                if UMLAUT.search(w): corpus.add(w.lower())
    findings = []
    forb = [(rid, re.compile(rx), hint) for rid, rx, hint in DE_FORBIDDEN]
    aligned = [(rid, re.compile(e, re.I), re.compile(d)) for rid, e, d in EN_ALIGNED]
    twin = re.compile(r"[A-Za-zÄÖÜäöüß]*(?:(?<![aeqAEQ])[uU]e|[aA]e|[oO]e)[A-Za-zÄÖÜäöüß]*")
    for rel, path, en, de in segs:
        def add(rule, detail): findings.append((rule, rel, path, detail))
        if en is not NOEN and en != NOEN:
            if de is None: add("parity-missing-de", ""); continue
            if en is None: add("parity-missing-en", ""); continue
            exempt = any(x in rel for x in PARITY_EXEMPT)
            if de.strip() == "" and en.strip() != "" and not exempt and "/🧫️fixtures/" not in rel: add("parity-empty-de", "")
            fr, fo = fields_regex(en), fields_regex(de)
            if fr != fo and not exempt: add("parity-fields", f"{fr} != {fo}")
            oe, od = fields_oracle(en), fields_oracle(de)
            if oe is not None and od is not None and oe != od and fr == fo and not exempt: add("parity-oracle-disagreement", f"{oe} vs {od}")
            if sorted(re.sub(r"<[^>]*>", "<>", x) for x in SPAN.findall(en)) != sorted(re.sub(r"<[^>]*>", "<>", x) for x in SPAN.findall(de)) and not exempt: add("parity-code-spans", f"{SPAN.findall(en)} != {SPAN.findall(de)}")
            if en == de and len(ENGLISH_WORDS.findall(en.lower())) >= 3 and not UMLAUT.search(de): add("parity-untranslated", en[:80])
            for rid, ere, bad in aligned:
                if ere.search(en) and bad.search(de): add("glossary-" + rid, de[:100])
        if not de: continue
        text = SPAN.sub(" ", de)
        for rid, rx, hint in (forb if rel not in TRANSLIT_ONLY else []):
            if rid in ("task-gender",): pass
            m = rx.search(text)
            if m: add("glossary-" + rid, f"{hint}: …{text[max(0, m.start() - 25):m.end() + 25]}…".replace("\n", " "))
        if rel not in TRANSLIT_ONLY and not any(x in rel for x in TASK_EXEMPT) and re.search(r"[Aa]ufgabe", text) and en is not None and re.search(r"\btasks?\b", en, re.I): add("glossary-task", text[:100])
        for tok in re.findall(r"[A-Za-zÄÖÜäöüß]+(?:-[A-Za-zÄÖÜäöüß]+)*", text):
            if tok in tmap or any(p in tmap for p in tok.split("-")): add("translit", tok); continue
            if twin.fullmatch(tok) and len(tok) >= 4:
                conv = re.sub(r"(?<![aeqAEQ])ue", "ü", tok); conv = re.sub(r"(?<![aeqAEQ])Ue", "Ü", conv)
                conv = conv.replace("ae", "ä").replace("Ae", "Ä").replace("oe", "ö").replace("Oe", "Ö")
                if conv != tok and conv.lower() in corpus: add("translit", f"{tok} -> {conv}")
        if STEMS.search(text): add("translit", STEMS.search(text).group(0).strip())
        if path.endswith("/title") and TITLE_CAPITAL_VERB.search(de): add("grammar-title-verb", de)
    return findings


def selftest():
    bad = [
        ("a.json", "/x", "{n} files", "{n} Dateien in {m}"), ("a.json", "/x", "run `repo a`", "führe `repo b` aus"),
        ("a.json", "/x", "one", ""), ("a.json", "/x", "x", None),
        ("a.json", "/x", "the run of the tool is not ready for this", "the run of the tool is not ready for this"),
        ("a.json", "/x", "closures", "Hüllen"), ("a.json", "/x", "x", "Das ist fuer dich über Maschinen-Budget"),
        ("a.json", "/x", "x", "Hängen Sie es ab"), ("a.json", "/p/title", "x", "Cache Zurücksetzen"),
        ("a.json", "/x", "x", "wartet auf ein Lease"), ("a.json", "/x", "x", "Sie bestimmt"), ("a.json", "/x", "x", "13% und 3 Std."),
        ("a.json", "/x", "x", "Laeufe und Läufe"),
    ]
    good = [("a.json", "/x", "{n} files in {m}", "{n} Dateien in {m}"), ("a.json", "/x", "x", "Sie bestimmt, wie lange Bytes leben"), ("a.json", "/p/title", "x", "Cache zurücksetzen"), ("a.json", "/x", "13% in 3 h, first the new source", "13 % in 3 h, zuerst die neue Quelle")]
    hits = {r for r, *_x in check("", selftest_segments=bad)}
    expected = {"parity-fields", "parity-code-spans", "parity-empty-de", "parity-missing-de", "parity-untranslated", "glossary-closure", "translit", "glossary-rechner", "glossary-formal-sie", "grammar-title-verb", "glossary-lease-gender", "glossary-percent", "glossary-unit"}
    missing = expected - hits
    clean = check("", selftest_segments=good)
    if missing or clean:
        print("SELFTEST FAIL missing:", sorted(missing), "false positives:", clean); sys.exit(1)
    print("selftest ok:", len(hits), "rules fire on the red fixtures, 0 on the green ones")


def main():
    if "--selftest" in sys.argv: return selftest()
    root = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else os.getcwd()
    only = None
    if "--files-from" in sys.argv: only = set(json.load(open(sys.argv[sys.argv.index("--files-from") + 1], encoding="utf-8")))
    limit = int(sys.argv[sys.argv.index("--limit") + 1]) if "--limit" in sys.argv else 3
    findings = check(root, only)
    by = {}
    for rule, rel, path, detail in findings: by.setdefault(rule, []).append((rel, path, detail))
    for rule in sorted(by):
        print(f"{rule}: {len(by[rule])} in {len({r for r, _p, _d in by[rule]})} files")
        for rel, path, detail in by[rule][:limit]: print(f"    {rel.split('/', 3)[-1][-80:]} {path[-50:]} {detail[:150]}")
    if "--json" in sys.argv: json.dump([list(f) for f in findings], open(sys.argv[sys.argv.index("--json") + 1], "w", encoding="utf-8"), ensure_ascii=False)
    print(f"LAW germanqa: {len(findings)} findings" + (" GREEN" if not findings else " RED"))
    sys.exit(1 if findings else 0)


if __name__ == "__main__":
    main()
