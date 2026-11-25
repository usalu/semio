# 🧭️ Paths of lane VOL (source it): the MR paths plus T14 (the r3 tools of the 3D ticket), S4 (the prepared volume scripts), IDS (the 27 ids), asset <id>.
export BATCH=VOL; source "${0:A:h}/../MR/env.sh"
export T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
export S4="$U/🧰️linux/s4"
IDS=($(cat "$S4/assets.txt"))
asset() { ls "$S/🗂️catalog"/*/"$1"/🔣️.json; }
