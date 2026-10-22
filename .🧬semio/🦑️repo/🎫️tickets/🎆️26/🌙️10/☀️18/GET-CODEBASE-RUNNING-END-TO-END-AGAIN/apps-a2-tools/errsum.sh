#!/bin/bash
# usage: errsum.sh <log> : error lines with the long framework paths shortened
/usr/bin/grep -E ': error|^error: could not' "$1" | sed 's#/Users/ueli/Documents/semio/##' | sed -E 's#^[^ ]*/([^/ ]+/[^/ ]+/[^/ ]+\.rs)#…/\1#' | cut -c1-300
