#!/bin/zsh
# 🏭️ The repair of one tree of round MV-L3, the last step of its chain (after `../MV-1/make.sh <id>`, whose plates it replaces), idempotent, inside the grant of its caller. The method stands in `flags/<id>.method`: `body` runs `../MV-3/body.ts <id> --ops body/<id>.json --write` (one rounded body per foliage part in the place of its depth plates) and `../MV-3/anchors.ts <id> --ops anchors/<id>.json --write` when that file exists (anchors on the drawing of their bone); `loft` runs `../MV-L1/loft.ts <id> --ops loft/<id>.json --write` (the foliage masses named in `loft/<id>.json` as the lofts of their ellipsoids) and then the `body` steps for what is left; `mu` (an asset of lane MU) makes `ops/<id>.json` of this folder the spec `../MU/ops/<id>.json` of that lane's chain and runs `../MU/fix.ts <id> --write`. It prints the digest of the catalog file before and after. make.sh <id>
source "${0:A:h}/env.sh"
a="$1"; f="$(ls "$C"/*/"$a"/🔣️.json)"; mkdir -p "$GEN/log"; b="$(shasum -a 256 "$f" | cut -c1-16)"; cd "$T"
case "$(cat "$W/flags/$a.method")" in
  body)
    timeout 1800 "$B" "$W3/body.ts" "$a" --ops "$W/body/$a.json" --write > "$GEN/log/$a-make-body.log" 2>&1; echo "body exit $? $(tail -2 "$GEN/log/$a-make-body.log" | cut -c1-160 | tr '\n' ';')"
    [[ -f "$W/anchors/$a.json" ]] && { timeout 3000 "$B" "$W3/anchors.ts" "$a" --ops "$W/anchors/$a.json" --write > "$GEN/log/$a-make-anchors.log" 2>&1; echo "anchors exit $? $(tail -1 "$GEN/log/$a-make-anchors.log" | cut -c1-160)"; } ;;
  loft)
    timeout 3000 "$B" "$U/🧰️matrix/MV-L1/loft.ts" "$a" --ops "$W/loft/$a.json" --write > "$GEN/log/$a-make-loft.log" 2>&1; echo "loft exit $? $(tail -2 "$GEN/log/$a-make-loft.log" | cut -c1-160 | tr '\n' ';')"
    timeout 1800 "$B" "$W3/body.ts" "$a" --ops "$W/body/$a.json" --write > "$GEN/log/$a-make-body.log" 2>&1; echo "body exit $? $(tail -2 "$GEN/log/$a-make-body.log" | cut -c1-160 | tr '\n' ';')"
    [[ -f "$W/anchors/$a.json" ]] && { timeout 3000 "$B" "$W3/anchors.ts" "$a" --ops "$W/anchors/$a.json" --write > "$GEN/log/$a-make-anchors.log" 2>&1; echo "anchors exit $? $(tail -1 "$GEN/log/$a-make-anchors.log" | cut -c1-160)"; } ;;
  mu)
    cmp -s "$W/ops/$a.json" "$U/🧰️matrix/MU/ops/$a.json" || { cp "$U/🧰️matrix/MU/ops/$a.json" "$GEN/backup/$a.mu-ops.json"; cp "$W/ops/$a.json" "$U/🧰️matrix/MU/ops/$a.json"; echo "spec of lane MU set to ops/$a.json of this round"; }
    timeout 1800 "$B" "$U/🧰️matrix/MU/fix.ts" "$a" --write --quiet --no-scan > "$GEN/log/$a-make-mu.log" 2>&1; echo "fix exit $? $(tail -1 "$GEN/log/$a-make-mu.log" | cut -c1-160)" ;;
  *) echo "$a: no method in flags/$a.method, nothing written"; exit 1 ;;
esac
echo "$a catalog file $b -> $(shasum -a 256 "$f" | cut -c1-16) ($(wc -c < "$f" | tr -d ' ') bytes)"
