#!/usr/bin/env python3
# 🧪️ Shell test bins (plan Appendix D): moves the test mounts of the Shell tree into the companion crates `shell-tests-<bin>`.
# usage: bins.py plan | bins.py apply <bin>... | bins.py reroot <bin>...
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/w1")
sys.path.insert(0, HERE)
import flipfc as X
import flip as F
from flip import *

PLAN = read(SPLIT + "/PLAN-RENDERER-WGPU-CRATES.md")
sec = PLAN[PLAN.index("## Appendix D."):PLAN.index("## Appendix E.")]
BINS = {}
for m in re.finditer(r"^\| `shell-tests-([a-z0-9-]+)` \| ([^|]+) \|", sec, re.M):
    BINS[m.group(1)] = [x.strip() for x in m.group(2).split(",")]


def test_mounts():
    files = [SH + "/🦀️.rs"] + [m["file"] for m in original_mounts() if m["kind"] == "chunk"]
    out = []
    for f in files:
        t = read(f)
        for m in re.finditer(r"((?:#\[cfg[^\n]*\]\n)*)#\[path\s*=\s*\"([^\"\n]*)\"\]\n((?:pub(?:\([a-z]+\))? )?)mod ([A-Za-z_0-9]+);\n", t):
            tgt = os.path.normpath(os.path.join(os.path.dirname(f), m.group(2)))
            if "🧪️tests" in m.group(2) or "/tests/" in m.group(2):
                out.append(dict(host=f, cfg=m.group(1), target=tgt, name=m.group(4), vis=m.group(3), text=m.group(0), dirname=os.path.basename(os.path.dirname(tgt))))
    return out


def assignment():
    mounts = test_mounts()
    by_dir = {m["dirname"]: m for m in mounts}
    res, unassigned = {}, []
    used = set()
    for b, dirs in BINS.items():
        res[b] = []
        for d in dirs:
            if d in by_dir:
                res[b].append(by_dir[d]); used.add(d)
    for m in mounts:
        if m["dirname"] not in used:
            unassigned.append(m)
    return res, unassigned


X.SHELL_IDX.setdefault("section_fixtures", "shell-windows")


def bin_crate(b):
    return "shell-tests-" + b


def existing_mounts(b):
    suffix = bin_crate(b)
    root = T + "/" + X.TABLE[suffix] + "/🦀️.rs"
    text = read(root)
    out = []
    for m in re.finditer(r"((?:#\[cfg[^\n]*\]\n)*)#\[path = \"([^\"\n]+)\"\]\nmod ([A-Za-z_0-9]+);", text):
        if m.group(2).startswith("🧪️tests"):
            continue
        target = os.path.normpath(os.path.join(os.path.dirname(root), m.group(2)))
        out.append(dict(host=None, cfg=m.group(1), target=target, name=m.group(3), vis="", text="", dirname=os.path.basename(os.path.dirname(target))))
    return out


SHELL_SOURCE_FN = '''
pub(crate) fn shell_source() -> &'static str {
    static SOURCE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    SOURCE
        .get_or_init(|| {
            let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let mut roots = vec![manifest.join("../../../../..").join("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")];
            for folder in SHELL_CRATE_FOLDERS {
                roots.push(manifest.join("../../..").join(folder).join("🦀️.rs"));
            }
            roots
                .iter()
                .map(|root| semio_framework_os_renderer_wgpu_test_support::read_module_tree_source(root).expect("the shell source tree is readable"))
                .collect::<String>()
        })
        .as_str()
}
'''


def root_text(b, mounts):
    suffix = bin_crate(b)
    X.CURRENT[0] = suffix
    files = [m["target"] for m in mounts]
    aliases, uses_ = needs_bin(suffix, files)
    folder = X.TABLE[suffix]
    head = [l for l in read(T + "/" + folder + "/🦀️.rs").split("\n") if l.startswith("//!")][:1]
    mt = []
    for m in mounts:
        rel = os.path.relpath(m["target"], T + "/" + folder)
        cfg = m["cfg"] if m["cfg"] else ""
        cfg = re.sub(r"#\[cfg\(test\)\]\n", "", cfg) if cfg.strip() == "#[cfg(test)]" else cfg
        mt.append(f'{cfg}#[path = "{rel}"]\nmod {m["name"]};')
    src = ""
    if any("shell_source()" in read(m["target"]) for m in mounts):
        folders = [X.TABLE[c] for c in SHELL_ORDER]
        src = "\nconst SHELL_CRATE_FOLDERS: &[&str] = &[" + ", ".join('"' + f + '"' for f in folders) + "];\n" + SHELL_SOURCE_FN
    body = head + ["//!", "//! 🧭️ The Shell tests of this bin are mounted from their place in the Shell element tree; the namespace they were written against is rebuilt by the imports below.", "", "#![cfg(test)]", ""] + aliases + uses_ + [""] + mt + [""] + ([src] if src else [])
    return "\n".join(body)


EXTRA_P1 = {}


