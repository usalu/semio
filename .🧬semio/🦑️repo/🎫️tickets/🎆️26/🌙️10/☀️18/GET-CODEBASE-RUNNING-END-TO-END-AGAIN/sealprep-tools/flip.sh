#!/bin/sh
# 🏛️ [sealprep] The sealed flip as ONE sequence. usage: sh flip.sh check|apply|revert [ROOT]   (ROOT defaults to the live repository this script lives in; a mirror root takes only the product batches: `chainlaw`, `flip`)
# check : every batch row must say base (or applied); exit 1 otherwise (then: python3 -I make.py --root ROOT rebuilds the three batches from the current files).
# apply : mirror default (mirror_seal.py), post-flip.sh copy (audit11), chainlaw, flip, then post-flip.sh ROOT; stops at the first failure; prints POSTFLIP exit N.
# revert: flip back to live, chainlaw back, post-flip.sh back, mirror default back; the snapshots in .🧬semio/🦑️repo/⚡️cache/implementation stay and are harmless.
H="$(cd "$(dirname "$0")" && pwd)"; C="$(dirname "$H")"; verb="${1:-check}"; ROOT="${2:-$(cd "$C/../../../../../../.." && pwd)}"; live=0; [ -d "$ROOT/${C#$ROOT/}" ] && live=1
st(){ python3 -I "$H/staged/$1/stage.py" "$2" --root "$ROOT"; }
case "$verb" in
 check) rc=0; for b in chainlaw flip; do st $b check || rc=1; done; [ "$live" = 1 ] && { st tools check || rc=1; python3 -I "$H/mirror_seal.py" status; }; exit $rc;;
 apply) for b in chainlaw flip; do st $b check >/dev/null || { echo "batch $b drifted: python3 -I $H/make.py --root $ROOT"; exit 2; }; done
  if [ "$live" = 1 ]; then st tools check >/dev/null || { echo "batch tools drifted: python3 -I $H/make.py --root $ROOT"; exit 2; }; python3 -I "$H/mirror_seal.py" apply; rc=$?; [ "$rc" = 0 ] || [ "$rc" = 3 ] || exit 2; st tools apply || exit 2; fi
  st chainlaw apply || exit 2; st flip apply || exit 2
  if [ "$live" = 1 ]; then sh "$C/self-g-tools/post-flip.sh" "$ROOT"; else sh "$H/staged/flip/verify.sh" "$ROOT"; fi;;
 revert) st flip revert; st chainlaw revert; if [ "$live" = 1 ]; then st tools revert; python3 -I "$H/mirror_seal.py" revert; fi; true;;
 *) echo "usage: sh flip.sh check|apply|revert [ROOT]"; exit 2;;
esac
