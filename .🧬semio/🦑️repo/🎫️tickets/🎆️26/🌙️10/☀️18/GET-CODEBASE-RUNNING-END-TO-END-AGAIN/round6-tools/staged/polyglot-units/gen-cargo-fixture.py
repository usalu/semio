#!/usr/bin/env python3
"""Builds a four-crate path-only workspace under /private/tmp/polyglot-units/lab/cargo-ws, runs `cargo metadata --offline` and `cargo tree` on it (third-party oracles) and writes the fixture staged/🧩️units/🧪️tests/🧫️fixtures/cargo-metadata.json (metadata + `_files` + `_tree`). One cargo process at a time, nice 10."""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
WS = "/private/tmp/polyglot-units/lab/cargo-ws"
OUT = os.path.join(HERE, "staged", "🧩️units", "🧪️tests", "🧫️fixtures", "cargo-metadata.json")
FILES = {
    "Cargo.toml": '[workspace]\nmembers = ["core", "mac", "app", "tool"]\nresolver = "2"\n',
    "core/Cargo.toml": '[package]\nname = "core-lib"\nversion = "0.1.0"\nedition = "2021"\n',
    "core/src/lib.rs": "pub fn one() -> u32 { 1 }\n",
    "mac/Cargo.toml": '[package]\nname = "mac"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\nproc-macro = true\n',
    "mac/src/lib.rs": "extern crate proc_macro;\nuse proc_macro::TokenStream;\n#[proc_macro]\npub fn two(_: TokenStream) -> TokenStream { \"2u32\".parse().unwrap() }\n",
    "app/Cargo.toml": '[package]\nname = "app"\nversion = "0.2.0"\nedition = "2021"\n\n[dependencies]\ncore-lib = { path = "../core" }\nmac = { path = "../mac" }\n\n[dev-dependencies]\n\n[[bin]]\nname = "app-cli"\npath = "src/main.rs"\n',
    "app/src/lib.rs": "pub fn total() -> u32 { core_lib::one() + mac::two!() }\n",
    "app/src/main.rs": "fn main() { println!(\"{}\", app::total()); }\n",
    "tool/Cargo.toml": '[package]\nname = "tool"\nversion = "0.3.0"\nedition = "2021"\n\n[dependencies]\napp = { path = "../app" }\n',
    "tool/src/main.rs": "fn main() { println!(\"{}\", app::total()); }\n",
}
for path, text in FILES.items():
    target = os.path.join(WS, path)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    open(target, "w").write(text)
env = {**os.environ, "CARGO_NET_OFFLINE": "true"}
run = lambda args: subprocess.run(["nice", "-n", "10", "cargo", *args], cwd=WS, env=env, capture_output=True, text=True, check=True).stdout
metadata = json.loads(run(["metadata", "--offline", "--format-version", "1"]))
tree = run(["tree", "--offline", "--workspace", "-e", "normal,build", "--prefix", "depth", "--depth", "1", "-f", "{p}"])
edges, current = set(), None
for line in tree.splitlines():
    if not line.strip() or line.startswith("["):
        continue
    depth = int("".join(ch for ch in line[:3] if ch.isdigit()) or "0")
    name = line.lstrip("0123456789").split(" ")[0]
    if depth == 0:
        current = name
    elif current:
        edges.add("%s->%s" % (current, name))
metadata["_files"] = sorted(FILES)
metadata["_tree"] = sorted(edges)
os.makedirs(os.path.dirname(OUT), exist_ok=True)
json.dump(metadata, open(OUT, "w"), ensure_ascii=False, indent=1)
print("wrote", OUT, "packages", len(metadata["packages"]), "edges", sorted(edges))
