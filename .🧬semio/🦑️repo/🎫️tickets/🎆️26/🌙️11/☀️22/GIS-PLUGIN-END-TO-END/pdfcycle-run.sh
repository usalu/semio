#!/bin/sh
# usage: pdfcycle-run.sh <logname> <cargo args...>  (shared compile lock around one cargo invocation)
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
name="$1"; shift
until mkdir "$T/🗑️generated/compile.lock" 2>/dev/null; do if [ $(( $(date +%s) - $(stat -f %m "$T/🗑️generated/compile.lock") )) -gt 7200 ]; then rmdir "$T/🗑️generated/compile.lock"; fi; sleep 20; done
echo pdfcycle-lane > "$T/🗑️generated/compile.lock/owner"
cd /Users/ueli/Documents/semio
repo cargo --lang=en --store=shared --configuration=prototype -- "$@" > "$T/🗑️generated/pdfcycle-$name.log" 2>&1
echo "exit=$?" >> "$T/🗑️generated/pdfcycle-$name.log"
rm -rf "$T/🗑️generated/compile.lock"
sleep 120
