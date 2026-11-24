#!/bin/zsh
# 🏃️ Runs a zsh snippet with the paths of batch MS set: x.sh '<snippet>'
source "${0:A:h}/env.sh"
eval "$1"
