#!/bin/sh
set -e
ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"
MSG="$(mktemp)"
bun ./📜️script.ts micro-commit prepare >/dev/null
cp .git/gkcommittemplate.txt "$MSG"
echo "[DEBUG] commit.template=$(git config --local --get commit.template || echo unset)"
printf '\n%s\n' "USER EDIT" >>"$MSG"
bun ./📜️script.ts micro-commit prepare-commit-msg "$MSG" template >/dev/null
if grep -q "USER EDIT" "$MSG"; then
  echo "PASS: user edit preserved on commit hook"
else
  echo "FAIL: hook overwrote user edit"
  exit 1
fi
cp .git/gkcommittemplate.txt "$MSG"
bun ./📜️script.ts micro-commit prepare-commit-msg "$MSG" template >/dev/null
if grep -q "🚩️" "$MSG" && ! grep -q "USER EDIT" "$MSG"; then
  echo "PASS: unchanged prepared message refreshed at commit"
else
  echo "FAIL: hook did not refresh prepared message"
  exit 1
fi
rm -f "$MSG"
echo "ALL PASS"
