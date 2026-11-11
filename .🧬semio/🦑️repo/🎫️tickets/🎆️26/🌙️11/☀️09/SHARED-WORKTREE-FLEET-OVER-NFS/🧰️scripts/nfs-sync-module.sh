# usage (inside a VM, as root): sync-module.sh ; copies the shared-worktree module from the host (read-only mount) into /srv/semio, keeping the VM topology json
set -e
H=/Users/ueli/Documents/semio; T=/srv/semio
M=$(cd $H && ls -d [!n]*/*/*/*/*shared-worktree)
TJ=$(cd $T/$M && ls -d *topology/*.json)
cp -p "$T/$M/$TJ" /tmp/topology.vm.json 2>/dev/null || true
rsync -a --delete --exclude 'node_modules' "$H/$M/" "$T/$M/" 2>/dev/null || { rm -rf "$T/$M.new"; cp -a "$H/$M" "$T/$M.new"; rm -rf "$T/$M"; mv "$T/$M.new" "$T/$M"; }
[ -f /tmp/topology.vm.json ] && cp -p /tmp/topology.vm.json "$T/$M/$TJ"
chown -R semio "$T/$M"
echo synced "$M"
