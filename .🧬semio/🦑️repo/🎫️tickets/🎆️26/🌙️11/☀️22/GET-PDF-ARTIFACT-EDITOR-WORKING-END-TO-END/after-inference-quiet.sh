#!/bin/sh
# usage: after-inference-quiet.sh  (waits until the inference module has had no edit for 15 minutes, then reruns the strict checks of dag and note and the emit runs of cad and demonstrator, one after the other through the memory gate)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
inference="$root/🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference"
while [ -n "$(find "$inference" -name '*.rs' -mmin -15 2>/dev/null | head -1)" ]; do sleep 60; done
rm -f "$here"/🗑️generated/vf3.summary "$here"/🗑️generated/em8.summary
sh "$here/refresh-plugin-descriptors.sh" em8 semio-hub-cad-component semio-hub-demonstrator-component
sh "$here/verify-plugin-descriptors.sh" vf3 semio-hub-dag-component semio-hub-note-component
