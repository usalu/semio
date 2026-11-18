#!/bin/sh
# usage: remount-image.sh <partition letter>; refuses while any process has a file open on the mount or a lease is held
set -eu
root=/Users/ueli/Documents/semio/.cache/b/$1
mount_point=$root/m
if lsof -w +f -- "$mount_point" 2>/dev/null | awk 'NR>1' | grep -q .; then echo "busy: open files on $mount_point"; exit 1; fi
dev=$(mount | awk -v m="$mount_point" '$3==m{print $1}')
whole=$(diskutil info "$dev" | awk -F: '/Part of Whole/{gsub(/ /,"",$2);print $2}')
hdiutil detach "/dev/$whole"
diskutil image attach --nobrowse --mountOptions owners --mountPoint "$mount_point" "$root/fixed.dmg"
df -k "$mount_point" | tail -1
