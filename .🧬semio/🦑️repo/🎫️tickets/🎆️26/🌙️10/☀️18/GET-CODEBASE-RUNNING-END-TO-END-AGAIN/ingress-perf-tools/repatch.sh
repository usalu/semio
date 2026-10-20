#!/bin/sh
# 🔁 Rebuilds the patched overlay from the pristine overlay and applies the staged set to it.
cd "$(dirname "$0")/.." || exit 2
G="🗑️generated"
rm -rf "$G/ingress-perf-patched"
cp -R "$G/ingress-perf-overlay" "$G/ingress-perf-patched" || exit 2
python3 ingress-perf-tools/apply.py --root "$G/ingress-perf-patched"
