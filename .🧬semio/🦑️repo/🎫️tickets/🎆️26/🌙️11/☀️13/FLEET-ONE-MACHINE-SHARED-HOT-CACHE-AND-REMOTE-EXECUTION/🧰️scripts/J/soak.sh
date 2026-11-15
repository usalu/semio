#!/bin/zsh
# usage: soak.sh <log> <minutes>   repeated small real builds with --offload=auto, sampling replica health, sidecars and memory
LOG="$1"; MIN="${2:-15}"; cd /Users/Shared/semio || exit 2
crates=(semio-framework-repo-model semio-framework-repo-identity semio-framework-repo-workspace semio-framework-repo-providers semio-framework-repo-test-runner semio-framework-repo-codebase semio-framework-repo-hooks semio-framework-pack-json)
end=$(( $(date +%s) + MIN*60 )); i=0; : > "$LOG"
sample() { python3 - <<'PY'
import json,subprocess,time
h=json.load(open('/Users/Shared/semio/.cache/f/health.json'))
def rss(p):
    try:return int(subprocess.run(['ps','-o','rss=','-p',str(p)],capture_output=True,text=True).stdout.strip() or 0)//1024
    except Exception:return -1
sc=[(s['name'],s['state'],s['pid'],s['restarts'],rss(s['pid']) if s['pid'] else None) for s in h.get('sidecars',[])]
print('health age_ms=%d daemon_pid=%s rss_mb=%s code=%s store=%s failures=%d sidecars=%s'%(time.time()*1000-h['wallMs'],h['pid'],rss(h['pid']),h['code'][:8],h['store'],len(h['failures']),sc))
PY
}
while [ $(date +%s) -lt $end ]; do
  c=${crates[$(( i % ${#crates[@]} + 1 ))]}; i=$((i+1)); t0=$(date +%s)
  out=$(repo cargo --offload=auto --store=shared --configuration=prototype -- build -p $c 2>&1); code=$?
  t1=$(date +%s)
  print "$(date +%T) build#$i $c exit=$code ${$((t1-t0))}s $(printf '%s' "$out" | grep -E '^error' | head -1 | cut -c1-120)" >> "$LOG"
  print "$(date +%T) $(sample)" >> "$LOG"
  sleep 20
done
echo DONE >> "$LOG"
