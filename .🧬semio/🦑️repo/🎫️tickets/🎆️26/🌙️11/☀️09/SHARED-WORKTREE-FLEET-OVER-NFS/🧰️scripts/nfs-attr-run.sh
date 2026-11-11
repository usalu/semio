# usage: attr-run.sh MODE(stat|read) TAG ; writer on srv, watchers on wrk for the four mounts
MODE=$1; TAG=$2; N=100
export LIMA_HOME=$HOME/.semio-fleet-vm
off1=$(limactl shell wrk -- python3 /tmp/nfs-clock.py measure 192.168.104.1 7777 600 | python3 -c 'import sys,json;print(json.load(sys.stdin)["offset_ms"])')
for m in plan:/mnt/semio-store def:/mnt/probe-def ac0:/mnt/probe-ac0 lcn:/mnt/probe-lcn acd0:/mnt/probe-acd0; do
  n=${m%%:*}; p=${m#*:}
  limactl shell wrk -- sudo -u semio python3 /tmp/nfs-attr-probe.py watch $MODE $p/nfsprobe $((N*14/10+20)) /tmp/probe-$TAG-$n.log &
done
sleep 3
limactl shell srv -- sudo -u semio python3 /tmp/nfs-attr-probe.py write /srv/semio-store/nfsprobe $N 1.37 /tmp/probe-$TAG.write
wait
off2=$(limactl shell wrk -- python3 /tmp/nfs-clock.py measure 192.168.104.1 7777 600 | python3 -c 'import sys,json;print(json.load(sys.stdin)["offset_ms"])')
limactl shell srv -- cat /tmp/probe-$TAG.write > probe-$TAG.write
off=$(python3 -c "print(-($off1+$off2)/2)")
echo "mode=$MODE offsets before=$off1 after=$off2 ms; reader-minus-writer=$off"
for n in plan def ac0 lcn acd0; do limactl shell wrk -- cat /tmp/probe-$TAG-$n.log > probe-$TAG-$n.log; echo -n "$n: "; python3 ""$(pwd)"/nfs-attr-probe.py" join probe-$TAG.write probe-$TAG-$n.log $off; done
