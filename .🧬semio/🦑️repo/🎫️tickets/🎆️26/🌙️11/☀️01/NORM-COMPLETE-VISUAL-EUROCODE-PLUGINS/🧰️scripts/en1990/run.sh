#!/usr/bin/env bash
cd "$(dirname "$0")" && exec /home/deck/semio/.venv/bin/python -I -c "import sys; sys.path.insert(0, '.'); exec(compile(open('$1', encoding='utf8').read(), '$1', 'exec'))"
