"""🩹️ Root patch of the energy Rust adapter after the table split: explicit table imports, the chained iterator and its three callers."""
p = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🦀️.rs"
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert s.count(a) == 1, a
    s = s.replace(a, b)
rep('''#[path = "📆️schedules-and-layers/🦀️.rs"]
mod schedules;
''', '''#[path = "📆️schedules-and-layers/🦀️.rs"]
mod schedules;
use self::airflow::AIRFLOW_VECTORS;
use self::building::BUILDING_VECTORS;
use self::fabric::FABRIC_VECTORS;
use self::openings::OPENING_VECTORS;
use self::power::POWER_VECTORS;
use self::schedules::SCHEDULE_VECTORS;
use self::zone_systems::ZONE_SYSTEM_VECTORS;

/// 🗂️ Every committed specification vector in catalog order, chained across the domain tables.
fn vectors() -> impl Iterator<Item = &'static Vector> {
    BUILDING_VECTORS.iter().chain(OPENING_VECTORS).chain(FABRIC_VECTORS).chain(AIRFLOW_VECTORS).chain(ZONE_SYSTEM_VECTORS).chain(POWER_VECTORS).chain(SCHEDULE_VECTORS)
}
''')
rep("    VECTORS.iter().find(|vector| vector.id == id).unwrap_or_else", "    vectors().find(|vector| vector.id == id).unwrap_or_else")
rep("debug_assert!(KINDS.iter().all(|kind| VECTORS.iter().any(|vector| vector.kind == *kind)));", "debug_assert!(KINDS.iter().all(|kind| vectors().any(|vector| vector.kind == *kind)));")
rep("    for vector in VECTORS {", "    for vector in vectors() {")
open(p, "w", encoding="utf-8").write(s)
print("patched")
