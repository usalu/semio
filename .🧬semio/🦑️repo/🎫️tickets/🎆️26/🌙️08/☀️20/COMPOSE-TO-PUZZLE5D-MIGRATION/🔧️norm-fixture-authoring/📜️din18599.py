"""📙️ DIN V 18599 — 13 hand-authored mutation fixture cases (12 applied + 1 rejected)."""
import copy
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from importlib import import_module

common = import_module("📜️common")
REPO = common.REPO

ROOT = os.path.join(REPO, "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")

# 🧱️ A 100 m2 gas-heated dwelling. `climate` is the composed `s.stdio.semio` table CHILD slot — a
# two-string handle (`childId` + flattened `target` ArtifactRef), never the twelve-month data
# itself; the data lives behind the handle in the session-side working-scene cache.
CLIMATE_CHILD = {
    "childId": "din18599-climate-fixture-zone2",
    "target": {"artifactId": "din18599-climate", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "table"}},
}

BEFORE = {
    "useClass": "Residential",
    "heatedAreaM2": 100.0,
    "occupants": 4,
    "hT": 92.5,
    "hV": 40.75,
    "climate": CLIMATE_CHILD,
    "internalGainsWM2": 3.5,
    "solarGainsKwh": 84.0,
    "systemLossesKwh": 800.0,
    "renewableKwh": 1500.0,
    "annualLimitKwh": 7500.0,
    "energyCarrier": "natural_gas",
    "referenceQPKwh": 10000.0,
}

DIFF_NULL = {
    "useClass": None,
    "heatedAreaM2": None,
    "occupants": None,
    "hT": None,
    "hV": None,
    "climate": None,
    "internalGainsWM2": None,
    "solarGainsKwh": None,
    "systemLossesKwh": None,
    "renewableKwh": None,
    "annualLimitKwh": None,
    "energyCarrier": None,
    "referenceQPKwh": None,
    "selectedCheckIndex": None,
}

APPLIED = {"status": "applied"}

CASES = []


def case(leaf, kind, module, variant, name, summary, field, payload_key, value, extra_applied, extra_diff):
    assert field in BEFORE and BEFORE[field] != value, field
    after = copy.deepcopy(BEFORE)
    after[field] = value
    diff = copy.deepcopy(DIFF_NULL)
    diff[field] = value
    CASES.append(
        dict(
            leaf=leaf,
            kind=kind,
            module=module,
            case=name,
            summary=summary,
            after=after,
            mutation={variant: {payload_key: value}},
            diff=diff,
            extra_applied=extra_applied,
            extra_diff=extra_diff,
            rejected=False,
        )
    )


# ── 1. change-use-class ───────────────────────────────────────────────────────
case(
    "🦏change-use-class",
    "change-use-class",
    "change_use_class",
    "ChangeUseClass",
    "reclassifies-the-building-as-an-office",
    "The building's use class moves `Residential` → `Office`, which is what selects DIN V 18599's "
    "energy-reference-area factors and usage profile. `use_class` is the only non-numeric, "
    "non-string field in the whole snapshot — an enum, so the diff must carry the bare variant name.",
    "useClass",
    "new_use_class",
    "Office",
    '''    assert_eq!(snapshot.use_class, crate::artifacts::din18599::UseClass::Office, "change-use-class/reclassifies-the-building-as-an-office: the use class must be Office");
    assert_eq!(snapshot.occupants, before().occupants, "change-use-class/reclassifies-the-building-as-an-office: reclassifying must not silently re-derive the occupancy");''',
    '''    assert_eq!(raised_diff.use_class, Some(crate::artifacts::din18599::UseClass::Office), "change-use-class/reclassifies-the-building-as-an-office: the diff must publish useClass = Office");
    assert!(raised_diff.heated_area_m2.is_none(), "change-use-class/reclassifies-the-building-as-an-office: the heated area is a separate scalar and must stay null");''',
)

