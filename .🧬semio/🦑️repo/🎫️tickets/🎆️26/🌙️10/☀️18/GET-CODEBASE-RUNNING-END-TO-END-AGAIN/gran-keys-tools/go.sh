#!/bin/zsh
# 🚦 Post-GO sequence of the gran-keys set (one process at a time, nice -n 10; never while chain15 is running).
set -e
cd "$(dirname "$0")"
python3 apply.py                       # dry run: 170 operations, any anchor drift is refused here
python3 apply.py --apply               # all or nothing, backup in C/🗑️generated/gran-keys/; --undo restores while nothing moved
L="../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
# suites (run from the library test directories, one at a time):
#   🏗️build/📥️inputs/🦀️resolution/🧪️tests            (SEMIO_RESOLUTION_REPO=1)
#   🏗️build/📥️inputs/🎯️precision/🧪️tests
#   🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️closure and 📥️fingerprint
#   ⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules       (expect only the 2 known uncached-without-reason peers)
#   🏗️build/📥️inputs/🧮️digest/🧪️tests                 (needs the repo test runner's issued process envelope)
# then the first live exercise: `bun repo show projects`, afterwards `verify/adapters-check.ts` with GK_ROOT set to the checkout.
