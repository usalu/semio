#!/bin/zsh
# 🌎️ W1: hub 7800 on an APFS clone of the published catalog A root, with the binary the bootstrap validated the
# catalog with (copied to wp-w1/bin so a later rebuild of the shared target never replaces a running binary),
# credential sign-in enabled. Then a second fresh hub (7801) binds the same published catalog by content hash
# through OS_HUB_TRUSTED_CATALOG_SOURCE and must report the same generation.
set -u
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w1
SRC="/Users/ueli/Documents/semio/.🧬semio/🌐hub/w1-catalog-a"
DATA="/Users/ueli/Documents/semio/.🧬semio/🌐hub/w1-hub-7800"
BIN="$W/bin/os-hub"
mkdir -p "$W/bin"
rm -f "$BIN"; cp "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/os-hub" "$BIN"; codesign -f -s - "$BIN" 2>/dev/null
rm -rf "$DATA"; mkdir -m 700 "$DATA"; cp -c -R "$SRC/trusted-catalog" "$DATA/"
rm -rf "$DATA/trusted-catalog/validation" "$DATA/trusted-catalog"/build-* "$DATA/trusted-catalog"/staging-*
cd "$W" && OS_HUB_CREDENTIAL_SIGN_IN=1 python3 w1-detach.py generated/hub-7800.txt bun w1-hub-hold.ts 7800 "$DATA" "$BIN"