# ── 2. change-heated-area-m2 ──────────────────────────────────────────────────
case(
    "🦛change-heated-area-m2",
    "change-heated-area-m2",
    "change_heated_area_m2",
    "ChangeHeatedAreaM2",
    "extends-the-heated-area-to-160-m2",
    "The energy reference area grows 100 → 160 m2. The builder's only guard is a finiteness check; "
    "160.0 passes it, so a one-scalar diff is published and every derived quantity (`h_t`, `h_v`, "
    "the gains, the limits) is deliberately left for its own mutation to move.",
    "heatedAreaM2",
    "new_heated_area_m2",
    160.0,
    '''    assert_eq!(snapshot.heated_area_m2, 160.0, "change-heated-area-m2/extends-the-heated-area-to-160-m2: the heated area must be 160 m2");
    assert_eq!(snapshot.h_t, before().h_t, "change-heated-area-m2/extends-the-heated-area-to-160-m2: the transmission loss coefficient is NOT re-derived from the area");
    assert_eq!(snapshot.annual_limit_kwh, before().annual_limit_kwh, "change-heated-area-m2/extends-the-heated-area-to-160-m2: the annual limit is NOT re-derived from the area");''',
    '''    assert_eq!(raised_diff.heated_area_m2, Some(160.0), "change-heated-area-m2/extends-the-heated-area-to-160-m2: the diff must publish heatedAreaM2 = 160");
    assert!(raised_diff.reference_q_p_kwh.is_none(), "change-heated-area-m2/extends-the-heated-area-to-160-m2: the reference-building energy must stay null — no cascade");''',
)

# ── 3. change-occupants ───────────────────────────────────────────────────────
case(
    "🐪change-occupants",
    "change-occupants",
    "change_occupants",
    "ChangeOccupants",
    "raises-the-occupancy-to-six-people",
    "The occupant count goes 4 → 6. `occupants` is the snapshot's only `u32`, so both the mutation "
    "payload and the diff must carry a JSON integer here, never a float — and this builder has NO "
    "finiteness guard, only the `mutation.no-op` equality check, because an integer cannot be NaN.",
    "occupants",
    "new_occupants",
    6,
    '''    assert_eq!(snapshot.occupants, 6, "change-occupants/raises-the-occupancy-to-six-people: the occupant count must be 6");
    assert_eq!(snapshot.internal_gains_w_m2, before().internal_gains_w_m2, "change-occupants/raises-the-occupancy-to-six-people: the internal gains figure is an independent input, not a per-occupant derivation");''',
    '''    assert_eq!(raised_diff.occupants, Some(6u32), "change-occupants/raises-the-occupancy-to-six-people: the diff must publish occupants as the u32 6");
    assert!(raised_diff.use_class.is_none(), "change-occupants/raises-the-occupancy-to-six-people: the use class must stay null");''',
)

# ── 4. change-ht ──────────────────────────────────────────────────────────────
case(
    "🐫change-ht",
    "change-h-t",
    "change_h_t",
    "ChangeHT",
    "raises-the-transmission-loss-coefficient-to-118-w-per-k",
    "The specific transmission heat-loss coefficient H_T goes 92.5 → 118.0 W/K (a worse envelope). "
    "The companion ventilation coefficient H_V is a SEPARATE mutation and must not move with it.",
    "hT",
    "new_h_t",
    118.0,
    '''    assert_eq!(snapshot.h_t, 118.0, "change-h-t/raises-the-transmission-loss-coefficient-to-118-w-per-k: H_T must be 118 W/K");
    assert_eq!(snapshot.h_v, before().h_v, "change-h-t/raises-the-transmission-loss-coefficient-to-118-w-per-k: H_V is its own mutation and must be untouched");''',
    '''    assert_eq!(raised_diff.h_t, Some(118.0), "change-h-t/raises-the-transmission-loss-coefficient-to-118-w-per-k: the diff must publish hT = 118");
    assert!(raised_diff.h_v.is_none(), "change-h-t/raises-the-transmission-loss-coefficient-to-118-w-per-k: hV must stay null in this diff");''',
)

# ── 5. change-hv ──────────────────────────────────────────────────────────────
case(
    "🦒change-hv",
    "change-h-v",
    "change_h_v",
    "ChangeHV",
    "raises-the-ventilation-loss-coefficient-to-52-25-w-per-k",
    "The specific ventilation heat-loss coefficient H_V goes 40.75 → 52.25 W/K (a higher air-change "
    "rate). Its envelope counterpart H_T stays exactly where it was.",
    "hV",
    "new_h_v",
    52.25,
    '''    assert_eq!(snapshot.h_v, 52.25, "change-h-v/raises-the-ventilation-loss-coefficient-to-52-25-w-per-k: H_V must be 52.25 W/K");
    assert_eq!(snapshot.h_t, before().h_t, "change-h-v/raises-the-ventilation-loss-coefficient-to-52-25-w-per-k: H_T is its own mutation and must be untouched");''',
    '''    assert_eq!(raised_diff.h_v, Some(52.25), "change-h-v/raises-the-ventilation-loss-coefficient-to-52-25-w-per-k: the diff must publish hV = 52.25");
    assert!(raised_diff.h_t.is_none(), "change-h-v/raises-the-ventilation-loss-coefficient-to-52-25-w-per-k: hT must stay null in this diff");''',
)

