from spec import *
p = SCH + "/📸️snapshot/🦀️.rs"
t = open(p, encoding="utf8").read()
if "FatigueSpectrum" in t: raise SystemExit("already applied")

edit(p, """    #[dsl(table)]
    #[state(artifact)]
    pub effects: Vec<MemberEffect>,
}
""", """    #[dsl(table)]
    #[state(artifact)]
    pub effects: Vec<MemberEffect>,
    #[dsl(table)]
    #[state(artifact)]
    pub fatigue_spectra: Vec<FatigueSpectrum>,
    #[dsl(table)]
    #[state(artifact)]
    pub reliability_analyses: Vec<ReliabilityAnalysis>,
    #[dsl(table)]
    #[state(artifact)]
    pub test_series: Vec<TestSeries>,
}
""")
edit(p, """                MemberEffect { member_id: "beam-B1".into(), action_id: "Q-snow".into(), influence: 1.0 },
            ],
""", """                MemberEffect { member_id: "beam-B1".into(), action_id: "Q-snow".into(), influence: 1.0 },
            ],
            fatigue_spectra: vec![],
            reliability_analyses: vec![],
            test_series: vec![],
""")
for cat, qk in (("office", "40_000.0"), ("wind", "20_000.0"), ("snow", "15_000.0")):
    edit(p, f'category: "{cat}".into(), qk: {qk} }}', f'category: "{cat}".into(), qk: {qk}, group: String::new() }}')
edit(p, """/// 🏋️ Variable action — category selects ψ (site altitude is on the snapshot).""", """/// 🏋️ Variable action — category selects ψ (site altitude is on the snapshot); actions sharing a non-empty `group` are mutually exclusive (EN 1990 §4.1.1, pattern loading per span: one action per loaded span in one group).""")
edit(p, """    pub category: String,
    pub qk: f64,
}""", """    pub category: String,
    pub qk: f64,
    pub group: String,
}""")
DER = '#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]\n#[canonical_json(owner = semio_framework_pack_json)]\n#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n#[cfg_attr(test, serde(rename_all = "camelCase"))]\n#[value(rename_all = "camelCase")]\n'
out = ""
for e in ENTITIES.values():
    out += f"/// {e['emoji']} {e['doc']}\n" + DER + f"pub struct {e['name']} {{\n" + "".join(f"    pub {f['snake']}: {f['ty']},\n" for f in e["fields"]) + "}\n\n"
edit(p, "/// 🗿️ The computed-compliance artifact", out + "/// 🗿️ The computed-compliance artifact")
print("model ok")
