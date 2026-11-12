import sys
sys.path.insert(0, ".")
from migrate import T, run
H = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/"
X = "stage.transition"

S = "cooling.system-sizing."
run(H + "📐️system-sizing",
    [("ReplacementTransform(solar_chip, hygiene_chip),\n                  ReplacementTransform(final_eq, num_eq), ReplacementTransform(final_box, num_box), run_time=0.9)",
      f'{X}("{S}airflow.solar-to-hygiene-chip", source=solar_chip, target=hygiene_chip),\n                  {X}("{S}airflow.final-to-numeric-equation", source=final_eq, target=num_eq), {X}("{S}airflow.final-to-numeric-box", source=final_box, target=num_box), run_time=0.9)'),
     ("self.play(ReplacementTransform(hygiene_chip, duct_chip), ReplacementTransform(num_eq, area_eq),\n                  ReplacementTransform(num_box, area_box), FadeOut(factor_group), run_time=0.9)",
      f'self.play({X}("{S}airflow.hygiene-to-duct-chip", source=hygiene_chip, target=duct_chip), {X}("{S}airflow.numeric-to-area-equation", source=num_eq, target=area_eq),\n                  {X}("{S}airflow.numeric-to-area-box", source=num_box, target=area_box), FadeOut(factor_group), run_time=0.9)')],
    [T(S + "airflow.solar-to-hygiene-chip", "transformation", S + "airflow.solar-chip", S + "airflow.hygiene-chip", "replace", 0.9),
     T(S + "airflow.final-to-numeric-equation", "transformation", S + "airflow.symbolic-equation", S + "airflow.numeric-equation", "replace", 0.9),
     T(S + "airflow.final-to-numeric-box", "continuation", S + "airflow.symbolic-box", S + "airflow.numeric-box", "replace", 0.9),
     T(S + "airflow.hygiene-to-duct-chip", "transformation", S + "airflow.hygiene-chip", S + "airflow.duct-chip", "replace", 0.9),
     T(S + "airflow.numeric-to-area-equation", "transformation", S + "airflow.numeric-equation", S + "airflow.area-equation", "replace", 0.9),
     T(S + "airflow.numeric-to-area-box", "continuation", S + "airflow.numeric-box", S + "airflow.area-box", "replace", 0.9)],
    ["Beat3_IsolateAirflow"])

S = "cooling.transmission-humidity."
run(H + "💧️transmission-humidity",
    [("ReplacementTransform(opaque_borders.copy(), u_token),\n            ReplacementTransform(area_fill, a_token),",
      f'{X}("{S}formula.borders-to-u", source=opaque_borders, target=u_token),\n            {X}("{S}formula.area-to-a", source=area_fill, target=a_token),'),
     ('ReplacementTransform(u_token, items["u"]),\n            ReplacementTransform(a_token, items["a"]),',
      f'{X}("{S}formula.u-into-slot", source=u_token, target=items["u"]),\n            {X}("{S}formula.a-into-slot", source=a_token, target=items["a"]),')],
    [T(S + "formula.borders-to-u", "abstraction", S + "formula.opaque-borders", S + "formula.u-token", "replace", 1.4, copy=True),
     T(S + "formula.area-to-a", "abstraction", S + "formula.area-fill", S + "formula.a-token", "replace", 1.4),
     T(S + "formula.u-into-slot", "continuation", S + "formula.u-token", S + "formula.u-slot", "replace", 1.2),
     T(S + "formula.a-into-slot", "continuation", S + "formula.a-token", S + "formula.a-slot", "replace", 1.2)],
    ["Beat1_TransmissionOpaque"])

S = "cooling.internal-gains."
run(H + "👥️internal-gains",
    [('self.play(ReplacementTransform(facade, room["shell"]), FadeIn(room["glass"])', f'self.play({X}("{S}room.facade-to-shell", source=facade, target=room["shell"]), FadeIn(room["glass"])'),
     ("*[FadeTransform(fig, dot) for fig, dot in zip(figures, dots)],", f'{X}("{S}activity.figures-to-dots", source=figures, target=dots),'),
     ("FadeTransform(single, grid_icons[0]),\n            LaggedStart(*[TransformFromCopy(grid_icons[0], icon) for icon in grid_icons[1:]], lag_ratio=0.03),",
      f'{X}("{S}hall.single-to-first-icon", source=single, target=grid_icons[0]),\n            {X}("{S}hall.first-icon-to-grid", source=[grid_icons[0]], target=list(grid_icons[1:])),')],
    [T(S + "room.facade-to-shell", "concretization", S + "room.facade", S + "room.shell", "replace", 1.8),
     T(S + "activity.figures-to-dots", "abstraction", S + "activity.figures", S + "activity.dots", "fade", 2.2, relation="many-to-many"),
     T(S + "hall.single-to-first-icon", "continuation", S + "hall.single-person", S + "hall.first-icon", "fade", 2.0),
     T(S + "hall.first-icon-to-grid", "concretization", S + "hall.first-icon-seed", S + "hall.grid-icons", "split", 2.0, relation="one-to-many", corr="pairs", pairs=[["0", "0"]], lag=0.06, copy=True)],
    ["Beat1_OfficeRoom", "Beat2_HumanFactor"])

S = "cooling.solar-radiation."
run(H + "☀️solar-radiation",
    [("ReplacementTransform(building, cube), Create(sun_path),", f'{X}("{S}geometry.building-to-cube", source=building, target=cube), Create(sun_path),'),
     ('self.play(ReplacementTransform(sun.copy(), eq_items["i"]), run_time=1.4)', f'self.play({X}("{S}equation.sun-to-irradiance", source=sun, target=eq_items["i"]), run_time=1.4)')],
    [T(S + "geometry.building-to-cube", "abstraction", S + "geometry.building", S + "geometry.cube", "replace", 1.5),
     T(S + "equation.sun-to-irradiance", "abstraction", S + "equation.sun", S + "equation.irradiance-symbol", "replace", 1.4, copy=True)],
    ["Beat1_SolarIrradiance"])

S = "cooling.summer-strategy."
run(H + "🌤️summer-strategy",
    [("ReplacementTransform(chip_nat, chip_mech), *heat(1.4, 0.18),", f'{X}("{S}reserve.natural-to-mechanical-chip", source=chip_nat, target=chip_mech), *heat(1.4, 0.18),')],
    [T(S + "reserve.natural-to-mechanical-chip", "transformation", S + "reserve.natural-chip", S + "reserve.mechanical-chip", "replace", 1.4)],
    ["Beat1_LastZuerstSenken"])
