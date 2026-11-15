import os, re, io

HERE = os.path.dirname(os.path.abspath(__file__))
TK = os.path.dirname(HERE)
PREFIX = "🎓️teaching/🏛️architecture/⚡️energy/"
OUT = os.path.join(TK, "CONSISTENCY-AUDIT.md")


def rd(name):
    p = os.path.join(HERE, name)
    if not os.path.exists(p):
        return []
    with io.open(p, encoding="utf-8") as fh:
        return [l.rstrip("\n") for l in fh if l.strip()]


def short(path):
    return path[len(PREFIX):] if path.startswith(PREFIX) else path


def parse(lines):
    rows = []
    for l in lines:
        if l.startswith("TOTAL"):
            continue
        m = re.match(r"^(.*?\.(?:py|json|md|ts|txt)):(\d+): ?(.*)$", l)
        if m:
            rows.append((m.group(1), int(m.group(2)), m.group(3)))
    return rows


def esc(s):
    return s.replace("|", "\\|")


def snip(s, n=150):
    s = s.strip()
    return s if len(s) <= n else s[: n - 1] + "…"


out = []
W = out.append

# ---------- Section 1 : internal gains ----------
I_TOK = re.compile(r"(\\dot\{Q\}|Q̇|Q)_(\{\\mathrm\{I((?:,\w+)?)\}\}|\\mathrm\{I((?:,\w+)?)\}|\{I((?:,\w+)?)\}|I((?:,\w+)?))")


def i_repl(m):
    head = m.group(1)
    raw = m.group(2)
    suffix = next((g for g in (m.group(3), m.group(4), m.group(5), m.group(6)) if g is not None), "")
    if raw.startswith("{") or suffix:
        return f"{head}_{{i{suffix}}}"
    return f"{head}_i"


def i_tokens(text):
    return [m.group(0) for m in I_TOK.finditer(text)]


i_rows = []
for f, n, t in parse(rd("s1-I.txt")):
    if not re.search(r"(\\dot\{Q\}|Q̇|Q)_\{?(\\mathrm\{)?I(?![A-Za-z])", t):
        continue
    toks = i_tokens(t)
    if not toks:
        continue
    pairs = sorted(set(f"`{x}` → `{I_TOK.sub(i_repl, x)}`" for x in toks))
    i_rows.append((short(f), n, "; ".join(pairs)))

# vent-a/cooling extra variant
for f, n, t in parse(rd("s1-misc.txt")):
    if "I,lat" in t and not any(r[0] == short(f) and r[1] == n for r in i_rows):
        i_rows.append((short(f), n, "`\\dot{Q}_{\\mathrm{I,lat}}` → `\\dot{Q}_{i,lat}`"))

W("## 1. Cross-video symbol inconsistencies")
W("")
W("Decisions applied: internal gains lower case `i` (`Q̇_i`, `Q̇_{i,p}`, `Q̇_{i,e}`, `Q̇_{i,l}`); `e_E` for the Anlagenaufwandszahl; `G_{t,20/15}` for degree days; cooling demand `Q_c`, cooling load `Q̇_C`; heating load `Q̇_H`; no `Φ_V`, `Q̇_L`, `Q̇_K`, `F_sh`, `F_V`, `F_f`, `R_ges`, `ϑ`, `ΔT`.")
W("")
W(f"### 1a. Uppercase internal-gains symbols ({len(i_rows)} lines, lower case `i` required)")
W("")
W("| # | file:line | token(s) and proposed replacement |")
W("|---|---|---|")
for k, (f, n, p) in enumerate(i_rows, 1):
    W(f"| {k} | `{f}:{n}` | {esc(p)} |")
W("")
W("Notes: `i` is reserved for internal gains in the energy videos. Heating scenes already use lower-case `Q̇_i` (heat-b). The `Q̇_{I,lat}` variant sits in ventilation `❓️need/🐍️.py:288`. Python identifiers such as `THETA_I` are not symbols and are excluded.")
W("")

# forbidden symbols
FORB = {
    "Φ_V": "Q̇_V", "Φ_T": "Q̇_T", "Q̇_L": "Q̇_V", "R_ges": "R_T", "F_V": "F_C",
    "ΔT_eq": "Δθ_eq", "ΔT": "Δθ", "Q̇_Licht": "Q̇_{i,l}", "Q̇_Pers": "Q̇_{i,p}",
    "Q̇_Geräte": "Q̇_{i,e}", "I_S,max": "I_{s,max}", "I_reduziert": "I_{s,red}",
    "F_f": "F_F", "F_sh": "F_C", "Q̇_K": "Q̇_C", "\\Phi_V": "\\dot{Q}_V",
    "\\Phi_T": "\\dot{Q}_T", "\\dot{Q}_L": "\\dot{Q}_V", "\\dot{Q}_K": "\\dot{Q}_C",
    "R_{ges}": "R_T", "F_{sh}": "F_C", "\\Delta T": "\\Delta\\theta", "\\vartheta": "\\theta",
    "ϑ": "θ",
}
forb_rows = []
seen = set()
for f, n, t in parse(rd("s1-forbidden.txt") + rd("s1-forb-str.txt")):
    sf = short(f)
    if (sf, n) in seen:
        continue
    if not (f.endswith(".md") or f.endswith("README.md") or f.endswith(".json") or re.search(r"\.py$", f) and re.search(r"[\"']", t)):
        continue
    if "🧪️tests" in f or "lexicon" in f or "tex/" in f or "🧮️tex" in f or "guard" in f or "fonts/🧪️tests" in f or "🔗️chain" in f:
        continue
    hits = [k for k in FORB if k in t]
    if not hits:
        continue
    seen.add((sf, n))
    pairs = "; ".join(f"`{k}` → `{FORB[k]}`" for k in hits)
    forb_rows.append((sf, n, pairs))

