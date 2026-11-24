#!/bin/zsh
# O17: everything the play snapshot lacks for "finals go to workers first, ordinary units local first", as ONE ordered list.
# usage: apply.sh <snapshot-root>        (use a copy first: stage 1 patches temporary copies, stage 2 moves them in place in dependency order)
# Needs a driver restart only for the rows marked D in list.tsv; the rows marked W are read per rustc call (sealed copy per digest).
set -e
root=${1:?snapshot root}
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/.." && pwd)
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; RE="$L/🏗️build/🌐️remote-execution"; OFF="$RE/🚀️offload"; UC="$L/⚡️caching/🦀️cargo/🗃️unit-cache"; NB="🧰️framework/🔨️modules/🏃️process/📦️artifacts"; DS="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages"
files=("$OFF/🧬️schema/🔣️.json" "$OFF/📐️policy/🟦️.ts" "$OFF/🚦️admission/🟦️.ts" "$OFF/🟦️.ts" "$OFF/🔌️client/🟦️.ts" "$RE/🟦️.ts" "$OFF/📜️action/🟦️.ts" "$UC/📜️script.ts" "$UC/🔒️lock/🟦️.ts" "$NB/🏗️native-build/🎰️jobserver/🟦️.ts" "$NB/🕸️wasm-build/🟦️.ts" "$NB/🏗️native-build/🧩️component-pass/🟦️.ts" "$NB/🏗️native-build/🎟️admission/🟦️.ts" "$DS/🟦️.ts")
patches=(
 "$W/O11-patched/offload-schema.json.patch $W/O13-patched/offload-schema.patch $here/o17-schema.patch"
 "$W/O11-patched/offload-policy.ts.patch $W/O13-patched/offload-policy.patch $here/o17-policy.patch"
 "$W/O11-patched/offload-admission.ts.patch $W/O13-patched/admission.patch $here/o17-admission.patch"
 "$W/O13-patched/offload.patch $here/o17-offload.patch"
 "$W/O13-patched/client.patch"
 "$W/O13-patched/remote-execution-root.patch"
 "$W/O16-patched/action.patch"
 "$W/O13-patched/script.patch $here/o17-script.patch"
 "$W/O13-patched/lock.patch"
 "$W/O13-patched/jobserver.patch"
 "$W/O13-patched/wasm-build.patch"
 "$W/O13-patched/component-pass.patch"
 "$W/O11-patched/native-admission.ts.patch"
 "$W/O11-patched/driver-stages.ts.patch")
staged=()
for index in {1..${#files[@]}}; do
 target="$root/${files[$index]}"; temporary="$target.o17tmp"; cp "$target" "$temporary"; staged+=("$temporary")
 for patchfile in ${=patches[$index]}; do patch -s -F3 "$temporary" < "$patchfile" || { echo "FAILED: $patchfile on ${files[$index]}" >&2; rm -f "${staged[@]}"; exit 1; }; done
done
echo "stage 1 ok: ${#files[@]} files patched in temporary copies"
for index in {1..${#files[@]}}; do target="$root/${files[$index]}"; mv "$target.o17tmp" "$target"; done
echo "stage 2 ok: moved in place"
