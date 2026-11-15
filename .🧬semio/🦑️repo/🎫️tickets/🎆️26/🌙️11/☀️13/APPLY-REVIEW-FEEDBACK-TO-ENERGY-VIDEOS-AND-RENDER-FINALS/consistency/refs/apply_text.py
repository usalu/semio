import ast
import importlib
import json
import re
import sys
from pathlib import Path

ENERGY = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
sys.path.insert(0, r"C:\git\semio")
R = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
CASES = json.loads((ENERGY / "🏗️authoring" / "📚️references" / "🧪️tests" / "🔣️.cases.json").read_text(encoding="utf-8"))
SCAN = CASES["scan"]
DESIGNATION = re.compile(SCAN["designationPattern"])
KEY = re.compile(SCAN["narrationKeyPattern"])
EXCLUDED = set(SCAN["excludedDirs"])

EXACT = {
    "VDI 4650-1:2024-02": "vdi-4650-1",
    "VDI 4650 Blatt 1:2024-02": "vdi-4650-1",
    "DIN EN ISO 13789:2017-10": "din-en-iso-13789",
    "DIN 4108 Bbl. 2:2019-06": "din-4108-beiblatt-2",
    "DIN 4108 Bbl. 2": "din-4108-beiblatt-2",
}
SHORT = {
    "DIN V 18599": "din-ts-18599", "DIN V 18599-1": "din-ts-18599-1", "DIN V 18599-2": "din-ts-18599-2",
    "DIN V 18599-5": "din-ts-18599-5", "DIN V 18599-10": "din-ts-18599-10", "DIN EN 12831": "din-en-12831-1",
    "EN 16798-1": "din-en-16798-1", "EN 673": "din-en-673", "VDI 4650": "vdi-4650-1",
    "DIN V 4108-6": "din-v-4108-6", "VDI 2052": "vdi-2052-1", "VDI 6022": "vdi-6022-1", "DIN EN 14511": "din-en-14511-2",
}
for reference_id, item in R.registry().items():
    if item["status"] == "current" and item["kind"] in ("standard", "guideline"):
        SHORT.setdefault(item["designation"], reference_id)
OLD_V = re.compile(r"^DIN V 18599(-\d+)?(:2016-10|:2018-09)?$")


def narration_lines(tree):
    lines = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Tuple) and len(node.elts) == 3 and all(isinstance(e, ast.Constant) and isinstance(e.value, str) for e in node.elts) and KEY.match(node.elts[0].value):
            for element in node.elts[1:]:
                lines.update(range(element.lineno, element.end_lineno + 1))
    return lines


def docstring_lines(tree):
    lines = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant) and isinstance(node.value.value, str):
            lines.update(range(node.lineno, node.end_lineno + 1))
    return lines


def canonical(found: str, narration: bool, law: bool) -> str:
    if found in EXACT:
        return R.short(EXACT[found]) if narration else R.designation(EXACT[found])
    base = found.split(":")[0]
    if OLD_V.match(found) and not law:
        reference_id = SHORT[base]
        return R.short(reference_id) if narration else R.designation(reference_id)
    if found in SHORT and ":" not in found:
        reference_id = SHORT[found]
        return R.short(reference_id) if narration else R.designation(reference_id)
    return found


CLEANUPS = [
    (re.compile(r"(DIN V 4108-6:2003-06),? \d\.\d(?:\.\d)?(?: · \d\.\d(?:\.\d)?)*"), r"\1"),
    (re.compile(r"(DIN V 4108-6:2003-06),? (?:Anhang|Anh\.) [A-D](?:\.\d)?"), r"\1"),
    (re.compile(r" \((?:Annex|Anhang) D\.2\)"), ""),
    (re.compile(r"(?:IWU )?(?:Deutsche )?Wohngebäudetypologie(?: 2015| \(2015\))?"), "IWU Wohngebäudetypologie (2015)"),
    (re.compile(r"(?:IWU )?(?:German )?[Rr]esidential [Bb]uilding [Tt]ypology(?: 2015| \(2015\))?"), "IWU residential building typology (2015)"),
]
IWU_TABLE = re.compile(r"IWU 2015(?:, Tab(?:le|elle) \d+)?")


def rewrite_line(line: str, narration: bool) -> str:
    law = "GModG" in line or "GEG" in line

    def replace(match):
        return canonical(match.group(0), narration, law)

    line = DESIGNATION.sub(replace, line)
    for pattern, replacement in CLEANUPS:
        line = pattern.sub(replacement, line)
    line = IWU_TABLE.sub("IWU residential building typology (2015)" if '"en":' in line else "IWU Wohngebäudetypologie (2015)", line)
    return line


def rename_law(line: str) -> str:
    if "GModG" in line and "GEG" in line:
        return line
    return re.sub(r"(?<![A-Za-z_0-9])GEG(?![A-Za-z_0-9])", "GModG", line)


def process(path: Path, rename: bool) -> int:
    text = path.read_bytes().decode("utf-8")
    lines = text.splitlines(keepends=True)
    narration, skip = set(), set()
    if path.suffix == ".py":
        tree = ast.parse(text)
        narration, skip = narration_lines(tree), docstring_lines(tree)
    changed = 0
    for index, line in enumerate(lines):
        number = index + 1
        if number in skip or (path.suffix == ".py" and line.lstrip().startswith("#")):
            continue
        new = rewrite_line(line, number in narration)
        if rename:
            new = rename_law(new)
        if new != line:
            lines[index] = new
            changed += 1
    if changed:
        rewrite(path, "".join(lines).encode("utf-8"))
    return changed


def rewrite(path: Path, data: bytes) -> None:
    old = path.stat().st_size
    with open(path, "r+b") as handle:
        handle.write(data)
        try:
            handle.truncate(len(data))
        except OSError:
            handle.write(b" " * max(0, old - len(data)))


def main(argv):
    rename = "--law" in argv
    names = [a for a in argv if not a.startswith("--")]
    total = 0
    for name in names:
        path = ENERGY / name
        count = process(path, rename)
        total += count
        print(count, name)
    print("changed lines", total)


main(sys.argv[1:])