W(f"### 1b. Forbidden symbols in shown or documented text ({len(forb_rows)} lines)")
W("")
W("Scope: README, concepts, quiz JSON and shown strings in scenes. Test fixtures, the speech lexicon and the font test that deliberately contain `ϑ` or `\\vartheta` are excluded, as are Python identifiers such as `PHI_V_W`, which are not shown.")
W("")
W("| # | file:line | forbidden token → replacement |")
W("|---|---|---|")
for k, (f, n, p) in enumerate(forb_rows, 1):
    W(f"| {k} | `{f}:{n}` | {esc(p)} |")
W("")
W("Also: `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:5` (docstring) still says `Φ_V`; change to `Q̇_V`. The concepts file `📖️concepts/📝️.md` contains about 20 of these and needs a full pass, not only the listed lines.")
W("")

# phi index
phi_rows = [
    ("📊️demand/🎬️clip/⚖️regulations/🐍️.py:551-552", "`\\Phi_{HL}`", "Quoted DIN EN 12831-1 name for the heating load. Defined here (the norm's symbol, not a building index). Keep only as a norm quote, or drop: convention puts the load at `Q̇_H`.", "keep as norm quote with `Q̇_H` on the same line"),
    ("📊️demand/🎬️clip/⚡️demand-power/🐍️.py:589-590, 652", "`\\Phi_{\\mathrm{HL}}`", "Same norm quote, plus a `Symbol:` fact row (line 652) that pairs `Q̇_H` with `Φ_HL`.", "keep the quote; drop the `Φ_HL` fact row or label it 'in der Norm'"),
    ("🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1026-1027, 1093, 1111", "`\\Phi_i`, `\\Sigma\\,\\Phi_{i}`", "Per-element heat flow with a building index `i` (sum over building elements). Convention: `Φ` without building index.", "`\\Phi` alone; write the sum as `\\Sigma\\,\\Phi`"),
    ("🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:626-627, 1014, 1058", "`\\Phi_{\\mathrm{k}}`", "Mechanism index (conduction). Defined at line 626. Not a building index, so acceptable; keep upright.", "keep `\\Phi_{\\mathrm{k}}`; owner decision"),
    ("🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:736-737, 1015, 1059", "`\\Phi_{\\mathrm{c}}`", "Mechanism index (convection). Defined at line 736. Acceptable as mechanism index.", "keep `\\Phi_{\\mathrm{c}}`"),
    ("🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:913-914, 1016, 1060", "`\\Phi_{\\mathrm{r}}`", "Mechanism index (radiation). Defined at line 913. Acceptable as mechanism index.", "keep `\\Phi_{\\mathrm{r}}`"),
    ("🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1057-1059", "`\\Phi_{\\mathrm{c,i}}`, `\\Phi_{\\mathrm{r,i}}`, `\\Phi_{\\mathrm{c,e}}`, `\\Phi_{\\mathrm{r,e}}`", "Index `i` (inside) collides with the internal-gains index `i`; `e` is outside.", "`\\Phi_{\\mathrm{c,si}}`, `\\Phi_{\\mathrm{r,si}}`, `\\Phi_{\\mathrm{c,se}}`, `\\Phi_{\\mathrm{r,se}}` (matches `R_si`, `R_se`)"),
    ("🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:375, 1747 and 1739", "`\\Phi_{\\mathrm{...}}` (typeset helper, `Φ = U·A·Δθ`)", "Generic `Φ` with a sub-index chosen by the helper; the derived line itself is correct.", "keep; no building index"),
    ("🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1093, 1442-1484 area", "`\\Sigma\\,U_{i}A_{i}`, `\\Sigma\\,\\psi_{k}\\,l_{k}`", "Sums over building elements / thermal bridges with lower-case index: a building index on the component `U`, not on `Φ`. Convention is silent on `U_i`; keep but state the sum once.", "keep; judge"),
    ("📖️concepts/📝️.md:197, 276, 302, 308-313", "`Φ_V`, `Φ_T`", "Ventilation and transmission heat flows written as `Φ` with a building index; forbidden by convention (`Q̇_V`, `Q̇_T`).", "`Q̇_V`, `Q̇_T`"),
]
W(f"### 1c. Indexed `Φ` uses ({len(phi_rows)} groups, from {len(rd('s1-phi.txt'))-1} hits)")
W("")
W("| # | file:line | token | definition / role | proposed |")
W("|---|---|---|---|---|")
for k, (f, t, role, prop) in enumerate(phi_rows, 1):
    W(f"| {k} | `{f}` | {esc(t)} | {esc(role)} | {esc(prop)} |")
