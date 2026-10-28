#!/bin/sh
# revert the staged batch procstore (anchored patches, see batches/procstore.py); extra arguments: --root <checkout> --backup-dir <dir> --force --tolerate-missing
exec python3 -I "$(dirname "$0")/../../stage.py" revert procstore "$@"
