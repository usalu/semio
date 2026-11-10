#!/usr/bin/env bash
# resolves every relative import/dynamic import/new URL specifier of the graph split modules
cd "$1" || exit 1
for m in 🔐️authority 📜️contract 🛑️custody 🕊️release 📁️workspace 🧭️discovery 📍️pin 🏷️retention 🕰️host 🤝️prefetch 📥️load 🎯️resolve; do
  grep -aoE '(from|import\(|new URL\()\s*"\.[^"]*"' $m/🟦️.ts | grep -aoE '"\.[^"]*"' | tr -d '"' | sort -u | while read -r p; do [ -e "$m/$p" ] || echo "MISSING $m: $p"; done
done
