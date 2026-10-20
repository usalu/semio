#!/usr/bin/env python3
"""Static oracle over the STAGED state of the Rust groups (no compiler): builds a copy-on-write overlay of the repository with the staged files and checks
 1. external-library tokens in the source closure of each touched crate (comments excluded);
 2. every `wasm::...` path a staged file uses is exported by the engine port;
 3. manifest hygiene: which crates still declare wasmtime / tokio / serde_json and in which section;
 4. no dependency cycle through the new engine edges.
usage: oracle.py [--keep]   (exit 1 on any violation)"""
import os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
OVL = "/private/tmp/iface-wrap/oracle-ovl"
sys.path.insert(0, HERE)
from closure import closure
subprocess.run([sys.executable, os.path.join(HERE, "mkovl.py"), OVL, "--prefix", "🧰️framework/"], check=True, capture_output=True)
OS = "🧰️framework/🛍️products/💻️os/🔨️modules/"
H = OVL + "/" + OS + "🔌️plugin/🖥️host/"
MCP = OVL + "/" + OS + "🌉️mcp/"
SYNC = OVL + "/" + OS + "🏪️store/🔄️sync/"
violations = []

def code(path):
    text = open(path, encoding="utf8").read()
    return "\n".join(line for line in text.splitlines() if not re.match(r"\s*//", line))

def tokens(root, pattern, test_ok=True):
    rx = re.compile(pattern)
    hits = []
    for f in closure(root):
        is_test = "🧪️tests" in f or f.endswith("guest-input-slot.rs")
        if test_ok and is_test:
            continue
        lines = code(f).splitlines()
        cut = next((i for i, line in enumerate(lines) if re.match(r"\s*(#\[test\]|#\[cfg\(test\)\]\s*$)", line) and any(re.match(r"\s*(mod |fn |#\[test\]|async fn |pub mod )", lines[j]) for j in range(i, min(i + 3, len(lines))))), len(lines)) if test_ok else len(lines)
        for number, line in enumerate(lines[:cut], 1):
            if rx.search(line):
                hits.append((f.split("/modules/")[-1][-90:], line.strip()[:110]))
    return hits

def check(name, hits):
    status = "ok" if not hits else "VIOLATION"
    print("%-9s %s" % (status, name))
    for hit in hits[:6]:
        print("          ", hit)
    if hits:
        violations.append(name)

check("plugin-runtime production closure names no tokio / serde_json", tokens(MCP + "⚙️plugin-runtime/🦀️.rs", r"\b(tokio|serde_json)\b"))
check("plugin-host-imports closure names no serde / serde_json / wasmtime", tokens(H + "📥️imports/🦀️.rs", r"\b(serde|serde_json|wasmtime|wasmtime_wasi)\b"))
check("plugin-host-operation closure names no wasmtime crate path", tokens(H + "🚪️io/🎛️operation/🦀️.rs", r"(?<![\w:])(?<!mod )wasmtime(_wasi)?::(?!\{OperationHostStore)"))
check("plugin-host-guest-runtime closure names no wasmtime / serde_json (production)", tokens(H + "🎭️guest-runtime/🦀️.rs", r"(?<![\w:\"-])(wasmtime(_wasi)?::|serde_json)"))
check("plugin-host-wit closure names no wasmtime path outside the bindgen port call", [h for h in tokens(H + "🔗️wit-bindings/🦀️.rs", r"(?<![\w:\"-])wasmtime(_wasi)?::")])

port_text = code(H + "🔧️engine/🔌️wasm-port/🦀️.rs")
exported = set()
def flatten(prefix, body):
    for m in re.finditer(r"pub use (wasmtime(?:_wasi)?)::((?:\w+::)*)\{([^}]*)\};|pub use (wasmtime(?:_wasi)?)::((?:\w+::)*\w+);", body):
        names = m.group(3).split(",") if m.group(3) else [m.group(5).split("::")[-1]]
        for n in names:
            n = n.strip()
            if n: exported.add(prefix + (n,))
flatten((), re.sub(r"pub mod \w+ \{.*?\n\}\n", "", port_text, flags=re.S))
for module in re.finditer(r"pub mod (\w+) \{(.*?)\n\}\n", port_text, flags=re.S):
    top = module.group(1)
    body = module.group(2)
    nested = re.search(r"pub mod p2 \{(.*)\}", body, flags=re.S)
    flatten((top,), re.sub(r"pub mod p2 \{.*\}", "", body, flags=re.S))
    if nested:
        p2 = nested.group(1)
        for m in re.finditer(r"pub use wasmtime_wasi::p2::(\w+);", p2): exported.add((top, "p2", m.group(1)))
        for m in re.finditer(r"pub use wasmtime_wasi::p2::pipe::(\w+);", p2): exported.add((top, "p2", "pipe", m.group(1)))
