#!/bin/sh
# Builds a symlink overlay of the checkout in $1 where only the files this lane edits are real copies.
set -eu
O="$1"; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; R=/Users/ueli/Documents/semio
rm -rf "$O"; mkdir -p "$O/🧰️framework/🛍️products/🦑️repo/🔨️modules"
for entry in "$R"/🧰️framework/*; do name=$(basename "$entry"); [ "$name" = "🛍️products" ] || ln -s "$entry" "$O/🧰️framework/$name"; done
mkdir -p "$O/🧰️framework/🛍️products"
for entry in "$R"/🧰️framework/🛍️products/*; do name=$(basename "$entry"); [ "$name" = "🦑️repo" ] || ln -s "$entry" "$O/🧰️framework/🛍️products/$name"; done
mkdir -p "$O/🧰️framework/🛍️products/🦑️repo"
for entry in "$R"/🧰️framework/🛍️products/🦑️repo/* "$R"/🧰️framework/🛍️products/🦑️repo/.[!.]*; do [ -e "$entry" ] || continue; name=$(basename "$entry"); [ "$name" = "🔨️modules" ] || ln -s "$entry" "$O/🧰️framework/🛍️products/🦑️repo/$name"; done
for entry in "$R"/🧰️framework/🛍️products/🦑️repo/🔨️modules/*; do name=$(basename "$entry"); [ "$name" = "📚️library" ] || ln -s "$entry" "$O/🧰️framework/🛍️products/🦑️repo/🔨️modules/$name"; done
mkdir -p "$O/$L"
for entry in "$R/$L"/* "$R/$L"/.[!.]*; do [ -e "$entry" ] || continue; name=$(basename "$entry"); case "$name" in 🧼️workspace-cleanup|🧪️tests) ;; *) ln -s "$entry" "$O/$L/$name";; esac; done
cp -R "$R/$L/🧼️workspace-cleanup" "$O/$L/🧼️workspace-cleanup"
mkdir -p "$O/$L/🧪️tests"
for entry in "$R/$L/🧪️tests"/*; do name=$(basename "$entry"); case "$name" in 🧹️clean-ticket-runs|🔬️workspace-contract|🧫️fixtures|🧱️root-clean-scaffold-source) ;; *) ln -s "$entry" "$O/$L/🧪️tests/$name";; esac; done
cp -R "$R/$L/🧪️tests/🧹️clean-ticket-runs" "$O/$L/🧪️tests/🧹️clean-ticket-runs"
mkdir -p "$O/$L/🧪️tests/🔬️workspace-contract" "$O/$L/🧪️tests/🧫️fixtures"
for entry in "$R/$L/🧪️tests/🔬️workspace-contract"/*; do name=$(basename "$entry"); case "$name" in 🎫️ticket-protection|🏪️managed-storage-protection) cp -R "$entry" "$O/$L/🧪️tests/🔬️workspace-contract/$name";; *) ln -s "$entry" "$O/$L/🧪️tests/🔬️workspace-contract/$name";; esac; done
for entry in "$R/$L/🧪️tests/🧫️fixtures"/*; do name=$(basename "$entry"); case "$name" in 🧱️root-clean-scaffold-source) cp -R "$entry" "$O/$L/🧪️tests/🧫️fixtures/$name";; *) ln -s "$entry" "$O/$L/🧪️tests/🧫️fixtures/$name";; esac; done
for entry in "$R"/* "$R"/.[!.]*; do [ -e "$entry" ] || continue; name=$(basename "$entry"); case "$name" in .git|.🧬semio|🧰️framework) continue;; esac; [ -e "$O/$name" ] || ln -s "$entry" "$O/$name"; done
echo overlay "$O"
