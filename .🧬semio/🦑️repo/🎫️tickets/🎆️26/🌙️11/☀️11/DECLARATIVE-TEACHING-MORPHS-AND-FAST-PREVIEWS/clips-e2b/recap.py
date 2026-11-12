import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from doc import DEMAND, entry as e, write
from patch import apply

R = "demand.recap."
BA, LO, GA, SE = R + "balance.", R + "loss.", R + "gain.", R + "seasons."
SLUGS = ("wall", "roof", "windows", "floor")
rows = [
    e(BA + "sink-symbol-into-senke", "abstraction", BA + "sink-symbol", BA + "senke-term", 1.6),
    e(BA + "source-symbol-into-quelle", "abstraction", BA + "source-symbol", BA + "quelle-term", 1.6),
    e(BA + "heat-symbol-into-qh", "abstraction", BA + "heat-symbol", BA + "qh-term", 1.6),
    e(LO + "ring-u-to-a", "continuation", LO + "ring-u", LO + "ring-a", 1.6),
    e(LO + "ring-a-to-dt", "continuation", LO + "ring-a", LO + "ring-dt", 0.8),
    e(LO + "ring-dt-to-t", "continuation", LO + "ring-dt", LO + "ring-t", 0.6),
]
for k in range(1, 4):
    rows.append(e(f"{LO}surface-head-{SLUGS[k - 1]}-to-{SLUGS[k]}", "continuation", f"{LO}surface-head-{SLUGS[k - 1]}", f"{LO}surface-head-{SLUGS[k]}", 1.2))
for slug in SLUGS:
    rows.append(e(f"{LO}surface-{slug}-into-stack", "abstraction", f"{LO}surface-{slug}-overlay", f"{LO}stack-bar-{slug}", 1.2, copy=True))
rows += [
    e(LO + "formula-box-with-bridges", "continuation", LO + "formula-box-transmission", LO + "formula-box-bridges", 1.6),
    e(LO + "formula-with-bridges", "transformation", LO + "formula-transmission", LO + "formula-bridges", 1.6, relation="many-to-many"),
    e(LO + "bridge-dots-into-stack-bar", "abstraction", LO + "bridge-dots", LO + "stack-bar-bridges", 1.4, relation="many-to-many"),
    e(LO + "formula-box-to-ventilation", "continuation", LO + "formula-box-bridges", LO + "formula-box-ventilation", 1.6),
    e(LO + "ring-n-to-v", "continuation", LO + "ring-n", LO + "ring-v", 1.6),
    e(LO + "ring-v-to-rc", "continuation", LO + "ring-v", LO + "ring-rc", 1.6),
    e(LO + "ventilation-volume-into-stack", "abstraction", LO + "ventilation-volume", LO + "stack-bar-ventilation", 1.4, copy=True),
    e(LO + "label-sink-into-sum", "abstraction", LO + "label-sink", LO + "sum-senke-term", 1.4, copy=True),
    e(LO + "label-transmission-into-sum", "abstraction", LO + "label-transmission", LO + "sum-qt-term", 1.4, copy=True),
    e(LO + "label-ventilation-into-sum", "abstraction", LO + "label-ventilation", LO + "sum-qv-term", 1.4, copy=True),
    e(GA + "ring-a-to-i", "continuation", GA + "ring-a", GA + "ring-i", 1.6),
    e(GA + "ring-i-to-g", "continuation", GA + "ring-i", GA + "ring-g", 1.6),
    e(GA + "ring-g-to-f", "continuation", GA + "ring-g", GA + "ring-f", 1.6),
    e(GA + "patches-into-solar-block", "abstraction", GA + "window-patches", GA + "solar-block", 1.6, relation="many-to-many", copy=True),
    e(GA + "formula-box-to-internal", "continuation", GA + "formula-box-solar", GA + "formula-box-internal", 1.6),
    e(GA + "ring-q-to-an", "continuation", GA + "ring-q", GA + "ring-an", 1.6),
    e(GA + "ring-an-to-t", "continuation", GA + "ring-an", GA + "ring-t", 1.6),
    e(GA + "heat-pack-into-internal-block", "abstraction", GA + "heat-pack", GA + "internal-block", 1.6, copy=True),
    e(GA + "blocks-into-usable-and-waste", "transformation", GA + "gain-blocks", GA + "usable-and-waste", 1.4, relation="many-to-many"),
    e(GA + "radiator-into-demand", "abstraction", GA + "radiator", GA + "demand-bar", 1.6, relation="many-to-many", copy=True),
    e(GA + "formula-box-to-balance", "continuation", GA + "formula-box-utilisation", GA + "formula-box-balance", 1.4),
    e(GA + "formula-to-balance", "transformation", GA + "formula-utilisation", GA + "formula-balance", 1.4, relation="many-to-many"),
    e(GA + "demand-label-into-qh", "abstraction", GA + "demand-label", GA + "qh-term", 1.4, copy=True),
    e(SE + "season-chip-to-summer", "continuation", SE + "season-chip-winter", SE + "season-chip-summer", 2.0),
    e(SE + "shell-loss-to-summer", "transformation", SE + "shell-loss-winter", SE + "shell-loss-summer", 2.6),
    e(SE + "shell-gain-to-summer", "transformation", SE + "shell-gain-winter", SE + "shell-gain-summer", 2.6),
]
write("🔁️recap", rows)

CLIP = DEMAND / "🔁️recap/🐍️.py"


def t(i, a, b):
    return f'stage.transition("{i}", source={a}, target={b})'


