#!/usr/bin/env bash
# usage: mkcopy.sh -- builds the lean throwaway repo copy (own git init) under the scratchpad
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/gofin5
REAL=/Users/ueli/Documents/semio
COPY="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/gofin5/repo-copy"
mkdir -p "$COPY"
rsync -a --delete \
  --exclude=/.git --exclude=/node_modules --exclude=/dist --exclude=/.cache --exclude=/.nx --exclude=/.venv --exclude=/bazel-out --exclude=/.tmp --exclude=/temp \
  --exclude=/.pytest_cache --exclude='/extension-host-build-*' --exclude='target/' --exclude='dist/' --exclude='.cache/' --exclude='node_modules/' --exclude='🗑️generated/' --exclude='.DS_Store' \
  --exclude='/♻️mit-bestand' --exclude='/✏️s' --exclude='/🌎️hub' --exclude='/🎓️teaching' --exclude='/🏢️semio-tech' --exclude='/👴️leutwiler' --exclude='/🧪️tests' --exclude='/🧬️schema' \
  --include='/🧰️framework/' --include='/🧰️framework/🛍️products/' --include='/🧰️framework/🛍️products/🦑️repo/***' --include='/🧰️framework/🔨️modules/***' --include='/🧰️framework/📦️packages/***' --exclude='/🧰️framework/**' \
  --include='/.🧬semio/' --include='/.🧬semio/🦑️repo/' --include='/.🧬semio/🦑️repo/🎯️goals/***' --include='/.🧬semio/🦑️repo/🧑️‍💻️devs/***' --include='/.🧬semio/🦑️repo/📋️config.toml' --exclude='/.🧬semio/🦑️repo/*' --exclude='/.🧬semio/*' \
  "$REAL/" "$COPY/" || exit 4
[ -e "$COPY/node_modules" ] || ln -s "$REAL/node_modules" "$COPY/node_modules"
cd "$COPY" || exit 2
if [ ! -d .git ]; then
  git init -q -b main && git -c user.email=t@t -c user.name=t -c commit.gpgsign=false add -A >/dev/null 2>&1 && git -c user.email=t@t -c user.name=t -c commit.gpgsign=false commit -q -m copy >/dev/null 2>&1
fi
du -sk "$COPY" | cut -f1
