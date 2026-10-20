#!/bin/zsh
set -e
cd /private/tmp/gk
python3 mk-sandbox.py
python3 realize.py "🧰️framework/🔨️modules/🏃️process" "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine" "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧩️composition/🟨️.mjs" "🧰️framework/🔨️modules/🪪️identity/📁️installation"
cp resolution.ts schema.json policy.json test.ts fixture.json fixture-schema.json oracle.py staged-current/; cp receipt/receipt.mjs receipt/receipt.d.mts staged-current/
export GK_ROOT=/private/tmp/gk/sb GK_LIBRARY="/private/tmp/gk/sb/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library" GK_STAGED=/private/tmp/gk/staged-current
python3 tools/apply.py --apply | tail -2
rm -rf sb/.git; git init -q sb
python3 - <<'P'
import subprocess
ms=[m for m in open('/private/tmp/gk/manifests.txt',encoding='utf8').read().split('\n') if m]
subprocess.run(['git','-C','/private/tmp/gk/sb','add','-f','--','nx.json','bunfig.toml']+ms,check=True,capture_output=True)
P
echo rebuilt
