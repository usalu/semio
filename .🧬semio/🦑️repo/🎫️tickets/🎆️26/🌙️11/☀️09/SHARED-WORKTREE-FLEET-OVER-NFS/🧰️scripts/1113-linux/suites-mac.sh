#!/bin/bash
# macOS counterpart of suites-inner.sh (runs on the live worktree, read only except os temp)
BUN=${BUN:-/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun}
R=${REAL:-/Users/Shared/semio}; M=$(cd "$R" && ls -d 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree)
run(){ echo "=== $1/$2"; ( cd "$R/$M/$1/🧪️tests" && env -i PATH="$PATH" HOME="$HOME" TMPDIR=/tmp "$BUN" test "./$2" --timeout 30000 2>&1 | grep -E "^ [0-9]+ (pass|fail|skip)|^\(fail\)" ); }
run 📒️journal 🟦️.ts; run 🚧️boundary 🟦️.ts; run 🌿️git 🟦️.ts; run 🩺️attest 🟦️.ts
run 🔁️replica 🟦️.ts; run 🔁️replica 🏁️race.ts; run 🔁️replica 🌿️git.ts; run 🔁️replica/🥾️bootstrap 🟦️.ts
run 🗺️topology 🟦️.ts; run 🗺️topology 🔵️.ts
