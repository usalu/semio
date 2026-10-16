#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad
rm -rf $S/orig; mkdir -p $S/orig
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules" && tar -cf - 🌉️mcp | (cd $S/orig && tar -xf -)
