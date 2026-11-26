# 🧭️ Paths of lane MX-3 (source it): the MR paths with BATCH=MX-3, plus the package folder P and asset <id>.
export BATCH=MX-3; source "${0:A:h}/../MR/env.sh"
export P="$S/📦️packages/🟦️typescript"
export L="$M/../ML-4"
asset() { ls "$S/🗂️catalog"/*/"$1"/🔣️.json; }
