#!/bin/zsh
# usage: imp.sh <needle>  -> tracked + untracked files (non-ticket) containing the fixed needle
export LC_ALL=en_US.UTF-8
cd /Users/ueli/Documents/semio
git -c core.quotepath=false grep -n -F -e "$1" -- . ':(exclude).🧬semio' ':(exclude)*.d.ts' 2>/dev/null
git ls-files --others --exclude-standard -z -- . ':(exclude).🧬semio' 2>/dev/null | xargs -0 /usr/bin/grep -n -F -e "$1" 2>/dev/null | /usr/bin/grep -v '^\.🧬semio'
