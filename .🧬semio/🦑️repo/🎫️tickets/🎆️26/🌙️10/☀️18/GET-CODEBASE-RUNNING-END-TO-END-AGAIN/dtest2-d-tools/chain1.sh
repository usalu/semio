#!/bin/sh
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
sh "$C/dtest2-d-tools/retry.sh" p14 --tool=bazel run @semio-tech/repo-test-domain:test-artifact-root-source
sh "$C/dtest2-d-tools/retry.sh" p15 --tool=bazel run @semio-tech/repo-codebase-go:test
