#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
mkdir -p "$T/🗑️generated"
for f in "$@"; do
  d=$(cd "$R" && ls -d *"$f")
  cd "$R/$d" || exit 1
  repo cargo --store=shared --configuration=prototype -- test -p "semio-s-artifact-norm-$f-editor" --offline history_edit_inputs_resolve > "$T/🗑️generated/labels-$f.log" 2>&1
  echo "exit $?" >> "$T/🗑️generated/labels-$f.log"
  touch "$T/🗑️generated/labels-$f.done"
done
