#!/bin/sh
# builds /private/tmp/noopg/stage: a copy of the authorizers module with the memo patch, importing the live runtime through a symlink
set -e
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
A="$L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers"
R=/private/tmp/noopg/stage
rm -rf "$R"; mkdir -p "$R/$A"; ln -s "$PWD/node_modules" "$R/node_modules"
ln -s "$PWD/🧰️framework/📦️packages" "$R/🧰️framework/📦️packages"
ln -s "$PWD/🧰️framework/🔨️modules" "$R/🧰️framework/🔨️modules"
mkdir -p "$R/🧰️framework/🛍️products/🦑️repo/🔨️modules"
ln -s "$PWD/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" "$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
(cd "$A" && tar cf - --exclude 🧪️tests .) | (cd "$R/$A" && tar xf -)
mkdir -p "$R/$A/🧊️memo"
cp "$C/noop-g-tools/staged/memo.ts" "$R/$A/🧊️memo/🟦️.ts"
python3 "$C/noop-g-tools/staged/patch_authorizers.py" "$A/🟦️.ts" "$R/$A/🟦️.ts"
mkdir -p "$R/$A/🧪️tests/🧊️memo"
python3 "$C/noop-g-tools/staged/devtest.py" "$C/noop-g-tools/staged/memo-test.ts" "$R/$A/🧪️tests/🧊️memo/🟦️.ts"
echo staged "$R/$A"
