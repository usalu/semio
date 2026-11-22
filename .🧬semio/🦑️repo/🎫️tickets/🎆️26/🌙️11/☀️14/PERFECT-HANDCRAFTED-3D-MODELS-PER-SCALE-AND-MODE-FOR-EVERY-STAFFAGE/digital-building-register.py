"""📝 Record the canonical sources reopened in the digital equipment review."""
import hashlib,json,re,sys
from pathlib import Path
from runpy import run_path
T=Path(__file__).resolve().parent
G=T/'🗑️generated'
catalog=run_path(str(T/'digital-building-equipment.py'))['catalog']
entries={
    'finned-heat-exchanger':('Finned heat exchanger','Two staggered rows of twelve tubes, two 28 mm headers and twelve hairpins cross 134 individual 0.2 mm fins at 3 mm pitch. The 133 air gaps persist at every scale. Exposed end fins and the reduced structural sample have 24 actual tube apertures. The interior foils overlap copper only at the hidden tube intersections. Both reverse views expose the return bends past the folded end channels.'),
    'panel-radiator':('Panel radiator','Both pressed water panels and the folded convector stock remain visible at all four scales, including in the structural view. The 32 top apertures remain open. The 1:200 paper plan shows the central air passage. The temperature scene follows three height bands of the pressed front. The two 15 mm service tails enter separate unions.'),
    'heat-pump-outdoor-unit':('Heat-pump outdoor unit','Three pitched blades and their motor sit behind five guard wires and eight radial spokes. The side intake retains its inclined louvres. The temperature outlet uses the centre cap and spokes. The front shows the rotor behind the guard and the reverse shows the inclined intake blades in front of the coil. The force view retains the supported fan motor and compressor.'),
    'diesel-generator':('Diesel generator','Four framed intakes pierce the side skins and the 50 mm acoustic lining. Each has fifteen inclined blades before three separated vertical acoustic splitters. The pressed steel roof is 2 mm thick with a folded eave and an opening below the discharge plenum. Folded 50 × 65 × 3 mm steel uprights carry the vent frames between the continuous top and bottom rails. The declared outside enclosure area includes the framed reveals. Both side elevations show the open banks.'),
    'scroll-compressor':('Scroll compressor','Four 40 mm rubber feet on 190.5 mm centres have a genuine 13 mm bore. Separate steel sleeves surround the mounting bolts, and the four bored ears of the 7.5 mm pressed plate capture the rubber shoulders. The force view bears through four independent annular supports.'),
}
for slug in sys.argv[1:]:
    cat=catalog(slug);doc=json.loads(cat.read_bytes())['asset'];sha=hashlib.sha256(cat.read_bytes()).hexdigest()[:16]
    kind=doc['category'];title,body=entries[slug]
    assert 'schema 0, validateAsset 0, checkVolumes 0' in (G/(slug+'-art-canonical.log')).read_text()
    audit=json.loads(next(l for l in (G/(slug+'-art-glb.log')).read_text().splitlines() if l.startswith('{')))
    assert audit['scenes']==12 and audit['findings']==[]
    for form in ('sheet','sheet-reverse'):assert sha[:10] in (G/(slug+'-art-'+form+'.log')).read_text()
    note=f'{body} All twelve GLB scenes reopened ({audit["bytes"]:,} bytes) and all strict checks passed.'
    ledger=T/'digital-building-equipment-review.md';s=ledger.read_text();s='\n'.join(l for l in s.splitlines() if not l.startswith('| '+title+' |'));ledger.write_text(s+'\n| '+title+' | `'+sha+'` | '+note+' |\n')
    for n in ('digital-art-index.md','digital-large-form-review.md'):
        p=T/n;s=p.read_text();row='| '+kind+' | '+slug+' | `'+sha+'` | '+('Rendered |' if n=='digital-art-index.md' else 'Matched | '+body+' |')
        lines=s.splitlines(True);old=[i for i,l in enumerate(lines) if l.startswith('| '+kind+' | '+slug+' | ')];assert len(old)==1;lines[old[0]]=row+'\n';p.write_text(''.join(lines))
    p=T/('r3-'+kind+'-'+slug+'.md');s=re.sub(r'\n## Digital working-stock readback, 2026-11-21\n\nCanonical source [^\n]+\n?', '', p.read_text())
    p.write_text(s.rstrip()+'\n\n## Digital working-stock readback, 2026-11-21\n\nCanonical source `'+sha+'`. '+note+'\n')
