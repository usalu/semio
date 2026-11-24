#!/usr/bin/env bash
# 🏃️ Runs a bash snippet with the paths of batch MB-2 set: x.sh '<snippet>'
source "$(dirname "$0")/env.sh"
eval "$1"
