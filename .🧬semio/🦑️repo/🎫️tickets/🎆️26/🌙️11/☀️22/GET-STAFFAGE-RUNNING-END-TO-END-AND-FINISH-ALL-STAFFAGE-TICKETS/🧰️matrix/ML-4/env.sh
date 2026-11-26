# 🧭️ Paths of lane ML-4 (source it): the MR paths with BATCH=ML-4, plus asset <id>.
export BATCH=ML-4; source "${0:A:h}/../MR/env.sh"
export P="$S/📦️packages/🟦️typescript"
asset() { ls "$S/🗂️catalog"/*/"$1"/🔣️.json; }
