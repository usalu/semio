#!/bin/zsh
# 🦾️ Repair ops of `excavator` of the second round: the volumes of 1:100 and 1:200 in length (excavator.py), then the side elevation of the track as the solids of the undercarriage project together (shoes, grousers, rails, idler, sprocket and its teeth, rollers, frame, yokes and adjuster with the open bays between them), in place of the smooth belt that was fuller than the body.
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 excavator.py && cd "$W"
t() { "$B" trace.ts excavator "$@" | tail -1 }
t 500 elevation "track-shoes,idler,idler-hub,idler-yoke-*,sprocket,sprocket-hub,roller-*,carrier-roller-*,track-frame" --id track-belt --bone undercarriage --like track-belt --replace track-belt --holes
t 200 elevation "track-shoes,track-links,idler,idler-hub,idler-yoke-*,idler-cross-pin,sprocket,sprocket-*,roller-*,carrier-*,track-frame,track-adjuster-*" --id track-belt --bone undercarriage --like track-belt --replace track-belt --holes
t 100 elevation "track-*,grouser-*,idler,idler-hub,idler-yoke-*,idler-cross-pin,sprocket,sprocket-*,roller-*,carrier-*" --id track-belt --bone undercarriage --like track-belt --replace track-belt --holes
(cd author && python3 excavator-parts.py)
"$B" drawable.ts excavator --ops ops/excavator.json
