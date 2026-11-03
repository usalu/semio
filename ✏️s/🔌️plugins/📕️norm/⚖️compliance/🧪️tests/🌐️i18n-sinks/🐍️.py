"""🌐 No user-visible sink of the Eurocode artifacts and shared norm crates is English only.

`python -I 🐍️.py` reads the Rust and JSON sources as text, independent of the Rust kernel, and exits non-zero listing every violation of:
1. spec-trace JSON: every `description` and every not-modelled `reason` carries a non-empty `en` and `de`, and the generated twin uses `SpecText`.
2. gate reasons: every literal `gate(.., "reason")` has a coded rule (EN pattern with German template) in the shared reason table.
3. outcome steps: an English prose literal handed to an outcome constructor or `.steps(..)` is followed by `.steps_de(..)`.
4. mechanics: `MechanicsError::Invalid` is built only through the bilingual `invalid(en, de)`.
5. no default language: no enum named `Lang`, `Locale` or `Language` has a `#[default]`, and no `if locale == Locale::De { De } else { En }` fallback exists.
6. clause labels and NDP locators: `ClauseId::new(..)` and `clause(..)` literals, and `.at(..)` locators naming an annex or table, hold only designators (`Annex`, `Table`, standard codes, numbers) rendered per locale, and no English prose.
"""
import json, pathlib, re, sys

NORM = pathlib.Path(__file__).resolve().parents[3]
ART = NORM / "🗿️artifacts"
SKIP = re.compile(r"🧪️|/tests/|🧫️|🔮️|🥒️|/target/|vdi3805|iso16757|din4108|din16798|din18599")
STOP = set("the and of with for from is are not must should be this that to or by no all per at as in on if when then than within against over under after before".split())
VOCAB = {"annex", "table", "tables", "figure", "eq", "expression", "design", "case", "section", "part", "na", "de", "en", "din", "iso", "see", "and", "or", "to", "nds", "ndp"}
STR = r'"((?:[^"\\\n]|\\.)*)"'


def rust_files(root=NORM):
    for p in root.rglob("*.rs"):
        rel = str(p.relative_to(NORM))
        if not SKIP.search(rel + "/"):
            yield p, rel


def words(text):
    text = re.sub(r"\{[^}]*\}", " ", text)
    return [w.lower() for w in re.findall(r"[A-Za-z]{3,}", text)]


def prose(text):
    found = words(text)
    return any(w in STOP for w in found) and len(found) >= 2 or len([w for w in found if len(w) >= 5]) >= 2


def spec_trace(bad):
    for p in sorted(ART.glob("*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧭️spec-trace/🔣️.json")):
        doc = json.loads(p.read_text(encoding="utf-8"))
        for it in doc["items"]:
            texts = [("description", it["description"])] + ([("reason", it["realisation"]["reason"])] if it["realisation"]["kind"] == "not-modelled" else [])
            for name, t in texts:
                if not isinstance(t, dict) or not t.get("en", "").strip() or not t.get("de", "").strip():
                    bad.append(f"spec-trace {doc['code']} {it['id']}: {name} lacks en or de")
                elif t["en"] == t["de"] and prose(t["en"]):
                    bad.append(f"spec-trace {doc['code']} {it['id']}: {name} is not translated")
        twin = p.with_name("🦀️.rs").read_text(encoding="utf-8")
        if re.search(r'Realisation::[A-Za-z]+\([^\n]*\), "[^"]*"\),\n', twin) or "SpecText::new" not in twin:
            bad.append(f"spec-trace {doc['code']}: the Rust twin does not use SpecText")


def rule_patterns():
    text = (NORM / "⚖️compliance/🗣️reasons/🦀️.rs").read_text(encoding="utf-8")
    out = []
    for en in re.findall(r'rule\("[^"]+",\s*' + STR, text):
        out.append(re.compile("^" + ".+?".join(re.escape(part) for part in en.split("{}")) + "$"))
    return out


