#!/usr/bin/env bash
set -eu
ANY="/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
for f in "$ANY/🧬️schema/🔣️.json" "$ANY/🧬️schema/📸️snapshot/🔣️.json"; do
  sed -i 's|"reliabilityClass":{"type":"string","enum":\["RC1","RC2","RC3"\],"x-semio-state":"artifact"}|"reliabilityClass":{"type":"string","enum":["RC1","RC2","RC3"],"x-semio-state":"artifact","x-semio-rust":{"type":"semio_s_plugin_norm_compliance::ReliabilityClass"}}|' "$f"
done
sed -i 's/"annex": "En"/"annex": "De"/' "$ANY/🧪️tests/🧫️fixtures/🧬️mutations/🎯change-reliability-class/✅apply/🔣️.after.json"
