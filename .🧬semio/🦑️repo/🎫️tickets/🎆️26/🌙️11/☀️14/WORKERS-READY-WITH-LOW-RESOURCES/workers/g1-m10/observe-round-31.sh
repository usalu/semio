#!/bin/zsh
G="$1"; R="$G/round-31.md"; O=/Volumes/semio-store/v1/origins/g1-m10
for i in $(seq 1 36); do
  rd=$(head -1 "$G/../../ROUND.md" | sed 's/round: //')
  rj=$(timeout 60 cat $O/ready.json 2>/dev/null)
  st=$(print -r -- "$rj" | python3 -I -c "import json,sys;d=json.load(sys.stdin);print(d.get('stage',''))" 2>/dev/null)
  {
    echo; echo "## Snapshot $(date -u +%FT%TZ) (ROUND.md round $rd)"
    echo '### ready.json'; echo '```'; print -r -- "${rj:-not readable within 60 s}"; echo '```'
    echo "### last 30 lines of ready-log-${st:-unknown}.txt"; echo '```'
    [[ -n $st ]] && timeout 60 tail -n 30 $O/ready-log-$st.txt 2>&1 | cut -c1-300; echo '```'
    echo '### memory_pressure | tail -1'; memory_pressure 2>&1 | tail -1
    echo '### five largest processes by RSS'; echo '```'; ps -axo pid,ppid,rss,etime,command | sort -k3 -nr | head -5 | cut -c1-260; echo '```'
    echo "### macOS permission dialog: not checked (no screen access in this session)"
  } >> "$R"
  print -r -- "$rj" | grep -q '"ready":true' && { echo READY; exit 0; }
  [[ "$rd" != 31 ]] && { echo "ROUND_CHANGED $rd"; exit 2; }
  sleep 600
done
