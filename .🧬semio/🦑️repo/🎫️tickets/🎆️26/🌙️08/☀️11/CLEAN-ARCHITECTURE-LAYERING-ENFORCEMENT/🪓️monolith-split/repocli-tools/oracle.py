"""🔬 Differential oracle for the repo CLI carve: patched full-resolution analysis, baseline sandbox vs union sandbox (analysis only, throwaway out-dir)."""
import collections, json, os, re, shutil, subprocess, sys

SCR = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/repocli"
CLI = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/"
TC = "+nightly-2026-07-20"
NEW = ["event_stream", "command_framework", "tree_source_and_search", "mcp_verb", "verbs"]
MOUNT = {
    "event_stream": "📡️event-stream/🦀️.rs",
    "command_framework": "🧭️repo-cli/🎮️command-framework/🦀️.rs",
    "tree_source_and_search": "🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs",
    "mcp_verb": "🔌️mcp-verb/🦀️.rs",
    "verbs": "🧭️repo-cli/🚦️verbs/🦀️.rs",
}
EXT = re.compile(r"^(unresolved import|failed to resolve: use of (unresolved|undeclared)|cannot find attribute|cannot find derive|can't find crate)")


def patch(tree):
    for dp, dn, fn in os.walk(tree):
        for n in fn:
            if not n.endswith(".rs"):
                continue
            p = os.path.join(dp, n)
            t = open(p, encoding="utf-8").read()
            t2 = re.sub(r"^\s*extern crate [^;]+;[ \t]*$", "", t, flags=re.M)
            if t2 != t:
                open(p, "w", encoding="utf-8").write(t2)


def mk_baseline(dst):
    shutil.rmtree(dst, ignore_errors=True)
    shutil.copytree(SCR + "/orig", dst)
    patch(dst)


def mk_union(dst, tree=CLI):
    shutil.rmtree(dst, ignore_errors=True)
    os.makedirs(dst)
    for dp, dn, fn in os.walk(tree):
        for n in fn:
            if n.endswith(".rs"):
                rel = os.path.relpath(os.path.join(dp, n), tree)
                os.makedirs(os.path.dirname(os.path.join(dst, rel)), exist_ok=True)
                shutil.copyfile(os.path.join(dp, n), os.path.join(dst, rel))
    names = "|".join(NEW)
    pat = re.compile(r"(?<![A-Za-z0-9_:])semio_framework_repo_cli_(" + names + r")(?![A-Za-z0-9_])")
    for dp, dn, fn in os.walk(dst):
        for n in fn:
            if n.endswith(".rs"):
                p = os.path.join(dp, n)
                t = open(p, encoding="utf-8").read()
                t2 = pat.sub(lambda m: "crate::semio_framework_repo_cli_" + m.group(1), t)
                if t2 != t:
                    open(p, "w", encoding="utf-8").write(t2)
    root = dst + "/🦀️.rs"
    t = open(root, encoding="utf-8").read()
    t = re.sub(r"^pub use crate::semio_framework_repo_cli_mcp_verb as mcp_verb;", "pub use crate::semio_framework_repo_cli_mcp_verb as mcp_verb;", t, flags=re.M)
    for k, rel in MOUNT.items():
        t += f'\n#[path = "{rel}"]\npub mod semio_framework_repo_cli_{k};\n'
    open(root, "w", encoding="utf-8").write(t)
    patch(dst)


def run(root, extra, name="semio_framework_repo_cli"):
    out = SCR + "/rmeta-out"
    shutil.rmtree(out, ignore_errors=True)
    os.makedirs(out)
    cmd = ["rustc", TC, "--edition", "2021", "--crate-type", "lib", "--crate-name", name, "--emit=metadata", "--out-dir", out, "--error-format=json", *extra, root]
    p = subprocess.run(cmd, cwd=os.path.dirname(root), capture_output=True, text=True)
    shutil.rmtree(out, ignore_errors=True)
    rows = []
    for line in p.stderr.splitlines():
        try:
            d = json.loads(line)
        except Exception:
            continue
        lvl = d.get("level", "")
        if lvl not in ("error", "warning"):
            continue
        code = (d.get("code") or {}).get("code") or ""
        ps = [s for s in d.get("spans", []) if s.get("is_primary")]
        f, ln, label = "", 0, ""
        if ps:
            s = ps[0]
            f = s["file_name"]
            if os.path.isabs(f):
                f = os.path.relpath(f, os.path.dirname(root))
            ln = s["line_start"]
            label = s.get("label") or ""
        rows.append({"level": lvl, "code": code, "msg": d["message"], "file": os.path.normpath(f), "line": ln, "label": label})
    return rows, p.stderr


def key(r):
    return (r["level"], r["code"], r["msg"], r["label"])


def summarize(rows):
    c = collections.Counter(key(r) for r in rows if r["level"] == "error")
    return c


if __name__ == "__main__":
    cfgs = {"native": [], "test": ["--test"]}
    which = sys.argv[1:] or ["native", "test"]
    mk_baseline(SCR + "/sb-base")
    mk_union(SCR + "/sb-split")
    for cfg in which:
        a, ea = run(SCR + "/sb-base/🦀️.rs", cfgs[cfg])
        b, eb = run(SCR + "/sb-split/🦀️.rs", cfgs[cfg])
        open(SCR + f"/err-base-{cfg}.txt", "w").write(ea)
        open(SCR + f"/err-split-{cfg}.txt", "w").write(eb)
        A, B = summarize(a), summarize(b)
        nonext = lambda c: sum(v for k, v in c.items() if not EXT.search(k[2]))
        print(f"[{cfg}] base errors={sum(A.values())} (non-ext {nonext(A)}) split errors={sum(B.values())} (non-ext {nonext(B)})")
        for k, v in sorted((B - A).items()):
            print("  +SPLIT", v, k)
        for k, v in sorted((A - B).items()):
            print("  -BASE ", v, k)
        wb = [r for r in b if r["level"] == "warning"]
        print(f"  split warnings: {len(wb)}")
        for r in wb[:40]:
            print("   W", r["code"], r["msg"][:100], r["file"], r["line"])
