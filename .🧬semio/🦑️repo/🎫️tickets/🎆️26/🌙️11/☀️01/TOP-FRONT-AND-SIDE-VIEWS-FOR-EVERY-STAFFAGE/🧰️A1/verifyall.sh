#!/usr/bin/env bash
# [DEBUG] verify all A1 assets in one bun process
HERE="$(cd "$(dirname "$0")" && pwd)"
cd "$HERE/../../../../../../../.."
repo tools exec bun -- "$HERE/verify.ts" "${1:-horse,dairy-cow,domestic-cat,goat,hedgehog,herring-gull,labrador-dog}" 50,100,200,500
