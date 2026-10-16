#!/bin/zsh
# usage: run.sh <label> [bun|vitest|both]  -> $G/tfe-<label>-bun.xml, $G/tfe-<label>-vitest.json
R=/Users/ueli/Documents/semio
G=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated
P=$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript
LABEL=$1; WHAT=${2:-both}
if [ "$WHAT" != vitest ]; then
  cd $R
  files=("${(@f)$(cat $G/tfe-run-bun.txt)}")
  perl -e 'alarm 1500; exec @ARGV' -- bun test --timeout 120000 --reporter=junit --reporter-outfile="$G/tfe-$LABEL-bun.xml" "${files[@]}" > "$G/tfe-$LABEL-bun.log" 2>&1
  echo "exit $?" >> "$G/tfe-$LABEL-bun.log"
fi
if [ "$WHAT" != bun ]; then
  mkdir -p "$G/tfe-vitest-cache"
  export SEMIO_TEST_LEVEL=exhaustive
  export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/tfe-vitest-cache\",\"coverageDirectory\":\"$G/tfe-vitest-cov\",\"budgetMs\":3300000}"
  filters=("${(@f)$(cat $G/tfe-run-vitest-filters.txt)}")
  cd "$P"
  perl -e 'alarm 3400; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "../../🧪️tests/🎚️config/🟦️.ts" --testTimeout 120000 --reporter=json --outputFile="$G/tfe-$LABEL-vitest.json" "${filters[@]}" "UiDocumentStore/" "PluginRuntime/" > "$G/tfe-$LABEL-vitest.log" 2>&1
  echo "exit $?" >> "$G/tfe-$LABEL-vitest.log"
fi
echo done > "$G/tfe-$LABEL-done"
