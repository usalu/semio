#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
rg -n -o --no-heading -F -e '[DEBUG]' --hidden -g '!node_modules' -g '!.cache' -g '!🗑️generated' -g '!bazel-*' -g '!dist' -g '!target' \
  🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching 🧰️framework/🔨️modules/🏃️process 🎓️teaching 🧰️framework/🛍️products/📓️print 🧰️framework/🛍️products/🎤️presentation 🧰️framework/🛍️products/🐾️pets 🧰️framework/🛍️products/📚️course ♻️mit-bestand/📋️bericht ♻️mit-bestand/🎤️präsentation 2>/dev/null | cut -d: -f1,2 | sort -u
