#!/bin/sh
# check the staged batch ledger (anchored patches, see batches/ledger.py); extra arguments: --root <checkout> --backup-dir <dir> --force --tolerate-missing
exec python3 -I "$(dirname "$0")/../../stage.py" check ledger "$@"
