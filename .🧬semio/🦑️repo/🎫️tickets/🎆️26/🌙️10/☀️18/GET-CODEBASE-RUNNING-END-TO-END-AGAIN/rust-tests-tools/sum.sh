#!/bin/sh
# usage: sum.sh <log>: errors, failing tests, per-binary results
/usr/bin/grep -E '^(error|Running|     Running|test result|failures:|---- |test .* FAILED|thread .* panicked|error: test failed|error: could not compile)|: error' "$1" | cut -c1-400
