#!/bin/zsh
# 🧪 LB landing window 2: native checks of lb-p1 + lb-p4 (native lane, build-fleet-b), one hold per invocation.
cd /Users/ueli/Documents/semio || exit 2
zsh .tmp-ticket/wp-lb/check-fleet.sh lw2-check-default-2 --keep-going -p semio-s-plugin-stdio -p semio-s-artifact-stdio-semio --lib --tests
zsh .tmp-ticket/wp-lb/check-fleet.sh lw2-check-full-2 --keep-going -p semio-s-plugin-stdio -p semio-s-artifact-stdio-semio --features semio-s-plugin-stdio/full-app-catalog --lib --tests
