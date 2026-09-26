#!/bin/zsh
# 🐳️ H10: cold `docker build` of 🌎️hub/Dockerfile from a prepared context directory (a patched copy of the repository),
# timed, capped at $2 compiler jobs. usage: docker-build-context.sh <context dir> <jobs> <tag>
CTX=$1; JOBS=$2; TAG=$3
cd "$CTX"
echo "=== start $(date +%T) context=$CTX jobs=$JOBS tag=$TAG"
start=$(date +%s)
docker build --progress=plain --build-arg CARGO_BUILD_JOBS=$JOBS -f "🌎️hub/Dockerfile" -t "semio/os-hub:$TAG" . 2>&1
rc=$?
echo "EXIT $rc after $(( $(date +%s) - start )) s"
docker image ls "semio/os-hub:$TAG"
echo "=== done $(date +%T)"
