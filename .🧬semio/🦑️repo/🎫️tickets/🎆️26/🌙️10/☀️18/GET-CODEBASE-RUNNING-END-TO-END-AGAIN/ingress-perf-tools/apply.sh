#!/bin/sh
# 🧩 Applies the ingress-perf staged set to the live checkout (anchored: refuses unless every anchor occurs exactly once; --dry-run writes nothing).
# Refuses while a dependency sync runs (it restarts on any library write) unless --ignore-sync.
cd "$(dirname "$0")" || exit 2
case " $* " in
  *" --ignore-sync "*) ;;
  *) if ps -axo command | grep -v grep | grep -q "dependencies sync"; then echo "refused: a dependency sync is running (use --ignore-sync after PAUSE LIFTED)"; exit 3; fi ;;
esac
args=""
for a in "$@"; do [ "$a" = "--ignore-sync" ] || args="$args $a"; done
exec python3 apply.py $args
