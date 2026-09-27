#!/bin/zsh
# 🧪 LB wasm32-wasip2 check through the fleet wasm mutex (landing build-dir): check-wasm.sh <capture-name> <cargo check args…>
cd /Users/ueli/Documents/semio || exit 2
name="$1"; shift
zsh .tmp-ticket/📜️fleet-mutex.sh wasm lb -- zsh .tmp-ticket/wp-lb/check-native.sh "$name" "$@" --target wasm32-wasip2
