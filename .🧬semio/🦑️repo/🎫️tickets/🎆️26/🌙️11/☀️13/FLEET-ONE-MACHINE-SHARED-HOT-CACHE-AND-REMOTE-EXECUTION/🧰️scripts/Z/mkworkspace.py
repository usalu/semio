#!/usr/bin/env python3
"""mkworkspace.py <dir> <salt> [--iterations N] [--layers 4] [--width 6]: writes a standalone cargo workspace of layers*width small library crates plus one top crate, outside any repository. Every crate burns about the same compile time in a const loop; the salt is part of every source so a run never meets the results of an earlier run in a cache. Same arguments, same bytes."""
import os,sys,argparse
ap=argparse.ArgumentParser();ap.add_argument('dir');ap.add_argument('salt');ap.add_argument('--iterations',type=int,default=700_000);ap.add_argument('--layers',type=int,default=4);ap.add_argument('--width',type=int,default=6)
a=ap.parse_args()
os.makedirs(a.dir,exist_ok=True)
names=[[f"pz{l}{j:02d}" for j in range(a.width)] for l in range(a.layers)]
members=[n for layer in names for n in layer]+["pztop"]
def w(path,text):
    os.makedirs(os.path.dirname(path),exist_ok=True)
    old=open(path).read() if os.path.exists(path) else None
    if old!=text:open(path,'w').write(text)
w(f"{a.dir}/Cargo.toml",'cargo-features = ["trim-paths"]\n\n[workspace]\nresolver = "2"\nmembers = ['+', '.join(f'"{m}"' for m in members)+']\n\n[profile.dev]\ntrim-paths = "object"\nincremental = false\ndebug = 1\n')
w(f"{a.dir}/rust-toolchain.toml",'[toolchain]\nchannel = "nightly-2026-07-20"\n')
w(f"{a.dir}/.cargo/config.toml",'[build]\nrustflags = ["-Z", "threads=0"]\n\n[unstable]\nembed-metadata = "DoNotEmbed"\nchecksum-freshness = true\n')
def crate(name,deps,seed):
    use="".join(f"use {d};\n" for d in [])
    calls=" ^ ".join([f"{d}::value()" for d in deps]) if deps else "0"
    src=f'''#![allow(long_running_const_eval)]
pub const SALT: &str = "{a.salt}";
pub const ACC: u64 = {{
    let mut i = 0u64;
    let mut s = {seed}u64;
    while i < {a.iterations} {{
        s = s.wrapping_mul(6364136223846793005).wrapping_add(i ^ (s >> 29));
        i += 1;
    }}
    s
}};
pub fn value() -> u64 {{
    ACC ^ {calls}
}}
pub fn salt() -> &'static str {{
    SALT
}}
'''
    toml=f'[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\npath = "src/lib.rs"\n\n[dependencies]\n'+"".join(f'{d} = {{ path = "../{d}" }}\n' for d in deps)
    w(f"{a.dir}/{name}/Cargo.toml",toml);w(f"{a.dir}/{name}/src/lib.rs",src)
seed=1
for l,layer in enumerate(names):
    for j,n in enumerate(layer):
        deps=[] if l==0 else [names[l-1][j],names[l-1][(j+1)%a.width]]
        crate(n,deps,seed);seed+=7
crate("pztop",names[-1],seed)
print(len(members),"crates in",a.dir)
