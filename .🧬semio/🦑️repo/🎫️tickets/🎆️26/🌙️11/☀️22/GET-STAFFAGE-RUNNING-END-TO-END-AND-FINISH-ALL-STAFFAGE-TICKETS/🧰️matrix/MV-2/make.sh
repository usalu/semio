#!/bin/zsh
# 🏗️ The multiview repair of one plant of MV-2 as the last step of its chain (after its generator and the silhouette ops of SV-2 or R4-P), idempotent byte for byte: the chain runs on a copy of the catalog file (`$GEN/make/<id>.json`) and the catalog file is written once, at the end, only when the result differs from it and it did not change meanwhile. Steps on the copy: the plant's own step `make-<id>.sh` when it has one, the ops `ops/<id>.json` (apply.ts), then, for the plants listed in `carried.txt`, the depth plates of `carry.ts` (with the flags of `carry-flags/<id>.txt` when the plant has such a file), then the ops again (anchors and shapes that carry.ts renamed, plates the ops remove). The line it prints ends with the SHA-256 of the catalog file before and after. make.sh <id>
BATCH=MV-2 source "${0:A:h}/../MR/env.sh"
a="$1"; src="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"; mkdir -p "$GEN/make"; tmp="$GEN/make/$a.json"; seen="$GEN/make/$a.seen.json"
cp "$src" "$tmp"; cp "$src" "$seen"; before="$(shasum -a 256 "$src" | cut -c1-64)"
[[ -x "$W/make-$a.sh" ]] && "$W/make-$a.sh" --in "$tmp" --out "$tmp"
cd "$T"
[[ -f "$W/ops/$a.json" ]] && "$B" "$W/apply.ts" "$a" --in "$tmp" --out "$tmp" | tail -1
if grep -qx "$a" "$W/carried.txt"; then
  timeout 3000 "$B" "$W/carry.ts" "$tmp" --out "$tmp" $(cat "$W/carry-flags/$a.txt" 2>/dev/null) | grep "plates with" | cut -c1-200
  [[ -f "$W/ops/$a.json" ]] && "$B" "$W/apply.ts" "$a" --in "$tmp" --out "$tmp" | tail -1
fi
if cmp -s "$tmp" "$src"; then echo "$a: unchanged, sha256 $before"
elif ! cmp -s "$src" "$seen"; then echo "$a: the catalog file changed while the chain ran: nothing written, run again"; rm -f "$tmp" "$seen"; exit 1
else cp "$tmp" "$src"; echo "$a: written, sha256 $before -> $(shasum -a 256 "$src" | cut -c1-64)"
fi
rm -f "$tmp" "$seen"