W("")

# quantities
W("### 1d. Same quantity, different values across files")
W("")
W("| quantity | value per file (file:line) | verdict and proposed single value |")
W("|---|---|---|")
rows_q = [
    ("g-value g⊥ (glazing, normal incidence)",
     "physics `🧲️fundamentals/🐍️.py:2124` `g_{\\perp} = 0.50`; heating `☀️solar-gains/🐍️.py:115` comment and `G_VALUE` = 0,60 (`:115` `g_⊥ = 0,60`); cooling `☀️solar-radiation/🐍️.py:102` uses `_heating_solar.G_VALUE` (0,60); demand `🔁️recap/🐍️.py:1079` `G_VALUE`; test `🏗️authoring/🎬️clip/🧪️tests/🔣️.cases.json:486` `0.6`",
     "CONTRADICTION. Heating `G_VALUE = SHARE_T + SHARE_IN` = 0,50 + 0,10 = 0,60 (`☀️solar-gains/🐍️.py:124`). The physics 0,50 equals only the direct-transmission share `SHARE_T`, labelled as the total g⊥. Proposed: physics line 2124 shows `g_⊥ = 0{,}60` (total), or relabels 0,50 as direct transmission τ with the 0,10 secondary share named."),
    ("F_C (sun-protection, blind)",
     "heating `☀️solar-gains/🐍️.py:116` comment `F_C = 0,15` (and `FC_WINTER = 1.0` at `:131` for the unshaded case); cooling `system-sizing/🐍️.py:103` takes `F_C` from the solar factors; test `shading: 0.15` at `solar-radiation/🐍️.py:102`",
     "Consistent value 0,15 for the blind; `FC_WINTER = 1.0` means no blind in winter: label it. Proposed: name the case in the scene text."),
    ("F_S (roof overhang), F_F (glazed fraction), F_W (incidence)",
     "heating `☀️solar-gains/🐍️.py:121-130`: `FF_GLASS = 0.70`, `FW_INCIDENCE = 0.90`, `FS_WINTER = 0.80`; cooling test `glass_share: 0.7` (`🔣️.cases.json:485`)",
     "Consistent (0,70 / 0,90 / 0,80). No change."),
    ("H_T and H_V (W/K)",
     "running example `🏗️authoring/📏️running-example/🐍️.py:52-53` derived: H_T = 2300 W / 20 K = 115 W/K, H_V = 1500 W / 20 K = 75 W/K; demand `⚡️demand-power/🐍️.py:610, 613` `H_T = 115 W/K`, `H_V = 75 W/K`",
     "Consistent. Proposed: keep, but state the reference Δθ (20 K, 0 °C) on the same slide."),
    ("design outdoor temperature and Δθ (heating)",
     "running example `🏗️authoring/📏️running-example/🔣️.json` `winter.outdoorDesignC = 0` (Δθ = 20 K); demand peak `⚡️demand-power/🐍️.py:88` `DESIGN_DT = THETA_I - THETA_E` (= 32 K, θ_e = −12 °C, `🐍️.py:89` `PEAK_KW`); physics quiz `🧲️physics/❓️quiz/🔣️.json:1557` `−12 °C design temperature` (other building)",
     "CONTRADICTION in the design basis: 0 °C (winter ventilation and transmission) vs −12 °C (design peak). Proposed: name both cases explicitly, `θ_e = 0 °C` for the running example and `θ_e = −12 °C` (DIN EN 12831-1, Hannover) for the design load, on the demand slide."),
    ("ρ (air density) and c_p",
     "running example `🏗️authoring/📏️running-example/🔣️.json` 1.2 kg/m³, 1005 J/(kg·K); heat-flow `🌡️heat-flow/🐍️.py:107` 1.2 / 1005.0; natural `🪟️natural/🐍️.py:281` 1.2 / 1005.0; cooling `💧️transmission-humidity/🐍️.py:101` 1.2 / 0.34 / 2450; README `🌬️ventilation/🎬️clip/README.md:29` ρ ≈ 1,2 at 20 °C, 1,27 at 5 °C",
     "Consistent (1,2 kg/m³ at 20 °C; 1,27 kg/m³ at 5 °C). Ok under F08. The concept file `📖️concepts/📝️.md:187` derives 0,34 from `1,2 · 1 005 / 3 600` with no units (see section 2)."),
    ("c_Luft = 0,34 Wh/(m³·K)",
     "consistent across ventilation, heating, cooling, demand, concepts (≈ 0,34 Wh/(m³·K)); test case `🏗️authoring/📝️narration/🧪️tests/🧮️tex/🔣️.cases.json:16`",
     "Consistent."),
    ("latent ventilation loss Q̇_V,lat (winter, summer)",
     "running example: 770 W (Δx = 4,3 g/kg, winter); 810 W (Δx = 4,5 g/kg, summer); vent-b reports and `🌬️ventilation/🎬️clip/README.md:50, 53` agree; concept `📖️concepts/📝️.md:189` 810 W",
     "Consistent."),
    ("ventilation sensible loss (winter, summer)",
     "1500 W (Δθ = 20 K), 450 W (Δθ = 6 K), 300 W and 90 W with η = 0,8; cooling `💧️transmission-humidity/🐍️.py:367` 450 W; heating `🧱️convection/🐍️.py:144` 1500 W",
     "Consistent."),
    ("degree days G_t20/15",
     "3 500 Kd/a: demand `🔁️recap/🧪️tests/🧮️balance/🐍️.py:73`, heating `🧮️calculation/🐍️.py:427` (θ_i = 20 °C, θ_HG = 15 °C); notation differs: `G_t20/15` in README `📊️demand/🎬️clip/README.md:28`, concepts `📖️concepts/📝️.md:400`; `G_{t,20/15}` in demand `⚖️regulations/🐍️.py:1121`; `G_{\\mathrm{t20/15}}` in heating `🧮️calculation/🐍️.py:568`",
     "Value consistent. Notation inconsistent: propose `G_{t,20/15}` everywhere (convention)."),
    ("F_Gt (heating-period factor)",
     "demand `⚖️regulations/🐍️.py:1133-1134` `F_{Gt} = 0,024 · G_t`; concepts `📖️concepts/📝️.md:401` `F_Gt = 24 h/d · G_t`",
     "Same quantity, two forms (0,024 kKh/(Kd·…) vs 24 h/d). Consistent after unit conversion. Proposed: one form, `F_{Gt} = 0{,}024\\,\\mathrm{kKh/(Kd)} \\cdot G_t`."),
    ("solar load, heating (winter) vs cooling (summer)",
     "heating winter `☀️solar-gains/🐍️.py:120` `G_WINTER = 400 W/m²`, 232 W after the blind and overhang (heat-b); cooling summer `solar-radiation/🐍️.py:102` `irradiance: 800.0`; cooling sizing 1510 W (system-sizing, report)",
     "Different cases (winter 400 W/m² vs summer 800 W/m²); no contradiction. Label the case on each slide."),
    ("U-value wall (running example)",
     "running example JSON `wall` 0,24 W/(m²·K); heat-a: `U = 0,24 W/(m²K)` shown, computed layer result 0,242",
     "Consistent (rounded). Proposed: show 0,24 and say 'gerundet'."),
]
for qn, vals, verdict in rows_q:
    W(f"| {esc(qn)} | {esc(vals)} | {esc(verdict)} |")
