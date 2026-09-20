# Render Physical Fundamentals HQ silent

Goal: `🎯r2602🎯updateddocs🎯updateduserdocs🎯updatedtutorials`

Repo MCP was unavailable. Work logged here.

Render intro `Demo_Intro_PhysikalischeGrundlagen` plus beats 1–8 at `-qh` (1080p60), German captions on screen, no VO, ffmpeg concat with `-an`.

Order:

1. Demo_Intro_PhysikalischeGrundlagen
2. Beat1_UnsichtbareDimension
3. Beat2_KraftUndArbeit
4. Beat3_ArbeitZuLeistung
5. Beat4_Kilowattstunde
6. Beat5_Groessenordnungen
7. Beat6_Energieerhaltung
8. Beat7_Waermepumpe
9. Beat8_Ausblick

Command:

```
.venv/bin/python tutorial/energy/demand/1_physical_fundamentals/full_physical_fundamentals_video.py -q h --no-play --force
```

Re-render intro after dropping subtitle “und warum kW nicht kWh ist.” (`topic_explain_de` = “Kraft, Leistung und Energie.”). Reuse existing Beat1–8 1080p60 clips, silent concat.

Command:

```
.venv/bin/python tutorial/energy/demand/1_physical_fundamentals/full_physical_fundamentals_video.py -q h --no-play
```
