#!/bin/sh
# apply the staged batch rootguard (anchored patches, see batches/rootguard.py); extra arguments: --root <checkout> --backup-dir <dir> --force --tolerate-missing
exec python3 -I "$(dirname "$0")/../../stage.py" apply rootguard "$@"
