"""🗺️ c4p25 split plans for Package 25: remodel optimize internals, the energy engine kernel, model, precompute and sim-wire lib roots, the energy model editor root and its unit tests, and the epJSON serializer.
`mode` "mod": children are `#[path]` modules (`use super::*;`, root `<vis> use <mod>::*;` when the child exports a name another file uses). `ranges` are 1-based inclusive line ranges of the pre-edit file.
Boundaries are computed from item names and region names of the pre-edit blob, never typed as raw line numbers: `S(key, name)` is the first line of the doc/attribute block of the item `name`, `T(key, text)` of the first line containing `text`, `R(key, region)` the span of a region.
`pre` / `post` lines wrap a child body (a region that spans two children is repeated, balanced, in each, as in `RESULT-REGION-RESTORE.md`); `macro_use` mounts the child with `#[macro_use]` (it defines a `macro_rules!` another child expands)."""
import re
from c4plib import *
import splitter as S
from rslex import outline


def _ls(key):
    return lines_of(read(f"{ORIG}/{key}.rs"))


def S(key, name, nth=1):
    ls = _ls(key)
    its = [it for it in outline("\n".join(ls) + "\n") if it["name"] == name and it["kind"] not in ("use", "other")]
    assert len(its) >= nth, (key, name)
    return _block_start(ls, its[nth - 1]["start"] + 1)


def T(key, text, nth=1):
    ls = _ls(key)
    hits = [i + 1 for i, l in enumerate(ls) if text in l]
    assert len(hits) >= nth, (key, text)
    return _block_start(ls, hits[nth - 1])


def _is_c(l):
    s = l.strip()
    return (s.startswith(("///", "//", "#[")) and not s.startswith(("//#region", "//#endregion", "// #region", "// #endregion")))


def _block_start(ls, line):
    i = line - 1
    while i - 1 >= 0:
        prev = ls[i - 1]
        if _is_c(prev):
            i -= 1
            continue
        if prev.strip() == "":
            j = i - 2
            while j >= 0 and ls[j].strip() == "":
                j -= 1
            if j >= 0 and _is_c(ls[j]) and ls[j].strip().startswith("///"):
                i = j + 1
                continue
        break
    return i + 1


def R(key, name, which=1):
    ls = _ls(key)
    stack, out = [], []
    for i, l in enumerate(ls, 1):
        m = re.match(r"^\s*//\s?#region\s*(.*)$", l)
        if m:
            stack.append((i, m.group(1).strip()))
            continue
        m = re.match(r"^\s*//\s?#endregion\b", l)
        if m and stack:
            s, n = stack.pop()
            if n == name:
                out.append((s, i))
    out.sort()
    return out[which - 1]


def OPEN(key, name):
    for i, l in enumerate(_ls(key), 1):
        m = re.match(r"^\s*//\s?#region\s*(.*)$", l)
        if m and m.group(1).strip() == name:
            return i
    raise KeyError((key, name))


def END(key):
    return len(_ls(key))


def _kids(specs):
    out = []
    for folder, mod, doc, ranges, extra in specs:
        d = dict(folder=folder, mod=mod, ranges=ranges, doc=doc)
        d.update(extra)
        out.append(d)
    return out


def _plan(rel, lib, specs, mode="mod", **more):
    d = dict(rel=rel, mode=mode, lib=lib, children=_kids(specs))
    d.update(more)
    return d


RMLIB = f"{RM}/📦️packages/🦀️rust/🦀️.rs"


def rem():
    k = "rem"
    return _plan(f"{RM}/🦀️.rs", RMLIB, [
        ("🧮️problem-and-robust-losses", "problem_and_robust_losses", "🧮️ The least-squares problem trait, the numeric Jacobian and the robust loss functions.", [(R(k, "🔖️Problem")[0], R(k, "🔖️RobustLoss")[1])], {}),
        ("📐️newton-and-levenberg-marquardt", "newton_and_levenberg_marquardt", "📐️ The Gauss-Newton step and the Levenberg-Marquardt damped solver with its configuration and result.", [(R(k, "🔖️GaussNewton")[0], R(k, "🔖️LevenbergMarquardt")[1])], {}),
        ("🕸️schur-bundle", "schur_bundle", "🕸️ The Schur-complement bundle solver: bipartite residual terms, the accumulator, the damped step, covariances and the resumable preparation.", [R(k, "🔖️SchurBundle")], {}),
        ("🎲️consensus-and-scalar-search", "consensus_and_scalar_search", "🎲️ RANSAC and LO-RANSAC consensus estimation and the golden-section and Brent scalar minimisers.", [(R(k, "🔖️Consensus")[0], R(k, "🔖️Scalar")[1])], {}),
    ])


