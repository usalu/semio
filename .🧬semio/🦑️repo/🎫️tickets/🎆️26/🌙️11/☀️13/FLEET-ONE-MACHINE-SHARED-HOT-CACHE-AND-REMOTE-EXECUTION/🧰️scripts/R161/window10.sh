#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R161
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R161
LA="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"
M=$(date -u +%M); [ "$M" -lt 5 ] || { echo "outside the window"; exit 1; }
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 200 ] || { echo "no lock"; exit 1; }; done
echo "claude-R161 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land10.py restore $SCR/bk10; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SCR/bk10
python3 $D/land10.py apply $SCR/bk10 || fail apply
rm -rf $SCR/go3; mkdir -p $SCR/go3/mini/flat; cp "$LA/📦️packages/🐹️go/go.mod" $SCR/go3/mini/flat/
n=0; find "$LA/📦️packages/🐹️go" -mindepth 2 -maxdepth 2 -name '*.go' -not -path '*📦️main*' | sort | while read f; do n=$((n+1)); cp "$f" "$SCR/go3/mini/flat/src$n.go"; done
m=0; find "$LA/🧪️tests" -name '*.go' | sort | while read f; do m=$((m+1)); cp "$f" "$SCR/go3/mini/flat/t${m}_test.go"; done
cp "$LA/📦️packages/🐹️go/🐹️.go" $SCR/go3/mini/flat/root.go
echo '{}' > "$SCR/go3/mini/📋️project.json"; echo '{"workspaces":[]}' > $SCR/go3/mini/package.json
mkdir -p "$SCR/go3/mini/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"; cp -R "$LA/🗂️registry" "$LA/🧫️fixtures" "$SCR/go3/mini/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/"
(cd $SCR/go3/mini/flat && SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio GOTOOLCHAIN=local GOPROXY=off GOWORK=off /Users/ueli/.local/go/bin/go test -run 'MachinePath|LauncherReads|ManagedWorkspaceGenerations|WorkspaceManaged|WorkspaceDetached|WorkspaceLinks|FleetStore|Provisioning' ./... 2>&1 | tail -8 > $SCR/gotest.txt; cat $SCR/gotest.txt; grep -q "^ok" $SCR/gotest.txt) || fail "go tests"
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
