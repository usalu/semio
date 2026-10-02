#!/usr/bin/env bash
# Stages both deliverables of the architecture quiz from the checkout, in the order the site's README gives (a build of
# the site empties `dist`, the bundle included), then copies them to the stable git-ignored hand-over folder
# `.🧬semio/🎓️teaching/architecture-quiz-poc/{site,proctor}` and writes what was staged beside them.
#   bash stage_deliverables.sh <log directory>
set -u
cd /c/git/semio || exit 1
LOGS="$1"; mkdir -p "$LOGS"
export RUSTC_WRAPPER="" NX_PLUGIN_NO_TIMEOUTS=true
DIST="🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/dist"
OUT=".🧬semio/🎓️teaching/architecture-quiz-poc"
step() {
  local name="$1"; shift
  "$@" > "$LOGS/$name.log" 2>&1
  local code=$?
  echo "$name exit=$code"
  [ "$code" -eq 0 ] || exit "$code"
}
step publish bun nx run @teaching/architecture-quiz:publish
step docker-image-build bun nx run @teaching/architecture-quiz:docker-image-build
step docker-stack-bundle bun nx run @teaching/architecture-quiz:docker-stack-bundle
[ -z "$(ls -A "$OUT/proctor/certificates" 2>/dev/null)" ] || { echo "certificates/ of the hand-over folder is not empty: not replaced"; exit 1; }
rm -rf "$OUT/site" "$OUT/proctor"
mkdir -p "$OUT"
cp -R "$DIST/pages/quizzes" "$OUT/site"
cp -R "$DIST/proctor" "$OUT/proctor"
{
  echo "staged $(date -u +%Y-%m-%dT%H:%M:%SZ) from $(git rev-parse HEAD) with the working tree"
  echo "site: $(find "$OUT/site" -type f | wc -l) files, $(du -sb "$OUT/site" | cut -f1) bytes, index.html sha256 $(sha256sum "$OUT/site/index.html" | cut -d' ' -f1)"
  echo "proctor: $(find "$OUT/proctor" -type f | wc -l) files, $(du -sb "$OUT/proctor" | cut -f1) bytes, proctor-image.tar sha256 $(sha256sum "$OUT/proctor/proctor-image.tar" | cut -d' ' -f1)"
  echo "image: $(docker image inspect ghcr.io/usalu/architecture-quiz-proctor:latest --format '{{.Id}} {{index .Config.Labels "org.opencontainers.image.revision"}} {{index .Config.Labels "org.opencontainers.image.created"}}')"
} | tee "$OUT/STAGED.txt"
diff -rq "$DIST/pages/quizzes" "$OUT/site" && diff -rq "$DIST/proctor" "$OUT/proctor" && echo "copies equal the staged artifacts"
