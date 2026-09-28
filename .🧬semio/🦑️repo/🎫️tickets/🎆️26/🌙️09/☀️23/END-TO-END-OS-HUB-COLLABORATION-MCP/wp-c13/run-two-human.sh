#!/bin/zsh
# 👥️ C13: the permanent two-human harness (os-dev `verify two-human`) with a journey, against one hub through one or two serves.
# usage: zsh run-two-human.sh <tag> <journey> <en|de> <hubUrl> <serveA> <serveB> <adminCapabilityFile> [kinds] [mountBudgetMs]
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false
export SEMIO_ACCEPTANCE_RESULT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-acceptance/$1.json"
echo "START $(date '+%F %T') tag=$1 journey=$2 locale=$3 pid=$$"
nice -n 10 bun ./📜️script.ts verify two-human --journey "$2" --hub "$4" --serve "$5" --serve-b "$6" --locale "$3" --admin-capability "$7" --tag "$1" --out /Users/ueli/Documents/semio/.tmp-ticket/wp-c13/generated ${8:+--kinds} ${8:+$8} --mount-budget-ms "${9:-240000}"
echo "EXIT rc=$? $(date '+%F %T')"
