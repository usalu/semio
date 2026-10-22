#!/bin/zsh
# 🚦 Post-GO sequence of the langs-conc set (one process at a time, nice -n 10; never while a cold `show` is running).
set -e
cd "$(dirname "$0")"
python3 apply.py --check-host          # informational: is the repository's pinned Go provisioned (the Go on PATH is never consulted)
python3 apply.py                       # dry run: 57 operations, any anchor drift is refused here
python3 apply.py --apply --coordinator-go   # provisions the pinned Go 1.26.9 into the tools partition when missing (checksummed), then all or nothing, backup in C/🗑️generated/langs-conc/; --undo restores while nothing moved
# first live exercise, in this order (each one command at a time):
#   suites to run first, from their test directories:
#   process/🏗️build/🧰️toolchains/🧪️tests, ⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🧪️tests, 🏗️build/🔧️tools/🏗️bazel/🧰️toolchains/🧪️tests,
#   🏗️build/🏃️execution/🔒️hermetic/🧪️tests, 🧪️tests/🧰️toolchain-pins, ⚡️caching/🧪️tests/📦️native-dependencies (needs the repo test envelope)
#   bun repo show projects                 # the keys of python/dotnet/cmake/go projects change once (toolchain fingerprint, no root python/cmake files)
#   bun repo run deps-cpp                  # downloads and verifies the pinned CMake and Ninja once (about 90 MB), then cpp-setup/cpp-test-toolchain
#   bun repo run deps-python               # installs the managed interpreters and syncs the workspace environment outside the checkout
#   bun repo run @semio-tech/energy-oracle-py:deps, @semio-tech/ui-styling-py:deps, then their test/build targets
#   bun repo run deps-dotnet, @semio-tech/ui-styling-dotnet:build
