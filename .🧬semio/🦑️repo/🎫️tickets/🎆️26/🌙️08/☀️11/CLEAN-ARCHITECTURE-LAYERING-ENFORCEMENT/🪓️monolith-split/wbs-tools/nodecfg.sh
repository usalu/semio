#!/bin/zsh
R=/Users/ueli/Documents/semio
C="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript"
cd "$C"
SEMIO_BUILD_MODE=prototype SEMIO_NEXT_BUILD_DIRECTORY=../../../../../../../../.cache/b/p/m/wbs-next/next SEMIO_NEXT_TYPESCRIPT_PROJECT=../../../../../../../../.cache/b/p/m/wbs-next/tsconfig.json node --experimental-transform-types -e 'import("./next.config.mjs").then(m=>console.log("CONFIG-OK",Object.keys(m.default).join(",")),e=>{console.log(String(e.message).slice(0,600)+"\n"+(e.stack||"").split("\n").slice(1,4).join("\n"));process.exit(1)})' 2>&1 | /usr/bin/grep -v ExperimentalWarning
