#!/bin/bash
cd "$(dirname "$0")"
for a in vegetation/japanese-maple vegetation/lombardy-poplar vegetation/london-plane-pollarded vegetation/multi-stem-birch vegetation/scots-pine vegetation/miscanthus-grass vegetation/small-leaved-lime vehicles/city-bicycle vehicles/motor-scooter; do ./qa2-run.sh $a >/dev/null 2>&1; ./qa2-mont.sh ${a#*/} >/dev/null 2>&1; echo "$a $(date +%H:%M:%S)" >> 🗑️generated/qa2/progress2.txt; done
