#!/bin/bash
S=$1; shift
M="spines-dome=dome,spines-skirt=skirt-rear-near+skirt-rear-far+skirt-front-near+skirt-front-far,face-fur=neck+head,nose=nose,limbs-fur=fore-n+hind-n+fore-f+hind-f"
[ "$S" = "50" ] && M="$M,belly-fur=belly,paws-skin=forepaw-n+hindpaw-n+forepaw-f+hindpaw-f"
./author-29-bun.sh kit-thermal.ts animals/hedgehog --scale $S --map "$M" "$@"
