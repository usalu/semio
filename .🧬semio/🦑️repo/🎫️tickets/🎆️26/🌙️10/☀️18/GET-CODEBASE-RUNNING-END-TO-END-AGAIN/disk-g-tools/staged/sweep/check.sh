#!/bin/sh
# check the staged batch sweep (anchored patches, see batches/sweep.py); extra arguments: --root <checkout> --backup-dir <dir> --force --tolerate-missing
exec python3 -I "$(dirname "$0")/../../stage.py" check sweep "$@"
