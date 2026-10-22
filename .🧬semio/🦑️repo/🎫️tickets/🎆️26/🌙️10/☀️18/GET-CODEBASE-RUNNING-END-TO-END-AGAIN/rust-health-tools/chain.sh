#!/bin/sh
# usage: chain.sh LOGPREFIX  -> update lock then root workspace check
P=$1
sh "$(dirname "$0")/run.sh" "$P-lock.log" update --workspace --offline
sh "$(dirname "$0")/run.sh" "$P.log" check --workspace --all-targets --keep-going --message-format=short
