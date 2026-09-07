# Fix Section VO Beat Timing

- Root cause: full_*_video section hosts call construct on many beats without begin_vo_beat.
- Symptom: shared clause keys deplete budget; later beats jump after ~0.3s.
- Fix: call begin_vo_beat per beat; merge vo_timing loads.

## Verification
- Simulated spent leak: second beat intro/outro → 0.3s without begin_vo_beat; full budget with reset.
- Imports of full_heating_video / full_cooling_video succeed after path fix.

## Files
- tutorial/energy/demand/Cooling/full_cooling_video.py
- tutorial/energy/demand/Heating/full_heating_video.py
- tutorial/manim_visuals.py (load_vo_timing merge)
