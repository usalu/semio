#!/bin/bash
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
exec "$BUN" "$@"
