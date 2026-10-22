#!/bin/sh
# 🔎️ Runs every verification of the ordered plan list one after the other (dry run on the checkout, manifests with Bun TOML, cycles, definition identity, rustfmt parse of every rewritten source). One process at a time.
H="$(cd "$(dirname "$0")" && pwd)"
ALL=umbrella-reserved,macros-elsewhere,umbrella,os-kernel-macros,os-kernel-derive,os-kernel,plugin-macros,plugin,umbrella-dead,plugin-dead,os-kernel-dead,job-dead
nice -n 10 python3 "$H/apply.test.py"
nice -n 10 python3 "$H/apply.py" --plan $ALL | tail -2
nice -n 10 python3 "$H/verify_final.py" $ALL | head -8
nice -n 10 python3 "$H/../analysis/cycle_check.py" $ALL | tail -2
nice -n 10 python3 "$H/../analysis/verify_identity.py" $ALL | head -4
nice -n 10 python3 "$H/verify_rustfmt_final.py" $ALL | head -8
