#!/bin/zsh
# 🧺️ The six suites of the brief for the leftover `storage-crate-stack` (no other suite names it), one vitest process each: crate.sh <tag>   (run it through q.sh with 2500 MB)
SUITES=silhouette,three-views,physics,breathing,validation,power "${0:h}/suites.sh" "$1" storage-crate-stack
