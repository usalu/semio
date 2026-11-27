# fseventsd host memory starvation, 2026-11-26

At 16:48 CET on the coordinator Mac (16 GiB, uptime 11 days), `top -o mem` reported `fseventsd` (PID 412) at **14 GB, of which 11 GB was compressed**. The next process used 859 MB. Swap was 8.5 of 9.2 GB used. The capacity gate attributed its refusals to "the host itself (nothing of ours)". The warden cancelled the cargo `bun --eval` helpers of the EN 1990 replay and of the component publication under critical memory pressure.

Earlier on the same machine, `fseventsd` used 1.8 GB RSS (14:20 CET), and the Wave 1122 audit recorded growth from 6.4 to 7.6 GiB within 20 minutes ([audit-H5-pulse.md](../☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/audit-H5-pulse.md)). The daemon grows with the file churn of the replicated worktree and its in-tree `.cache`.

## Remedy

`fseventsd` runs as root and is restarted by launchd. Only an administrator can reclaim it:

```bash
sudo killall fseventsd
```

`sudo -n` requires a password on this machine. No agent has administrator rights here, so the coordinator could not apply the fix. No user-level FSEvents client is listed by `lsof /dev/fsevents`. Moving high-churn build output out of the watched volume path would address the cause, but this is shared fleet infrastructure owned by other tickets.
