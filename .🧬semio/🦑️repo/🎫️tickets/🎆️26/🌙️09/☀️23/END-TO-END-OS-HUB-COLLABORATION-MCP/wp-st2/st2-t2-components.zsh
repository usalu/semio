#!/bin/zsh
# 🧩️ ST2 T2 law (wasm lane, one job): stdio `editor-component-check` — links and validates every stdio package component
# (wasm-release cdylib → wasm-component-ld → jco transpile → componentizable core), the 1M-function ceiling proof per package.
# usage: zsh st2-t2-components.zsh <capture>   (detached via w2-detach.py)
setopt no_bg_nice
capture="${1:A}"
out=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-st2-components
export NX_DAEMON=false
cd '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust' || exit 1
{
  echo "[st2-comp] QUEUED $(date '+%F %T') pid $$"
  zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm st2 -- zsh -c "echo \"[st2-comp] HOLD \$(date '+%F %T') jobs=\$CARGO_BUILD_JOBS\"; nice -n 15 bun ./📜️script.ts editor-component-check '$out'; echo \"[st2-comp] component check rc=\$? \$(date '+%F %T')\""
  echo "[st2-comp] END rc=$? $(date '+%F %T')"
} > "$capture" 2>&1
