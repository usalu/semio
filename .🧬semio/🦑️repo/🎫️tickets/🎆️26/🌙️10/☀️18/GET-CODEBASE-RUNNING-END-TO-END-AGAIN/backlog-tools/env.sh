# source me: backlog lane helpers (mirrors named bl*, never run an apply without an explicit mirror root)
export SEMIO_REPO=/Users/ueli/Documents/semio
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
export C T=$C/backlog-tools
SM=${TMPDIR%/}/sm
mm(){ python3 -I $C/mirror-tools/mirror.py "$@"; }
export PG_BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export BUN=$PG_BUN
export PATH="$(dirname $PG_BUN):$PATH"
export SEMIO_REPO_WATCH_BINARY=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/repo-watch/*/semio-repo-watch 2>/dev/null | head -1)
F="🧰️framework"; L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"; P="$F/🔨️modules/🏃️process"; B="$L/🏗️build"
# mr <mirror> -- cmd...   : run in the mirror (no seal: bun test needs the shared caches; guard stays on)
mr(){ n=$1; shift; [ "$1" = -- ] && shift; mm run $n --peers-ok --no-seal -- "$@"; }
# rs <mirror> script args : run a lane script retargeted from the live root to the mirror root
rs(){ n=$1; shift; PG_MIRROR=$n PG_ROOT=$SM/$n python3 -I $C/pregate-g-tools/rs.py "$@"; }
# snap SRC DST: copy-on-write clone of a registered mirror
snap(){ python3 -I $C/pregate-g-tools/snap.py "$1" "$2"; }
lnd(){ python3 -I $T/land.py "$@"; }
