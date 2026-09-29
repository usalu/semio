#!/bin/zsh
# 🧪️ One overlay-lane job for the shared scratch: the p17 live-baseline registry run, then the p19 proof (both mutate the scratch, so they never run concurrently).
zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/lb2-p17-baseline.sh
zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/lb2-p19-proof.sh p19-s1
