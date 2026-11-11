export SCRATCH="/private/tmp/claude-501/-Users-ueli-Library-Application-Support-Claude-scratch-workspaces-7ec0bfe6-f6ed-4f9d-bc88-c7a2d5c27fea-8dcf834a-ba0d-430c-b55b-3db29c84b30c-scratch-2026-11-10-842a8c/c043bf6f-d922-4a73-9988-8193c2d6fc91/scratchpad"
export PATH="$SCRATCH/bin:$PATH"
export R="$SCRATCH/rehearsal"
export REAL=/Users/ueli/Documents/semio
export M=$(cd $REAL && ls -d 🧰️framework/🛍️products/🦑️repo/🔨️modules/*shared-worktree)
export SCRIPT=$(cd $REAL/$M && ls -d *replica/*script.ts)
export TESTPY=$(cd $REAL/$M && ls -d *replica/*tests/*.py)
# rp <worktree> <command...>: run the replica script copy inside that worktree
rp(){ local w="$1"; shift; ( cd "$w" && bun "./$M/$SCRIPT" "$@" ); }
# rpt <worktree> <command...>: same with /usr/bin/time -l
rpt(){ local w="$1"; shift; ( cd "$w" && /usr/bin/time -l bun "./$M/$SCRIPT" "$@" ); }
# sync-code <worktree...>: copy the subsystem from the real repo into clones
sync_code(){ for w in "$@"; do rsync -a --delete "$REAL/$M/" "$w/$M/"; done; }
