#!/bin/bash
# usage: short-overlay.sh   copies the verification overlay to /private/tmp/rco (the repository's 256 UTF-16 unit path rule rejects some library modules at the overlay's depth inside the ticket) and adds dereferenced copies of ajv and @iarna/toml
set -e
cd /Users/ueli/Documents/semio
C=$(cat /tmp/rcache_C.txt); OV="$C/🗑️generated/rcache-overlay"
for mountpoint in $(mount | grep "/private/tmp/rco/" | sed -E "s/^.* on (\/private\/tmp\/rco\/[^ ]*) \(.*$/\1/"); do hdiutil detach "$mountpoint" >/dev/null || true; done
rm -rf /private/tmp/rco; cp -R "$OV" /private/tmp/rco; mkdir -p /private/tmp/rco/node_modules/@iarna
NM=$(readlink node_modules); cp -RL "$NM"/.bun/ajv@8.20.0/node_modules/* /private/tmp/rco/node_modules/; cp -RL node_modules/@iarna/toml /private/tmp/rco/node_modules/@iarna/
echo ready
cp -R "$C/🗑️generated/rcache-nx/ws/node_modules/"* /private/tmp/rco/node_modules/ 2>/dev/null || true
