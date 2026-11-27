#!/bin/sh
# usage: run-check-pkg.sh <log name> <package>  (gated cargo check of one package with its tests; log and exit code in 🗑️generated)
cd "$(dirname "$0")"
G=🗑️generated
python3 watchdog-cargo.py sh ./native-cargo.sh check -p "$2" --tests > $G/$1.log 2>&1
echo "exit $?" > $G/$1.done
