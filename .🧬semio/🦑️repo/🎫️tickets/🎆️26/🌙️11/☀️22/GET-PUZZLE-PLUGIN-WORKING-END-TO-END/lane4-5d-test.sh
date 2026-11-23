#!/usr/bin/env bash
set -uo pipefail
export CARGO_PROFILE_DEV_CODEGEN_UNITS=1
export CARGO_PROFILE_TEST_CODEGEN_UNITS=1
export CARGO_PROFILE_DEV_OPT_LEVEL=0
export CARGO_PROFILE_TEST_OPT_LEVEL=0
purge() { find /tmp/build -type f -perm -u+x ! -name "*.so" ! -name "*.rlib" ! -name "*.rmeta" ! -name "*build*script*" -delete 2>/dev/null || true; }
fail=0
echo "=== CHECK workspace ==="
cargo check --workspace --all-targets --jobs 2 --offline
rc=$?
echo "CHECK workspace:$rc"
if [[ $rc -ne 0 ]]; then echo "LANE4_5D_FAIL:1"; exit 1; fi
echo "=== CHECK feature ==="
cargo check -p semio-s-artifact-puzzle-5d --all-targets --features component-app-assembly --jobs 2 --offline
rc=$?
echo "CHECK feature:$rc"
if [[ $rc -ne 0 ]]; then echo "LANE4_5D_FAIL:1"; exit 1; fi
pkgs=(
  semio-s-artifact-puzzle-5d
  semio-s-artifact-puzzle-5d-catalog
  semio-s-artifact-puzzle-5d-diff
  semio-s-artifact-puzzle-5d-diff-2
  semio-s-artifact-puzzle-5d-dispatch
  semio-s-artifact-puzzle-5d-dispatch-panels
  semio-s-artifact-puzzle-5d-document-records
  semio-s-artifact-puzzle-5d-host-admission-tests
  semio-s-artifact-puzzle-5d-interaction
  semio-s-artifact-puzzle-5d-kind-catalogs
  semio-s-artifact-puzzle-5d-mut-any
  semio-s-artifact-puzzle-5d-mut-any-part2d
  semio-s-artifact-puzzle-5d-mut-any-2
  semio-s-artifact-puzzle-5d-mut-any-3
  semio-s-artifact-puzzle-5d-mut-any-4
  semio-s-artifact-puzzle-5d-mut-any-5
  semio-s-artifact-puzzle-5d-mut-any-6
  semio-s-artifact-puzzle-5d-mut-any-7
  semio-s-artifact-puzzle-5d-mut-any-8
  semio-s-artifact-puzzle-5d-puzzle5d
  semio-s-artifact-puzzle-5d-retained
  semio-s-artifact-puzzle-5d-schema
  semio-s-artifact-puzzle-5d-snapshot
  semio-s-artifact-puzzle-5d-store-preparation
  semio-s-artifact-puzzle-5d-store-preparation-commands
  semio-s-artifact-puzzle-5d-testkit
  semio-s-artifact-puzzle-5d-tests
  semio-s-artifact-puzzle-5d-v1
  semio-s-artifact-puzzle-5d-v1-unit-tests
)
for p in "${pkgs[@]}"; do
  extra=()
  if [[ "$p" == "semio-s-artifact-puzzle-5d" ]]; then extra=(--features component-app-assembly); fi
  echo "=== TEST $p ==="
  cargo test -p "$p" --all-targets "${extra[@]}" --no-fail-fast --jobs 2 --offline
  rc=$?
  echo "TEST $p:$rc"
  if [[ $rc -ne 0 ]]; then fail=1; fi
  purge
done
echo "=== TEST snapshot-sqlite ==="
cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib hoisted_snapshot_2 --no-fail-fast --jobs 2 --offline
rc=$?
echo "TEST snapshot-sqlite:$rc"
if [[ $rc -ne 0 ]]; then fail=1; fi
purge
echo "=== TEST puzzle5d_host_admission_ ==="
cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib puzzle5d_host_admission_ --no-fail-fast --jobs 2 --offline
rc=$?
echo "TEST puzzle5d_host_admission_root:$rc"
if [[ $rc -ne 0 ]]; then fail=1; fi
cargo test -p semio-s-artifact-puzzle-5d-host-admission-tests --lib puzzle5d_host_admission_ --no-fail-fast --jobs 2 --offline
rc=$?
echo "TEST puzzle5d_host_admission_crate:$rc"
if [[ $rc -ne 0 ]]; then fail=1; fi
purge
echo "LANE4_5D_FAIL:$fail"
exit "$fail"
