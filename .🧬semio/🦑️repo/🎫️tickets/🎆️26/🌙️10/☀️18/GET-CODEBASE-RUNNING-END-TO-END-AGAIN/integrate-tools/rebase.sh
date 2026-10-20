#!/usr/bin/env bash
# 🔁 usage: rebase.sh <set-prefix> [<set-prefix> ...]
# Re-stages one or more finished sets that changed after the series was built (or that finished later) against the CURRENT live tree and builds a new anchored series from them:
#   1. snapshot of the live library + the manifests/process tree the sets touch (copies, /private/tmp/ig/base-<stamp>),
#   2. runset.py per prefix against that snapshot (each set's own anchored apply tool, on a copy),
#   3. finalize.py: merge (3-way, hand resolutions in integrate-tools/resolutions are reused only when their conflict digest is unchanged), semantic patches, regenerated help snapshots,
#   4. prints the apply command: apply-all.sh --series final-<stamp> [--yes] (dry run by default, refuses on drift, undo with --undo <group>).
# A conflict stops the run: read /private/tmp/ig/merged-<group>/_conflicts/*.token, write the choices and seal them with seal.py (see result-integrate.md), then run rebase.sh again.
# Sets known to runset.py: cache-fix rcache rexec bworkers bzl-coverage bzl-rust bzl-xplat zt-e2e-fix tax6 vit4 ingress-perf uncached-owners bzl-lazy graph-perf
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../../../../../../../.." && pwd)"
[ $# -ge 1 ] || { sed -n 2,12p "$0"; exit 2; }
stamp="$(date +%m%d%H%M)"; base="/private/tmp/ig/base-$stamp"; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
mkdir -p "$base" && cd "$root"
rsync -a --exclude node_modules --exclude .git "$L/" "$base/$L/"
rsync -a --relative --files-from="$here/extras2.txt" ./ "$base/"
rsync -a --exclude node_modules --exclude target --relative "./🧰️framework/🔨️modules/🏃️process/" "$base/"
applied="cache-fix,rcache,rexec,bworkers,bzl-coverage,bzl-rust,bzl-xplat,zt-e2e-fix,tax6,vit4,ingress-perf,uncached-owners,bzl-lazy,graph-perf"
groups="["; sep=""
for set in "$@"; do
  python3 "$here/runset.py" "$set" --base "$base" --out "/private/tmp/ig/r-$stamp-$set"
  groups="$groups$sep{\"id\":\"R-$set\",\"title\":\"rebase of $set\",\"sets\":[\"$set\"]}"; sep=","
  applied="$(echo ",$applied," | sed "s/,$set,/,/;s/^,//;s/,$//")"
done
echo "$groups]" > "$here/groups-$stamp.json"
python3 "$here/finalize.py" --groups "$here/groups-$stamp.json" --out "$here/final-$stamp" --base "$base" --orig "$base" --prefix "r-$stamp-" --applied "$applied"
echo "series ready: $here/apply-all.sh --series final-$stamp          (add --yes to apply, --group R-<set> for one set)"
