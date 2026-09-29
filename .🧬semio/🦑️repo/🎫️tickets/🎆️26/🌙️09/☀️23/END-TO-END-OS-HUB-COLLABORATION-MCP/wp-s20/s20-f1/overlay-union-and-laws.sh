#!/bin/zsh
# 🧾️ S20 faults overlay: the row-12 native proof (L1's train union, `overlay-union.sh`) followed by the fault laws
# (record, fault text + catalog class scanner, MCP gateway mapping) — one overlay-lane job after another.
# usage: zsh overlay-union-and-laws.sh <tag>
tag="$1"
zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-s20/s20-f1/overlay-union.sh "$tag-union"
zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-s20/s20-f1/overlay-cargo.sh "$tag-laws" test --offline --no-fail-fast --lib -p semio-framework -p semio-framework-plugin -p semio-framework-os-mcp -p semio-framework-replication -- fault typed_operation catalog_classes map_fault gateway
