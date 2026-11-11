#!/bin/bash
# usage: describe.sh [label] [pluginId...|closure]  — coordinator only. Refreshes the committed descriptor pairs (default energy; `closure` = the 32 demonstrator closure ids) in write mode. Run BEFORE activate.sh: a fresh build whose descriptor differs from the committed pair is refused. Arguments of the target go after `--` (workspace:components forwards all arguments). SEMIO_BUILD_MODE=prototype is pinned: the driver refuses `--profile prototype` under any other mode. describe.sh --help prints this and the command, runs nothing. Run it through slot.sh with a W- label: slot.sh W-<label> $0 <label> ...
T="$(cd "$(dirname "$0")/.." && pwd)"
CLOSURE="demonstrator cad gis procedural process puzzle sourcing flow playbook fem energy flow-extension-bim flow-extension-list flow-extension-brep flow-extension-dictionary flow-extension-text flow-extension-primitive flow-extension-draw flow-extension-logic flow-extension-math process-extension-metal process-extension-robotic process-extension-concrete process-extension-wood cad-extension-aec-building-structure cad-extension-aec-building cad-extension-spatial-shape cad-extension-aec-building-energy playbook-module-procedural sourcing-module-slabs sourcing-module-windows sourcing-module-beams"
case "${1:-}" in -h|--help) sed -n 2p "$0"; echo "command: SEMIO_BUILD_MODE=prototype repo run workspace:components -- describe ${2:-energy} --profile prototype"; exit 0;; esac
L="${1:-canary-describe}"; shift
[ "$#" -gt 0 ] || set -- energy
[ "$*" = closure ] && set -- $CLOSURE
ROOT="$T"; while [ ! -f "$ROOT/AGENTS.md" ] && [ "$ROOT" != "/" ]; do ROOT="$(dirname "$ROOT")"; done
cd "$ROOT" || exit 2
O="$T/../🗑️generated/1105b/$L"; mkdir -p "$(dirname "$O")"
SEMIO_LANGUAGE=en SEMIO_BUILD_MODE=prototype repo run workspace:components -- describe "$@" --profile prototype > "$O.log" 2>&1
rc=$?; echo "$rc" > "$O.rc"; exit $rc
