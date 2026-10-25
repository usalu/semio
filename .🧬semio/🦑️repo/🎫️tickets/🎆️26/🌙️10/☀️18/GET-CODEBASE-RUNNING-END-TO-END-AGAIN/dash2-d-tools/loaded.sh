#!/bin/sh
# usage: loaded.sh <module-path>  lists the TypeScript modules a bare `import()` of the module loads (same trace as the door startup law)
cd /Users/ueli/Documents/semio || exit 1
d=$(mktemp -d)
cat > "$d/plugin.ts" <<'X'
import {readFileSync,writeFileSync} from "node:fs";const seen:string[]=[];Bun.plugin({name:"loaded-modules",setup(build){build.onLoad({filter:/\.(ts|mjs)$/},args=>{seen.push(args.path);return {contents:readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};});}});process.on("exit",()=>{writeFileSync(process.env.LOADED_MODULES_OUT!,JSON.stringify(seen));});
X
echo 'await import(process.argv[2]!);' > "$d/driver.ts"; : > "$d/bunfig.toml"
LOADED_MODULES_OUT="$d/out.json" SEMIO_LANGUAGE=en bun --config="$d/bunfig.toml" --preload "$d/plugin.ts" "$d/driver.ts" "$1" >/dev/null 2>&1
python3 -c "
import json,os,sys
l=json.load(open('$d/out.json'));print(len(l),sum(os.path.getsize(p) for p in l))
for p in l: print(os.path.getsize(p),p.replace('/Users/ueli/Documents/semio/',''))"
rm -rf "$d"
