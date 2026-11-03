"""Lists binaryTag values of the mutation leaf JSON schemas per artifact, compares them with the protocol file and flags duplicates; usage: python3 -I binary_tags.py [artifact-dir-substring] [--fix]."""
import glob, json, re, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
fix = "--fix" in sys.argv
want = [a for a in sys.argv[1:] if not a.startswith("--")]
def find(o):
    if isinstance(o, dict):
        for k, v in o.items():
            if k == "binaryTag": yield v
            else: yield from find(v)
    elif isinstance(o, list):
        for v in o: yield from find(v)
for art in sorted(glob.glob(ROOT + "/*")):
    if want and not any(w in art for w in want): continue
    protos = glob.glob(art + "/**/💾️binary/🧬️mutations/📡️.protocol.semio", recursive=True)
    tags = {}
    for p in protos:
        for k, t in re.findall(r"^record (\S+) tag=(\d+)$", open(p).read(), re.M): tags[k] = int(t)
    seen = {}
    files = {}
    for f in glob.glob(art + "/**/🧬️mutations/**/🔣️.json", recursive=True):
        try: d = json.load(open(f))
        except Exception: continue
        if not isinstance(d, dict) or "binaryTag" not in d or "semanticKind" not in d: continue
        seen.setdefault(d["binaryTag"], []).append((d["semanticKind"], f))
        files[d["semanticKind"]] = (f, d["binaryTag"])
    dups = {t: [k for k, _ in v] for t, v in seen.items() if len(v) > 1}
    wrong = [(k, t, tags.get(k)) for k, (f, t) in files.items() if k in tags and tags[k] != t]
    missing = [k for k in files if k not in tags]
    print(art.split("/")[-1], "protocol", len(tags), "json", len(files), "dups", dups, "mismatch", wrong[:8], len(wrong), "no-record", missing[:5])
    if fix:
        for k, t, p in wrong:
            f = files[k][0]
            txt = open(f).read()
            new = re.sub(r'("binaryTag":\s*)%d\b' % t, r"\g<1>%d" % p, txt, count=1)
            assert new != txt, f
            open(f, "w").write(new)
        print("fixed", len(wrong))