exported.add(("generated_code_support",))
used = set()
for crate in ("📥️imports", "🎭️guest-runtime", "🚪️io/🎛️operation", "🔗️wit-bindings"):
    for f in closure(H + crate + "/🦀️.rs"):
        for m in re.finditer(r"(?<![\w:])wasm((?:::\w+)+)", code(f)):
            used.add((tuple(m.group(1).split("::")[1:]), f.split("/modules/")[-1][-60:]))
unresolved = []
for path, f in sorted(used):
    ok = any(path[: len(e)] == e and len(path) - len(e) <= 1 for e in exported) or any(e[: len(path)] == path for e in exported)
    if not ok: unresolved.append((f, "wasm::" + "::".join(path)))
print("port exports:", sorted("::".join(e) for e in exported))
check("every wasm:: path used by a host layer is exported by the port", unresolved)

def sections(cargo):
    out, section = [], None
    for number, line in enumerate(open(cargo, encoding="utf8").read().splitlines(), 1):
        m = re.match(r"\[(.+)\]", line.strip())
        if m: section = m.group(1)
        elif re.match(r"(wasmtime|wasmtime-wasi|tokio|serde_json|serde)\s*=", line.strip()): out.append((section, line.split("=")[0].strip()))
    return out
crates = {
    "plugin-runtime": MCP + "⚙️plugin-runtime/📦️packages/🦀️rust/Cargo.toml",
    "host-engine": H + "🔧️engine/📦️packages/🦀️rust/Cargo.toml",
    "host-imports": H + "📥️imports/📦️packages/🦀️rust/Cargo.toml",
    "host-wit": H + "🔗️wit-bindings/📦️packages/🦀️rust/Cargo.toml",
    "host-guest-runtime": H + "🎭️guest-runtime/📦️packages/🦀️rust/Cargo.toml",
    "host-operation": H + "🚪️io/🎛️operation/📦️packages/🦀️rust/Cargo.toml",
}
expect = {
    "plugin-runtime": [("dev-dependencies", "serde_json")],
    "host-engine": [("dependencies", "wasmtime"), ("dependencies", "wasmtime-wasi"), ("dev-dependencies", "serde_json")],
    "host-imports": [],
    "host-wit": [("dependencies", "serde"), ("dependencies", "serde_json")],
    "host-guest-runtime": [("dev-dependencies", "serde_json")],
    "host-operation": [("dev-dependencies", "serde_json")],
}
for name, cargo in crates.items():
    found = sorted(sections(cargo))
    check("manifest %s declares exactly %s" % (name, expect[name]), [] if found == sorted(expect[name]) else [(name, found)])

def deps(cargo):
    base = os.path.dirname(cargo)
    text = open(cargo, encoding="utf8").read()
    pkg = re.search(r'^name\s*=\s*"([^"]+)"', text, re.M).group(1)
    edges = []
    for m in re.finditer(r'path\s*=\s*"([^"]+)"', text):
        target = os.path.normpath(os.path.join(base, m.group(1), "Cargo.toml"))
        if os.path.isfile(target): edges.append(target)
    return pkg, edges
cache = {}
def graph(cargo, seen):
    real = os.path.realpath(cargo)
    if real in cache: return cache[real]
    pkg, edges = deps(cargo)
    cache[real] = (pkg, edges)
    return cache[real]
def reaches(start, goal_names):
    seen, stack, found = set(), [start], set()
    while stack:
        cur = stack.pop()
        real = os.path.realpath(cur)
        if real in seen or not os.path.isfile(cur): continue
        seen.add(real)
        pkg, edges = graph(cur, seen)
        if pkg in goal_names: found.add(pkg)
        stack.extend(edges)
    return found
engine = crates["host-engine"]
cyc = reaches(engine, {"semio-framework-plugin-host-wit", "semio-framework-plugin-host-operation", "semio-framework-plugin-host-imports", "semio-framework-plugin-host-guest-runtime"})
check("engine reaches none of wit / operation / imports / guest-runtime through path dependencies (no cycle from the new edges)", [(c, "reachable from engine") for c in sorted(cyc)])
print("VIOLATIONS" if violations else "ORACLE OK", violations)
sys.exit(1 if violations else 0)
