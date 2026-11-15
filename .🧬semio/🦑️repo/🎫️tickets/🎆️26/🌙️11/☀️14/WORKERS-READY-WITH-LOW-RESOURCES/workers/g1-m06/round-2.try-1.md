# g1-m06 Round 2, Try 1 (interim, the round is still running)

Result: `native_rc=1`, fleet refusal. Start 2026-11-14T16:00:29Z, end 2026-11-14T16:19:29Z. The 7776 MB memory wait of round 1 is gone; the graph step started at once.

| Item | Value |
| --- | --- |
| Projects wanted from the fleet | 703 |
| Arrived | 0 |
| Missing | 703 |
| Seconds waited | 901 |
| Projects this machine can infer itself | 192 |
| Peak resident of a process of the run | 3,991,504 KB (pid 79424, constant during the wait) |
| Memory pressure level | 1 in all 38 samples |

The log of the run does not contain the fleet lines. It shows `Reading the project graph slice (N s): flush 1 of 1.` for 991 s and then:

```
The resident service met an unexpected failure (reference 80ba70d1); try again, the details under that reference are in the service log /var/folders/_d/gdnvzj856_zbxn3kl48wfkfr0000gn/T/semio-repo/501/host-38cdf415c7fb.log.
```

So the designed refusal reaches the caller as "unexpected failure" without the words `graph-wanted` or "fleet", and without exit status 75 in the log of the run. The fleet lines are only in the service log. Its distinct `graph:` lines (seconds replaced by N, repeat count in front) and the failure record:

```
   1 graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
   1 graph: No verified change journal cursor is recorded for generation 15; verifying the tree once.
   1 graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 901 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
  90 graph: waiting for the fleet: 0 of 703 projects arrived
2026-11-14T16:19:29.289Z request-failed 80ba70d1 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":901,"missing":703,"local":192}
```

Try 2 started 5 minutes after the end of try 1. My retry wrapper now also reads the service log to recognise the refusal.
