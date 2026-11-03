"""🌍 Audit en/de human-facing text of every staffage asset referenced by the manifest casts (read-only)."""
import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

REPO = Path(r"C:\git\semio")
CATALOG = REPO / "🎓️teaching" / "🏛️architecture" / "🧍️staffage" / "🗂️catalog"
MANIFEST = REPO / "🎓️teaching" / "🏛️architecture" / "⚡️energy" / "🏗️authoring" / "🎨️visuals" / "🧍️staffage" / "🔣️.json"
TICKET = REPO / ".🧬semio" / "🦑️repo" / "🎫️tickets" / "🎆️26" / "🌙️11" / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
OUT = TICKET / "🗑️generated" / "bl"

PLAIN_TEXT_KEYS = {"title", "summary", "description", "label", "name", "reason", "notes", "limitingFeature"}
EN_STOP = {"the", "and", "with", "from", "this", "that", "which", "are", "is", "of", "for", "its", "their", "into", "while", "across", "between", "shows", "about", "than", "has", "have", "under", "during", "some", "each"}
DE_STOP = {"der", "die", "das", "und", "mit", "eine", "einer", "einem", "ist", "von", "zum", "zur", "im", "auf", "nicht", "sich", "wird", "werden", "oder", "für", "über", "bei", "sind", "den", "dem", "des", "ein"}
EN_LOAN = {
    "standing", "sitting", "kneeling", "walking", "running", "jogging", "cycling", "idle", "breathing", "typing",
    "pose", "poses", "clip", "clips", "loop", "loops", "frame", "frames", "relaxed", "pointing", "shivering", "fanning",
    "cooling", "heating", "dimmed", "parked", "tilted", "stroller", "cane", "crutches", "rollator", "toddler",
    "umbrella", "commuter", "scooter", "cargo", "phone", "worker", "carrying", "mirror", "mirrored", "full", "load",
    "power", "switch", "warm", "warm-up", "hands", "arms", "crossed", "leaning", "seated", "reader", "truck", "guide",
    "tour", "box", "walk", "run", "cycle", "idle", "still", "rest", "default", "scene", "shot", "ground", "anchor",
    "support", "phase", "gust", "breeze", "sway", "emitter", "transpiration", "hatch", "hatching", "pattern", "lines",
    "overlay", "overlays", "silhouette", "styles", "merged", "omitted", "retained", "presentation", "viewpoint",
    "viewpoints", "thermal", "plan", "elevation", "section", "rated", "open", "closed", "on", "off", "up", "down",
    "view", "scale", "fresh", "turgid", "wind", "rain", "snow", "dark", "line", "lines", "cog", "drum", "casing",
    "grille", "plate", "region", "regions", "body", "kind", "role", "status", "authored", "handcrafted", "limiting",
    "feature", "alternative", "boiling", "transition", "hob", "server", "rack", "desk", "monitor", "laptop", "lamp",
    "chair", "boiler", "cylinder", "meter", "thermostat", "radiator", "valve", "pump", "fan", "heater", "kettle",
    "toaster", "vacuum", "cleaner", "humidifier", "diffuser", "unit", "outdoor", "indoor", "pathlines", "stroke",
    "bulb", "hood", "extractor", "shower", "head", "compressor", "scroll", "wheel", "loader", "lift", "crane", "mixer",
    "generator", "excavator", "forklift", "pumpjack", "exchanger", "heat", "plate", "bike", "car", "vintage",
    "blade", "prosthesis", "wheelchair", "elderly", "child", "kick", "toddler", "pregnant", "gardener", "skateboarder",
}
ASCII_UMLAUT = re.compile(r"(?i)[a-zäöüß]*(?:ae|oe|ue)[a-zäöüß]*")
SS_TOKEN = re.compile(r"(?i)[a-zäöüß]*ss[a-zäöüß]*")
TOKEN = re.compile(r"[A-Za-zÄÖÜäöüß][A-Za-zÄÖÜäöüß\-]*")


def tokens(text):
    return [t.lower() for t in TOKEN.findall(text)]


