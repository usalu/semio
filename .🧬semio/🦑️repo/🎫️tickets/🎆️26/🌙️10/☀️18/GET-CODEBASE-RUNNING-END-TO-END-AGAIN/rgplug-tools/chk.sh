#!/bin/zsh
# usage: chk.sh <workspace Cargo.toml rel> <extra cargo args...>   (update --workspace, then check)
cd /Users/ueli/Documents/semio
W="$1"; shift
for i in 1 2 3; do
  perl -e 'alarm 300; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- update --workspace --manifest-path "$W" > /tmp/rgp-upd.log 2>&1
  if command grep -q "System image plist" /tmp/rgp-upd.log; then /bin/sleep 15; continue; fi
  break
done
command grep -v '^\[script\]' /tmp/rgp-upd.log | command grep -v "^ *Adding\|Removing" | cut -c1-300 | tail -4
perl -e 'alarm 570; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path "$W" "$@" --message-format=short > /tmp/rgp-chk.log 2>&1
command grep -v '^\[script\]' /tmp/rgp-chk.log | cut -c1-400 | tail -${TAILN:-40}
