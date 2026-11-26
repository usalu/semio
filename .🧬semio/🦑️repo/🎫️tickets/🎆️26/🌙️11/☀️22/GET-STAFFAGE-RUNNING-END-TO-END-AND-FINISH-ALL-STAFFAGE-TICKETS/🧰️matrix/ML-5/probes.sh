# 🔦️ Subjects of a trial copy under the lamp (sourced by q.sh): probes.sh <tag> <id>.<variant> <scale>=<subject>…   runs `$H/probe.ts` per entry on $GEN/try/<id>.<variant>.json (variant `base`: the base) [BONES=a,b].
shift; f="$GEN/try/$1.json"; [[ "$1" == *.base ]] && f="$GEN/base/${1%.base}.json"; shift
for t in "$@"; do
  echo "#### 1:${t%%=*} ${t#*=}"
  cap 600 "$T" "$B" "$H/probe.ts" "$f" --scale "${t%%=*}" --subject "${t#*=}" ${BONES:+--bones "$BONES"} 2>&1 | cut -c1-${COLS:-300} | head -${LINES_MAX:-70}
done
