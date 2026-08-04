"""Generate TTS audio for Scene 5 (Systemauslegung / Mechanical Ventilation) using Now I Get It TTS."""

import sys
from pathlib import Path

NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT_ROOT))

from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent

NARRATIONS = {
    "beat_1_audio": (
        "The room is already saturated with heat. To actively remove that cooling load, "
        "we rely on a mechanical supply and exhaust system—a Zu-Abluftsystem. "
        "Cool outdoor air is pushed in through a supply grille, while warm indoor air "
        "is drawn out through an exhaust grille at the same time. "
        "Watch the airflow: blue particles enter, spread through the room, and displace "
        "the red heat. The engineering question that follows is precise: "
        "what volumetric flow rate do we need to neutralize this heat load?"
    ),
    "beat_2_audio": (
        "The convective cooling capacity of that airflow is defined by a fundamental "
        "thermodynamic product. Q-dot V equals air density times specific heat capacity "
        "times the temperature difference between cool supply air and the warm room, "
        "times q v R—the required room airflow volume. "
        "Density and specific heat are material properties of air. Delta theta is the "
        "designed temperature lift we allow. And q v R is the free design variable: "
        "how much air we must move every second to carry the heat away."
    ),
    "beat_3_audio": (
        "Thermal equilibrium demands that cooling capacity equals the solar transmission "
        "load we calculated earlier. So we set Q-dot V equal to Q-dot S,tr, then rearrange "
        "algebraically to isolate q v R. "
        "Substituting standard air constants and our designed temperature drop yields the "
        "exact volumetric flow rate required to keep the room climate stable. "
        "That flow rate is the sizing target for the ventilation system."
    ),
    "beat_4_audio": (
        "With the required air volume known, we size the physical duct using continuity. "
        "Volumetric flow equals mean air velocity times cross-sectional area. "
        "Particles stream through the duct: the same flow can pass through a narrow section "
        "at high speed, or a wider section at lower speed. "
        "To limit noise, engineers cap velocity—often around two point five meters per second. "
        "With velocity fixed, the equation directly gives the required duct area A."
    ),
    "beat_5_audio": (
        "Modern ventilation ducts are typically round, so the cross-section is a circle: "
        "A equals pi r squared. Rearranging for the radius gives r equals the square root "
        "of A over pi. "
        "That final step turns abstract heat and airflow requirements into a tangible "
        "duct diameter that manufacturing and construction can build. "
        "From heat load, to volume flow, to area, to radius—the system is fully sized."
    ),
}


def main():
    print("=== Scene 5 TTS Audio Generation (Systemauslegung) ===\n")
    for name, text in NARRATIONS.items():
        out_path = BASE_DIR / f"{name}.mp3"
        print(f"Generating: {name} ...")
        try:
            result_path, skipped = synthesize_narration(text, out_path)
            if skipped:
                print("  ⚠ Skipped (no TTS API key configured or empty text)")
            elif result_path:
                print(f"  ✓ Saved → {result_path}")
            else:
                print("  ✗ Failed (no output)")
        except Exception as e:
            print(f"  ✗ Error: {e}")
    print("\nDone!")


if __name__ == "__main__":
    main()
