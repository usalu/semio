"""🕳️ Adds or repairs the taxonomy rows of the en1997 ground-investigation kinds from the artifact's edit-rules leaf (the bun generator is unavailable); idempotent. Run: python3 -I taxonomy_ground_investigation.py [--check]"""
import json, os, re, sys

NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = NORM + "/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
TAX = NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"


def pascal(s):
    return "".join(p.capitalize() for p in s.split("-"))


def rules():
    text = open(ART + "/🧭️edit-rules/🦀️.rs", encoding="utf8").read()
    out = []
    for m in re.finditer(r'(SetFieldRule|InsertItemRule|RemoveItemRule) \{ path: "([^"]+)", kind: "([^"]*ground-investigation[^"]*)", selectors: &\[([^\]]*)\](?:, (?:value|index): "(\w+)")?(?:, item: "(\w+)")?', text):
        sels = [{"kind": "id", "key": k} for k in re.findall(r'SelectorKey::Id\("(\w+)"\)', m.group(4))]
        kind = {"SetFieldRule": "set-field", "InsertItemRule": "insert-item", "RemoveItemRule": "remove-item"}[m.group(1)]
        edit = {"rule": kind, "path": m.group(2), "selectors": sels}
        if kind == "set-field":
            edit["value"] = m.group(5)
        else:
            edit["index"] = m.group(5)
            if kind == "insert-item":
                edit["item"] = m.group(6)
        out.append((m.group(3), edit))
    return out


def main(check):
    dirs = {d.split(re.match(r"\W+", d).group(0))[-1] if False else re.sub(r"^[^a-z]+", "", d): d for d in os.listdir(ART)}
    tax = json.load(open(TAX, encoding="utf8"))
    rows = tax["rows"]
    template = next(r for r in rows if r["artifact"].endswith("en1997") and r["kind"] == "change-ground-investigation-depth")
    present = {r["kind"]: r for r in rows if r["artifact"] == template["artifact"]}
    wanted = rules()
    stale = [k for k in present if "ground-investigation" in k and k not in {w[0] for w in wanted}]
    new = []
    for kind, edit in wanted:
        module = dirs[kind]
        verb, slug = kind.split("-", 1)
        row = present.get(kind) or {}
        row = {**template, **row, "edit": edit, "kind": kind, "module": module, "entity": slug, "verb": verb, "aggregateVariant": pascal(kind), "type": pascal(kind),
               "record": {"change": "Changed", "insert": "Inserted", "remove": "Removed"}[verb] + pascal(slug), "source": template["source"].replace(template["module"], module)}
        row = dict(sorted(row.items()))
        if kind in present:
            rows[rows.index(present[kind])] = row
        else:
            new.append(row)
    for k in stale:
        rows.remove(present[k])
    at = max(i for i, r in enumerate(rows) if r["artifact"] == template["artifact"] and "ground-investigation" in r["kind"])
    rows[at + 1:at + 1] = new
    print(f"wanted {len(wanted)} rows, added {len(new)}, removed stale {stale}")
    if not check:
        json.dump(tax, open(TAX, "w", encoding="utf8"), indent=2, ensure_ascii=False)
        open(TAX, "a").write("\n")


main("--check" in sys.argv)