W("")
W("Not exhaustively checked: U-values in every quiz line and every `H_T` derivation outside the running example; the cooling sizing numbers were only checked against the report.")
W("")

# ---------- Section 2 : units ----------
W("## 2. Remaining unit violations")
W("")
W("### 2a. Bare bracketed units (`[W]`, `[kWh]`, `[\\mathrm…]`, `\\left[`)")
W("")
W("| file:line | text | proposed |")
W("|---|---|---|")
bare = [
    ("📖️concepts/📝️.md:238", "Wärmestrom durch Bauteil [W]", "Wärmestrom durch Bauteil, in W"),
    ("📖️concepts/📝️.md:267", "[W/(m²·K)]", "in W/(m²·K)"),
    ("📖️concepts/📝️.md:275", "[W/K]", "in W/K"),
    ("📖️concepts/📝️.md:276", "Transmissionswärmestrom der Hülle [W]", "Transmissionswärmestrom der Hülle, in W"),
    ("📖️concepts/📝️.md:456", "Personenlast [W]", "Personenlast, in W"),
]
for f, t, p in bare:
    W(f"| `{f}` | {esc(t)} | {esc(p)} |")
W("")
W("No `\\left[` or `\\,[\\mathrm` remains in shown scene code (grep of 364 source files). Quiz and README are clean for bracket units.")
W("")

W("### 2b. Compound units inline in display math (`\\mathrm{m^{3}/h}` etc. must become `\\frac`)")
W("")
W("Rule: `\\mathrm{A/B}` in a formula → `\\frac{\\mathrm{A}}{\\mathrm{B}}`. Running text, captions (`$…$` in narration) and quiz unit fields may keep `W/(m²·K)`.")
W("")
W("| # | file:line | inline unit in display | proposed |")
W("|---|---|---|---|")
disp = []


def mathrm_groups(t):
    res = []
    for m in re.finditer(r"\\mathrm\{", t):
        i = m.end()
        depth = 1
        j = i
        while j < len(t) and depth:
            if t[j] == "{":
                depth += 1
            elif t[j] == "}":
                depth -= 1
            j += 1
        if depth == 0:
            res.append(t[m.start():j])
    return res


def frac_unit(s):
    inner = s[len("\\mathrm{"):-1]
    if "/" not in inner:
        return None
    a, b = inner.split("/", 1)
    b = b.strip("()")
    return f"\\frac{{\\mathrm{{{a}}}}}{{\\mathrm{{{b}}}}}"


for f, n, t in parse(rd("s2-inline-slash.txt")):
    if not re.search(r"math_label|MathTex|math_text|math_readout|lambda:|tex\(|label_number|_fact|rf\"", t):
        continue
    if f.endswith(".json"):
        continue
    pairs_l = []
    for g in mathrm_groups(t.replace("{{", "{").replace("}}", "}")):
        fr = frac_unit(g)
        if fr:
            pairs_l.append(f"`{g}` → `{fr}`")
    if not pairs_l:
        continue
    disp.append((short(f), n, "; ".join(dict.fromkeys(pairs_l))))
