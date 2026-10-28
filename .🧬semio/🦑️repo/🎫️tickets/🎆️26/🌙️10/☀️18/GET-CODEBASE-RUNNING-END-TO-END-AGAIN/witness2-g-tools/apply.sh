#!/bin/sh
exec python3 -I "$(dirname "$0")/stage.py" "${1:-apply}"
