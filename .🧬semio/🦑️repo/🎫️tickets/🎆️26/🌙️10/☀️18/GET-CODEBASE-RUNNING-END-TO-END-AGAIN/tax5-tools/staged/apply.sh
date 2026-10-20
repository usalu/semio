#!/bin/zsh
# 🔧 Applies (or with --check verifies) the tax5 library patches (leaf case table coverage). usage: apply.sh [--root <library dir>] [--check] [--only <patch id>...]
cd /Users/ueli/Documents/semio
exec python3 "${0:A:h}/apply.py" "$@"
