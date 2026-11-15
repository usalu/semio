#!/bin/sh
# Runs volumes-oracle.ts on a fixed set of 12 assets across categories; log to 🗑️generated/volumes-tools/oracle-12.txt
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
: > 🗑️generated/volumes-tools/oracle-12.txt
for a in animals/horse animals/sheep appliances/laptop appliances/gas-boiler components/glulam-beam components/panel-radiator furniture/park-bench furniture/single-bed humans/cyclist humans/standing-adult machinery/concrete-mixer-truck vehicles/city-bicycle vegetation/english-oak; do
  echo "=== $a" >> 🗑️generated/volumes-tools/oracle-12.txt
  timeout 400 $BUN volumes-oracle.ts "$a" --scale 100 2>&1 | cut -c1-260 >> 🗑️generated/volumes-tools/oracle-12.txt
done
echo DONE >> 🗑️generated/volumes-tools/oracle-12.txt
