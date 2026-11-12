#!/bin/sh
# usage: lateral-png.sh <file.svg>  (renders to <file>.svg.png next to it with qlmanage)
cd "$(dirname "$1")" && qlmanage -t -s 1400 -o . "$(basename "$1")" >/dev/null 2>&1
