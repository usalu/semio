#!/usr/bin/env bash
# 🕹️ Posts pre.js + h2.js + im-hook.js + body to my page driver on port 6252. Usage: sa-run.sh body.js
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; H="$M/🗑️generated/hx"
cat "$H/pre.js" "$H/h2.js" "$M/im-hook.js" "$1" | curl -s --max-time 600 -X POST --data-binary @- http://127.0.0.1:6252/
