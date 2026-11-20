# Staffage overview in the in-app browser

The overview ran in a visible 1440 × 900 viewport. The harness used native animation frames and checked the shared-border pending marker on every sampled frame.

| Build | Sample time | Frames | Median interval | Pending samples |
| --- | ---: | ---: | ---: | ---: |
| Before follow-up changes | 18.45 s | 5 | 3,091 ms | 5 |
| Inert archive grid mounts visible cards | 17.36 s | 9 | 2,605 ms | 5 |
| Before moving ambient drawing to a worker | 16.36 s | 7 | 1,683 ms | 3 |
| Ambient module worker and compositor fixes | 15.06 s | 156 | 88 ms | 29 |
| Closed editors suspended and category loading coordinated | 15.05 s | 550 | 27 ms | 5 |

The fourth sample retained 43 rendered frieze drawings throughout. The pending marker cleared in 127 frames. All 29 new invalidations came from the compositor's resize observer. The 95th-percentile interval was 179 ms. Compositor commits occupied 1,767 ms and worker-message handlers accounted for 229 ms of long-frame script time. The trace still recorded 930 value updates on range controls and 465 data-time updates in the overview. See [profile harness](profile.html).

The final sample retained 41 drawings throughout while the shared catalog was being edited. The 95th-percentile interval was 46 ms, with 37 long animation frames. The pending marker was absent in 545 of 550 samples. Five insertions and five removals accounted for all ten pending-attribute mutations. Compositor commits occupied 114 ms across the sample, with 5 ms in the clock tick wrapper and 1 ms in the drawing callback wrapper. The worker continued to deliver rendered artwork. No resize callback appeared among the sources of new pending work.
