# Current Cgroup Process Limits

Read-only observation at2026-10-30T03:59:21.843Z. This is after the earlier Cargo EAGAIN failure, not an observation at its failure boundary. Actual cgroup2 mounts and current membership were mapped before reading leaf-to-root process-controller values. Missing/inaccessible files are reported verbatim and do not establish an unlimited limit. No limit, admission policy, process, lease or environment was changed.

- /sys/fs/cgroup/user.slice/user-1000.slice/user@1000.service/app.slice/app-org.kde.konsole-2735.scope/tab(2925).scope: pids.current=1135, pids.max=max, pids.peak=1703, pids.events=max 0, pids.events.local=max 0, cgroup.controllers=cpu memory pids dmem, cgroup.subtree_control=
- /sys/fs/cgroup/user.slice/user-1000.slice/user@1000.service/app.slice/app-org.kde.konsole-2735.scope: pids.current=1196, pids.max=18980, pids.peak=1764, pids.events=max 0, pids.events.local=max 0, cgroup.controllers=cpu memory pids dmem, cgroup.subtree_control=cpu memory pids dmem
- /sys/fs/cgroup/user.slice/user-1000.slice/user@1000.service/app.slice: pids.current=1550, pids.max=max, pids.peak=2037, pids.events=max 0, pids.events.local=max 0, cgroup.controllers=cpu memory pids dmem, cgroup.subtree_control=cpu memory pids dmem
- /sys/fs/cgroup/user.slice/user-1000.slice/user@1000.service: pids.current=1764, pids.max=max, pids.peak=2251, pids.events=max 0, pids.events.local=max 0, cgroup.controllers=cpu memory pids dmem, cgroup.subtree_control=cpu memory pids dmem
- /sys/fs/cgroup/user.slice/user-1000.slice: pids.current=1767, pids.max=41757, pids.peak=2254, pids.events=max 0, pids.events.local=max 0, cgroup.controllers=cpu memory pids dmem, cgroup.subtree_control=cpu memory pids dmem
- /sys/fs/cgroup/user.slice: pids.current=1767, pids.max=max, pids.peak=2254, pids.events=max 0, pids.events.local=max 0, cgroup.controllers=cpu memory pids dmem, cgroup.subtree_control=cpu memory pids dmem
- /sys/fs/cgroup: pids.current=!ENOENT, pids.max=!ENOENT, pids.peak=!ENOENT, pids.events=!ENOENT, pids.events.local=!ENOENT, cgroup.controllers=cpuset cpu io memory hugetlb pids rdma misc dmem, cgroup.subtree_control=cpu memory pids dmem

System threads-max=126539, pid_max=4194304, load=10.29 19.64 15.83 10/2073 320848. These global values alone cannot identify the origin of Cargo's Resource temporarily unavailable. Raw metadata is in generated/cgroup-pids-audit.json.
