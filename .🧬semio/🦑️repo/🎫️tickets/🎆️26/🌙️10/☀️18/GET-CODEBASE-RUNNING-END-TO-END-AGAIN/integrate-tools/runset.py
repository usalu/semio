#!/usr/bin/env python3
# 🧩 Runs ONE staged set against a COPY of the base tree and records what it changed.
# usage: runset.py <set> [--base DIR] [--work DIR] [--out DIR]
#   base: sparse snapshot of the live tree (library + extras), default /private/tmp/ig/base0
#   work: scratch root for the copy, default /private/tmp/ig/w-<set>
#   out : result directory; receives changes.json {rel: "created"|"modified"} and files/<n> copies
import hashlib, json, os, runpy, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
LIVE = "/Users/ueli/Documents/semio"


def run(cmd, env=None, cwd=None):
    result = subprocess.run(cmd, env={**os.environ, **(env or {})}, cwd=cwd, capture_output=True, text=True)
    sys.stdout.write(result.stdout[-3000:])
    sys.stderr.write(result.stderr[-3000:])
    if result.returncode:
        raise SystemExit(f"{cmd[0]} {cmd[1] if len(cmd) > 1 else ''} exited {result.returncode}")


def shim(path, replacements, argv, env=None, cwd=None):
    source = open(path, encoding="utf8").read()
    for old, new in replacements:
        if old not in source:
            raise SystemExit(f"shim anchor missing in {path}: {old[:60]}")
        source = source.replace(old, new)
    sys.argv = [path] + argv
    os.environ.update(env or {})
    sys.path.insert(0, os.path.dirname(path))
    if cwd:
        os.chdir(cwd)
    exec(compile(source, path, "exec"), {"__file__": path, "__name__": "__main__"})


def set_policy_admit(w):
    run(["bun", f"{C}/policy-admit-tools/apply.ts"], {"SEMIO_ROOT": w})


def set_cache_fix(w):
    env = {"SEMIO_ROOT": w}
    run(["bun", f"{C}/cache-fix-tools/apply-library.ts"], env)
    run(["bun", f"{C}/cache-fix-tools/apply-manifests.ts"], env)


def set_rcache(w):
    tools = f"{C}/rcache-tools"
    sys.path.insert(0, tools)
    import engine
    engine.ROOT = w
    sys.argv = [f"{tools}/apply.py", "--allow-drift"]
    try:
        runpy.run_path(f"{tools}/apply.py", run_name="__main__")
    except SystemExit as error:
        if error.code:
            raise


def set_rexec(w):
    run(["python3", f"{C}/rexec-tools/apply.py", "--apply", "--force-while-running"], {"REXEC_ROOT": w})


def set_bworkers(w):
    run(["python3", f"{C}/bworkers-tools/apply.py", "--write", "--root", w])


def set_bzl_coverage(w):
    run(["python3", "-I", f"{C}/bzl-coverage-tools/apply.py", "apply", "--root", w, "--rebase"])


def set_bzl_rust(w):
    run(["python3", f"{C}/bzl-rust-tools/apply.py"], cwd=w)


def set_bzl_lazy(w):
    shim(f"{C}/bzl-lazy-tools/apply.py",
         [('ROOT = "/Users/ueli/Documents/semio"', f'ROOT = "{w}"'), ('if not dry and subprocess.run(["pgrep"', 'if False and subprocess.run(["pgrep"'),
          ("        target = ALIAS.get(target, target)\n", f"        target = ALIAS.get(target, target)\n        target = '{w}/' + target[len('{LIVE}/'):] if target.startswith('{LIVE}/') else target\n")],
         [])


def set_graph_perf(w):
    run(["python3", f"{C}/graph-perf-tools/staged/apply.py", "apply", "--ignore-running"], {"GP_ROOT": w})


def set_bzl_xplat(w):
    run(["python3", "-I", f"{C}/bzl-xplat-tools/apply.py", "apply", "--root", w, "--ignore-sync"])


def set_tax6(w):
    run(["python3", f"{C}/tax6-tools/staged/apply.py", "--root", f"{w}/{L}"])


def set_vit4(w):
    shim(f"{C}/vit4-tools/staged-apply.py", [('R="/Users/ueli/Documents/semio/"', f'R="{w}/"')], ["--write"])


def set_zt_e2e_fix(w):
    run(["python3", f"{C}/zt-e2e-fix-tools/apply.py", "--root", w, "--ignore-running"])


def set_ingress_perf(w):
    run(["python3", f"{C}/ingress-perf-tools/apply.py", "--root", w])


def set_uncached_owners(w):
    env = {"SEMIO_ROOT": w}
    run(["bun", f"{C}/uncached-owners-tools/apply-library.ts", "--rebase"], env)
    run(["bun", f"{C}/uncached-owners-tools/apply-manifests.ts"], env)


SETS = {
    "ingress-perf": set_ingress_perf,
    "uncached-owners": set_uncached_owners,
    "zt-e2e-fix": set_zt_e2e_fix,
    "policy-admit": set_policy_admit, "cache-fix": set_cache_fix, "rcache": set_rcache, "rexec": set_rexec,
    "bworkers": set_bworkers, "bzl-coverage": set_bzl_coverage, "bzl-rust": set_bzl_rust, "bzl-lazy": set_bzl_lazy,
    "graph-perf": set_graph_perf, "bzl-xplat": set_bzl_xplat, "tax6": set_tax6, "vit4": set_vit4,
}


def tree(root):
    for base, dirs, names in os.walk(root):
        dirs[:] = [d for d in dirs if d != "__pycache__"]
        for name in names:
            path = os.path.join(base, name)
            yield os.path.relpath(path, root), path


def same(a, b):
    return os.path.getsize(a) == os.path.getsize(b) and open(a, "rb").read() == open(b, "rb").read()


def main():
    args = sys.argv[1:]
    name = args[0]
    opt = lambda flag, default: args[args.index(flag) + 1] if flag in args else default
    base, work = opt("--base", "/private/tmp/ig/base0"), opt("--work", f"/private/tmp/ig/w-{name}")
    out = opt("--out", f"/private/tmp/ig/r-{name}")
    shutil.rmtree(work, ignore_errors=True)
    shutil.rmtree(out, ignore_errors=True)
    subprocess.run(["cp", "-cR", base, work], check=True)
    SETS[name](work)
    os.makedirs(f"{out}/files")
    changes, index = {}, 0
    seen = set()
    for rel, path in tree(work):
        seen.add(rel)
        origin = os.path.join(base, rel)
        if os.path.exists(origin) and same(path, origin):
            continue
        if rel.endswith((".tmp", ".rcache.tmp", ".rexec.tmp", ".bzl-xplat.tmp", ".graphperf.tmp", ".tax6")):
            continue
        state = "modified" if os.path.exists(origin) else "created"
        shutil.copyfile(path, f"{out}/files/{index:04d}")
        changes[rel] = {"state": state, "file": f"{index:04d}", "sha256": hashlib.sha256(open(path, "rb").read()).hexdigest()}
        index += 1
    for rel, _ in tree(base):
        if rel not in seen:
            changes[rel] = {"state": "deleted"}
    json.dump(changes, open(f"{out}/changes.json", "w"), ensure_ascii=False, indent=1)
    json.dump({"base": base}, open(f"{out}/meta.json", "w"))
    print(f"{name}: {sum(1 for c in changes.values() if c['state'] == 'modified')} modified, {sum(1 for c in changes.values() if c['state'] == 'created')} created, {sum(1 for c in changes.values() if c['state'] == 'deleted')} deleted")


main()
