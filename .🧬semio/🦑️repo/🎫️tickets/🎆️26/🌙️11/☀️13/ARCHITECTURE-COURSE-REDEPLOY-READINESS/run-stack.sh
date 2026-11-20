#!/bin/sh
# 🚀️ Starts the static stack on 8799/6068 from this ticket's generated/intro folder.
cd "$(dirname "$0")" || exit 1
export PATH=$HOME/.local/share/semio/node/v24.15.0/bin:$PATH
G=🗑️generated/intro
CHAOS_WORK=$G STATIC=1 PROCTOR_PORT=8799 SITE_PORT=6068 PROCTOR_EXECUTABLE="$PWD/$G/proctor-bin" exec node intro-stack.mjs
