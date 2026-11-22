B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
H="$T/🧰️scripts/HW-18"
G="$T/🗑️generated/HW-18"
C="$S/🗂️catalog/👤️humans"
prev() { a=$1; shift; out=$1; shift; (cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset $a --no-png --out "$out" "$@" >/dev/null 2>&1); $B "$H/png.ts" ${DENS:-150} "$out"/*.svg; }
