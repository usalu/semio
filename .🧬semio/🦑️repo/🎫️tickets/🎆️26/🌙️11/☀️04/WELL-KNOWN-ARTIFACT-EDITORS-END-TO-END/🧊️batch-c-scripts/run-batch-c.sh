#!/bin/bash
R="/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"; G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END/🗑️generated"
run() { cd "$R/$1" && repo cargo --store=shared --configuration=prototype -- test -p "$2" --lib "$3" > "$G/c-$2.txt" 2>&1; echo "EXIT $? $2 $3" >> "$G/c-summary.txt"; }
: > "$G/c-summary.txt"
run 🪟️bmp semio-s-artifact-stdio-bmp-editor-2 editor::bmp
run 🎞️gif semio-s-artifact-stdio-gif-editor editor
run 📸️jpg semio-s-artifact-stdio-jpg-io-registry editor
run 🖼️tiff semio-s-artifact-stdio-tiff-mutation-regressions editor
run 📷️png semio-s-artifact-stdio-png-tests editor::png
