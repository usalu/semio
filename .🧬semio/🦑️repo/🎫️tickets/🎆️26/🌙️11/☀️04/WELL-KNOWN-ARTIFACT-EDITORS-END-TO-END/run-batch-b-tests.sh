#!/usr/bin/env bash
A="/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
O="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END/🗑️generated"
run() { local dir="$1" name="$2"; shift 2; (cd "$dir" && repo cargo --store=shared --configuration=prototype -- test "$@") > "$O/b08-$name.txt" 2>&1; echo "exit $?" >> "$O/b08-$name.txt"; }
run "$A/📚️idml" idml -p semio-s-artifact-stdio-idml-story -p semio-s-artifact-stdio-idml-any --lib
run "$A/📜️docx" docx -p semio-s-artifact-stdio-docx-base --lib custom_xml_with_comments
run "$A/📕️xlsx" xlsx -p semio-s-artifact-stdio-xlsx-base --lib custom_xml_with_comments
run "$A/📽️pptx" pptx -p semio-s-artifact-stdio-pptx-base --lib custom_xml_with_comments
run "$A/📚️idml" idml-rerun -p semio-s-artifact-stdio-idml-story -p semio-s-artifact-stdio-idml-any --lib
run "$A/📊️csv" csv -p semio-s-artifact-stdio-csv-io-registry --lib editor
run "$A/📑️tsv" tsv -p semio-s-artifact-stdio-tsv-io-registry --lib editor
echo done > "$O/b08-script-done"
