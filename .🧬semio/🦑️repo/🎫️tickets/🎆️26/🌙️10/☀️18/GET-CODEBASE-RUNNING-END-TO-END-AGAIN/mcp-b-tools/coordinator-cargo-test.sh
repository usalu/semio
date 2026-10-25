#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
exec nice -n 10 repo cargo --store=shared --configuration=prototype -- test --manifest-path "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust/Cargo.toml" -p semio-framework-repo-coordinator --no-fail-fast --message-format=short
