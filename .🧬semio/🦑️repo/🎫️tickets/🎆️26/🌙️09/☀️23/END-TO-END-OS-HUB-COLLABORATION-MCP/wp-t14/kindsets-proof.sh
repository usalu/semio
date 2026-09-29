#!/bin/zsh
# ✅️ T14 scratch proof of the T3 kind sets (W4 kind-spec identity + W4 codec gate + T14 kind-choices) on a clone of the live tree
# (`.🧬semio/🌐hub/s14-t14-kindsets`, PRIVATE build-dir inside it, registry units seeded): native --lib --tests check of every touched
# crate + demonstrator, then the identity/gate laws. Resumable (state file), ≤ 28 min per run. usage: kindsets-proof.sh
S="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-kindsets"; L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-logs"; T=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14
export CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$S/.t14-build" CARGO_TARGET_DIR="$S/.t14-target"
STATE=$L/s14c-kindsets-state.txt; touch $STATE
DEADLINE=$(( $(date +%s) + 28 * 60 ))
P=(-p semio-framework -p semio-framework-plugin-describe -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-flow-flow -p semio-s-artifact-forms-forms -p semio-s-artifact-imperative-procedure -p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-mathematical-equation -p semio-s-artifact-norm-din16798 -p semio-s-artifact-norm-din18599 -p semio-s-artifact-norm-din4108 -p semio-s-artifact-norm-en1990 -p semio-s-artifact-norm-en1991 -p semio-s-artifact-norm-en1992 -p semio-s-artifact-norm-en1993 -p semio-s-artifact-norm-en1994 -p semio-s-artifact-norm-en1995 -p semio-s-artifact-norm-en1996 -p semio-s-artifact-norm-en1997 -p semio-s-artifact-norm-en1998 -p semio-s-artifact-norm-en1999 -p semio-s-artifact-norm-iso16757 -p semio-s-artifact-norm-vdi3805 -p semio-s-artifact-shooting-shooting -p semio-s-artifact-sourcing-curation -p semio-s-artifact-trinity-rewriting -p semio-s-plugin-fem -p semio-s-plugin-norm  -p semio-s-artifact-demonstrator-playground)
F=semio-s-artifact-fem-2d/component-app-assembly,semio-s-artifact-fem-3d/component-app-assembly,semio-s-artifact-trinity-rewriting/component-app-assembly
run() { python3 $T/deadline.py $DEADLINE nice -n 15 "$@"; }
cd "$S" || exit 2
[ -d "$S/.t14-build/debug" ] || python3 $T/overlay-build-seed.py "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug" "$S/.t14-build/debug"
echo "RUN $(date +%T)"
if ! /usr/bin/grep -q '^CHECK 0' $STATE; then
  run cargo check --keep-going ${P[@]} --features $F --lib --tests --message-format short > $L/s14c-kindsets-CHECK.txt 2>&1; rc=$?
  echo "CHECK $rc $(date +%T)" | tee -a $STATE; [ $rc -ne 0 ] && exit $rc
fi
if ! /usr/bin/grep -q '^LAWS ' $STATE; then
  run cargo test --no-fail-fast ${P[@]} --features $F --lib -- artifact_kind kind_identity > $L/s14c-kindsets-LAWS.txt 2>&1; rc=$?
  echo "LAWS $rc $(date +%T)" | tee -a $STATE
fi
echo "END $(date +%T)"
