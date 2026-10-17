import csv,sys
tag=sys.argv[1]
rows=list(csv.DictReader(open(f'/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tc2-{tag}/tc2-census-crates.tsv'),delimiter='\t'))
for r in rows:
    w=[(r[f'{l}_libTestFiles'],r[f'{l}_libTestBytes'],r[f'{l}_libTestLines'],r[f'{l}_libTestRatio']) for l in ('native','wasm32-unknown-unknown','wasm32-wasip2')]
    flag="OVER" if any(x[3] and float(x[3])>1 for x in w) or (r['native_worstTestRatio'] and float(r['native_worstTestRatio'])>1) else "ok"
    print(f"{r['crate'].replace('semio-s-artifact-','')}: native {w[0][0]}/{w[0][1]}/{w[0][2]} ({w[0][3]}) worstTest {r['native_worstTestRatio']} {flag}")
