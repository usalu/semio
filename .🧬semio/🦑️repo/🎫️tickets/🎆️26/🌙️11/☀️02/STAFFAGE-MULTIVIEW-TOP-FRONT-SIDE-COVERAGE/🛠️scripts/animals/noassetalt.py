import json, sys
for f in sys.argv[1:]:
    doc = json.load(open(f, encoding="utf-8-sig"))
    for k, r in doc["asset"]["representations"].items():
        if r["status"] == "unavailable":
            r["alternatives"] = [x for x in r["alternatives"] if x["kind"] != "asset"]
            print(doc["asset"]["id"], k, r["alternatives"], "|", r["reason"]["en"][-160:], "|", r["reason"]["de"][-120:])
    open(f, "w", encoding="utf-8", newline="\n").write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
