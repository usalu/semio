#!/bin/bash
# usage: bz.sh <bazel args...>   the provisioned Bazel against the scratch workspace/output base
cd /private/tmp/bzlcov/ws || exit 2
exec /Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel --output_base=/private/tmp/bzlcov/ob --install_base=/private/tmp/bzlcov/install --nosystem_rc --nohome_rc --bazelrc=/private/tmp/bzlcov-ov/.bazelrc "$@"
