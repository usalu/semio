#!/bin/bash
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
R=.cache/b/r/m/components/runs/$(ls -t .cache/b/r/m/components/runs | head -1)
P='error\[E|could not compile|boundary-exceeded|CargoLockRefused|cyclic package|Cannot find module'
echo "run $R $(date +%H:%M) supervisor $(pgrep -f "verify play-release" >/dev/null && echo alive || echo DEAD)"
grep -E "$P" "$R"/l/*.log 2>/dev/null | cut -c1-300 | head -40
grep -aE "$P" "$T/🗑️generated/native/site-build.log" | cut -c1-300 | tail -10
tail -1 "$T/🗑️generated/native/site-build.log" | cut -c1-250
