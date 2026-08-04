"""Generate TTS audio for Scene 6 (Lüftungssysteme / ventilation system types) using Now I Get It TTS."""

import sys
from pathlib import Path

NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT_ROOT))

from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent

NARRATIONS = {
    "beat_1_audio": (
        "So far we have sized one ventilation system. But 'ventilation' is not a single "
        "machine—it is a family of systems, and the choice between them decides whether "
        "you can control the cooling load at all. "
        "At the top level there are only two ways to move air. Free ventilation is driven "
        "by nature: by wind pressure on the facade and by thermal buoyancy inside the "
        "building. Fan-assisted, mechanical ventilation is driven by a motor. "
        "Everything else—exhaust systems, supply systems, balanced systems, full air "
        "conditioning—hangs off these two branches. For a designed cooling load, only the "
        "mechanical branch gives us a flow rate we can actually specify."
    ),
    "beat_2_audio": (
        "Let's start with free ventilation. Open a single window, and air exchanges over "
        "that one opening: warm air leaves at the top, cooler air enters at the bottom. "
        "The driving pressure is tiny, so the air change rate is small and uncontrolled. "
        "Open windows on opposite facades and you get cross ventilation. Wind pressure "
        "pushes air straight through the building, and the air change rate rises "
        "dramatically—but it depends entirely on wind speed and direction. "
        "The third mechanism is buoyancy. Warm indoor air is less dense than outdoor air, "
        "so it rises in a shaft. The driving pressure difference equals the shaft height "
        "times gravity times the density difference. Taller shaft, bigger temperature "
        "difference, stronger flow. "
        "We can exploit this at night. Night ventilation flushes the structure with cool "
        "night air and discharges the heat stored in the thermal mass, so the building "
        "starts the next day cooler. "
        "But note the limits. Free ventilation cannot filter the air, cannot dehumidify it, "
        "and cannot recover heat. Above all, the volume flow is not adjustable—so we cannot "
        "size it against a calculated cooling load."
    ),
    "beat_3_audio": (
        "That brings us to the three mechanical base types. "
        "An exhaust-only system runs a single fan that extracts air. The room falls to a "
        "slight negative pressure, and replacement air is drawn in passively through "
        "outdoor air inlets in the facade. Simple and cheap—but the incoming air is "
        "untreated. "
        "A supply-only system does the opposite. The fan pushes filtered air in, the room "
        "goes to positive pressure, and the air escapes through joints and leaks. The "
        "supply air is treated, but you have no control over where it leaves. "
        "A balanced supply and exhaust system runs both fans. Supply and extract flows are "
        "matched, so the room stays at neutral pressure and both air streams are ducted. "
        "That difference is decisive: only when you control both streams can you filter the "
        "air, recover heat between them, and regulate humidity."
    ),
    "beat_4_audio": (
        "Controlling both air streams unlocks heat recovery. In a plate heat exchanger the "
        "supply and extract air pass each other in adjacent channels—separated, so they "
        "never mix, but thermally coupled through the plates. "
        "In winter this preheats the cold outdoor air. In summer it works in reverse and "
        "becomes cold recovery. Outdoor air at thirty-two degrees meets exhaust air leaving "
        "the room at twenty-six, and is pre-cooled to about twenty-seven degrees before it "
        "ever reaches the cooling coil. "
        "The quality of that exchange is the temperature ratio, Phi: the temperature change "
        "achieved on the supply side, divided by the maximum change available. A modern unit "
        "reaches roughly zero point eight. That means eighty percent of the ventilation "
        "cooling load from beat three is removed for free—before the chiller does any work."
    ),
    "beat_5_audio": (
        "Now, how the air is distributed inside the room matters as much as how much of it "
        "there is. There are two competing strategies. "
        "Mixing ventilation supplies air at high velocity from a ceiling diffuser. The jet "
        "induces room air, and the whole volume is stirred to one uniform temperature. This "
        "allows large temperature differences and therefore high cooling capacity—but it "
        "also distributes contaminants evenly through the room. "
        "Displacement ventilation does the opposite. Cool air is introduced slowly at floor "
        "level, with almost no momentum. It spreads across the floor as a shallow lake. "
        "Warm sources—people, computers—generate thermal plumes that carry heat and "
        "contaminants upward, where they are extracted at the ceiling. "
        "The result is a vertical temperature stratification and markedly better air quality "
        "in the occupied zone. The trade-off: the supply temperature cannot be too low or "
        "occupants feel a draught at their feet, which caps the cooling capacity at roughly "
        "forty watts per square metre."
    ),
    "beat_6_audio": (
        "Finally, what actually happens inside the air handling unit. Air passes through a "
        "sequence of components: first a filter, then heating, cooling, humidification and "
        "dehumidification. "
        "The four thermodynamic functions—heating, cooling, humidifying and "
        "dehumidifying—define the classification. A unit that only moves and filters air is "
        "a ventilation system. A unit that provides some but not all four functions is a "
        "partial air conditioning system. Only a unit that can do all four, in every season, "
        "is a full air conditioning system. "
        "And one number ties this whole series together: the air change rate, n. It is the "
        "designed volume flow divided by the room volume, in air changes per hour. It "
        "connects the cooling load we calculated in the earlier videos to the system we have "
        "just selected."
    ),
    "beat_7_audio": (
        "So, to summarise. Free ventilation is driven by wind and buoyancy, and its flow rate "
        "cannot be regulated. Of the three mechanical types, only the balanced supply and "
        "exhaust system controls both air streams, and only that makes filtration, heat "
        "recovery and humidity control possible. Heat recovery reaches a temperature ratio "
        "around zero point eight, pre-cooling the supply air for free. Mixing ventilation "
        "produces one uniform room temperature, displacement ventilation produces a "
        "stratification with better air quality but limited capacity. And the air change rate "
        "ties the volume flow back to the room. "
        "If you want to look any of this up: residential ventilation system types are in "
        "DIN 1946 part 6. Non-residential air handling systems are in DIN EN 16798 part 3, "
        "with the design air flow rates in part 1. The heat recovery temperature ratio is "
        "defined in DIN EN 308. Comfort and draught risk are in DIN EN ISO 7730. And the "
        "cooling load calculation behind the earlier videos in this series is VDI guideline "
        "2078. Note that these are cited without a year — always check the current edition."
    ),
}


def main():
    print("=== Scene 6 TTS Audio Generation (Lüftungssysteme) ===\n")
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