def gates(bad):
    rules = rule_patterns()
    for p, rel in rust_files(ART):
        text = p.read_text(encoding="utf-8")
        for m in re.finditer(r"\bgate\(((?:[^()]|\([^()]*\))*?),\s*" + STR + r"\)\?", text):
            reason = m.group(2).strip().rstrip(".")
            if not any(r.match(reason) for r in rules):
                bad.append(f"{rel}: gate reason without a coded rule: {reason}")


def steps(bad):
    sink = re.compile(r"Outcome::|\binfo\(|limit_or_info\(|::num\(|::nmm\(|::n\(|::force\(")
    for p, rel in rust_files(ART):
        text = p.read_text(encoding="utf-8")
        if "fn steps_de" not in text and ".steps(" not in text and "steps_de" not in text:
            continue
        for m in re.finditer(r'\.steps\((?:format!\()?' + STR, text):
            call_end = text.find("\n", m.end())
            tail = text[m.end():m.end() + 400]
            if prose(m.group(1)) and ".steps_de(" not in text[m.end():m.end() + 600].split("\n")[0] + tail[:300].split(".steps(")[0]:
                bad.append(f"{rel}: English steps prose without steps_de: {m.group(1)[:60]}")
        for m in re.finditer(STR + r"\.into\(\)\)(\.steps_de\()?", text):
            line = text[text.rfind("\n", 0, m.start()) + 1:m.end()]
            if prose(m.group(1)) and sink.search(line) and not m.group(2) and "Err(" not in line and "fn " not in line and "reason" not in line:
                bad.append(f"{rel}: English outcome steps without steps_de: {m.group(1)[:60]}")


def mechanics(bad):
    for p, rel in rust_files(NORM / "🧮️mechanics"):
        text = p.read_text(encoding="utf-8")
        for m in re.finditer(r'MechanicsError::Invalid\(\s*(?:format!\(|")', text):
            bad.append(f"{rel}: MechanicsError::Invalid built from English text only")
        for m in re.finditer(r'\binvalid\(\s*(?:format!\(' + STR + r'(?:,[^()]*)?\)|' + STR + r')\s*\)', text):
            if "fn invalid" not in text[max(0, m.start() - 30):m.start()]:
                bad.append(f"{rel}: invalid(..) with a single language: {(m.group(1) or m.group(2))[:60]}")


def no_default_language(bad):
    for p, rel in rust_files(NORM):
        text = p.read_text(encoding="utf-8")
        for m in re.finditer(r"enum (Lang|Locale|Language)\s*\{([^}]*)\}", text):
            if "#[default]" in m.group(2):
                bad.append(f"{rel}: enum {m.group(1)} has a default variant")
        if re.search(r"==\s*(?:crate::)?(?:\w+::)*Locale::De\s*\{[^{}]*\bLang::De\b[^{}]*\}\s*else\s*\{[^{}]*\bLang::En\b", text):
            bad.append(f"{rel}: Lang chosen from the locale with an English fallback, use Lang::of")


def designators(bad):
    sink = [re.compile(r"ClauseId::new\(((?:[^()]|\([^()]*\))*)\)"), re.compile(r"\bclause\(((?:[^()]|\([^()]*\))*)\)")]
    locator = re.compile(r"\.at\(" + STR + r"\)")

    def stray(lit):
        return [w for w in words(lit) if w not in VOCAB and w.islower() and len(w) >= 4]

    for p, rel in rust_files(NORM):
        text = p.read_text(encoding="utf-8")
        for rx in sink:
            for m in rx.finditer(text):
                for lit in re.findall(STR, m.group(1)):
                    if "{" not in lit and (len(stray(lit)) >= 2 or (stray(lit) and any(w in STOP for w in words(lit)))):
                        bad.append(f"{rel}: clause label holds English prose: {lit[:70]}")
        for m in locator.finditer(text):
            lit = m.group(1)
            if re.match(r"(Annex|Table)\b", lit) and stray(lit):
                bad.append(f"{rel}: NDP locator holds English prose: {lit[:70]}")


def main():
    bad = []
    for check in (spec_trace, gates, steps, mechanics, no_default_language, designators):
        check(bad)
    for line in sorted(set(bad)):
        print(line)
    if not bad:
        print("ok: no English-only user-visible sink")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
