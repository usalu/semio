#!/bin/zsh
# 📋️ The state of lane CL-T1 in a few lines: the finished jobs of both workers, and per asset the verdict of its light proof and the matrix, model-catalog and GLB lines of its proof. status.sh
source "${0:A:h}/env.sh"
date +%H:%M:%S; tail -n 3 "$GEN/log/jobs-done.txt" 2>/dev/null | cut -c1-90; tail -n 3 "$GEN/log/jobs2-done.txt" 2>/dev/null | cut -c1-90
for f in "$GEN"/light/*.txt(N); do a="${${f:t}%.txt}"; p="$GEN/proof/$a.txt"
echo "$a | $(grep -o 'LIGHT PROOF.*: [A-Z]*' "$f" | awk '{print $NF}') | $(grep -o 'glb of the default pose: [0-9]* bytes ([0-9]*e-2 MiB' "$f" | awk '{print $8}' | tr -d '(') | $(grep -o '[0-9]* cells: [0-9]* ok, [0-9]* unavailable, [0-9]* fail' "$p" 2>/dev/null) | $(grep -o 'model-catalog exit [0-9]* ([0-9]* s): [^;]*' "$p" 2>/dev/null) | $([[ -f "$GEN/proof/$a.proved" ]] && echo proved)"
done
