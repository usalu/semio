#!/bin/zsh
# ⏭️ Runs proof steps under their own grant of the memory queue once the repairs are written: it starts steps.sh only when every given asset has its second apply log saying "file unchanged" (🗑️generated/matrix/ML-1/apply/<id>.again.txt), else it gives the grant back at once with exit 3. The summary goes to 🗑️generated/matrix/ML-1/proof/summary-<tag>.txt: after.sh <tag> <MB> <what,…> "<ids that must be written>" <id>…
source "${0:A:h}/env.sh"
tag="$1"; mb="$2"; what="$3"; need="$4"; shift 4; mkdir -p "$GEN/proof"
guard="for a in $need; do grep -q 'file unchanged' '$GEN/apply/'\$a.again.txt 2>/dev/null || { echo \"not written yet: \$a\"; exit 3; }; done"
qs "$mb" ${TO:-28800} "$guard; zsh '$W/steps.sh' '$what' $*" 2>&1 | grep -v '^\[capacity\]' > "$GEN/proof/summary-$tag.txt"
