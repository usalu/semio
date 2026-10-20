#!/bin/zsh
# usage: ap.sh arms|apply <spec.json>: snapshots the Rust file into orig/rs/<spec name>.rs, applies the spec, rustfmt-parses and audits format captures
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/fnsplit4b-tools
mode=$1; spec=$2; name=${spec:t:r}
f=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['file'])" "$spec")
cp "$f" "$T/orig/rs/$name.rs"
if [ "$mode" = arms ]; then python3 $T/rsfn.py armsapply "$spec" | tail -1; else python3 $T/rsfn.py apply "$spec" | tail -1; fi
rustfmt --edition 2021 --emit stdout "$f" >/dev/null 2>/tmp/ap-rf.err && echo "rustfmt ok" || { echo "RUSTFMT FAIL"; head -5 /tmp/ap-rf.err; }
python3 $T/rscap.py "$f" | tail -2
bun $T/rslen.ts "$f"
