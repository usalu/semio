#!/bin/bash
# usage: warm.sh <reps> <variant...>; interleaved warm rebuilds of the test crate, one row per run: variant user sys real rssMB total_s link_s codegen_s
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H90
R=$1; shift
cd $S/meas; export RUSTC_BOOTSTRAP=1
TC=~/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin; export PATH=$TC:$PATH RUSTC=$TC/rustc
for i in $(seq 1 $R); do for v in "$@"; do
  export CARGO_TARGET_DIR=$S/t-$v CARGO_PROFILE_DEV_OPT_LEVEL=0 CARGO_PROFILE_DEV_CODEGEN_UNITS=256 CARGO_PROFILE_DEV_INCREMENTAL=false
  case $v in A*) export CARGO_PROFILE_DEV_DEBUG=1;; B*) export CARGO_PROFILE_DEV_DEBUG=line-tables-only;; C*) export CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off;; D*) export CARGO_PROFILE_DEV_DEBUG=2;; E*) export CARGO_PROFILE_DEV_DEBUG=0;; esac
  echo "// $v $i" >> tests/big.rs
  /usr/bin/time -l cargo rustc --offline --test big -- -Z time-passes > $S/w.out 2> $S/w.err; cat $S/w.err >> $S/w.out
  python3 - "$v" $S/w.out <<'P'
import re,sys
t=open(sys.argv[2],errors="replace").read()
g=lambda p:(re.search(p,t) or [None,"nan"])[1]
print(sys.argv[1],"user",g(r"([\d.]+) user"),"sys",g(r"([\d.]+) sys"),"real",g(r"([\d.]+) real"),"rssMB",round(int(g(r"(\d+)\s+maximum resident"))/1048576),"total",g(r"time:\s+([\d.]+); rss:\s+\d+MB ->\s+\d+MB \(\s*[+-]\d+MB\)\ttotal"),"link",g(r"time:\s+([\d.]+); rss:[^\n]*\tlink_binary\n"),"llvm",g(r"time:\s+([\d.]+); rss:[^\n]*\tLLVM_passes\n"))
P
  sed -i '' '$d' tests/big.rs
done; done
