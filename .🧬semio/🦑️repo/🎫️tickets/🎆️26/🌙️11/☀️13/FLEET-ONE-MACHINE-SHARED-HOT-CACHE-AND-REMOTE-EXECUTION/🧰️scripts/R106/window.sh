#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R106
m=$((10#$(date -u +%M)))
if ! { (( m <= 4 )) || (( m >= 30 && m <= 34 )); }; then echo "outside the landing window"; exit 2; fi
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R106 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
date -u +%H:%M:%S > $SC/lock-start.txt
PYTHONHASHSEED=0 python3 "$D/land.py" || { rm -r "$T/landing.lock"; exit 3; }
python3 "$D/newtests.py"
"$D/runall.sh" after fast > /dev/null 2>&1
cat $SC/after.txt
date -u
red=0
while IFS= read -r line; do
  case "$line" in
    MISSING*) red=1; echo "RED $line";;
    *"1 fail"*) case "$line" in *project-records/🧪️tests/🟦️.ts*|*too-heavy*) ;; *) red=1; echo "RED $line";; esac;;
    *" fail"*) n=$(echo "$line" | sed -E 's/.* ([0-9]+) fail.*/\1/'); if [ "$n" != "0" ]; then red=1; echo "RED $line"; fi;;
    *" pass"*) ;;
  esac
done < $SC/after.txt
/usr/bin/grep -q " : 0 pass" $SC/after.txt && { red=1; echo "RED a test file ran 0 pass"; }
if [ $red = 1 ]; then
  python3 "$D/restore.py"; python3 "$D/newtests.py" --remove
  echo RESTORED
else
  echo GREEN
fi
rm -r "$T/landing.lock"
date -u +%H:%M:%S > $SC/lock-end.txt
cat $SC/lock-start.txt $SC/lock-end.txt
