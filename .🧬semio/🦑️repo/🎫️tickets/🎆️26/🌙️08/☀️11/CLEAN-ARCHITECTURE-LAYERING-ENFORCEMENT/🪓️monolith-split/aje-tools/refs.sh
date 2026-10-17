#!/bin/sh
# Lists every tracked and untracked text reference to a needle. Usage: refs.sh <needle>
cd /Users/ueli/Documents/semio || exit 2
git -c core.quotepath=false -c core.fsmonitor=false grep -n -I -F --untracked -e "$1" -- . ':(exclude).🧬semio/*' ':(exclude)*/🗑️generated/*' ':(exclude)*.lock' ':(exclude)bun.lock' 2>&1 | cut -c1-400
