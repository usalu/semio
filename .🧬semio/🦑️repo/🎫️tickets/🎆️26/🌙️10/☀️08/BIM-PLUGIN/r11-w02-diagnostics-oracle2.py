import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
def rep(old, new):
    global s
    assert s.count(old) == 1, (old, s.count(old))
    s = s.replace(old, new)
rep('''def export_csv_table(text):''', '''PANEL_SEVERITIES = (("error", "error"), ("warning", "warning"), ("info", "note"))
"""🚨️ The severities of the exported findings and the keys of the panel groups, most severe first."""


def panel_group_table(document, snapshot):
    """🚨️ The groups of the diagnostics panel recomputed from the exported findings: per severity, the storeys by level (the storeys the model lacks after the known ones, the whole model last) with the findings counted per kind (the domain of the code)."""
    levels = {identifier: storey["level"] for identifier, storey in snapshot["storeys"].items()}
    table = {}
    for token, key in PANEL_SEVERITIES:
        groups = {}
        for finding in document["findings"]:
            if finding["severity"] != token:
                continue
            storey = finding["storey"]
            rank = (0, levels[storey], storey) if storey in levels else ((1, 0, storey) if storey is not None else (2, 0, ""))
            kinds = groups.setdefault(rank, {})
            kind = finding["code"].split(".", 1)[0]
            kinds[kind] = kinds.get(kind, 0) + 1
        table[key] = [{"storey": rank[2] if rank[0] < 2 else None, "kinds": {kind: float(count) for kind, count in kinds.items()}} for rank, kinds in sorted(groups.items())]
    return table


def panel_groups_handler(ctx):
    """🚨️ Oracle answer for the panel groups: the exported JSON is read with the json module and grouped with plain dictionaries."""
    from semio_repo_test import Outcome

    document = json.loads(ctx.input_bytes(export_uri(ctx, "⚠️diagnostics.json")).decode("utf-8"))
    table = panel_group_table(document, case_snapshot(ctx))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def export_csv_table(text):''')
rep('''.oracle("diagnostics-export-csv", export_csv_handler)''', '''.oracle("diagnostics-export-csv", export_csv_handler).oracle("diagnostics-panel-groups", panel_groups_handler)''')
open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
