#!/bin/sh
# ✅ [isov] verifies an ISOLATE batch against the corrected expectations and the isov supersessions: sh verify-isolate.sh <batch> <root>. Rows whose bytes are a `new` of an isov batch count as applied; expected-override/<batch>.json replaces the recorded failure sets measured before the compliance landing.
HERE="$(cd "$(dirname "$0")" && pwd)"
exec python3 -I "$HERE/verify_batch.py" "$HERE/../isolate-tools/staged/${1:?batch}" "${2:?root}"
