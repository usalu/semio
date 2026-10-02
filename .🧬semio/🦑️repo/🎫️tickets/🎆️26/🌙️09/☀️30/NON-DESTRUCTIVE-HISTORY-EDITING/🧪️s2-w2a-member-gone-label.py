"""🧩️ Adds `timeTravel.member-gone` (a composed member closed mid-session) to the time-travel vocabulary in every twin:
Rust `TimeTravelLabel` + code table, TS twin, schema, lifecycle-law fixture and its generator, and the plugin's raise site."""
import json, pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
FWT = ROOT / "🧰️framework/🔨️modules/⏪️time-travel"
GEN = pathlib.Path(__file__).with_name("🧪️w1-b-generate-lifecycle-law.py")
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs"
EN = "The part this history edit targets was closed"
DE = "Der Teil, den diese Verlaufsbearbeitung betrifft, wurde geschlossen"


def patch(path, pairs):
    t = path.read_text()
    for a, b in pairs:
        if t.count(b) == 1 and t.count(a) == 0:
            continue
        assert t.count(a) == 1, (path.name, t.count(a), a[:90])
        t = t.replace(a, b)
    path.write_text(t)


patch(FWT / "🦀️.rs", [
    ("""/// 🧨️ Driver fault: the finalize commit failed for a reason other than a stale base or a blocking report.
pub const TIME_TRAVEL_COMMIT_FAILED_CODE: &str = "timeTravel.commit-failed";
""", """/// 🧨️ Driver fault: the finalize commit failed for a reason other than a stale base or a blocking report.
pub const TIME_TRAVEL_COMMIT_FAILED_CODE: &str = "timeTravel.commit-failed";

/// 🧩️ Driver fault: the composed member store the session edits was closed mid-session.
pub const TIME_TRAVEL_MEMBER_GONE_CODE: &str = "timeTravel.member-gone";
"""),
    ("""    CommitFailed,
    OutcomeIntroduced,
}""", """    CommitFailed,
    OutcomeIntroduced,
    RefusalMemberGone,
}"""),
    ("""    pub const ALL: [Self; 35] = [""", """    pub const ALL: [Self; 36] = ["""),
    ("""        Self::CommitFailed,
        Self::OutcomeIntroduced,
    ];""", """        Self::CommitFailed,
        Self::OutcomeIntroduced,
        Self::RefusalMemberGone,
    ];"""),
    ("""            Self::OutcomeIntroduced => ("outcomeIntroduced", "New since this edit", "Neu durch diese Bearbeitung"),
""", f"""            Self::OutcomeIntroduced => ("outcomeIntroduced", "New since this edit", "Neu durch diese Bearbeitung"),
            Self::RefusalMemberGone => ("refusalMemberGone", "{EN}", "{DE}"),
"""),
    ("""pub const TIME_TRAVEL_CODE_LABELS: [(&str, TimeTravelLabel); 17] = [""", """pub const TIME_TRAVEL_CODE_LABELS: [(&str, TimeTravelLabel); 18] = ["""),
    ("""    (TIME_TRAVEL_COMMIT_FAILED_CODE, TimeTravelLabel::CommitFailed),
];""", """    (TIME_TRAVEL_COMMIT_FAILED_CODE, TimeTravelLabel::CommitFailed),
    (TIME_TRAVEL_MEMBER_GONE_CODE, TimeTravelLabel::RefusalMemberGone),
];"""),
])
patch(FWT / "🟦️.ts", [
    ("""/** 🧨️ Driver fault: the finalize commit failed for a reason other than a stale base or a blocking report. */
export const TIME_TRAVEL_COMMIT_FAILED_CODE = "timeTravel.commit-failed";
""", """/** 🧨️ Driver fault: the finalize commit failed for a reason other than a stale base or a blocking report. */
export const TIME_TRAVEL_COMMIT_FAILED_CODE = "timeTravel.commit-failed";

/** 🧩️ Driver fault: the composed member store the session edits was closed mid-session. */
export const TIME_TRAVEL_MEMBER_GONE_CODE = "timeTravel.member-gone";
"""),
    ("""  outcomeIntroduced: { en: "New since this edit", de: "Neu durch diese Bearbeitung" },
} as const""", f"""  outcomeIntroduced: {{ en: "New since this edit", de: "Neu durch diese Bearbeitung" }},
  refusalMemberGone: {{ en: "{EN}", de: "{DE}" }},
}} as const"""),
    ("""  [TIME_TRAVEL_COMMIT_FAILED_CODE, "commitFailed"],
] as const""", """  [TIME_TRAVEL_COMMIT_FAILED_CODE, "commitFailed"],
  [TIME_TRAVEL_MEMBER_GONE_CODE, "refusalMemberGone"],
] as const"""),
])
patch(GEN, [
    ("""    ("outcomeIntroduced", "New since this edit", "Neu durch diese Bearbeitung"),
]""", f"""    ("outcomeIntroduced", "New since this edit", "Neu durch diese Bearbeitung"),
    ("refusalMemberGone", "{EN}", "{DE}"),
]"""),
    ("""    ("timeTravel.commit-failed", "commitFailed"),
]""", """    ("timeTravel.commit-failed", "commitFailed"),
    ("timeTravel.member-gone", "refusalMemberGone"),
]"""),
])
for path, edit in [(FWT / "🧬️schema/🔣️.json", "schema"), (FWT / "🧫️fixtures/🧫️lifecycle-law/🔣️.json", "fixture")]:
    text = path.read_text()
    value = json.loads(text)
    assert json.dumps(value, indent=2, ensure_ascii=False) + "\n" == text, path
    if edit == "schema":
        enum = value["$defs"]["TimeTravelLabelKey"]["enum"]
        if "refusalMemberGone" not in enum:
            enum.append("refusalMemberGone")
        props = value["$defs"]["LifecycleLawFixture"]["properties"]
        props["labels"]["minItems"] = props["labels"]["maxItems"] = len(enum)
        props["codeLabels"]["minItems"] = props["codeLabels"]["maxItems"] = 18
    else:
        if not any(row["key"] == "refusalMemberGone" for row in value["labels"]):
            value["labels"].append({"key": "refusalMemberGone", "en": EN, "de": DE})
        if not any(row["code"] == "timeTravel.member-gone" for row in value["codeLabels"]):
            value["codeLabels"].append({"code": "timeTravel.member-gone", "key": "refusalMemberGone"})
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")
patch(PLUGIN, [
    ("""FaultCode::new("timeTravel.member-gone")""", """FaultCode::new(TIME_TRAVEL_MEMBER_GONE_CODE)"""),
    ("""TIME_TRAVEL_INVALID_INPUT_CODE,
    TIME_TRAVEL_NAME_INVALID_CODE,""", """TIME_TRAVEL_INVALID_INPUT_CODE, TIME_TRAVEL_MEMBER_GONE_CODE,
    TIME_TRAVEL_NAME_INVALID_CODE,"""),
])
print("ok")
