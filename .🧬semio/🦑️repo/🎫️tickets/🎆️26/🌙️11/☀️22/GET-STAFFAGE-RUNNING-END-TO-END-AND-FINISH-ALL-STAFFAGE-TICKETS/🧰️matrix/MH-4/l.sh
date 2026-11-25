#!/bin/zsh
# 🧷️ One bun script of this folder under the lock of the batch, run from the suite folder: l.sh <script.ts> <args…>
source "${0:A:h}/env.sh"
source "$W/lock.sh"
"$W/cap.sh" ${MB:-1000} 1800 "$T" "$B" "$W/$1" "${@:2}"
