#!/bin/zsh
# usage: presentation-hub-py.sh <python args>; the course Python env with the output root inside the workspace (the browser render requires a short owned workspace directory)
R=/Users/ueli/Documents/semio
mkdir -p "$R/.cache/pres-hub"
cd "$R"
export SEMIO_TEACHING_OUTPUT_ROOT="$R/.cache/pres-hub" SEMIO_TEACHING_CACHE_ROOT=/private/tmp/claude-501/cache PYTHONDONTWRITEBYTECODE=1 PYTHONPATH="$R"
export SEMIO_TEACHING_PROFILE=${SEMIO_TEACHING_PROFILE:-preview} SEMIO_TEACHING_LANGUAGE=${SEMIO_TEACHING_LANGUAGE:-en} SEMIO_TEACHING_AUDIENCE=${SEMIO_TEACHING_AUDIENCE:-speaker}
exec /private/tmp/claude-501/semio-teaching-venv/bin/python "$@"
