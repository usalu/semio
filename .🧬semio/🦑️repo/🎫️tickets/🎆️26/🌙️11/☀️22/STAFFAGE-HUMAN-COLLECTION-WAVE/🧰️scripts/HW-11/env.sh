# HW-11 shell helpers: source this file.
R=/Users/ueli/Documents/semio
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
H="$T/🧰️scripts/HW-11"
GEN="$T/🗑️generated/HW-11"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export PATH="$H/tools:$PATH"
prev() { (cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts "$@"); }
# look <slug> <pose> [view] [dpi]: multiview sheets at 1:50, 1:100, 1:200, 1:500 into $GEN/<slug>/<pose>
look() { local o="$GEN/$1/$2-${3:-light-solid}"; mkdir -p "$o"; for sc in 50 100 200 500; do prev --asset "$1" --scale $sc --pose "$2" --view "${3:-light-solid}" --views --projection third --flow none --out "$o" --dpi "${4:-150}" >/dev/null 2>&1 || echo "[DEBUG] fail $sc"; done; ls "$o"/*.png; }
