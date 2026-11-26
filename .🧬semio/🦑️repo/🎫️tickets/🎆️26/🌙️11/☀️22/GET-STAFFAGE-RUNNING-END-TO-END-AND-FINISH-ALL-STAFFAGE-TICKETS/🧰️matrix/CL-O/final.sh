#!/bin/zsh
# 🎬️ The closing chain of lane CL-O inside one grant, on the catalog files as written: the pinned cases (`red.sh after`), then the real test files of every asset named (`suites.sh`). final.sh <id>…
source "${0:A:h}/env.sh"
zsh "$W/red.sh" after
zsh "$W/suites.sh" "$@"