def needs_bin(suffix, files):
    toks = set()
    allclean = ""
    for f in files:
        t = read(f)
        toks |= tokens(t)
        allclean += blank(t) + "\n"
    local = set()
    for f in files:
        for it in itm.table(read(f)):
            if it["kind"] not in ("impl", "use"):
                local.add(it["name"])
    by = collections.defaultdict(lambda: collections.defaultdict(set))
    plain = []
    P1ALL = collections.defaultdict(list)
    for k, v in F.P1.items():
        P1ALL[k] += v
    for k, v in X.P1R.items():
        for e in v:
            if e not in P1ALL[k]:
                P1ALL[k].append(e)
    for k, v in EXTRA_P1.items():
        for e in v:
            if e not in P1ALL[k]:
                P1ALL[k].append(e)
    for t in sorted(toks):
        if t in local:
            continue
        if t in X.SHELL_IDX:
            y = X.SHELL_IDX[t]
            cfgs = F.exports_of(y)[t]
            cfg = cfgs[0] if len(set(cfgs)) == 1 else ""
            by[(X.lib_of(y), cfg)][t] = None
            continue
        if t in X.FC_IDX:
            y = X.FC_IDX[t]
            mod = X.FC_MOD.get(t)
            cfgs = X.fc_exports(y)[t]
            cfg = cfgs[0] if len(set(cfgs)) == 1 else ""
            by[(X.lib_of(y) + ("::" + mod if mod else ""), cfg)][t] = None
            continue
        if t in X.ROOT_GLOB:
            by[(X.ROOT_GLOB[t], "")][t] = None
            continue
        if t in P1ALL:
            seen = set()
            for cfg, p in P1ALL[t]:
                if len(p) == 1:
                    plain.append((cfg + "\n" if cfg else "") + f"use {p[0]} as {t};")
                    continue
                if not cfg and len(p) == 2 and p[0].startswith(PFX):
                    cfg = def_cfg(p[0], p[1])
                key = (cfg, tuple(p))
                if key in seen:
                    continue
                seen.add(key)
                piece = p[-1] if p[-1] == t else f"{p[-1]} as {t}"
                by[("P1", cfg, tuple(p[:-1]))][piece] = None
    lines = []
    for key in sorted(by, key=lambda k: (str(k[0]) if k[0] != "P1" else "~" + "::".join(k[2]), k[1] if len(k) > 1 else "")):
        names = sorted(by[key])
        if key[0] == "P1":
            lines.append(render_use(key[1], list(key[2]), names))
        else:
            lines.append(render_use(key[1], [key[0]], names))
    aliases = []
    for a, (line, cfg) in EXT_ALIASES.items():
        if re.search(r"(?<![A-Za-z0-9_])" + a + r"\s*::", allclean) or any(re.match(r"(?:#\[[^\n]*\]\n)?use " + a + r"::", l) for l in lines):
            aliases.append(((cfg + "\n") if cfg else "") + line)
    return aliases, lines + sorted(set(plain))


def rewrite_source_consts(text):
    names = re.findall(r"(?m)^const ([A-Z_]*SHELL_SOURCE): &str = crate::shell::SHELL_SOURCE;\n", text)
    for n in names:
        text = re.sub(r"(?m)^const " + n + r": &str = crate::shell::SHELL_SOURCE;\n", "#[allow(non_snake_case)]\nfn " + n + "() -> &'static str {\n    shell_source()\n}\n", text)
        text = re.sub(r"(?<![A-Za-z0-9_])" + n + r"(?![A-Za-z0-9_(])", n + "()", text)
        text = text.replace("fn " + n + "()() ->", "fn " + n + "() ->")
    text = text.replace("crate::shell::SHELL_SOURCE", "shell_source()")
    return text


def transform_test(path):
    t = rewrite_source_consts(read(path))
    t = X.rewrite_heads(t)
    t = F.rewrite_paths(t)
    return t


def transform_test_probe(text):
    t = rewrite_source_consts(text)
    t = X.rewrite_heads(t)
    return F.rewrite_paths(t)


def transform_test_old(path):
    t = read(path)
    t = X.rewrite_heads(t)
    t = F.rewrite_paths(t)
    return t


STAYS = []


def apply_bin(b, res):
    suffix = bin_crate(b)
    X.CURRENT[0] = suffix
    folder = X.TABLE[suffix]
    mounts, newtexts = existing_mounts(b), {}
    for m in mounts:
        newtexts[m["target"]] = read(m["target"])
    for m in res[b]:
        try:
            newtexts[m["target"]] = transform_test(m["target"])
            mounts.append(m)
        except KeyError as e:
            STAYS.append((b, m["dirname"], str(e)))
    if not mounts:
        return "no test could move"
    F.backup(suffix, [m["target"] for m in mounts] + [m["host"] for m in mounts if m.get("host")] + [T + "/" + folder + "/🦀️.rs", T + "/" + folder + "/📦️packages/🦀️rust/Cargo.toml"])
    for m in mounts:
        t = newtexts[m["target"]]
        if t != read(m["target"]):
            write(m["target"], t)
    for m in mounts:
        if m.get("host") is None:
            m["hostless"] = True
    rt = root_text(b, mounts)
    write(T + "/" + folder + "/🦀️.rs", rt)
    hosts = collections.defaultdict(list)
    for m in mounts:
        if m.get("host"):
            hosts[m["host"]].append(m)
    for h, ms in hosts.items():
        t = read(h)
        for m in ms:
            assert t.count(m["text"]) == 1, (h, m["name"])
            t = t.replace(m["text"], "")
        write(h, t)
    used = first_party_used([m["target"] for m in mounts]) | {x.replace("_", "-") for x in re.findall(PFX + r"([a-z0-9_]+)", rt)}
    return ensure_deps(suffix, used, rt)


if __name__ == "__main__":
    res, un = assignment()
    if sys.argv[1] == "plan":
        for b, ms in res.items():
            print(b, len(ms), [m["dirname"] for m in ms])
        print("unassigned:", [(m["dirname"], m["name"]) for m in un])
    elif sys.argv[1] == "apply":
        for b in sys.argv[2:]:
            print(b, apply_bin(b, res))
    elif sys.argv[1] == "reroot":
        for b in sys.argv[2:]:
            X.CURRENT[0] = bin_crate(b)
            folder = X.TABLE[bin_crate(b)]
            files = [m["target"] for m in res[b]] if res[b] else []
            print(b, "needs reroot via saved mounts")
