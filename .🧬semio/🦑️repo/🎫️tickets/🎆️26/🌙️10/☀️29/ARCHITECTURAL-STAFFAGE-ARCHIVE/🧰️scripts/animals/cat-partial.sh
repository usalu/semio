#!/usr/bin/env bash
# [scratch] partial assembly (head, rig, anchors) of the cat for kinematics
D="$(cd "$(dirname "$0")" && pwd)/domestic-cat"
{ printf '{"assets":['; cat "$D/00-head.json" "$D/10-rig.json" "$D/20-anchors.json"; printf '"x":0}]}'; } > "$1"
