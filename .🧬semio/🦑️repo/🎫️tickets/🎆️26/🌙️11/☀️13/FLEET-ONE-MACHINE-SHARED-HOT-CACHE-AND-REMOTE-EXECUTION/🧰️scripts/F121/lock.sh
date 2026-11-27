#!/bin/zsh
T=$(ls -d /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE*/)
case "$1" in
 take) for i in $(seq 1 30); do if mkdir "${T}landing.lock.$2" 2>/dev/null; then echo "F121 $(date +%H:%M:%S) $3" > "${T}landing.lock.$2/owner"; echo "took $2 $(date +%H:%M:%S)" | tee -a /private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F121/locks.log; exit 0; fi; sleep 5; done; echo "BUSY $2: $(cat ${T}landing.lock.$2/owner)"; exit 1;;
 release) rm -f "${T}landing.lock.$2/owner"; rmdir "${T}landing.lock.$2" && echo "released $2 $(date +%H:%M:%S)" | tee -a /private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F121/locks.log;;
esac
