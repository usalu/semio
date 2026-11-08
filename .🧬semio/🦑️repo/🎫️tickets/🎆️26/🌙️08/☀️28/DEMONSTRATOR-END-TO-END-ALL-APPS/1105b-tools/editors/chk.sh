#!/bin/bash
# usage: editors/chk.sh <label> <manifest> <wasm|native> <args...>  (detached-run wrapper: writes generated/1105b/<label>.{txt,rc})
exec "$(dirname "$0")/../chk.sh" "$@"
