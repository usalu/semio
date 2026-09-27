#!/bin/zsh
# 🧪️ T13: parity runs during the rebuild — the generated hosts' cargo builds go through the native mutex (rule 30).
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native t13 -- zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-t13/parity-case.sh "$@"
