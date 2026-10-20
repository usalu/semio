#!/bin/zsh
# usage: pone.sh "<file>|<name>|<line>|<lines>" : pgo apply with automatic revert when tsdiff is not clean or the largest piece stays >= 90% of the original
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio
IFS='|' read -r f n l ln <<< "$1"
N=$(echo "$f:$n" | md5 | cut -c1-8); EXT="${f##*.}"
LOG=$T/logs/$N.log; mkdir -p $T/logs
perl -e 'alarm 560; exec @ARGV' -- $T/pgo.sh "$f" "$n" 150 apply > $LOG 2>&1
S=$(grep -c "^APPLIED" $LOG); TD=$(grep -c "TSDIFF CLEAN" $LOG); TI=$(grep -c "TITLES-IDENTICAL" $LOG); MX=$(grep -E "@[0-9]+ " $LOG | tail -1 | awk '{print $1}')
ERR=$(grep -o '"errors": \[[^]]*' $LOG | head -1 | cut -c1-160)
ACT=kept
if [[ $S == 1 ]] && { [[ $TD != 1 ]] || [[ -n "$MX" && $MX -ge $(( ln * 9 / 10 )) ]]; }; then $T/revert2.sh $T/specs/p-$N.json $T/orig/p-$N.$EXT > /dev/null 2>&1; ACT=REVERTED; fi
echo "[$n $ln] applied=$S tsdiff=$TD titles=$TI max=$MX $ACT $ERR"
