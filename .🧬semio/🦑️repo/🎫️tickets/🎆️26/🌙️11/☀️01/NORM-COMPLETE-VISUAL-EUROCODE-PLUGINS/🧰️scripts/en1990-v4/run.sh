#!/usr/bin/env bash
cd "$(dirname "$0")" && exec /home/deck/semio/.venv/bin/python -I -c "import sys; sys.path.insert(0, '.'); import $1; $1.main()"
