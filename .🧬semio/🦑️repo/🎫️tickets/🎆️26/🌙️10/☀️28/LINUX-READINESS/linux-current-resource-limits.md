# Current Linux Resource Limits

Observed at 2026-10-30T02:22:24.846Z after the parent Cargo failure. This is current host context, not a sample captured at the failed spawn, and does not itself establish the EAGAIN cause. No limit or process was changed.

## /proc/self/limits

```text
Limit                     Soft Limit           Hard Limit           Units     
Max cpu time              unlimited            unlimited            seconds   
Max file size             unlimited            unlimited            bytes     
Max data size             unlimited            unlimited            bytes     
Max stack size            8388608              unlimited            bytes     
Max core file size        unlimited            unlimited            bytes     
Max resident set          unlimited            unlimited            bytes     
Max processes             63269                63269                processes 
Max open files            524288               524288               files     
Max locked memory         8388608              8388608              bytes     
Max address space         unlimited            unlimited            bytes     
Max file locks            unlimited            unlimited            locks     
Max pending signals       63269                63269                signals   
Max msgqueue size         819200               819200               bytes     
Max nice priority         0                    28                   
Max realtime priority     0                    0                    
Max realtime timeout      unlimited            unlimited            us        
```

## /proc/self/cgroup

```text
0::/user.slice/user-1000.slice/user@1000.service/app.slice/app-org.kde.konsole-2735.scope/tab(2925).scope
```

## /sys/fs/cgroup/pids.current

Read error: ENOENT

## /sys/fs/cgroup/pids.max

Read error: ENOENT

## /sys/fs/cgroup/pids.peak

Read error: ENOENT

## /sys/fs/cgroup/pids.events

Read error: ENOENT

## /proc/sys/kernel/threads-max

```text
126539
```

## /proc/sys/kernel/pid_max

```text
4194304
```

## /proc/sys/fs/file-nr

```text
18728	0	9223372036854775807
```

## Root Current Pressure Observation

At03:57:17.688Z, actual /proc metadata reported16229024KiBtotal RAM,3758096KiBavailable,9162744KiBswap and1707132KiBfree swap; memory pressure some avg6015.69/full avg6012.43, load29.41/25.60/16.96. Root shell request296300 had already exited; native-dependencies296362 remained its actual repository:command owner. No limit, floor, swap ceiling, process or peer was modified. Separate inner SIGTERM receipt does not establish the sender. Bounded raw evidence is generated/root-canonical-pressure.json.
