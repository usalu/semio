#!/bin/zsh
# 🧫️ S19 set `norm-fixtures` — WINDOW-3 landing procedure (after `norm-examples` landed: en1998's regenerated assets are the
# vector bases). Production-derived mutation fixtures for the 12 norm families whose committed cases do not replay through
# production (measured 2026-09-28 `s14-s19-logs/fixture-verify-1.txt`: 0 valid of 62/28/10/25/22/48/29/19 cases, none for
# en1991/en1992/en1997/en1999 — the program matrix stages the first case's `➡️after` → 12/15 norm rows red). The three
# families whose cases all replay (din18599, en1993, en1995) are left untouched.
#   1. overlay sync + emitter build (overlay lane, overlay-only `harness = false` `[[test]] s19_emitter` of the norm plugin)
#   2. vectors from the LIVE tree's production (`S19_ROOT=<repo>`), 3. materialize into the repo, 4. replay-verify 15/15.
# usage: zsh s19-norm-fixtures.sh <capture>
capture="$1"
R=/Users/ueli/Documents/semio; H="$R/.tmp-ticket/wp-s19"; O="$R/.🧬semio/🌐hub/s14-s19-overlay"
FAMILIES=(din16798 din4108 en1990 en1991 en1992 en1994 en1996 en1997 en1998 en1999 iso16757 vdi3805)
echo "START $(date '+%T')" > "$capture"
python3 "$H/s19-overlay.py" "$O" >> "$capture" 2>&1 || exit 1
M="$O/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml"
/usr/bin/grep -q 's19_emitter' "$M" || printf '\n[[test]]\nname = "s19_emitter"\npath = "%s/s19-emitter.rs"\nharness = false\n' "$H" >> "$M"
( cd "$O" && CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$O/.s19-build" CARGO_TARGET_DIR="$O/.s19-target" \
  zsh "$R/.tmp-ticket/📜️fleet-mutex.sh" overlay s19 -- nice -n 15 cargo test -p semio-s-plugin-norm --test s19_emitter --no-run ) >> "$capture" 2>&1 || exit 1
E=$(ls -t "$O"/.s19-build/debug/build/semio-s-plugin-norm/*/out/s19_emitter-*(N) | head -1)
echo "EMITTER $E" >> "$capture"
S19_ROOT="$R" nice -n 10 python3 "$H/s19-vectors.py" "$E" $FAMILIES >> "$capture" 2>&1 || exit 1
( cd "$H" && S19_ROOT="$R" nice -n 10 bun s19-materialize.ts --write $FAMILIES ) >> "$capture" 2>&1 || exit 1
nice -n 10 python3 "$H/s19-fixture-verify.py" "$E" "$R" >> "$capture" 2>&1
echo "END $(date '+%T')" >> "$capture"
