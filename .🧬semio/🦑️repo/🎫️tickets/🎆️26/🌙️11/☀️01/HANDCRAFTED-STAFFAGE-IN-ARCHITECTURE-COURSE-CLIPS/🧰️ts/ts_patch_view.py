import re
p = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🔨️modules\🎨️depiction\🌡️thermal-view\🟦️.ts"
s = open(p, encoding="utf-8", newline="").read()
en_old = '${mode === "auto" ? "AUTO range: normalised to the values present in this figure, not comparable between figures." : mode === "shared" ? `Shared comparison range ${n(scale.min)} … ${n(scale.max)} ${spec.unit}, identical for every asset, scale and time.` : `Fixed range ${n(scale.min)} … ${n(scale.max)} ${spec.unit}.`}'
de_old = '${mode === "auto" ? "AUTO-Bereich: auf die in dieser Abbildung vorhandenen Werte normiert, nicht zwischen Abbildungen vergleichbar." : mode === "shared" ? `Gemeinsamer Vergleichsbereich ${n(scale.min)} … ${n(scale.max)} ${spec.unit}, gleich für jedes Asset, jeden Maßstab und jede Zeit.` : `Fester Bereich ${n(scale.min)} … ${n(scale.max)} ${spec.unit}.`}'
assert s.count(en_old) == 2 and s.count(de_old) == 2
s = s.replace(en_old, "${note.en}").replace(de_old, "${note.de}")
old = '''  const scale: Scale = scaleOf([...values.values()], centre, options.range, spec.quantity.endsWith("temperature") ? 1 : 10, spec.shared);
  const mode = options.range.mode;
'''
new = '''  const display: DisplayRange = options.range.mode === "span" && options.thermalQuantity !== "surface-temperature" ? { mode: "shared" } : options.range;
  const scale: Scale = scaleOf([...values.values()], centre, display, spec.quantity.endsWith("temperature") ? 1 : 10, spec.shared);
  const mode = display.mode;
  const note = rangeNote(display, scale, spec.unit);
'''
assert old in s
s = s.replace(old, new)
fn = '''/** 📝️ The sentence of a legend that states how its range was chosen, in both languages. */
function rangeNote(display: DisplayRange, scale: Scale, unit: string): Text {
  const bounds = `${n(scale.min)} … ${n(scale.max)} ${unit}`;
  switch (display.mode) {
    case "auto":
      return tx("AUTO range: normalised to the values present in this figure, not comparable between figures.", "AUTO-Bereich: auf die in dieser Abbildung vorhandenen Werte normiert, nicht zwischen Abbildungen vergleichbar.");
    case "shared":
      return tx(`Shared comparison range ${bounds}, identical for every asset, scale and time.`, `Gemeinsamer Vergleichsbereich ${bounds}, gleich für jedes Asset, jeden Maßstab und jede Zeit.`);
    case "span":
      return tx(`Span ±${n(display.spanK)} K around the air temperature (${bounds}), identical for every cast of one scene; values beyond it saturate.`, `Spanne ±${n(display.spanK)} K um die Lufttemperatur (${bounds}), gleich für alle Figuren einer Szene; Werte darüber hinaus sättigen.`);
    default:
      return tx(`Fixed range ${bounds}.`, `Fester Bereich ${bounds}.`);
  }
}

/** 🌡️ Builds the thermal view'''
assert "/** 🌡️ Builds the thermal view" in s
s = s.replace("/** 🌡️ Builds the thermal view", fn, 1)
open(p, "w", encoding="utf-8", newline="").write(s)