# ── 6. change-internal-gains-wm2 ──────────────────────────────────────────────
case(
    "🦘change-internal-gains-wm2",
    "change-internal-gains-w-m2",
    "change_internal_gains_w_m2",
    "ChangeInternalGainsWM2",
    "raises-the-internal-gains-to-5-w-per-m2",
    "The area-specific internal heat gains go 3.5 → 5.0 W/m2. This is a per-area intensity, so it "
    "is deliberately independent of both `heated_area_m2` and `occupants` — neither is republished.",
    "internalGainsWM2",
    "new_internal_gains_w_m2",
    5.0,
    '''    assert_eq!(snapshot.internal_gains_w_m2, 5.0, "change-internal-gains-w-m2/raises-the-internal-gains-to-5-w-per-m2: the internal gains must be 5 W/m2");
    assert_eq!(snapshot.solar_gains_kwh, before().solar_gains_kwh, "change-internal-gains-w-m2/raises-the-internal-gains-to-5-w-per-m2: the solar gains are a separate absolute figure");''',
    '''    assert_eq!(raised_diff.internal_gains_w_m2, Some(5.0), "change-internal-gains-w-m2/raises-the-internal-gains-to-5-w-per-m2: the diff must publish internalGainsWM2 = 5");
    assert!(raised_diff.occupants.is_none(), "change-internal-gains-w-m2/raises-the-internal-gains-to-5-w-per-m2: the occupant count is not re-derived from the gains");''',
)

# ── 7. change-solar-gains-kwh ─────────────────────────────────────────────────
case(
    "🦥change-solar-gains-kwh",
    "change-solar-gains-kwh",
    "change_solar_gains_kwh",
    "ChangeSolarGainsKwh",
    "raises-the-annual-solar-gains-to-132-kwh",
    "The annual solar heat gains go 84 → 132 kWh. Physically these come from the climate profile, "
    "but the snapshot stores them as their own scalar — so this mutation must NOT touch the "
    "composed `climate` child slot.",
    "solarGainsKwh",
    "new_solar_gains_kwh",
    132.0,
    '''    assert_eq!(snapshot.solar_gains_kwh, 132.0, "change-solar-gains-kwh/raises-the-annual-solar-gains-to-132-kwh: the solar gains must be 132 kWh");
    assert_eq!(snapshot.climate.child_id, before().climate.child_id, "change-solar-gains-kwh/raises-the-annual-solar-gains-to-132-kwh: the composed climate child handle must be identical, not re-minted");''',
    '''    assert_eq!(raised_diff.solar_gains_kwh, Some(132.0), "change-solar-gains-kwh/raises-the-annual-solar-gains-to-132-kwh: the diff must publish solarGainsKwh = 132");
    assert!(raised_diff.climate.is_none(), "change-solar-gains-kwh/raises-the-annual-solar-gains-to-132-kwh: the climate child slot must stay null — only update-climate may write it");''',
)

# ── 8. change-system-losses-kwh ───────────────────────────────────────────────
case(
    "🦦change-system-losses-kwh",
    "change-system-losses-kwh",
    "change_system_losses_kwh",
    "ChangeSystemLossesKwh",
    "cuts-the-system-losses-to-450-kwh",
    "Annual distribution/storage/generation losses drop 800 → 450 kWh after a plant-room upgrade. "
    "The renewable yield that offsets them is a separate scalar and stays put.",
    "systemLossesKwh",
    "new_system_losses_kwh",
    450.0,
    '''    assert_eq!(snapshot.system_losses_kwh, 450.0, "change-system-losses-kwh/cuts-the-system-losses-to-450-kwh: the system losses must be 450 kWh");
    assert_eq!(snapshot.renewable_kwh, before().renewable_kwh, "change-system-losses-kwh/cuts-the-system-losses-to-450-kwh: the renewable yield must not move with the losses");''',
    '''    assert_eq!(raised_diff.system_losses_kwh, Some(450.0), "change-system-losses-kwh/cuts-the-system-losses-to-450-kwh: the diff must publish systemLossesKwh = 450");
    assert!(raised_diff.renewable_kwh.is_none(), "change-system-losses-kwh/cuts-the-system-losses-to-450-kwh: renewableKwh must stay null");''',
)

