#!/usr/bin/env bash
# Sourced by the python-manim scripts: the pinned uv, the machine-cache environment and the native prefix of the provisioner.
ROOT="/Users/ueli/Documents/semio"
export UV="$ROOT/.cache/b/t/m/native/uv/0.11.6/darwin-arm64-4b69a4e366ec38cd/uv"
export BUN="$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
export C="$ROOT/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring"
export PROJECT="$C/📦️packages/🐍️python"
export CACHE="$HOME/Library/Caches/semio/teaching-python"
export UV_PYTHON_INSTALL_DIR="$HOME/Library/Caches/semio/teaching-python-runtimes"
export UV_PYTHON_DOWNLOADS=never UV_PYTHON_PREFERENCE=only-managed
export UV_PROJECT_ENVIRONMENT="$CACHE/env"
export UV_BUILD_CONSTRAINT="$PROJECT/build-constraints.txt"
export SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts"
eval "$("$BUN" -e 'import {provisionNative} from "'"$PROJECT"'/🧱️native/🟦️.ts"; const r=await provisionNative(); for (const [k,v] of Object.entries(r?.environment??{})) console.log(`export ${k}=${JSON.stringify(v)}`)')"
export PY="$CACHE/env/bin/python"
export SEMIO_TOOL_PYTHON="$PY"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1
export SEMIO_TEACHING_OUTPUT_ROOT="$ROOT/.🧬semio/🎓️teaching/⚡️cache/tests" PYTHONWARNINGS=ignore
unset PYTHONPATH
