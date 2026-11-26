#!/bin/zsh
# 🏭️ The repair of one plant of batch MV-L1, the last step of its chain (after `../MV-1/make.sh`: rebind, the asset's own fix, depth plates, anchors), idempotent: `loft.ts --ops ops/<id>.json --write` removes the depth plates and sets the loft of every foliage mass and view (the body of its ellipsoids) that the search run (`trial.sh`) saved as absolute values; `../MV-1/anchors.ts --ops ops/<id>-anchors.json --write` then sets the anchors that run moved (when it saved any: no `ops/<id>-anchors.json`, no anchor step). It writes the catalog file and prints the digest before and after. Runs inside the reservation of its caller (`finish.sh`) or asks the queue for 1000 MB. make.sh <id>
source "${0:A:h}/env.sh"
a="$1"; f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"; mkdir -p "$GEN/log"
b="$(shasum -a 256 "$f" | cut -c1-16)"
"$W/q.sh" 1000 "$T" 3000 "$B" "$W/loft.ts" "$a" --ops "$W/ops/$a.json" --write > "$GEN/log/$a-make.log" 2>&1; echo "loft exit $? $(tail -2 "$GEN/log/$a-make.log" | cut -c1-160 | tr '\n' ';')"
[[ -f "$W/ops/$a-anchors.json" ]] && { "$W/q.sh" 1000 "$T" 3000 "$B" "$W1/anchors.ts" "$a" --ops "$W/ops/$a-anchors.json" --write > "$GEN/log/$a-make-anchors.log" 2>&1; echo "anchors exit $? $(tail -2 "$GEN/log/$a-make-anchors.log" | cut -c1-160 | tr '\n' ';')"; }
echo "$a catalog file $b -> $(shasum -a 256 "$f" | cut -c1-16)"
