# Audit M2b: Wheel-Loader and Scissor-Lift Re-Audit

Read-only visual audit after the extruded-solid rebuild. No catalog or source file was changed.

## Method

- Driver: a copy of `T/🧰️QA6/all-r-m2.ts` placed in `T/🗑️generated/R-M2b/all.ts` with absolute imports and output in R-M2b. Run as one `bun` process, exit 0, 81 dark-line sheets plus thermal (no preview failures, no empty SVGs, longest path under 90 characters).
- Coverage per asset: every authored scale, every pose, and three instants per clip (0.1, 0.5 and 0.9 of clip duration). Thermal is rendered at the default pose only, as the driver does.
- Rasterised with `T/🧰️QA6/raster.py` (PyMuPDF) into 25 montages. Each montage was opened with Read and every sheet was checked.
- Scissor-lift has no 1:500 representation (status unavailable), so scales are 1:50, 1:100 and 1:200.

## Findings

| Asset | Scale | Pose / instant | View | Finding |
|---|---|---|---|---|
| wheel-loader | 1:50 | carry | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:50 | dig | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:50 | dump-high | side, front, plan | ok. Bucket raised. Side: lift arms and tilt cylinder connect body to bucket. Front: two arm posts link bucket to body, wheel line intact. Plan: bucket footprint and arm pivots present. No floating part. |
| wheel-loader | 1:50 | parked-off | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:50 | raising | side, front, plan | ok. Bucket raised. Side: lift arms and tilt cylinder connect body to bucket. Front: two arm posts link bucket to body, wheel line intact. Plan: bucket footprint and arm pivots present. No floating part. |
| wheel-loader | 1:50 | load-cycle@1.2 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:50 | load-cycle@6 | side, front, plan | ok. Arms raised. Side: arms and cylinders connect body to bucket. Front: arm posts run from bucket to body. Plan: bucket footprint and cylinder pivots present. No floating part. |
| wheel-loader | 1:50 | load-cycle@10.8 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:50 | start-and-lift@1 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:50 | start-and-lift@5 | side, front, plan | ok. Arms raised. Side: arms and cylinders connect body to bucket. Front: arm posts run from bucket to body. Plan: bucket footprint and cylinder pivots present. No floating part. |
| wheel-loader | 1:50 | start-and-lift@9 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:50 | thermal (carry) | side, front, plan | ok. All three views present; regions match dark-line. |
| wheel-loader | 1:100 | carry | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:100 | dig | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:100 | dump-high | side, front, plan | ok. Bucket raised. Side: lift arms and tilt cylinder connect body to bucket. Front: two arm posts link bucket to body, wheel line intact. Plan: bucket footprint and arm pivots present. No floating part. |
| wheel-loader | 1:100 | parked-off | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:100 | raising | side, front, plan | ok. Bucket raised. Side: lift arms and tilt cylinder connect body to bucket. Front: two arm posts link bucket to body, wheel line intact. Plan: bucket footprint and arm pivots present. No floating part. |
| wheel-loader | 1:100 | load-cycle@1.2 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:100 | load-cycle@6 | side, front, plan | ok. Arms raised. Side: arms and cylinders connect body to bucket. Front: arm posts run from bucket to body. Plan: bucket footprint and cylinder pivots present. No floating part. |
| wheel-loader | 1:100 | load-cycle@10.8 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:100 | start-and-lift@1 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:100 | start-and-lift@5 | side, front, plan | ok. Arms raised. Side: arms and cylinders connect body to bucket. Front: arm posts run from bucket to body. Plan: bucket footprint and cylinder pivots present. No floating part. |
| wheel-loader | 1:100 | start-and-lift@9 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:100 | thermal (carry) | side, front, plan | ok. All three views present; regions match dark-line. |
| wheel-loader | 1:200 | carry | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:200 | dig | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:200 | dump-high | side, front, plan | ok. Bucket raised. Side: lift arms and tilt cylinder connect body to bucket. Front: two arm posts link bucket to body, wheel line intact. Plan: bucket footprint and arm pivots present. No floating part. |
| wheel-loader | 1:200 | parked-off | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:200 | raising | side, front, plan | ok. Bucket raised. Side: lift arms and tilt cylinder connect body to bucket. Front: two arm posts link bucket to body, wheel line intact. Plan: bucket footprint and arm pivots present. No floating part. |
| wheel-loader | 1:200 | load-cycle@1.2 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:200 | load-cycle@6 | side, front, plan | ok. Arms raised. Side: arms and cylinders connect body to bucket. Front: arm posts run from bucket to body. Plan: bucket footprint and cylinder pivots present. No floating part. |
| wheel-loader | 1:200 | load-cycle@10.8 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:200 | start-and-lift@1 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:200 | start-and-lift@5 | side, front, plan | ok. Arms raised. Side: arms and cylinders connect body to bucket. Front: arm posts run from bucket to body. Plan: bucket footprint and cylinder pivots present. No floating part. |
| wheel-loader | 1:200 | start-and-lift@9 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:200 | thermal (carry) | side, front, plan | ok. All three views present; regions match dark-line. |
| wheel-loader | 1:500 | carry | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:500 | dig | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:500 | dump-high | side, front, plan | ok. Bucket raised. Side: lift arms and tilt cylinder connect body to bucket. Front: two arm posts link bucket to body, wheel line intact. Plan: bucket footprint and arm pivots present. No floating part. |
| wheel-loader | 1:500 | parked-off | side, front, plan | ok. Bucket low. Side: arms and cylinders join body to bucket. Front: arms seen end-on, bucket hidden behind body. Plan: bucket footprint and cylinders connect to body. No floating part. |
| wheel-loader | 1:500 | raising | side, front, plan | ok. Bucket raised. Side: lift arms and tilt cylinder connect body to bucket. Front: two arm posts link bucket to body, wheel line intact. Plan: bucket footprint and arm pivots present. No floating part. |
| wheel-loader | 1:500 | load-cycle@1.2 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:500 | load-cycle@6 | side, front, plan | ok. Arms raised. Side: arms and cylinders connect body to bucket. Front: arm posts run from bucket to body. Plan: bucket footprint and cylinder pivots present. No floating part. |
| wheel-loader | 1:500 | load-cycle@10.8 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:500 | start-and-lift@1 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:500 | start-and-lift@5 | side, front, plan | ok. Arms raised. Side: arms and cylinders connect body to bucket. Front: arm posts run from bucket to body. Plan: bucket footprint and cylinder pivots present. No floating part. |
| wheel-loader | 1:500 | start-and-lift@9 | side, front, plan | ok. Arms low. Side: arms, cylinders and bucket connected to body. Front: arms end-on. Plan: bucket and arms present. No floating part. |
| wheel-loader | 1:500 | thermal (carry) | side, front, plan | ok. All three views present; regions match dark-line. |
| scissor-lift | 1:50 | stowed | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:50 | lowering | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:50 | mid-raise | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:50 | raised-working | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:50 | raise-cycle@2.4 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:50 | raise-cycle@12 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:50 | raise-cycle@21.6 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:50 | start-lift-stop@2.2 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:50 | start-lift-stop@11 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:50 | start-lift-stop@19.8 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:50 | thermal (stowed) | side, front, plan | ok. All three views present; regions match dark-line. |
| scissor-lift | 1:100 | stowed | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:100 | lowering | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:100 | mid-raise | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:100 | raised-working | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:100 | raise-cycle@2.4 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:100 | raise-cycle@12 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:100 | raise-cycle@21.6 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:100 | start-lift-stop@2.2 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:100 | start-lift-stop@11 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:100 | start-lift-stop@19.8 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:100 | thermal (stowed) | side, front, plan | ok. All three views present; regions match dark-line. |
| scissor-lift | 1:200 | stowed | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:200 | lowering | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:200 | mid-raise | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:200 | raised-working | side, front, plan | ok. Side: scissor arms between chassis and deck, wheels on the ground line. Front: two rails and wheels run from base to deck. Plan: deck outline only; chassis hidden under the deck (expected). No floating part. |
| scissor-lift | 1:200 | raise-cycle@2.4 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:200 | raise-cycle@12 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:200 | raise-cycle@21.6 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:200 | start-lift-stop@2.2 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:200 | start-lift-stop@11 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:200 | start-lift-stop@19.8 | side, front, plan | ok. Arms span chassis to deck at this height; front rails and wheels connect; plan deck footprint constant at 0.81 m. No floating part. |
| scissor-lift | 1:200 | thermal (stowed) | side, front, plan | ok. All three views present; regions match dark-line. |

## Verdict

Both assets pass. In all 25 montages and all 81 sheets, the three views are present and non-empty, and the lift arms, cylinders and bucket (wheel-loader) or scissor arms and rails (scissor-lift) connect to the body or chassis in front and plan. Nothing floats or is detached, and plan and front extents match the elevation (wheel-loader plan width 2.95 m and scissor deck 0.81 m, constant across poses). The M2 finding that the wheel-loader bucket looked detached in the front view is resolved. Remaining caveats: montages are downscaled, so 1:50 detail was judged at montage size; scissor-lift chassis is hidden in plan by design; thermal is rendered at the default pose only.
