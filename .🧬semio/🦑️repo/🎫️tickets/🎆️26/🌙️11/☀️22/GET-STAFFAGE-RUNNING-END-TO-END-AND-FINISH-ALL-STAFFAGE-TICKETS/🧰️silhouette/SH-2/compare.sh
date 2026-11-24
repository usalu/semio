#!/bin/zsh
# 🪞️ Before and after pictures of people at the given scales (see compare.ts): compare.sh <scale> <slug>…
source "${0:A:h}/env.sh"
cd "$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE" && for a in "${@:2}"; do "$B" "$W/compare.ts" "$a" "$1"; done
