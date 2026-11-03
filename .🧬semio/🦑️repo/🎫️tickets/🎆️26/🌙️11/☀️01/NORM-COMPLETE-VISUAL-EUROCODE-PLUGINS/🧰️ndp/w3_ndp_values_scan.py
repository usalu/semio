import json, re, os, sys, collections
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts"
REG = ROOT + "/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json"
CODES = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1992": "🏛️en1992", "en1993": "🔩️en1993", "en1994": "🧩️en1994", "en1995": "🪵️en1995", "en1996": "🪨️en1996", "en1997": "🌍️en1997", "en1998": "🫨️en1998", "en1999": "🪶️en1999"}
E = {e["id"]: e for e in json.load(open(REG, encoding="utf-8"))["entries"]}
LIT = re.compile(r'"(ndp\.[A-Za-z0-9_][A-Za-z0-9_.\-]*[A-Za-z0-9_])"')
COMMON = re.compile(r'ndp_common\(\s*"(ndp\.[^"{]+)"')
CELLC = re.compile(r'ndp_cell_common\(\s*"(ndp\.[^"{]+)"\s*,\s*&\[((?:\s*"[^"{}]*"\s*,?)*)\s*\]')
def has(v): return v is not None
def scan(code):
    reads = collections.defaultdict(list); commons = collections.defaultdict(list)
    for dp, dn, fn in os.walk(os.path.join(ART, CODES[code])):
        dn[:] = [d for d in dn if not d.startswith("🧪") and d not in ("tests", "target", "__pycache__", "🗑️generated")]
        for f in fn:
            if not f.endswith(".rs"): continue
            p = os.path.join(dp, f); rel = os.path.relpath(p, ROOT)
            text = open(p, encoding="utf-8").read()
            for ln, line in enumerate(text.split("\n"), 1):
                for m in LIT.finditer(line): reads[m.group(1)].append(f"{rel}:{ln}")
                for m in COMMON.finditer(line): commons[m.group(1)].append(f"{rel}:{ln}")
            for m in CELLC.finditer(text):
                i = m.group(1) + "." + ".".join(re.findall(r'"([^"]*)"', m.group(2)))
                commons[i].append(rel)
    return reads, commons
def resolve(i):
    if i in E: return [i]
    return [k for k in E if k.startswith(i + ".")]
if __name__ == "__main__":
    out = {}
    for code in CODES:
        reads, commons = scan(code)
        miss_en, miss_de, nonnum = [], [], []
        for i, where in sorted(reads.items()):
            ids = resolve(i)
            if not ids: continue
            en = any(has(E[k]["recommended"]) for k in ids); de = any(has(E[k]["naDe"]) for k in ids)
            if not en: miss_en.append(i)
            if not de: miss_de.append(i)
        cm = {}
        for i, where in sorted(commons.items()):
            ids = resolve(i)
            if not ids: cm[i] = "unregistered"; continue
            for k in ids:
                e = E[k]; a, b = e["recommended"], e["naDe"]
                if a is None or b is None: cm[k] = f"unset en={a} de={b}"
                elif a != b: cm[k] = f"differs en={a} de={b}"
        out[code] = dict(read=len(reads), miss_en=miss_en, miss_de=miss_de, common=len(commons), common_bad=cm)
        print(code, "read", len(reads), "noEN", len(miss_en), "noDE", len(miss_de), "common", len(commons), "common_bad", len(cm))
    json.dump(out, open(sys.argv[1], "w"), ensure_ascii=False, indent=1)
