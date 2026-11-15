#!/bin/bash
S=$1; shift
L=""; for p in far near; do L="$L"; done
case $S in
50|100) B="rump+flank-back+flank-belly+ribs-back+ribs-belly"; FACE="head"; [ "$S" = "50" ] && FACE="head+jaw+beard"
 M="coat-trunk=$B,coat-neck=neck,face=$FACE,ears=ear-n+ear-f,horns=horn-n+horn-f,tail=tail-1+tail-2,coat-upper-legs=far-thigh+far-gaskin+far-shoulder+far-humerus+far-forearm+near-thigh+near-gaskin+near-shoulder+near-humerus+near-forearm,lower-legs=far-cannon-h+far-cannon-f+near-cannon-h+near-cannon-f,hooves=far-hoof-h+far-hoof-f+near-hoof-h+near-hoof-f";;
200) M="coat-trunk=hindbody+chest,coat-neck=neck,face=head,ears=ear,horns=horn,tail=tail,coat-upper-legs=far-thigh+far-gaskin+far-humerus+far-forearm+near-thigh+near-gaskin+near-humerus+near-forearm,lower-legs=far-cannon-h+far-cannon-f+near-cannon-h+near-cannon-f,hooves=far-hoof-h+far-hoof-f+near-hoof-h+near-hoof-f";;
500) M="coat-trunk=hindbody+chest,coat-neck=neck,face=head,ears=ear,horns=horn,coat-upper-legs=far-haunch+far-shoulder+far-leg-hind-upper+far-leg-fore-upper+near-haunch+near-shoulder+near-leg-hind-upper+near-leg-fore-upper,lower-legs=far-leg-hind-lower+far-leg-fore-lower+near-leg-hind-lower+near-leg-fore-lower";;
esac
./author-29-bun.sh kit-thermal.ts animals/goat --scale $S --map "$M" "$@"
