"""🎒 Retargets the pack family from the replication facade to the codec crate (and the value and pack-error crates it also named through the facade)."""
import os, re, subprocess, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rcrates as C
import rewrite

PK = FW + "🎒️pack/"
CODEC = "semio_framework_replication_codec"

def git_files(patterns):
    out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "--"] + patterns, cwd=REPO, capture_output=True, text=True).stdout
    return [l for l in out.split("\n") if l.strip()]

def map_segments(segs):
    """segs after `protocol`; returns (replacement path text, consumed count)."""
    s0 = segs[0]
    if s0 == "codec": return CODEC, 1
    if s0 == "io": return CODEC + "::io", 1
    if s0 == "ByteSpan": return CODEC + "::ByteSpan", 1
    if s0 == "PackRefusal": return CODEC + "::PackRefusal", 1
    if s0 == "value": return "semio_framework_value", 1
    if s0 == "list": return "semio_framework_value::list", 1
    raise KeyError(segs)

def rewrite_source(text, used):
    toks = tokenize(text)
    sig = [i for i, t in enumerate(toks) if t[0] not in ("ws", "comment")]
    n = len(sig); edits = []
    for si in range(n):
        kd, tx, a, b = toks[sig[si]]
        if kd == "id" and tx == "protocol" and si + 2 < n and toks[sig[si + 1]][1] == ":" and toks[sig[si + 2]][1] == ":" and not (si > 0 and (toks[sig[si - 1]][1] == "." or (toks[sig[si - 1]][1] == ":" and si > 1 and toks[sig[si - 2]][1] == ":"))):
            if toks[sig[si + 3]][1] == "{":
                edits.append((a, toks[sig[si]][3], CODEC)); used.add("codec")
                continue
            segs = []; ends = []
            k = si + 3
            while k < n and toks[sig[k]][0] == "id":
                segs.append(toks[sig[k]][1]); ends.append(sig[k])
                if k + 3 < n and toks[sig[k + 1]][1] == ":" and toks[sig[k + 2]][1] == ":" and toks[sig[k + 3]][0] == "id": k += 3
                else: break
            repl, c = map_segments(segs)
            edits.append((a, toks[ends[c - 1]][3], repl)); used.add(segs[0])
    out = []; last = 0
    for a, b, r in sorted(edits):
        out.append(text[last:a]); out.append(r); last = b
    out.append(text[last:])
    return "".join(out)

def main(write_mode):
    model = rewrite.Model({})
    codec_world = model.worlds["codec"]
    rs = [f for f in git_files([PK + "**/*.rs"])]
    changed = {}
    check = set()
    for f in rs:
        t = open(REPO + f, encoding="utf8").read()
        if not re.search(r"\bprotocol\s*::", t): continue
        used = set()
        n = rewrite_source(t, used)
        if n != t: changed[f] = n
        if "protocol" in re.sub(r"//[^\n]*", "", n) and re.search(r"\bprotocol\s*::", "".join(x[1] for x in tokenize(n) if x[0] not in ("comment", "str"))):
            print("LEFTOVER protocol path", f)
    for f, n in changed.items():
        for m in re.finditer(CODEC + r"::((?:[A-Za-z_]\w*(?:::)?)+)", n):
            segs = [s for s in m.group(1).split("::") if s]
            if not segs: continue
            r, c = codec_world.walk(codec_world.root, ["crate"] + segs, mode=2)
            if r is None: print("UNRESOLVED in codec crate:", f.replace(PK, ""), "::".join(segs))
    mans = [f for f in git_files([PK + "**/Cargo.toml"]) if "semio-framework-replication" in open(REPO + f, encoding="utf8").read()]
    nodes = {os.path.dirname(os.path.dirname(os.path.dirname(f))): f for f in mans}
    owner = {}
    for rs_ in changed:
        best = max((d for d in nodes if rs_.startswith(d + "/")), key=len)
        owner.setdefault(nodes[best], []).append(rs_)
    newman = {}
    for f in mans:
        t = open(REPO + f, encoding="utf8").read()
        here = os.path.dirname(REPO + f)
        dep = C.cname("codec") + ' = { path = "' + os.path.relpath(REPO + C.pkgdir("codec"), here) + '" }'
        t2 = t.replace("semio-framework-replication = { workspace = true }", dep)
        t2 = t2.replace('"semio-framework-replication/deflate"', '"' + C.cname("codec") + '/deflate"')
        assert "semio-framework-replication " not in t2 and '"semio-framework-replication/' not in t2, f
        body = "\n".join(changed[x] for x in owner.get(f, []))
        add = []
        for ident, name, path in (("semio_framework_value", "semio-framework-value", FW + "🌱️value/" + PACKAGE),):
            if re.search(r"\b" + ident + r"\b", body) and not re.search(r"(?m)^" + re.escape(name) + r"\b", t2):
                add.append(name + ' = { path = "' + os.path.relpath(REPO + path, here) + '" }')
        if add:
            sect = re.search(r"(?m)^\[dependencies\]\n", t2)
            lines = t2[sect.end():].split("\n")
            k = 0
            while k < len(lines) and lines[k].strip() and not lines[k].startswith("["): k += 1
            block = sorted(lines[:k] + add, key=lambda l: l.split("=")[0].strip())
            t2 = t2[:sect.end()] + "\n".join(block) + "\n" + "\n".join(lines[k:])
        newman[f] = t2
    return changed, newman, mans

if __name__ == "__main__":
    changed, newman, mans = main(False)
    print("rs files:", len(changed), "manifests:", len(newman))
    for f in sorted(changed): print("  rs", f.replace(PK, ""))
    for f in sorted(newman): print("  toml", f.replace(PK, ""))

    if "--apply" in sys.argv:
        fx = PK + "🌱️value/🧪️tests/🧫️fixtures/🧩️ownership/🔣️.json"
        s = open(REPO + fx, encoding="utf8").read()
        assert s.count('    "semio-framework-replication",\n') == 1
        open(REPO + fx, "w", encoding="utf8").write(s.replace('    "semio-framework-replication",\n', '    "semio-framework-replication-codec",\n'))
        for lk in ("🧰️framework/Cargo.lock", "Cargo.lock"):
            s = open(REPO + lk, encoding="utf8").read()
            out = []
            for blk in s.split("\n[[package]]\n"):
                if re.match(r'name = "semio-framework-pack[a-z\-]*"\n', blk) and ' "semio-framework-replication",\n' in blk:
                    blk = blk.replace(' "semio-framework-replication",\n', ' "semio-framework-replication-codec",\n')
                out.append(blk)
            open(REPO + lk, "w", encoding="utf8").write("\n[[package]]\n".join(out))
        for rel in ("📜️script.ts", FW + "../🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-db-io-direct-writer/🟦️.ts"):
            rel = os.path.normpath(rel)
            s = open(REPO + rel, encoding="utf8").read()
            assert s.count("protocol::codec::KIND_") >= 1, rel
            open(REPO + rel, "w", encoding="utf8").write(s.replace("protocol::codec::KIND_", CODEC + "::KIND_"))
        for f, n in changed.items(): open(REPO + f, "w", encoding="utf8").write(n)
        for f, n in newman.items(): open(REPO + f, "w", encoding="utf8").write(n)
        print("applied")
