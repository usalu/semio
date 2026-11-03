#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
for c in 1991 1999 1990 1995 1997 1998; do
  "$T/🧰️scripts/compile-review-x/run.sh" $c s:resolve s:imports s:exprpaths s:visibility s:deps s:paths s:literals s:callargs s:structs s:fieldnames s:params s:receivers s:match s:format s:dupes s:kebab s:methods s:rowkeys s:assign s:strmeth t:twophase t:tempborrow t:nestedmut t:cmp t:derive t:move t:traitdupes t:valuederive
done
echo ALLDONE
