"""🐍️ DIN EN 16798's contribution to the norm reference implementation — the four things that are
genuinely per-standard, and nothing else.

The second producer this case's differential comparison needs is
`semio_norm_vocabulary`, the ONE independent Python implementation of the norm mutation vocabulary,
imported here rather than copied. Its module docstring carries the survey that established no
third-party library reads or writes `s.norm.*`, the two committed documents it was written from, and
the honest boundary on the `.dsl.semio` carrier. This file adds no verb, no addressing rule and no
carrier rule: everything below is DATA read off this subset's own committed catalog, its own
committed specification vectors and its own committed example document.

Stating it this way is the point. The fifteen norm adapters used to hold fifteen byte-identical
copies of that engine, which made the reference surface read as fifteen independent implementations
when it was one. One import says what fifteen copies concealed — a shared bug here agrees with itself
in all fifteen cases, and that is now visible instead of pretended.
"""

from __future__ import annotations

# region 🔖️Imports
from semio_norm_vocabulary import Subset, build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind this subset's committed catalog declares, in catalog order.
KINDS = [
    "change-annex",
    "change-occupancy",
    "change-comfort-category",
    "change-t-op-c",
    "change-rh-percent",
    "change-air-speed-ms",
    "change-theta-rm-c",
    "change-co2-ppm",
    "change-df-percent",
    "change-l-aeq-db",
    "change-persons",
    "change-ida-class",
    "change-ventilation-m3-h",
    "change-floor-area-m2",
    "change-bedrooms",
    "change-dwelling-ventilation-m3-h",
    "change-occupants",
    "change-residential-ventilation-m3-h",
    "change-sfp-wm3-s",
    "change-sfp-required-class",
    "change-heat-recovery-eta",
    "change-heat-recovery-eta-min",
    "change-system-type",
    "change-years-since-inspection",
    "change-humidification-required-kg-h",
    "change-humidification-provided-kg-h",
    "change-fan-qvm3-s",
    "change-fan-t-run-h",
    "change-fan-energy-reference-kwh",
    "change-night-setback-k",
    "change-hr-m-dot-kg-s",
    "change-hr-cp-j-kgk",
    "change-hr-delta-tc",
    "change-hr-th",
    "change-hr-savings-reference-kwh",
    "change-n50-h-inv",
    "change-volume-m3",
    "change-infiltration-allowance-m3-h",
    "change-cellar-area-m2",
    "change-cellar-ventilation-m3-h",
    "change-h-tr-wk",
    "change-h-ve-wk",
    "change-theta-ec",
    "change-theta-set-c",
    "change-cooling-delta-th",
    "change-cooling-gains-kwh",
    "change-cooling-utilization-factor",
    "change-cooling-reference-kwh",
    "change-chiller-type",
    "change-eer-actual",
    "change-qc-kwh",
    "change-generation-reference-kwh",
    "change-data-center-supply-c",
    "change-h-st-wk",
    "change-theta-st-c",
    "change-theta-amb-c",
    "change-storage-th",
    "change-storage-allowance-kwh",
    "change-dhw-delivery-c",
    "change-duct-class",
    "change-duct-test-pressure-pa",
    "change-duct-leakage-m3-sm2",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🏷️change-annex", "switches-the-check-to-the-en-annex"),
    "change-occupancy": ("🍂change-occupancy", "reclassifies-the-space-as-office"),
    "change-comfort-category": ("🪛change-comfort-category", "tightens-the-comfort-category-to-i"),
    "change-t-op-c": ("🌊change-t-op-c", "raises-the-operative-temperature-to-24-point-5-c"),
    "change-rh-percent": ("🌹change-rh-percent", "drops-indoor-humidity-to-42-point-5-percent"),
    "change-air-speed-ms": ("🔀change-air-speed-ms", "doubles-the-draught-air-speed-to-0-point-25-ms"),
    "change-theta-rm-c": ("🌍️change-theta-rm-c", "raises-the-running-mean-outdoor-temperature-to-18-point-5-c"),
    "change-co2-ppm": ("🛠️change-co2-ppm", "raises-the-measured-co2-to-950-ppm"),
    "change-df-percent": ("🧵change-df-percent", "raises-the-daylight-factor-to-3-point-75-percent"),
    "change-l-aeq-db": ("🌳change-l-aeq-db", "raises-the-equivalent-sound-level-to-30-db"),
    "change-persons": ("🌱change-persons", "raises-the-design-occupancy-to-16-people"),
    "change-ida-class": ("🌵change-ida-class", "relaxes-the-indoor-air-class-to-ida-3"),
    "change-ventilation-m3-h": ("🌐change-ventilation-m3-h", "raises-the-supply-airflow-to-360-m3-per-hour"),
    "change-floor-area-m2": ("🧼change-floor-area-m2", "grows-the-conditioned-floor-area-to-120-m2"),
    "change-bedrooms": ("🔢change-bedrooms", "adds-a-fourth-bedroom"),
    "change-dwelling-ventilation-m3-h": ("🧲change-dwelling-ventilation-m3-h", "raises-the-dwelling-airflow-to-96-m3-per-hour"),
    "change-occupants": ("🍃change-occupants", "raises-the-household-to-five-occupants"),
    "change-residential-ventilation-m3-h": ("🌸change-residential-ventilation-m3-h", "raises-the-residential-airflow-to-110-m3-per-hour"),
    "change-sfp-wm3-s": ("🌻change-sfp-wm3-s", "improves-the-specific-fan-power-to-1250-w-per-m3-s"),
    "change-sfp-required-class": ("🌺change-sfp-required-class", "tightens-the-required-sfp-class-to-3"),
    "change-heat-recovery-eta": ("🪑change-heat-recovery-eta", "raises-the-achieved-heat-recovery-to-0-point-875"),
    "change-heat-recovery-eta-min": ("🪞change-heat-recovery-eta-min", "raises-the-required-heat-recovery-minimum-to-0-point-625"),
    "change-system-type": ("🌰change-system-type", "switches-to-a-decentral-mechanical-system"),
    "change-years-since-inspection": ("🏔️change-years-since-inspection", "ages-the-last-inspection-to-six-years"),
    "change-humidification-required-kg-h": ("🌾change-humidification-required-kg-h", "raises-the-required-humidification-to-3-point-5-kg-per-hour"),
    "change-humidification-provided-kg-h": ("🍀change-humidification-provided-kg-h", "drops-the-provided-humidification-to-1-point-25-kg-per-hour"),
    "change-fan-qvm3-s": ("🪥change-fan-qvm3-s", "raises-the-fan-volume-flow-to-1-point-5-m3-per-second"),
    "change-fan-t-run-h": ("🧴change-fan-t-run-h", "extends-the-daily-fan-runtime-to-12-hours"),
    "change-fan-energy-reference-kwh": ("🪒change-fan-energy-reference-kwh", "raises-the-fan-energy-reference-to-18-kwh"),
    "change-night-setback-k": ("🍁change-night-setback-k", "deepens-the-night-setback-to-5-kelvin"),
    "change-hr-m-dot-kg-s": ("🚿change-hr-m-dot-kg-s", "raises-the-heat-recovery-mass-flow-to-0-point-75-kg-per-second"),
    "change-hr-cp-j-kgk": ("🛋️change-hr-cp-j-kgk", "corrects-the-air-specific-heat-to-1010-j-per-kgk"),
    "change-hr-delta-tc": ("🛏️change-hr-delta-tc", "drops-the-heat-recovery-temperature-lift-to-12-point-5-c"),
    "change-hr-th": ("🌿change-hr-th", "extends-the-heat-recovery-operating-hours-to-14"),
    "change-hr-savings-reference-kwh": ("🛁change-hr-savings-reference-kwh", "raises-the-heat-recovery-savings-reference-to-65-kwh"),
    "change-n50-h-inv": ("🌲change-n50-h-inv", "loosens-the-blower-door-result-to-2-point-5-per-hour"),
    "change-volume-m3": ("🗻change-volume-m3", "grows-the-air-volume-to-640-m3"),
    "change-infiltration-allowance-m3-h": ("🌴change-infiltration-allowance-m3-h", "raises-the-infiltration-allowance-to-52-point-5-m3-per-hour"),
    "change-cellar-area-m2": ("🛡️change-cellar-area-m2", "grows-the-cellar-floor-area-to-62-point-5-m2"),
    "change-cellar-ventilation-m3-h": ("🧯change-cellar-ventilation-m3-h", "raises-the-cellar-airflow-to-22-point-5-m3-per-hour"),
    "change-h-tr-wk": ("🧹change-h-tr-wk", "improves-the-transmission-heat-transfer-to-175-w-per-k"),
    "change-h-ve-wk": ("🧺change-h-ve-wk", "raises-the-ventilation-heat-transfer-to-125-w-per-k"),
    "change-theta-ec": ("🪨change-theta-ec", "raises-the-external-design-temperature-to-34-point-5-c"),
    "change-theta-set-c": ("🌎️change-theta-set-c", "lowers-the-cooling-set-point-to-25-c"),
    "change-cooling-delta-th": ("🪚change-cooling-delta-th", "extends-the-cooling-period-to-12-point-5-hours"),
    "change-cooling-gains-kwh": ("🪜change-cooling-gains-kwh", "raises-the-internal-cooling-gains-to-7-point-5-kwh"),
    "change-cooling-utilization-factor": ("🪣change-cooling-utilization-factor", "raises-the-cooling-utilization-factor-to-0-point-875"),
    "change-cooling-reference-kwh": ("🪝change-cooling-reference-kwh", "raises-the-cooling-reference-to-25-kwh"),
    "change-chiller-type": ("🚨change-chiller-type", "switches-to-a-water-cooled-chiller"),
    "change-eer-actual": ("🪤change-eer-actual", "raises-the-achieved-eer-to-3-point-5"),
    "change-qc-kwh": ("🌷change-qc-kwh", "raises-the-annual-cooling-demand-to-1250-kwh"),
    "change-generation-reference-kwh": ("🧽change-generation-reference-kwh", "raises-the-generation-reference-to-450-kwh"),
    "change-data-center-supply-c": ("🧰change-data-center-supply-c", "raises-the-data-centre-supply-air-to-27-c"),
    "change-h-st-wk": ("🪠change-h-st-wk", "raises-the-storage-loss-coefficient-to-6-point-5-w-per-k"),
    "change-theta-st-c": ("🌏️change-theta-st-c", "lowers-the-storage-temperature-to-55-c"),
    "change-theta-amb-c": ("🐚change-theta-amb-c", "lowers-the-storage-room-ambient-to-18-c"),
    "change-storage-th": ("🍄change-storage-th", "shortens-the-storage-standby-period-to-18-hours"),
    "change-storage-allowance-kwh": ("🌼change-storage-allowance-kwh", "tightens-the-storage-loss-allowance-to-4-point-5-kwh"),
    "change-dhw-delivery-c": ("🧶change-dhw-delivery-c", "raises-the-dhw-delivery-temperature-to-60-c"),
    "change-duct-class": ("🪡change-duct-class", "upgrades-the-duct-tightness-class-to-d"),
    "change-duct-test-pressure-pa": ("🧷change-duct-test-pressure-pa", "raises-the-duct-test-pressure-to-500-pa"),
    "change-duct-leakage-m3-sm2": ("🪢change-duct-leakage-m3-sm2", "halves-the-measured-duct-leakage-to-0-point-0625"),
}

#: 🗣️ The real committed DIN EN 16798 document, read where the domain already keeps it.
DSL_ASSET = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.din16798.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("DIN EN 16798", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
