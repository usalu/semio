"""⚖️ E51 oracle comparison: diffs the non-stub rows of two oracle runs of one family (before = tree with the slice's edits undone, after = current tree), keyed by code, crate, file tail and message (line numbers dropped because the edits shift them).
Reads the full `new_list` of GEN/e51-<family>-<label>-result.json (the `-new-*.txt` files are capped at 400 rows). Prints per cfg: canary reached, totals, privacy rows, new and gone rows.
usage: cmp.py <family> <before-label> <after-label>"""
import collections, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/../pos-tools")
from poslib import GEN


def load(fam, label):
    p = f"{GEN}/e51-{fam}-{label}-result.json"
    return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else None


def key(x):
    f = x["file"]
    crate, _, rest = f.partition("/")
    return (x["code"], crate, rest.split("/")[-1], re.sub(r"\s+", " ", x["msg"])[:160])


def main():
    fam, b, a = sys.argv[1:4]
    rb, ra = load(fam, b), load(fam, a)
    if not rb or not ra:
        print(f"{fam}: missing result json")
        return 1
    new_total = 0
    for cfg in ra["summary"]:
        sb, sa = rb["summary"].get(cfg), ra["summary"][cfg]
        if not sb:
            print(f"{fam} {cfg}: not in before run")
            continue
        kb = collections.Counter(key(x) for x in sb["new_list"] if not x["stub"])
        ka = collections.Counter(key(x) for x in sa["new_list"] if not x["stub"])
        priv = sum(1 for x in sa["new_list"] if not x["stub"] and x["cat"] == "privacy")
        new, gone = ka - kb, kb - ka
        new_total += sum(new.values())
        print(f"{fam} {cfg}: canary after {sa['canary_cur']} before {sb['canary_cur']}; errors before {sb['cur_total']} after {sa['cur_total']}; non-stub rows before {sum(kb.values())} after {sum(ka.values())}; privacy after {priv}; NEW {sum(new.values())} GONE {sum(gone.values())}")
        for k, n in sorted(new.items()):
            print(f"   NEW  x{n} {k}")
        for k, n in sorted(gone.items()):
            print(f"   GONE x{n} {k}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
