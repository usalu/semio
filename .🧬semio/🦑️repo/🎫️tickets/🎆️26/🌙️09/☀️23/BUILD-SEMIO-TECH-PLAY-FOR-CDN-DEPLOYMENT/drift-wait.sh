#!/bin/bash
python3 -c "import time;time.sleep(165)"
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
./"$T"/drift-scan.sh | cut -c1-260 | sed -E 's/(cpu [0-9.]+ %.*·)//' | tail -12
