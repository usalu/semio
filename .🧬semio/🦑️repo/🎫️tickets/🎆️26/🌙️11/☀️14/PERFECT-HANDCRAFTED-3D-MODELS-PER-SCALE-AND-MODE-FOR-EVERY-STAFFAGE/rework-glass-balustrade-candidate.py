"""🧪 Builds the candidate asset document of `components/glass-balustrade`: catalog file, volumes source and the hand-made 2D edits."""
import json
import pathlib
import sys

TICKET = pathlib.Path(__file__).parent
CATALOG = TICKET.parents[6] / "🎓️teaching" / "🏛️architecture" / "🧍️staffage" / "🗂️catalog" / "🧱️components" / "glass-balustrade" / "🔣️.json"
SOURCE = TICKET / "volumes" / "components" / "glass-balustrade.json"
CANDIDATE = TICKET / "🗑️generated" / "rework-glass-balustrade" / "🔣️.json"


def shape(shapes, identifier):
    return next(item for item in shapes if item["id"] == identifier)


def edit(asset):
    coarse = asset["representations"]["200"]["geometry"]
    shape(coarse["elevation"], "glass-line")["penMm"] = 0.1075


def main():
    document = json.loads(CATALOG.read_text())
    edit(document["asset"])
    if "--publish" in sys.argv:
        CATALOG.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
        return
    document["asset"]["volumes"] = json.loads(SOURCE.read_text())["volumes"]
    CANDIDATE.parent.mkdir(parents=True, exist_ok=True)
    CANDIDATE.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")


if __name__ == "__main__":
    main()
