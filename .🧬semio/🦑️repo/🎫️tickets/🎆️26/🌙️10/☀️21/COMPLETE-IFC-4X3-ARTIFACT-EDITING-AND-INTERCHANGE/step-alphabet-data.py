"""🔤 Pin the official ISO 8859 mappings behind the STEP alphabet contract."""
import hashlib
import json
from pathlib import Path
import urllib.request

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
DATA = ROOT / "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🔤️lexer-and-parse/alphabets"
GENERATED = TICKET / "🗑️generated/step-alphabets"


def main():
    """📚 Retain exact source hashes and compiled scalar tables."""
    GENERATED.mkdir(parents=True, exist_ok=True)
    DATA.mkdir(parents=True, exist_ok=True)
    existing = json.loads((DATA / "data.json").read_text(encoding="utf-8")) if (DATA / "data.json").is_file() else None
    pages = []
    for index in range(1, 10):
        url = f"https://www.unicode.org/Public/MAPPINGS/ISO8859/8859-{index}.TXT"
        raw = urllib.request.urlopen(url).read()
        if existing is not None:
            assert hashlib.sha256(raw).hexdigest() == existing["pages"][index - 1]["sha256"], url
        (GENERATED / f"8859-{index}.TXT").write_bytes(raw)
        mappings = {}
        for line in raw.decode("utf-8").splitlines():
            cells = line.split("#", 1)[0].split()
            if len(cells) == 2:
                mappings[int(cells[0], 16)] = int(cells[1], 16)
        pages.append({"selector": chr(64 + index), "encoding": f"ISO-8859-{index}", "url": url, "sha256": hashlib.sha256(raw).hexdigest(), "scalars": [mappings.get(code) for code in range(160, 255)]})
    document = {"schema": "stdio.part21.alphabets", "version": 1, "shiftedStart": 160, "shiftedEnd": 254, "pages": pages}
    (DATA / "data.json").write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")
    cases = [{"name": f"{page['selector']}-{code + 160:02X}", "source": "\\P" + page["selector"] + "\\\\S\\" + chr(code + 32), "expected": None if scalar is None else chr(scalar)} for page in pages for code, scalar in enumerate(page["scalars"])]
    (DATA / "cases.json").write_text(json.dumps({"schema": "stdio.part21.alphabet-cases", "cases": cases}, indent=2) + "\n", encoding="utf-8")
    rows = ["    [" + ",".join("0" if value is None else f"0x{value:X}" for value in page["scalars"]) + "]," for page in pages]
    (DATA / "data.rs").write_text("//! 🗺️ Pinned ISO 8859 scalar tables; see data.json and LICENSE.txt.\n\npub(super) const ALPHABETS: [[u32; 95]; 9] = [\n" + "\n".join(rows) + "\n];\n", encoding="utf-8")
    license_bytes = urllib.request.urlopen("https://www.unicode.org/license.txt").read()
    assert hashlib.sha256(license_bytes).hexdigest() == "e7a93b009565cfce55919a381437ac4db883e9da2126fa28b91d12732bc53d96"
    (DATA / "LICENSE.txt").write_bytes(license_bytes)
    print("[DEBUG] " + json.dumps({"pages": 9, "scalars": 855, "data": str(DATA), "licenseSha256": hashlib.sha256(license_bytes).hexdigest()}))


if __name__ == "__main__":
    main()
