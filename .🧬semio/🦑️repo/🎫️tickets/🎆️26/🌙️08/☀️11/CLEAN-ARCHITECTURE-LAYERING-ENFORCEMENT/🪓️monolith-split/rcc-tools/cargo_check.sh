#!/bin/zsh
# 🛠️ Runs one sanctioned launcher cargo call and writes the log and the exit status next to it. Args: <log-name> <manifest-path> <cargo args...>
cd /Users/ueli/Documents/semio || exit 2
LOG="$1"; MANIFEST="$2"; shift 2
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- "$@" --message-format=short --manifest-path "$MANIFEST" > "$G/$LOG.log" 2>&1
echo "exit=$?" > "$G/$LOG.status"
