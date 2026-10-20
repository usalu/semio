#!/bin/zsh
# usage: go.sh <label> <bazel build args...>   (cwd anywhere)
BASE=/private/tmp/bzlrust
BZ=$(echo /Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/*/bazel)
LABEL=$1; shift
cd $BASE/bw || exit 1
: > $BASE/spawns.before; [ -f $BASE/spawns.log ] && cp $BASE/spawns.log $BASE/spawns.before
START=$(python3 -c 'import time;print(time.time())')
perl -e 'alarm 1500; exec @ARGV' -- $BZ --output_base=$BASE/ob --install_base=$BASE/install --nosystem_rc --nohome_rc --noworkspace_rc --max_idle_secs=60 build --lockfile_mode=off --incompatible_strict_action_env --incompatible_autoload_externally= --disk_cache=$BASE/disk --jobs=${JOBS:-3} --experimental_convenience_symlinks=ignore --color=no --curses=no --execution_log_json_file=$BASE/exec-$LABEL.json "$@" > $BASE/out-$LABEL.txt 2>&1
RC=$?
END=$(python3 -c 'import time;print(time.time())')
NEW=$(( ( $(wc -l < $BASE/spawns.log 2>/dev/null || echo 0) - $(wc -l < $BASE/spawns.before) ) / 2 ))
echo "[$LABEL] rc=$RC wall=$(python3 -c "print(round($END-$START,1))")s ingress_spawns=$NEW"
command grep -E 'INFO: [0-9]+ process|Build completed|ERROR|FAILED' $BASE/out-$LABEL.txt | head -8
