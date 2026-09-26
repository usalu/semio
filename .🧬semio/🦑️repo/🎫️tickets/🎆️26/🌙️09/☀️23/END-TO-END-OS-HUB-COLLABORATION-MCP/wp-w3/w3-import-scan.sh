#!/bin/zsh
# 🔍 Lists every component-dev/-release deliverable's function-bearing imports outside the browser-actor admission (quick triage).
cd /Users/ueli/Documents/semio
for f in $(find ✏️s -path '*/dist/component-*/*.wasm' -not -path '*/node_modules/*' | sort); do
  bad=$(node_modules/.bin/jco wit "$f" 2>&1 | /usr/bin/grep '^ *import ' | sed 's/^ *import \(.*\);/\1/' | /usr/bin/grep -v '^semio:framework/\(pure\|host-async\|types\|capabilities\|effects\|ui\|instance-lifetime\|events\)@1.0.0$' | /usr/bin/grep -v '^wasi:\(cli/\(environment\|exit\|stdin\|stdout\|stderr\|terminal-input\|terminal-output\|terminal-stdin\|terminal-stdout\|terminal-stderr\)\|clocks/\(monotonic-clock\|wall-clock\)\|io/\(error\|poll\|streams\)\|random/\(insecure-seed\|random\)\)@0\.2\.[0-9]*$' | tr '\n' ' ')
  [ -n "$bad" ] && echo "$f :: $bad"
done
echo SCAN-DONE
