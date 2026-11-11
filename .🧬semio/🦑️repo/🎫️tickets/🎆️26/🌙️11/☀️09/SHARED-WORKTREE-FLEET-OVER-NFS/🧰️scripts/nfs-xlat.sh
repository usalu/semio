# usage: xlat.sh NAME WRITER_VM READER_VM WRITER_TREE READER_TREE N INTERVAL OFFSET_SIGN
# offset (server minus worker) is measured before and after; JSON result printed
NAME=$1; WV=$2; RV=$3; WT=$4; RT=$5; N=$6; IV=$7
export LIMA_HOME=$HOME/.semio-fleet-vm
off1=$(limactl shell wrk -- python3 /tmp/nfs-clock.py measure 192.168.104.1 7777 600 | python3 -c 'import sys,json;print(json.load(sys.stdin)["offset_ms"])')
DUR=$(python3 -c "print($N*$IV+25)")
limactl shell $RV -- sudo -u semio bash -c "python3 /tmp/nfs-xread.py $RT/rehearsal-live/$NAME.txt $DUR /tmp/$NAME.read" &
sleep 4
limactl shell $WV -- sudo -u semio bash -c "python3 /tmp/nfs-xwrite.py $WT/rehearsal-live/$NAME.txt $N $IV /tmp/$NAME.write"
wait
off2=$(limactl shell wrk -- python3 /tmp/nfs-clock.py measure 192.168.104.1 7777 600 | python3 -c 'import sys,json;print(json.load(sys.stdin)["offset_ms"])')
limactl shell $WV -- cat /tmp/$NAME.write > $NAME.write; limactl shell $RV -- cat /tmp/$NAME.read > $NAME.read
if [ "$RV" = srv ]; then sign=1; else sign=-1; fi
off=$(python3 -c "print($sign*($off1+$off2)/2)")
echo "offsets before=$off1 after=$off2 ms (server minus worker); reader-minus-writer=$off"
python3 "$(dirname $0)/nfs-xjoin.py" $NAME.write $NAME.read $off
