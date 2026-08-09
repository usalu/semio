# Center Cooling Layout — Scene 2 Remaining Beats

Ticket: `26/08/08/CENTER-COOLING-LAYOUT`  
Date: 2026-08-08  
Scope: Migrate remaining `merged_scenes.py` beats into `scene_2.py` (five-rule template), mid-screen centering only.

## Goal

`tutorial/energy` — Cooling internal-gains pedagogy on the shared Manim template.  
Repo MCP unavailable this session; ticket folder used manually (already open).

## Beat classes added

| Class | Source | Beat subtitle (DE) |
|-------|--------|--------------------|
| `Beat5_SensibleVsLatent` | `SensibleVsLatentHeat` | Sensible versus latente Wärme |
| `Beat6_HeatTrap` | `Scene8` | Die isolierte Wärmefalle |
| `Beat7_HvacCooling` | `Scene9` | HLK-Kühlbedarf |
| `Beat8_Mitigation` | `Scene10` | Minderung & intelligentes Design |

Skipped: `FullHiddenHeatVideo` (compose helper).

Persistent module title remains `TITLE_DE = "Interne Wärmegewinne"`; Beat5+ use `self.add(title)` + `beat_subtitle(...)`.

## Mid-screen layout choices

Safe band: below title/beat_subtitle (~y≤2.0) and above `formula_panel` (edge_buff≈1.7) + `caption_bar` (edge_buff≈0.35). Main group centers ~y=0…+0.4.

- **Beat5:** Split panel `mid_y=0.25`; headers ~+1.6; gauges/tags pulled up from legacy y≈−2.5 → `mid_y−1.55`. Summary banner → `formula_panel`: `Q̇_ges = Q̇_sens + Q̇_lat [W]`.
- **Beat6:** `room_c = UP*0.3`, inner 4.4×2.2, ins 0.45. Explain callouts moved *inside* room (`room_c + DOWN*0.75`) instead of under outer envelope.
- **Beat7:** `room_c = UP*0.25`, room 6.6×2.55 (was 7.0×3.4). Step labels at floor *inside* room. Particle flow count 16 / `flow_rt=10` (same motion pattern).
- **Beat8:** `room_c = UP*0.55`, room 7.2×2.35; control strip `next_to(room, DOWN, buff=0.18)` height 0.9 so strip bottom stays above formula zone. Legacy step texts at `to_edge(DOWN)` replaced by `caption_bar` clauses.

Palette: local heat hexes mapped to `P_ORANGE` / `P_YELLOW` / `P_RED` / `P_CYAN` / `P_TEAL` where practical; a few furniture fills (`#1A1E28`, `#243040`, `#3A4050`) kept for line-art contrast.

## Pipeline

- `generate_audio.py` — BEATS includes Beat5–8 → `beat_5..8_audio.mp3`
- `build_full_video.py` — scenes list Beat5–8 with matching audio stems

## Smoke

`manim -ql -s` → ticket `media_s2_rest/images/scene_2/`:

- `Beat5_SensibleVsLatent` ✓ — mid split + formula clear of title/caption
- `Beat6_HeatTrap` ✓ — room mid-centered; trapped-heat end state
- `Beat7_HvacCooling` ✓ — shortened room + particle flow
- `Beat8_Mitigation` ✓ — room + control strip above formula/caption band

## Files

- Updated: `tutorial/energy/demand/Cooling/2_internal_gains/scene_2.py`
- Updated: `…/generate_audio.py`, `…/build_full_video.py`
- Ticket draft (kept): `beats_5_8_append.py`, this notes file
- Untouched: `merged_scenes.py` (legacy source of truth left in place)
