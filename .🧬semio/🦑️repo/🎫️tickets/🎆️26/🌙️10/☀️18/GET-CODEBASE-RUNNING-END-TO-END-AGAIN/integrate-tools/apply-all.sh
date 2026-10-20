#!/usr/bin/env bash
# 🚦 usage: apply-all.sh [--yes] [--group ID] [--undo ID] [--series DIR] [--root DIR] [--force-while-running]
# Dry run by default; --yes writes. Without --series the three series run in order: final (A-C: contracts, adapter, closure/taxonomy), final2 (D-F: ingress, uncached owners, lazy mapping), final3 (G: graph-perf incl. show-perf).
# Every file carries the hash of its preimage: drift refuses the group before the first write; an already applied file is skipped, so the whole script is idempotent. --undo ID restores a group from its backup while the tree still holds its result.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
for arg in "$@"; do if [ "$arg" = "--series" ]; then exec bun "$here/apply-all.ts" "$@"; fi; done
case " $* " in *" --undo "*) echo "--undo needs --series (final, final2 or final3)" >&2; exit 2;; esac
for series in final final2 final3; do
  [ -f "$here/$series/manifest.json" ] || continue
  bun "$here/apply-all.ts" --series "$series" "$@" || exit $?
done
