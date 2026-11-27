#!/bin/sh
T="$(cd "$(dirname "$0")" && pwd)"
cd "$T/../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990" || exit 2
n=${1:-4}
i=21
while [ "$i" -lt $((21+n)) ]; do
  L="/private/tmp/claude-501/-Users-ueli-Documents-semio/9ea38fee-6468-41de-903a-f7fec4948a18/scratchpad/en1990-replay-run$i.log"
  repo capacity run --memory=2663 --cpu=1 -- repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 -- test --offline --locked -p semio-s-artifact-norm-en1990-mutations --test replay_preparation_physical > "$L" 2>&1
  rc=$?
  echo "exit $rc" >> "$L"
  grep -qE 'host warden cancelled|E0277|E0599|E0433|E0432|E0425|E0308|source boundary' "$L" || break
  i=$((i+1))
done
