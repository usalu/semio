#!/usr/bin/env bash
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
M="$T/measure.sh"
D92="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust"
D94="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust"
DH="/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/📕️norm/🧩️extensions/📓️eurocode-structural/📦️packages/🦀️rust"
"$M" m1 "$D92" semio-s-artifact-norm-en1992-probe-a semio-s-artifact-norm-en1992-probe-b semio-s-artifact-norm-en1992-probe-c
"$M" m2 "$D92" semio-s-artifact-norm-en1992-mutations semio-s-artifact-norm-en1992-editor-field-meta semio-s-artifact-norm-en1992-editor-family semio-s-artifact-norm-en1992-editor-surfaces semio-s-artifact-norm-en1992-editor
"$M" m2 "$D94" semio-s-artifact-norm-en1994-mutations semio-s-artifact-norm-en1994-editor-field-meta semio-s-artifact-norm-en1994-editor-family semio-s-artifact-norm-en1994-editor-surfaces semio-s-artifact-norm-en1994-editor
"$M" m3 "$DH" semio-hub-norm-eurocode-structural-en1992-editor semio-hub-norm-eurocode-structural-en1992-viewer semio-hub-norm-eurocode-structural-en1994-editor semio-hub-norm-eurocode-structural-en1994-viewer
