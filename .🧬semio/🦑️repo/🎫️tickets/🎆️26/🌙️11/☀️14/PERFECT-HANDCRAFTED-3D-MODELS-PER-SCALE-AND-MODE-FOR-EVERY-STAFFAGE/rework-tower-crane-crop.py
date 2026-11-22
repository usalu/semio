"""🔍 Writes a candidate document that keeps only the body solids of the named bones (and an id filter), for close-up sheets with `r3-sheet.ts --file`.

Usage: python3 rework-tower-crane-crop.py <scale> <out.json> <bone,bone,...> [id-substring-to-drop,...]
"""
import json
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json"
SOURCE = "volumes/machinery/tower-crane.json"

if __name__ == "__main__":
    scale, out, bones = sys.argv[1], sys.argv[2], set(sys.argv[3].split(","))
    drop = sys.argv[4].split(",") if len(sys.argv) > 4 else []
    document = json.load(open(CATALOG))
    document["asset"]["volumes"] = json.load(open(SOURCE))["volumes"]
    volume = document["asset"]["volumes"][scale]
    volume["body"] = [solid for solid in volume["body"] if ("ALL" in bones or solid["bone"] in bones) and not any(part in solid["id"] for part in drop)]
    json.dump(document, open(out, "w"), ensure_ascii=False)
