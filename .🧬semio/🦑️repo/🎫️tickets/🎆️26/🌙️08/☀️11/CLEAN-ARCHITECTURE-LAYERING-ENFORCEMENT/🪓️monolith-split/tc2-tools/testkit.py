"""🧰 tc2: `testkit` feature for a set of crates whose test-only serde derives (`#[cfg_attr(test, derive(serde::..))]`) must exist in the companion test crates.
usage: testkit.py <pkg>... [--apply]      (the packages form the set; features are forwarded along normal dependency edges inside the set)
Edits per package: every `cfg_attr(test,` of its source closure becomes `cfg_attr(any(test, feature = "testkit"),`; its Cargo.toml gets `testkit = [..]` (dep:serde, forwarded `<dep>/testkit`) and an optional `serde` dependency when serde is only a dev-dependency."""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, HERE + "/ttc")
from ttclib import *
import ttcunion as U
import fixrt

MANS = Manifests(Source(None))
MANS.build_index()
SRC = Source(None)
CFG_RX = re.compile(r"cfg_attr\(\s*test\s*,")


def closure_files(pkg):
    man = MANS.manifest(pkg, SRC)
    lib = MANS.lib_rel(man)
    files, code, missing = U.reach(SRC, lib, man["dir"])
    return man, {rel: files[rel].decode("utf-8") for rel in code}


def edit_manifest(man, forwards, needs_serde, apply):
    path = ROOT + "/" + man["rel"]
    text = open(path, encoding="utf-8").read()
    lines = text.split("\n")
    feat_items = []
    normal_serde = man["sections"].get("dependencies", {}).get("serde")
    serde_optional = normal_serde is None or "optional" in normal_serde
    if needs_serde and serde_optional:
        feat_items.append('"dep:serde"')
    feat_items += [f'"{k}/testkit"' for k in forwards]
    entry = "testkit = [" + ", ".join(feat_items) + "]"
    if re.search(r"^\s*testkit\s*=", text, re.M):
        cur = [l for l in lines if re.match(r"^\s*testkit\s*=", l)][0]
        have = set(re.findall(r'"([^"]+)"', cur))
        want = set(re.findall(r'"([^"]+)"', entry))
        if want <= have:
            entry = None
        else:
            merged = sorted(have | want)
            entry = "testkit = [" + ", ".join(f'"{x}"' for x in merged) + "]"
            lines = [entry if l == cur else l for l in lines]
            entry = None
    if entry is not None:
        at = next((i for i, l in enumerate(lines) if l.strip() == "[features]"), -1)
        if at >= 0:
            lines.insert(at + 1, entry)
        else:
            ins = next((i for i, l in enumerate(lines) if l.strip() == "[dependencies]"), -1)
            if ins < 0:
                ins = len(lines)
            lines[ins:ins] = ["[features]", entry, ""]
    if needs_serde and serde_optional:
        sec = man["sections"]
        normal = sec.get("dependencies", {})
        if "serde" not in normal:
            at = next((i for i, l in enumerate(lines) if l.strip() == "[dependencies]"), -1)
            assert at >= 0, man["rel"]
            end = at + 1
            while end < len(lines) and not lines[end].startswith("["):
                end += 1
            body = lines[at + 1:end]
            new = "serde = { workspace = true, optional = true }"
            pos = len(body)
            while pos > 0 and not body[pos - 1].strip():
                pos -= 1
            idx = pos
            for i, l in enumerate(body[:pos]):
                k = l.split("=")[0].strip()
                if k and k > "serde" and not l.startswith("#"):
                    idx = i
                    break
            body.insert(idx, new)
            lines[at + 1:end] = body
    new_text = "\n".join(lines)
    if new_text != text and apply:
        fixrt.save_orig(path)
        open(path, "w", encoding="utf-8").write(new_text)
    return new_text != text


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    apply = "--apply" in sys.argv
    pset = set(args)
    for pkg in args:
        man, files = closure_files(pkg)
        n = 0
        for rel, t in files.items():
            c = len(CFG_RX.findall(t))
            if c:
                n += c
                if apply:
                    fixrt.save_orig(ROOT + "/" + rel)
                    open(ROOT + "/" + rel, "w", encoding="utf-8").write(CFG_RX.sub('cfg_attr(any(test, feature = "testkit"),', t))
        normal_deps = man["sections"].get("dependencies", {})
        forwards = []
        for kind, name, pk, feats, optional, renamed in MANS.deps(man):
            if kind == "dependencies" and pk in pset and pk != pkg:
                forwards.append(name + ("?" if optional else ""))
        changed = edit_manifest(man, sorted(forwards), n > 0, apply)
        print(pkg.replace("semio-s-artifact-", ""), "cfg_attr sites", n, "forwards", forwards, "manifest changed" if changed else "manifest unchanged")


if __name__ == "__main__":
    main()
