#!/usr/bin/env bash
# Syncs the build library of the live checkout into the private root: only files whose bytes differ are replaced (the others stay hard links with their recorded stamps); the store is never touched.
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
R=${OWNERS_ROOT:-/home/deck/pw/o1}
rsync -a --checksum "/home/deck/semio/$B/" "$R/$B/"
