#!/bin/bash
S=$1; shift
M="plumage-body=body+near-thigh+far-thigh,plumage-neck-head=neck-1+neck-2+neck-3+head,plumage-wings=near-primaries+near-coverts+far-primaries+far-coverts+near-shoulder,plumage-tail=tail+tail-f,legs-feet=near-shank+near-tarsus+near-toe-mid+near-toe-in+near-toe-out+near-ball+near-hallux+near-ankle+far-shank+far-tarsus+far-toe-mid+far-toe-in+far-toe-out+far-ball+far-hallux+far-ankle,beak=beak+jaw,cere=cere,eye=eye+eye-f"
./author-29-bun.sh kit-thermal.ts animals/pigeon --scale $S --map "$M" "$@"
