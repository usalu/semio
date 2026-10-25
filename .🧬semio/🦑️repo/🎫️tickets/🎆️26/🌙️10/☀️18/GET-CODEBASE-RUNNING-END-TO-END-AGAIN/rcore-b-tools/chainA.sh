#!/bin/sh
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sh $C/rcore-b-tools/run.sh $C/🗑️generated/rcore-b/he6a.out test --manifest-path Cargo.toml -p semio-framework-plugin-host-effects -p semio-framework-plugin-guest-contract --no-fail-fast --message-format=short -- --test-threads=1 --nocapture --skip original_entry_full_failure_borrowed_publication
sh $C/rcore-b-tools/run.sh $C/🗑️generated/rcore-b/he6b.out test --manifest-path Cargo.toml -p semio-framework-plugin-guest-contract --lib --message-format=short -- --test-threads=1 --nocapture original_entry_full_failure_borrowed_publication
