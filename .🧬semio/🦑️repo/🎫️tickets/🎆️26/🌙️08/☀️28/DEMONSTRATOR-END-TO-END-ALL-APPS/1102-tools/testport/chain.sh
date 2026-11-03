#!/bin/bash
D="$(cd "$(dirname "$0")" && pwd)"
"$D/t.sh" W-testport-cmd4 w4law.txt
"$D/ladder.sh" 1
