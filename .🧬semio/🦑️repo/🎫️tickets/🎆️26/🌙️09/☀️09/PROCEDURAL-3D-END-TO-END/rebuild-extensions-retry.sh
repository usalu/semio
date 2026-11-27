#!/bin/sh
# Rebuilds the generation3d flow extension components until success; each attempt logs under the ticket generated folder.
cd "$(dirname "$0")/../../../../../../.." || exit 1
T="$(pwd)/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated"
for project in flow-extension-math-rust flow-extension-brep-rust; do
  n=0
  until [ $n -ge 12 ]; do
    n=$((n+1))
    repo run "@semio-tech/$project:package" -c prototype --exclude-dependencies > "$T/retry-$project-$n.txt" 2>&1 && { echo "$project ok attempt $n" >> "$T/retry-status.txt"; break; }
    echo "$project failed attempt $n" >> "$T/retry-status.txt"
    sleep 120
  done
done
echo done >> "$T/retry-status.txt"
