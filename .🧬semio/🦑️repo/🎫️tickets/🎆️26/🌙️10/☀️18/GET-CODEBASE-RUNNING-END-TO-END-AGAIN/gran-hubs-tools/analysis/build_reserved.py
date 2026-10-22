"""🔒️ Plan `umbrella-reserved`: the one own definition of the umbrella, `FRAMEWORK_RESERVED_CONTRIBUTIONS`, moves into the new leaf crate `semio-framework-manifest-reserved`; the umbrella keeps nothing else it must define. Writes own-map.json (umbrella) and newcrates.json. `python3 build_reserved.py`"""
from hubenv import *
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from planlib import Plan, manifest_text, add_member, rel_to
world, I2L, I2D, LIB = make_world()
D = "🧰️framework/🔨️modules/🛂️manifest/🔒️reserved"
M = D + "/📦️packages/🦀️rust"
plan = Plan("umbrella-reserved", "umbrella", "semio-framework")
need = ["semio-framework-manifest-app-surface", "semio-framework-time-travel-actions", "semio-framework-tool-run-actions"]
deps = [(n, rel_to(M, byname[n])) for n in need]
plan.create(D + "/🦀️.rs", '''//! 🔒️ The framework features that reserve actions or dialogs on every hosting app.

/// 🧩️ The framework features that reserve actions or dialogs on every hosting app, in injection order: the history-edit
/// verbs join the app roster after the history lane, the tool run actions join every window kind after the interaction
/// verbs. A host adds its own contributions (such as audience rules) after these.
pub const FRAMEWORK_RESERVED_CONTRIBUTIONS: [semio_framework_manifest_app_surface::ReservedContribution; 2] = [semio_framework_time_travel_actions::RESERVED, semio_framework_tool_run_actions::RESERVED];
''')
plan.create(M + "/Cargo.toml", manifest_text("semio-framework-manifest-reserved", "manifest-reserved", "The reserved contributions every hosting app receives from the framework", deps, "../../../../..", "../../🦀️.rs"))
add_member(plan, "🧰️framework/Cargo.toml", "🔨️modules/🛂️manifest/🕹️actions/📦️packages/🦀️rust", M[len("🧰️framework/"):])
print(plan.write(os.path.normpath(T + "/../staged/plans")))
nc = json.load(open(OUT + "/newcrates.json")) if os.path.exists(OUT + "/newcrates.json") else {}
nc["semio_framework_manifest_reserved"] = {"name": "semio-framework-manifest-reserved", "dir": M}
json.dump(nc, open(OUT + "/newcrates.json", "w"), indent=1)
om = json.load(open(OUT + "/own-map.json")); om.setdefault("umbrella", {})["FRAMEWORK_RESERVED_CONTRIBUTIONS"] = ["semio_framework_manifest_reserved", ["FRAMEWORK_RESERVED_CONTRIBUTIONS"]]
json.dump(om, open(OUT + "/own-map.json", "w"), indent=1)
