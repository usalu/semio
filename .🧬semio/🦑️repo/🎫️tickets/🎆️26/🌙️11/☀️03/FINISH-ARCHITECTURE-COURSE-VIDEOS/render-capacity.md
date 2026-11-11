# Render Capacity

A one-second native Windows GetSystemTimes sample, invoked through repo-managed Python, measured 41.2% utilization across 16 logical cores; the tool capacity snapshot showed 31.8 GB available. The energy agent independently measured 39% and 31.3 GiB free. This supports an additional disjoint later energy-balance job=2 cohort while the primary energy queue is only at source 12/13. Its source folders 88–113 plus shared 65 are not currently written by primary workers. Primary rechecks clip freshness when each queued task starts and should reuse this cohort when it arrives; the agent supervises both streams to avoid overlapping writers.

BIM geometry and Nakagin construction may be silent for many minutes before the first animation; unchanged logs alone do not prove a hang. No unrelated process was stopped. Memory headroom remains a condition for adding heavy render jobs.

November 7 workspace statfs: 106.87 GiB available of 784.4 GiB total. This covers the estimated multi-GB course normalization, but concurrent scene intermediates have no established hard bound. No cleanup performed during active renders.

## Observed Full-Detail Tree Peak

Duringcurrent124operate(flatten)phase,theexistingunevensemanticdrawingandfuture sweepdrawingremainwhilethelitnewtoweriscomposed/frozen. Read-onlystackconfirmsnativegeometrywork,notafadeloop. Observedpeakworking set13.57GiB(current13.49);hostavailable18.19GiBafterdeferringnewerworkers. The12GiBinitialgrantunderestimatedthisboundedmulti-modelphase;futurefull-detailtreejobsneedatleast16GiBreservationandmeasuredconcurrency. No source edit or premature stop was made whilehostheadroomremainedsafe. Memoryearlywarningis8GiBavailable,above4GiBbrokerfloor;keepthreeoldestworkersuntilcompletedslotsreleaseheadroom.

The124working setfellfrom13.49to8.88GiBaftercomposition;hostavailabilityrose22.60GiB. Thisconfirmsasignificanttemporaryoverlapratherthanmonotonicretentionforthatphase. Continuehost8GiBearlywarninganddonotadmitnewheavyworkersuntilcompletedslotsreleaseheadroom.

## Current Reservation Forecast

A current psutil checkpoint measured 63.91 GiB physical total and 30.72 GiB available. Native working sets were 8.98 GiB for Trees 124, 3.25 GiB for Tower 122 and 2.16 GiB for the projection capture. The approximate 18.8 GiB residual includes OS/cache/shared pages and is not a precise private-memory figure. Future reservations were 16 + 16 + 8 = 40 GiB; observed current free RAM alone did not justify adding another 8 GiB worker. Recalculate baseline and peak headroom when a job ends.

## Pending source memory evidence

Read-only review of retained reports and prior metadata samples. These are observed working-set/RSS samples or partial maxima, not proven full-run peaks or admission bounds. Later source edits change the relevant workload.

| Source | Attributed observation | Scope and uncertainty |
|---|---|---|
| 120 | 2.63 GiB sampled in work-plan.md; separate real static-strip benchmark 0.791 GiB after capture in NAKAGIN-DRAWING-PERFORMANCE.md | Early in an older production attempt; benchmark is a selected strip, not a complete source peak. Subsequent ownership changes mean neither bounds the next run. |
| 121 | 9.46 GiB observed while old pre-sprite hook built targets, stopped above its 8 GiB admission; focused lifecycle reached at least 6.88 GiB | The full production value predates sprite and lifecycle fixes; lifecycle is a separate workload. Current full-source peak remains unmeasured. |
| 123 | 3.32 GiB sampled in work-plan.md during early hook progress | Lower-bound observation only, before later ownership/timing corrections; not a full-source peak. |
| 125 | 14.54 GiB RSS observed by ticket memory-watch.py during the completed pre-handoff-fix native attempt | A point sample, not a peak. Subsequent handoff fix requires rerender; memory of that final version is unmeasured. |
| 126 | No source-attributed RSS or peak found in retained reports | 8 GiB was an admission, not a measurement; the interrupted render cannot establish its peak. |
| 127 | 9.62 GiB RSS recorded in process-control.md at deferral; source was stopped | Point sample, not a peak or completed-run bound. |
| 128 | No source-attributed RSS or peak found in retained reports | Earlier attempt reports large object count and slow frames, not memory; later 128 worker was stopped before geometry. |

Current full-suite and BIM reservations must finish or be reassessed separately. Tower 122 and Trees 124 measurements do not establish peaks for these clients. Physical total, available memory and each worker RSS are snapshots; shared/OS pages and future geometry phases prevent reliable admission forecasts from simple subtraction.

## Current v4 cohort memory sample

Physical total 63.91 GiB, available 29.68 GiB; exact native workers sampled: Excel 125 PID 41132 10.01 GiB, GH 120 PID 64820 4.24 GiB, v4 slab validator PID 15124 0.40 GiB. Approximate residual total minus available minus those RSS values is 19.58 GiB. Working-set shared pages can be counted in multiple processes and available includes reclaimable OS cache, so this is a rough accounting estimate, not a reservation or safe admission guarantee. Worker peaks can still grow. The previous 18.8 GiB residual is historical.

## Admission forecast after Excel stop

Excel session 1851 was deliberately stopped and reaped exit 1 for the confirmed source 126 fix; coordinator verified capacity root 1864 and all ten descendants gone. Excel 125 remains published and accepted. All heavy work is stopped or complete; v4 native cache gate passed all ten cases. Current sampled accepted masters: 15/26.

Sample: physical total 63.91 GiB, available 44.49 GiB, no native production worker among the largest processes. Suite 24 + BIM 117 8 + another BIM 8 and suite 24 + casting 123 16 both reserve 40 GiB. Subtracting all nominal reservations from this instantaneous available sample leaves approximately 4.49 GiB, below the 8 GiB critical warning and 12 GiB forecast warning. Neither combination should be treated as safely covered by this conservative arithmetic; reservations are not actual RSS or proven peaks, shared/reclaimable memory complicates the estimate, and future workload growth is uncertain. A smaller initial mix with measured headroom and fresh capacity approval is safer; reassess at each completion or growth. This report authorizes no launch.

## Evidence for a third BIM worker

Retained records do not establish an aggregate full-suite peak: v3–v8 suite attempts were interrupted before successful complete coverage. The 13.64 GiB observed stale GH/Excel lifecycle worker is a single old test worker, not aggregate unfiltered-suite peak; 24 GiB is a reservation. Likewise, BIM 114/115 samples near 1.25 GiB and canonical 117 sample 1.322 GiB were early construction-phase point samples, not completed full-scene peaks; 8 GiB is a reservation, not a measured maximum. Existing 124 Trees 13.57 GiB observation is a different detailed source and cannot be assigned to BIM or the suite.

Thus existing evidence does not support adding another 8 GiB BIM job alongside v9 suite plus 117 while confidently retaining 12 GiB actual available headroom through all later phases. Current available memory alone does not establish remaining peak growth. Defer until representative later-phase/complete aggregate observations or an existing slot releases; continue warning below 12 GiB and critical action below 8 GiB. No producer was started or stopped by this audit.

Canonical 117 later checkpoint: PID 66908 RSS 2.063 GiB and 2,276.3 user CPU seconds, available 39.65 GiB. No closed partial newer than attempt start 22:28:06 UTC or current receipt. This remains preparation-phase evidence, not a full-scene peak or proof of later progress.
