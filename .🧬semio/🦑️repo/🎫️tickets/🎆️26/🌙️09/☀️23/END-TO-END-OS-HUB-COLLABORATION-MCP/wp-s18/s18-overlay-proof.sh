#!/bin/zsh
# 🧪️ S18 overlay proof of the named-layout Rust twin, run INSIDE the overlay lane: re-mirror the live tree into the S18
# overlay, apply the prepared patch there (then a dry-run must report 0 changes = idempotent), run the config + wgpu-shell
# laws with private build/target dirs (s18-overlay-run.sh), and delete those private dirs afterwards (rule 23).
here="/Users/ueli/Documents/semio/.tmp-ticket/wp-s18"
out="$here/generated"
overlay="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-overlay"
echo "[s18-ov] HELD $(date '+%F %T')"
echo "[s18-ov] sync $(python3 "$here/s18-overlay-sync.py") $(date '+%T')"
S18_PATCH_ROOT="$overlay" python3 "$here/s18-named-layout-rust-twin.py" > "$out/s18-14c-ov-apply.txt" 2>&1
echo "[s18-ov] apply rc=$? $(tail -1 "$out/s18-14c-ov-apply.txt") $(date '+%T')"
S18_PATCH_ROOT="$overlay" python3 "$here/s18-named-layout-rust-twin.py" --dry-run > "$out/s18-14c-ov-redry.txt" 2>&1
echo "[s18-ov] re-dry-run $(tail -1 "$out/s18-14c-ov-redry.txt")"
S18_OV_PREFIX=s18-14c-ov zsh "$here/s18-overlay-run.sh"
echo "[s18-ov] disk before cleanup $(du -sh "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-build" "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-target" 2>/dev/null | tr '\n' ' ')"
rm -rf "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-build" "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-target"
echo "[s18-ov] DONE (private build/target dirs deleted) $(date '+%F %T')"