apply(CLIP, [
    ('SURFACES = [\n', 'SURFACE_SLUGS = ("wall", "roof", "windows", "floor")\nSURFACES = [\n'),
    ('def _merge(sources, target):', 'def _merge(stage, transition, sources, target):'),
    ('    anims = [ReplacementTransform(sources[0], target)]', '    anims = [stage.transition(transition, source=sources[0], target=target)]'),
    ('_merge([tag_t[1], tag_v[1]], items["senke"])', f'_merge(stage, "{BA}sink-symbol-into-senke", [tag_t[1], tag_v[1]], items["senke"])'),
    ('_merge([tag_s[1], tag_i[1]], items["quelle"])', f'_merge(stage, "{BA}source-symbol-into-quelle", [tag_s[1], tag_i[1]], items["quelle"])'),
    ('ReplacementTransform(tag_h[1], items["qh"])', t(BA + "heat-symbol-into-qh", "tag_h[1]", 'items["qh"]')),
    ('ReplacementTransform(ring_u, ring_a)', t(LO + "ring-u-to-a", "ring_u", "ring_a")),
    ('ReplacementTransform(ring_a, ring_dt)', t(LO + "ring-a-to-dt", "ring_a", "ring_dt")),
    ('ReplacementTransform(ring_dt, ring_t)', t(LO + "ring-dt-to-t", "ring_dt", "ring_t")),
    ('[ReplacementTransform(heads[k - 1], heads[k])]', '[stage.transition(f"' + LO + 'surface-head-{SURFACE_SLUGS[k - 1]}-to-{SURFACE_SLUGS[k]}", source=heads[k - 1], target=heads[k])]'),
    ('TransformFromCopy(hl[name], stack["bars"][k])', 'stage.transition(f"' + LO + 'surface-{SURFACE_SLUGS[k]}-into-stack", source=hl[name], target=stack["bars"][k])'),
    ('ReplacementTransform(box_t, box_t2), ReplacementTransform(row_t, row_t2)', t(LO + "formula-box-with-bridges", "box_t", "box_t2") + ', ' + t(LO + "formula-with-bridges", "row_t", "row_t2")),
    ('ReplacementTransform(dots, stack["bars"][4])', t(LO + "bridge-dots-into-stack-bar", "dots", 'stack["bars"][4]')),
    ('ReplacementTransform(box_t2, box_v)', t(LO + "formula-box-to-ventilation", "box_t2", "box_v")),
    ('ReplacementTransform(ring_n, ring_v)', t(LO + "ring-n-to-v", "ring_n", "ring_v")),
    ('ReplacementTransform(ring_v, ring_rc)', t(LO + "ring-v-to-rc", "ring_v", "ring_rc")),
    ('TransformFromCopy(vol, stack["bars"][5])', t(LO + "ventilation-volume-into-stack", "vol", 'stack["bars"][5]')),
    ('TransformFromCopy(lab_all, it_s["senke"]), TransformFromCopy(lab_t, it_s["qt"]),\n                  TransformFromCopy(lab_v, it_s["qv"])',
     t(LO + "label-sink-into-sum", "lab_all", 'it_s["senke"]') + ', ' + t(LO + "label-transmission-into-sum", "lab_t", 'it_s["qt"]') + ',\n                  ' + t(LO + "label-ventilation-into-sum", "lab_v", 'it_s["qv"]')),
    ('ReplacementTransform(ring_a, ring_i)', t(GA + "ring-a-to-i", "ring_a", "ring_i")),
    ('ReplacementTransform(ring_i, ring_g)', t(GA + "ring-i-to-g", "ring_i", "ring_g")),
    ('ReplacementTransform(ring_g, ring_f)', t(GA + "ring-g-to-f", "ring_g", "ring_f")),
    ('TransformFromCopy(patches, solar_block)', t(GA + "patches-into-solar-block", "patches", "solar_block")),
    ('ReplacementTransform(box_s, box_i)', t(GA + "formula-box-to-internal", "box_s", "box_i")),
    ('ReplacementTransform(ring_q, ring_an)', t(GA + "ring-q-to-an", "ring_q", "ring_an")),
    ('ReplacementTransform(ring_an, ring_t)', t(GA + "ring-an-to-t", "ring_an", "ring_t")),
    ('TransformFromCopy(heat_pack, intern_block)', t(GA + "heat-pack-into-internal-block", "heat_pack", "intern_block")),
    ('ReplacementTransform(VGroup(intern_block, solar_block), VGroup(usable, waste))', t(GA + "blocks-into-usable-and-waste", "VGroup(intern_block, solar_block)", "VGroup(usable, waste)")),
    ('TransformFromCopy(rad, demand)', t(GA + "radiator-into-demand", "rad", "demand")),
    ('ReplacementTransform(box_e, box_b), ReplacementTransform(row_e, row_b),\n                  TransformFromCopy(lab_dem, it_b["qh"])',
     t(GA + "formula-box-to-balance", "box_e", "box_b") + ', ' + t(GA + "formula-to-balance", "row_e", "row_b") + ',\n                  ' + t(GA + "demand-label-into-qh", "lab_dem", 'it_b["qh"]')),
    ('ReplacementTransform(season_chip, season_chip_j)', t(SE + "season-chip-to-summer", "season_chip", "season_chip_j")),
    ('ReplacementTransform(cap_loss, cap_loss_k), ReplacementTransform(cap_gain, cap_gain_k)', t(SE + "shell-loss-to-summer", "cap_loss", "cap_loss_k") + ', ' + t(SE + "shell-gain-to-summer", "cap_gain", "cap_gain_k")),
], scenes=("Beat1_Bilanzgrenze", "Beat2_Verlustseite", "Beat3_Gewinnseite", "Beat4_WinterUndSommer"))
