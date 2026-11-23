#!/bin/sh
# usage: cargo-ui.sh <log-name> <cargo args...>   (runs through the repo cargo ingress, output under 🗑️generated)
T="$(cd "$(dirname "$0")" && pwd)"
name="$1"; shift
cd /Users/ueli/Documents/semio/🧰️framework || exit 1
repo cargo --store=shared --configuration=prototype -- "$@" > "$T/🗑️generated/$name.txt" 2>&1
echo "exit=$?" >> "$T/🗑️generated/$name.txt"
