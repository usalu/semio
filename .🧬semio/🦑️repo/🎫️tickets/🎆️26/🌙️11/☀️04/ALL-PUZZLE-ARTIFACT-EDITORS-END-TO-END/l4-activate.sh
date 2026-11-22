#!/bin/sh
# L4: repo route for editor activation; usage: l4-activate.sh <2d|3d|5d>
T="$(cd "$(dirname "$0")" && pwd)"
cd "$(git -C "$T" rev-parse --show-toplevel)"
timeout "${L4_TIMEOUT:-14400}" repo run "@semio-tech/framework-os-dev:activate-puzzle$1-react-prototype" --lang=en > "$T/🗑️generated/l4-activate-$1.log" 2>&1
echo "EXIT $?" >> "$T/🗑️generated/l4-activate-$1.log"
