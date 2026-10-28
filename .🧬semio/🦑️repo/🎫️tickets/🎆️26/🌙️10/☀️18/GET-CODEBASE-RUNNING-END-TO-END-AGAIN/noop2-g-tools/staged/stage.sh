#!/bin/sh
# builds /private/tmp/noop2g/stage: symlinks to the live library except the patched Bazel adapter script and the two staged modules
set -e
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/noop2-g-tools/staged"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
R=/private/tmp/noop2g/stage
rm -rf "$R"; mkdir -p "$R/$L/🏗️build/🔧️tools/🏗️bazel"
ln -s "$PWD/node_modules" "$R/node_modules"
for entry in package.json tsconfig.json; do [ -e "$entry" ] && ln -s "$PWD/$entry" "$R/$entry" || true; done
link_siblings() { # link_siblings <real dir> <stage dir> <names to skip...>
  real="$1"; stage="$2"; shift 2
  for entry in "$real"/* "$real"/.[!.]*; do
    [ -e "$entry" ] || continue
    name=$(basename "$entry"); skip=0
    for s in "$@"; do [ "$name" = "$s" ] && skip=1; done
    [ $skip = 1 ] || ln -s "$PWD/$entry" "$stage/$name"
  done
}
# every ancestor of the library: link siblings
anc=""; for part in 🧰️framework 🛍️products 🦑️repo 🔨️modules 📚️library; do
  case "$part" in 🧰️framework) real="$PWD/🧰️framework"; stage="$R/🧰️framework";; *) :;; esac
done
# explicit chain
link_chain() {
  real="$PWD"; stage="$R"
  for part in "🧰️framework" "🛍️products" "🦑️repo" "🔨️modules" "📚️library" "🏗️build" "🔧️tools" "🏗️bazel"; do
    next="$part"
    case "$part" in 🧰️framework) realdir="$real/$part";; *) realdir="$real/$part";; esac
    mkdir -p "$stage/$part"
    for entry in "$real/$part"/* ; do
      [ -e "$entry" ] || continue
      name=$(basename "$entry")
      skip=0
      case "$part/$name" in
        "🧰️framework/🛍️products") skip=1;; "🛍️products/🦑️repo") skip=1;; "🦑️repo/🔨️modules") skip=1;; "🔨️modules/📚️library") skip=1;;
        "📚️library/🏗️build") skip=1;; "🏗️build/🔧️tools") skip=1;; "🔧️tools/🏗️bazel") skip=1;; "🏗️bazel/📜️script.ts") skip=1;;
      esac
      [ $skip = 1 ] || ln -s "$entry" "$stage/$part/$name"
    done
    real="$real/$part"; stage="$stage/$part"
  done
}
rm -rf "$R/$L"; mkdir -p "$R"
link_chain
B="$R/$L/🏗️build"
rm -f "$B/🔭️witness" "$B/🔧️tools/🏗️bazel/♻️plan-reuse"
M="${STAGE_MODULES:-$S}"
cp -R "$M/🔭️witness" "$B/🔭️witness"
cp -R "$M/♻️plan-reuse" "$B/🔧️tools/🏗️bazel/♻️plan-reuse"
cp "$L/🏗️build/🔧️tools/🏗️bazel/📜️script.ts" "$B/🔧️tools/🏗️bazel/📜️script.ts"
echo staged "$R"
