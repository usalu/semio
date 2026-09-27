#!/bin/zsh
# ⚖️ LC landing laws, one native-lane hold per cargo (session 13 rule 30): build-fleet-b, nice 15, private target.
cd /Users/ueli/Documents/semio || exit 2
out=".🧬semio/🌐hub/s13-lc-laws"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-lc/target RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
run() {
  local name="$1"; shift
  echo "START $name $(date '+%H:%M:%S')"
  zsh .tmp-ticket/📜️fleet-mutex.sh native lc -- nice -n 15 cargo "$@" > "$out/$name.txt" 2>&1
  echo "END $name rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error' "$out/$name.txt" | tr '\n' ' ' | cut -c1-300)"
}
laws=("$@"); (( $#laws )) || laws=(hub-bin flow space-home writer jack vcs reasoning architect kernel hub-policy)
for law in $laws; do
  case "$law" in
    hub-bin) run hub-bin check -p semio-hub --bin os-hub --tests --message-format short ;;
    flow) run flow test -p semio-s-artifact-flow-flow --lib --no-fail-fast -- --test-threads 4 ;;
    space-home) run space-home test -p semio-s-artifact-space-home --lib --no-fail-fast -- --test-threads 4 ;;
    space-home-editor) run space-home-editor test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- editor --test-threads 4 ;;
    writer) run writer test -p semio-s-artifact-writer-writer --lib --no-fail-fast -- a_typing_run_longer_than_the_edit_ledger ;;
    jack) run jack test -p semio-s-artifact-trinity-jack --features component-app-assembly --lib --no-fail-fast -- a_typing_run_longer_than_the_edit_ledger text_edit ;;
    vcs) run vcs test -p semio-s-artifact-vcs-vcs --lib --no-fail-fast -- a_typing_run_longer_than_the_edit_ledger text_edit ;;
    reasoning) run reasoning test -p semio-s-artifact-reasoning-wires --lib --no-fail-fast -- declared_verb_laws ;;
    architect) run architect test -p semio-s-artifact-architect-program --lib --no-fail-fast -- declared_verb_laws ;;
    kernel) run kernel test -p semio-framework-os-kernel --lib --no-fail-fast -- user_preference_record space_artifact_creation directory_event_page ;;
    hub-policy) run hub-policy test -p semio-hub --lib --no-fail-fast -- access_policy ;;
  esac
done
echo "LC-LAWS DONE $(date '+%H:%M:%S')"