# ── 9. change-renewable-kwh ───────────────────────────────────────────────────
case(
    "🦨change-renewable-kwh",
    "change-renewable-kwh",
    "change_renewable_kwh",
    "ChangeRenewableKwh",
    "raises-the-on-site-renewable-yield-to-2250-kwh",
    "The on-site renewable yield credited against primary energy goes 1500 → 2250 kWh (a larger PV "
    "array). Neither the system losses it offsets nor the annual limit it is judged against move.",
    "renewableKwh",
    "new_renewable_kwh",
    2250.0,
    '''    assert_eq!(snapshot.renewable_kwh, 2250.0, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: the renewable yield must be 2250 kWh");
    assert_eq!(snapshot.annual_limit_kwh, before().annual_limit_kwh, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: the compliance limit is not lowered by generating more");''',
    '''    assert_eq!(raised_diff.renewable_kwh, Some(2250.0), "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: the diff must publish renewableKwh = 2250");
    assert!(raised_diff.system_losses_kwh.is_none(), "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: systemLossesKwh must stay null");''',
)

# ── 10. change-annual-limit-kwh ───────────────────────────────────────────────
case(
    "🦡change-annual-limit-kwh",
    "change-annual-limit-kwh",
    "change_annual_limit_kwh",
    "ChangeAnnualLimitKwh",
    "tightens-the-annual-primary-energy-limit-to-6000-kwh",
    "The permitted annual primary-energy demand tightens 7500 → 6000 kWh. This is the compliance "
    "THRESHOLD; the reference building's own demand (`reference_q_p_kwh`) is a different field and "
    "must not follow it.",
    "annualLimitKwh",
    "new_annual_limit_kwh",
    6000.0,
    '''    assert_eq!(snapshot.annual_limit_kwh, 6000.0, "change-annual-limit-kwh/tightens-the-annual-primary-energy-limit-to-6000-kwh: the annual limit must be 6000 kWh");
    assert_eq!(snapshot.reference_q_p_kwh, before().reference_q_p_kwh, "change-annual-limit-kwh/tightens-the-annual-primary-energy-limit-to-6000-kwh: the reference building's demand is a different field");''',
    '''    assert_eq!(raised_diff.annual_limit_kwh, Some(6000.0), "change-annual-limit-kwh/tightens-the-annual-primary-energy-limit-to-6000-kwh: the diff must publish annualLimitKwh = 6000");
    assert!(raised_diff.reference_q_p_kwh.is_none(), "change-annual-limit-kwh/tightens-the-annual-primary-energy-limit-to-6000-kwh: referenceQPKwh must stay null");''',
)

# ── 11. change-energy-carrier ─────────────────────────────────────────────────
case(
    "📐change-energy-carrier",
    "change-energy-carrier",
    "change_energy_carrier",
    "ChangeEnergyCarrier",
    "switches-the-energy-carrier-to-an-electric-heat-pump",
    "The delivered-energy carrier string goes `natural_gas` → `electricity_heat_pump`, which is what "
    "selects the primary-energy factor. `energy_carrier` is the snapshot's only `String` field, so "
    "the diff must carry a JSON string here.",
    "energyCarrier",
    "new_energy_carrier",
    "electricity_heat_pump",
    '''    assert_eq!(snapshot.energy_carrier, "electricity_heat_pump", "change-energy-carrier/switches-the-energy-carrier-to-an-electric-heat-pump: the carrier must be electricity_heat_pump");
    assert_eq!(snapshot.system_losses_kwh, before().system_losses_kwh, "change-energy-carrier/switches-the-energy-carrier-to-an-electric-heat-pump: swapping the carrier does not re-derive the plant losses");''',
    '''    assert_eq!(raised_diff.energy_carrier.as_deref(), Some("electricity_heat_pump"), "change-energy-carrier/switches-the-energy-carrier-to-an-electric-heat-pump: the diff must publish energyCarrier as a string");
    assert!(raised_diff.use_class.is_none(), "change-energy-carrier/switches-the-energy-carrier-to-an-electric-heat-pump: the use class must stay null");''',
)

# ── 12. change-reference-qp-kwh ───────────────────────────────────────────────
case(
    "🔽change-reference-qp-kwh",
    "change-reference-q-p-kwh",
    "change_reference_q_p_kwh",
    "ChangeReferenceQPKwh",
    "lowers-the-reference-building-primary-energy-to-8750-kwh",
    "The reference building's annual primary-energy demand Q_p drops 10000 → 8750 kWh. Serde's "
    "camelCase rule turns `reference_q_p_kwh` into `referenceQPKwh` — three consecutive capitals — "
    "so this case doubles as the pin on that field's wire name.",
    "referenceQPKwh",
    "new_reference_q_p_kwh",
    8750.0,
    '''    assert_eq!(snapshot.reference_q_p_kwh, 8750.0, "change-reference-q-p-kwh/lowers-the-reference-building-primary-energy-to-8750-kwh: the reference demand must be 8750 kWh");
    assert_eq!(snapshot.annual_limit_kwh, before().annual_limit_kwh, "change-reference-q-p-kwh/lowers-the-reference-building-primary-energy-to-8750-kwh: the compliance limit is a different field");''',
    '''    assert_eq!(raised_diff.reference_q_p_kwh, Some(8750.0), "change-reference-q-p-kwh/lowers-the-reference-building-primary-energy-to-8750-kwh: the diff must publish referenceQPKwh = 8750");
    assert!(raised_diff.annual_limit_kwh.is_none(), "change-reference-q-p-kwh/lowers-the-reference-building-primary-energy-to-8750-kwh: annualLimitKwh must stay null");''',
)

