#!/bin/zsh
# 🔁️ The pinned cases once more (`red.sh <tag>`), then the other named suites (`more.sh`), inside one grant. again.sh <tag>
source "${0:A:h}/env.sh"
zsh "$W/red.sh" "$1"
zsh "$W/more.sh"
