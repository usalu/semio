#!/bin/sh
# Verifies (or with --write regenerates) every thermal fixture with the third-party oracles.
exec uv run --with scipy --with CoolProp --with psychrolib --with pythermalcomfort --with pyet --with pandas python -I "$(dirname "$0")/thermal-oracles.py" "$@"