for k, (f, n, p) in enumerate(disp, 1):
    W(f"| {k} | `{f}:{n}` | {esc(p)} |")
W("")
W(f"{len(disp)} display formulas. The owner's convention demands `\\frac` in display math; each row is one formula line and may hold more than one unit.")
W("")

W("### 2c. Numeric substitutions where a factor lacks a unit")
W("")
W("Dimensionless factors (η, F_C, 0,70, 1,04 …) are acceptable. Listed are factors that carry a physical quantity without a unit, or a result without unit.")
W("")
W("| file:line | text | status |")
W("|---|---|---|")
numsub_flag = {
    "📖️concepts/📝️.md:187": "missing units on 1,2 (kg/m³) and 1 005 (J/(kg·K)); proposed `1{,}2\\,\\mathrm{kg/m^3} \\cdot 1005\\,\\mathrm{J/(kg\\,K)} / 3600\\,\\mathrm{J/Wh} ≈ 0{,}34\\,\\mathrm{Wh/(m^3K)}`",
    "📖️concepts/📝️.md:188": "440 · 0,5 · 0,34 · 20 without units; proposed `440 m³ · 0,5 h⁻¹ · 0,34 Wh/(m³·K) · 20 K ≈ 1 500 W`",
    "📖️concepts/📝️.md:189": "1,2 · 2 450 kJ/kg · 4,5 g/kg · 0,061 m³/s without units; proposed with each unit",
    "📖️concepts/📝️.md:308": "440 · 0,5 · 0,34 · 20 without units; same fix as line 188",
    "📖️concepts/📝️.md:101": "0,5 · 2,0 m² · 500 W/m² = 500 W: dimensionless g, ok; add unit on g if shown",
    "🧲️physics/❓️quiz/🔣️.json:2270": "1,496 × 0.27 / 8,760 without TWh or h units; proposed `1{,}496\\,\\mathrm{TWh} \\cdot 0{,}27 / 8760\\,\\mathrm{h} ≈ 46\\,\\mathrm{GW}`",
    "🧲️physics/❓️quiz/🔣️.json:2271": "same as 2270 (German)",
    "🌬️ventilation/❓️quiz/🔣️.json:898": "`28 × 25 m³/h` (28 persons unit missing); proposed `28 Pers. × 25 m³/(h·Pers.)`",
    "🌬️ventilation/❓️quiz/🔣️.json:899": "same as 898 (German)",
    "🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:401": "`30 m² × 0.49 × 0.15 × 0.60 × 800 W/m²` (glazing factor 0,49 is an old value, see solar section, now 0,70); fix test fixture",
}
for f, n, t in parse(rd("s2-numsub.txt")):
    key = f"{short(f)}"
    k2 = f"{short(f)}:{n}"
    flag = None
    for kk, vv in numsub_flag.items():
        if kk == k2 or (kk.startswith(short(f)) and kk.endswith(f":{n}")):
            flag = vv
            break
    if flag is None:
        continue
    W(f"| `{k2}` | {esc(snip(t, 110))} | {esc(flag)} |")
for k2, vv in numsub_flag.items():
    if not any(f"{short(f)}:{n}" == k2 for f, n, _ in parse(rd("s2-numsub.txt"))):
        W(f"| `{k2}` | (line text not matched by scan) | {esc(vv)} |")
W("")
W("Other numeric chains (concepts 101, 114-115 scales) were judged acceptable or are pure exponent notation.")
W("")

W("### 2d. Thin space, upright and number/unit spacing")
W("")
W("A complete pass is not done. The scan of 517 candidate lines (`s2-thin.txt`) mixes running text (`20 °C` with a space, which is correct in prose) and formulas. Clear TeX cases: `$…\\mathrm{…}$` directly after a digit without `\\,` (`1500\\mathrm{W}`) must get `\\,`. Units in on-screen labels should be upright (`\\mathrm`), not italic, e.g. the axis label in `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py:239` uses `\\mathrm{W/m^{2}}` inside `tr(...)`, which is acceptable as a label.")
W("")

# ---------- Section 3 ----------
W("## 3. Remaining spelled-out symbols and bare Unicode math letters")
W("")
W("Narration (`(key, en, de)` clauses), labels, README and quiz: symbols must sit inside `$…$`, with real Unicode only as the symbol itself inside math.")
W("")
W("### 3a. Bare Unicode Greek or Φ in README, concepts and quiz text (outside `$…$`)")
W("")
W("| # | file:line | text (excerpt) | proposed |")
W("|---|---|---|---|")
uni_rows = []
for l in rd("s3-md.txt"):
    m = re.match(r"^(.*?):(\d+): \[(UNI|UNISPELL)\] (.*)$", l)
    if not m:
        continue
    f, n, tag, t = m.group(1), m.group(2), m.group(3), m.group(4)
    uni_rows.append((short(f), n, t))
for k, (f, n, t) in enumerate(uni_rows, 1):
    W(f"| {k} | `{f}:{n}` | {esc(snip(t, 120))} | wrap every symbol as `$…$`, e.g. `$\\Delta\\theta$`, `$\\dot{{Q}}_V$`, `$\\lambda$` |")
