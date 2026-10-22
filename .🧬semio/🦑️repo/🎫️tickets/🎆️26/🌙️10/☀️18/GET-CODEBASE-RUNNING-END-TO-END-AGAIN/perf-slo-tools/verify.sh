#!/bin/sh
# Builds the staged set, an overlay of copies under /private/tmp/perf-slo/ov and runs the bench tests there (nice 10, load guard first). Usage: sh verify.sh [bun test options]
set -e
here="$(cd "$(dirname "$0")" && pwd)"
python3 "$here/guard.py" --below "${LOAD_BELOW:-30}" --max "${LOAD_WAIT:-900}"
python3 "$here/build.py" >/dev/null
python3 "$here/mkoverlay.py" >/dev/null
test_file="$(python3 -c "import sys;sys.path.insert(0,'$here');import names;print('./'+names.BENCHDIR+'/'+names.TESTS+'/'+names.TS)")"
cd /private/tmp/perf-slo/ov
SEMIO_TEST_REPO_ROOT=/Users/ueli/Documents/semio nice -n 10 python3 "$here/closure-run.py" /private/tmp/perf-slo/ov -- bun test --timeout 120000 "$@" "$test_file"
