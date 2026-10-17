"""⚖️ C4P44 exact losslessness proof on the LIVE tree: per file, the stripped non-blank line multiset of the snapshot equals root + children after (a) removing the declared wiring (one `include!` line per child and the child's first docstring line) and (b) reversing the declared rebase (one `../` removed from every relative include_str!/include_bytes!/include! literal of a CHILD).
Also checks the sequence: splicing the children back at their include! lines in the root reproduces the snapshot text line for line (blank lines normalised). usage: lossless.py"""
import collections, glob, json, os, re, sys
REPO = "/Users/ueli/Documents/semio/"
ok = True
for path in sorted(glob.glob(os.path.dirname(os.path.abspath(__file__)) + "/specs/*.json")):
    s = json.load(open(path, encoding="utf-8"))
    base = os.path.dirname(s["file"])
    orig = open(s["original"], encoding="utf-8").read()
    root = open(REPO + s["file"], encoding="utf-8").read()
    kids = {c["folder"]: open(f"{REPO}{base}/{c['folder']}/🦀️.rs", encoding="utf-8").read() for c in s["chunks"]}
    def derebase(l):
        return re.sub(r'(include(?:_str|_bytes)?!\(\s*)"\.\./', r'\1"', l, count=1)
    new = []
    spliced = []
    for line in root.split("\n"):
        m = re.fullmatch(r'include!\("([^"]+)/🦀️\.rs"\);', line.strip())
        if m:
            body = kids[m.group(1)].split("\n")
            assert re.match(r"^//[/]? \S", body[0]), (m.group(1), body[0][:50])
            body = body[1:]
            body = [derebase(l) for l in body]
            new.extend(body)
            spliced.extend(body)
        else:
            new.append(line)
            spliced.append(line)
    strip = lambda ls: collections.Counter(l.strip() for l in ls if l.strip())
    gone = strip(orig.split("\n")) - strip(new)
    extra = strip(new) - strip(orig.split("\n"))
    seq_o = [l.strip() for l in orig.split("\n") if l.strip()]
    seq_n = [l.strip() for l in spliced if l.strip() and not re.fullmatch(r'include!\("[^"]+/🦀️\.rs"\);', l.strip())]
    seq_o2 = seq_o
    same_seq = seq_o2 == seq_n
    rel_in_root = sum(1 for l in root.split("\n") if re.search(r'include(?:_str|_bytes)?!\(\s*"[^"/]', l) or re.search(r'include_str!\(\s*"\.\./', l))
    print(f"{os.path.basename(path)} gone={sum(gone.values())} extra={sum(extra.values())} sequence-equal={same_seq} include-literals-in-root={rel_in_root}")
    for l, k in list(gone.items())[:5]: print("   GONE", k, l[:120])
    for l, k in list(extra.items())[:5]: print("   EXTRA", k, l[:120])
    if gone or extra or not same_seq:
        ok = False
print("LOSSLESS", "OK" if ok else "FAILED")
sys.exit(0 if ok else 1)
