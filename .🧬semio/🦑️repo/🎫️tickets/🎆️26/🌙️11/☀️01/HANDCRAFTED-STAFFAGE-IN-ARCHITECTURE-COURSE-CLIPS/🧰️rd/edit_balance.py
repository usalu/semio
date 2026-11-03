import glob

p = glob.glob("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🧱️balance/*.ts")[0]
t = open(p, encoding="utf-8", newline="").read()
assert "\r\n" not in t


def rep(a, b):
    global t
    assert a in t, a
    t = t.replace(a, b, 1)


rep("import type { Environment, Material, PhaseChange, ThermalRegion } from", "import type { Environment, HydronicCircuit, Material, PhaseChange, ThermalRegion } from")
rep(
    """ * `q_in` is the conduction from inside: through the layers from the skin (bodies, handled in 🫀️physiology), from the interior (conditioned elements), the heat
 * released inside (operating surfaces) or zero (passive surfaces, leaves).""",
    """ * `q_in` is the conduction from inside: through the layers from the skin (bodies, handled in 🫀️physiology), from the interior (conditioned elements), from the water of a heating
 * circuit (hydronic surfaces) or of another region (coupled surfaces, which also hand their draw back to that region), the heat released inside (operating surfaces) or zero (passive surfaces, leaves).""",
)
rep(
    """  readonly stomatalMS: number;
};""",
    """  readonly stomatalMS: number;
  readonly sourceC: number;
  readonly drawWM2: number;
};""",
)
rep(
    """/** 📥️ Heat arriving at the outer surface from inside, W/m² of outer area, as a function of the surface temperature, and its inside temperature.
 * Passive regions and leaves: none. Operating: the released heat `heatW × level / area`. Conditioned: `(T_interior − T_s) / (ΣR + R_film)`. */""",
    """/** 🚰️ The water temperature of a heating circuit at its position along the flow and operating `level`, with the room at `airC`: the supply and return temperatures of the circuit table interpolated
 * linearly in the level (below the first row linearly down to `airC` at level 0, above the last row held), then `supply − (supply − return) · flowPosition`. */
export function circuitWaterC(c: HydronicCircuit, level: number, airC: number): number {
  const rows = c.rows;
  const first = rows[0]!;
  const last = rows[rows.length - 1]!;
  const v = Math.max(0, level);
  let supply: number;
  let back: number;
  if (v < first.v) {
    const w = first.v > 0 ? v / first.v : 1;
    supply = airC + (first.supplyTempC - airC) * w;
    back = airC + (first.returnTempC - airC) * w;
  } else if (v >= last.v) {
    supply = last.supplyTempC;
    back = last.returnTempC;
  } else {
    const k = rows.findIndex((row, i) => v >= row.v && v < rows[i + 1]!.v);
    const lo = rows[k]!;
    const hi = rows[k + 1]!;
    const w = (v - lo.v) / (hi.v - lo.v);
    supply = lo.supplyTempC + (hi.supplyTempC - lo.supplyTempC) * w;
    back = lo.returnTempC + (hi.returnTempC - lo.returnTempC) * w;
  }
  return supply - (supply - back) * c.flowPosition;
}

/** 📥️ Heat arriving at the outer surface from inside, W/m² of outer area, as a function of the surface temperature, and its inside temperature.
 * Passive regions and leaves: none. Operating: the released heat `heatW × level / area` less the draw of the regions coupled to it (`drawWM2`). Conditioned: `(T_interior − T_s) / (ΣR + R_film)`.
 * Hydronic: the same with the circuit water `sourceC`. Coupled: the same with the water `sourceC` of the source region, over the wetted share of the area. */""",
)
rep(
    """    const q = (r.operating.heatW * level) / r.areaM2;
    return () => q;
  }""",
    """    const q = (r.operating.heatW * level) / r.areaM2 - f.drawWM2;
    return () => q;
  }
  if (r.model === "hydronic" && r.hydronic) {
    const resistance = f.setup.layerResistance + r.hydronic.filmM2KW;
    return (ts) => (f.sourceC - ts) / resistance;
  }
  if (r.model === "coupled" && r.coupled) {
    const conductance = r.coupled.wetFraction / (f.setup.layerResistance + r.coupled.filmM2KW);
    return (ts) => conductance * (f.sourceC - ts);
  }""",
)
open(p, "w", encoding="utf-8", newline="").write(t)
print("ok")
