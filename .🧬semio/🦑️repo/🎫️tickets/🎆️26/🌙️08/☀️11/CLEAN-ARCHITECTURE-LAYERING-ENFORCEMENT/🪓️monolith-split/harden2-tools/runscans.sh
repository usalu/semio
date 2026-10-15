#!/bin/zsh
cd "${0:A:h}"
R=/Users/ueli/Documents/semio
E5="$R/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
E2="$R/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
T="$R/🧰️framework/🔨️modules/🗺️surface"
C="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli"
G="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust"
HB="$R/🌎️hub"
O=${HO_WORK:-/tmp}
F5="✏️editor/(🧩️entities|🧭️view|🗂️catalog|🕹️interaction|🖌️session|🧵️retained|👆️gestures|🧰️kit)/🦀️\\.rs\$"
F2="✏️editor/(🕹️interaction|🖌️session|🧵️retained|🧰️kit)/🦀️\\.rs\$"
FT="🗺️tiled-map/(🧭️projection|🧩️tiles|🧱️vector-tiles|🪢️map-state|🏛️map-host|🖱️interaction-and-labels|🌐️wasm-session|🌍️gis-policy)/(.*/)?🦀️\\.rs\$"
FHD="📇️directory/(🧾️model|🎟️capabilities|🔤️wire|🕰️events|🧭️decider|🛠️service|🔌️port|🔀️dispatch)/🦀️\\.rs\$"
FHR="💡️inference/🏃️runtime/(🧾️approval-contract|🌍️gis-map-binding|🔐️committer|🧱️document-preparation|🔁️commit-turns|🔎️checkpoint-verify|📕️commit-and-close|🧭️job-types-and-port|🚧️checkpoint-test-gate|🏭️inference-runtime|🛣️routes)/🦀️\\.rs\$"
FC="⌨️cli/(🧭️repo-cli/(🎮️command-framework|🌳️command-tree|🗂️tree-source-and-search|🚦️verbs|🚀️run-and-projection)|📡️event-stream)/🦀️\\.rs\$"
FG="🦀️rust/(🔤️grammar|🏗️repo-schema|🎥️context-port|🖼️sources|⚡️execution|🩻️recording-context)/🦀️\\.rs\$"
one() {
  echo "=== $1"; shift
  python3 famscan.py "$@" "$O/fs.json" ${EXTRA:+$EXTRA}
  python3 famitems.py "$@" "$O/fi.json" ${EXTRA:+$EXTRA}
  python3 tuplecheck.py "$1" "$2" "$3" ${EXTRA:+$EXTRA}
}
one p5 "$R/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs" "$E5/🦀️.rs" "$F5"
one p2 "$R/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🦀️.rs" "$E2/🦀️.rs" "$F2"
one tm "$T/📦️packages/🦀️rust/🦀️.rs" "$T/🗺️tiled-map/🦀️.rs" "$FT"
one hub-directory "$HB/📦️packages/🦀️rust/🦀️.rs" "$HB/📇️directory/🦀️.rs" "$FHD"
one hub-runtime "$HB/📦️packages/🦀️rust/🦀️.rs" "$HB/💡️inference/🏃️runtime/🦀️.rs" "$FHR"
EXTRA="📡️event-stream/🦀️\\.rs" one cli "$C/🦀️.rs" "$C/🧭️repo-cli/🦀️.rs" "$FC"
one gql "$G/🦀️.rs" "$G/🦀️.rs" "$FG"
