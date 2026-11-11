# usage: mixed-outage-mac.sh MACTREE -- the NFS server of g1 stopped for 30 s while the macOS replica (hard mount over NFS 4.0) and the g1 server tree each create files; then they must converge. LIMA_HOME must be set; the mac daemon is a recorded PID in $S/fv/mac-daemon.pid.
cd "$(dirname "$0")"; W=$1
sh() { limactl shell g1 -- sudo bash -c "$*"; }
us() { limactl shell g1 -- sudo -u semio bash -c "$*"; }
limactl cp nfs-outage-edit.py g1:/tmp/nfs-outage-edit.py; limactl cp nfs-tree-digest.py g1:/tmp/nfs-tree-digest.py
rm -rf $W/nfs-outage; us "rm -rf /srv/semio/nfs-outage"; sleep 8
echo "before: mac daemon pid $(cat $S/fv/mac-daemon.pid) alive=$(kill -0 $(cat $S/fv/mac-daemon.pid) && echo yes); g1 replica pid $(sh 'systemctl show semio-fleet-replica@semio -p MainPID --value') restarts=$(sh 'systemctl show semio-fleet-replica@semio -p NRestarts --value')"
echo "t=0: stop nfs-server on g1 ($(date +%T))"
sh "systemctl stop nfs-server"; T0=$(date +%s)
(us "python3 /tmp/nfs-outage-edit.py /srv/semio g1 30 28" > o-g1.out) &
(python3 nfs-outage-edit.py $W mac 30 28 > o-mac.out) &
sleep $((30 - ($(date +%s) - T0)))
echo "t=30: start nfs-server on g1 ($(date +%T))"
sh "systemctl start nfs-server"; T1=$(date +%s)
wait
for i in $(seq 1 120); do
  a=$(us "python3 /tmp/nfs-tree-digest.py /srv/semio/nfs-outage" | tr -d '\r'); m=$(python3 nfs-tree-digest.py $W/nfs-outage)
  echo "poll $i t+$(( $(date +%s) - T1 )) s: g1 $a | mac $m"
  if [ "$a" = "$m" ] && [ "${a%% *}" = 60 ]; then echo "CONVERGED after $(( $(date +%s) - T1 )) s from the restart ($(( $(date +%s) - T0 )) s from the stop)"; break; fi
  sleep 3
done
echo "after: mac daemon pid $(cat $S/fv/mac-daemon.pid) alive=$(kill -0 $(cat $S/fv/mac-daemon.pid) && echo yes); g1 replica pid $(sh 'systemctl show semio-fleet-replica@semio -p MainPID --value') restarts=$(sh 'systemctl show semio-fleet-replica@semio -p NRestarts --value')"
