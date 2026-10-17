"""🗺️ c4p11 split plans for Package 11: the eight norm check files (din18599, en1997 checks and test root, din16798, en1992, vdi3805 document and sheet checks, iso16757 part 1).
`mode` "mod": children are `#[path]` modules (`use super::*;`, root `<vis> use <mod>::*;` when the child exports a name another file uses). `ranges` are 1-based inclusive line ranges of the PREPARED text (the baseline plus the declared function extractions, see `extract`); each starts at an item start (docs and attributes included) and ends at an item end or its trailing region marker.
`mode` "include": the file is a test module; children are fragments spliced back with `include!`.
`extract`: a self-contained tail of a function longer than a file may hold becomes its own function (lines moved verbatim)."""
from c4plib import *

def K(key):
    return next(rel for rel, k in KEYS.items() if k == key)


def C(emoji, slug, ranges, doc, force=()):
    e = emoji + "\ufe0f"
    return dict(folder=e + slug, mod=slug.replace("-", "_"), ranges=ranges, doc=f"{e} {doc}", force=list(force))


VDI_LIB = f"{NORM}/🏭️vdi3805/{INF}/🦀️.rs"
PLANS = {
    "dsn": dict(rel=K("dsn"), mode="mod", lib=K("dsn"), children=[
        C("🧮", "balance-model", [(17, 294)], "DIN V 18599 balance model: usage profiles, zone transmission and ventilation, solar gains, utilization and the zone balances."),
        C("📜", "norm-tables", [(296, 467)], "DIN V 18599 and GEG norm tables: primary energy factors, reference U-values, hours, air change and profile constants."),
        C("🔥", "derived-balance", [(469, 634)], "DIN V 18599 derived balance: domestic hot water, element paths, the whole-building balance and the required U-value for H_T'."),
        C("🩺", "document-evaluation", [(638, 1394)], "DIN V 18599 document evaluation: duplicate ids and the clause-by-clause evaluation of a snapshot."),
    ]),
    "e97": dict(
        rel=K("e97"), mode="mod", lib=K("e97"), children=[
            C("🧭", "design-approach-and-parameters", [(17, 227)], "EN 1997 design approach: national annex factors, design situations and the resolved partial-factor and uplift parameters."),
            C("📐", "geotechnical-parts", [(229, 833)], "EN 1997 geotechnical parts: the clause modules of EN 1997-1 and EN 1997-2 with their shared subject helpers."),
            C("🪨", "layer-and-duplicate-id-helpers", [(835, 1056)], "EN 1997 layer lookup, required-width search, design-situation labels and the duplicate-id and governing-situation checks."),
            C("🏗", "project-checks", [(1058, 1640)], "EN 1997 project checks: footing and layer checks of a project snapshot and the governing situation."),
            C("🧱", "pile-wall-slope-and-uplift-checks", [(1642, 2178)], "EN 1997 pile, retaining wall, overall stability and uplift checks."),
        ],
        extract=[dict(
            name="check_piles_walls_slopes_and_uplift", cut=(1610, 2143), host_end=2173,
            head=["/// 🧱️ Pile, retaining wall, overall stability and uplift checks of [`check_project`]: the second half of its design loop, moved out unchanged.",
                  "fn check_piles_walls_slopes_and_uplift(doc: &En1997Snapshot, report: &mut CheckReport, approach: DesignApproach, annex: AnnexChoice, gwl: f64, part1: &str, part1054: &str, design_situations: &[String]) {"],
            call=["    check_piles_walls_slopes_and_uplift(doc, &mut report, approach, annex, gwl, part1, part1054, &design_situations);"])]),
    "e97t": dict(rel=K("e97t"), mode="include", lib=None, children=[
        C("🧮", "design-approach-and-resistance-laws", [(14, 282)], "Design approach, resistance, settlement, pile and remedy laws."),
        C("🔮", "oracle-and-remedy-laws", [(283, 529)], "Python oracle parity, snapshot schema, remedy application and sensitivity laws."),
        C("🏷", "field-meta-and-perturbation-laws", [(546, 1132)], "Field metadata, perturbation, typed facet and duplicate-id laws."),
    ]),
    "d16": dict(rel=K("d16"), mode="mod", lib=K("d16"), children=[
        C("🎛", "comfort-models-and-parsers", [(16, 152)], "DIN EN 16798 comfort, ventilation system and method models with the usage, category and pollution parsers."),
        C("📜", "annex-parameters-and-parts", [(154, 475)], "DIN EN 16798 annex parameters and the clause modules of parts 1, 3, 5-1, 7 and 17."),
        C("🔎", "environment-check-entry", [(477, 586)], "DIN EN 16798 environment check entry: duplicate ids and the complete building evaluation."),
        C("🏠", "zone-evaluation", [(588, 958)], "DIN EN 16798 zone evaluation: comfort, pollution, acoustics and thermal checks of one zone."),
        C("🌀", "vent-and-envelope-evaluation", [(960, 1253)], "DIN EN 16798 ventilation system and envelope airtightness evaluation."),
    ]),
    "e92": dict(rel=K("e92"), mode="mod", lib=K("e92"), children=[
        C("🧰", "helpers-and-annex-parts", [(18, 657)], "EN 1992 quantity and path helpers, the national annex and the clause modules of parts 1-1, 1-2, 2, 3 and 4."),
        C("📦", "design-effects", [(661, 835)], "EN 1992 design effects: EN 1990 combination of member actions and the governing effect."),
        C("📋", "member-evaluation", [(837, 1693)], "EN 1992 member evaluation against EN 1992-1-1 and 1-2, with optional bridge and liquid-retaining checks."),
        C("⚓", "anchor-evaluation", [(1695, 1980)], "EN 1992 anchor evaluation: EN 1990 combination of anchor actions and the EN 1992-4 resistances."),
    ]),
    "vdc": dict(
        rel=K("vdc"), mode="mod", lib=VDI_LIB, children=[
            C("🗃", "catalog-integrity", [(6, 913)], "VDI 3805 catalog integrity: limits, correction date, product, index and geometry checks of a snapshot."),
            C("🗂", "file-header-and-historical-checks", [(915, 1252)], "VDI 3805 catalog file header, historical sheet, edition profile and operative sheet coverage checks."),
        ],
        extract=[dict(
            name="check_catalog_file_header", cut=(910, 1122), host_end=1125,
            head=["/// 🗂️ Catalog file header checks of [`check_catalog_integrity`] (header version, building-system number, created date, manufacturer, charset), moved out unchanged.",
                  "fn check_catalog_file_header(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>) {"],
            call=["    check_catalog_file_header(document, &mut out);"])]),
    "vds": dict(rel=K("vds"), mode="mod", lib=VDI_LIB, children=[
        C("🚰", "valve-radiator-and-pump-checks", [(185, 715)], "VDI 3805 valve, radiator and pump sheet checks."),
        C("🔥", "heat-generator-and-operative-sheet-checks", [(717, 1056)], "VDI 3805 heat generator and operative sheet checks."),
    ]),
    "iso": dict(rel=K("iso"), mode="mod", lib=K("iso"), children=[
        C("🪪", "header-ids-and-referential-integrity", [(69, 670)], "ISO 16757-1 header, dictionary alignment, unique id and referential integrity checks."),
        C("🌐", "properties-and-multilingual-checks", [(672, 1408)], "ISO 16757-1 group dictionary subjects, multilingual texts, required properties, property values, units and variant domains."),
        C("🧩", "variants-accessories-and-indexes", [(1410, 1961)], "ISO 16757-1 variant article and geometry, accessory composition and product index checks."),
        C("🎯", "selection-and-bim-checks", [(1963, 2396)], "ISO 16757-1 selection and BIM embedding checks."),
    ]),
}
ORDER = ["dsn", "e97", "e97t", "d16", "e92", "vdc", "vds", "iso"]
