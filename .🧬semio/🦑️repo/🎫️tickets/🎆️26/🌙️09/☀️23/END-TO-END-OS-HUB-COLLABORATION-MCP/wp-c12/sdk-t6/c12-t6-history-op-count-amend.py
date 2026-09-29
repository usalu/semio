"""🔢️ C12: one-shot, idempotent amendment of `c12-sdk-composition-patch.py` (history preview, host half). A history row
previews only its edit's newest `HISTORY_ROW_OPERATION_PREVIEW` operations, so the exported `kernel::HistoryEntry` also carries
`op_count` (the edit's forward operations) and the React history panel marks an omitted head with a leading "…" — a row of a
10 000-key typing run no longer reads as if its edit held 8 operations. Extends the SDK law with the exported row.
Usage: python3 <this>"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
PATCH = os.path.join(HERE, "c12-sdk-composition-patch.py")
text = open(PATCH, encoding="utf-8").read()
if "EXPORT_OLD = " in text:
    print("already amended")
    raise SystemExit(0)


def swap(old, new):
    global text
    assert text.count(old) == 1, (old[:90], text.count(old))
    text = text.replace(old, new)


swap("""   digest (the single-operation sealer path is unchanged); the hub, the TS worker and replication carry revisions opaquely.
""", """   digest (the single-operation sealer path is unchanged); the hub, the TS worker and replication carry revisions opaquely.
   (f) The exported `kernel::HistoryEntry` carries `op_count` beside the bounded `op_lines` preview (Rust + TS twin), and the
   React history panel prefixes an omitted head with "…"; the SDK law checks the exported row too.
""")
swap('''PREVIEW_EVAL_LAWS = "''', '''KERNEL = "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs"
KERNEL_TS = "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts"
SHELL_HOST = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
PREVIEW_EVAL_LAWS = "''')

NEW_HUNKS = r'''
EXPORT_OLD = """                    op_lines: entry.op_lines.clone(),
                    applied: entry.applied,"""
EXPORT_NEW = """                    op_lines: entry.op_lines.clone(),
                    op_count: entry.op_count as u64,
                    applied: entry.applied,"""
KERNEL_ENTRY_OLD = """    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub op_lines: Vec<String>,
    #[serde(default)]
    #[value(default)]
    pub applied: bool,"""
KERNEL_ENTRY_NEW = """    /// 📜️ The newest forward operations of this row's edit, newest last — a bounded preview; `op_count` says how many the
    /// edit holds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub op_lines: Vec<String>,
    /// 🔢️ Forward operations of this row's edit; more than `op_lines` holds means the preview omits the older ones.
    #[serde(default)]
    #[value(default)]
    pub op_count: u64,
    #[serde(default)]
    #[value(default)]
    pub applied: bool,"""
KERNEL_TS_OLD = """  readonly opLines?: readonly string[];
  readonly applied?: boolean;"""
KERNEL_TS_NEW = """  /** 📜️ The newest forward operations of this row's edit, newest last — a bounded preview. */
  readonly opLines?: readonly string[];
  /** 🔢️ Forward operations of this row's edit; more than `opLines` holds means the preview omits the older ones. */
  readonly opCount?: number;
  readonly applied?: boolean;"""
SHELL_HOST_OLD = """                description: entry.opLines?.join(" · "),"""
SHELL_HOST_NEW = """                description: entry.opLines && (entry.opCount ?? 0) > entry.opLines.length ? ["…", ...entry.opLines].join(" · ") : entry.opLines?.join(" · "),"""
'''
swap('\nDAG_OLD = ', NEW_HUNKS + '\nDAG_OLD = ')
swap('''        (RUNNABLE_OLD, RUNNABLE_NEW),
    ],''', '''        (RUNNABLE_OLD, RUNNABLE_NEW),
        (EXPORT_OLD, EXPORT_NEW),
    ],
    KERNEL: [(KERNEL_ENTRY_OLD, KERNEL_ENTRY_NEW)],
    KERNEL_TS: [(KERNEL_TS_OLD, KERNEL_TS_NEW)],
    SHELL_HOST: [(SHELL_HOST_OLD, SHELL_HOST_NEW)],''')
swap('''        assert!(row.op_lines.last().is_some_and(|line| line.contains(&value)), "the newest operation closes the preview: {:?}", row.op_lines);
    }
"""
''', '''        assert!(row.op_lines.last().is_some_and(|line| line.contains(&value)), "the newest operation closes the preview: {:?}", row.op_lines);
        let exported = app.history_snapshot().await.expect("history snapshot");
        let exported = exported.upserts.iter().find(|entry| entry.action_id == "setLabel").expect("the gesture's exported row");
        assert_eq!((exported.op_count, exported.op_lines.len()), (64, HISTORY_ROW_OPERATION_PREVIEW), "the exported row carries the bounded preview and its edit's operation count");
    }
"""
''')

open(PATCH, "w", encoding="utf-8").write(text)
print("amended")
