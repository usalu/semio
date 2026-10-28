# source me: pregate-g shell helpers
export SEMIO_REPO=/Users/ueli/Documents/semio
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad
C=$S/C
T=$C/pregate-g-tools
M=/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/pg-all
MM="python3 -I $C/mirror-tools/mirror.py"
F="🧰️framework"
L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"
P="$F/🔨️modules/🏃️process"
mrun(){ python3 -I $C/mirror-tools/mirror.py run pg-all --peers-ok "$@"; }
export PG_BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export PATH="$(dirname $PG_BUN):$PATH"
log(){ printf '%s %s\n' "$(date +%H:%M)" "$*" >> $T/out/verdicts.txt; echo "$*"; }
docs(){ n=$1; mrun --no-seal -- python3 -I $C/docs3-g-tools/staged/$n/stage.py ${2:-apply} --root $M 2>&1 | tail -${3:-4}; }
export SEMIO_REPO_WATCH_BINARY=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/repo-watch/*/semio-repo-watch 2>/dev/null | head -1)
export PG_ROOT=$M T C
rs(){ python3 -I $T/rs.py "$@"; }
vs(){ python3 -I $T/rs.py "$@"; }
# grp <n>: snapshot pg-all into pg-g<n> and run the cross-cutting laws there in the background
grp(){ python3 -I $T/snap.py pg-all pg-g$1p >/dev/null && python3 -I $T/snap.py pg-all pg-g$1 >/dev/null && ( nohup python3 -I $C/mirror-tools/mirror.py run pg-g$1 --no-seal --peers-ok -- python3 -I $T/laws.py $TMPDIR/sm/pg-g$1 $T/out/laws-g$1.jsonl -j 3 > $T/out/laws-g$1.txt 2>&1 & ) ; echo "laws g$1 started $(date +%H:%M)"; }
export BATCH_ROOT=$M
