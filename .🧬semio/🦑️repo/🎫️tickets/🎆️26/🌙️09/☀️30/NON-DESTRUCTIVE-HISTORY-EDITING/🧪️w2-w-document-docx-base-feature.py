"""📨️ W2-W-document: rewrites `📜️mutate-docx-ecma-376`'s Examples rows to the leaf wire payload (design §11, F10) —
run edits carry the canonical `DocxXmlAddress` the old `{path, runIndex}` shorthand resolved to (printed by the crate's
own `docx_block_run_address` on the real README, see `🗑️generated/w2w-document/docx-probe.txt`), `set-style-based-on`
carries `based_on`, `set-part` carries `content_type` + `bytes`; the `set-snapshot` rows leave the tables for their own
scenario pair (an entire package is no table cell) and the `no-mutation` baselines are deleted.

  python3 🧪️w2-w-document-docx-base-feature.py <probe-output.txt>
"""
import json, pathlib, re, sys

FEATURE = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧪️tests/📜️mutate-docx-ecma-376/🥒️.feature")
SNAPSHOT = '''  @id-set-snapshot
  @level-exhaustive
  @mode-differential
  Scenario: Replace the real document with the committed after-document's whole snapshot
    Given the real input document shared://📜️example-readme.docx
    And the committed after-document shared://🧾️readme-afters/📸️set-snapshot/➡️after.docx
    When the whole document is replaced by the snapshot decoded from the committed after-document
    Then the jszip reader reads the subject's package and the committed after-document as the same DOCX

  @id-set-snapshot-inverse
  @level-exhaustive
  @mode-differential
  Scenario: Undoing the whole-document replacement restores the document
    Given the real input document shared://📜️example-readme.docx
    And the committed after-document shared://🧾️readme-afters/📸️set-snapshot/➡️after.docx
    When the whole document is replaced by the committed after-document's snapshot and then undone
    Then the jszip reader reads the restored package and the real README as the same DOCX

'''


def wire(kind, params, address):
    if kind in ("set-run-text", "set-run-formatting"):
        rest = {key: value for key, value in params.items() if key not in ("path", "runIndex")}
        return {"address": address, **rest}
    if kind == "set-style-based-on":
        return {"id": params["id"], "based_on": params["basedOn"]}
    if kind == "set-part":
        return {"path": params["path"], "content_type": params["contentType"], "bytes": list(params["content"].encode())}
    return params


def align(block):
    rows = [[cell.strip() for cell in line.strip().strip("|").split("|")] for line in block]
    widths = [max(len(row[index]) for row in rows) for index in range(len(rows[0]))]
    indent = re.match(r"^(\s*)", block[0]).group(1)
    return [indent + "| " + " | ".join(cell.ljust(widths[index]) for index, cell in enumerate(row)) + " |" for row in rows]


def rewrite(text, address):
    text = re.sub(r"\n  @id-no-mutation-baseline-(mutate|inverse)\n.*?(?=\n  @id-)", "\n", text, flags=re.S)
    out, block = [], []
    for line in text.split("\n") + [""]:
        if re.match(r"^\s*\|", line):
            cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
            if cells[0] == "set-snapshot":
                continue
            if cells[0] != "id":
                cells[1] = json.dumps(wire(cells[0], json.loads(cells[1]), address), ensure_ascii=False, separators=(", ", ": "))
            block.append("      | " + " | ".join(cells) + " |")
            continue
        if block:
            out.extend(align(block))
            block = []
        out.append(line)
    text = re.sub(r"\n\n\n+", "\n\n", "\n".join(out[:-1]))
    return text.replace("  @id-identity-round-trip\n", SNAPSHOT + "  @id-identity-round-trip\n", 1)


if __name__ == "__main__":
    probe = pathlib.Path(sys.argv[1]).read_text()
    address = json.loads(re.search(r"\[DEBUG\] w2w address177 (\{.*\})", probe).group(1))
    text = FEATURE.read_text()
    assert "@id-set-snapshot" not in text, "already converted"
    FEATURE.write_text(rewrite(text, address))
    print("rewritten with", address)
