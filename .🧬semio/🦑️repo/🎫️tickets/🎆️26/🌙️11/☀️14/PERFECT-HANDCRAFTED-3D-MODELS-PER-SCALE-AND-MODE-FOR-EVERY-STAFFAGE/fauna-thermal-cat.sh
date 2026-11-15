#!/bin/bash
S=$1; shift
case $S in
50) M="coat-trunk=rump+flank-back+flank-belly+ribs-back+ribs-belly+chest-back+chest-belly+neck,coat-head=head+jaw-n+jaw-f,tongue=tongue,ears=near-ear+far-ear,nose-skin=nose-leather,eyes=eye-a+eye-b,pads=far-hind-pad+far-fore-pad+near-hind-pad+near-fore-pad,coat-forelimbs=far-scapula+far-humerus+far-forearm+far-wrist+far-fore-paw+near-scapula+near-humerus+near-forearm+near-wrist+near-fore-paw,coat-hindlimbs=far-thigh+far-shank+far-hock+far-hind-paw+near-thigh+near-shank+near-hock+near-hind-paw,coat-tail=tail-1+tail-2+tail-3+tail-4+tail-5";;
100) M="coat-trunk=rump+flank-back+flank-belly+ribs-back+ribs-belly+chest+neck,coat-head=head+jaw-n+jaw-f,tongue=tongue,ears=near-ear+far-ear,nose-skin=nose-leather,eyes=eye-a+eye-b,pads=far-hind-pad+far-fore-pad+near-hind-pad+near-fore-pad,coat-forelimbs=far-scapula+far-humerus+far-forearm+far-wrist+far-fore-paw+near-scapula+near-humerus+near-forearm+near-wrist+near-fore-paw,coat-hindlimbs=far-thigh+far-shank+far-hock+far-hind-paw+near-thigh+near-shank+near-hock+near-hind-paw,coat-tail=tail-1+tail-2+tail-3+tail-4+tail-5";;
200) M="coat-trunk=hindbody+chest+neck,coat-head=head+jaw,ears=near-ear+far-ear,coat-forelimbs=f-fore-upper+f-fore-lower+f-fore-foot+n-fore-upper+n-fore-lower+n-fore-foot,coat-hindlimbs=f-hind-thigh+f-hind-lower+f-hind-foot+n-hind-thigh+n-hind-lower+n-hind-foot,coat-tail=tail-1+tail-2+tail-3+tail-4+tail-5";;
esac
./author-29-bun.sh kit-thermal.ts animals/domestic-cat --scale $S --map "$M" "$@"
