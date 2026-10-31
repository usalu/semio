#!/bin/zsh
cd /Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator
node --experimental-strip-types --no-warnings --input-type=module -e "import('./🏗️builder/🌐️vite/🟦️.ts').then(()=>console.log('OK'),e=>console.log(e.stack.slice(0,500)))" 2>&1 | python3 -c "import sys,urllib.parse;print(urllib.parse.unquote(sys.stdin.read())[:700])"
