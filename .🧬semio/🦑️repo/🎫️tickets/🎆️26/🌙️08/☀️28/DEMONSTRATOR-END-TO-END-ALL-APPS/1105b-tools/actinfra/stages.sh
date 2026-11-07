#!/bin/bash
cd /Users/ueli/Documents/semio
export SEMIO_SCRIPT_PROVIDERS='{"version":1,"providers":{"repository-tooling":"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts"}}'
exec bun "$(dirname "$0")/${STAGE_SCRIPT:-stages.ts}" "$@"
