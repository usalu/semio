#!/bin/bash
# Compiles one synthetic generic-heavy crate 3x with -Z threads=0 and 3x with -Z threads=1 and prints the digest of rlib+rmeta and of the dep-info per compile.
S=${1:?scratch dir}; mkdir -p "$S"; cd "$S"; RC=~/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/rustc; export RUSTC_BOOTSTRAP=1
python3 - <<'P'
lines=["#![allow(dead_code)]","use std::collections::HashMap;"]
for i in range(500):
    lines.append(f"#[derive(Debug,Clone,PartialEq,Default)] pub struct S{i}<T>{{pub a:T,pub b:Vec<u32>,pub c:Option<String>}}")
    lines.append(f"impl<T:Clone+Default> S{i}<T>{{pub fn f{i}(&self,m:&mut HashMap<u32,T>)->usize{{m.insert({i},self.a.clone());self.b.len()+{i}}}}}")
    lines.append(f"pub fn g{i}()->usize{{let s=S{i}::<u8>::default();let mut m=HashMap::new();s.f{i}(&mut m)+m.len()}}")
lines.append("pub trait Tr{fn t(&self)->u32;}")
for i in range(200): lines.append(f"impl Tr for S{i}<u8>{{fn t(&self)->u32{{{i}}}}}")
open("lib.rs","w").write("\n".join(lines))
P
i=0; for t in 0 1 0 1 0 1; do i=$((i+1)); d=out-$t-$i; mkdir -p $d; nice -n 10 $RC --crate-name tiny --edition 2021 --crate-type lib -Z threads=0 -Z threads=$t -C metadata=m1 -C extra-filename=-x --emit=dep-info,metadata,link --out-dir $d --remap-path-prefix=$S=/r lib.rs; done
for d in out-*; do echo $d $(cat $d/*.rlib $d/*.rmeta | shasum | cut -c1-16) $(sed "s#$d##g" $d/*.d | shasum | cut -c1-8); done
