set -u
BUN=$(ls /home/ueli/.cache/semio/runtimes/bun/*/bun | head -1)
W=$(mktemp -d /tmp/lx.XXXX); cd $W
echo '{"a":1}' > "🔣️.json"; echo '{"a":1}' > plain.json
for f in "$W/🔣️.json" "$W/plain.json"; do "$BUN" -e 'import {readFile} from "node:fs/promises"; try{console.log(JSON.parse(await readFile(process.argv[1],"utf8")))}catch(e){console.log("ERR",e.message)}' "$f"; done
"$BUN" --version
rm -rf $W
