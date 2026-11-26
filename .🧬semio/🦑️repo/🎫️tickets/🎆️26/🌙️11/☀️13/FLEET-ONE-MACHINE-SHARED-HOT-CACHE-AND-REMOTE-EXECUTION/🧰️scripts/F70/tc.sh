#!/bin/zsh
cd /Users/ueli/Documents/semio
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
args=()
for a in "$@"; do args+=("$B/$a"); done
repo typecheck --json "${args[@]}" 2>&1 | python3 -c "
import sys,json,re;t=sys.stdin.read();i=t.index('{');d=json.loads(t[i:]);print(d['status'],d['errors'],d['elsewhere'],len(d['drift']),d['importers'])
for x in d['diagnostics']:print(re.sub(r'.*/🏗️build/','',x['file']),x['line'],x['message'][:${W:-260}])"
