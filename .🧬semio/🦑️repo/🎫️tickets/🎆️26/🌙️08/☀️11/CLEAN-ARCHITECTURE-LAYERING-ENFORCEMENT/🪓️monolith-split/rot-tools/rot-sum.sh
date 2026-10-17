#!/bin/sh
awk -F'\t' '{n=split($2,a," "); for(i=1;i<=n;i++){ if(a[i]=="pass") p+=a[i-1]; if(a[i]=="fail") f+=a[i-1]; } if($2 ~ /Cannot find/) l++ } END{print "pass="p" fail="f" loaderrors="l" files="NR}' "$1"
