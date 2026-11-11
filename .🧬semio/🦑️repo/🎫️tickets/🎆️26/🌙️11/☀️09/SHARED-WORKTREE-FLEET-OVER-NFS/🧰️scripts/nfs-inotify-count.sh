# usage: inotify-count.sh PID... : number of inotify instances and watches held by each pid
for p in "$@"; do
  inst=0; w=0
  for fd in /proc/$p/fd/*; do
    if [ "$(readlink "$fd")" = "anon_inode:inotify" ]; then
      inst=$((inst+1)); n=$(grep -c '^inotify' /proc/$p/fdinfo/$(basename "$fd")); w=$((w+n))
    fi
  done
  echo "pid=$p inotify_instances=$inst watches=$w limit=$(cat /proc/sys/fs/inotify/max_user_watches)"
done
