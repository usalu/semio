# usage: mixed-outage.sh -- NFS server of g1 stopped for 30 s while g1 (server tree), g2 (server tree) and the g2 worker (/srv/w2g over NFS) edit; relay g2<-g1 hangs on its hard peer mount. LIMA_HOME must be set.
cd "$(dirname "$0")"
sh() { n=$1; shift; limactl shell $n -- sudo bash -c "$*"; }
us() { n=$1; shift; limactl shell $n -- sudo -u semio bash -c "$*"; }
for n in g1 g2; do for f in nfs-outage-edit.py nfs-tree-digest.py; do limactl cp $f $n:/tmp/$f; done; done
pids() { for u in "g1 semio-fleet-replica@semio" "g1 semio-fleet-relay" "g2 semio-fleet-replica@semio" "g2 semio-fleet-relay" "g2 fleet-g2w1"; do set -- $u; echo "$1 $2 pid=$(sh $1 "systemctl show $2 -p MainPID --value") restarts=$(sh $1 "systemctl show $2 -p NRestarts --value")"; done; }
echo "before:"; pids
for t in "g1 /srv/semio" "g2 /srv/semio" "g2 /srv/w2g"; do set -- $t; sh $1 "rm -rf $2/nfs-outage"; done
sleep 8
echo "t=0: stop nfs-server on g1 ($(date +%T))"
sh g1 "systemctl stop nfs-server"
T0=$(date +%s)
(us g1 "python3 /tmp/nfs-outage-edit.py /srv/semio g1 30 28" > o-g1.out) &
(us g2 "python3 /tmp/nfs-outage-edit.py /srv/semio g2s 30 28" > o-g2s.out) &
(us g2 "python3 /tmp/nfs-outage-edit.py /srv/w2g g2w 10 28" > o-g2w.out) &
sleep $((30 - ($(date +%s) - T0)))
echo "t=30: start nfs-server on g1 ($(date +%T))"
sh g1 "systemctl start nfs-server"
T1=$(date +%s)
wait
echo "restarted; waiting for convergence"
for i in $(seq 1 150); do
  a=$(us g1 "python3 /tmp/nfs-tree-digest.py /srv/semio/nfs-outage" | tr -d '\r'); b=$(us g2 "python3 /tmp/nfs-tree-digest.py /srv/semio/nfs-outage" | tr -d '\r'); c=$(us g2 "python3 /tmp/nfs-tree-digest.py /srv/w2g/nfs-outage" | tr -d '\r')
  echo "poll $i t+$(( $(date +%s) - T1 )) s: g1 $a | g2 server $b | g2 worker $c"
  if [ "$a" = "$b" ] && [ "$b" = "$c" ] && [ "${a%% *}" = 70 ]; then echo "CONVERGED after $(( $(date +%s) - T1 )) s from the restart ($(( $(date +%s) - T0 )) s from the stop)"; break; fi
  sleep 2
done
echo "after:"; pids
sh g1 "dmesg | grep -i 'nfs\|server not' | tail -3"; sh g2 "dmesg | grep -i 'not responding\|nfs: server' | tail -5"
sh g2 "journalctl -u semio-fleet-relay --since '-4min' --no-pager | grep -v progress | grep -i 'error\|unreach\|reachable' | cut -c1-220 | head"
