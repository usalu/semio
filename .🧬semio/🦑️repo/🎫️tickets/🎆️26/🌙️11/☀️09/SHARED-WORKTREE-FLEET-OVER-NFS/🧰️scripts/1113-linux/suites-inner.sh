#!/bin/bash
# runs inside the container; /w is the read-only worktree. usage: suites-inner.sh [pwsh]
M=$(cd /w && ls -d 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree)
cd "/w/$M" || exit 2
echo "platform $(uname -sm) bun $(bun --version) git $(git --version) python $(python3 --version) pwsh $(command -v pwsh >/dev/null && pwsh -NoProfile -Command '$PSVersionTable.PSVersion.ToString()' || echo none)"
run(){ local dir="$1" file="$2"; echo "=== $dir/$file"; ( cd "$dir/🧪️tests" && env -i PATH="$PATH" HOME=/tmp/home TMPDIR=/tmp DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1 POWERSHELL_TELEMETRY_OPTOUT=1 bun test "./$file" --timeout 30000 2>&1 | grep -E "^\s*(\(fail\)|\(skip\)|[0-9]+ (pass|fail|skip|todo)|Ran [0-9]+)|error:" ); }
mkdir -p /tmp/home
if [ "$1" = pwsh ]; then run 🗺️topology 🔵️.ts; exit; fi
run 📒️journal 🟦️.ts
run 🚧️boundary 🟦️.ts
run 🌿️git 🟦️.ts
run 🩺️attest 🟦️.ts
run 🔁️replica 🟦️.ts
run 🔁️replica 🏁️race.ts
run 🔁️replica 🌿️git.ts
run 🔁️replica/🥾️bootstrap 🟦️.ts
run 🗺️topology 🟦️.ts
run 🗺️topology 🔵️.ts
