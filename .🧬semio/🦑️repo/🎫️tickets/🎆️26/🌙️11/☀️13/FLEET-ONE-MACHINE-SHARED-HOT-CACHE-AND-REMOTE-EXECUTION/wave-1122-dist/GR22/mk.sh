#!/bin/zsh
# Writes run/<name>.test.ts that loads the staged files and then one checkout test file (path relative to the build library).
here=${0:A:h}; mkdir -p $here/run
print -r -- "import \"../plug.ts\";
await import(\"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/$1\");" > $here/run/$2.test.ts
