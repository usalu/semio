#!/bin/bash
# 🔎️ Runs one bun test file and prints each failing test with its failing source line and the first expectation message. usage: tsfail.sh <./path.ts>
cd /Users/ueli/Documents/semio
perl -e 'alarm 240; exec @ARGV' -- bun test "$1" 2>&1 | /usr/bin/grep -E '^\(fail\)|^error:|^Expected|^Received|^ +at <anonymous>| pass$| fail$' | cut -c1-330
