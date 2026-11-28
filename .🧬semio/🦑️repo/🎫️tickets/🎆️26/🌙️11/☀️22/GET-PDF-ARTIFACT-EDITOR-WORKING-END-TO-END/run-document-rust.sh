#!/bin/sh
cd "$(dirname "$0")/../../../../../../.." || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END"
sh $T/native-cargo.sh test -p semio-s-artifact-stdio-pdf-mut-family-11 -p semio-s-artifact-stdio-pdf-testkit --lib -- output_intents_without_a_profile real_world_documents_lift_like --test-threads=1 > $T/🗑️generated/oi-test.log 2>&1
echo "exit lift $?" >> $T/🗑️generated/oi-test.log
sh $T/native-cargo.sh test -p semio-s-artifact-stdio-pdf-page-editor-tests --lib -- --test-threads=1 > $T/🗑️generated/editor-test.log 2>&1
echo "exit editor $?" >> $T/🗑️generated/editor-test.log
