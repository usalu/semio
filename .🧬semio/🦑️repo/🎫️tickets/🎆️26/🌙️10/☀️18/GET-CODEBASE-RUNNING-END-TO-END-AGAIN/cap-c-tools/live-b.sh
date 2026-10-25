#!/bin/sh
# live proof B: one repo cargo check beside it (counted compile units, default parallelism = planned)
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_SCRIPT_STALL_MS=14400000
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml -p semio-framework-value-retirement 2>&1
echo "rc=$?"
