# usage: deps.py <lib-root> <crate_snake>   -> unresolved extern names per cfg (declared-dependency completeness)
import sys, subprocess, json, re, collections
root, crate = sys.argv[1], sys.argv[2]
CFG = {"native": [], "test": ["--test"], "wasm32": ["--target", "wasm32-unknown-unknown"], "wasip2": ["--target", "wasm32-wasip2"], "test-wasm32": ["--test", "--target", "wasm32-unknown-unknown"]}
only = sys.argv[3].split(",") if len(sys.argv) > 3 else list(CFG)
extra = sys.argv[4:] if len(sys.argv) > 4 else []
for name in only:
    p = subprocess.run(["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", "--error-format=json", "--cap-lints", "allow"] + CFG[name] + extra + [root], capture_output=True, text=True, env={**__import__('os').environ, 'CARGO_MANIFEST_DIR': __import__('os').environ.get('PKGDIR', '/nonexistent')})
    names = collections.Counter(); other = collections.Counter(); n = 0
    for line in p.stderr.splitlines():
        if not line.startswith("{"): continue
        d = json.loads(line)
        if not str(d.get("level", "")).startswith("error"): continue
        n += 1
        m = d.get("message", "")
        mm = re.search(r"unlinked crate `(\w+)`", m) or re.search(r"cannot find module or crate `(\w+)`", m) or re.search(r"unresolved import `(\w+)", m) or re.search(r"use of undeclared crate or module `(\w+)`", m) or re.search(r"could not find `\w+` in `(\w+)`", m) or re.search(r"failed to resolve: use of unresolved module or unlinked crate `(\w+)`", m) or re.search(r"cannot find (?:macro|attribute) `(\w+)`", m)
        if mm: names[mm.group(1)] += 1
        else: other[(d.get("code") or {}).get("code") or m[:90]] += 1
    print(name, "errors", n, "rc", p.returncode)
    print("  extern/unresolved:", dict(sorted(names.items())))
    print("  other:", dict(other.most_common(12)))
