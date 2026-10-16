"""Static proof of a split (svm): line multiset, rustfmt parse, #[path] resolution, sizes, region balance, glob ambiguity.

usage: verify.py <plan.py> [--stage]   (default: the real tree)"""
import collections, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from split import load_plan, read, ROOT, F, WORK, REGION
from rslex import outline
from rsmask import mask, idents, parse_use

NIGHTLY = "+nightly-2026-07-20"


def lines_of(t):
    ls = t.split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def norm(l):
    return re.sub(r"pub\(super\) ", "", l.strip())


def main():
    plan = load_plan(sys.argv[1])
    stage = "--stage" in sys.argv
    name = plan["name"]
    real_path = ROOT + "/" + plan["real"]
    real_dir = os.path.dirname(real_path)
    base = f"{WORK}/{name}/stage/" if stage else ROOT + "/"
    files = [plan["real"]] + [os.path.relpath(f"{real_dir}/{c['folder']}/🦀️.rs", ROOT) for c in plan["children"]]
    orig = lines_of(read(F + "/" + plan["orig"]))
    new = []
    out = []
    for rel in files:
        p = base + rel
        assert os.path.exists(p), f"missing {p}"
        t = read(p)
        ls = lines_of(t)
        new.extend(ls)
        depth = 0
        stack = []
        for l in ls:
            m = REGION.match(l)
            if m:
                if m.group(2):
                    assert stack and stack[-1] == m.group(3), f"{rel}: region mismatch {m.group(3)}"
                    stack.pop()
                else:
                    stack.append(m.group(3))
        assert not stack, f"{rel}: unclosed regions {stack}"
        r = subprocess.run(["rustfmt", NIGHTLY, "--edition", "2021", "--check", "--config", "skip_children=true", p], capture_output=True, text=True)
        perr = [l for l in (r.stdout + r.stderr).split("\n") if l.startswith("error")]
        out.append(f"{os.path.relpath(p, base)}: {len(ls)} lines, regions balanced, parse {'OK' if not perr else 'ERROR ' + perr[0]}")
        assert not perr, perr
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]', t):
            target = os.path.normpath(os.path.join(os.path.dirname(base + rel), m.group(1)))
            if not os.path.exists(target):
                alt = os.path.normpath(os.path.join(os.path.dirname(ROOT + "/" + rel), m.group(1)))
                assert os.path.exists(alt), f"{rel}: #[path] target missing {m.group(1)}"
            out.append(f"   path ok: {m.group(1)}")
    for rel in files:
        txt = read(base + rel)
        its = outline(txt)
        use_lines = set()
        stm = []
        for it in its:
            if it["kind"] == "use":
                for k in range(it["start"], it["end"] + 1):
                    use_lines.add(k)
                stm.append(it)
        ls = txt.split("\n")
        body = mask("\n".join(l for k, l in enumerate(ls) if k not in use_lines))
        pool = idents(body) | idents(body, True)
        for it in stm:
            head, trees = parse_use(mask("\n".join(ls[it["start"]:it["end"] + 1])))
            if "pub" in head or "cfg" in head:
                continue
            for tr in trees:
                if tr.glob:
                    continue
                b = tr.alias or (tr.path[-2] if tr.path[-1] == "self" else tr.path[-1])
                if b not in pool:
                    out.append(f"   UNUSED IMPORT? {os.path.basename(os.path.dirname(rel))}: {b}")
    a = collections.Counter(norm(l) for l in orig if l.strip())
    b = collections.Counter(norm(l) for l in new if l.strip())
    removed = a - b
    added = b - a
    out.append(f"orig lines={len(orig)} (nonblank {sum(a.values())}); new total lines={len(new)} (nonblank {sum(b.values())})")
    out.append(f"multiset removed from orig: {sum(removed.values())} lines in {len(removed)} distinct")
    for l, n in sorted(removed.items()):
        out.append(f"   - {n}x {l[:160]}")
    out.append(f"multiset added: {sum(added.values())} lines in {len(added)} distinct")
    for l, n in sorted(added.items()):
        out.append(f"   + {n}x {l[:160]}")
    big = [(rel, len(lines_of(read(base + rel)))) for rel in files]
    out.append("sizes: " + ", ".join(f"{os.path.basename(os.path.dirname(r))}={n}" for r, n in big))
    text = "\n".join(out)
    open(f"{F}/🗑️generated/svm-{name}-verify.log", "w", encoding="utf-8").write(text + "\n")
    print(text)


if __name__ == "__main__":
    main()
