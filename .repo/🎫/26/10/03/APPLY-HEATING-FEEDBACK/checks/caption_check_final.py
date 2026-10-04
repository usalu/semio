import sys
from pathlib import Path
FINAL = Path(__file__).resolve().parents[7] / "tutorial" / "energy" / "demand" / "Heating" / "final_calculation"
sys.path.insert(0, str(FINAL))
import merged_scenes as ms
from manim import VGroup
from manim_visuals import caption_bar
for name in ("ReviewingHeatLosses", "Scene2", "ReviewingHeatGains", "Scene4", "UltimateEnergyBalance", "AnlagenVerluste"):
    for key, _en, de in getattr(ms, name).NARRATION:
        label = caption_bar(de)[1]
        lines = len(label.submobjects) if isinstance(label, VGroup) else 1
        print(f"{name:24s} {key:10s} {lines} lines {len(de):3d} chars{'  !!' if lines > 2 else ''}")
print(f"PHI_T={ms._PHI_T:.1f} H_T={ms._H_T:.2f} PHI_V={ms._PHI_V:.1f} H_V={ms._H_V:.2f} PHI_LOSS={ms._PHI_LOSS:.1f}")
print(f"Q_LOSS={ms._Q_LOSS:.1f} shown={ms._Q_LOSS_SHOWN} GT_sum={ms._gt_profile().sum():.2f}")
print(f"SOLAR_PEAK={ms._SOLAR_PEAK:.4f} ETA_LIGHT={ms._ETA_LIGHT:.4f} ETA_HEAVY={ms._ETA_HEAVY:.4f} A_GAIN={ms._utilization(ms._SOLAR_PEAK, 0)[1]:.2f}")
print(f"Q_USE={ms._Q_USE:.1f} Q_H={ms._Q_H:.1f} CE={ms._Q_CE:.1f} D={ms._Q_D:.1f} S={ms._Q_S:.1f} G={ms._Q_G:.1f} Q_E={ms._Q_E:.1f}")
