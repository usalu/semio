#!/bin/zsh
# 🧪️ SH2 overlay runs (session 14 rule 3 / session 13 rule 37): ONE overlay build fleet-wide via the `overlay` fleet mutex, inside the
# scratch overlay with PRIVATE build/target dirs (never the shared build-dir), nice 15, incremental off.
overlay="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-sh2-overlay"
out="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-sh2-captures"
tag="${SH2_TAG:-ov}"
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-sh2-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-sh2-target"
run() {
  local name="$1"; shift
  echo "START $name $(date '+%H:%M:%S')"
  ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 cargo "$@" ) > "$out/$tag-$name.txt" 2>&1
  echo "END $name rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)' "$out/$tag-$name.txt" | sort | uniq -c | tr '\n' ' ' | cut -c1-600)"
}
jobs=("$@")
for job in $jobs; do
  case "$job" in
    check) run check check -p semio-s-artifact-space-home -p semio-s-plugin-space --features semio-s-artifact-space-home/component-app-assembly --lib --tests --message-format short ;;
    check-wasm) run check-wasm check -p semio-s-plugin-space --target wasm32-wasip2 --lib --message-format short ;;
    home) run home test -p semio-s-artifact-space-home --lib --no-fail-fast -- --test-threads 4 ;;
    home-feature) run home-feature test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4 ;;
    space) run space test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4 ;;
    plugin) run plugin test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4 ;;
    items)
      echo "START items $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'cargo check -p semio-framework-os-kernel --features sync,ureq --lib --tests --message-format short; echo "== home-feature"; cargo test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== home"; cargo test -p semio-s-artifact-space-home --lib --no-fail-fast -- --test-threads 4' ) > "$out/$tag-items.txt" 2>&1
      echo "END items rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)|^== ' "$out/$tag-items.txt" | sort | uniq -c | tr '\n' ' ' | cut -c1-600)" ;;
    b1)
      echo "START b1 $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'echo "== config"; cargo test -p semio-framework-os-config --lib --no-fail-fast; echo "== home-feature"; cargo test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== plugin"; cargo test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4' ) > "$out/$tag-b1.txt" 2>&1
      echo "END b1 rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)|^== ' "$out/$tag-b1.txt" | tr '\n' ' ' | cut -c1-900)" ;;
    b1-guest)
      echo "START b1-guest $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'echo "== home-feature"; cargo test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== home"; cargo test -p semio-s-artifact-space-home --lib --no-fail-fast -- --test-threads 4; echo "== plugin"; cargo test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4' ) > "$out/$tag-b1-guest.txt" 2>&1
      echo "END b1-guest rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)|^== ' "$out/$tag-b1-guest.txt" | tr '\n' ' ' | cut -c1-900)" ;;
    plugin-proofs) run plugin-proofs test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4 interactive_job_catalog ;;
    p1)
      echo "START p1 $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'echo "== home-feature"; cargo test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== home"; cargo test -p semio-s-artifact-space-home --lib --no-fail-fast -- --test-threads 4; echo "== plugin"; cargo test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4; echo "== space-check"; cargo check -p semio-s-artifact-space-space --features component-app-assembly --lib --tests --message-format short' ) > "$out/$tag-p1.txt" 2>&1
      echo "END p1 rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)|^== ' "$out/$tag-p1.txt" | tr '\n' ' ' | cut -c1-900)" ;;
    p1-check)
      echo "START p1-check $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'cargo check -p semio-s-artifact-space-home -p semio-s-plugin-space --features semio-s-artifact-space-home/component-app-assembly --lib --tests --message-format short' ) > "$out/$tag-p1-check.txt" 2>&1
      echo "END p1-check rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^error(\[|:)|^warning: `' "$out/$tag-p1-check.txt" | tr '\n' ' ' | cut -c1-900)" ;;
    p2)
      echo "START p2 $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'echo "== space-check"; cargo check -p semio-s-artifact-space-space -p semio-s-artifact-space-home -p semio-s-plugin-space --features semio-s-artifact-space-space/component-app-assembly,semio-s-artifact-space-home/component-app-assembly --lib --tests --message-format short; echo "== space-feature"; cargo test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== space"; cargo test -p semio-s-artifact-space-space --lib --no-fail-fast -- --test-threads 4; echo "== home-feature"; cargo test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== plugin"; cargo test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4; echo "== kernel-directory"; cargo test -p semio-framework-os-kernel --lib --no-fail-fast -- --test-threads 4 os_directory; echo "== renderer-check"; cargo check -p semio-framework-os-renderer-wgpu --lib --tests --message-format short' ) > "$out/$tag-p2.txt" 2>&1
      echo "END p2 rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)|^== ' "$out/$tag-p2.txt" | tr '\n' ' ' | cut -c1-1200)" ;;
    p2-base)
      echo "START p2-base $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/p2/p2-set.py revert --root "$PWD"; echo "== base-space-feature"; cargo test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/p2/p2-set.py apply --write --root "$PWD" | tail -2; echo "== space-feature"; cargo test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== kernel-directory"; cargo test -p semio-framework-os-kernel --lib --no-fail-fast -- --test-threads 4 os_directory; echo "== home-feature"; cargo test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4 transient' ) > "$out/$tag-p2-base.txt" 2>&1
      echo "END p2-base rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)|^== ' "$out/$tag-p2-base.txt" | tr '\n' ' ' | cut -c1-1200)" ;;
    p2-cycle)
      echo "START p2-cycle $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'set -e; python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/sh2-overlay-sync.py | tail -1; python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/p2/p2-set.py clear-created --root "$PWD"; echo "== base-space-feature"; cargo test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4 || true; python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/p2/p2-set.py apply --write --root "$PWD" | tail -2 | tee /dev/stderr | grep -q "^written"; set +e; echo "== space-feature"; cargo test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== space"; cargo test -p semio-s-artifact-space-space --lib --no-fail-fast -- --test-threads 4; echo "== kernel-directory"; cargo test -p semio-framework-os-kernel --lib --no-fail-fast -- --test-threads 4 os_directory; echo "== home-feature"; cargo test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== plugin"; cargo test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4; echo "== renderer-check"; cargo check -p semio-framework-os-renderer-wgpu --lib --tests --message-format short' ) > "$out/$tag-p2-cycle.txt" 2>&1
      echo "END p2-cycle rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)|^== |conflicts=' "$out/$tag-p2-cycle.txt" | tr '\n' ' ' | cut -c1-1500)" ;;
    p2-proof)
      echo "START p2-proof $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'set -e; python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/sh2-overlay-sync.py | tail -1; python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/p2/p2-set.py clear-created --root "$PWD"; python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/p2/p2-set.py apply --write --root "$PWD" | tail -2 | tee /dev/stderr | grep -q "^written"; set +e; echo "== space-feature"; cargo test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== space"; cargo test -p semio-s-artifact-space-space --lib --no-fail-fast -- --test-threads 4; echo "== kernel-directory"; cargo test -p semio-framework-os-kernel --lib --no-fail-fast -- --test-threads 4 os_directory; echo "== kernel-check"; cargo check -p semio-framework-os-kernel --features sync,ureq --lib --tests --message-format short; echo "== home-feature"; cargo test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== home"; cargo test -p semio-s-artifact-space-home --lib --no-fail-fast -- --test-threads 4; echo "== plugin"; cargo test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4; echo "== renderer-check"; cargo check -p semio-framework-os-renderer-wgpu --lib --tests --message-format short' ) > "$out/$tag-p2-proof.txt" 2>&1
      echo "END p2-proof rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)|^== |conflicts=' "$out/$tag-p2-proof.txt" | tr '\n' ' ' | cut -c1-1500)" ;;
    p2-space)
      echo "START p2-space $(date '+%H:%M:%S')"
      ( cd "$overlay" && zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay sh2 -- nice -n 15 zsh -c 'echo "== space-feature"; cargo test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4; echo "== space"; cargo test -p semio-s-artifact-space-space --lib --no-fail-fast -- --test-threads 4' ) > "$out/$tag-p2-space.txt" 2>&1
      echo "END p2-space rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)|^== ' "$out/$tag-p2-space.txt" | tr '\n' ' ' | cut -c1-900)" ;;
    kernel-check) run kernel-check check -p semio-framework-os-kernel --features sync,ureq --lib --tests --message-format short ;;
    kernel) run kernel test -p semio-framework-os-kernel --features sync,ureq --lib --no-fail-fast -- --test-threads 4 ;;
  esac
done
echo "SH2-OVERLAY DONE $(date '+%H:%M:%S')"
