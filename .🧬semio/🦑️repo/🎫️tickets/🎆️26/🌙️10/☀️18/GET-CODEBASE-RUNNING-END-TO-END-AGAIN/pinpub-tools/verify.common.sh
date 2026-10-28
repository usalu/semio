# ✅ [pinpub] shared helper of the batch verify scripts: law <pass>/<fail> <repo-relative test file> [bun test args]  (bare `bun test` from ROOT, artifacts under ROOT/.cache/test-sandbox/pinpub; sets FAILED)
FAILED=0; LOGS="$(cd "$HERE/../../.." && pwd)/🗑️generated/pinpub/verify"; mkdir -p "$LOGS"
law(){ want="$1"; f="$2"; shift 2; log="$LOGS/$(printf '%s' "$f" | shasum | cut -c1-10).log"
 (cd "$ROOT" && mkdir -p .cache/test-sandbox/pinpub && SEMIO_TEST_ARTIFACT_DIR="$ROOT/.cache/test-sandbox/pinpub" nice -n 10 "${BUN:-bun}" test --timeout 300000 "./$f" "$@" > "$log" 2>&1)
 got="$(/usr/bin/grep -E '^ *[0-9]+ pass$' "$log" | tr -dc 0-9)/$(/usr/bin/grep -E '^ *[0-9]+ fail$' "$log" | tr -dc 0-9)"
 if [ "$got" = "$want" ]; then echo "ok   $got $f"; else echo "FAIL $got (want $want) $f  log: $log"; /usr/bin/grep -E '^\(fail\)' "$log" | cut -c1-200; FAILED=1; fi; }
