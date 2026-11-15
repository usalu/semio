#!/bin/bash
# full rebuild of one proof asset: figure-all.sh <slug>   (builds 50/100, 200/500 with refit, sleeper duvet post-step, prints the failing measurements)
source "$(dirname "$0")/author-33-env.sh"; cd "$(dirname "$0")"
A=humans/$1
for sc in 50 100; do $BUN figure-build.ts $A --scale $sc --write | tail -1; done
for sc in 200 500; do $BUN figure-build.ts $A --scale $sc --write --refit | tail -2; done
if [ "$1" = "sleeping-person" ]; then for sc in 50 100 200 500; do $BUN figure-duvet.ts $A --scale $sc --write; done; fi
./figure-fails.sh $A | cut -c1-330
