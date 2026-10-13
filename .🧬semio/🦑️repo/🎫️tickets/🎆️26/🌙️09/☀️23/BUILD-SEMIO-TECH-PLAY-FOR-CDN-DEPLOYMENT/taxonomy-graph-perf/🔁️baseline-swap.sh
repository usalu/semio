#!/usr/bin/env bash
# 🔁️ Runs listed tests against the pre-change sources in place: backs up the current sources and edited tests, installs the snapshot sources
# and the HEAD test files, runs the tests, and always restores the current files (trap). argv: <repoRoot> <snapshotRoot> <backupDir> <testList> <artifactDir> <outFile>
set -u
root="$1"; snapshot="$2"; backup="$3"; list="$4"; artifacts="$5"; out="$6"
lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
sources=("$lib/🧹️normalization/🟦️.ts" "$lib/🧹️normalization/📁️input/🟦️.ts" "$lib/🧹️normalization/🛣️path/🟦️.ts" "$lib/🧹️normalization/🚪️source-admission/🟦️.ts" "$lib/🧹️normalization/🚪️source-admission/📁️io/🟦️.ts" "$lib/🔍️discovery/🟦️.ts" "$lib/🧾️serialization/🔣️json/🟦️.ts" "$lib/⚡️caching/🚀️bootstrap/📜️script.ts" "$lib/🟨️.mjs" "$lib/🕸️dependencies/🟦️typescript/🟨️.mjs" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs" "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs")
tests=("$lib/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts" "$lib/🧪️tests/📈️reference-coordinate-progress/🟦️.ts" "$lib/🧪️tests/🔏️path-emoji-statutes/🟦️.ts" "$lib/🧪️tests/🔗️markdown-inline-references/🟦️.ts" "$lib/🧪️tests/🔬️workspace-contract/🟦️.ts" "$lib/🧪️tests/🚚️readme-move-source-authority/🟦️.ts" "$lib/🧪️tests/🛤️typescript-path-collection/🟦️.ts" "$lib/🧪️tests/🛫️preflight-reference-basis/🟦️.ts" "$lib/🧪️tests/🥇️bare-reference-sibling-precedence/🟦️.ts" "$lib/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts" "$lib/🧹️normalization/🧪️tests/🚪️source-admission-io/🟦️.ts")
mkdir -p "$backup"
for file in "${sources[@]}" "${tests[@]}"; do mkdir -p "$backup/$(dirname "$file")"; cp "$root/$file" "$backup/$file"; done
restore() { for file in "${sources[@]}" "${tests[@]}"; do cp "$backup/$file" "$root/$file"; done; }
trap restore EXIT
for file in "${sources[@]}"; do cp "$snapshot/$file" "$root/$file"; done
for file in "${tests[@]}"; do (cd "$root" && git show "HEAD:$file" > "$file"); done
"$(dirname "$0")/🧪️run-tests.sh" "$root/🧰️framework/🛍️products/🦑️repo/🔨️modules" "$list" "$artifacts" "$out"
