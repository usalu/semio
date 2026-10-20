#!/bin/sh
# Ticket input script: orchestrated end-to-end driver for course, play and demo on Bazel (see orch.py --help).
#   run.sh <course|play|demo|all> [--phase preflight|cold|warm|e2e|probe|report|all[,..]] [--go] [--run-id ID] [--parallel N] [--dry-run]
#   run.sh status|cancel [--run-id ID]
# Light without --go: preflight, report, --dry-run, status, cancel. cold, warm, e2e and probe need --go (or E2E_GO=1): the coordinator's E2E GO.
# Detached (no child owned by an agent):  python3 <C>/chain-tools/detach.py <log> sh <C>/e2e-orch-tools/run.sh all --go --run-id <id>
here=$(cd "$(dirname "$0")" && pwd)
py=$(command -v python3 || command -v python) || { echo "python3 not found on PATH" >&2; exit 3; }
if command -v nice >/dev/null 2>&1; then exec nice -n 10 "$py" "$here/orch.py" "$@"; fi
exec "$py" "$here/orch.py" "$@"
