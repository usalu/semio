#!/bin/sh
set -eu
here=$(cd "$(dirname "$0")" && pwd)
play=${1:-/Users/ueli/.sinc/play}
cd "$play"
patch -p1 --dry-run < "$here/o15.patch"
patch -p1 < "$here/o15.patch"
