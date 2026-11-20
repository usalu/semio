#!/bin/zsh
# usage: batch.sh <tag> <crates-tsv>; one workspace build testing every listed package, log in 🗑️generated/<tag>.log, per-package summary in <tag>-packages.tsv
F="/Users/ueli/Documents/semio/🧰️framework"; G="${0:A:h}/🗑️generated"; tag=$1; cd "$F"
args=(); while IFS=$'\t' read -r name m; do args+=(-p "$name"); done < "$2"
timeout 14400 repo cargo --store=isolated --configuration=prototype -- test --offline --no-fail-fast --manifest-path "$F/Cargo.toml" "${args[@]}" > "$G/$tag.log" 2>&1
echo "exit $?" >> "$G/$tag.log"
awk '/Running /{match($0,/\/[a-z0-9_]+-[0-9a-f]+\)$/);p=substr($0,RSTART+1,RLENGTH-2);sub(/-[0-9a-f]+$/,"",p)} /Doc-tests /{p="doc:"$2} /^test result/{print p"\t"$4"\t"$6}' "$G/$tag.log" > "$G/$tag-packages.tsv"
