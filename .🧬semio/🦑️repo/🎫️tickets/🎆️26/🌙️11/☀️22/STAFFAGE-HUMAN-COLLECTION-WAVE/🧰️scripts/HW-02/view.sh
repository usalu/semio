#!/bin/zsh
# 🖼️ Renders one asset file at given scales and viewpoints with the preview CLI and rasterises the SVGs (sharp).
# view.sh <file.json> <outdir> <pose|clip@t> <scales "50 100"> <viewpoints "elevation front plan"> [view light-line] [widthPx]
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
H=${0:A:h}
file=${1:A}; out=${2:A}; q=$3; scales=(${=4}); vps=(${=5}); view=${6:-light-line}; px=${7:-1400}
mkdir -p $out
asset=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['asset']['id'])" $file)
if [[ $q == *@* ]]; then sel=(--clip ${q%@*} --time ${q#*@}); else sel=(--pose $q); fi
for s in $scales; do for vp in $vps; do
  (cd $S && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset $asset --file $file --scale $s $sel --view $view --viewpoint $vp --no-png --out $out >/dev/null 2>&1) || echo "[DEBUG] preview failed $s $vp"
done; done
$B $H/png.ts $px $out/*.svg >/dev/null && ls $out/*.png
