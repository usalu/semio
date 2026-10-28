# [rebase3] helpers: . env.sh ; mm <mirror args>; mr <cmd...> (run in mirror rb1)
cd /Users/ueli/Documents/semio || return 1
export SEMIO_REPO=$PWD
C="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
H="$C/graphlat-g-tools/staged"; RF="$C/rustfast-g-tools/staged/rustfast"; M="${TMPDIR}sm/rb1"
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'; G="$L/🏗️build/🕸️graph"
mm() { python3 -I "$C/mirror-tools/mirror.py" "$@"; }
mr() { python3 -I "$C/mirror-tools/mirror.py" run rb1 "$@"; }
bt() { python3 -I "$C/graphlat-g-tools/staged/batch_generic.py" "$H/$1" "$2"; }
