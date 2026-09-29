#!/bin/zsh
# ⚖️ SH2 native-lane runs (session 14 rule 3): build-fleet-b, nice 15, incremental off, private target, one fleet-mutex hold per cargo.
cd /Users/ueli/Documents/semio || exit 2
out="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-sh2-captures"
tag="${SH2_TAG:-run}"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/target RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
run() {
  local name="$1"; shift
  echo "START $name $(date '+%H:%M:%S')"
  zsh .tmp-ticket/📜️fleet-mutex.sh native sh2 -- nice -n 15 cargo "$@" > "$out/$tag-$name.txt" 2>&1
  echo "END $name rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)' "$out/$tag-$name.txt" | tr '\n' ' ' | cut -c1-400)"
}
jobs=("$@"); (( $#jobs )) || jobs=(home home-feature space plugin kernel)
for job in $jobs; do
  case "$job" in
    home) run home test -p semio-s-artifact-space-home --lib --no-fail-fast -- --test-threads 4 ;;
    home-feature) run home-feature test -p semio-s-artifact-space-home --features component-app-assembly --lib --no-fail-fast -- --test-threads 4 ;;
    space) run space test -p semio-s-artifact-space-space --features component-app-assembly --lib --no-fail-fast -- --test-threads 4 ;;
    plugin) run plugin test -p semio-s-plugin-space --lib --no-fail-fast -- --test-threads 4 ;;
    kernel) run kernel test -p semio-framework-os-kernel --features sync,ureq --lib --no-fail-fast -- --test-threads 4 ;;
    hub-all)
      echo "START hub-all $(date '+%H:%M:%S')"
      zsh .tmp-ticket/📜️fleet-mutex.sh native sh2 -- nice -n 15 zsh -c 'cargo check -p semio-hub --lib --tests --bins --message-format short && cargo test -p semio-hub --lib --no-fail-fast -- --test-threads 4 delete_space_revokes_every_membership_before_the_space command_naming_a_deleted_space_is_not_found && cargo test -p semio-hub --bin os-hub --no-fail-fast -- --test-threads 2 a_member_of_a_deleted_space_is_told_its_access_was_revoked a_reader_added_to_an_older_space_is_told_its_access_changed' > "$out/$tag-hub-all.txt" 2>&1
      echo "END hub-all rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)' "$out/$tag-hub-all.txt" | tr '\n' ' ' | cut -c1-400)" ;;
    hub-regress)
      echo "START hub-regress $(date '+%H:%M:%S')"
      zsh .tmp-ticket/📜️fleet-mutex.sh native sh2 -- nice -n 15 zsh -c 'cargo test -p semio-hub --lib --no-fail-fast -- --test-threads 4 artifact_chunk_cas space_delete deleted_space; cargo test -p semio-hub --bin os-hub --no-fail-fast -- --test-threads 2 directory_invite_redemption retained_short_admin_shutdown access_changed deleted_space' > "$out/$tag-hub-regress.txt" 2>&1
      echo "END hub-regress rc=$? $(date '+%H:%M:%S') :: $(/usr/bin/grep -E '^test result|^error(\[|:)' "$out/$tag-hub-regress.txt" | tr '\n' ' ' | cut -c1-400)" ;;
    hub-check) run hub-check check -p semio-hub --lib --tests --bins --message-format short ;;
    hub-laws) run hub-laws test -p semio-hub --lib --no-fail-fast -- --test-threads 4 delete_space_revokes_every_membership_before_the_space command_naming_a_deleted_space_is_not_found ;;
    hub-socket-law) run hub-socket-law test -p semio-hub --bin os-hub --no-fail-fast -- --test-threads 2 a_member_of_a_deleted_space_is_told_its_access_was_revoked a_reader_added_to_an_older_space_is_told_its_access_changed ;;
    home-check) run home-check check -p semio-s-artifact-space-home -p semio-s-plugin-space --features semio-s-artifact-space-home/component-app-assembly --lib --tests --message-format short ;;
  esac
done
echo "SH2-NATIVE DONE $(date '+%H:%M:%S')"
