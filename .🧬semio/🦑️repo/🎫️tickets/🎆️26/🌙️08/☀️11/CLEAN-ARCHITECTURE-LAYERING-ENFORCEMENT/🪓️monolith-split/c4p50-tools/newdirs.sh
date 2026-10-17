#!/bin/zsh
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p50-tools/env.sh
# lists new child folders (relative to R) of the five modules
for p in "⏯️tool-run:🚦️lifecycle 📊️progress 🗄️trace-store 🎞️tick 🔤️codec 🧾️json 🪧️panel 🏷️actions-labels" "📇️directory/🧬️schema:📜️event 📄️event-page 🎫️command 🛠️admin 👀️views 📋️admin-page 📂️document-open 🎯️execution-target 🧊️artifact-checkpoints 🪢️checkpoint-pair 🌊️stream" "🔄️machine:🔎️inspect 🧠️kernel 🌐️host 💾️persist 🎬️runtime 🔁️step 🧭️testing" "🛰️backbone:🧭️opening-relay 🌐️envelope-io 🔀️message 🪟️desktop-window 🔑️persistence-binding 🎭️artifact-actor 🧱️worker-wire 🔬️worker-parsers 📨️worker-messages 🗺️workflow-planner" "🧾️app-channel:🏷️codec-primitives 🎛️command-codec 🎞️frame-codec 📋️report-decoders 🔢️request-sequence 🗃️archive-load-host"; do
  base=${p%%:*}; kids=${p#*:}
  for k in ${=kids}; do echo "🧰️framework/🔨️modules/$base/$k"; done
done
