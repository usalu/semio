#!/bin/zsh
# 🧪️ LB2 p20 scratch prep: clone gis + vcs (semio-hub's native-artifact-execution closure) into the scratch and make the hub crate a
# scratch workspace member, so `-p semio-hub` builds there. Scratch-only; refuses the live tree.
SC="$1"; R="/Users/ueli/Documents/semio"
[[ "$SC" == "$R" || -z "$SC" ]] && { echo "scratch-only"; exit 2; }
for p in 🌍️gis 🌿️vcs; do rm -rf "$SC/✏️s/🔌️plugins/$p" && cp -c -R "$R/✏️s/🔌️plugins/$p" "$SC/✏️s/🔌️plugins/$p" || exit 2; done
python3 - "$SC/Cargo.toml" <<'PY'
import sys
path = sys.argv[1]
text = open(path, encoding="utf-8").read()
member = '    "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust",\n'
if "🌎️hub/📦️packages/🦀️rust" not in text:
    text = text.replace(member, member + '    "🌎️hub/📦️packages/🦀️rust",\n', 1)
open(path, "w", encoding="utf-8").write(text)
print("members:", text[text.index("members"):text.index("]", text.index("members")) + 1].replace("\n", " "))
PY
