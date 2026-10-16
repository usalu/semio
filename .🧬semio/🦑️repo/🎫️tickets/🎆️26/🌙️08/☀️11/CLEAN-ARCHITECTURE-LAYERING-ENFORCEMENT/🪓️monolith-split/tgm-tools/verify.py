"""Static gates for one slice target: lossless multiset, regions, rustfmt parse, path literals, mounts, sizes, defscan, docstrings."""
import collections, importlib, json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from tgmlib import *
from modtree import reachable, mounts_of
import split as S

TOOLCHAIN = "+nightly-2026-07-20"
REGION = re.compile(r"^\s*//\s?#(end)?region\b")


def live_files(name):
    t = TARGETS[name]
    r = reachable(os.path.join(ROOT, t["lib"]))
    return {os.path.relpath(f, ROOT): read(f) for f in r}


def main():
    name = sys.argv[1]
    spec = S.load_spec(name)
    t = TARGETS[name]
    base = os.path.dirname(t["root"])
    orig = read(os.path.join(snap_dir(name), t["root"]))
    live = live_files(name)
    root_new = live[t["root"]]
    kids = {}
    for ch in spec["children"]:
        rel = f"{base}/{ch['folder']}/🦀️.rs"
        kids[ch["folder"]] = read(os.path.join(ROOT, rel))
    out = dict(name=name)

    norm = lambda ls: [l.strip() for l in ls if l.strip()]
    old = collections.Counter(norm(orig.split("\n")))
    new_lines = norm(root_new.split("\n"))
    for k in kids.values():
        new_lines += norm(k.split("\n"))
    new = collections.Counter(new_lines)
    removed = old - new
    added = new - old
    classes = collections.Counter()
    leftover_added = collections.Counter(added)
    for l, n in list(added.items()):
        if l.startswith("//!"):
            classes["header-docstring"] += n; del leftover_added[l]
        elif l == "use super::*;":
            classes["use-super-glob"] += n; del leftover_added[l]
        elif re.match(r'^#\[path = "[^"]+/🦀️\.rs"\]$', l) or re.match(r"^(pub )?mod \w+;$", l) or re.match(r"^pub use \w+::\*;$", l):
            classes["mount"] += n; del leftover_added[l]
    leftover_removed = collections.Counter(removed)
    for l, n in list(added.items()):
        if l not in leftover_added:
            continue
        if l.startswith("pub(crate) ") and l[len("pub(crate) "):] in leftover_removed:
            m = min(n, leftover_removed[l[len("pub(crate) "):]])
            classes["widened-pub-crate"] += m
            leftover_removed[l[len("pub(crate) "):]] -= m
            if leftover_removed[l[len("pub(crate) "):]] <= 0:
                del leftover_removed[l[len("pub(crate) "):]]
            leftover_added[l] -= m
            if leftover_added[l] <= 0:
                del leftover_added[l]
        elif 'include!("../' in l:
            src = l.replace('include!("../', 'include!("')
            if src in leftover_removed:
                classes["include-rewrite"] += 1
                del leftover_removed[src]
                del leftover_added[l]
    # inline mod heads/closers and pub(super) conversions
    for l, n in list(leftover_removed.items()):
        if re.match(r"^(pub(\([^)]*\))? )?mod \w+ \{$", l) or l == "}":
            classes["inline-mod-wrapper"] += n
            del leftover_removed[l]
    for l, n in list(leftover_added.items()):
        if l == "}" or re.match(r"^(pub )?mod \w+ \{$", l):
            del leftover_added[l]
    for l, n in list(leftover_removed.items()):
        if "pub(super)" in l and l.replace("pub(super)", "pub(crate)") in leftover_added:
            classes["pub-super-to-crate"] += n
            del leftover_removed[l]
            del leftover_added[l.replace("pub(super)", "pub(crate)")]
    # hoisted / reordered docs leave identical multiset, so anything left is unexplained
    out["lossless"] = dict(old_lines=sum(old.values()), new_lines=sum(new.values()), classes=dict(classes),
                           unexplained_removed=dict(leftover_removed), unexplained_added=dict(leftover_added))

    reg_old = collections.Counter(l.strip() for l in orig.split("\n") if REGION.match(l))
    reg_new = collections.Counter()
    for tx in [root_new] + list(kids.values()):
        reg_new.update(l.strip() for l in tx.split("\n") if REGION.match(l))
    out["regions"] = dict(old=sum(reg_old.values()), new=sum(reg_new.values()), same_multiset=(reg_old == reg_new))
    for rel, tx in [(t["root"], root_new)] + [(f"{base}/{f}/🦀️.rs", k) for f, k in kids.items()]:
        st = collections.Counter()
        for l in tx.split("\n"):
            m = re.match(r"^\s*//\s?#(end)?region\s+(.*)$", l)
            if m:
                st[m.group(2).strip()] += -1 if m.group(1) else 1
        bad = {k: v for k, v in st.items() if v != 0}
        out.setdefault("region_balance_bad", {})[rel[-40:]] = bad if bad else "ok"

    rf = {}
    for rel, tx in [(t["root"], root_new)] + [(f"{base}/{f}/🦀️.rs", k) for f, k in kids.items()]:
        p = subprocess.run(["rustfmt", TOOLCHAIN, "--edition", "2021", "--config", "skip_children=true", "--check", os.path.join(ROOT, rel)], capture_output=True, text=True)
        err = [l for l in (p.stderr + "\n" + p.stdout).split("\n") if l.startswith("error")]
        rf[rel[-45:]] = ("parse-ok" if not err else err[:3]) if p.returncode in (0, 1) else ["rc=%d" % p.returncode] + err[:3]
    out["rustfmt_parse"] = rf

    pe = []
    maxlen = 0
    for rel, tx in [(t["root"], root_new)] + [(f"{base}/{f}/🦀️.rs", k) for f, k in kids.items()]:
        d = os.path.dirname(os.path.join(ROOT, rel))
        for kind, p, inl in mounts_of(tx):
            if kind == "plain":
                pe.append(("plain-mod", rel[-40:], p)); continue
            tgt = os.path.normpath(os.path.join(d, *inl, p)) if kind == "path" else os.path.normpath(os.path.join(d, p))
            if not os.path.isfile(tgt):
                pe.append(("missing", rel[-40:], p))
            maxlen = max(maxlen, len(os.path.relpath(tgt, ROOT).encode("utf-8")))
    for ch in spec["children"]:
        rel = f"{base}/{ch['folder']}/🦀️.rs"
        maxlen = max(maxlen, len(rel.encode("utf-8")))
    out["paths"] = dict(problems=pe, longest_bytes=maxlen)

    cnt = collections.Counter()
    for f, ms in reachable(os.path.join(ROOT, t["lib"])).items():
        for kind, p, tg in ms:
            if tg:
                cnt[tg] += 1
    dups = {os.path.relpath(k, ROOT)[-70:]: v for k, v in cnt.items() if v > 1}
    snapcnt = collections.Counter()
    for f, ms in reachable(os.path.join(snap_dir(name), t["lib"])).items():
        for kind, p, tg in ms:
            if tg:
                snapcnt[os.path.relpath(tg, snap_dir(name))] += 1
    sd = {k[-70:]: v for k, v in snapcnt.items() if v > 1}
    out["duplicate_mounts"] = dict(now=dups, before=sd, new=[k for k in dups if k not in sd])

    sizes = {}
    reach = reachable(os.path.join(ROOT, t["lib"]))
    tot_b = tot_l = 0
    for f in reach:
        tx = read(f)
        tot_b += len(tx.encode("utf-8"))
        tot_l += len([l for l in tx.split("\n") if l.strip() and not l.strip().startswith("//")])
    sizes["lib_closure"] = dict(files=len(reach), bytes=tot_b, active_lines=tot_l)
    sizes["root_lines"] = len(root_new.split("\n")) - 1
    sizes["children"] = {f: len(k.split("\n")) - 1 for f, k in kids.items()}
    out["sizes"] = sizes

    # definition scan on the live tree
    files = {os.path.relpath(f, ROOT): read(f) for f in reach}
    words, meth = S.token_sets(files)
    crossing = []
    for ch in spec["children"]:
        if ch.get("kind", "lines") != "lines":
            continue
        rel = f"{base}/{ch['folder']}/🦀️.rs"
        lines, depths, inlit, items = S.analyse(files[rel])
        for it in items:
            if it["kind"] in S.ITEM_KW and it["private"] and it["name"]:
                u = [f for f in words.get(it["name"], ()) if f != rel]
                if u:
                    crossing.append((ch["folder"], "item", it["name"], [x[-30:] for x in u[:3]]))
            for (li, ck, nm) in it["cands"]:
                if ck == "method":
                    u = [f for f in meth.get(nm, ()) if f != rel]
                elif ck == "field":
                    u = []
                    for f in words.get(nm, ()):
                        if f == rel:
                            continue
                        mt = S.masked_of(files, f)
                        if re.search(r"\." + re.escape(nm) + r"\b(?!\s*\()", mt) or re.search(r"\b(?:" + re.escape(it["name"]) + r"|Self)\b[^;{}]*\{[^{}]*\b" + re.escape(nm) + r"\b", mt):
                            u.append(f)
                else:
                    u = [f for f in words.get(it["name"], ()) if f != rel]
                if u:
                    crossing.append((ch["folder"], ck, nm, [x[-30:] for x in u[:3]]))
    out["defscan_remaining_private_crossings"] = crossing

    mods = []
    for ch in spec["children"]:
        for f, tx in files.items():
            if f == f"{base}/{ch['folder']}/🦀️.rs":
                continue
            if re.search(r"(?<![A-Za-z0-9_])" + re.escape(ch["mod"]) + r"\s*::", mask(tx)) and f != t["root"]:
                mods.append((ch["mod"], f[-60:]))
    out["modname_path_uses_elsewhere"] = mods
    print(json.dumps(out, ensure_ascii=False, indent=1))
    write(f"{F}/🗑️generated/tgm-{name}-verify.json", json.dumps(out, ensure_ascii=False, indent=1))


if __name__ == "__main__":
    main()