W("")
W(f"{len(uni_rows)} lines. The concepts file accounts for most of them and is written as Unicode tables, so it needs a full rewrite into `$…$` form, not line edits.")
W("")
W("### 3b. Spelled-out symbol names in narration or fixtures that are spoken or shown")
W("")
W("| file:line | text | proposed |")
W("|---|---|---|")
W("| `🎨️visuals/🧪️tests/🔣️integration.json:369` (de) | \"Q Punkt V ist gleich Rho a mal c p a mal Delta Theta mal q v R.\" | \"$\\dot{Q}_V$ ist gleich $\\rho \\cdot c_p \\cdot \\Delta\\theta \\cdot q_{v,R}$.\" (fixture mirrors shipped narration: fix) |")
W("| `🎨️visuals/🧪️tests/🔣️integration.json:380` (de) | \"Weil Q Punkt V gleich Q Punkt S t r ist, …\" | \"Weil $\\dot{Q}_V = \\dot{Q}_{S,tr}$ ist, …\" |")
W("| `🎨️visuals/🧪️tests/🔣️integration.json:382` (en) | \"Because Q-dot V equals Q-dot S,tr, …\" | \"Because $\\dot{Q}_V = \\dot{Q}_{S,tr}$, …\" |")
W("| `📊️demand/🎬️clip/🧪️tests/🔗️chain/🔣️.json:8` | forbidden list (`\"Delta Theta\"`, `\"Delta-Theta\"`) | keep: guard fixture |")
W("| `🏗️authoring/📝️narration/🧪️tests/🛡️guard/🐍️.py:98-99` | \"Die Temperaturdifferenz Delta-Theta hier\" | keep: guard must reject it |")
W("| `🏗️authoring/🔊️speech/🧪️tests/🗣️lexicon/🔣️.cases.json:56, 64` and `🏗️authoring/🔊️speech/🗣️lexicon/🔣️.json:166-171` | \"Delta-Theta\", \"Q-Punkt V\" as speech input and the lexicon's spoken names | keep: these are the speech layer's input and output mapping, not shown text |")
W("")
W("The scan `s3-spelled.txt` (159 hits) matched mostly TeX `\\Delta` and `\\Phi` commands (word-boundary false positives), not words. No spelled-out name was found in on-screen `Text(`, `body_text(` or `tr(` labels outside `$…$`.")
W("")
W("### 3c. Bare Unicode letters in code strings of scenes")
W("")
W("The scan `s3-uni.txt` (43 hits) lists Greek letters in quoted strings outside `$…$`. Entries are docstrings (not shown), speech lexicon keys, the TeX-to-speech map `🏗️authoring/📝️narration/🧮️tex/🔣️.json`, and the quiz and README lines already listed in 3a. Not classified one by one: the docstrings do not reach the screen.")
W("")

# ---------- Section 4 ----------
W("## 4. On-screen references and citation strings")
W("")
W("The reference audit ran as a sub-task and is embedded here in full: `audit2/sec4-references.md` holds the occurrence index (d1-d47 with every file:line). Summary: 47 distinct references; VERIFIED 2, EDITION-ONLY 12, UNVERIFIED 17, WRONG 16.")
W("")
sec4 = io.open(os.path.join(HERE, "sec4-references.md"), encoding="utf-8").read()
m4 = re.search(r"(## \(a\) Reference table.*?)(?=\n## \(d\))", sec4, re.S)
if m4:
    W(m4.group(1).replace("## (a) Reference table", "### 4a. Reference table (sub-audit)").replace("## (b)", "### 4b.").replace("## (c)", "### 4c."))
W("")
W("### 4d. My spot checks (independent of the sub-audit)")
W("")
W("- DIN V 4108-6:2003-06 is withdrawn. Withdrawal dated 2021-07 (baunormenlexikon); DIN Media names DIN/TS 18599-12:2021-04 as replacement; sources disagree whether it was replaced directly. Cited ~51 times in heating and demand: needs a rewrite to DIN/TS 18599-12 or a note 'zurückgezogen'. https://www.baunormenlexikon.de/norm/din-v-4108-6/bc30ac26-55c3-44a2-ac9e-782a9e61931f ; https://www.dinmedia.de/en/pre-standard/din-v-4108-6/63939447")
W("- DIN 4108-2:2026-05 is the current edition and replaces 2013-02 (DIN Media). But the GModG § 11 still cites the 2013-02 edition according to two secondary sources, so 'DIN 4108-2:2013-02' is correct only as the legal reference. https://www.dinmedia.de/en/standard/din-4108-2/401017482 ; https://www.legat-living.de/de/ueber-uns/magazin/magazin-leser/sommerlicher-waermeschutz-was-die-din-4108-2-verlangt-und-was-sie-nicht-regelt.html")
W("- GModG: promulgated 28.07.2026, in force 29.07.2026 (sub-audit, confirmed by sources listed in section (c)). Files still write 'GEG 2024' as the current law.")
W("")

