#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS"
R=/Users/ueli/Documents/semio
L="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️24/COMPLETE-CONSOLIDATED-BORDER-AND-IN-CONTENT-CHIPS-IN-LA-TE-X-TEMPLATES"
REP="$R/🧰️framework/🛍️products/📓️print/🧾️template/📋️report"
export SEMIO_PRINT_SCRATCH=/private/tmp/claude-501/semio-print2
cd "$T"
b() { lockf -k /private/tmp/claude-501/semio-manim.lock node print-build.ts "$@" >> "$T/🗑️generated/fixtures.txt" 2>&1; }
: > "$T/🗑️generated/fixtures.txt"
b "$L/junctions" junctions.tex junctions 1,2,3 1.6
b "$REP" 📋️report.tex report-light 3,5 1.6
b "$REP" 📋️report.tex report-dark 3,5 1.6 dark
b "$T/print2-fixtures" assignment.tex assignment 1 1.6
echo ALLDONE >> "$T/🗑️generated/fixtures.txt"
