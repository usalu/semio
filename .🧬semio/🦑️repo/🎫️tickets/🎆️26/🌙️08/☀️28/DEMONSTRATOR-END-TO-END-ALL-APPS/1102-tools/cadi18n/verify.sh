#!/bin/zsh
# applies the cadi18n patch set (after uii18n cad-step-labels-1/2) on a copy outside the repo; never touches the tree
export LC_ALL=en_US.UTF-8
HERE=${0:A:h}
V=/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/verify
rm -rf $V; mkdir -p $V
cd /Users/ueli/Documents/semio
ORDER=(${HERE:h}/uii18n/patches/cad-step-labels-1.patch ${HERE:h}/uii18n/patches/cad-step-labels-2.patch $HERE/patches/*.patch)
for p in $ORDER; do /usr/bin/grep -h '^--- a/' $p | sed 's|^--- a/||'; done | sort -u | while IFS= read -r f; do mkdir -p "$V/${f:h}"; cp "$f" "$V/$f"; done
cd $V
rc=0
for p in $ORDER; do
  git apply --check $p && git apply $p && echo "OK ${p:t}" || { echo "FAIL ${p:t}"; rc=1; }
done
exit $rc
