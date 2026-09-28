#!/bin/zsh
# 🧊️ EN2 overlay reader-pipeline parity of the prepared glTF sets (F10b-1 production inverses, F10b-2 create-material
# fixture, ♾️any real-input afters + GLB probe) and the obj material case T13 re-derived — the 96-pair reader census plus
# the artifact-root case's new pairs. One overlay-lane hold, capped at 28 min by `en2-deadline.py`; a timed-out hold leaves
# the private build-dir warm for the next one.
# usage: zsh 📜️fleet-mutex.sh overlay en2 -- zsh en2-overlay-gltf.sh [<case>…]
set -u
HERE="/Users/ueli/Documents/semio/.tmp-ticket/wp-en2"
CASES=("$@")
[ ${#CASES} -eq 0 ] && CASES=(🏛️export-epjson-runs-in-energyplus 🎥️mutate-gltf-2-0-camera 🦴️mutate-gltf-2-0-skin 🎞️mutate-gltf-2-0-animation 💎️mutate-gltf-2-0-material 🪪️mutate-gltf-2-0-asset 🧊️mutate-gltf-2-0 🎨️mutate-obj-3-0-material)
echo "[en2] $(date '+%H:%M:%S') START gltf parity ${CASES[*]}"
python3 "$HERE/en2-deadline.py" 1680 zsh "$HERE/en2-overlay-parity.sh" "${CASES[@]}"
rc=$?
echo "[en2] $(date '+%H:%M:%S') gltf parity rc=$rc"
exit $rc
