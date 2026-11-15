#!/bin/zsh
# usage: rehearse.sh : copies the live originals of every file the landing touches into a scratch root, lands, lands again, rolls back, compares
S=${0:A:h}
LAND="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R40/land.py"
case "$S" in /private/tmp/claude-501/*) ;; *) echo "scratch only"; exit 2;; esac
rm -rf $S/rh && mkdir -p $S/rh/live $S/rh/overlay
python3 -I "$LAND" --overlay $S/rh/overlay/ov --merge > /dev/null || exit 1
cd $S/rh/overlay/ov && find . -type f | sed 's|^\./||' > $S/rh/files.txt
cd $S/rh && while IFS= read -r f; do if [[ -f "/Users/Shared/semio/$f" ]]; then mkdir -p "live/${f:h}"; cp "/Users/Shared/semio/$f" "live/$f"; fi; done < files.txt
cp -R live orig
echo "files in plan: $(wc -l < files.txt), live originals copied: $(find live -type f | wc -l)"
echo "== land (all four sets, R29 owns the stamp)"
python3 -I "$LAND" --land --root $S/rh/live --authorized --no-tests --merge | tail -4
echo "== compare with overlay"; diffs=0; while IFS= read -r f; do cmp -s "live/$f" "overlay/ov/$f" || { echo "DIFF $f"; diffs=$((diffs+1)); }; done < files.txt; echo "differences: $diffs"
echo "== second run"; python3 -I "$LAND" --land --root $S/rh/live --authorized --no-tests --merge | tail -2
B=$(find live -type d -name "R40-landing-*" | head -1); echo "backup: ${B}"
echo "== rollback"; python3 -I "$LAND" --rollback "$S/rh/$B" --root $S/rh/live | tail -1
d=0; while IFS= read -r f; do if [[ -f "orig/$f" ]]; then cmp -s "live/$f" "orig/$f" || { echo "NOT RESTORED $f"; d=$((d+1)); }; else [[ -e "live/$f" ]] && { echo "NEW FILE LEFT $f"; d=$((d+1)); }; fi; done < files.txt; echo "rollback differences: $d"
