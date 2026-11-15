#!/bin/bash
# runs one test file inside a Linux container: /s = stage (module copy of the working copy), /w = live tree read only. usage: linux-suite.sh <image> <test-dir> <test-file>
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/stage
docker run --rm -v "$S":/s -v /Users/Shared/semio:/w:ro "$1" bash -c '
M=$(cd /s && ls -d 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree); mkdir -p /tmp/home
cd "/s/$M/$1/🧪️tests" 2>/dev/null || cd "/s/$M/$1"
echo "platform $(uname -sm) bun $(bun --version) pwsh $(command -v pwsh >/dev/null && echo yes || echo no)"
env -i PATH="$PATH" HOME=/tmp/home TMPDIR=/tmp DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1 POWERSHELL_TELEMETRY_OPTOUT=1 bun test "./$2" --timeout 180000 2>&1 | tail -45' bash "$2" "$3"
