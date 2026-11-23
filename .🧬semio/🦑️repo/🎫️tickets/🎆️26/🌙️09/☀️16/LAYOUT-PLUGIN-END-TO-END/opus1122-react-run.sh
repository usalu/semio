#!/usr/bin/env bash
# usage: opus1122-react-run.sh <logname> <cwd> <project:target> <script args...>
cd /home/deck/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
n=$1; shift
LAYOUT_RENDERER=react repo tools exec --light bun -- "$T/launch-native.ts" "$@" > "$T/🗑️generated/opus1122/react2-$n.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/opus1122/react2-$n.log"
