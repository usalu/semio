#!/usr/bin/env bash
# 🦀️ en1990 editor: bless the DSL and pack assets through the codecs, then run the lib and integration tests (latency_bench skipped, it asserts in release builds only).
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
cd "$T/../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990" || exit 1
SEMIO_UPDATE_GOLDEN=1 repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1990-editor --test integration --offline -- regen_dsl_and_pack_assets > "$G/en1990-ed-bless.log" 2>&1
echo "bless exit $?" > "$G/en1990-ed-bless.exit"
repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1990-editor -p semio-s-artifact-norm-en1990-scene --lib --test integration --no-fail-fast --offline -- --skip latency_bench > "$G/en1990-ed-b.log" 2>&1
echo "full exit $?" > "$G/en1990-ed-b.exit"
