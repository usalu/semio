#!/bin/zsh
# 🧩️ The whole repair of one asset: apply.sh <source.json> <ops.json> [<out.json>] runs fit.ts with the ops and, when `<ops without .json>.lofts.json` exists, the loft step of MX-2 (`../MX-2/loft.ts`, D54) after it. Without <out.json> it is a dry run on a copy: both steps must report that nothing changes.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
src="$1"; ops="$2"; lofts="${2%.json}.lofts.json"; out="${3:-$GEN/try/.dry.$$.json}"; mkdir -p "$GEN/try"
cd "$T"
"$W/b.sh" "$W/fit.ts" "$src" --ops "$ops" --out "$out"; code=$?
[[ -f "$lofts" ]] && { "$W/b.sh" "$U/🧰️matrix/MX-2/loft.ts" "$out" --ops "$lofts" --out "$out" || code=1; }
[[ -n "$3" ]] || rm -f "$out"
exit $code
