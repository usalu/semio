#!/bin/zsh
cd /Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator
node --experimental-strip-types --no-warnings --input-type=module -e 'import("./🔨️modules/🧩️runtime/♻️activation/🟦️.ts").then(()=>console.log("OK"),e=>console.log(String(e)))' 2>&1 | python3 -c "import sys,urllib.parse;print(urllib.parse.unquote(sys.stdin.read())[:700])"
