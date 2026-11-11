# NFS server stopped for 30 s while three worker replicas and the server replica keep editing
export LIMA_HOME=$HOME/.semio-fleet-vm
cd "$(dirname "$0")"
for v in srv wrk; do limactl cp nfs-outage-edit.py $v:/tmp/nfs-outage-edit.py; limactl cp nfs-tree-digest.py $v:/tmp/nfs-tree-digest.py; done
limactl shell wrk -- sudo bash -c 'for u in semio-fleet-replica@semio fleet-w2 fleet-w3; do echo "$u pid=$(systemctl show $u -p MainPID --value) restarts=$(systemctl show $u -p NRestarts --value)"; done; rm -rf /srv/semio/nfs-outage'
limactl shell srv -- sudo bash -c 'rm -rf /srv/semio/nfs-outage; echo "srv pid=$(systemctl show semio-fleet-replica@semio -p MainPID --value) relay=$(systemctl show semio-fleet-relay -p MainPID --value)"; cat /proc/fs/nfsd/nfsv4gracetime /proc/fs/nfsd/nfsv4leasetime 2>/dev/null | tr "\n" " "; echo'
sleep 5
echo "t=0: stop nfs-server ($(date +%T))"
limactl shell srv -- sudo systemctl stop nfs-server
T0=$(date +%s)
(limactl shell wrk -- sudo -u semio python3 /tmp/nfs-outage-edit.py /srv/semio w1 30 28 > w1.out) &
(limactl shell wrk -- sudo -u semio python3 /tmp/nfs-outage-edit.py /srv/w2 w2 10 28 > w2.out) &
(limactl shell srv -- sudo -u semio python3 /tmp/nfs-outage-edit.py /srv/semio srv 30 28 > srv.out) &
sleep $((30 - ($(date +%s) - T0)))
echo "t=30: start nfs-server ($(date +%T))"
limactl shell srv -- sudo systemctl start nfs-server
T1=$(date +%s)
wait
echo "restarted; waiting for convergence"
want=70
for i in $(seq 1 120); do
  a=$(limactl shell wrk -- sudo -u semio bash -c 'for t in /srv/semio /srv/w2 /srv/w3; do python3 /tmp/nfs-tree-digest.py $t/nfs-outage; done')
  b=$(limactl shell srv -- sudo -u semio python3 /tmp/nfs-tree-digest.py /srv/semio/nfs-outage)
  echo "poll $i t+$(( $(date +%s) - T1 )) s: w1/w2/w3 = $(echo $a) | srv = $b"
  if [ "$(printf '%s\n%s\n' "$a" "$b" | sort -u | wc -l)" = 1 ] && [ "${b%% *}" = "$want" ]; then echo "CONVERGED after $(( $(date +%s) - T1 )) s from the restart ($(( $(date +%s) - T0 )) s from the stop)"; break; fi
  sleep 2
done
limactl shell wrk -- sudo bash -c 'for u in semio-fleet-replica@semio fleet-w2 fleet-w3; do echo "$u pid=$(systemctl show $u -p MainPID --value) restarts=$(systemctl show $u -p NRestarts --value)"; done; rm -rf /srv/semio/nfs-outage'
limactl shell srv -- sudo bash -c 'rm -rf /srv/semio/nfs-outage; echo "srv pid=$(systemctl show semio-fleet-replica@semio -p MainPID --value) relay=$(systemctl show semio-fleet-relay -p MainPID --value)"; cat /proc/fs/nfsd/nfsv4gracetime /proc/fs/nfsd/nfsv4leasetime 2>/dev/null | tr "\n" " "; echo'
sleep 5
echo "t=0: stop nfs-server ($(date +%T))"
limactl shell srv -- sudo systemctl stop nfs-server
T0=$(date +%s)
(limactl shell wrk -- sudo -u semio python3 /tmp/nfs-outage-edit.py /srv/semio w1 30 28 > w1.out) &
(limactl shell wrk -- sudo -u semio python3 /tmp/nfs-outage-edit.py /srv/w2 w2 10 28 > w2.out) &
(limactl shell srv -- sudo -u semio python3 /tmp/nfs-outage-edit.py /srv/semio srv 30 28 > srv.out) &
sleep $((30 - ($(date +%s) - T0)))
echo "t=30: start nfs-server ($(date +%T))"
limactl shell srv -- sudo systemctl start nfs-server
T1=$(date +%s)
wait
echo "restarted; waiting for convergence"
want=70
for i in $(seq 1 120); do
  a=$(limactl shell wrk -- sudo -u semio bash -c 'for t in /srv/semio /srv/w2 /srv/w3; do python3 /tmp/nfs-tree-digest.py $t/nfs-outage | md5sum; done | sort | uniq -c | tr "\n" " "; ls /srv/w3/nfs-outage 2>/dev/null | wc -l')
  b=$(limactl shell srv -- sudo -u semio bash -c 'python3 /tmp/nfs-tree-digest.py /srv/semio/nfs-outage | md5sum')
  echo "poll $i t+$(( $(date +%s) - T1 )) s: $a | srv $b"
  n3=$(echo "$a" | awk '{print $NF}'); same=$(echo "$a" | grep -c "3 ")
  w=$(echo "$a" | awk '{print $2}'); s=$(echo "$b" | awk '{print $1}')
  if [ "$n3" = "$want" ] && [ "$same" = 1 ] && [ "$w" = "$s" ]; then echo "CONVERGED after $(( $(date +%s) - T1 )) s from the restart ($(( $(date +%s) - T0 )) s from the stop)"; break; fi
  sleep 3
done
limactl shell wrk -- sudo bash -c 'for u in semio-fleet-replica@semio fleet-w2 fleet-w3; do echo "$u pid=$(systemctl show $u -p MainPID --value) restarts=$(systemctl show $u -p NRestarts --value) active=$(systemctl is-active $u)"; journalctl -u $u --since "-4min" --no-pager | grep -v "\"progress\"" | grep -iE "error|unreach|reach|stale|fail" | head -5 | cut -c1-200; done; dmesg | grep -i nfs | tail -5'
limactl shell srv -- sudo bash -c 'journalctl -u semio-fleet-replica@semio -u semio-fleet-relay --since "-4min" --no-pager | grep -v "\"progress\"" | grep -iE "error|fail" | head -5 | cut -c1-200; echo "srv active: $(systemctl is-active semio-fleet-replica@semio semio-fleet-relay | tr "\n" " ")"'
