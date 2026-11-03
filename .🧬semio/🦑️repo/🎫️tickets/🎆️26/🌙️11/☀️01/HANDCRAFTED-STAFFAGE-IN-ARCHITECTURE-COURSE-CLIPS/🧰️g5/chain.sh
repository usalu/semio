#!/usr/bin/env bash
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/g5"
r() { bash "$T/🧰️g5/render.sh" "$@" > "$G/r_$2.txt" 2>&1; echo "$2 exit $?" >> "$G/chain.log"; }
r "🔥️heating/🎬️clip/☀️solar-gains/🐍️.py" Beat5_Verschattung
r "../🎯️optimization/🎬️clip/🧪️experiments/🐍️.py" Beat3_FactorialExperiments
r "../🎯️optimization/🎬️clip/⚖️index/🐍️.py" Beat4_NormalizedIndex
r "../🎯️optimization/🎬️clip/📉️extrema/🐍️.py" Beat1_LocalGlobalExtrema
echo finished >> "$G/chain.log"