# ── 13. update-climate (REJECTED) ─────────────────────────────────────────────
# 🌡️ A plausible German Zone 2 profile whose JANUARY irradiance is negative — physically impossible,
# and exactly what `update-climate`'s `mutation.invariant` guard exists to refuse. A non-finite
# temperature would trip the same guard but cannot be written in JSON at all (no Infinity literal),
# so the negative-irradiance half of the predicate is the one a committed fixture can pin.
BAD_CLIMATE = {
    "theta_e_c": [-2.0, -1.0, 3.5, 8.0, 13.5, 17.0, 19.0, 18.5, 14.0, 9.0, 4.0, 0.5],
    "g_h_w_m2": [-30.0, 60.0, 100.0, 140.0, 180.0, 200.0, 210.0, 190.0, 140.0, 90.0, 40.0, 20.0],
}

REJECTED_CASE = dict(
    leaf="🐘update-climate",
    kind="update-climate",
    module="update_climate",
    case="refuses-a-negative-january-irradiance",
    summary=(
        "`update-climate` is the only DIN V 18599 mutation that writes the composed `climate` child "
        "slot, and the only one whose payload carries real data (a literal twelve-month "
        "`MonthlyClimate`) rather than a handle. Its `mutation.invariant` guard runs BEFORE the "
        "no-op check and refuses any non-finite value or any negative global irradiance: here "
        "January's `g_h_w_m2` is -30 W/m2, so the builder returns a FATAL outcome with the default "
        "all-null diff and the snapshot — climate handle included — is left untouched."
    ),
    mutation={"UpdateClimate": {"new_climate": BAD_CLIMATE}},
    outcome={"status": "rejected", "code": "mutation.invariant", "messages": [{"level": "fatal", "code": "mutation.invariant"}]},
    extra_rejected='''    assert_eq!(snapshot.climate.child_id, before().climate.child_id, "update-climate/refuses-a-negative-january-irradiance: the composed climate child handle must not be re-minted by a refused payload");
    assert_eq!(snapshot.climate.target.artifact_id, "din18599-climate", "update-climate/refuses-a-negative-january-irradiance: the child slot must still point at the same table artifact");
    assert_eq!(snapshot.solar_gains_kwh, before().solar_gains_kwh, "update-climate/refuses-a-negative-january-irradiance: no climate-derived scalar may be recomputed from a refused payload");''',
)

# ── emit ──────────────────────────────────────────────────────────────────────
assert len(CASES) == 12, len(CASES)

for entry in CASES:
    rust = common.test_source(
        artifact="din18599",
        snapshot_ty="Din18599Snapshot",
        diff_ty="Din18599Diff",
        mutation_ty="Din18599Mutation",
        kind=entry["kind"],
        case=entry["case"],
        summary=entry["summary"],
        extra_applied=entry["extra_applied"],
        extra_diff=entry["extra_diff"],
    )
    common.emit_case(ROOT, entry["leaf"], entry["case"], BEFORE, entry["after"], entry["mutation"], entry["diff"], APPLIED, rust)

rejected_rust = common.rejected_test_source(
    artifact="din18599",
    snapshot_ty="Din18599Snapshot",
    diff_ty="Din18599Diff",
    mutation_ty="Din18599Mutation",
    kind=REJECTED_CASE["kind"],
    case=REJECTED_CASE["case"],
    summary=REJECTED_CASE["summary"],
    extra_rejected=REJECTED_CASE["extra_rejected"],
)
common.emit_rejected_case(ROOT, REJECTED_CASE["leaf"], REJECTED_CASE["case"], BEFORE, REJECTED_CASE["mutation"], REJECTED_CASE["outcome"], rejected_rust)

lines = []
for entry in CASES + [REJECTED_CASE]:
    lines.append('    #[path = "{}/🧪️tests/{}/🦀️component.rs"]'.format(entry["leaf"], entry["case"]))
    lines.append("    mod tests_{}_{};".format(entry["module"], entry["case"].replace("-", "_")))
print("\n".join(lines))
