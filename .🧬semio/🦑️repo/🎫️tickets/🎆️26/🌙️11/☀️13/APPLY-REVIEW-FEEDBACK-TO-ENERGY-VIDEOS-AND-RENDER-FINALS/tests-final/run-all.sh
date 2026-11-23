#!/bin/bash
# Usage: run-all.sh <results-name>; runs every module of modules.txt, three at a time, with the final profile.
cd /c/git/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
mkdir -p "$T/🗑️generated/tests-final/logs"
export RES="${1:-results-last.txt}" EXTRAENV="SEMIO_TEACHING_PROFILE=final SEMIO_TEACHING_LANGUAGE=en"
tr -d '\r' < "$T/tests-final/modules.txt" | grep -v '^\s*$' | xargs -P 3 -I{} bash "$T/tests-final/run-one.sh" "{}"
sort "$T/🗑️generated/tests-final/$RES" | awk '{print $1}' | uniq -c
