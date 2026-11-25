#!/bin/zsh
# ⛓️ Submits the two proof chains of the batch one after the other from one place, each in its own grant: chain.sh <tag> <id>… runs measure.sh (800 MB), then rest.sh (2500 MB).
here="${0:A:h}"; tag="$1"; shift
"$here/measure.sh" "$tag" "$@"; "$here/rest.sh" "$tag" "$@"
