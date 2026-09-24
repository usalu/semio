#!/bin/bash
# g8: plugin-host lib suite three times back to back, one cargo at a time.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-g8
for run in 1 2 3; do bash g8-cargo.sh host-lib-$run test -p semio-framework-plugin-host --lib --no-fail-fast; done