def check_pair(en, de, findings, ctx):
    if not isinstance(en, str) or not en.strip():
        findings.append((*ctx, "en empty or missing", repr(en), repr(de)))
    if not isinstance(de, str) or not de.strip():
        findings.append((*ctx, "de empty or missing", repr(en), repr(de)))
    if not (isinstance(en, str) and isinstance(de, str)):
        return
    if not en.strip() or not de.strip():
        return
    if en.strip() == de.strip():
        findings.append((*ctx, "de identical to en", en, de))
    de_tokens = set(tokens(de))
    en_tokens = set(tokens(en))
    for w in sorted(de_tokens & EN_STOP):
        findings.append((*ctx, f"English stopword in de: {w}", en, de))
    for w in sorted(en_tokens & DE_STOP):
        findings.append((*ctx, f"German stopword in en: {w}", en, de))
    for m in sorted(set(ASCII_UMLAUT.findall(de))):
        findings.append((*ctx, f"ASCII umlaut substitute in de: {m}", en, de))
    for m in sorted(set(SS_TOKEN.findall(de))):
        findings.append((*ctx, f"ss token in de (check for eszett): {m}", en, de))
    for w in sorted(de_tokens & EN_LOAN):
        findings.append((*ctx, f"English-looking word in de: {w}", en, de))
    for w in sorted(de_tokens & en_tokens):
        if len(w) > 3 and w not in DE_STOP and w not in EN_STOP and w in EN_LOAN:
            findings.append((*ctx, f"English word copied verbatim into de: {w}", en, de))
    ratio = len(de) / max(1, len(en))
    if ratio < 0.4 or ratio > 2.5:
        findings.append((*ctx, f"length ratio de/en {ratio:.2f} (possible truncation)", en, de))


def walk(node, path, owner, sink, findings):
    if isinstance(node, dict):
        keys = set(node)
        if keys and keys <= {"en", "de"}:
            sink.append((path, node.get("en"), node.get("de")))
            if keys != {"en", "de"}:
                findings.append((owner, path, "localized object missing en or de", repr(node.get("en")), repr(node.get("de"))))
            return
        if {"en", "de"} <= keys:
            findings.append((owner, path, f"mixed localized object with extra keys {sorted(keys - {'en', 'de'})}", "", ""))
        for k, v in node.items():
            if k in PLAIN_TEXT_KEYS and isinstance(v, str):
                findings.append((owner, f"{path}.{k}", "human-facing text not localized (plain string)", v[:80], ""))
            walk(v, f"{path}.{k}", owner, sink, findings)
    elif isinstance(node, list):
        for i, v in enumerate(node):
            walk(v, f"{path}[{i}]", owner, sink, findings)


def load(path):
    with open(path, encoding="utf-8") as fh:
        return json.load(fh)


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    manifest = load(MANIFEST)
    findings = []
    walk(manifest, "manifest", "manifest", [], findings)

    index = {}
    dupes = []
    for f in CATALOG.rglob("🔣️.json"):
        doc = load(f)
        aid = doc.get("asset", {}).get("id") if isinstance(doc.get("asset"), dict) else None
        if aid is None:
            continue
        if aid in index:
            dupes.append((aid, str(index[aid].relative_to(CATALOG)), str(f.relative_to(CATALOG))))
        index[aid] = f

    wanted = []
    seen = set()
    for cast_id, cast in manifest.get("casts", {}).items():
        wanted.append(cast["asset"])
    queue = list(dict.fromkeys(wanted))
    referenced = set()
    while queue:
        aid = queue.pop(0)
        if aid in referenced:
            continue
        referenced.add(aid)
        if aid not in index:
            findings.append(("manifest", f"casts -> {aid}", "asset id not found in catalog", "", ""))
            continue
        doc = load(index[aid])
        for rep in doc.get("asset", {}).get("representations", {}).values():
            for alt in rep.get("alternatives", []) if isinstance(rep, dict) else []:
                if isinstance(alt, dict) and "asset" in alt:
                    queue.append(alt["asset"])

    for aid in sorted(referenced):
        if aid not in index:
            continue
        f = index[aid]
        owner = str(f.relative_to(CATALOG))
        sink = []
        doc = load(f)
        walk(doc, "", owner, sink, findings)
        for path, en, de in sink:
            check_pair(en, de, findings, (owner, path))

    lines = ["file\tpath\tissue\ten\tde"]
    for row in findings:
        lines.append("\t".join(str(c).replace("\t", " ").replace("\n", " ") for c in row))
    (OUT / "findings.tsv").write_text("\n".join(lines) + "\n", encoding="utf-8")
    cats = Counter(re.sub(r":.*", "", r[2]) for r in findings)
    print(f"referenced assets: {len(referenced)}; catalog duplicates: {len(dupes)}; findings: {len(findings)}")
    for k, v in cats.most_common():
        print(v, k)
    for d in dupes:
        print("DUPLICATE", d)


if __name__ == "__main__":
    sys.exit(main())
