#!/bin/zsh
# 🎚️ H13 hold 3 (session 14b, items 1, 2, 4): the db-independent half of the re-proof — os-mcp lib/tests type-check + the
# agent-binding role law and its authenticated-hub neighbours (🔗️remote), the guest-twin populated-pair codec law (server-minted
# genesis id), and the semio-os-mcp gateway build for the live P0 probe. The hub half waits for the db crate (hold 4).
# usage: h13-hold-3.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
zsh $W/h13-cargo.sh "$1-check-mcp" check -p semio-framework-os-mcp --lib --tests
zsh $W/h13-cargo.sh "$1-mcp-laws" test -p semio-framework-os-mcp --lib --no-fail-fast -- an_agent_binds_at_its_capped_role authenticated_hub_workspace authenticated_hub_catalog
zsh $W/h13-cargo.sh "$1-os-mcp" build -p semio-framework-os-mcp --bin semio-os-mcp
zsh $W/h13-cargo.sh "$1-plugin-codec-law" test -p semio-framework-plugin --lib --no-fail-fast -- the_codec_table_mirrors_and_passes_through_a_populated_pair_without_aborting
