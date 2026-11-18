#!/bin/bash
# 🚗 Fits the three views of one asset scale in order elevation, front, plan from a pre-ops file. humansb-B-drive.sh <asset> <scale> <pre.json> <solidsE> <solidsF> <solidsP> [tol mm] [rounds]
H="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
G="$H/🗑️generated"; R="$H/humansb-B-run.sh"
a=$1; s=$2; pre=$3; se=$4; sf=$5; sp=$6; tol=${7:-4}; rounds=${8:-3}
$R humansb-B-fit.ts $a $s elevation "$G/humansb-B-$a-$s-e.json" --in "$pre" --rounds $rounds --tol $tol --solids "$se" 2>&1 | tail -1
$R humansb-B-fit.ts $a $s front "$G/humansb-B-$a-$s-f.json" --in "$G/humansb-B-$a-$s-e.json" --rounds $rounds --tol $tol --solids "$sf" 2>&1 | tail -1
$R humansb-B-fit.ts $a $s plan "$G/humansb-B-$a-$s-p.json" --in "$G/humansb-B-$a-$s-f.json" --rounds $rounds --tol $tol --solids "$sp" 2>&1 | tail -1
$R humans-try.ts $a "$G/humansb-B-$a-$s-p.json" $s
