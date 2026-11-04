#!/usr/bin/env bash
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END/🗑️generated"
run() { (cd "$1" && repo cargo --store=shared --configuration=prototype -- test "${@:3}" > "$G/$2.log" 2>&1); }
run "/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip" h-zip-editor -p semio-s-artifact-stdio-zip-editor --lib
run "/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip" h-zip-opc -p semio-s-artifact-stdio-zip-opc --lib
run "/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate" h-deflate -p semio-s-artifact-stdio-deflate-mutations --lib
run "/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary" h-binary -p semio-s-artifact-stdio-binary-mutations --lib
run "/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio" h-semio-drawing -p semio-s-artifact-stdio-semio-drawing-editor --lib
