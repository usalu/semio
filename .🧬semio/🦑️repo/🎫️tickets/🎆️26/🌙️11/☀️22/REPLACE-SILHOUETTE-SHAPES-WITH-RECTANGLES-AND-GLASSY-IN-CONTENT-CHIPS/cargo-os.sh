#!/bin/sh
# usage: cargo-os.sh <log-name> <cargo args...>   (repo cargo ingress from the repo-root workspace (OS crates), output under 🗑️generated)
T="$(cd "$(dirname "$0")" && pwd)"
name="$1"; shift
cd /Users/ueli/Documents/semio || exit 1
repo cargo --store=shared --configuration=prototype -- "$@" > "$T/🗑️generated/os-$name.txt" 2>&1
echo "exit=$?" >> "$T/🗑️generated/os-$name.txt"
