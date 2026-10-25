#!/bin/sh
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sh $C/rcore-b-tools/run.sh $C/🗑️generated/rcore-b/he7a.out test --manifest-path Cargo.toml -p semio-framework-plugin-host-effects -p semio-framework-plugin-guest-contract --no-fail-fast --message-format=short -- --test-threads=1 --nocapture --skip original_entry_full_failure_borrowed_publication
sh $C/rcore-b-tools/run.sh $C/🗑️generated/rcore-b/he7b.out test --manifest-path Cargo.toml -p semio-framework-plugin-guest-contract --lib --message-format=short -- --test-threads=1 --nocapture original_entry_full_failure_borrowed_publication
sh $C/rcore-b-tools/run.sh $C/🗑️generated/rcore-b/census.out build --manifest-path Cargo.toml --workspace --tests --keep-going --message-format=short
