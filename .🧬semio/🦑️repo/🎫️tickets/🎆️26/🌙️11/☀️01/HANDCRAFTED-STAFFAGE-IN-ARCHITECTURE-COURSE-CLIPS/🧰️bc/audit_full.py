"""🔬 Audit en/de text of every catalog asset: pairs, fragments, German agreement, sibling label consistency (read-only)."""
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "🧰️bl"))
import audit_bilingual as a

OUT = a.TICKET / "🗑️generated" / "bc"
END_EN = re.compile(r"(?i)\b(in|into|of|the|and|with|a|an|to|from|or|for|by|on|at)\.?$")
END_DE = re.compile(r"(?i)\b(in|der|die|das|dem|den|des|dess|und|mit|von|zu|zum|zur|ein|eine|einer|oder|für)\.?$")
SUBJECTLESS = re.compile(r"^\s*(is|are|ist|sind|has|hat|replaces|ersetzt)\b|;\s*(is|ist)\s", re.I)
AGREE = [
    (re.compile(r"\b(Hinterer|Vorderer|Linker|Rechter|Oberer|Unterer|Äußerer|Innerer)\s+(Rolle|Räder|Rollen|Seite|Hand|Achse|Tür|Klappe|Platte|Lamelle|Schiene|Kante)\b"), "masc. adjective before fem./plural noun"),
    (re.compile(r"\b(Hintere|Vordere|Linke|Rechte|Obere|Untere|Äußere|Innere)\s+(Arm|Fuß|Bein|Flügel|Griff|Rahmen|Rand|Rad|Zylinder|Deckel|Teil|Stuhl|Tisch|Kopf|Körper|Ast|Stamm)\b"), "fem. adjective before masc./neut. noun"),
    (re.compile(r"\b(Hinteres|Vorderes|Linkes|Rechtes|Oberes|Unteres)\s+(Rolle|Arm|Bein)\b"), "neut. adjective mismatch"),
    (re.compile(r"\b(ein|der|die|das)\s+(ein|der|die|das)\b", re.I), "doubled article"),
    (re.compile(r"\b(\w{3,})\s+\1\b", re.I), "repeated word"),
    (re.compile(r"\s{2,}|\s[,.;:]|,,|\.\.(?!\.)"), "spacing or punctuation"),
]
EN_AGREE = [(re.compile(r"\b(\w{3,})\s+\1\b", re.I), "repeated word"), (re.compile(r"\s{2,}|\s[,.;:]|,,"), "spacing or punctuation"), (re.compile(r"\b(a|an|the)\s+(a|an|the)\b", re.I), "doubled article")]
LABEL_PATH = re.compile(r"(\.title|\.label|\.name)$")
KIND_PATH = re.compile(r"\.(bones|materials|anchors|contacts|regions|phases|parts|elements)\[\d+\]\.title$")


def text_checks(owner, path, en, de, out):
    for lang, s, rules in (("en", en, EN_AGREE), ("de", de, AGREE)):
        if not isinstance(s, str) or not s.strip():
            continue
        if s != s.strip():
            out.append((owner, path, f"{lang}: leading or trailing space", en, de))
        if s.strip()[:1].islower() and not LABEL_PATH.search(path):
            out.append((owner, path, f"{lang}: lowercase start", en, de))
        if (lang == "en" and END_EN.search(s.strip())) or (lang == "de" and END_DE.search(s.strip())):
            out.append((owner, path, f"{lang}: ends with function word (truncated?)", en, de))
        if SUBJECTLESS.search(s):
            out.append((owner, path, f"{lang}: subject-less fragment", en, de))
        if len(s) > 60 and s.rstrip()[-1] not in ".!?)":
            out.append((owner, path, f"{lang}: long text without final punctuation", en, de))
        if re.search(r"[\.;,]\s*[\.;,]", s):
            out.append((owner, path, f"{lang}: doubled punctuation", en, de))
        if s.count("(") != s.count(")"):
            out.append((owner, path, f"{lang}: unbalanced parentheses", en, de))
        for rx, why in rules:
            for m in rx.finditer(s):
                out.append((owner, path, f"{lang}: {why}: {m.group(0)}", en, de))
    if isinstance(en, str) and isinstance(de, str):
        for ch in "äöüÄÖÜß":
            if ch in en and ch not in de and len(en) > 0:
                out.append((owner, path, f"umlaut in en, none in de: {ch}", en, de))
                break


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    findings = []
    labels = defaultdict(lambda: defaultdict(set))
    files = sorted(a.CATALOG.rglob("🔣️.json"))
    count = 0
    for f in files:
        doc = a.load(f)
        if not isinstance(doc.get("asset"), dict):
            continue
        count += 1
        owner = str(f.relative_to(a.CATALOG))
        kind = f.parent.parent.name
        sink = []
        a.walk(doc, "", owner, sink, findings)
        for path, en, de in sink:
            a.check_pair(en, de, findings, (owner, path))
            text_checks(owner, path, en, de, findings)
            if isinstance(en, str) and isinstance(de, str) and KIND_PATH.search(path):
                labels[(kind, en.strip().lower())][de.strip()].add(owner)
    for (kind, en), des in sorted(labels.items()):
        if len(des) > 1:
            variants = "; ".join(f"{d} ({len(o)})" for d, o in sorted(des.items()))
            findings.append((kind, en, "inconsistent sibling label wording across assets", en, variants))
    lines = ["file\tpath\tissue\ten\tde"]
    for row in findings:
        lines.append("\t".join(str(c).replace("\t", " ").replace("\n", " ") for c in row))
    (OUT / "findings.tsv").write_text("\n".join(lines) + "\n", encoding="utf-8")
    cats = Counter(re.sub(r":.*", "", r[2]) for r in findings)
    print(f"assets: {count}; findings: {len(findings)}")
    for k, v in cats.most_common():
        print(v, k)


if __name__ == "__main__":
    sys.exit(main())
