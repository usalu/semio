#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
python3 "$D/restore.py"; "$D/release.sh"
