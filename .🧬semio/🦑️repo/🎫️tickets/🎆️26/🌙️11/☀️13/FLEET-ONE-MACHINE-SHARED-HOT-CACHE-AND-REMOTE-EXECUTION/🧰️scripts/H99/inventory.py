#!/usr/bin/env python3
"""H99 inventory: classify every build script (registry crates of the cargo cache + workspace build.rs) by what it probes.
Usage: inventory.py <out.json> [workspace-build.rs ...]. Python 3.9, stdlib only. Mirrors offload policy v1 (current) and the proposed probe classes."""
import glob, json, os, re, sys, collections

CUR_MARKERS = [
 ("spawns-process", r"\bCommand\s*::\s*new\b|process\s*::\s*Command"),
 ("network", r"std\s*::\s*net\b|\bTcpStream\b|\bUdpSocket\b|\bTcpListener\b|\breqwest\b|\bureq\b|\bcurl\s*::|\bhyper\b"),
 ("host-probing", r"\bpkg_config\b|\bcc\s*::|\bcmake\s*::|\bbindgen\b|\bvcpkg\b|\bsystem_deps\b|\bautocfg\b|\bversion_check\b|\brustc_version\b|\bwhich\s*::|\bprost_build\b|\btonic_build\b|\bcxx_build\b|\bprotoc\b"),
 ("reads-outside-manifest", r"\"(?:/[A-Za-z0-9_.~-][^\"]*|~/[^\"]*|\.\.(?:/[^\"]*)?)\""),
]
DECLARED = {"TARGET","HOST","PROFILE","OPT_LEVEL","DEBUG","NUM_JOBS","OUT_DIR","CARGO_MANIFEST_DIR"}
PREFIX = ("CARGO_PKG_","CARGO_CFG_","CARGO_FEATURE_")
ENV_READ = re.compile(r'\bvar(?:_os)?\s*\(\s*"([^"]+)"')
ALL_ENV = re.compile(r'\benv\s*::\s*vars(?:_os)?\s*\(')
SPAWN = re.compile(r'\bCommand\s*::\s*new\s*\(\s*([^)]*?)\s*\)')
RUSTC_SPELL = re.compile(r'^(?:rustc|rustc_wrapper|wrapper|rustc_workspace_wrapper|workspace_wrapper|cargo_env_var\s*\(\s*"RUSTC"\s*\)|env\s*::\s*var_os\s*\(\s*"RUSTC"\s*\)\s*\??|rustc\.next\(\)\.unwrap\(\)|[a-z_]*rustc[a-z_]*)$')
VERSION_ARGS = re.compile(r'\.arg\s*\(\s*"(?:--version|-V|-vV|--verbose)"\s*\)|\.args\s*\(\s*&?\[\s*"(?:--version|-vV)"')
COMPILE_ARGS = re.compile(r'--emit|--crate-type|--crate-name|--out-dir|probe\.rs|\.rs"')

def strip(code):
    return "\n".join(l for l in code.split("\n") if not re.match(r"^\s*//", l))

def current(code):
    for reason, pat in CUR_MARKERS:
        if re.search(pat, code): return reason
    if ALL_ENV.search(code): return "reads-all-environment"
    for m in ENV_READ.finditer(code):
        n = m.group(1)
        if n in DECLARED or n.startswith(PREFIX): continue
        return "undeclared-environment:" + n[:40]
    return None

def first_arg(code, i):
    depth = 1; j = i
    while j < len(code) and depth:
        depth += {"(": 1, ")": -1}.get(code[j], 0); j += 1
    return code[i:j-1].strip()

GUARDS = {"freebsd-version": "freebsd", "emcc": "emscripten"}
LIBS = {"version_check": "version", "rustc_version": "version", "autocfg": "compile"}

def build_deps(path):
    try: t = open(os.path.join(os.path.dirname(path), "Cargo.toml"), encoding="utf8", errors="replace").read()
    except OSError: return []
    names = []
    for m in re.finditer(r"^\[(?:[\w.\-'\"() ]+\.)?build-dependencies\]([^\[]*)", t, re.M):
        names += re.findall(r'^\s*"?([A-Za-z0-9_\-]+)"?\s*[=.]', m.group(1), re.M)
    return [n.replace("-", "_") for n in names]

def probe_of(code, deps, target_os=None):
    spawns = [first_arg(code, m.end()) for m in re.finditer(r'\bCommand\s*::\s*new\s*\(', code)]
    foreign = [s for s in spawns if not RUSTC_SPELL.match(s) and not (GUARDS.get(s.strip('"')) is not None and target_os not in (None, GUARDS[s.strip('"')]))]
    if foreign: return "foreign"
    libs = [LIBS[d] for d in deps if d in LIBS]
    comp = bool(spawns)
    if "compile" in libs or (comp and re.search(r"\bprobe\b|--emit|--crate-type|--out-dir", code)): return "compile"
    return "version" if comp or libs else "none"

def classify(code):
    cur = current(code)
    spawns = [first_arg(code, m.end()) for m in re.finditer(r'\bCommand\s*::\s*new\s*\(', code)]
    foreign = [s for s in spawns if not RUSTC_SPELL.match(s)]
    cls = "hermetic" if cur is None else None
    if cur == "spawns-process":
        if foreign: cls = "host-bound:spawns:" + foreign[0][:30]
        elif COMPILE_ARGS.search(code) and re.search(r"probe", code): cls = "toolchain-compile-probe"
        else: cls = "toolchain-version-probe"
    elif cur is not None: cls = "host-bound:" + cur
    return cur, cls

def main():
    out, extra = sys.argv[1], sys.argv[2:]
    rows = []
    for p in sorted(glob.glob(os.path.expanduser("~/.cargo/registry/src/*/*/build.rs"))):
        crate = os.path.basename(os.path.dirname(p))
        rows.append((crate, "registry", p))
    for p in extra: rows.append((os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(p))))) or p, "workspace", p))
    res = []
    for crate, origin, p in rows:
        try: code = strip(open(p, encoding="utf8", errors="replace").read())
        except OSError: continue
        cur, cls = classify(code)
        res.append({"crate": crate, "origin": origin, "current": cur, "class": cls, "probe": probe_of(code, build_deps(p), os.environ.get("TARGET_OS")), "path": p})
    json.dump(res, open(out, "w"), ensure_ascii=False, indent=0)
    c = collections.Counter(r["class"] for r in res); print(len(res), dict(c))
main()
