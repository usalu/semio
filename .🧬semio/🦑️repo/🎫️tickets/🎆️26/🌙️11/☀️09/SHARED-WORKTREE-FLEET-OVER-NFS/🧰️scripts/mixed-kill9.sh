# usage: mixed-kill9.sh -- SIGKILL (by recorded PID) of the relay of g2 in the middle of a copy from g1: g1 writes bursts of 1500 small + 6 large files, the kill fires when the store of g2 has in-flight writes in v1/tmp. LIMA_HOME must be set.
cd "$(dirname "$0")"
sh() { n=$1; shift; limactl shell $n -- sudo bash -c "$*"; }
us() { n=$1; shift; limactl shell $n -- sudo -u semio bash -c "$*"; }
for n in g1 g2; do for f in nfs-kill9-burst.py nfs-tree-digest.py nfs-store-check.py mixed-kill-tmp.py; do limactl cp $f $n:/tmp/$f; done; done
sh g1 "rm -rf /srv/semio/nfs-kill9"; sleep 5
echo "load: $(uptime | sed 's/.*load/load/')"
deltas=(1 1 2 3 4)
for r in 1 2 3 4 5; do
  d=${deltas[$((r-1))]}; rr=$((10+r))
  (us g1 "python3 /tmp/nfs-kill9-burst.py /srv/semio $rr 1500 6" > /dev/null) &
  out=$(sh g2 "python3 /tmp/mixed-kill-tmp.py /srv/semio-store/v1/tmp semio-fleet-relay $d 240" | tr -d '\r')
  echo "round $r: relay SIGKILL when v1/tmp of the g2 store held +$d in-flight write(s): $out"
  wait
  sleep 6
  echo "round $r: relay pid now $(sh g2 'systemctl show semio-fleet-relay -p MainPID --value') NRestarts=$(sh g2 'systemctl show semio-fleet-relay -p NRestarts --value')"
done
T1=$(date +%s); want=$((5 * 1506))
for i in $(seq 1 180); do
  a=$(us g1 "python3 /tmp/nfs-tree-digest.py /srv/semio/nfs-kill9/" | tr -d '\r'); b=$(us g2 "python3 /tmp/nfs-tree-digest.py /srv/semio/nfs-kill9/" | tr -d '\r'); c=$(us g2 "python3 /tmp/nfs-tree-digest.py /srv/w2g/nfs-kill9/" | tr -d '\r')
  echo "poll $i t+$(( $(date +%s) - T1 )) s: g1 $a | g2 server $b | g2 worker $c"
  if [ "$a" = "$b" ] && [ "$b" = "$c" ] && [ "${a%% *}" = "$want" ]; then echo "CONVERGED $(( $(date +%s) - T1 )) s after the last kill+restart (files ${a%% *})"; break; fi
  sleep 5
done
for n in g1 g2; do echo "store check $n: $(us $n 'python3 /tmp/nfs-store-check.py /srv/semio-store')"; done