# ---------- Section 5 ----------
W("## 5. Parentheses drawn as brackets")
W("")
W("### 5a. Verdict")
W("")
W("Not settled. The font setup does not remap `(` to `[`: no `replace`, `translate` or `maketrans` on parentheses exists in `energy/`, and the fallback in `🏗️authoring/🔤️fonts/🐍️.py:138-161` only substitutes characters the face lacks. My own glyph render of the body face shows that `(` and `[` are visibly different in this face: `(` is a rounded arc, `[` a square bracket. Evidence: `audit2/paren-glyph-check.png` (rendered from `🧰️framework/.../🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf`, which is `FONT_SANS` = Anta from `🎨️theme/🐍️.py:13`).")
W("")
W("The sub-audit's claim that a rendered frame shows `[2024]` for `(2024)` (`🗑️generated/physics-out/fr/Beat8_Zusammenfassung.png`, `audit2/crop-refs-willems.png`) is not confirmed at the resolution inspected: the 2560 px contact sheet is too small to separate the glyphs reliably. The source at `🧲️fundamentals/🐍️.py:3185, 3188` uses real parentheses. So the observed bracket look is probably a rendering effect of the small rounded face, not a substitution. Next step: re-render `Beat8` at full resolution and crop the citation line.")
W("")
W("Test gap confirmed: `🏗️authoring/🔤️fonts/🧪️tests/🔣️.cases.json` has no `(` or `)` in `required_symbols` or `rendered_glyphs`, so the outline test never checks them. Fix: add `()[]` to both lists.")
W("")
W("### 5b. Parenthesis strings in on-screen text (sub-audit count, per folder)")
W("")
sec5 = io.open(os.path.join(HERE, "sec5-parentheses.md"), encoding="utf-8").read()
m5 = re.search(r"(\| Folder \| Count.*?\n\| \*\*Total\*\*[^\n]*)", sec5, re.S)
if m5:
    W(m5.group(1))
W("")
W("Frames checked: `🗑️generated/physics-out/fr/Beat8_Zusammenfassung.png` (citation block), `audit2/crop-refs-willems.png`, `audit2/paren-glyph-check.png` (my render).")
W("")

# ---------- Section 6 ----------
W("## 6. German wording")
W("")
W("### 6a. `Kühlung` where the technical system is meant (should be `Kühltechnik`)")
W("")
W("| file:line | text | proposed |")
W("|---|---|---|")
kuehl_sys = [
    ("🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:213", "Basis für Heizung und Kühlung", "Basis für Heizung und Kühltechnik"),
    ("🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:216", "Basis für Heizung und Kühlung", "Basis für Heizung und Kühltechnik"),
    ("🌬️ventilation/🎬️clip/❓️need/🐍️.py:239", "…neben Heizung und Kühlung.", "…neben Heizung und Kühltechnik."),
    ("🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:779", "die Kühlung beide Sommeranteile als Lüftungslast", "die Kühltechnik beide Sommeranteile als Lüftungslast"),
    ("🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:862", "→ Kühlung: (chip)", "→ Kühltechnik: (chip)"),
    ("🌬️ventilation/🎬️clip/📦️volume/🐍️.py:914", "die Kühlung $q_{v,\\mathrm{R}}$", "die Kühltechnik $q_{v,\\mathrm{R}}$"),
    ("🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1027", "Kühlung · Sommer (card)", "Kühltechnik · Sommer"),
    ("🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1095", "→ Kühlung: (chip)", "→ Kühltechnik:"),
    ("🌬️ventilation/🎬️clip/🗺️roadmap/🐍️.py:53", "die Kühlung den Sommerwert", "die Kühltechnik den Sommerwert"),
    ("🌬️ventilation/🎬️clip/🗺️roadmap/🐍️.py:68", "→ Kühlung: (chip)", "→ Kühltechnik:"),
    ("🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:1 (docstring)", "Übergabe an Heizung und Kühlung", "Übergabe an Heizung und Kühltechnik (align with scene title)"),
    ("📊️demand/🎬️clip/🗺️roadmap/🐍️.py:63", "← Cooling: Q_c → Kühlung:", "← Kühltechnik: Q_c"),
    ("🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3136", "Heizung und Kühlung", "Heizung und Kühltechnik"),
    ("❄️cooling/❓️quiz/🔣️.json:859", "durchgehende Kühlung (server room)", "durchgehende Kühltechnik (judge)"),
    ("🌬️ventilation/❓️quiz/🔣️.json:647", "aktive Kühlung", "aktive Kühltechnik"),
    ("🌬️ventilation/❓️quiz/🔣️.json:651", "Abgestimmte Lüftung und Kühlung", "Abgestimmte Lüftung und Kühltechnik"),
    ("🏗️authoring/🎬️clip/🪟️presentation/📚️collection/🔣️.json:91", "Auslegung der Kühlung", "Auslegung der Kühltechnik (judge: video title)"),
    ("📖️concepts/📝️.md:18", "Übergabe an Heizung und Kühlung", "Übergabe an Heizung und Kühltechnik"),
    ("📖️concepts/📝️.md:25-30", "Kühlung Teil 1…6 (video title)", "keep as video name 'Kühlung', judge"),
]
for f, t, p in kuehl_sys:
    W(f"| `{f}` | {esc(t)} | {esc(p)} |")
