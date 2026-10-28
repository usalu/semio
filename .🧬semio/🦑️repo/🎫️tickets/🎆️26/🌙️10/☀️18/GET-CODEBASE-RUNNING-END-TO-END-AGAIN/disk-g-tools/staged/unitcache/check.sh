#!/bin/sh
# check the staged batch unitcache (anchored patches, see batches/unitcache.py); extra arguments: --root <checkout> --backup-dir <dir> --force --tolerate-missing
exec python3 -I "$(dirname "$0")/../../stage.py" check unitcache "$@"