def pre():
    k = "pre"
    return _plan(f"{EN}/🧠️precompute/🦀️.rs", f"{EN}/🧠️precompute/🦀️.rs", [
        ("🧾️records-and-precomputed-model", "records_and_precomputed_model", "🧾️ The per-zone, surface, window and enclosure precompute records and the `PrecomputedModel` aggregate.", [(R(k, "🔖️Records")[0], R(k, "🔖️PrecomputedModel")[1])], {}),
        ("🏗️precompute-builder", "precompute_builder", "🏗️ The staged `PrecomputeBuilder` cursor that derives geometry, constructions, glazing optics, shading casters and enclosures.", [R(k, "🔖️PrecomputeBuilder")], {}),
        ("🔺️geometry-and-enclosure-factors", "geometry_and_enclosure_factors", "🔺️ Fenestration placement, polygon helpers and the gray-body enclosure radiation factors.", [R(k, "🔖️Geometry")], {}),
    ])


def mod():
    k = "mod"
    return _plan(f"{EN}/🔋️model/🦀️.rs", f"{EN}/🔋️model/🦀️.rs", [
        ("🏷️ids-and-fixed-table", "ids_and_fixed_table", "🏷️ The entity identifier, the glazing pane limit and the fixed-capacity table.", [(R(k, "🔖️Ids")[0], R(k, "🔖️FixedTable")[1]), (T(k, "impl semio_framework_dsl_record::BorrowedDslField for EntityId"),) * 2], {}),
        ("🧱️spatial-material-and-gain-entities", "spatial_material_and_gain_entities", "🧱️ The site, zone, surface, fenestration, material, construction and internal gain entities.", [(R(k, "🔖️Site")[0], R(k, "🔖️Gains")[1])], {}),
        ("⚙️hvac-entities", "hvac_entities", "⚙️ The thermostat, ideal loads, setpoint, plant, airflow, electrical, fault, output, sizing and ground temperature entities.", [R(k, "🔖️Hvac")], {}),
        ("🧬️model-aggregate-and-fixtures", "model_aggregate_and_fixtures", "🧬️ The infiltration entity, the `Model` aggregate with its validation and the public test models.", [(R(k, "🔖️Infiltration")[0], R(k, "🔖️Model")[1]), R(k, "🔖️Fixtures")], {}),
    ])


def wire():
    k = "wire"
    tests = T(k, "#[cfg(test)]", 2)
    return _plan(f"{EN}/🧪️sim/📡️wire/🦀️.rs", f"{EN}/🧪️sim/📡️wire/🦀️.rs", [
        ("📬️packets-and-queues", "packets_and_queues", "📬️ The retained wire constants, packets, lease, lossless queues, publication channels and the commit reservation.", [(S(k, "ENERGY_WIRE_MAGIC"), S(k, "EnergyQualityTier") - 1), (S(k, "EnergyCommitReservation"), END(k))], {}),
        ("🛡️job-vocabulary-and-census", "job_vocabulary_and_census", "🛡️ The quality tier, job stage, preview and cursor vocabulary and the numerical admission census with its bounds.", [(S(k, "EnergyQualityTier"), S(k, "observed_weather_records") - 1)], {}),
        ("🔎️observation-and-decode", "observation_and_decode", "🔎️ The observed working-set sizes, the stage and tier code tables, the preview decoder and the packet close.", [(S(k, "observed_weather_records"), tests - 1)], {}),
    ])


def ed():
    k = "ed"
    return _plan(f"{EA}/✏️editor/🦀️.rs", f"{EA}/✏️editor/🦀️.rs", [
        ("🧮️reduce-and-helpers", "reduce_and_helpers", "🧮️ The editor reducer, its run-period, ordering and lookup helpers and the camera emit.", [(S(k, "run_period_steps"), OPEN(k, "🔍️InspectorProperties") - 1)], {}),
        ("🔍️inspector-properties", "inspector_properties", "🔍️ The inspector property setters for surfaces, fenestrations, materials, zones and constructions.", [(OPEN(k, "🔍️InspectorProperties"), OPEN(k, "🧵️RetainedCommands") - 1)], {}),
        ("🎛️retained-commands", "retained_commands", "🎛️ The retained command job: payload schema, extent, reduce and the bounded command work.", [R(k, "🧵️RetainedCommands")], {}),
        ("📬️store-preparation", "store_preparation", "📬️ The one-item store preparation factories of the artifact and config lanes.", [R(k, "📬️StorePreparation")], {}),
        ("🖥️editor-impl", "editor_impl", "🖥️ The `EnergyModelEditor` type with its `ArtifactEditor` implementation and the body renderer.", [R(k, "🔖️Editor")], {}),
        ("📚️examples-and-manifest", "examples_and_manifest", "📚️ The registered examples, the app-level action definitions and the editor manifest constructor.", [(R(k, "📚️Examples")[0], R(k, "🔖️Manifest")[1])], {}),
    ])


