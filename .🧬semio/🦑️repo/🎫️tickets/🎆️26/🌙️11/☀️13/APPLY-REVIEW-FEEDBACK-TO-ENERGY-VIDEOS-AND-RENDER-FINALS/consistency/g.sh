#!/usr/bin/env bash
cd /c/git/semio
E="🎓️teaching/🏛️architecture/⚡️energy"
rg -n --glob '!**/.venv/**' --glob '!**/__pycache__/**' --glob '!**/🖼️posters/**' --glob '!**/.🧬semio/**' --glob '!**/venv/**' -g '*.py' -g '*.json' -g '*.md' "$1" "$E"/{🧲️physics,🌬️ventilation,🔥️heating,❄️cooling,📊️demand,🏗️authoring/🎨️visuals,🏗️authoring/📏️running-example} 2>/dev/null | cut -c1-${2:-260}
