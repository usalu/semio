# kill -9 the daemon of w2 in the middle of publishing (the kill is triggered by head.json growing), six rounds, then converge and check the store
export LIMA_HOME=$HOME/.semio-fleet-vm
cd "$(dirname "$0")"; D="$(pwd)"
for v in srv wrk; do for f in nfs-kill9-burst.py nfs-tree-digest.py nfs-store-check.py nfs-kill-when.py; do limactl cp "$D/$f" $v:/tmp/$f; done; done
SINCE=$(limactl shell srv -- date +%s)
echo "start epoch (srv) $SINCE; host $(uptime | sed 's/.*load/load/')"
deltas=(1 30 200 700 1200 1600)
for r in 1 2 3 4 5 6; do
  d=${deltas[$((r-1))]}; rr=$((10+r))
  (limactl shell wrk -- sudo -u semio python3 /tmp/nfs-kill9-burst.py /srv/w2 $rr 1500 6 > /dev/null) &
  out=$(limactl shell wrk -- sudo python3 /tmp/nfs-kill-when.py /mnt/semio-store/v1/origins/w2/head.json fleet-w2 $d 240)
  echo "round $r: kill after head.events +$d: $out"
  wait
  for i in $(seq 1 60); do np=$(limactl shell wrk -- sudo systemctl show fleet-w2 -p MainPID --value | tr -d '\r'); echo "$out" | grep -q "\"killed_pid\":$np," || break; sleep 1; done
  echo "round $r: restarted as pid $np, NRestarts=$(limactl shell wrk -- sudo systemctl show fleet-w2 -p NRestarts --value)"
  sleep 4
done
T1=$(date +%s); want=$((6 * 1506))
for i in $(seq 1 180); do
  a=$(limactl shell wrk -- sudo -u semio bash -c 'for t in /srv/semio /srv/w2 /srv/w3; do python3 /tmp/nfs-tree-digest.py $t/nfs-kill9/; done' | tr -d '\r')
  b=$(limactl shell srv -- sudo -u semio python3 /tmp/nfs-tree-digest.py /srv/semio/nfs-kill9/ | tr -d '\r')
  echo "poll $i t+$(( $(date +%s) - T1 )) s: w1/w2/w3 = $(echo $a) | srv = $b"
  if [ "$(printf '%s\n%s\n' "$a" "$b" | sort -u | wc -l | tr -d ' ')" = 1 ]; then n=${b%% *}; [ "$n" -ge $((9048 + want)) ] && { echo "CONVERGED $(( $(date +%s) - T1 )) s after the last restart (files $n)"; break; }; fi
  sleep 5
done
