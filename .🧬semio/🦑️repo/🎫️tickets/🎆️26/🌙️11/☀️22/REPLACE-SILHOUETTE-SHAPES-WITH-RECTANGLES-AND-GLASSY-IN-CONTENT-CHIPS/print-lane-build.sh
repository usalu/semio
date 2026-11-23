#!/bin/bash
# usage: build.sh main  [outdir]
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/0fec7092-b872-4c05-9d02-7a8aa493f8d0/scratchpad
R=/Users/ueli/Documents/semio
P="$R/🧰️framework/🛍️products/📓️print"; B=$R/.cache/b/t/m/print; F=$R/.cache/b/p/m/print/prototype/fonts
cd $S/w; out=${2:-$S/out}; mkdir -p $out
env HOME=$S TECTONIC_CACHE_DIR=$S/cache SOURCE_DATE_EPOCH=1782864000 TZ=UTC TEXINPUTS="$P/🖋️latex//:$P//:$F//:" $B/tectonic/aarch64-apple-darwin/tectonic --bundle "file://$B/tex" --keep-logs -Z search-path="$P/🖋️latex" -Z search-path="$F" -Z search-path="$P/🖼️assets/🪧️logo" -Z search-path="$P/🔨️modules/🕸️graph/📐️.tex" --outdir $out $1.tex 2>&1 | tail -${3:-15}
