"""📏️ Independent Python oracle of the owner byte budget (standard library only): a bootstrap ceiling must not exceed the Source revision text ceiling, and every fixed recovery owner manifest must leave the authored headroom below the bootstrap ceiling. `rows` answers JSON rows from stdin; `tree <root>` reads the two schemas and the owner manifests of a checkout and exits 1 when one owner outgrew its budget."""
import json, os, sys

CLI_SCHEMA = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🧬️schema/🔣️.json"
REVISION_SCHEMA = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json"


def accepted(row):
    """✅️ Decides one budget row: the bootstrap ceiling fits under the revision text ceiling and the owner stays inside its headroom (integer arithmetic)."""
    return row["maximumOwnerBytes"] <= row["maximumTextBytes"] and row["ownerBytes"] * 100 <= row["maximumOwnerBytes"] * (100 - row["headroomPercent"])


def tree(root):
    """🌳️ Reads both schemas and every fixed owner manifest of a checkout and decides each owner."""
    with open(os.path.join(root, CLI_SCHEMA), encoding="utf8") as handle:
        protocol = json.load(handle)["$defs"]["Protocol"]["const"]
    with open(os.path.join(root, REVISION_SCHEMA), encoding="utf8") as handle:
        text = json.load(handle)["definitions"]["Policy"]["const"]["maximumTextBytes"]
    owners = sorted({protocol["owner"], protocol["maintenance"]["owner"], protocol["reset"]["owner"]})
    rows = [{"owner": owner, "ownerBytes": os.path.getsize(os.path.join(root, owner, "📋️project.json")), "maximumOwnerBytes": protocol["maximumOwnerBytes"], "maximumTextBytes": text, "headroomPercent": protocol["ownerHeadroomPercent"]} for owner in owners]
    return [{**row, "accepted": accepted(row)} for row in rows]


if sys.argv[1] == "rows":
    print(json.dumps([accepted(row) for row in json.load(sys.stdin)["rows"]]))
else:
    found = tree(os.path.abspath(sys.argv[2]))
    print(json.dumps(found, ensure_ascii=False))
    sys.exit(0 if all(row["accepted"] for row in found) else 1)
