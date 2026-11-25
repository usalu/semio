cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust" && cargo metadata --offline --format-version 1 --no-deps >/dev/null; echo meta $?
cd /home/deck/semio; diff <(git show HEAD:"🌎️hub/Cargo.lock" 2>/dev/null) 🌎️hub/Cargo.lock | head -0
cp "🌎️hub/Cargo.lock" /tmp/lock-after.txt; cmp "🌎️hub/Cargo.lock" /home/deck/semio/.cache/none 2>/dev/null
