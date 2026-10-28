"""🔤 Derives {ASCII fallback token: real German token} from the German segments of a root with the macOS German spell checker as the independent oracle (token rejected, umlaut form accepted)."""
import json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import gqlib, rules
root = sys.argv[1]; out = sys.argv[2]; sp = sys.argv[3]
seg = []
for rel in gqlib.files_of(root, (".json", ".md") + gqlib.CODE_EXT):
    try: text = open(os.path.join(root, rel), encoding="utf-8").read()
    except Exception: continue
    if rel.endswith(".json"):
        try: doc = json.loads(text)
        except Exception: continue
        seg += [d for _p, _e, d in gqlib.pairs_of(doc) if d]
        seg += [d for _p, _e, d in gqlib.description_pairs(doc) if d]
    elif rel.endswith(".md"): seg += [l for _i, l in gqlib.md_german_lines(text)]
    else: seg += [s for _a, _b, s in gqlib.code_literals(text, [k for k in rules.TRANSLIT if len(k) >= 5])]
tok = re.compile(r"[A-Za-zÄÖÜäöüß]*(?:[aA]e|[oO]e|[uU]e)[A-Za-zÄÖÜäöüß]*")
words = set()
for s in seg:
    s = re.sub(r"`[^`]*`|\{[^}]*\}|https?://\S+", " ", s)
    for w in tok.findall(s):
        if len(w) >= 4 and not w.isupper(): words.add(w)
words = sorted(words)
conv = [rules._umlaut(w) for w in words]
def check(lst):
    r = subprocess.run([sp], input="\n".join(lst) + "\n", capture_output=True, text=True)
    return set(r.stdout.split("\n"))
bad_in, bad_conv = check(words), check(conv)
res = {w: c for w, c in zip(words, conv) if w in bad_in and c not in bad_conv and w != c}
json.dump(res, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1, sort_keys=True)
print(len(seg), "segments", len(words), "candidates", len(res), "accepted")
rejected = [(w, c) for w, c in zip(words, conv) if w in bad_in and c in bad_conv]
print("neither form accepted (manual review):", rejected[:80])
