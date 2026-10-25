#!/bin/sh
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sh $C/rcore-b-tools/run.sh $C/🗑️generated/rcore-b/upd2.out update --manifest-path Cargo.toml --workspace --offline
sh $C/rcore-b-tools/run.sh $C/🗑️generated/rcore-b/census2.out build --manifest-path Cargo.toml --workspace --tests --keep-going --message-format=json
