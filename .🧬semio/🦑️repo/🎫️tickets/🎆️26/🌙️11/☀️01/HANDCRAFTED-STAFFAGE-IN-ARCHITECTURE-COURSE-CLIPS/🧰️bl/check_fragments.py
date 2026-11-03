"""🧩 Find truncated or fragmentary en/de strings and leading-space or missing-subject texts in referenced staffage assets."""
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import audit_bilingual as a

END_EN = re.compile(r"(?i)\b(in|into|of|the|and|with|a|an|to|from)\.?$")
END_DE = re.compile(r"(?i)\b(in|der|die|das|dem|den|des|dess|und|mit|von|zu|zum|zur|ein|eine|einer)\.?$")


def main():
    manifest = a.load(a.MANIFEST)
    index = {}
    for f in a.CATALOG.rglob("🔣️.json"):
        d = a.load(f)
        if isinstance(d.get("asset"), dict) and "id" in d["asset"]:
            index[d["asset"]["id"]] = f
    ids = sorted({c["asset"] for c in manifest["casts"].values()})
    hits = []
    for aid in ids:
        f = index[aid]
        sink = []
        a.walk(a.load(f), "", str(f.relative_to(a.CATALOG)), sink, [])
        for path, en, de in sink:
            if not isinstance(en, str) or not isinstance(de, str):
                continue
            for lang, s in (("en", en), ("de", de)):
                t = s.strip()
                if not t:
                    continue
                if s != s.strip() or s[:1].islower() or s[:1] == " ":
                    hits.append((aid, path, f"{lang}: leading space or lowercase start", en, de))
                if (lang == "en" and END_EN.search(t)) or (lang == "de" and END_DE.search(t)):
                    hits.append((aid, path, f"{lang}: sentence ends with function word (truncated?)", en, de))
    for h in hits:
        print("\t".join(h))
    print(len(hits), "hits")


if __name__ == "__main__":
    sys.exit(main())
