import sys
G, E = sys.argv[1], sys.argv[2]
def patch(p, pairs):
    s = open(p, encoding='utf-8', newline='').read()
    for old, new in pairs:
        assert s.count(old) == 1, (p, old, s.count(old))
        s = s.replace(old, new)
    open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
patch(G + "/🕸️model-graph/🦀️.rs", [("            DiagnosticIndex => &[Diagnostics],\n", "            Self::DiagnosticIndex => &[Diagnostics],\n")])
patch(E + "/🔮️inference/🦀️.rs", [("diagnostic_index => \"s.bim.model.inference.diagnostic-index\", |_, inferred|", "diagnostic_index => \"s.bim.model.inference.diagnostic-index\", |_snapshot, inferred|")])
patch(E + "/📌️panels/🚨️diagnostics/🦀️.rs", [(".map(|((_, _, storey), mut kinds)| StoreyGroup { storey, kinds: order.iter().filter_map(|category| kinds.remove(category).map(|findings| KindGroup { category, findings })).collect() })", ".map(|((_, _, storey), mut kinds)| StoreyGroup { storey, kinds: order.iter().filter_map(|&category| kinds.remove(category).map(|findings| KindGroup { category, findings })).collect() })")])
