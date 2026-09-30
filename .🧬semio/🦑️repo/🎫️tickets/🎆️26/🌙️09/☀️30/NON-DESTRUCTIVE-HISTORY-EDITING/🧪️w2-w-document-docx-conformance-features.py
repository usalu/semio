"""📨️ W2-W-document: rewrites the DOCX strict/transitional conformance-class features to the wire (design §11, F10) —
`set-snapshot` rows leave the Examples tables for the `stamp-conformance-class` scenario pair (the whole stamped
package is its payload, no table cell), the `no-mutation` baselines are deleted (the identity round trip is the
baseline), `insert-vml-part` carries its `markup`, and every table is re-aligned."""
import json, pathlib, re

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets")
VML = '<xml xmlns:v="urn:schemas-microsoft-com:vml"><v:shape id="legacyShape" type="#_x0000_t202"/></xml>'
STAMP = '''  @id-stamp-conformance-class
  @level-exhaustive
  @mode-differential
  Scenario: Stamp the real package into the strict class as one whole-package set-snapshot
    Given the real input package shared://📜️example-readme.docx
    When the package is stamped strict and the stamped package replaces it through set-snapshot
    Then the oracle and the subject agree on the conformance-class projection

  @id-stamp-conformance-class-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing the strict stamp restores the real package
    Given the real input package shared://📜️example-readme.docx
    When the package is stamped strict through set-snapshot and then stamped back
    Then the conformance-class projection is the one the package started from

'''


def align(lines):
    rows = [[cell.strip() for cell in line.strip().strip("|").split("|")] for line in lines]
    widths = [max(len(row[index]) for row in rows) for index in range(len(rows[0]))]
    indent = re.match(r"^(\s*)", lines[0]).group(1)
    return [indent + "| " + " | ".join(cell.ljust(widths[index]) if index < len(row) - 1 else cell.ljust(widths[index]) for index, cell in enumerate(row)) + " |" for row in rows]


def rewrite(text):
    text = re.sub(r"\n  @id-no-mutation-baseline-(mutate|inverse)\n.*?(?=\n  @id-)", "\n", text, flags=re.S)
    lines = [line for line in text.split("\n") if not re.match(r"^\s*\|\s*set-snapshot\s*\|", line)]
    out, block = [], []
    for line in lines + [""]:
        if re.match(r"^\s*\|", line):
            match = re.match(r"^(\s*)\|\s*insert-vml-part\s*\|\s*(\{.*\})\s*\|\s*$", line)
            if match:
                params = json.loads(match.group(2))
                params["markup"] = VML
                line = f"{match.group(1)}| insert-vml-part | {json.dumps(params, ensure_ascii=False, separators=(', ', ': '))} |"
            block.append(line)
            continue
        if block:
            out.extend(align(block))
            block = []
        out.append(line)
    text = "\n".join(out[:-1])
    return text.replace("  @id-identity-round-trip\n", STAMP + "  @id-identity-round-trip\n", 1)


for folder in ("📏️strict", "🔄️transitional"):
    path = next((ROOT / folder / "🧪️tests").glob("*/🥒️.feature"))
    text = path.read_text()
    if "@id-stamp-conformance-class" in text:
        print("unchanged", folder)
        continue
    path.write_text(rewrite(text))
    print("rewritten", folder)