def epj():
    k = "epj"
    return _plan(f"{EP}/🦀️.rs", f"{EA}/🚪️io/📤️export/🦀️.rs", [
        ("🏷️names", "constants_and_names", "🏷️ The epJSON release constants, the diagnostic record and the object naming functions.", [(R(k, "🔖️Constants")[0], R(k, "🔖️Names")[1])], {}),
        ("📐️geometry", "geometry_and_helpers", "📐️ The vertex vector math, aperture placement and the field helpers of the encoder.", [(R(k, "🔖️Geometry")[0], R(k, "🔖️Helpers")[1])], {}),
        ("🧬️encoder", "encoder", "🧬️ The model to epJSON encoder: schedules, materials, constructions, zones, surfaces, gains, hvac and outputs.", [R(k, "🔖️Encoder")], {}),
        ("🌉️bridge", "leaf_and_bridge", "🌉️ The serializer leaf registration and the JSON bridge entry points.", [(R(k, "🔖️Leaf")[0], R(k, "🌉️Bridge")[1])], {}),
    ])


def unit():
    k = "unit"
    return _plan(f"{EA}/✏️editor/🧪️tests/🔬️unit/🦀️.rs", f"{EA}/✏️editor/🦀️.rs", [
        ("🧰️definition-and-vocabulary-laws", "", "🧰️ Definition, roster, dispatch classification, document verb and granular vocabulary laws of the editor.", [(S(k, "definition"), OPEN(k, "🧵️DispatchLaw") - 1)], {}),
        ("🧵️dispatch-scrub-and-run-laws", "", "🧵️ Real action route dispatch, scrub gesture, simulation tool run and keybinding laws.", [(OPEN(k, "🧵️DispatchLaw"), OPEN(k, "🔍️InspectorVerbs") - 1)], dict(replace=[['include_str!("../../🧵️simulation-session', 'include_str!("../../../🧵️simulation-session']])),
        ("🔍️inspector-and-verb-laws", "", "🔍️ Inspector verb, action ownership and material and construction verb laws.", [(OPEN(k, "🔍️InspectorVerbs"), R(k, "🧱️MaterialAndConstructionVerbs")[1])], {}),
        ("🌉️host-merge-ownership-and-recolour-laws", "", "🌉️ Host merge round trip, app level ownership and the finalized-run recolour laws.", [(OPEN(k, "🌉️HostMergeRoundTrip"), END(k))], {}),
    ], mode="include")


def kern():
    k = "kern"
    imp = "impl TimestepWork {"
    ihead = T(k, imp)
    zprep = T(k, "fn step_zone_preparation(")
    bal = T(k, "fn step_balance(")
    commit = T(k, "fn step_zone_commit(")
    region_end = R(k, "🔖️TimestepJob")[1]
    ropen = "// #region 🔖️TimestepJob"
    rclose = "// #endregion 🔖️TimestepJob"
    return _plan(f"{EN}/🌰️kernel/🦀️.rs", f"{EN}/🌰️kernel/🦀️.rs", [
        ("⚙️config-and-state", "config_and_state", "⚙️ The simulation environment and configuration, the delivered energy record and the zone, surface, window and solver state.", [(R(k, "🔖️Config")[0], R(k, "🔖️State")[1])], {}),
        ("📋️timestep-job-types", "timestep_job_types", "📋️ The timestep stage vocabulary, the resumable work records, the builder and the cursor accessors of `TimestepWork`.", [(OPEN(k, "🔖️TimestepJob"), ihead - 1), (ihead + 1, zprep - 1, imp)], dict(post=[rclose])),
        ("🧭️zone-preparation", "zone_preparation", "🧭️ The per-zone preparation step and the schedule lookup cursor it drives.", [[ropen], (zprep, bal - 1, imp), [rclose], (S(k, "advance_zone_preparation"), OPEN(k, "🔖️Kernel") - 1)], {}),
        ("🧮️zone-balance", "zone_balance", "🧮️ The coupled surface and zone heat balance: window solar, beam patches, faces, the dense solve, settling and commit.", [[ropen], (bal, commit - 1, imp), [rclose]], {}),
        ("🏭️commit-and-secondary-systems", "commit_and_secondary_systems", "🏭️ The zone commit and the bounded plant, PV, battery, hot water, refrigeration and water stages.", [[ropen], (commit, region_end - 2, imp), (region_end, region_end)], {}),
        ("🧰️kernel-helpers", "kernel_helpers", "🧰️ The delivered energy accumulation, state pane source, inside face temperature and default weather helpers.", [(S(k, "accumulate_delivered"), T(k, "#[cfg(test)]", 3) - 1)], {}),
    ], drop=[(ihead, ihead), (region_end - 1, region_end - 1)])


PLANS = {"rem": rem(), "kern": kern(), "mod": mod(), "pre": pre(), "wire": wire(), "ed": ed(), "unit": unit(), "epj": epj()}
ORDER = ["rem", "kern", "mod", "pre", "wire", "ed", "unit", "epj"]
