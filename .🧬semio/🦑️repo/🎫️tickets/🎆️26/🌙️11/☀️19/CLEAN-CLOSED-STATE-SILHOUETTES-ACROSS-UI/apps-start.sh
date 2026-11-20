#!/bin/sh
case "$0" in
  */*) PREVIEW_DIR=${0%/*} ;;
  *) PREVIEW_DIR=. ;;
esac
exec repo tools exec --light node -- "$PREVIEW_DIR/preview-cli.cjs" start "${1:-all}"
