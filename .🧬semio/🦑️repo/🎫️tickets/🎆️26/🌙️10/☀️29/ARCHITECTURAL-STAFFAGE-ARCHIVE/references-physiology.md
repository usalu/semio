# Physiology / Respiration Reference Check (haiku-reader, web, 2026-10-29)

Binding consequence for content and engine workers: tag as `literature` only what is marked V below; everything else is
`authored-estimate` (with the reference id kept as "guided by") or `illustrative`.

| Parameter | Status | Source |
|---|---|---|
| Gupta, Lin, Chen 2010, Indoor Air 20(1):31–39 — existence, breathing flow ≈ sinusoid | V (existence), S (sinusoid) | https://research.polyu.edu.hk/en/publications/characterizing-exhaled-airflow-from-breathing-and-talking/ |
| Nose 0.71 cm² / mouth 1.2 cm² opening, nose jet ≈ 60° down | **U** → authored-estimate | not confirmed in accessible text |
| Höppe 1981 expired-air temperature | **U**; expired air up to ≈ 37 °C saturated: S (DTIC ADA210378) | https://apps.dtic.mil/sti/pdfs/ADA210378.pdf |
| Gagge, Stolwijk, Nishi 1971 two-node model | V (existence) | ASHRAE Trans 77(1) |
| Set points 36.8 / 34.1 °C, basal 6.3 & max 90 L/(m²h) skin blood flow | S (pythermalcomfort two_nodes defaults) | https://pythermalcomfort.readthedocs.io/en/latest/_modules/pythermalcomfort/models/two_nodes.html |
| 1 clo = 0.155 m²K/W; i_m ≈ 0.38 permeable clothing default | S (pythermalcomfort, ISO 9920) | https://pythermalcomfort.readthedocs.io/en/stable/documentation/clothing.html |
| Lewis relation 16.5 K/kPa | S (ISO 13731 terminology) | |
| ASHRAE respiratory loss 0.0014 M (34 − ta) + 0.0173 M (5.87 − pa) | S (quoted, ASHRAE 2005) | |
| DuBois & DuBois 1916, Arch Intern Med 17:863–871; BSA = 0.007184 W^0.425 H^0.725 (kg, cm) | V | PubMed 2520314 (reprint) |
| Adult rest 12–20 bpm, VT ≈ 0.5 L | S (textbook consensus) | |
| Child resting rates (school-age 18–25, preschool 20–28 bpm) | S, sources conflict | |
| Exercise VT/f numbers | **U** | |
| Resting rates (bpm): cat 16–40, dog 18–34, horse 10–14, dairy cow 26–50, sheep 16–34, pig 32–58 | V | Merck Vet Manual "Resting Respiratory Rates" (adapted from Reece, Dukes' Physiology of Domestic Animals, 12th ed., 2004) https://www.merckvetmanual.com/multimedia/table/resting-respiratory-rates |
| Goat, fox, birds, dog panting rates; all animal tidal volumes | **U** | |
| Kleiber 1932 Hilgardia 6(11):315, doi 10.3733/hilg.v06n11p315; 70 kcal·kg^-0.75·d^-1 ≈ 3.39 W·kg^-0.75 (Kleiber 1961) | V | |
| Scholander et al. 1950 Biol Bull 99:225–236, doi 10.2307/1538740 (fur insulation vs thickness, qualitative) | V (numbers U) | |
| Sheep fleece, cattle coat, plumage insulation numbers | **U** | |
| Nobel & Hartsock 1979 Plant Physiol (Agave deserti CAM, qualitative) | V (numbers U) | |
| Körner 1979 stomatal conductance maxima | **U** | |
