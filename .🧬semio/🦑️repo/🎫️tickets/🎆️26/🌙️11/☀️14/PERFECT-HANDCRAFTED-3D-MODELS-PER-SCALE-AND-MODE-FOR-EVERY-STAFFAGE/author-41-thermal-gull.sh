#!/bin/bash
S=$1; shift
case $S in
50) M="body-plumage=body-back+body-belly+far-thigh+near-thigh,neck-plumage=neck-1+neck-2,head-plumage=head,bill=bill,wing-plumage=far-arm+far-hand+near-arm+near-hand,tail-plumage=tail,legs-skin=far-shank+far-tarsus+near-shank+near-tarsus,web-feet=far-foot+near-foot";;
100) M="body-plumage=body-back+body-belly,neck-plumage=neck-1+neck-2,head-plumage=head,bill=bill,wing-plumage=far-arm+far-hand+near-arm+near-hand,tail-plumage=tail,legs-skin=far-leg+near-leg";;
200) M="body-plumage=body-back+body-belly,neck-plumage=neck-1+neck-2,head-plumage=head,bill=bill,wing-plumage=far-arm+far-hand+near-arm+near-hand,tail-plumage=tail,legs-skin=near-leg+far-leg";;
esac
./author-29-bun.sh kit-thermal.ts animals/herring-gull --scale $S --map "$M" "$@"
