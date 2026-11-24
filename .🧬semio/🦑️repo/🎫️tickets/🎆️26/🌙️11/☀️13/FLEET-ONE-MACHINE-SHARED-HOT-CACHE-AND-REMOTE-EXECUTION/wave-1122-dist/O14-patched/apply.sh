#!/bin/sh
# O14: apply the three patches to a snapshot root (copy to a temporary name, patch, then mv; -F3 tolerates the neighbouring import line of O13).
# usage: apply.sh <snapshot-root>   (use a copy first; the driver must be restarted afterwards)
set -eu
root="${1:?snapshot root}"
here="$(cd "$(dirname "$0")" && pwd)"
nb="$root/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass"
apply() { target="$1"; patchfile="$2"; tmp="$target.o14tmp"; cp "$target" "$tmp"; patch -s -F3 "$tmp" < "$here/$patchfile"; mv "$tmp" "$target"; }
apply "$nb/🧵️job-slots/🧬️schema/🔣️.json" job-slots-schema.patch
apply "$nb/🧵️job-slots/🟦️.ts" job-slots.patch
apply "$nb/🟦️.ts" component-pass.patch
