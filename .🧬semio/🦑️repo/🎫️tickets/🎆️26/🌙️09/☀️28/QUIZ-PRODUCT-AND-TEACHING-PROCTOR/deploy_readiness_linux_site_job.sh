#!/bin/bash
# 🐧️ Deploy-readiness rehearsal of the workflow's `site` job on Linux, without GitHub: a fresh clone of the repository's
# HEAD with the working tree's changes laid over it (what the owner would push), the workflow's tool versions, then the
# job's own steps. Runs inside `node:24.15.0-bookworm` with the repository mounted read-only at /mnt/repo:
#   git status --porcelain --untracked-files=all -z > <overlay list>
#   docker run --rm --volume <repo>:/mnt/repo:ro --volume deploy-readiness-linux-work:/work --env OVERLAY=/mnt/repo/<overlay list> node:24.15.0-bookworm bash /mnt/repo/<this file>
# It also answers what the `proctor` job relies on: after `bun install --frozen-lockfile --ignore-scripts` and an nx run,
# no tracked file differs from the commit (the publish verb refuses an image built from uncommitted changes).
set -u
step() { echo "[DEBUG] ===== $* ====="; }
git config --global --add safe.directory '*'
git config --global core.quotepath false
step "clone HEAD"
[ -d /work/.git ] || git clone --quiet --depth 1 "file:///mnt/repo" /work || exit 1
cd /work || exit 1
rm -f .git/index.lock
git reset --quiet --hard && git clean --quiet -fdx -e node_modules
step "overlay the working tree ($(tr '\0' '\n' < "$OVERLAY" | grep -c .) changed paths, listed on the host by git status --porcelain --untracked-files=all -z)"
while IFS= read -r -d '' entry; do
  path="${entry:3}"
  case "$path" in .🧬semio/*) continue ;; esac
  if [ -f "/mnt/repo/$path" ]; then mkdir -p "$(dirname "$path")" && cp "/mnt/repo/$path" "$path"; else rm -f "$path"; fi
done < "$OVERLAY"
git add --all >/dev/null 2>&1 && git -c user.name=rehearsal -c user.email=rehearsal@localhost commit --quiet --message "rehearsal: working tree" && echo "[DEBUG] committed the overlay as $(git rev-parse --short HEAD)"
step "tools"
npm install --global --silent "bun@$(node -p "require('./package.json').packageManager.split('@')[1]")" || exit 1
echo "[DEBUG] node $(node --version), bun $(bun --version)"
step "bun install --frozen-lockfile --ignore-scripts"
bun install --frozen-lockfile --ignore-scripts 2>&1 | tail -6
echo "[DEBUG] install exit=${PIPESTATUS[0]}"
step "bun nx run @teaching/architecture-quiz:test"
bun nx run @teaching/architecture-quiz:test 2>&1 | grep -v "^$" | tail -25
echo "[DEBUG] test exit=${PIPESTATUS[0]}"
step "bun nx run @teaching/architecture-quiz:publish"
bun nx run @teaching/architecture-quiz:publish 2>&1 | grep -v "resolve at build time\|^$" | tail -60
echo "[DEBUG] publish exit=${PIPESTATUS[0]}"
step "artifact"
pages="🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/dist/pages/quizzes"
ls -la "$pages" | head -30
echo "[DEBUG] files $(find "$pages" -type f | wc -l), bytes $(du -sb "$pages" | cut -f1)"
echo "[DEBUG] tar as actions/upload-pages-artifact does (dot files excluded): $(tar --dereference --hard-dereference --directory "$pages" -cf - --exclude=.git --exclude=.github --exclude='.[^/]*' . | tar -tf - | grep -c .) entries"
step "tracked files after install, test and publish"
git status --porcelain --untracked-files=no | head -20
echo "[DEBUG] tracked changes: $(git status --porcelain --untracked-files=no | grep -c .)"
