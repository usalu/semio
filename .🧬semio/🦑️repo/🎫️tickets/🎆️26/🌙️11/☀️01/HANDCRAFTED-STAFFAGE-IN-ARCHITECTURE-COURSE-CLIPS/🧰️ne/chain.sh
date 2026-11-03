#!/bin/bash
D="$(dirname "$0")"
R="$D/render.sh"
O="🎯️optimization/🎬️clip"
"$R" "$O/📉️extrema/🐍️.py" Beat1_LocalGlobalExtrema
"$R" "$O/🧪️experiments/🐍️.py" Beat3_FactorialExperiments
"$R" "$O/⚖️index/🐍️.py" Beat4_NormalizedIndex
"$R" "📊️excel/🎬️clip/📋️program/🐍️.py" Beat3_Program
"$R" "🏗️bim/🎬️clip/🔗️consistency/🐍️.py" Beat6_Consistency
"$R" "🏗️bim/🎬️clip/📊️information/🐍️.py" Beat1_DataToInformation
"$R" "🏗️bim/🎬️clip/🔌️exchange/🐍️.py" Beat5_Interfaces
"$R" "$O/🎲️sampling/🐍️.py" Beat2_GridRandomSampling
"$R" "$O/🏆️pareto/🐍️.py" Beat5_ParetoFront
"$R" "$O/🧬️algorithms/🐍️.py" Beat6_EvolutionarySearch
echo ALL >> "$D/../🗑️generated/ne/done.log"
