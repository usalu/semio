# HW-12 shell helpers: source this file.
R=/Users/ueli/Documents/semio
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
H="$T/🧰️scripts/HW-12"
GEN="$T/🗑️generated/HW-12"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export PATH="$H/tools:$PATH"
prev() { (cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts "$@"); }
run() { (cd "$S/📦️packages/🟦️typescript" && $B "$H/$1" "${@:2}"); }
