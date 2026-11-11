# usage: switch-mount.sh OPTIONS ; stop the three worker daemons, remount the store with OPTIONS, start them again and wait for their first scan
OPT=$1
systemctl stop semio-fleet-replica@semio fleet-w2 fleet-w3 2>/dev/null; systemctl reset-failed 2>/dev/null
umount /mnt/semio-store || exit 1
mount -t nfs4 -o "$OPT" 192.168.104.1:/srv/semio-store /mnt/semio-store || exit 1
nfsstat -m | grep -A1 "^/mnt/semio-store" | tail -1 | cut -c1-240
t0=$(date +%s)
systemctl start semio-fleet-replica@semio; bash /tmp/start-extra.sh >/dev/null
for u in semio-fleet-replica@semio fleet-w2 fleet-w3; do
  while ! journalctl -u $u --since "@$t0" --no-pager | grep -q '"event":"scan"'; do sleep 2; done
done
echo "daemons scanned after $(( $(date +%s) - t0 )) s"
