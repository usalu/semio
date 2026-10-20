#!/bin/sh
# rebuilds the scratch overlay from the live tree and the staged set: generators, ops, mirror, apply, tsconfig
# usage: overlay.sh [OVERLAY_DIR]   (default /private/tmp/conc-graph/overlay)
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
OVERLAY=${1:-/private/tmp/conc-graph/overlay}
python3 "$HERE/gen_fixture.py" >/dev/null
python3 "$HERE/gen_env_fixture.py" >/dev/null
python3 "$HERE/gen_load_fixture.py" >/dev/null
python3 "$HERE/ops_gen.py"
python3 "$HERE/mirror.py" "$OVERLAY" >/dev/null
python3 "$HERE/apply.py" apply --write --root "$OVERLAY" --ignore-running | tail -1
python3 - "$OVERLAY" <<'PY'
import json, os, sys
overlay = sys.argv[1]
E = lambda c: chr(c) + "️"
L = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
B = os.path.join(L, E(0x1F3D7) + "build"); G = os.path.join(B, E(0x1F578) + "graph"); GEN = os.path.join(G, E(0x1F570) + "generations")
TS = E(0x1F7E6) + ".ts"; T = E(0x1F9EA) + "tests"
files = [os.path.join(G, TS), os.path.join(GEN, T, TS), os.path.join(GEN, T, E(0x1F3C3) + "world", TS), os.path.join(GEN, E(0x1F50C) + "load", T, TS), os.path.join(GEN, E(0x1F30D) + "environment", T, TS), os.path.join(G, T, E(0x1F570) + "generation-protocol", TS), os.path.join(B, E(0x1F9ED) + "invocation", TS), os.path.join(B, E(0x1F4DC) + "script.ts")]
json.dump({"extends": "./tsconfig.json", "compilerOptions": {"noEmit": True, "incremental": False, "skipLibCheck": True}, "include": [], "files": files}, open(os.path.join(overlay, "tsconfig.conc.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
PY
echo "overlay ready: $OVERLAY"
