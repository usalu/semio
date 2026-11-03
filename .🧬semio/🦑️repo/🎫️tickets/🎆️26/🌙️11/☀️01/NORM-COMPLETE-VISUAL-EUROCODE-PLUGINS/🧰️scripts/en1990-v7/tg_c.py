"""Traffic groups layer C: field-meta rows, outline field, test path lists."""
from tg_lib import *
from tg_a import put
FM = SUB + "/✏️editor/🏷️field-meta"
def fm(t):
    if "trafficGroups" in t:
        return t
    choices = "const LOAD_GROUPS: &[NormFieldChoice] = &[\n" + "".join('    choice("%s", "%s", "%s"),\n' % (k, OPTIONS[k][0], OPTIONS[k][1]) for k in KEYS) + "];\n"
    t = put(t, "const IMPORTANCE: &[NormFieldChoice] = &[", choices + "const IMPORTANCE: &[NormFieldChoice] = &[")
    rows = ('    ("trafficGroups", FieldSpec::new("Traffic load groups (EN 1991-2 Table 4.4a)", "Verkehrslastgruppen (EN 1991-2 Tabelle 4.4a)")),\n'
            '    ("trafficGroups[].id", FieldSpec::new("Group row id", "Gruppenzeilen-Id")),\n'
            '    ("trafficGroups[].group", FieldSpec::choice("Traffic load group", "Verkehrslastgruppe", LOAD_GROUPS)),\n'
            '    ("trafficGroups[].variableId", FieldSpec::new("Load component (variable action id)", "Lastbestandteil (Id der veränderlichen Einwirkung)")),\n')
    return put(t, '    ("factorOverrides[].justification", FieldSpec::new("Justification", "Begründung")),\n', '    ("factorOverrides[].justification", FieldSpec::new("Justification", "Begründung")),\n' + rows)
edit(FM + "/🦀️.rs", fm)
def tests(t):
    if "trafficGroups" in t:
        return t
    return put(t, '        "factorOverrides[].justification",\n', '        "factorOverrides[].justification",\n        "trafficGroups",\n        "trafficGroups[].id",\n        "trafficGroups[].group",\n        "trafficGroups[].variableId",\n')
edit(FM + "/🧪️tests/🔬️unit/🦀️.rs", tests)
for p in (INF + "/🦀️.rs", INF + "/🧾outline/🦀️.rs"):
    edit(p, lambda t: t if "trafficGroups" in t else re.sub(r'(\n( +)"factorOverrides",\n)', lambda m: m.group(1) + m.group(2) + '"trafficGroups",\n', t, count=1))
edit(INF + "/🧾outline/🦀️.rs", lambda t: t if "snapshot.traffic_groups.len()" in t else rep(t, "snapshot.factor_overrides.len()) as u32", "snapshot.factor_overrides.len() + snapshot.traffic_groups.len()) as u32"))