W("")
W("Process sense, kept as 'Kühlung': `🧲️physics/…:2745` (Kühlung entfeuchtet), `❄️cooling/…/🌤️summer-strategy/🐍️.py:211, 275, 305, 502` (freie Kühlung, sorptionsgestützte Kühlung), `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:437`.")
W("")
W("### 6b. Other wording")
W("")
W("| file:line | text | status | proposed |")
W("|---|---|---|---|")
W("| `📖️concepts/📝️.md:163` | \"1 000 ppm Pettenkoferwert\" | F04 violation | \"1 000 ppm, Grenzwert nach Pettenkofer\" |")
W("| `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:349` (en) | \"each facade lights up in turn\" | animation-describing narration | \"each facade receives the sun in turn and draws its own power curve\" |")
W("| `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:203` | \"the energy … simply disappears through the opening\" | conceptual, not animation: keep, judge | \"… leaves through the opening\" |")
W("| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1222` | \"Real bridges appear where material pierces the insulation\" | conceptual, keep | — |")
W("| `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:1060` | \"a large reserve appears\" | conceptual, keep | — |")
W("| `📊️demand/🎬️clip/🏷️certificate/🐍️.py:750` | \"its key entries have to appear in commercial property ads\" | conceptual, keep | — |")
W("| all energy narration | `verblasst`, `erscheint`, `leuchtet auf`, `blendet`, `fade`(in clauses) | 0 hits in narration | — (F07 closed) |")
W("")
W("Other `fade` hits are code docstrings and transition recipe names (`\"recipe\": \"fade\"`), not narration.")
W("")

# ---------- Section 7 ----------
W("## 7. Test registration in `🏗️authoring/📜️script.ts`")
W("")
s7 = rd("s7-tests.txt")
reg = next((l for l in s7 if l.startswith("registered:")), "registered: ?")
miss = [l.split(" ", 1)[1] for l in s7 if l.startswith("MISSING ")]
W(f"Test modules found: 70 `.py` files under `🧪️*` folders in the energy tree, of which {reg.split(':')[1].strip()} are registered in the `test` list. The {len(miss)} unregistered files define no tests (no `unittest`, `TestCase` or `def test_`), so they are scene or helper modules, not test modules. **Result: no unregistered test module.**")
W("")
W("| file | defines tests? | status |")
W("|---|---|---|")
for p in miss:
    W(f"| `{p}` | no | not a test module (scene or helper); do not register |")
W("")
W("Registry entries with no matching file: none under the energy tree. The excel entry `📊️excel/…/⚡️energy/🧪️tests/📐️result` is outside the energy tree and not compared.")
W("")

# ---------- header ----------
header = []
header.append("# Consistency audit: energy videos after reviewer feedback")
header.append("")
header.append("Read-only audit of `🎓️teaching/🏛️architecture/⚡️energy/` on 2026-11-13. No source file, git state or working tree was changed. Scratch and sub-audits live in `audit2/`.")
header.append("")
header.append("## Summary")
header.append("")
header.append(f"- 1. Symbols: {len(i_rows)} uppercase internal-gains lines (`_I`, `_{{\\mathrm{{I}}}}`, `Q̇_{{I,lat}}`); {len(forb_rows)} forbidden-symbol lines; 10 indexed `Φ` groups; 3 value contradictions or conflicts (g-value 0,50 vs 0,60; design basis 0 °C / Δθ 20 K vs −12 °C / Δθ 32 K) and 2 notation variants (`G_t20/15` vs `G_{{t,20/15}}`; `F_Gt` in two forms).")
header.append(f"- 2. Units: 5 bare brackets (all in concepts); {len(disp)} inline compound units in display math; numeric factors without units in concepts, two physics quiz lines and one ventilation quiz line; thin-space pass partial.")
header.append(f"- 3. Unicode and spelled symbols: {len(uni_rows)} bare Unicode lines in README, concepts and quiz text; 3 spelled-out lines in a fixture; the rest are intended guard or lexicon tests.")
header.append("- 4. References: 47 distinct; 2 verified, 12 edition-only, 17 unverified, 16 wrong. DIN V 4108-6 (withdrawn, ~51 uses), DIN V 18599-2 (withdrawn), DIN EN ISO 13789 edition, DIN EN 410, DIN EN 14825, DIN EN ISO 13786, VDI 6022, VDI 3807 editions wrong; GModG renaming confirmed.")
header.append("- 5. Parentheses: no code or font remap; Anta draws `(` as a rounded arc and `[` as a square bracket. The bracket look in the frame is not confirmed at the resolution checked. Test gap: no parentheses in font tests.")
header.append("- 6. German: ~18 `Kühlung` system uses to change to `Kühltechnik`; `Pettenkoferwert` (1); one animation-describing clause (`lights up`); no `verblasst`/`erscheint` in narration.")
header.append("- 7. Tests: 70 test-folder modules, 65 registered, 5 not tests; no unregistered test module.")
header.append("")
header.append("Scripts used: `audit2/g.py` with regex files `audit2/p-*.txt`; `audit2/s3.py`, `audit2/s3md.py`, `audit2/s7.py`, `audit2/paren_glyph.py`, `audit2/build_report.py`.")
header.append("")

with io.open(OUT, "w", encoding="utf-8") as fh:
    fh.write("\n".join(header + out) + "\n")
print("WROTE", OUT, len(header + out))
