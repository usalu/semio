#!/bin/sh
cd "$(dirname "$0")"
start=$(date +%s)
sh large-documents-run.sh "$@"
echo "wall $(( $(date +%s) - start )) s" > "🗑️generated/$1.wall"
