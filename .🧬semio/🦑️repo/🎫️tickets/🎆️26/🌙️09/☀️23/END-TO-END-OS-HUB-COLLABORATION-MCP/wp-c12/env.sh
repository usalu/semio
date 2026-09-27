#!/bin/zsh
# 🔑️ C12: exports the local test credentials the 7800 recipe provisions (wp-w3/w3-restart-7800.sh) into the environment of
# probes, never onto argv or into logs. usage: source env.sh
export C12_USER1_PASSWORD=$(sed -n 's/.*user1@semio.dev|[^|]*|\([^"|]*\).*/\1/p' /Users/ueli/Documents/semio/.tmp-ticket/wp-w3/w3-restart-7800.sh | head -1)
export C12_USER2_PASSWORD=$(sed -n 's/.*user2@semio.dev|[^|]*|\([^"|]*\).*/\1/p' /Users/ueli/Documents/semio/.tmp-ticket/wp-w3/w3-restart-7800.sh | head -1)
export S_MATRIX_HUB=http://127.0.0.1:7800 S_MATRIX_ADMIN_FILE="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-w3-state-7800/admin-capability.json"
