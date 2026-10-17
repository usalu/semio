#!/bin/sh
# Derives the lock bun would write for each of the six scopes, one after the other. Usage: lockall.sh
cd "$(dirname "$0")" || exit 2
for s in . ✏️s 🌎️hub ♻️mit-bestand 🏢️semio-tech 🎓️teaching; do ./lockgen.sh "$s"; done
