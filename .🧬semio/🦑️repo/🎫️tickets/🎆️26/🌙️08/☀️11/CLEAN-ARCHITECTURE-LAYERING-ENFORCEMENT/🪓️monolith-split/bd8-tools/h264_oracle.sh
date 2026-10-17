#!/bin/zsh
H="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🎞️h264"
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SB=/private/tmp/claude-501/scratch/h264sb
rm -rf $SB; mkdir -p $SB; cp -R "$H/." $SB/; rm -rf $SB/📦️packages
python3 - <<PY
p="$SB/🦀️.rs"; s=open(p).read()
a="use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;"
assert s.count(a)==1
stub="mod remodeling_image { #[derive(Clone, Debug, PartialEq)] pub struct ImageRgba8 { pub width: u32, pub height: u32, pub data: Vec<u8> } impl ImageRgba8 { pub fn new(width: u32, height: u32) -> Self { Self { width, height, data: vec![0; (width as usize) * (height as usize) * 4] } } } }"
open(p,"w").write(s.replace(a,stub))
PY
cd $SB && ulimit -n 65536 && nice -n 10 rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name h264sb --emit=metadata $@ -o /dev/null 🦀️.rs 2> "$F/🗑️generated/bd8-h264sb.err"
echo "exit=$? errors=$(/usr/bin/grep -c '^error' $F/🗑️generated/bd8-h264sb.err) warnings=$(/usr/bin/grep -c '^warning' $F/🗑️generated/bd8-h264sb.err)"
