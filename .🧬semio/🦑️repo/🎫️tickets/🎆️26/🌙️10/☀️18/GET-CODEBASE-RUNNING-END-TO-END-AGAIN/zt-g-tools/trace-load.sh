#!/bin/sh
# usage: trace-load.sh <out.json> <door args...>: modules a door invocation loads (same plugin as the startup law)
out="$1"; shift
d="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$d/../../../../../../../.." && pwd)"
printf '' > "$d/../🗑️generated/zt-g/s/bunfig.toml"
cat > "$d/../🗑️generated/zt-g/s/plugin.ts" <<'EOP'
import {readFileSync,writeFileSync} from "node:fs";const seen:string[]=[];Bun.plugin({name:"loaded-modules",setup(build){build.onLoad({filter:/\.(ts|mjs)$/},args=>{seen.push(args.path);return {contents:readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};});}});process.on("exit",()=>{writeFileSync(process.env.LOADED_MODULES_OUT!,JSON.stringify(seen));});
EOP
SEMIO_LANGUAGE=en SEMIO_REPO_IMPLEMENTATION=live LOADED_MODULES_OUT="$out" bun --config="$d/../🗑️generated/zt-g/s/bunfig.toml" --preload "$d/../🗑️generated/zt-g/s/plugin.ts" "${DOOR_SCRIPT:-$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts}" "$@" >/dev/null 2>&1
echo "exit $?"
