#!/bin/sh
# usage: sum.sh <log> : errors, panics, failures and test results only
/usr/bin/grep -E ': error|^error|panicked|test result|FAILED|failed|left:|right:|Running |^---- |compile_error|could not compile' "$1" | /usr/bin/grep -v 'warning' | cut -c1-700
