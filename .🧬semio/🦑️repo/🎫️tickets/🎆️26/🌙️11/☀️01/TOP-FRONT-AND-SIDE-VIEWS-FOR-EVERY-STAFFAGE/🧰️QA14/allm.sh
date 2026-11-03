cd "$(dirname "$0")"
for a in steel-column sliding-door tilt-window tied-canopy wheelchair-ramp stair-flight spiral-stair; do for s in 500 200 100 50; do python montage.py $a $s 110 3 2>&1 | grep -v -i warn; done; done
echo ALLDONE
