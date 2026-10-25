#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sh "$C/ext-f-tools/run.sh" bzcomp repo --tool=bazel run @semio-tech/framework-compiler-rs:test-native
sh "$C/ext-f-tools/run.sh" bzpix repo --tool=bazel run semio-framework-pixels:test
