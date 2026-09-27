#!/bin/zsh
# ✅ LB2 item 4: lb-p1 (brep casing) + lb-p4 (stdio shipped guard, editor_catalog required-features) still green on the current tree — one chain of native-lane holds.
cd /Users/ueli/Documents/semio || exit 2
zsh .tmp-ticket/wp-lb2/cargo-lane.sh i4-check-default-1 check --keep-going -p semio-s-plugin-stdio -p semio-s-artifact-stdio-semio --lib --tests
zsh .tmp-ticket/wp-lb2/cargo-lane.sh i4-test-laws-1 test -p semio-s-plugin-stdio --test shipped_fleet --no-fail-fast
zsh .tmp-ticket/wp-lb2/cargo-lane.sh i4-test-semio-lib-1 test -p semio-s-artifact-stdio-semio --lib --no-fail-fast -- brep
zsh .tmp-ticket/wp-lb2/parity-lane.sh i4-parity-brep-1 🧊️mutate-semio-brep
echo "ALL DONE $(date '+%H:%M:%S')" > .tmp-ticket/wp-lb2/generated/i4-done.txt
