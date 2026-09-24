#!/bin/bash
# g7: full plugin-host lib and os-mcp lib suites on the final host tree, one cargo at a time.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-g7
bash g7-cargo.sh host-lib-final test -p semio-framework-plugin-host --lib --no-fail-fast
bash g7-cargo.sh mcp-lib-final test -p semio-framework-os-mcp --lib --no-fail-fast
