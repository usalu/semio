# usage: verify-all.sh WORKTREE... ; verify --json of each with wall time and peak RSS
for w in "$@"; do
  M=$(cd $w && ls -d [!n]*/*/*/*/*shared-worktree); R=$(cd $w/$M && ls -d *replica); SC=$(cd $w/$M/$R && ls *script.ts)
  echo "=== verify $w"
  su semio -c "cd $w/$M/$R && export PATH=/var/lib/semio/.local/bin:/home/semio/.local/bin:\$PATH; /usr/bin/time -f 'wall=%e s maxrss=%M KiB' repo ./$SC verify --json 2>&1 | grep -v '\"event\":\"progress\"' | cut -c1-400"
done
