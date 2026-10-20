#!/bin/sh
cd /private/tmp/runtime-seam/clone
SEMIO_REPOSITORY_ROOT=/private/tmp/runtime-seam/clone nice -n 10 bun test "./🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🟦️.ts" 2>&1 | tail -40
