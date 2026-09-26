#!/bin/zsh
# 🐳️ H10: cold `docker build` of 🌎️hub/Dockerfile (context = repo root), timed, capped at $1 compiler jobs.
# usage: docker-build.sh <jobs> <tag>
JOBS=$1; TAG=$2
cd /Users/ueli/Documents/semio
echo "=== start $(date +%T) jobs=$JOBS tag=$TAG"
start=$(date +%s)
docker build --progress=plain --build-arg CARGO_BUILD_JOBS=$JOBS -f "🌎️hub/Dockerfile" -t "semio/os-hub:$TAG" . 2>&1
rc=$?
echo "EXIT $rc after $(( $(date +%s) - start )) s"
docker image ls "semio/os-hub:$TAG"
echo "=== done $(date +%T)"
