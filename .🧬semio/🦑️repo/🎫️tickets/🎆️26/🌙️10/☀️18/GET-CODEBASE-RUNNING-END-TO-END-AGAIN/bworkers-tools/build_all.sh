#!/bin/bash
# 🧱 Rebuilds every staged file from the current checkout, refreshes the scratch overlay, renders the golden fixtures from the staged code and records the bases the apply step checks.
set -e
cd "$(dirname "$0")"
python3 build_adapter.py
python3 build_misc.py
python3 make_fixture.py > /dev/null
python3 make_fixture_bazel.py > /dev/null
python3 overlay.py
python3 apply.py --root "${BW_OVERLAY:-/private/tmp/bworkers/r}" --write --force > /dev/null
python3 regen_fixtures.py
python3 apply.py --record > /dev/null
python3 apply.py --root "${BW_OVERLAY:-/private/tmp/bworkers/r}" --write --force | tail -1
python3 harness/repoint.py | tail -1
