#!/bin/zsh
# 🏭️ The trial copy of one asset of lane CL-T4: `facet.ts` with `ops/<id>.json` (or `$OPS/<id>.json`) into `$GEN/try/<id>.json` (or `$GEN/$OUT/<id>.json`); the digest of the catalog file of this moment goes to `<id>.base` beside it. The source is the catalog file; when the catalog file still is the copy this lane wrote last (`$GEN/written/<id>.sha`), the source is the file as it was before that write (`$GEN/backup/<id>.json`), so a second rewrite starts from the authored solids again and not from the lobes of the first. The ops a copy was made with are kept in `$GEN/applied/<id>.json`. make.sh <id>…
source "${0:A:h}/env.sh"
o="$GEN/${OUT:-try}"; mkdir -p "$o" "$GEN/backup" "$GEN/written"
for a in "$@"; do
f="$(ls "$C"/*/"$a"/🔣️.json)"; now="$(shasum -a 256 < "$f" | cut -c1-16)"; src="$f"
if [[ -f "$GEN/written/$a.sha" && "$now" == "$(cat "$GEN/written/$a.sha")" ]]; then src="$GEN/backup/$a.json"; else cp "$f" "$GEN/backup/$a.json"; fi
echo "$now" > "$o/$a.base"
mkdir -p "$GEN/applied"; cp "${OPS:-$W/ops}/$a.json" "$GEN/applied/$a.json"
zsh "$W/run.sh" facet.ts "$src" --ops "$GEN/applied/$a.json" --out "$o/$a.json" && echo "$a: $(wc -c < "$f" | tr -d ' ') -> $(wc -c < "$o/$a.json" | tr -d ' ') bytes (source ${src:t})"
done
