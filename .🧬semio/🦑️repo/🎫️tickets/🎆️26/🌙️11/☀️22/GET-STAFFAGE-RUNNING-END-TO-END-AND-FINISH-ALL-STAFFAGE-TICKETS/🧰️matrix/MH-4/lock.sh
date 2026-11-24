# 🔒️ One tool process at a time for this fixer (source it in a script): waits until no other script of the batch holds $GEN/lock, takes it and gives it back at exit; a script started by one that holds the lock runs under it.
if [[ -z "$MH4_LOCKED" ]]; then
  until mkdir "$GEN/lock" 2>/dev/null; do sleep 3; done
  trap 'rmdir "$GEN/lock" 2>/dev/null' EXIT INT TERM
  export MH4_LOCKED=1
fi
